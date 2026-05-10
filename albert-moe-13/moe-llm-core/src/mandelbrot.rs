// MandelbrotSurgery — self-similar weight perturbation for Net2Net layer surgery
//
// Standard Net2Net surgery clones a layer then adds Gaussian noise to break
// gradient symmetry. The noise is structureless: two surgeries at different
// depths produce statistically identical perturbations, and the new layer has
// no relationship to the existing stack beyond accident.
//
// This module replaces Gaussian noise with Mandelbrot-parameterised perturbation.
//
// Key property used: the Mandelbrot boundary is a fractal — zooming into any
// region reveals structure self-similar to the whole. By mapping each cloned
// layer to a different "latitude" in Mandelbrot parameter space (a different
// c_im), surgeries at different depths generate perturbation patterns that are
// self-similar to each other but never identical. The layer stack grows the
// same way the Mandelbrot set does: each zoom level (each new layer) inherits
// the global structure while expressing unique local geometry.
//
// The second key property: interior vs. boundary distinction.
//   - Weights mapping to Mandelbrot interior (does not escape): perturbation ≈ 0.
//     These are "settled" regions — near-zero iteration divergence means the
//     weight is in a stable basin. We preserve them. They carry learned features.
//   - Weights mapping near the boundary (slow escape, high iteration count):
//     perturbation is large. These are plastic weights — near-boundary in the
//     loss landscape too, likely. We shake them hardest.
//   - Weights mapping to fast-escape exterior: intermediate perturbation.
//
// This is asymmetric in a principled way. It is not equivalent to curiosity-
// driven RL (which explores state-space) — it explores weight-space, guided by
// the fractal geometry of the surgery's parameter manifold.
//
// No external RNG is used. The perturbation is fully deterministic from:
//   (weight value, tensor position, layer index, total layers)
// This means surgery is reproducible and the perturbation can be logged exactly.

const PHI: f64 = 0.618_033_988_749_895; // golden ratio conjugate

/// Core Mandelbrot iteration z_{n+1} = z_n^2 + c, starting from z = 0.
/// Returns the escape iteration count ∈ [0, max_iter].
/// Returning max_iter means the orbit did not escape — point is inside the set.
#[inline]
pub fn mandelbrot_iter(c_re: f64, c_im: f64, max_iter: u32) -> u32 {
    let (mut z_re, mut z_im) = (0.0_f64, 0.0_f64);
    for i in 0..max_iter {
        let z_re2 = z_re * z_re;
        let z_im2 = z_im * z_im;
        if z_re2 + z_im2 > 4.0 {
            return i;
        }
        z_im = 2.0 * z_re * z_im + c_im;
        z_re = z_re2 - z_im2 + c_re;
    }
    max_iter
}

/// Maps an iteration count to a perturbation magnitude in [0.0, 1.0].
///
/// Interior (iters == max_iter):  0.02  — nearly zero; preserve learned weights.
/// Boundary (iters near max_iter, slow escape): approaches 1.0.
/// Exterior (iters ≈ 0, fast escape): near 0.0.
///
/// The quadratic t^2 curve concentrates sensitivity at the boundary — the only
/// region that matters for breaking symmetry without destroying knowledge.
#[inline]
fn boundary_weight(iters: u32, max_iter: u32) -> f64 {
    if iters == max_iter {
        0.02
    } else {
        let t = iters as f64 / max_iter as f64;
        t * t
    }
}

/// Map a layer index to its unique Mandelbrot latitude (c_im).
///
/// Uses a golden-ratio sequence to distribute layers across the boundary-rich
/// equatorial band c_im ∈ [-0.75, 0.75]. The golden ratio gives maximal
/// spacing between consecutive layers — each new surgery lands in the region
/// farthest from all prior ones, maximising the geometric diversity of the
/// perturbation patterns across the layer stack.
pub fn layer_c_im(layer_idx: usize) -> f64 {
    // Golden-ratio sequence: (layer_idx * φ) mod 1 → well-distributed in [0,1]
    let t = (layer_idx as f64 * PHI).fract();
    // Map [0, 1] → [-0.75, 0.75] — the equatorial boundary band
    -0.75 + t * 1.5
}

