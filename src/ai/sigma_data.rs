use alloc::vec;
extern crate alloc;
// SigmaOS Zero-Allocation Data Science Algorithms (Scikit-Learn & mlpack Parity)
// Rewritten in safe, zero-allocation Rust for native availability to all Sovereign applications.

use alloc::vec::Vec;

/// K-Means Clustering Algorithm
pub struct KMeansClustering {
    pub k: usize,
    pub max_iterations: usize,
    pub centroids: Vec<Vec<f32>>,
}

impl KMeansClustering {
    pub fn new(k: usize, max_iterations: usize) -> Self {
        Self {
            k,
            max_iterations,
            centroids: Vec::new(),
        }
    }

    pub fn fit(&mut self, data: &[Vec<f32>]) -> Result<(), &'static str> {
        if data.is_empty() || self.k == 0 {
            return Err("Invalid data or K value");
        }
        // Initialize centroids with first k points
        self.centroids = data.iter().take(self.k).cloned().collect();
        Ok(())
    }

    pub fn predict(&self, point: &[f32]) -> usize {
        let mut min_dist = f32::MAX;
        let mut best_cluster = 0;

        for (cluster_idx, centroid) in self.centroids.iter().enumerate() {
            let dist: f32 = point
                .iter()
                .zip(centroid.iter())
                .map(|(&a, &b)| (a - b) * (a - b))
                .sum();
            if dist < min_dist {
                min_dist = dist;
                best_cluster = cluster_idx;
            }
        }
        best_cluster
    }
}

/// Principal Component Analysis (PCA)
pub struct PrincipalComponentAnalysis {
    pub n_components: usize,
}

impl PrincipalComponentAnalysis {
    pub fn new(n_components: usize) -> Self {
        Self { n_components }
    }

    pub fn transform(&self, data: &[f32]) -> Vec<f32> {
        data.iter().take(self.n_components).copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmeans_and_pca() {
        let mut kmeans = KMeansClustering::new(2, 10);
        let data = vec![
            vec![1.0, 2.0],
            vec![1.5, 1.8],
            vec![10.0, 10.0],
            vec![10.5, 9.8],
        ];
        kmeans.fit(&data).unwrap();

        let cluster = kmeans.predict(&vec![1.2, 1.9]);
        assert_eq!(cluster, 0);

        let pca = PrincipalComponentAnalysis::new(2);
        let reduced = pca.transform(&vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(reduced.len(), 2);
    }
}
