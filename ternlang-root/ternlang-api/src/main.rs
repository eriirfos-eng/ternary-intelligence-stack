/// ternlang-api — REST HTTP server for the Ternary Intelligence Stack
///
/// Powers ternlang.com/api
///
/// Public routes (no auth):
///   GET  /                        — API info + available endpoints
///   GET  /health                  — health check
///
/// API routes (X-Ternlang-Key header required):
///   POST /api/trit_decide         — scalar ternary decision
///   POST /api/trit_vector         — multi-dimensional evidence aggregation
///   POST /api/trit_consensus      — consensus(a, b)
///   POST /api/quantize_weights    — BitNet f32 → ternary
///   POST /api/sparse_benchmark    — sparse vs dense matmul stats
///
/// Admin routes (X-Admin-Key header required):
///   POST   /admin/keys            — generate a new API key
///   GET    /admin/keys            — list all keys with usage
///   DELETE /admin/keys/{key}      — revoke a key
///
/// Env vars:
///   TERNLANG_ADMIN_KEY   — admin secret (required in production)
///   KEYS_FILE            — path to JSON key store (default: ./ternlang_keys.json)
///   PORT                 — listening port (default: 3731)
///
/// Run:
///   TERNLANG_ADMIN_KEY=secret cargo run --release --bin ternlang-api

use axum::{
    Router,
    Json,
    extract::{Path, State},
    http::{HeaderMap, Method, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{delete, get, post},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    env,
    path::PathBuf,
    sync::Arc,
};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

use ternlang_core::trit::Trit;
use ternlang_ml::{
    TritScalar, TritEvidenceVec, TEND_BOUNDARY,
    bitnet_threshold, benchmark, dense_matmul, sparse_matmul, TritMatrix,
};

// ─── Key store ───────────────────────────────────────────────────────────────

/// One API key entry. Raw key string is used as the HashMap key so lookup is O(1).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyEntry {
    pub key_id:        String,   // "tk_<uuid_short>"
    pub tier:          u8,       // 1=open, 2=restricted, 3=enterprise
    pub email:         String,
    pub note:          String,   // free-form admin note
    pub created_at:    String,   // ISO 8601
    pub is_active:     bool,
    pub request_count: u64,
}

/// Persistent key store — serialised as JSON to `path`.
#[derive(Debug, Serialize, Deserialize, Default)]
struct KeyStoreData {
    /// Map from raw key string → entry metadata
    keys: HashMap<String, ApiKeyEntry>,
}

pub struct KeyStore {
    data: RwLock<KeyStoreData>,
    path: PathBuf,
}

impl KeyStore {
    /// Load from disk (creates empty file if it doesn't exist).
    pub async fn load(path: PathBuf) -> Arc<Self> {
        let data = if path.exists() {
            let raw = tokio::fs::read_to_string(&path).await.unwrap_or_default();
            serde_json::from_str::<KeyStoreData>(&raw).unwrap_or_default()
        } else {
            KeyStoreData::default()
        };
        Arc::new(KeyStore { data: RwLock::new(data), path })
    }

    /// Persist current state to disk (best-effort; logs on error).
    async fn save(&self) {
        let data = self.data.read().await;
        match serde_json::to_string_pretty(&*data) {
            Ok(json) => {
                if let Err(e) = tokio::fs::write(&self.path, json).await {
                    eprintln!("[key-store] save error: {}", e);
                }
            }
            Err(e) => eprintln!("[key-store] serialise error: {}", e),
        }
    }

    /// Check a raw key and, if valid, increment its counter.
    pub async fn validate_and_bump(&self, raw_key: &str) -> Option<ApiKeyEntry> {
        let mut data = self.data.write().await;
        let entry = data.keys.get_mut(raw_key)?;
        if !entry.is_active {
            return None;
        }
        entry.request_count += 1;
        Some(entry.clone())
    }

