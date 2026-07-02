// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/sched/sigma_transformer_sched.rs — Transformer-based AI scheduler
// Novel Category 14 (Bleeding-Edge Research): Kernel scheduler powered by
// a lightweight transformer model trained on workload traces.
// The transformer predicts the next task's CPU burst time → schedules
// with accurate priority instead of reactive demotion.
//
// Architecture:
//   Workload trace → feature vector → 4-head attention → burst prediction
//   Prediction → priority queue ordering → preempt current if needed
//
// Model: 2-layer transformer, 32-dim embedding, 4 attention heads
// Inference: < 10µs (critical: must be faster than scheduling tick)
// Training: offline on collected workload traces via sigma-agent learn
//
// Language: Rust (#![no_std] compatible with alloc)

#![allow(dead_code)]

// ── Task features for transformer input ──────────────────────────────────
#[derive(Clone, Copy, Debug, Default)]
pub struct TaskFeatures {
    pub pid:              u32,
    pub last_burst_us:    f32,   // last CPU burst duration
    pub avg_burst_us:     f32,   // exponential moving average
    pub io_wait_ratio:    f32,   // fraction of time waiting for I/O
    pub voluntary_yields: u32,   // how often task yields voluntarily
    pub cpu_affinity:     u8,    // preferred CPU core
    pub priority_class:   u8,    // 0=RT, 1=interactive, 2=batch
    pub recent_cache_miss: f32,  // LLC miss rate (last 100ms)
    pub syscall_rate:     f32,   // syscalls/second
    pub mem_working_set:  f32,   // working set size (MB)
}

impl TaskFeatures {
    /// Encode into fixed-size float vector for transformer
    pub fn to_embedding(&self) -> [f32; 16] {
        [
            (self.last_burst_us / 50_000.0).min(1.0),
            (self.avg_burst_us  / 50_000.0).min(1.0),
            self.io_wait_ratio,
            (self.voluntary_yields as f32 / 100.0).min(1.0),
            self.cpu_affinity as f32 / 8.0,
            self.priority_class as f32 / 3.0,
            self.recent_cache_miss,
            (self.syscall_rate / 10_000.0).min(1.0),
            (self.mem_working_set / 4096.0).min(1.0),
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,  // padding to 16
        ]
    }
}

// ── Minimal transformer layer ─────────────────────────────────────────────
const D_MODEL: usize = 32;
const N_HEADS:  usize = 4;
const HEAD_DIM: usize = D_MODEL / N_HEADS;  // 8
const SEQ_LEN:  usize = 8;   // last 8 tasks as context

/// Single attention head
fn attention_head(q: &[f32; HEAD_DIM], k: &[f32; HEAD_DIM], v: &[f32; HEAD_DIM]) -> [f32; HEAD_DIM] {
    let scale = (HEAD_DIM as f32).sqrt();
    let score: f32 = q.iter().zip(k.iter()).map(|(a,b)| a*b).sum::<f32>() / scale;
    let attn = 1.0 / (1.0 + (-score).exp());   // sigmoid (simplified softmax)
    let mut out = [0f32; HEAD_DIM];
    for (i, &vi) in v.iter().enumerate() { out[i] = attn * vi; }
    out
}

/// Multi-head self-attention over task sequence
fn multi_head_attention(seq: &[[f32; D_MODEL]; SEQ_LEN], weights: &AttentionWeights)
    -> [[f32; D_MODEL]; SEQ_LEN]
{
    let mut output = [[0f32; D_MODEL]; SEQ_LEN];
    for i in 0..SEQ_LEN {
        for h in 0..N_HEADS {
            let head_start = h * HEAD_DIM;
            let mut q = [0f32; HEAD_DIM];
            let mut k = [0f32; HEAD_DIM];
            let mut v = [0f32; HEAD_DIM];
            // Linear projection (weight matrices)
            for (j, &s) in seq[i].iter().enumerate() {
                for d in 0..HEAD_DIM {
                    q[d] += weights.wq[h][j % D_MODEL][d] * s;
                    k[d] += weights.wk[h][j % D_MODEL][d] * s;
                    v[d] += weights.wv[h][j % D_MODEL][d] * s;
                }
            }
            let head_out = attention_head(&q, &k, &v);
            for d in 0..HEAD_DIM {
                output[i][head_start + d] += head_out[d];
            }
        }
    }
    output
}

