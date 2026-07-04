// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/ai/sigma_ai.rs — sigma-ai: On-device TinyLlama Inference Daemon
// Language: Rust (std) — OOP via InferenceDaemon + Model trait

use std::collections::VecDeque;

// ── Token types ───────────────────────────────────────────────────────────────

pub type TokenId = u32;

#[derive(Clone, Debug)]
pub struct Token { pub id: TokenId, pub text: String, pub logprob: f32 }

// ── Tokenizer (BPE-style, cleanroom, vocabulary from GGUF metadata) ──────────

pub struct Tokenizer {
    vocab:    Vec<String>,
    scores:   Vec<f32>,
    eos_id:   TokenId,
    bos_id:   TokenId,
}

impl Tokenizer {
    pub fn new() -> Self {
        // Placeholder vocabulary — real vocab loaded from GGUF metadata block
        Self {
            vocab:  vec!["<unk>".to_owned(), "<s>".to_owned(), "</s>".to_owned()],
            scores: vec![0.0, 0.0, 0.0],
            eos_id: 2, bos_id: 1,
        }
    }

    pub fn encode(&self, text: &str) -> Vec<TokenId> {
        // Simplified: byte-pair splitting (real: sentencepiece BPE)
        let mut tokens = vec![self.bos_id];
        for word in text.split_whitespace() {
            // Look up word in vocab
            let mut found = false;
            for (i, v) in self.vocab.iter().enumerate() {
                if v == word { tokens.push(i as TokenId); found = true; break; }
            }
            if !found {
                // Unknown token: encode as character-level fallback
                for c in word.chars() {
                    let s = c.to_string();
                    if let Some(i) = self.vocab.iter().position(|v| v == &s) {
                        tokens.push(i as TokenId);
                    } else {
                        tokens.push(0); // <unk>
                    }
                }
            }
        }
        tokens
    }

    pub fn decode(&self, token: TokenId) -> &str {
        self.vocab.get(token as usize).map(|s| s.as_str()).unwrap_or("<unk>")
    }

    pub fn is_eos(&self, id: TokenId) -> bool { id == self.eos_id }
}

// ── Quantized Tensor (GGUF-compatible, Q4_0 simplified) ──────────────────────

#[derive(Clone)]
pub struct QTensor {
    pub rows: usize,
    pub cols: usize,
    data:     Vec<i8>,   // quantized weights
    scale:    Vec<f32>,  // per-row scale factors
}

impl QTensor {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self { rows, cols, data: vec![0i8; rows * cols], scale: vec![1.0f32; rows] }
    }

    /// Dequantize row i into f32 buffer
    pub fn dequant_row(&self, row: usize, out: &mut Vec<f32>) {
        out.resize(self.cols, 0.0);
        let s = self.scale[row % self.scale.len()];
        let base = row * self.cols;
        for j in 0..self.cols { out[j] = self.data[base + j] as f32 * s; }
    }
}

// ── Attention Layer ───────────────────────────────────────────────────────────

pub struct AttentionLayer {
    pub wq: QTensor, pub wk: QTensor, pub wv: QTensor, pub wo: QTensor,
    pub n_heads: usize, pub head_dim: usize,
}

impl AttentionLayer {
    pub fn new(dim: usize, n_heads: usize) -> Self {
        let hd = dim / n_heads;
        Self {
            wq: QTensor::zeros(dim, dim), wk: QTensor::zeros(dim, dim),
            wv: QTensor::zeros(dim, dim), wo: QTensor::zeros(dim, dim),
            n_heads, head_dim: hd,
        }
    }

