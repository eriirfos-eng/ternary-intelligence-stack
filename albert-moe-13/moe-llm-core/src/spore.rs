// SporeManager — federated weight ingestion for the albert. colony.
//
// Collaborators run local CPU training loops via `albert-train`, export their
// checkpoint as a safetensors "spore" via `albert-spore --name you`, and push
// it to the private albert-spores GitHub repo.  On each epoch boundary the
// main training process scans the local clone of that repo for new spores,
// applies a fitness gate, and blends passing spores into the live VarMap.
//
// Blend strategy (α = 0.08 by default):
//   F32 tensors (gate linears, biases, layer-norms, embeddings):
//     w_merged = (1-α) · w_main + α · w_spore
//   Ternary weight tensors (expert fc1/fc2, attention Q/K/V/O):
//     same blend, then re-ternarize:
//       v >  0.04  →  1.0
//       v < -0.04  → -1.0
//       else       →  0.0
//     With α=0.08, a zero in the main model can be flipped by a clear ±1 in
//     the spore.  The main model wins all sign-flip contests (main=±1 → blend
//     magnitude 0.84, well above threshold).
//
// Fitness gate: spore loss_at_production must be < current_best_epoch_loss + FITNESS_MARGIN.
// FITNESS_MARGIN = 1.0 nat is generous — CPU spores train slowly and may not
// reach GPU-tier loss, but their routing diversity is still valuable.
//
// State: ingested filenames written to models/albert_v3.0.spore_state, one per
// line.  Backward-compatible: if the file is absent, all spores are new.

use candle_core::{Device, DType};
use candle_nn::VarMap;
use serde::Deserialize;
use std::collections::HashSet;
use std::path::PathBuf;
use std::fs;

const DEFAULT_ALPHA:    f32 = 0.08;
const FITNESS_MARGIN:   f32 = 1.0;
const TERNARY_THRESH:   f32 = 0.04;

#[derive(Deserialize, Default)]
struct SporeArch {
    num_layers:  Option<usize>,
    hidden_size: Option<usize>,
    #[allow(dead_code)]
    num_experts: Option<usize>,
    #[allow(dead_code)]
    vocab_size:  Option<usize>,
}

#[derive(Deserialize)]
pub struct SporeMeta {
    pub contributor:        String,
    pub epoch_produced:     usize,
    pub loss_at_production: f64,
    #[serde(default)]
    architecture:           SporeArch,
}

pub struct SporeCandidate {
    pub path: PathBuf,
    pub meta: SporeMeta,
}

pub struct SporeManager {
    spores_dir:         PathBuf,
    state_path:         String,
    pub ingested:       HashSet<String>,
    pub alpha:          f32,
    pub fitness_margin: f32,
}

impl SporeManager {
    /// `spores_dir`  — path to the albert-spores/spores/ directory.
    /// `state_path`  — path to the ingestion-state sidecar file.
    pub fn new(spores_dir: impl Into<PathBuf>, state_path: impl Into<String>) -> Self {
        let mut mgr = SporeManager {
            spores_dir:     spores_dir.into(),
            state_path:     state_path.into(),
            ingested:       HashSet::new(),
            alpha:          DEFAULT_ALPHA,
            fitness_margin: FITNESS_MARGIN,
        };
        mgr.load_state();
        mgr
    }

    fn load_state(&mut self) {
        let Ok(content) = fs::read_to_string(&self.state_path) else { return };
        for line in content.lines() {
            let t = line.trim();
            if !t.is_empty() { self.ingested.insert(t.to_string()); }
        }
        if !self.ingested.is_empty() {
            println!("[spore] {} previously ingested spore(s) on record.", self.ingested.len());
        }
    }

    fn save_state(&self) {
        let content: String = self.ingested.iter().map(|s| format!("{s}\n")).collect();
        if let Err(e) = fs::write(&self.state_path, &content) {
            eprintln!("[spore] Warning: could not save state to {}: {e}", self.state_path);
        }
    }