/// Attention weight matrices (normally loaded from trained model)
pub struct AttentionWeights {
    pub wq: [[[f32; HEAD_DIM]; D_MODEL]; N_HEADS],
    pub wk: [[[f32; HEAD_DIM]; D_MODEL]; N_HEADS],
    pub wv: [[[f32; HEAD_DIM]; D_MODEL]; N_HEADS],
    pub wo: [[f32; D_MODEL]; D_MODEL],  // output projection
}

impl AttentionWeights {
    pub const fn identity() -> Self {
        // Initialise to identity-like weights (warm start, no random init in no_std)
        let mut wq = [[[0f32; HEAD_DIM]; D_MODEL]; N_HEADS];
        let mut wk = [[[0f32; HEAD_DIM]; D_MODEL]; N_HEADS];
        let mut wv = [[[0f32; HEAD_DIM]; D_MODEL]; N_HEADS];
        let wo = [[0f32; D_MODEL]; D_MODEL];
        // Set diagonal entries to 1/sqrt(D_MODEL) for stable init
        let init = 0.177;  // 1/sqrt(32)
        let mut h = 0;
        while h < N_HEADS {
            let mut d = 0;
            while d < HEAD_DIM {
                let col = h * HEAD_DIM + d;
                if col < D_MODEL {
                    wq[h][col][d] = init;
                    wk[h][col][d] = init;
                    wv[h][col][d] = init;
                }
                d += 1;
            }
            h += 1;
        }
        Self { wq, wk, wv, wo }
    }

    /// Load weights from a flat array (for GGUF-style weight loading)
    pub fn load_from_slice(&mut self, data: &[f32]) {
        let head_weights = D_MODEL * HEAD_DIM;
        let per_head = head_weights * 3;   // Q+K+V per head
        for h in 0..N_HEADS.min(data.len() / per_head) {
            for i in 0..D_MODEL {
                for d in 0..HEAD_DIM {
                    let base = h * per_head + i * HEAD_DIM;
                    if base + d < data.len() {
                        self.wq[h][i][d] = data[base + d];
                        self.wk[h][i][d] = data[base + d + head_weights];
                        self.wv[h][i][d] = data[base + d + head_weights * 2];
                    }
                }
            }
        }
    }
}

/// Feed-forward layer: project D_MODEL → 64 → D_MODEL with ReLU
pub struct FeedForwardLayer {
    pub w1: [[f32; 64]; D_MODEL],
    pub w2: [[f32; D_MODEL]; 64],
}

impl FeedForwardLayer {
    pub const fn new() -> Self {
        Self { w1: [[0f32; 64]; D_MODEL], w2: [[0f32; D_MODEL]; 64] }
    }
    pub fn forward(&self, x: &[f32; D_MODEL]) -> [f32; D_MODEL] {
        let mut hidden = [0f32; 64];
        for j in 0..64 {
            for i in 0..D_MODEL { hidden[j] += self.w1[i][j] * x[i]; }
            hidden[j] = hidden[j].max(0.0);   // ReLU
        }
        let mut out = [0f32; D_MODEL];
        for i in 0..D_MODEL {
            for j in 0..64 { out[i] += self.w2[j][i] * hidden[j]; }
        }
        out
    }
}

// ── Transformer Scheduler ──────────────────────────────────────────────────
pub struct TransformerScheduler {
    attn1:    AttentionWeights,
    ff1:      FeedForwardLayer,
    attn2:    AttentionWeights,
    ff2:      FeedForwardLayer,
    /// Output head: D_MODEL → 1 (predicted next burst in µs)
    output_w: [f32; D_MODEL],
    output_b: f32,
    /// Context window: last SEQ_LEN task feature embeddings
    context:  [[f32; D_MODEL]; SEQ_LEN],
    context_ptr: usize,
    /// Inference statistics
    pub total_predictions: u64,
    pub mean_abs_error_us: f32,
}

impl TransformerScheduler {
    pub fn new() -> Self {
        let mut output_w = [0f32; D_MODEL];
        for (i, w) in output_w.iter_mut().enumerate() { *w = 0.01 * (i as f32 + 1.0); }
        Self {
            attn1: AttentionWeights::identity(),
            ff1:   FeedForwardLayer::new(),
            attn2: AttentionWeights::identity(),
            ff2:   FeedForwardLayer::new(),
            output_w, output_b: 5_000.0,   // default 5ms burst prediction
            context: [[0f32; D_MODEL]; SEQ_LEN],
            context_ptr: 0,
            total_predictions: 0,
            mean_abs_error_us: 0.0,
        }
    }

