//! SigmaOS AI Developer Platform & Automation Suite
//! Zero-dependency #![no_std] implementation of AI orchestration, ML experiment tracking,
//! safety policy engine, signed model marketplace, multi-device model scheduling,
//! privacy prompt redaction, default-deny network policy, and OpenShell sandboxing.

#![no_std]

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// =========================================================================
// 1. MULTI-DEVICE MODEL SCHEDULING (LocalLlmOrchestrator)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceTarget {
    TpuSlot(u8),
    GpuMemory { vram_offset: u64, size_bytes: usize },
    CpuDemandPaging { use_thp: bool },
}

#[derive(Debug, Clone)]
pub struct ModelAllocation {
    pub model_name: String,
    pub target: DeviceTarget,
    pub allocated_size_bytes: usize,
}

pub struct LocalLlmOrchestrator {
    pub tpu_capacity_bytes: usize,
    pub tpu_used_bytes: usize,
    pub gpu_vram_capacity_bytes: usize,
    pub gpu_vram_used_bytes: usize,
    pub allocations: Vec<ModelAllocation>,
}

impl LocalLlmOrchestrator {
    pub fn new(tpu_bytes: usize, gpu_bytes: usize) -> Self {
        Self {
            tpu_capacity_bytes: tpu_bytes,
            tpu_used_bytes: 0,
            gpu_vram_capacity_bytes: gpu_bytes,
            gpu_vram_used_bytes: 0,
            allocations: Vec::new(),
        }
    }

    /// Attempts TPU allocation first, falls back to GPU with bounds checking,
    /// and defaults to CPU demand paging with Transparent Huge Pages (THP).
    pub fn schedule_model(&mut self, model_name: &str, required_bytes: usize) -> DeviceTarget {
        // 1. Try TPU slot allocation
        if self.tpu_used_bytes + required_bytes <= self.tpu_capacity_bytes {
            let slot_id = (self.allocations.len() % 4) as u8;
            self.tpu_used_bytes += required_bytes;
            let target = DeviceTarget::TpuSlot(slot_id);
            self.allocations.push(ModelAllocation {
                model_name: model_name.to_string(),
                target,
                allocated_size_bytes: required_bytes,
            });
            return target;
        }

        // 2. Fallback to GPU VRAM with strict bounds checking
        if self.gpu_vram_used_bytes + required_bytes <= self.gpu_vram_capacity_bytes {
            let offset = self.gpu_vram_used_bytes as u64;
            self.gpu_vram_used_bytes += required_bytes;
            let target = DeviceTarget::GpuMemory {
                vram_offset: offset,
                size_bytes: required_bytes,
            };
            self.allocations.push(ModelAllocation {
                model_name: model_name.to_string(),
                target,
                allocated_size_bytes: required_bytes,
            });
            return target;
        }

        // 3. Fallback to CPU virtual memory demand paging with Transparent Huge Pages (THP)
        let target = DeviceTarget::CpuDemandPaging { use_thp: true };
        self.allocations.push(ModelAllocation {
            model_name: model_name.to_string(),
            target,
            allocated_size_bytes: required_bytes,
        });
        target
    }
}

// =========================================================================
// 2. OPENSHELL SANDBOXING & PRIVACY GUARDRAILS
// =========================================================================

pub struct PrivacyRouter;

impl PrivacyRouter {
    /// Scans prompt bytes, redacting confidential markers (Credit Card numbers, Aadhaar IDs).
    pub fn redact_prompt(input: &str) -> String {
        let bytes = input.as_bytes();
        let mut sanitized = Vec::new();
        let mut i = 0;

        while i < bytes.len() {
            // Check for 16 contiguous digits first (Credit Card number)
            if i + 16 <= bytes.len() && bytes[i..i + 16].iter().all(|b| b.is_ascii_digit()) {
                sanitized.extend_from_slice(b"[REDACTED_CREDIT_CARD]");
                i += 16;
            } else if i + 12 <= bytes.len() && bytes[i..i + 12].iter().all(|b| b.is_ascii_digit()) {
                // Check for 12 contiguous digits (Aadhaar number)
                sanitized.extend_from_slice(b"[REDACTED_AADHAAR]");
                i += 12;
            } else {
                sanitized.push(bytes[i]);
                i += 1;
            }
        }

        String::from_utf8(sanitized).unwrap_or_else(|_| input.to_string())
    }
}