    /// Walk {spores_dir}/{contributor}/{date}/*.safetensors and return candidates
    /// that have not been ingested yet and pass the fitness gate.
    pub fn scan_pending(&self, current_best_loss: f32) -> Vec<SporeCandidate> {
        let mut out = Vec::new();
        if !self.spores_dir.exists() { return out; }

        let Ok(contrib_iter) = fs::read_dir(&self.spores_dir) else { return out };
        for contrib in contrib_iter.flatten() {
            let cp = contrib.path();
            if !cp.is_dir() { continue; }
            let Ok(date_iter) = fs::read_dir(&cp) else { continue };
            for date in date_iter.flatten() {
                let dp = date.path();
                if !dp.is_dir() { continue; }
                let Ok(spore_iter) = fs::read_dir(&dp) else { continue };
                for entry in spore_iter.flatten() {
                    let sp = entry.path();
                    if sp.extension().and_then(|e| e.to_str()) != Some("safetensors") { continue; }
                    let fname = sp.file_name().unwrap().to_string_lossy().to_string();
                    if self.ingested.contains(&fname) { continue; }

                    let meta_path = sp.with_extension("json");
                    let Some(meta) = fs::read_to_string(&meta_path).ok()
                        .and_then(|s| serde_json::from_str::<SporeMeta>(&s).ok())
                    else {
                        println!("[spore] Skipping {fname} — companion JSON missing or invalid.");
                        continue;
                    };

                    let spore_loss = meta.loss_at_production as f32;
                    if spore_loss > current_best_loss + self.fitness_margin {
                        println!("[spore] {fname} loss={spore_loss:.4} > gate ({:.4}+{:.1}) — skipped.",
                            current_best_loss, self.fitness_margin);
                        continue;
                    }
                    println!("[spore] Candidate: {fname}  contributor={}  ep={}  loss={spore_loss:.4}",
                        meta.contributor, meta.epoch_produced);
                    out.push(SporeCandidate { path: sp, meta });
                }
            }
        }
        out
    }

    /// Blend a spore checkpoint into the live VarMap in-place.
    ///
    /// Each matching tensor is blended: w = (1-α)·w_main + α·w_spore.
    /// Ternary weight tensors are re-ternarized after blending via a CPU round-trip.
    /// Returns a SPORE_INGEST log-line on success, or an error string on failure.
    pub fn ingest(
        &mut self,
        varmap:       &VarMap,
        device:       &Device,
        candidate:    SporeCandidate,
        current_arch: (usize, usize, usize, usize), // (num_layers, hidden_size, num_experts, vocab_size)
    ) -> Result<String, String> {
        let fname = candidate.path.file_name()
            .unwrap().to_string_lossy().to_string();

        // Reject on layer/hidden mismatch — expert count and vocab can be looser.
        let arch = &candidate.meta.architecture;
        let (layers, hidden, _, _) = current_arch;
        if arch.num_layers.map_or(false, |v| v != layers) {
            return Err(format!("arch mismatch: spore {}L vs current {}L",
                arch.num_layers.unwrap(), layers));
        }
        if arch.hidden_size.map_or(false, |v| v != hidden) {
            return Err(format!("arch mismatch: spore H={} vs current H={}",
                arch.hidden_size.unwrap(), hidden));
        }

        let spore_data = candle_core::safetensors::load(&candidate.path, device)
            .map_err(|e| format!("load failed: {e}"))?;

        let alpha    = self.alpha as f64;
        let all_vars = varmap.data().lock().unwrap();
        let mut merged  = 0usize;
        let mut skipped = 0usize;

        for (name, spore_t) in &spore_data {
            let Some(var) = all_vars.get(name) else { skipped += 1; continue };
            if var.shape() != spore_t.shape() { skipped += 1; continue; }

            let main_t = var.as_tensor();

            let blend = main_t.affine(1.0 - alpha, 0.0)
                .and_then(|a| spore_t.affine(alpha, 0.0).and_then(|b| a.add(&b)))
                .map_err(|e| format!("blend error on {name}: {e}"))?;

            // Ternary expert and attention weight tensors: re-ternarize.
            // Gate weights, biases, layer-norm parameters, and embeddings stay as F32 blends.
            let is_ternary = name.contains(".weight")
                && !name.contains("gate")
                && !name.contains("norm")
                && !name.contains("embed")
                && !name.contains("head")
                && !name.contains("anastomosis");

            let final_t = if is_ternary {
                let flat = blend.flatten_all()
                    .and_then(|t| t.to_dtype(DType::F32))
                    .and_then(|t| t.to_vec1::<f32>())
                    .map_err(|e| format!("to_vec1 on {name}: {e}"))?;
                let ternary: Vec<f32> = flat.iter().map(|&v| {
                    if v >  TERNARY_THRESH { 1.0 }
                    else if v < -TERNARY_THRESH { -1.0 }
                    else { 0.0 }
                }).collect();
                candle_core::Tensor::from_vec(ternary, blend.shape(), device)
                    .map_err(|e| format!("from_vec on {name}: {e}"))?
            } else {
                blend
            };

            var.set(&final_t).map_err(|e| format!("set on {name}: {e}"))?;
            merged += 1;
        }
        drop(all_vars);

        self.ingested.insert(fname);
        self.save_state();

        Ok(format!(
            "SPORE_INGEST contributor={} epoch_produced={} spore_loss={:.4} alpha={:.2} merged={} skipped={}",
            candidate.meta.contributor,
            candidate.meta.epoch_produced,
            candidate.meta.loss_at_production,
            self.alpha,
            merged,
            skipped,
        ))
    }
}
