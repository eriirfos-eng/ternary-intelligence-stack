pub mod causal;
pub mod cultural;
pub mod ecological;
pub mod ethics;
pub mod legal;
pub mod linguistic;
pub mod logic;
pub mod mathematical;
pub mod medical;
pub mod science;
pub mod spatial;
pub mod technical;
pub mod temporal;

/// Project a query vector through 6 competence axes and return a weighted scalar.
///
/// The 6 axes are: [syntax, world_knowledge, reasoning, tool_use, persona, safety].
/// Input dimensions are distributed across axes by index modulo 6.
pub(crate) fn domain_score(query: &[f32], competence: &[f32; 6]) -> f32 {
    if query.is_empty() {
        return 0.0;
    }
    let mut axis_sum = [0.0f32; 6];
    let mut axis_count = [0usize; 6];
    for (i, &v) in query.iter().enumerate() {
        let axis = i % 6;
        axis_sum[axis] += v;
        axis_count[axis] += 1;
    }
    let mut axis_means = [0.0f32; 6];
    for i in 0..6 {
        if axis_count[i] > 0 {
            axis_means[i] = axis_sum[i] / axis_count[i] as f32;
        }
    }
    let weight_sum: f32 = competence.iter().sum();
    if weight_sum < 1e-9 {
        return 0.0;
    }
    axis_means.iter().zip(competence.iter()).map(|(m, w)| m * w).sum::<f32>() / weight_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_score_empty() {
        assert_eq!(domain_score(&[], &[1.0, 0.5, 0.5, 0.0, 1.0, 0.9]), 0.0);
    }

    #[test]
    fn test_domain_score_uniform_positive() {
        let query = vec![1.0f32; 12];
        let competence = [1.0; 6];
        let score = domain_score(&query, &competence);
        assert!(score > 0.0, "uniform positive query should score positive");
    }

    #[test]
    fn test_domain_score_uniform_negative() {
        let query = vec![-1.0f32; 12];
        let competence = [1.0; 6];
        let score = domain_score(&query, &competence);
        assert!(score < 0.0, "uniform negative query should score negative");
    }
}