    /// Generate a new key. Returns (raw_key, entry).
    pub async fn generate(&self, tier: u8, email: String, note: String) -> (String, ApiKeyEntry) {
        let uid   = Uuid::new_v4().to_string().replace('-', "");
        let raw   = format!("tern_{}_{}", tier, &uid[..24]);
        let key_id = format!("tk_{}", &uid[..8]);

        let entry = ApiKeyEntry {
            key_id:        key_id.clone(),
            tier,
            email,
            note,
            created_at:    Utc::now().to_rfc3339(),
            is_active:     true,
            request_count: 0,
        };

        self.data.write().await.keys.insert(raw.clone(), entry.clone());
        self.save().await;
        (raw, entry)
    }

    /// Revoke a key by raw value. Returns true if the key existed.
    pub async fn revoke(&self, raw_key: &str) -> bool {
        let mut data = self.data.write().await;
        if let Some(entry) = data.keys.get_mut(raw_key) {
            entry.is_active = false;
            drop(data);
            self.save().await;
            return true;
        }
        false
    }

    /// List all entries (key hidden, only metadata).
    pub async fn list(&self) -> Vec<Value> {
        let data = self.data.read().await;
        data.keys.iter().map(|(raw, e)| json!({
            "key_id":        e.key_id,
            "key_preview":   format!("{}…", &raw[..12]),
            "tier":          e.tier,
            "email":         e.email,
            "note":          e.note,
            "created_at":    e.created_at,
            "is_active":     e.is_active,
            "request_count": e.request_count,
        })).collect()
    }
}

// ─── App state ───────────────────────────────────────────────────────────────

#[derive(Clone)]
struct AppState {
    admin_key: String,
    keys:      Arc<KeyStore>,
    version:   &'static str,
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn trit_to_i8(t: Trit) -> i8 {
    match t { Trit::NegOne => -1, Trit::Zero => 0, Trit::PosOne => 1 }
}

fn i8_to_trit(v: i64) -> Option<Trit> {
    match v { -1 => Some(Trit::NegOne), 0 => Some(Trit::Zero), 1 => Some(Trit::PosOne), _ => None }
}

fn api_error(status: StatusCode, message: &str) -> Response {
    (status, Json(json!({ "error": message, "docs": "https://ternlang.com/docs/api" }))).into_response()
}

// ─── Auth middleware (API routes) ─────────────────────────────────────────────

async fn require_api_key(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    request: axum::extract::Request,
    next: Next,
) -> Response {
    let path = request.uri().path().to_string();

    // Public endpoints — no key required
    if path == "/" || path == "/health" || path.starts_with("/admin") {
        return next.run(request).await;
    }

    let raw = headers
        .get("X-Ternlang-Key")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if raw.is_empty() {
        return api_error(StatusCode::UNAUTHORIZED,
            "Missing X-Ternlang-Key header. Acquire a key at https://ternlang.com/#licensing");
    }

    match state.keys.validate_and_bump(raw).await {
        Some(_entry) => next.run(request).await,
        None => api_error(StatusCode::UNAUTHORIZED, "Invalid or revoked API key."),
    }
}

// ─── Admin middleware ──────────────────────────────────────────────────────────

async fn require_admin_key(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    request: axum::extract::Request,
    next: Next,
) -> Response {
    if !request.uri().path().starts_with("/admin") {
        return next.run(request).await;
    }

    let provided = headers
        .get("X-Admin-Key")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if provided.is_empty() {
        return api_error(StatusCode::UNAUTHORIZED, "Missing X-Admin-Key header.");
    }
    if provided != state.admin_key {
        return api_error(StatusCode::UNAUTHORIZED, "Invalid admin key.");
    }

    next.run(request).await
}

// ─── GET / ───────────────────────────────────────────────────────────────────

async fn root(State(state): State<Arc<AppState>>) -> Json<Value> {
    Json(json!({
        "name":    "Ternlang API",
        "version": state.version,
        "by":      "RFI-IRFOS",
        "website": "https://ternlang.com",
        "docs":    "https://ternlang.com/docs/api",
        "auth":    "X-Ternlang-Key header required for /api/* endpoints",
        "endpoints": {
            "POST /api/trit_decide":      "Scalar ternary decision: evidence[] → reject/tend/affirm + confidence",
            "POST /api/trit_vector":      "Multi-dimensional evidence: named dimensions + weights → aggregate",
            "POST /api/trit_consensus":   "consensus(a, b) → ternary result",
            "POST /api/quantize_weights": "f32[] → ternary weights via BitNet threshold",
            "POST /api/sparse_benchmark": "Sparse vs dense matmul performance stats",
        },
        "acquire_key": "https://ternlang.com/#licensing"
    }))
}

// ─── GET /health ─────────────────────────────────────────────────────────────

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "engine": "BET VM", "trit": 1 }))
}