    /// Simplified single-head attention (real: multi-head with RoPE)
    pub fn forward(&self, x: &[f32], out: &mut Vec<f32>) {
        let dim = x.len();
        out.resize(dim, 0.0);
        let mut q = vec![0.0f32; dim];
        let mut k = vec![0.0f32; dim];
        let mut v = vec![0.0f32; dim];
        let mut row_buf = Vec::new();
        // Q = Wq * x
        for i in 0..dim {
            self.wq.dequant_row(i, &mut row_buf);
            q[i] = row_buf.iter().zip(x).map(|(a,b)| a * b).sum();
        }
        // K = Wk * x
        for i in 0..dim {
            self.wk.dequant_row(i, &mut row_buf);
            k[i] = row_buf.iter().zip(x).map(|(a,b)| a * b).sum();
        }
        // V = Wv * x
        for i in 0..dim {
            self.wv.dequant_row(i, &mut row_buf);
            v[i] = row_buf.iter().zip(x).map(|(a,b)| a * b).sum();
        }
        // Scaled dot-product attention: score = softmax(q·k / sqrt(d)) * v
        let scale = (self.head_dim as f32).sqrt();
        let score: f32 = q.iter().zip(&k).map(|(a,b)| a * b).sum::<f32>() / scale;
        let attn = score.exp() / (score.exp() + 1.0); // simplified softmax
        // out = attn * v
        for i in 0..dim { out[i] = attn * v[i]; }
    }
}

// ── Feed-Forward Network ─────────────────────────────────────────────────────

pub struct FeedForward { pub w1: QTensor, pub w2: QTensor, pub w3: QTensor }

impl FeedForward {
    pub fn new(dim: usize, ff_dim: usize) -> Self {
        Self {
            w1: QTensor::zeros(ff_dim, dim),
            w2: QTensor::zeros(dim, ff_dim),
            w3: QTensor::zeros(ff_dim, dim),
        }
    }

    fn silu(x: f32) -> f32 { x / (1.0 + (-x).exp()) }

    pub fn forward(&self, x: &[f32], out: &mut Vec<f32>) {
        let ff_dim = self.w1.rows;
        let mut h1 = vec![0.0f32; ff_dim];
        let mut h3 = vec![0.0f32; ff_dim];
        let mut row = Vec::new();
        for i in 0..ff_dim {
            self.w1.dequant_row(i, &mut row);
            h1[i] = Self::silu(row.iter().zip(x).map(|(a,b)| a*b).sum());
            self.w3.dequant_row(i, &mut row);
            h3[i] = row.iter().zip(x).map(|(a,b)| a*b).sum();
        }
        let dim = x.len();
        out.resize(dim, 0.0);
        for i in 0..dim {
            self.w2.dequant_row(i, &mut row);
            out[i] = row.iter().enumerate().map(|(j,&w)| w * h1[j] * h3[j]).sum();
        }
    }
}

// ── Transformer Block ─────────────────────────────────────────────────────────

pub struct TransformerBlock {
    attn: AttentionLayer,
    ff:   FeedForward,
    dim:  usize,
}

impl TransformerBlock {
    pub fn new(dim: usize, n_heads: usize, ff_dim: usize) -> Self {
        Self { attn: AttentionLayer::new(dim, n_heads), ff: FeedForward::new(dim, ff_dim), dim }
    }

    fn rms_norm(x: &[f32], out: &mut Vec<f32>) {
        let eps = 1e-6f32;
        let mean_sq = x.iter().map(|v| v * v).sum::<f32>() / x.len() as f32;
        let scale = (mean_sq + eps).sqrt().recip();
        out.resize(x.len(), 0.0);
        for (i, &v) in x.iter().enumerate() { out[i] = v * scale; }
    }

    pub fn forward(&self, x: &[f32], out: &mut Vec<f32>) {
        let mut normed = Vec::new();
        let mut attn_out = Vec::new();
        let mut ff_out = Vec::new();
        // Pre-norm + attention with residual
        Self::rms_norm(x, &mut normed);
        self.attn.forward(&normed, &mut attn_out);
        let mut residual: Vec<f32> = x.iter().zip(&attn_out).map(|(a,b)| a + b).collect();
        // Pre-norm + FFN with residual
        Self::rms_norm(&residual, &mut normed);
        self.ff.forward(&normed, &mut ff_out);
        out.resize(self.dim, 0.0);
        for i in 0..self.dim { out[i] = residual[i] + ff_out.get(i).copied().unwrap_or(0.0); }
    }
}

