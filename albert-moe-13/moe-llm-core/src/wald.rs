// WaldModule — loss-space coverage tracking for the EvolutionManager
//
// Named after Abraham Wald (1902–1950), the statistician who corrected the
// WWII aircraft reinforcement problem. Returning planes showed bullet holes
// everywhere *except* the engines — because planes hit in the engines didn't
// return. The lesson: reinforce where there are *no* holes, not where there are.
//
// Applied to training: every batch loss is a bullet hole on the scatter plot.
// The black space — loss regions the model never visits — are the engines.
// A model that never achieves loss < 7.5 on any batch has a systematic gap
// there. That gap is not random. It means the training distribution has no
// examples that push the model into that territory. That's where to reinforce.
//
// The module operates at BATCH granularity (record_batch per step) and reports
// at EPOCH granularity (end_epoch → WaldReport). It emits a WALD telemetry
// line that the dashboard can visualise alongside MYCELIUM and TLIGHT.
//
// Future extension: WaldReport.sample_pressure feeds back into the corpus
// sampler to oversample inputs that historically land near dead_low. This
// closes the Wald loop from observation into curriculum action.
//
// ── Historical note (2026-05-11, Global Epoch 64) ────────────────────────
// For two full epochs this module ran at severity=0.983 — the maximum — and
// reported with four decimal places of certainty that the model had a massive
// dead zone below its loss floor. It detected the engines perfectly.
// Then it printed a warning and did nothing.
//
// The module named after the man who fixed survivorship bias had survivorship
// bias: it only observed what was happening, and ignored what wasn't.
// The very thing Wald became famous for pointing out.
//
// Fixed 2026-05-11: severity now drives amplify_early_layers() dynamically.
// At severity=0.983 → scale=47×. The engines are finally being reinforced.
// — Simeon Kepp & Claude Sonnet 4.6

use std::collections::VecDeque;

/// Histogram resolution constants.
const BUCKET_MIN:   f32   = 3.0;    // minimum trackable loss
const BUCKET_MAX:   f32   = 15.0;   // maximum trackable loss
const BUCKET_WIDTH: f32   = 0.25;   // each bucket spans 0.25 loss units
const NUM_BUCKETS:  usize = ((BUCKET_MAX - BUCKET_MIN) / BUCKET_WIDTH) as usize; // 48

/// A dead zone: a contiguous band of the loss histogram with no visits.
#[derive(Debug, Clone)]
pub struct DeadZone {
    /// Lower bound of the zone (inclusive).
    pub lo: f32,
    /// Upper bound of the zone (exclusive).
    pub hi: f32,
    /// Number of consecutive empty buckets.
    pub width_buckets: usize,
}

impl DeadZone {
    pub fn width(&self) -> f32 { self.hi - self.lo }
}

/// Per-epoch Wald analysis report.
#[derive(Debug)]
pub struct WaldReport {
    /// Fraction of histogram buckets that received at least one visit this epoch.
    pub fill_pct:      f32,
    /// Loss-weighted centroid of the visited distribution.
    pub mass_center:   f32,
    /// Largest contiguous dead zone *below* mass_center (the engines).
    /// None if the model is already visiting the full low range.
    pub dead_low:      Option<DeadZone>,
    /// Largest contiguous dead zone *above* mass_center (correctly avoided territory).
    pub dead_high:     Option<DeadZone>,
    /// Total batch samples recorded this epoch.
    pub total_batches: u64,
    /// Raw bucket visit counts — for dashboard visualisation.
    pub coverage:      Vec<u64>,
}

impl WaldReport {
    /// Returns the severity of the low dead zone in [0.0, 1.0]:
    /// 0.0 = no gap, 1.0 = gap spans entire sub-mass range.
    pub fn low_gap_severity(&self) -> f32 {
        match &self.dead_low {
            None => 0.0,
            Some(z) => {
                let sub_mass_range = (self.mass_center - BUCKET_MIN).max(0.01);
                (z.width() / sub_mass_range).min(1.0)
            }
        }
    }
}

pub struct WaldModule {
    /// Counts for the *current* epoch being accumulated.
    current_epoch:  Vec<u64>,
    /// Rolling window of completed-epoch histograms.
    epoch_history:  VecDeque<Vec<u64>>,
    /// How many epochs of history to retain for smoothed analysis.
    history_len:    usize,
    /// Minimum bucket-visit count to consider a bucket "visited" (filters noise).
    visit_threshold: u64,
}

impl WaldModule {
    pub fn new() -> Self {
        Self {
            current_epoch:   vec![0u64; NUM_BUCKETS],
            epoch_history:   VecDeque::with_capacity(5),
            history_len:     5,
            visit_threshold: 2,
        }
    }

    /// Record a single batch loss value. Call once per training step.
    pub fn record_batch(&mut self, loss: f32) {
        if loss < BUCKET_MIN || loss >= BUCKET_MAX || !loss.is_finite() {
            return;
        }
        let idx = ((loss - BUCKET_MIN) / BUCKET_WIDTH) as usize;
        if idx < NUM_BUCKETS {
            self.current_epoch[idx] += 1;
        }
    }