// ─── Admin: POST /admin/keys ──────────────────────────────────────────────────

#[derive(Deserialize)]
struct GenerateKeyBody {
    tier:  Option<u8>,
    email: Option<String>,
    note:  Option<String>,
}

async fn admin_generate_key(
    State(state): State<Arc<AppState>>,
    Json(body): Json<GenerateKeyBody>,
) -> Response {
    let tier  = body.tier.unwrap_or(2);
    let email = body.email.unwrap_or_default();
    let note  = body.note.unwrap_or_default();

    if tier < 1 || tier > 3 {
        return api_error(StatusCode::BAD_REQUEST, "tier must be 1, 2, or 3");
    }

    let (raw, entry) = state.keys.generate(tier, email, note).await;

    eprintln!("[admin] generated key {} for {}", entry.key_id, entry.email);

    (StatusCode::CREATED, Json(json!({
        "key":     raw,        // Only returned once — save it!
        "key_id":  entry.key_id,
        "tier":    entry.tier,
        "email":   entry.email,
        "created": entry.created_at,
        "warning": "Store this key securely — it will not be shown again.",
    }))).into_response()
}

// ─── Admin: GET /admin/keys ───────────────────────────────────────────────────

async fn admin_list_keys(State(state): State<Arc<AppState>>) -> Json<Value> {
    let entries = state.keys.list().await;
    Json(json!({ "total": entries.len(), "keys": entries }))
}

// ─── Admin: DELETE /admin/keys/{key} ─────────────────────────────────────────

async fn admin_revoke_key(
    State(state): State<Arc<AppState>>,
    Path(raw_key): Path<String>,
) -> Response {
    if state.keys.revoke(&raw_key).await {
        eprintln!("[admin] revoked key {}", &raw_key[..12.min(raw_key.len())]);
        (StatusCode::OK, Json(json!({ "revoked": true }))).into_response()
    } else {
        api_error(StatusCode::NOT_FOUND, "Key not found.")
    }
}

// ─── POST /api/trit_decide ───────────────────────────────────────────────────

