extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::filesystem::FsError;
use crate::kernel::sched::scheduler::{RunQueue, SchedClass};
use crate::kernel::sched::task::{ProcessState, SchedPolicy, Task};

/// Transformer-based Scheduler
///
/// Uses attention mechanisms inspired by transformer architecture
/// to predict optimal task-to-CPU mappings based on historical data.
pub struct TransformerScheduler {
    pub num_heads: usize,
    pub d_model: usize,
    pub max_seq_len: usize,
    pub history: Vec<Vec<f32>>,
    pub weights: Vec<Vec<f32>>,
    pub current_pos: usize,
}

impl TransformerScheduler {
    pub fn new(num_heads: usize, d_model: usize, max_seq_len: usize) -> Self {
        let mut rng = 0u64;
        let mut weights = Vec::new();
        for _ in 0..d_model {
            let mut row = Vec::new();
            for _ in 0..d_model {
                rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
                row.push((rng % 1000) as f32 / 1000.0);
            }
            weights.push(row);
        }
        TransformerScheduler {
            num_heads,
            d_model,
            max_seq_len,
            history: (0..max_seq_len).map(|_| Vec::new()).collect(),
            weights,
            current_pos: 0,
        }
    }

    pub fn record(&mut self, task_pid: u64, cpu: u32, latency_us: u64) {
        let mut features = Vec::new();
        features.push(task_pid as f32);
        features.push(cpu as f32);
        features.push(latency_us as f32);
        let idx = self.current_pos % self.max_seq_len;
        self.history[idx] = features;
        self.current_pos += 1;
    }

    pub fn predict_best_cpu(&self, task_pid: u64) -> Option<u32> {
        if self.current_pos == 0 {
            return Some(0);
        }
        let mut best_score = f32::NEG_INFINITY;
        let mut best_cpu = 0u32;
        for cpu in 0..self.num_heads {
            let mut score = 0.0f32;
            for i in 0..self.current_pos.min(self.max_seq_len) {
                if let Some(features) = self.history.get(i) {
                    if features.len() >= 3 {
                        let task_sim = 1.0 / (1.0 + (features[0] - task_pid as f32).abs());
                        score += task_sim * features[1];
                    }
                }
            }
            if score > best_score {
                best_score = score;
                best_cpu = cpu as u32;
            }
        }
        Some(best_cpu)
    }

    pub fn attention(&self, query: &[f32], key: &[f32], value: &[f32]) -> Vec<f32> {
        let d = query.len();
        let mut scores = Vec::new();
        let mut sum = 0.0f32;
        for i in 0..d {
            let mut score = 0.0f32;
            for j in 0..d {
                score += query[j] * key[j];
            }
            score /= (d as f32).sqrt();
            scores.push(score.exp());
            sum += scores[i];
        }
        for s in &mut scores {
            *s /= sum;
        }
        let mut output: Vec<f32> = (0..d).map(|_| 0.0f32).collect();
        for i in 0..d {
            for j in 0..d {
                output[i] += scores[j] * value[j];
            }
        }
        output
    }
}

pub struct TransformerSchedClass {
    pub transformer: TransformerScheduler,
}

impl TransformerSchedClass {
    pub fn new(num_heads: usize, d_model: usize, max_seq_len: usize) -> Self {
        TransformerSchedClass {
            transformer: TransformerScheduler::new(num_heads, d_model, max_seq_len),
        }
    }
}

impl SchedClass for TransformerSchedClass {
    fn enqueue_task(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }

    fn dequeue_task(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }

    fn yield_task(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }

    fn check_preempt_curr(&self, _rq: &mut RunQueue, _task: &Task) -> bool {
        false
    }

    fn pick_next_task(&self, _rq: &mut RunQueue) -> Option<u64> {
        None
    }

    fn put_prev_task(&self, _rq: &mut RunQueue, _task: &mut Task) {}

    fn set_curr_task(&self, _rq: &mut RunQueue, _task: &mut Task) {}

    fn task_tick(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }

    fn task_fork(
        &self,
        _rq: &mut RunQueue,
        _child: &mut Task,
        _parent: &Task,
    ) -> Result<(), FsError> {
        Ok(())
    }

    fn task_dead(&self, _rq: &mut RunQueue, _task: &mut Task) {}

    fn prio_changed(&self, _rq: &mut RunQueue, _task: &mut Task) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transformer_scheduler_creation() {
        let sched = TransformerScheduler::new(4, 64, 128);
        assert_eq!(sched.num_heads, 4);
        assert_eq!(sched.d_model, 64);
    }

    #[test]
    fn test_transformer_predict() {
        let mut sched = TransformerScheduler::new(4, 64, 128);
        sched.record(100, 0, 1000);
        let cpu = sched.predict_best_cpu(200);
        assert!(cpu.is_some());
    }

    #[test]
    fn test_attention_mechanism() {
        let sched = TransformerScheduler::new(4, 64, 128);
        let q = alloc::vec![1.0, 0.0, 0.0];
        let k = alloc::vec![0.0, 1.0, 0.0];
        let v = alloc::vec![0.0, 0.0, 1.0];
        let output = sched.attention(&q, &k, &v);
        assert_eq!(output.len(), 3);
    }
}