pub struct DefaultDenyNetworkPolicy {
    pub whitelisted_endpoints: Vec<String>,
}

impl DefaultDenyNetworkPolicy {
    pub fn new() -> Self {
        Self {
            whitelisted_endpoints: Vec::new(),
        }
    }

    pub fn allow_endpoint(&mut self, endpoint: &str) {
        self.whitelisted_endpoints.push(endpoint.to_string());
    }

    /// Default-denies outbound access from agent processes unless whitelisted.
    pub fn is_allowed(&self, target_endpoint: &str) -> bool {
        for allowed in &self.whitelisted_endpoints {
            if allowed == target_endpoint || target_endpoint.starts_with(allowed) {
                return true;
            }
        }
        false
    }
}

pub struct OpenShellAgentSandbox;

impl OpenShellAgentSandbox {
    /// Filters output commands against shell-escaping injection sequences (sudo, chmod, rm -rf).
    pub fn is_command_safe(command: &str) -> bool {
        let blacklisted = [
            "sudo",
            "chmod",
            "rm -rf",
            "dd if=/dev/zero",
            "mkfs",
            "chown",
            "> /dev/sd",
        ];
        for bad in blacklisted {
            if command.contains(bad) {
                return false;
            }
        }
        true
    }
}

// =========================================================================
// 3. ML EXPERIMENT TRACKER
// =========================================================================

#[derive(Debug, Clone)]
pub struct ExperimentRun {
    pub run_id: usize,
    pub name: String,
    pub params: Vec<(String, String)>,
    pub metrics: Vec<(String, f32)>,
    pub checkpoint_path: String,
    pub is_best: bool,
}

pub struct MlExperimentTracker {
    pub runs: Vec<ExperimentRun>,
    pub next_run_id: usize,
}

impl MlExperimentTracker {
    pub fn new() -> Self {
        Self {
            runs: Vec::new(),
            next_run_id: 1,
        }
    }

    pub fn start_run(&mut self, name: &str) -> usize {
        let id = self.next_run_id;
        self.next_run_id += 1;
        self.runs.push(ExperimentRun {
            run_id: id,
            name: name.to_string(),
            params: Vec::new(),
            metrics: Vec::new(),
            checkpoint_path: String::new(),
            is_best: false,
        });
        id
    }

    pub fn log_param(&mut self, run_id: usize, key: &str, value: &str) {
        if let Some(run) = self.runs.iter_mut().find(|r| r.run_id == run_id) {
            run.params.push((key.to_string(), value.to_string()));
        }
    }

    pub fn log_metric(&mut self, run_id: usize, key: &str, value: f32) {
        if let Some(run) = self.runs.iter_mut().find(|r| r.run_id == run_id) {
            run.metrics.push((key.to_string(), value));
        }
    }

    pub fn set_checkpoint(&mut self, run_id: usize, path: &str) {
        if let Some(run) = self.runs.iter_mut().find(|r| r.run_id == run_id) {
            run.checkpoint_path = path.to_string();
        }
    }

    pub fn mark_best_run(&mut self, run_id: usize) {
        for run in &mut self.runs {
            run.is_best = run.run_id == run_id;
        }
    }
}

// =========================================================================
// 4. AI SAFETY GUARDRAILS POLICY ENGINE
// =========================================================================

pub struct AiSafetyPolicyEngine {
    pub max_file_write_bytes: usize,
    pub enforce_sandbox: bool,
    pub blocked_commands: Vec<String>,
}

impl AiSafetyPolicyEngine {
    pub fn default_policy() -> Self {
        let mut blocked = Vec::new();
        blocked.push("rm -rf /".to_string());
        blocked.push("dd if=/dev/zero".to_string());
        blocked.push("sudo".to_string());
        blocked.push("chmod 777".to_string());
        Self {
            max_file_write_bytes: 100 * 1024 * 1024, // 100 MB default
            enforce_sandbox: true,
            blocked_commands: blocked,
        }
    }

    pub fn validate_file_write(&self, write_bytes: usize) -> bool {
        write_bytes <= self.max_file_write_bytes
    }

    pub fn validate_command(&self, cmd: &str) -> bool {
        for blocked in &self.blocked_commands {
            if cmd.contains(blocked) {
                return false;
            }
        }
        true
    }
}

// =========================================================================
// 5. SIGNED MODEL MARKETPLACE
// =========================================================================