// ── Language Model ────────────────────────────────────────────────────────────

pub struct LanguageModel {
    pub dim:    usize,
    pub n_heads: usize,
    pub n_layers: usize,
    pub vocab_size: usize,
    pub max_seq: usize,
    embed:   Vec<Vec<f32>>,
    layers:  Vec<TransformerBlock>,
    lm_head: QTensor,
    tokenizer: Tokenizer,
}

impl LanguageModel {
    pub fn new(dim: usize, n_heads: usize, n_layers: usize, vocab_size: usize) -> Self {
        let ff_dim = dim * 4;
        let mut layers = Vec::new();
        for _ in 0..n_layers { layers.push(TransformerBlock::new(dim, n_heads, ff_dim)); }
        Self {
            dim, n_heads, n_layers, vocab_size,
            max_seq: 2048,
            embed: vec![vec![0.0f32; dim]; vocab_size],
            layers,
            lm_head: QTensor::zeros(vocab_size, dim),
            tokenizer: Tokenizer::new(),
        }
    }

    fn softmax(logits: &mut Vec<f32>) {
        let max = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let sum: f32 = logits.iter_mut().map(|v| { *v = (*v - max).exp(); *v }).sum();
        for v in logits.iter_mut() { *v /= sum; }
    }

    pub fn sample_greedy(&self, logits: &[f32]) -> TokenId {
        logits.iter().enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(core::cmp::Ordering::Equal))
            .map(|(i, _)| i as TokenId).unwrap_or(0)
    }

    pub fn forward_token(&self, token: TokenId, out: &mut Vec<f32>) {
        // Embed token
        let tok = token as usize % self.vocab_size;
        let mut hidden = self.embed[tok].clone();
        let mut block_out = Vec::new();
        // Pass through all layers
        for layer in &self.layers {
            layer.forward(&hidden, &mut block_out);
            hidden = block_out.clone();
        }
        // Project to vocab
        out.resize(self.vocab_size, 0.0);
        let mut row = Vec::new();
        for i in 0..self.vocab_size.min(self.lm_head.rows) {
            self.lm_head.dequant_row(i, &mut row);
            out[i] = row.iter().zip(&hidden).map(|(a,b)| a * b).sum();
        }
    }

    pub fn generate(&self, prompt: &str, max_tokens: usize) -> String {
        let mut tokens = self.tokenizer.encode(prompt);
        let mut result = prompt.to_owned();
        let mut logits = Vec::new();
        for _ in 0..max_tokens {
            let last = *tokens.last().unwrap_or(&1);
            self.forward_token(last, &mut logits);
            let next = self.sample_greedy(&logits);
            if self.tokenizer.is_eos(next) { break; }
            result.push_str(self.tokenizer.decode(next));
            tokens.push(next);
            if tokens.len() > self.max_seq { tokens.remove(0); }
        }
        result
    }
}

// ── Inference Daemon ──────────────────────────────────────────────────────────

pub struct InferenceDaemon {
    model:   LanguageModel,
    history: VecDeque<String>,
}

impl InferenceDaemon {
    pub fn new() -> Self {
        // TinyLlama-1.1B dimensions (placeholder weights — loaded from GGUF in prod)
        Self {
            model:   LanguageModel::new(2048, 32, 22, 32000),
            history: VecDeque::with_capacity(16),
        }
    }

    pub fn complete(&mut self, prompt: &str, max_tokens: usize) -> String {
        self.history.push_back(prompt.to_owned());
        if self.history.len() > 8 { self.history.pop_front(); }
        self.model.generate(prompt, max_tokens)
    }

    pub fn clear_history(&mut self) { self.history.clear(); }
}