/// Apply Mandelbrot-parameterised perturbation to a flat weight buffer.
///
/// `layer_idx`:  index of the new (cloned) layer being initialised.
/// `base_scale`: maximum perturbation magnitude (typically 1e-3 to 5e-3).
/// `max_iter`:   Mandelbrot iteration depth (64 is sufficient; 128 for finer boundary).
///
/// The perturbation is deterministic: no RNG. Noise for element i is:
///   noise_i = sin(i·φ + c_re·π + c_im·e) · base_scale · boundary_weight(iters)
///
/// This is intentionally a simple sinusoidal hash — fast on CPU, zero allocation,
/// deterministic across restarts, and self-similar (same phase formula at every depth).
pub fn mandelbrot_perturb(
    data:       &[f32],
    layer_idx:  usize,
    base_scale: f64,
    max_iter:   u32,
) -> Vec<f32> {
    let c_im = layer_c_im(layer_idx);

    data.iter().enumerate().map(|(i, &w)| {
        // Map weight to c_re: tanh squashes ℝ → (-1, 1), shift to cover the
        // interesting c_re band [-1.5, 0.5] (main cardioid + bulbs live here).
        let c_re = (w as f64).tanh() - 0.5;

        let iters = mandelbrot_iter(c_re, c_im, max_iter);
        let bw    = boundary_weight(iters, max_iter);

        // Deterministic sinusoidal noise. PHI as the phase step gives uniform
        // coverage of the sine period across consecutive tensor elements.
        let phase = i as f64 * PHI + c_re * std::f64::consts::PI + c_im * std::f64::consts::E;
        let noise = phase.sin() * base_scale * bw;

        w + noise as f32
    }).collect()
}

/// High-level surgery perturbation interface for train_bible.rs.
pub struct MandelbrotSurgery {
    /// Maximum perturbation per weight (applied at the boundary, scaled by boundary_weight).
    pub base_scale: f64,
    /// Mandelbrot iteration depth. 64 captures the boundary well; raise to 128 for finer grain.
    pub max_iter:   u32,
}

impl MandelbrotSurgery {
    pub fn new() -> Self {
        Self { base_scale: 1e-3, max_iter: 64 }
    }

    /// Perturb a flat weight buffer for a cloned layer at `layer_idx`.
    /// Returns a new Vec<f32> with the perturbation applied.
    pub fn perturb(&self, data: &[f32], layer_idx: usize) -> Vec<f32> {
        mandelbrot_perturb(data, layer_idx, self.base_scale, self.max_iter)
    }
}

impl Default for MandelbrotSurgery {
    fn default() -> Self { Self::new() }
}

/// Format the Mandelbrot surgery event for training.log.
pub fn format_mandelbrot_line(
    epoch:      usize,
    step:       usize,
    layer_idx:  usize,
    tensor_count: usize,
    base_scale: f64,
    max_iter:   u32,
) -> String {
    let c_im = layer_c_im(layer_idx);
    format!(
        "MAND epoch={} step={} layer={} c_im={:.4} scale={:.2e} max_iter={} tensors={}",
        epoch, step, layer_idx, c_im, base_scale, max_iter, tensor_count,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interior_point_stays_put() {
        // c = (0, 0) is deep inside the Mandelbrot set — should not escape.
        assert_eq!(mandelbrot_iter(0.0, 0.0, 64), 64);
    }

    #[test]
    fn exterior_point_escapes_fast() {
        // c = (2, 2) is well outside — should escape in very few iterations.
        let iters = mandelbrot_iter(2.0, 2.0, 64);
        assert!(iters < 5, "expected fast escape, got {}", iters);
    }

    #[test]
    fn boundary_weight_interior_is_small() {
        assert!(boundary_weight(64, 64) < 0.1);
    }

    #[test]
    fn boundary_weight_slow_escape_is_large() {
        // Slow escape (nearly max_iter) → weight near 1.0
        let bw = boundary_weight(60, 64);
        assert!(bw > 0.8, "expected large boundary weight, got {}", bw);
    }

    #[test]
    fn layer_c_im_distributes_well() {
        // No two consecutive layers should share the same c_im.
        let c0 = layer_c_im(0);
        let c1 = layer_c_im(1);
        let c2 = layer_c_im(2);
        assert!((c0 - c1).abs() > 0.01);
        assert!((c1 - c2).abs() > 0.01);
        // All within the boundary band.
        for i in 0..20 {
            let c = layer_c_im(i);
            assert!(c >= -0.75 && c <= 0.75, "c_im={} out of band at layer {}", c, i);
        }
    }

    #[test]
    fn perturbation_is_small_and_deterministic() {
        let data: Vec<f32> = vec![0.1, -0.3, 0.7, 1.2, -0.8];
        let out1 = mandelbrot_perturb(&data, 3, 1e-3, 64);
        let out2 = mandelbrot_perturb(&data, 3, 1e-3, 64);
        // Deterministic
        assert_eq!(out1, out2);
        // Perturbation is small
        for (a, b) in data.iter().zip(out1.iter()) {
            assert!((a - b).abs() < 0.01, "perturbation too large: {} → {}", a, b);
        }
    }

    #[test]
    fn self_similarity_different_layers_differ() {
        let data: Vec<f32> = vec![0.1, -0.3, 0.7, 1.2, -0.8];
        let out3 = mandelbrot_perturb(&data, 3, 1e-3, 64);
        let out7 = mandelbrot_perturb(&data, 7, 1e-3, 64);
        // Same input, different layers → different output
        let identical = out3.iter().zip(out7.iter()).all(|(a, b)| a == b);
        assert!(!identical, "layers 3 and 7 should produce different perturbations");
    }
}