#[derive(Debug, Clone)]
pub struct MarketplaceModel {
    pub model_id: String,
    pub name: String,
    pub expected_blake3_hash: [u8; 32],
    pub is_verified: bool,
}

pub struct SignedModelMarketplace {
    pub models: Vec<MarketplaceModel>,
}

impl SignedModelMarketplace {
    pub fn new() -> Self {
        Self { models: Vec::new() }
    }

    pub fn register_model(&mut self, id: &str, name: &str, blake3_hash: [u8; 32]) {
        self.models.push(MarketplaceModel {
            model_id: id.to_string(),
            name: name.to_string(),
            expected_blake3_hash: blake3_hash,
            is_verified: false,
        });
    }

    /// Computes a lightweight checksum over binary bytes and verifies against registered hash.
    pub fn verify_and_load(&mut self, id: &str, model_bytes: &[u8]) -> Result<(), &'static str> {
        let model = self
            .models
            .iter_mut()
            .find(|m| m.model_id == id)
            .ok_or("Model not found in marketplace")?;

        let computed_hash = compute_blake3_simulated(model_bytes);
        if computed_hash == model.expected_blake3_hash {
            model.is_verified = true;
            Ok(())
        } else {
            Err("Model signature verification failed: BLAKE3 hash mismatch!")
        }
    }
}

pub fn compute_blake3_simulated(data: &[u8]) -> [u8; 32] {
    let mut hash = [0u8; 32];
    for (i, &b) in data.iter().enumerate() {
        hash[i % 32] ^= b.wrapping_add(i as u8);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_llm_orchestrator_scheduling() {
        let mut orch = LocalLlmOrchestrator::new(1000, 2000);

        let target1 = orch.schedule_model("phi-3", 500);
        assert_eq!(target1, DeviceTarget::TpuSlot(0));

        let target2 = orch.schedule_model("llama-3-8b", 800); // Exceeds TPU (500+800 > 1000)
        assert!(matches!(target2, DeviceTarget::GpuMemory { .. }));

        let target3 = orch.schedule_model("mistral-7b", 3000); // Exceeds GPU VRAM (800+3000 > 2000)
        assert_eq!(target3, DeviceTarget::CpuDemandPaging { use_thp: true });
    }

    #[test]
    fn test_privacy_router_redaction() {
        let prompt = "My Aadhaar is 123456789012 and Card is 1111222233334444";
        let redacted = PrivacyRouter::redact_prompt(prompt);
        assert!(redacted.contains("[REDACTED_AADHAAR]"));
        assert!(redacted.contains("[REDACTED_CREDIT_CARD]"));
        assert!(!redacted.contains("123456789012"));
    }

    #[test]
    fn test_default_deny_network_policy() {
        let mut policy = DefaultDenyNetworkPolicy::new();
        policy.allow_endpoint("https://api.sigmaos.org");

        assert!(policy.is_allowed("https://api.sigmaos.org/v1/models"));
        assert!(!policy.is_allowed("https://malicious.external.site"));
    }

    #[test]
    fn test_openshell_agent_sandbox() {
        assert!(OpenShellAgentSandbox::is_command_safe("ls -la /var/log"));
        assert!(!OpenShellAgentSandbox::is_command_safe("sudo rm -rf /"));
    }

    #[test]
    fn test_ml_experiment_tracker() {
        let mut tracker = MlExperimentTracker::new();
        let id = tracker.start_run("resnet50_baseline");
        tracker.log_param(id, "lr", "0.001");
        tracker.log_metric(id, "accuracy", 0.94);
        tracker.mark_best_run(id);

        assert_eq!(tracker.runs[0].params[0].1, "0.001");
        assert!(tracker.runs[0].is_best);
    }

    #[test]
    fn test_ai_safety_policy_engine() {
        let policy = AiSafetyPolicyEngine::default_policy();
        assert!(policy.validate_file_write(10 * 1024 * 1024));
        assert!(!policy.validate_file_write(200 * 1024 * 1024));
        assert!(!policy.validate_command("dd if=/dev/zero of=/dev/sda"));
    }

    #[test]
    fn test_signed_model_marketplace() {
        let mut market = SignedModelMarketplace::new();
        let model_data = b"MODEL_WEIGHTS_BINARY_DATA";
        let hash = compute_blake3_simulated(model_data);

        market.register_model("model-01", "Sigma-Llama-3", hash);
        assert!(market.verify_and_load("model-01", model_data).is_ok());
        assert!(market.models[0].is_verified);
    }
}