async fn trit_decide(Json(body): Json<Value>) -> Response {
    let evidence: Vec<f32> = match body["evidence"].as_array() {
        Some(arr) => match arr.iter()
            .map(|v| v.as_f64().map(|f| f as f32).ok_or(()))
            .collect::<Result<Vec<_>, _>>() {
                Ok(v) => v,
                Err(_) => return api_error(StatusCode::BAD_REQUEST, "evidence values must be numbers"),
            },
        None => return api_error(StatusCode::BAD_REQUEST, "evidence must be an array of numbers"),
    };

    if evidence.is_empty() {
        return api_error(StatusCode::BAD_REQUEST, "evidence cannot be empty");
    }

    let min_confidence = body["min_confidence"].as_f64().unwrap_or(0.0) as f32;
    let mean = evidence.iter().sum::<f32>() / evidence.len() as f32;
    let scalar = TritScalar::new(mean);

    let per_signal: Vec<Value> = evidence.iter().enumerate().map(|(i, &v)| {
        let s = TritScalar::new(v);
        json!({
            "index":      i,
            "raw":        (v * 1000.0).round() / 1000.0,
            "label":      s.label(),
            "confidence": (s.confidence() * 1000.0).round() / 1000.0,
            "trit":       trit_to_i8(s.trit()),
        })
    }).collect();

    let zeros = per_signal.iter().filter(|s| s["trit"] == 0).count();
    let actionable = scalar.is_actionable(min_confidence);

    let recommendation = match scalar.trit() {
        Trit::PosOne => format!(
            "Affirm — confidence {:.0}%{}.",
            scalar.confidence() * 100.0,
            if actionable { "" } else { " (below min_confidence — gather more evidence)" }
        ),
        Trit::NegOne => format!(
            "Reject — confidence {:.0}%{}.",
            scalar.confidence() * 100.0,
            if actionable { "" } else { " (below min_confidence — gather more evidence)" }
        ),
        Trit::Zero => format!(
            "Tend — scalar {:.3} is in the deliberation zone [{:.3}, +{:.3}]. Gather more evidence.",
            scalar.raw(), -TEND_BOUNDARY, TEND_BOUNDARY
        ),
    };

    (StatusCode::OK, Json(json!({
        "scalar":          (scalar.raw() * 1000.0).round() / 1000.0,
        "trit":            trit_to_i8(scalar.trit()),
        "label":           scalar.label(),
        "confidence":      (scalar.confidence() * 1000.0).round() / 1000.0,
        "is_actionable":   actionable,
        "tend_boundary":   TEND_BOUNDARY,
        "signal_sparsity": zeros as f64 / evidence.len() as f64,
        "recommendation":  recommendation,
        "per_signal":      per_signal,
    }))).into_response()
}

// ─── POST /api/trit_vector ───────────────────────────────────────────────────

async fn trit_vector(Json(body): Json<Value>) -> Response {
    let dims = match body["dimensions"].as_array() {
        Some(d) => d,
        None => return api_error(StatusCode::BAD_REQUEST,
            "dimensions must be an array of {label, value, weight} objects"),
    };

    if dims.is_empty() {
        return api_error(StatusCode::BAD_REQUEST, "dimensions cannot be empty");
    }

    let min_confidence = body["min_confidence"].as_f64().unwrap_or(0.5) as f32;

    let mut labels  = Vec::new();
    let mut values  = Vec::new();
    let mut weights = Vec::new();

    for (i, d) in dims.iter().enumerate() {
        let label  = d["label"].as_str().unwrap_or("unnamed").to_string();
        let value  = match d["value"].as_f64() {
            Some(v) => v as f32,
            None => return api_error(StatusCode::BAD_REQUEST,
                &format!("dimensions[{}].value must be a number", i)),
        };
        let weight = d["weight"].as_f64().unwrap_or(1.0) as f32;
        if weight < 0.0 {
            return api_error(StatusCode::BAD_REQUEST,
                &format!("dimensions[{}].weight must be >= 0", i));
        }
        labels.push(label);
        values.push(value);
        weights.push(weight);
    }

    let ev      = TritEvidenceVec::new(labels, values, weights);
    let agg     = ev.aggregate();
    let scalars = ev.scalars();
    let actionable = agg.is_actionable(min_confidence);

    let breakdown: Vec<Value> = ev.dimensions.iter()
        .zip(ev.values.iter())
        .zip(ev.weights.iter())
        .zip(scalars.iter())
        .map(|(((label, &val), &w), sc)| json!({
            "label":      label,
            "raw":        (val * 1000.0).round() / 1000.0,
            "weight":     w,
            "trit":       trit_to_i8(sc.trit()),
            "label_trit": sc.label(),
            "confidence": (sc.confidence() * 1000.0).round() / 1000.0,
        })).collect();

    let zeros = breakdown.iter().filter(|d| d["trit"] == 0).count();

    (StatusCode::OK, Json(json!({
        "aggregate": {
            "scalar":     (agg.raw() * 1000.0).round() / 1000.0,
            "trit":       trit_to_i8(agg.trit()),
            "label":      agg.label(),
            "confidence": (agg.confidence() * 1000.0).round() / 1000.0,
            "is_actionable": actionable,
        },
        "dimensions":       breakdown,
        "tend_boundary":    TEND_BOUNDARY,
        "signal_sparsity":  zeros as f64 / ev.dimensions.len() as f64,
        "recommendation":   match agg.trit() {
            Trit::PosOne => "Affirm — weighted evidence crosses threshold.".to_string(),
            Trit::NegOne => "Reject — weighted evidence crosses negative threshold.".to_string(),
            Trit::Zero   => format!(
                "Tend — aggregate {:.3} within deliberation zone. Resolve conflicting dimensions.",
                agg.raw()
            ),
        },
    }))).into_response()
}