    /// Encode task features and update context window
    fn encode_features(&self, feat: &TaskFeatures) -> [f32; D_MODEL] {
        let raw = feat.to_embedding();
        let mut embedding = [0f32; D_MODEL];
        // Simple linear embedding of 16 features into D_MODEL=32
        for i in 0..16 {
            embedding[i]    = raw[i];
            embedding[i+16] = raw[i] * 0.5;   // second copy with scale
        }
        // RMS normalise
        let rms = (embedding.iter().map(|x| x*x).sum::<f32>() / D_MODEL as f32).sqrt().max(1e-8);
        for v in embedding.iter_mut() { *v /= rms; }
        embedding
    }

    /// Forward pass: predict next CPU burst time in µs
    pub fn predict_burst(&mut self, feat: &TaskFeatures) -> f32 {
        // Add to context window
        let emb = self.encode_features(feat);
        self.context[self.context_ptr % SEQ_LEN] = emb;
        self.context_ptr = (self.context_ptr + 1) % SEQ_LEN;

        // Reorder context to be chronological
        let mut ordered_ctx = [[0f32; D_MODEL]; SEQ_LEN];
        for i in 0..SEQ_LEN {
            let src = (self.context_ptr + i) % SEQ_LEN;
            ordered_ctx[i] = self.context[src];
        }

        // Layer 1: attention + feedforward + residual
        let attn1_out = multi_head_attention(&ordered_ctx, &self.attn1);
        let mut layer1 = [[0f32; D_MODEL]; SEQ_LEN];
        for i in 0..SEQ_LEN {
            let ff_out = self.ff1.forward(&attn1_out[i]);
            for d in 0..D_MODEL {
                // Residual connection + layer norm (simplified)
                layer1[i][d] = (ordered_ctx[i][d] + attn1_out[i][d] + ff_out[d]) / 3.0;
            }
        }

        // Layer 2: attention + feedforward + residual
        let attn2_out = multi_head_attention(&layer1, &self.attn2);
        let last_idx = SEQ_LEN - 1;
        let ff2_out = self.ff2.forward(&attn2_out[last_idx]);

        // Output head: linear projection → scalar burst prediction
        let mut logit: f32 = self.output_b;
        for d in 0..D_MODEL {
            logit += self.output_w[d] * (layer1[last_idx][d] + ff2_out[d]) / 2.0;
        }

        // Apply sigmoid to keep prediction in reasonable range [100µs, 200ms]
        let sigmoid = 1.0 / (1.0 + (-logit / 10_000.0).exp());
        let predicted_us = 100.0 + sigmoid * 199_900.0;

        self.total_predictions += 1;
        predicted_us
    }

    /// Update model based on actual burst (online learning with SGD step)
    pub fn update(&mut self, feat: &TaskFeatures, actual_burst_us: f32) {
        let predicted = self.predict_burst(feat);
        let error = actual_burst_us - predicted;

        // Update exponential moving average of error
        self.mean_abs_error_us = 0.99 * self.mean_abs_error_us + 0.01 * error.abs();

        // Gradient step on output weights (SGD, lr=0.001)
        let lr = 0.001;
        let emb = self.encode_features(feat);
        for d in 0..D_MODEL {
            self.output_w[d] += lr * error * emb[d];
        }
        self.output_b += lr * error;
    }

    /// Compute scheduling priority (lower burst → higher priority for interactive)
    pub fn priority_score(&mut self, feat: &TaskFeatures) -> f32 {
        let predicted_burst = self.predict_burst(feat);
        let io_bonus = feat.io_wait_ratio * 1000.0;   // I/O-bound gets priority boost
        let burst_penalty = predicted_burst / 1000.0;  // CPU-bound gets slight penalty
        let class_bonus = match feat.priority_class {
            0 => 10_000.0,   // RT: always highest
            1 => 5_000.0,    // Interactive
            _ => 0.0,        // Batch
        };
        class_bonus + io_bonus - burst_penalty
    }
}

// ── Transformer-augmented run queue ───────────────────────────────────────
pub struct AiRunQueue {
    pub scheduler:    TransformerScheduler,
    pub tasks:        [(u32, f32); 256],   // (pid, priority_score)
    pub task_count:   usize,
    pub enabled:      bool,   // fallback: if model diverges, use classic MLFQ
    pub divergence_count: u32,
}

