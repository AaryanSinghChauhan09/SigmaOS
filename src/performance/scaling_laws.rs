// SigmaOS System Performance Scaling Laws Engine
// Mathematical models for thread scaling, work growth, contention overhead, and queueing theory
// Implements Amdahl's Law, Gustafson's Law, Gunther's Universal Scalability Law (USL), and Little's Law


/// Amdahl's Law: Models maximum theoretical speedup for a fixed workload given N parallel cores and parallel fraction P
/// Formula: Speedup(N) = 1 / ((1 - P) + (P / N))
#[derive(Debug, Clone, Copy)]
pub struct AmdahlScalingModel {
    pub parallel_fraction: f64, // P (0.0 to 1.0)
}

impl AmdahlScalingModel {
    pub fn new(parallel_fraction: f64) -> Self {
        Self {
            parallel_fraction: parallel_fraction.clamp(0.0, 1.0),
        }
    }

    pub fn compute_speedup(&self, num_cores: u32) -> f64 {
        let n = (num_cores.max(1)) as f64;
        let p = self.parallel_fraction;
        let serial = 1.0 - p;
        1.0 / (serial + (p / n))
    }

    pub fn compute_max_theoretical_speedup(&self) -> f64 {
        let serial = 1.0 - self.parallel_fraction;
        if serial <= 0.0 {
            f64::INFINITY
        } else {
            1.0 / serial
        }
    }
}

/// Gustafson's Law: Models scaled speedup when problem size grows proportionally with N cores
/// Formula: Speedup(N) = S + N * (1 - S) = (1 - P) + N * P
#[derive(Debug, Clone, Copy)]
pub struct GustafsonScalingModel {
    pub parallel_fraction: f64, // P (0.0 to 1.0)
}

impl GustafsonScalingModel {
    pub fn new(parallel_fraction: f64) -> Self {
        Self {
            parallel_fraction: parallel_fraction.clamp(0.0, 1.0),
        }
    }

    pub fn compute_scaled_speedup(&self, num_cores: u32) -> f64 {
        let n = (num_cores.max(1)) as f64;
        let p = self.parallel_fraction;
        let serial = 1.0 - p;
        serial + (n * p)
    }
}

/// Gunther's Universal Scalability Law (USL): Models concurrency scaling considering serialization AND cache coherency delay
/// Formula: Speedup(N) = N / (1 + sigma * (N - 1) + kappa * N * (N - 1))
/// sigma: Contention / Serialization fraction
/// kappa: Crosstalk / Cache Coherency delay fraction
#[derive(Debug, Clone, Copy)]
pub struct UniversalScalabilityModel {
    pub sigma_contention: f64, // Contention parameter (serialization)
    pub kappa_coherency: f64,  // Coherency crosstalk parameter
}

impl UniversalScalabilityModel {
    pub fn new(sigma_contention: f64, kappa_coherency: f64) -> Self {
        Self {
            sigma_contention: sigma_contention.max(0.0),
            kappa_coherency: kappa_coherency.max(0.0),
        }
    }

    pub fn compute_speedup(&self, num_cores: u32) -> f64 {
        let n = (num_cores.max(1)) as f64;
        let sigma = self.sigma_contention;
        let kappa = self.kappa_coherency;

        let denominator = 1.0 + sigma * (n - 1.0) + kappa * n * (n - 1.0);
        n / denominator
    }

    /// Computes optimal core count N_max where throughput peaks before coherency degradation
    pub fn compute_optimal_cores(&self) -> u32 {
        if self.kappa_coherency <= 0.0 {
            u32::MAX
        } else {
            let n_opt = ((1.0 - self.sigma_contention) / self.kappa_coherency).sqrt();
            n_opt.round().max(1.0) as u32
        }
    }
}

/// Little's Law (Queueing Theory): L = lambda * W
/// L: Average number of items in system (Concurrency)
/// lambda: Average arrival / throughput rate
/// W: Average time spent in system (Latency / Service time)
#[derive(Debug, Clone, Copy)]
pub struct LittleQueueModel;

impl LittleQueueModel {
    /// Computes required concurrency L given arrival rate lambda and latency W
    pub fn compute_concurrency(arrival_rate_lambda: f64, latency_w: f64) -> f64 {
        arrival_rate_lambda * latency_w
    }

    /// Computes average latency W given concurrency L and arrival rate lambda
    pub fn compute_latency(concurrency_l: f64, arrival_rate_lambda: f64) -> f64 {
        if arrival_rate_lambda <= 0.0 {
            0.0
        } else {
            concurrency_l / arrival_rate_lambda
        }
    }

    /// Computes required throughput lambda given concurrency L and latency W
    pub fn compute_throughput(concurrency_l: f64, latency_w: f64) -> f64 {
        if latency_w <= 0.0 {
            0.0
        } else {
            concurrency_l / latency_w
        }
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_amdahl_scaling() {
        let amdahl = AmdahlScalingModel::new(0.75); // 75% parallelizable
        let speedup_4_cores = amdahl.compute_speedup(4);
        // Speedup(4) = 1 / (0.25 + 0.75/4) = 1 / (0.25 + 0.1875) = 1 / 0.4375 = ~2.2857
        assert!((speedup_4_cores - 2.2857).abs() < 1e-3);
        assert_eq!(amdahl.compute_max_theoretical_speedup(), 4.0);
    }

    #[test]
    fn test_gustafson_scaling() {
        let gustafson = GustafsonScalingModel::new(0.90); // 90% parallel
        let speedup_16_cores = gustafson.compute_scaled_speedup(16);
        // Speedup(16) = 0.10 + 16 * 0.90 = 0.10 + 14.4 = 14.5
        assert_eq!(speedup_16_cores, 14.5);
    }

    #[test]
    fn test_universal_scalability_law() {
        let usl = UniversalScalabilityModel::new(0.02, 0.0005);
        let opt_cores = usl.compute_optimal_cores();
        assert!(opt_cores > 10 && opt_cores < 100);

        let speedup_opt = usl.compute_speedup(opt_cores);
        assert!(speedup_opt > 1.0);
    }

    #[test]
    fn test_littles_law() {
        // Arrival rate: 1000 req/sec, Latency: 0.005 sec (5ms) -> Concurrency: 5 items
        let l = LittleQueueModel::compute_concurrency(1000.0, 0.005);
        assert_eq!(l, 5.0);

        let w = LittleQueueModel::compute_latency(5.0, 1000.0);
        assert_eq!(w, 0.005);

        let lambda = LittleQueueModel::compute_throughput(5.0, 0.005);
        assert_eq!(lambda, 1000.0);
    }
}
