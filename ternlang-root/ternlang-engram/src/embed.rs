// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.

//! Text → vector embedding.
//!
//! The store core is embedding-agnostic: bring your own f32 vectors via
//! [`crate::EngramStore::remember_vec`] / [`crate::EngramStore::recall_vec`].
//! For a zero-dependency, offline, deterministic default we ship [`HashEmbedder`]
//! (the feature-hashing "hashing trick"). A transformer / NVIDIA / albert embedder
//! drops in behind the [`Embedder`] trait without touching the store.

/// Anything that turns text into a fixed-dimension f32 vector.
///
/// Implementations must be deterministic for the same input (episodic memory is
/// an audit trail; recall must be reproducible).
pub trait Embedder: Send + Sync {
    /// Output dimensionality. Must match the store's `dim`.
    fn dim(&self) -> usize;
    /// Embed a stored memory ("passage" side of an asymmetric model).
    fn embed(&self, text: &str) -> Vec<f32>;
    /// Embed a recall cue ("query" side). Defaults to [`Embedder::embed`] for
    /// symmetric embedders; asymmetric models (e.g. NVIDIA embedqa) override it.
    fn embed_query(&self, text: &str) -> Vec<f32> {
        self.embed(text)
    }
}

/// Deterministic, offline embedder using signed feature hashing over word
/// unigrams and bigrams. No network, no key, no model weights — runs anywhere.
///
/// It captures lexical-distributional similarity, not deep transformer semantics;
/// swap in a real semantic embedder via [`Embedder`] when you need that. Output is
/// naturally sparse, which the ternary quantizer + @sparseskip exploit directly.
#[derive(Clone, Debug)]
pub struct HashEmbedder {
    dim: usize,
}

impl HashEmbedder {
    /// Create a hashing embedder of the given dimensionality (e.g. 256 or 384).
    pub fn new(dim: usize) -> Self {
        assert!(dim > 0, "embedding dim must be > 0");
        Self { dim }
    }

    /// FNV-1a 64-bit — deterministic, no RNG (the core forbids nondeterminism).
    fn fnv1a(bytes: &[u8]) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        for &b in bytes {
            h ^= b as u64;
            h = h.wrapping_mul(0x00000100000001B3);
        }
        h
    }

    fn add_feature(&self, v: &mut [f32], feature: &str) {
        let h = Self::fnv1a(feature.as_bytes());
        let idx = (h % self.dim as u64) as usize;
        // A second, independent hash bit chooses the sign — keeps the mean near 0.
        let sign = if (h >> 63) & 1 == 0 { 1.0 } else { -1.0 };
        v[idx] += sign;
    }
}

impl Embedder for HashEmbedder {
    fn dim(&self) -> usize {
        self.dim
    }

    fn embed(&self, text: &str) -> Vec<f32> {
        let mut v = vec![0f32; self.dim];

        // Lowercase, split on anything non-alphanumeric.
        let tokens: Vec<String> = text
            .to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        for w in &tokens {
            self.add_feature(&mut v, w);
        }
        // Word bigrams add a little word-order signal.
        for pair in tokens.windows(2) {
            let bigram = format!("{}_{}", pair[0], pair[1]);
            self.add_feature(&mut v, &bigram);
        }

        v
    }
}

/// Semantic embedder backed by NVIDIA NIM's OpenAI-compatible embeddings API
/// (`integrate.api.nvidia.com`). Enable with the `nvidia` feature.
///
/// Reads the key from `NVIDIA_API_KEY`. Model/dim/url are overridable via
/// `TERNLANG_ENGRAM_MODEL` / `TERNLANG_ENGRAM_DIM` / `NVIDIA_EMBED_URL`. The
/// default model `nvidia/nv-embedqa-e5-v5` is asymmetric (1024-dim): memories are
/// embedded as `passage`, recall cues as `query`. On any network/API error the
/// embedder degrades gracefully to a zero vector (the episode is still stored)
/// and logs to stderr — it never panics the host.
#[cfg(feature = "nvidia")]
#[derive(Clone, Debug)]
pub struct NvidiaEmbedder {
    api_key: String,
    model: String,
    url: String,
    dim: usize,
}

#[cfg(feature = "nvidia")]
impl NvidiaEmbedder {
    /// Build from the environment. Errors only if `NVIDIA_API_KEY` is unset.
    pub fn from_env() -> anyhow::Result<Self> {
        let api_key = std::env::var("NVIDIA_API_KEY")
            .map_err(|_| anyhow::anyhow!("NVIDIA_API_KEY is not set"))?;
        if api_key.trim().is_empty() {
            return Err(anyhow::anyhow!("NVIDIA_API_KEY is empty"));
        }
        let model = std::env::var("TERNLANG_ENGRAM_MODEL")
            .unwrap_or_else(|_| "nvidia/nv-embedqa-e5-v5".to_string());
        let url = std::env::var("NVIDIA_EMBED_URL")
            .unwrap_or_else(|_| "https://integrate.api.nvidia.com/v1/embeddings".to_string());
        let dim = std::env::var("TERNLANG_ENGRAM_DIM")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1024);
        Ok(Self { api_key, model, url, dim })
    }

    /// Explicit constructor (e.g. for tests or non-env wiring).
    pub fn new(api_key: impl Into<String>, model: impl Into<String>, dim: usize) -> Self {
        Self {
            api_key: api_key.into(),
            model: model.into(),
            url: "https://integrate.api.nvidia.com/v1/embeddings".to_string(),
            dim,
        }
    }

    fn request(&self, text: &str, input_type: &str) -> Vec<f32> {
        let body = ureq::json!({
            "input": [text],
            "model": self.model,
            "input_type": input_type,
            "encoding_format": "float",
            "truncate": "END",
        });
        let resp = ureq::post(&self.url)
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .set("Content-Type", "application/json")
            .send_json(body);
        match resp {
            Ok(r) => {
                let v: serde_json::Value = r.into_json().unwrap_or_default();
                v["data"][0]["embedding"]
                    .as_array()
                    .map(|a| a.iter().map(|x| x.as_f64().unwrap_or(0.0) as f32).collect::<Vec<_>>())
                    .filter(|e| e.len() == self.dim)
                    .unwrap_or_else(|| vec![0.0; self.dim])
            }
            Err(e) => {
                eprintln!("[engram] NVIDIA embed failed ({e}); using zero vector");
                vec![0.0; self.dim]
            }
        }
    }
}

#[cfg(feature = "nvidia")]
impl Embedder for NvidiaEmbedder {
    fn dim(&self) -> usize {
        self.dim
    }
    fn embed(&self, text: &str) -> Vec<f32> {
        self.request(text, "passage")
    }
    fn embed_query(&self, text: &str) -> Vec<f32> {
        self.request(text, "query")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_and_dim() {
        let e = HashEmbedder::new(128);
        let a = e.embed("the cat sat on the mat");
        let b = e.embed("the cat sat on the mat");
        assert_eq!(a.len(), 128);
        assert_eq!(a, b, "embedding must be deterministic");
    }

    #[test]
    fn similar_text_closer_than_unrelated() {
        let e = HashEmbedder::new(512);
        let cos = |a: &[f32], b: &[f32]| {
            let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
            let na: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
            let nb: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
            dot / (na * nb + 1e-9)
        };
        let q = e.embed("ternary neural network inference");
        let near = e.embed("ternary neural network quantization");
        let far = e.embed("the weather in vienna is sunny today");
        assert!(cos(&q, &near) > cos(&q, &far));
    }
}
