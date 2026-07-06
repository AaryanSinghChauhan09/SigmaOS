#![no_std]
#![allow(dead_code)]

/// SigmaOS Data Science Tooling
/// In-kernel descriptive statistics for real-time telemetry analysis.

#[derive(Copy, Clone, Default)]
pub struct DescriptiveStats {
    pub count: usize,
    pub mean: f32,
    pub variance: f32,
    pub std_dev: f32,
    pub min: f32,
    pub max: f32,
}

pub struct SigmaDataScience;

impl SigmaDataScience {
    /// Calculates descriptive statistics over a static dataset.
    pub fn calculate_stats(data: &[f32]) -> DescriptiveStats {
        if data.is_empty() {
            return DescriptiveStats::default();
        }
        
        let mut min = data[0];
        let mut max = data[0];
        let mut sum = 0.0;
        
        for &val in data.iter() {
            if val < min { min = val; }
            if val > max { max = val; }
            sum += val;
        }
        
        let count = data.len();
        let mean = sum / (count as f32);
        
        let mut variance_sum = 0.0;
        for &val in data.iter() {
            let diff = val - mean;
            variance_sum += diff * diff;
        }
        
        // Sample variance
        let variance = if count > 1 {
            variance_sum / ((count - 1) as f32)
        } else {
            0.0
        };
        
        // Fast inverse square root could be used here, but we'll simulate std_dev
        // In a strict no_std without libm, sqrt requires a custom implementation.
        // We provide a basic Newton-Raphson sqrt approximation.
        let std_dev = Self::sqrt_approx(variance);
        
        DescriptiveStats {
            count,
            mean,
            variance,
            std_dev,
            min,
            max,
        }
    }
    
    /// Basic Newton-Raphson square root approximation
    fn sqrt_approx(val: f32) -> f32 {
        if val <= 0.0 {
            return 0.0;
        }
        
        let mut guess = val / 2.0;
        for _ in 0..10 {
            guess = 0.5 * (guess + (val / guess));
        }
        guess
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ds_analyze(data_ptr: *const f32, count: usize, out_stats: *mut DescriptiveStats) -> i32 {
    if data_ptr.is_null() || out_stats.is_null() || count == 0 {
        return -1;
    }
    
    let data = core::slice::from_raw_parts(data_ptr, count);
    let stats = SigmaDataScience::calculate_stats(data);
    
    core::ptr::write(out_stats, stats);
    0
}