    /// Finalise the current epoch, produce a WaldReport, and roll the window.
    /// Call once per epoch, after all batches have been recorded.
    pub fn end_epoch(&mut self) -> WaldReport {
        // Commit current epoch to history.
        let epoch_counts = std::mem::replace(&mut self.current_epoch, vec![0u64; NUM_BUCKETS]);
        if self.epoch_history.len() >= self.history_len {
            self.epoch_history.pop_front();
        }
        self.epoch_history.push_back(epoch_counts);

        // Aggregate over history window for smoothed analysis.
        let mut smoothed = vec![0u64; NUM_BUCKETS];
        for hist in &self.epoch_history {
            for (i, &v) in hist.iter().enumerate() {
                smoothed[i] += v;
            }
        }

        let total_batches: u64 = smoothed.iter().sum();
        let threshold = self.visit_threshold * self.epoch_history.len() as u64;

        // Fraction of buckets visited above threshold.
        let visited: Vec<bool> = smoothed.iter().map(|&v| v >= threshold).collect();
        let n_visited = visited.iter().filter(|&&v| v).count();
        let fill_pct = if NUM_BUCKETS > 0 {
            n_visited as f32 / NUM_BUCKETS as f32 * 100.0
        } else { 0.0 };

        // Loss-weighted mass center.
        let mass_center = {
            let (weight_sum, val_sum) = smoothed.iter().enumerate()
                .filter(|&(_, &v)| v > 0)
                .fold((0u64, 0.0f64), |(ws, vs), (i, &v)| {
                    let center = BUCKET_MIN + (i as f32 + 0.5) * BUCKET_WIDTH;
                    (ws + v, vs + v as f64 * center as f64)
                });
            if weight_sum > 0 { (val_sum / weight_sum as f64) as f32 } else { 8.0 }
        };

        // Find all contiguous dead zones.
        let dead_zones = find_dead_zones(&visited);

        // Split into low (below mass_center) and high (above).
        let dead_low = dead_zones.iter()
            .filter(|z| z.hi <= mass_center)
            .max_by(|a, b| a.width_buckets.cmp(&b.width_buckets))
            .cloned();

        let dead_high = dead_zones.iter()
            .filter(|z| z.lo >= mass_center)
            .max_by(|a, b| a.width_buckets.cmp(&b.width_buckets))
            .cloned();

        WaldReport {
            fill_pct,
            mass_center,
            dead_low,
            dead_high,
            total_batches,
            coverage: smoothed,
        }
    }

    /// Called after Net2Net surgery — no state reset needed (loss range is
    /// architecture-independent), but we clear history so post-surgery
    /// analysis isn't polluted by pre-surgery loss levels.
    pub fn on_surgery(&mut self) {
        self.epoch_history.clear();
        self.current_epoch.fill(0);
    }
}

impl Default for WaldModule {
    fn default() -> Self { Self::new() }
}

/// Find all contiguous bands of unvisited buckets in the histogram.
fn find_dead_zones(visited: &[bool]) -> Vec<DeadZone> {
    let mut zones = Vec::new();
    let mut in_zone = false;
    let mut zone_start = 0usize;

    for (i, &v) in visited.iter().enumerate() {
        match (in_zone, v) {
            (false, false) => { in_zone = true; zone_start = i; }
            (true,  true)  => {
                if i - zone_start >= 2 {   // ignore single-bucket gaps (noise)
                    zones.push(DeadZone {
                        lo: BUCKET_MIN + zone_start as f32 * BUCKET_WIDTH,
                        hi: BUCKET_MIN + i as f32 * BUCKET_WIDTH,
                        width_buckets: i - zone_start,
                    });
                }
                in_zone = false;
            }
            _ => {}
        }
    }
    // Close a trailing dead zone.
    if in_zone && visited.len() - zone_start >= 2 {
        zones.push(DeadZone {
            lo: BUCKET_MIN + zone_start as f32 * BUCKET_WIDTH,
            hi: BUCKET_MIN + visited.len() as f32 * BUCKET_WIDTH,
            width_buckets: visited.len() - zone_start,
        });
    }
    zones
}

/// Format a WaldReport as the WALD telemetry line written to training.log.
/// Parsed by the dashboard alongside MYCELIUM and TLIGHT lines.
pub fn format_wald_line(epoch: usize, step: usize, report: &WaldReport) -> String {
    let dead_low_str = match &report.dead_low {
        None    => "none".to_string(),
        Some(z) => format!("{:.2}-{:.2}({:.2})", z.lo, z.hi, z.width()),
    };
    let dead_high_str = match &report.dead_high {
        None    => "none".to_string(),
        Some(z) => format!("{:.2}+({:.2})", z.lo, z.width()),
    };
    // Compact coverage vector: emit only the non-zero slice.
    let first_nonzero = report.coverage.iter().position(|&v| v > 0).unwrap_or(0);
    let last_nonzero  = report.coverage.iter().rposition(|&v| v > 0).unwrap_or(0);
    let coverage_slice: Vec<String> = report.coverage[first_nonzero..=last_nonzero]
        .iter().map(|v| v.to_string()).collect();

    format!(
        "WALD epoch={} step={} fill={:.1}% mass={:.3} dead_low={} dead_high={} severity={:.3} n={}  coverage=[{}]",
        epoch, step,
        report.fill_pct,
        report.mass_center,
        dead_low_str,
        dead_high_str,
        report.low_gap_severity(),
        report.total_batches,
        coverage_slice.join(","),
    )
}