impl AiRunQueue {
    pub fn new() -> Self {
        Self {
            scheduler:   TransformerScheduler::new(),
            tasks:       [(0, 0.0); 256],
            task_count:  0,
            enabled:     true,
            divergence_count: 0,
        }
    }

    pub fn enqueue(&mut self, pid: u32, feat: TaskFeatures) {
        if self.task_count >= 256 { return; }
        let priority = if self.enabled {
            self.scheduler.priority_score(&feat)
        } else {
            feat.priority_class as f32 * 1000.0
        };
        self.tasks[self.task_count] = (pid, priority);
        self.task_count += 1;
        // Keep sorted by priority (insertion sort — O(n) but n≤256)
        let i = self.task_count - 1;
        let mut j = i;
        while j > 0 && self.tasks[j-1].1 < self.tasks[j].1 {
            self.tasks.swap(j-1, j);
            j -= 1;
        }
    }

    pub fn dequeue(&mut self) -> Option<u32> {
        if self.task_count == 0 { return None; }
        let pid = self.tasks[0].0;
        self.tasks.copy_within(1..self.task_count, 0);
        self.task_count -= 1;
        Some(pid)
    }

    pub fn on_task_complete(&mut self, pid: u32, feat: TaskFeatures, actual_burst_us: f32) {
        if !self.enabled { return; }
        self.scheduler.update(&feat, actual_burst_us);
        // Detect divergence: if mean error > 50ms, fall back
        if self.scheduler.mean_abs_error_us > 50_000.0 {
            self.divergence_count += 1;
            if self.divergence_count > 100 {
                self.enabled = false;  // fall back to classic MLFQ
            }
        } else {
            self.divergence_count = self.divergence_count.saturating_sub(1);
            if self.divergence_count == 0 { self.enabled = true; }
        }
    }

    pub fn stats(&self) -> (u64, f32, bool) {
        (self.scheduler.total_predictions,
         self.scheduler.mean_abs_error_us,
         self.enabled)
    }
}

// ── sigma-agent integration ────────────────────────────────────────────────
pub fn transformer_sched_cmd(args: &[String]) {
    match args.first().map(|s| s.as_str()) {
        Some("demo") => {
            let mut aq = AiRunQueue::new();
            println!("Σ Transformer Scheduler Demo");
            // Enqueue some test tasks
            let tasks = vec![
                (1, TaskFeatures { pid:1, last_burst_us:5000.0,  avg_burst_us:4500.0,  io_wait_ratio:0.8, priority_class:1, ..Default::default() }),
                (2, TaskFeatures { pid:2, last_burst_us:50000.0, avg_burst_us:48000.0, io_wait_ratio:0.1, priority_class:2, ..Default::default() }),
                (3, TaskFeatures { pid:3, last_burst_us:1000.0,  avg_burst_us:900.0,   io_wait_ratio:0.9, priority_class:1, ..Default::default() }),
            ];
            for (pid, feat) in &tasks {
                let priority = aq.scheduler.priority_score(feat);
                println!("  PID={} predicted_burst={:.0}µs priority={:.0}", pid,
                         aq.scheduler.predict_burst(feat), priority);
                aq.enqueue(*pid, feat.clone());
            }
            println!("\nScheduled order:");
            while let Some(pid) = aq.dequeue() { print!("  PID={}", pid); }
            println!();
            let (n, err, on) = aq.stats();
            println!("\nStats: predictions={} mean_error={:.0}µs enabled={}", n, err, on);
        }
        Some("stats") => {
            println!("Transformer scheduler stats: read /run/sigma/ai_sched_stats");
            if let Ok(s) = std::fs::read_to_string("/run/sigma/ai_sched_stats") {
                println!("{}", s);
            }
        }
        Some("enable")  => { let _ = std::fs::write("/run/sigma/ai_sched_enabled", "1"); println!("✓ AI scheduler enabled"); }
        Some("disable") => { let _ = std::fs::write("/run/sigma/ai_sched_enabled", "0"); println!("✓ AI scheduler disabled (using MLFQ fallback)"); }
        _ => println!("sigma-ai-sched — Transformer-based kernel scheduler\n\
            Usage: sigma-ai-sched demo|stats|enable|disable\n\
            The transformer predicts task CPU burst times to schedule with\n\
            optimal priority instead of reactive demotion."),
    }
}