// ─── POST /api/trit_consensus ────────────────────────────────────────────────

async fn trit_consensus(Json(body): Json<Value>) -> Response {
    let a = match body["a"].as_i64().and_then(i8_to_trit) {
        Some(t) => t,
        None => return api_error(StatusCode::BAD_REQUEST, "a must be -1, 0, or 1"),
    };
    let b = match body["b"].as_i64().and_then(i8_to_trit) {
        Some(t) => t,
        None => return api_error(StatusCode::BAD_REQUEST, "b must be -1, 0, or 1"),
    };

    // consensus: agree → common value (carry=0); disagree → 0 (carry=1)
    let result = if a == b { a } else { Trit::Zero };
    let carry  = if a == b { Trit::Zero } else { Trit::PosOne };

    (StatusCode::OK, Json(json!({
        "a":      trit_to_i8(a),
        "b":      trit_to_i8(b),
        "result": trit_to_i8(result),
        "carry":  trit_to_i8(carry),
        "label":  TritScalar::new(trit_to_i8(result) as f32).label(),
    }))).into_response()
}

// ─── POST /api/quantize_weights ──────────────────────────────────────────────

async fn quantize_weights(Json(body): Json<Value>) -> Response {
    let weights: Vec<f32> = match body["weights"].as_array() {
        Some(arr) => arr.iter().map(|v| v.as_f64().unwrap_or(0.0) as f32).collect(),
        None => return api_error(StatusCode::BAD_REQUEST, "weights must be an array of numbers"),
    };

    if weights.is_empty() {
        return api_error(StatusCode::BAD_REQUEST, "weights cannot be empty");
    }

    let threshold = body["threshold"].as_f64()
        .unwrap_or_else(|| bitnet_threshold(&weights) as f64) as f32;

    let trits: Vec<i8> = weights.iter().map(|&w| {
        if w > threshold { 1 }
        else if w < -threshold { -1 }
        else { 0 }
    }).collect();

    let zeros    = trits.iter().filter(|&&t| t == 0).count();
    let sparsity = zeros as f64 / trits.len() as f64;

    (StatusCode::OK, Json(json!({
        "threshold":       (threshold * 1000.0).round() / 1000.0,
        "trits":           trits,
        "sparsity":        (sparsity * 1000.0).round() / 1000.0,
        "non_zero":        trits.len() - zeros,
        "bits_saved":      format!("{:.1}%", sparsity * 100.0),
        "zone":            if sparsity < 0.40 { "warm" }
                           else if sparsity <= 0.60 { "goldilocks ★" }
                           else { "asymptotic" },
    }))).into_response()
}

// ─── POST /api/sparse_benchmark ──────────────────────────────────────────────

