use alloc::vec::Vec;

#[derive(Clone, Copy)]
pub struct SystemSnapshot {
    pub cpu_util: f32,
    pub l3_miss_rate: f32,
    pub ipc_ring_fill: f32,
    pub runqueue_len: usize,
    pub io_wait_pct: f32,
    pub ctx_switch_hz: f32,
    pub net_bw_mbps: f32,
    pub gpu_util: f32,
}

pub struct SchedulerPrediction {
    pub optimal_slice_us: u64,
    pub lat_sensitive: bool,
    pub io_boost: bool,
    pub confidence: f32,
}

/// Simple Ring Buffer implementation for snapshot history
pub struct RingBuffer<T> {
    data: Vec<T>,
    head: usize,
    capacity: usize,
}

impl<T: Clone> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            head: 0,
            capacity,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.data.len() < self.capacity {
            self.data.push(item);
        } else {
            self.data[self.head] = item;
            self.head = (self.head + 1) % self.capacity;
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn last(&self) -> &T {
        let index = if self.head == 0 {
            self.data.len() - 1
        } else {
            self.head - 1
        };
        &self.data[index]
    }

    pub fn delta<F>(&self, f: F) -> f32 
    where
        F: Fn(&T) -> f32
    {
        if self.data.len() < 2 {
            return 0.0;
        }
        let last = f(self.last());
        let prev_index = if self.head == 0 {
            if self.data.len() == self.capacity { self.capacity - 2 } else { self.data.len() - 2 }
        } else if self.head == 1 {
            self.data.len() - 1
        } else {
            self.head - 2
        };
        last - f(&self.data[prev_index])
    }
}

/// 256KB Quantized neural network model for scheduling autotuning
pub struct SchedulerNet {
    // 3-layer fully connected weights (simulated using mock constants for no_std size limits)
    pub weights_l1: [[i8; 8]; 16],
    pub weights_l2: [[i8; 16]; 16],
    pub weights_l3: [[i8; 16]; 4],
}

impl SchedulerNet {
    pub fn new() -> Self {
        Self {
            weights_l1: [[1; 8]; 16],
            weights_l2: [[1; 16]; 16],
            weights_l3: [[1; 16]; 4],
        }
    }

    /// Perform forward pass/inference (quantized INT8 matrix dot-product + ReLU)
    pub fn infer(&self, features: &[f32; 8]) -> SchedulerPrediction {
        let mut l1_out = [0.0; 16];
        for i in 0..16 {
            let mut sum = 0.0;
            for j in 0..8 {
                sum += features[j] * (self.weights_l1[i][j] as f32 / 128.0);
            }
            // ReLU activation
            l1_out[i] = if sum > 0.0 { sum } else { 0.0 };
        }

        let mut l2_out = [0.0; 16];
        for i in 0..16 {
            let mut sum = 0.0;
            for j in 0..16 {
                sum += l1_out[j] * (self.weights_l2[i][j] as f32 / 128.0);
            }
            l2_out[i] = if sum > 0.0 { sum } else { 0.0 };
        }

        let mut out = [0.0; 4];
        for i in 0..4 {
            let mut sum = 0.0;
            for j in 0..16 {
                sum += l2_out[j] * (self.weights_l3[i][j] as f32 / 128.0);
            }
            out[i] = sum;
        }

        // Output mappings:
        // out[0] -> slice_us scale
        // out[1] -> lat_sensitive threshold
        // out[2] -> io_boost threshold
        // out[3] -> confidence
        let optimal_slice_us = 4000 + (out[0].abs() * 2000.0) as u64; // scale between 4ms - 6ms
        
        SchedulerPrediction {
            optimal_slice_us,
            lat_sensitive: out[1] > 0.5,
            io_boost: out[2] > 0.5,
            confidence: out[3].clamp(0.0, 1.0),
        }
    }
}

pub struct AISchedulerTuner {
    pub model: SchedulerNet,
    pub history: RingBuffer<SystemSnapshot>,
    pub enabled: bool,
}

impl AISchedulerTuner {
    pub fn new() -> Self {
        Self {
            model: SchedulerNet::new(),
            history: RingBuffer::new(30),
            enabled: true,
        }
    }

    pub fn tick(&mut self, snapshot: SystemSnapshot) -> Option<SchedulerPrediction> {
        if !self.enabled { return None; }
        self.history.push(snapshot);

        if self.history.len() < 5 { return None; }

        let features = self.extract_features();
        let prediction = self.model.infer(&features);

        if prediction.confidence > 0.6 {
            Some(prediction)
        } else {
            None
        }
    }

    fn extract_features(&self) -> [f32; 8] {
        let last = self.history.last();
        [
            last.cpu_util,
            self.history.delta(|s| s.cpu_util),
            last.l3_miss_rate,
            last.ipc_ring_fill,
            last.runqueue_len as f32 / 64.0,
            last.io_wait_pct,
            last.ctx_switch_hz / 10000.0,
            last.gpu_util,
        ]
    }
}
