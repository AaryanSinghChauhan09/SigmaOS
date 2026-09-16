// SigmaOS AI Usage & Cost Tracker Engine
// Zero-dependency #![no_std] AI token accounting, compute resource monitoring, and quota enforcement engine

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelInferenceType {
    TextGeneration,
    CodeCompletion,
    VoiceSynthesis,
    ImageGeneration,
    Embedding,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiUsageRecord {
    pub session_id: String,
    pub model_name: String,
    pub inference_type: ModelInferenceType,
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub compute_duration_ms: u64,
    pub estimated_cost_cents: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiUsageQuota {
    pub max_tokens_per_day: usize,
    pub max_cost_cents_per_month: u64,
    pub alert_threshold_percent: u8,
}

impl Default for AiUsageQuota {
    fn default() -> Self {
        Self {
            max_tokens_per_day: 1_000_000,
            max_cost_cents_per_month: 5000, // $50.00
            alert_threshold_percent: 80,
        }
    }
}

pub struct AiUsageTrackerEngine {
    records: Vec<AiUsageRecord>,
    quota: AiUsageQuota,
}

impl AiUsageTrackerEngine {
    pub fn new(quota: AiUsageQuota) -> Self {
        Self {
            records: Vec::new(),
            quota,
        }
    }

    pub fn record_usage(
        &mut self,
        session_id: &str,
        model_name: &str,
        inference_type: ModelInferenceType,
        prompt_tokens: usize,
        completion_tokens: usize,
        compute_duration_ms: u64,
        timestamp: u64,
    ) -> &AiUsageRecord {
        let total_tokens = prompt_tokens + completion_tokens;
        // Estimated local compute cost (0.001 cents per 1K tokens)
        let estimated_cost = ((total_tokens as u64) / 1000).max(1);

        let record = AiUsageRecord {
            session_id: String::from(session_id),
            model_name: String::from(model_name),
            inference_type,
            prompt_tokens,
            completion_tokens,
            compute_duration_ms,
            estimated_cost_cents: estimated_cost,
            timestamp,
        };

        self.records.push(record);
        self.records.last().unwrap()
    }

    pub fn total_tokens_used(&self) -> usize {
        self.records.iter().map(|r| r.prompt_tokens + r.completion_tokens).sum()
    }

    pub fn total_cost_cents(&self) -> u64 {
        self.records.iter().map(|r| r.estimated_cost_cents).sum()
    }

    pub fn check_quota_warning(&self) -> (bool, u8) {
        let used = self.total_tokens_used();
        let max = self.quota.max_tokens_per_day;
        if max == 0 {
            return (false, 0);
        }

        let pct = ((used * 100) / max) as u8;
        let warning = pct >= self.quota.alert_threshold_percent;
        (warning, pct)
    }

    pub fn get_records(&self) -> &[AiUsageRecord] {
        &self.records
    }
}

impl Default for AiUsageTrackerEngine {
    fn default() -> Self {
        Self::new(AiUsageQuota::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_usage_tracker_engine() {
        let mut tracker = AiUsageTrackerEngine::default();
        tracker.record_usage(
            "sess_123",
            "Qwen-2.5-7B-Instruct",
            ModelInferenceType::TextGeneration,
            500_000,
            350_000,
            1200,
            1700000000,
        );

        assert_eq!(tracker.total_tokens_used(), 850_000);
        let (warn, pct) = tracker.check_quota_warning();
        assert!(warn);
        assert_eq!(pct, 85);
    }
}