async fn sparse_benchmark(Json(body): Json<Value>) -> Response {
    let rows = body["rows"].as_u64().unwrap_or(4) as usize;
    let cols = body["cols"].as_u64().unwrap_or(4) as usize;

    if rows == 0 || cols == 0 || rows > 512 || cols > 512 {
        return api_error(StatusCode::BAD_REQUEST, "rows and cols must be between 1 and 512");
    }

    let f32_weights: Vec<f32> = match body["weights"].as_array() {
        Some(arr) => arr.iter().map(|v| v.as_f64().unwrap_or(0.0) as f32).collect(),
        None => (0..rows * cols).map(|i| match i % 5 {
            0 => 0.9, 1 => -0.8, 2 => 0.1, 3 => -0.1, _ => 0.05
        }).collect(),
    };

    if f32_weights.len() != rows * cols {
        return api_error(StatusCode::BAD_REQUEST,
            &format!("weights length must equal rows×cols = {}", rows * cols));
    }

    let threshold = body["threshold"].as_f64()
        .unwrap_or_else(|| bitnet_threshold(&f32_weights) as f64) as f32;

    let w     = TritMatrix::from_f32(rows, cols, &f32_weights, threshold);
    let input = TritMatrix::new(rows, cols);
    let r     = benchmark(&input, &w);
    let (_, skipped) = sparse_matmul(&input, &w);

    (StatusCode::OK, Json(json!({
        "rows":                rows,
        "cols":                cols,
        "weight_sparsity":     r.weight_sparsity,
        "skip_rate":           r.skip_rate,
        "dense_ops":           r.dense_ops,
        "sparse_ops":          r.sparse_ops,
        "skipped_ops":         skipped,
        "ops_reduction_factor": r.dense_ops as f64 / r.sparse_ops.max(1) as f64,
        "threshold_used":      threshold,
        "summary": format!(
            "{:.1}% weight sparsity → {:.2}× fewer multiply ops ({} skipped of {})",
            r.weight_sparsity * 100.0,
            r.dense_ops as f64 / r.sparse_ops.max(1) as f64,
            skipped,
            r.dense_ops
        ),
    }))).into_response()
}

// ─── 404 fallback ─────────────────────────────────────────────────────────────

async fn not_found() -> Response {
    api_error(StatusCode::NOT_FOUND, "Endpoint not found. See GET / for available routes.")
}

// ─── Main ─────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let admin_key = env::var("TERNLANG_ADMIN_KEY").unwrap_or_else(|_| {
        eprintln!("[ternlang-api] WARNING: TERNLANG_ADMIN_KEY not set — using 'admin-dev'");
        eprintln!("[ternlang-api] Set TERNLANG_ADMIN_KEY=<secret> in production");
        "admin-dev".to_string()
    });

    let keys_file = env::var("KEYS_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("ternlang_keys.json"));

    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3731);   // 3731 — ternary

    let keys = KeyStore::load(keys_file).await;

    let state = Arc::new(AppState { admin_key, keys, version: "0.1.0" });

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        // Public
        .route("/",       get(root))
        .route("/health", get(health))
        // API (requires X-Ternlang-Key)
        .route("/api/trit_decide",       post(trit_decide))
        .route("/api/trit_vector",       post(trit_vector))
        .route("/api/trit_consensus",    post(trit_consensus))
        .route("/api/quantize_weights",  post(quantize_weights))
        .route("/api/sparse_benchmark",  post(sparse_benchmark))
        // Admin (requires X-Admin-Key)
        .route("/admin/keys",            post(admin_generate_key).get(admin_list_keys))
        .route("/admin/keys/{key}",      delete(admin_revoke_key))
        .fallback(not_found)
        .layer(middleware::from_fn_with_state(state.clone(), require_admin_key))
        .layer(middleware::from_fn_with_state(state.clone(), require_api_key))
        .layer(cors)
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    eprintln!("[ternlang-api] listening on http://{}", addr);
    eprintln!("[ternlang-api] docs: https://ternlang.com/docs/api");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
