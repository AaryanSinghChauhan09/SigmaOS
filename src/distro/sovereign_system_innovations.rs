extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// =========================================================================
// 1. EVENT-DRIVEN ADAPTIVE WORKLOAD SCHEDULER
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkloadType {
    RealTime,
    Interactive,
    Batch,
    ComputeHeavy,
}

#[derive(Debug, Clone)]
pub struct EventWorkloadTask {
    pub pid: u32,
    pub name: String,
    pub workload_type: WorkloadType,
    pub event_count: u64,
    pub last_latency_us: u64,
    pub energy_weight: u8,
}

pub struct PolicyAdaptiveEventScheduler {
    pub active_tasks: BTreeMap<u32, EventWorkloadTask>,
    pub current_workload_mode: WorkloadType,
    pub total_events_processed: u64,
}

impl PolicyAdaptiveEventScheduler {
    pub fn new() -> Self {
        Self {
            active_tasks: BTreeMap::new(),
            current_workload_mode: WorkloadType::Interactive,
            total_events_processed: 0,
        }
    }

    pub fn register_task(&mut self, pid: u32, name: &str, workload_type: WorkloadType) {
        self.active_tasks.insert(
            pid,
            EventWorkloadTask {
                pid,
                name: name.to_string(),
                workload_type,
                event_count: 0,
                last_latency_us: 10,
                energy_weight: 50,
            },
        );
    }

    pub fn trigger_event(&mut self, pid: u32, latency_us: u64) -> Result<(), &'static str> {
        if let Some(task) = self.active_tasks.get_mut(&pid) {
            task.event_count += 1;
            task.last_latency_us = latency_us;
            self.total_events_processed += 1;

            // Dynamically adapt system workload mode if RealTime events dominate
            if task.workload_type == WorkloadType::RealTime && latency_us < 50 {
                self.current_workload_mode = WorkloadType::RealTime;
            }
            Ok(())
        } else {
            Err("Scheduler: Task PID not found")
        }
    }

    pub fn select_next_task(&self) -> Option<&EventWorkloadTask> {
        self.active_tasks.values().min_by_key(|t| {
            let priority_bonus = if t.workload_type == self.current_workload_mode {
                0
            } else {
                100
            };
            t.last_latency_us + (priority_bonus as u64)
        })
    }
}

impl Default for PolicyAdaptiveEventScheduler {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. UNIFIED EXTENSIBLE SYSCALL INTERCEPTION HOOK GATE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookAction {
    Allow,
    Deny,
    AuditLog,
    Redirect(u32),
}

pub struct ExtensibleSyscallHookGate {
    pub blocked_syscalls: Vec<u32>,
    pub audit_log: Vec<(u32, u32, String)>, // (pid, syscall_nr, log)
    pub ebp_hooks_active: bool,
}

impl ExtensibleSyscallHookGate {
    pub fn new() -> Self {
        Self {
            blocked_syscalls: Vec::new(),
            audit_log: Vec::new(),
            ebp_hooks_active: true,
        }
    }

    pub fn block_syscall(&mut self, syscall_nr: u32) {
        if !self.blocked_syscalls.contains(&syscall_nr) {
            self.blocked_syscalls.push(syscall_nr);
        }
    }

    pub fn evaluate_syscall(&mut self, pid: u32, syscall_nr: u32) -> HookAction {
        if self.blocked_syscalls.contains(&syscall_nr) {
            self.audit_log.push((
                pid,
                syscall_nr,
                format!("Syscall {} DENIED for PID {}", syscall_nr, pid),
            ));
            HookAction::Deny
        } else {
            self.audit_log.push((
                pid,
                syscall_nr,
                format!("Syscall {} ALLOWED for PID {}", syscall_nr, pid),
            ));
            HookAction::Allow
        }
    }
}

impl Default for ExtensibleSyscallHookGate {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. VISUAL SANDBOX POLICY MANAGER (SELinux / Capsicum GUI Parity)
// =========================================================================

#[derive(Debug, Clone)]
pub struct VisualPolicyRule {
    pub app_name: String,
    pub allow_network: bool,
    pub allow_filesystem_write: bool,
    pub allow_camera_microphone: bool,
    pub restricted_paths: Vec<String>,
}

pub struct VisualSandboxPolicyManager {
    pub policies: BTreeMap<String, VisualPolicyRule>,
}

impl VisualSandboxPolicyManager {
    pub fn new() -> Self {
        Self {
            policies: BTreeMap::new(),
        }
    }

    pub fn set_policy(&mut self, rule: VisualPolicyRule) {
        self.policies.insert(rule.app_name.clone(), rule);
    }

    pub fn check_permission(&self, app_name: &str, operation: &str) -> bool {
        if let Some(rule) = self.policies.get(app_name) {
            match operation {
                "network" => rule.allow_network,
                "fs_write" => rule.allow_filesystem_write,
                "media" => rule.allow_camera_microphone,
                _ => false,
            }
        } else {
            true // Default permissive
        }
    }
}

impl Default for VisualSandboxPolicyManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. UNIFIED FIREWALL & VPN GUI ORCHESTRATOR (PF + nftables Parity)
// =========================================================================

#[derive(Debug, Clone)]
pub struct FirewallRule {
    pub rule_id: u32,
    pub proto: String,
    pub port: u16,
    pub action: String, // "pass" or "drop"
}

pub struct UnifiedFirewallVpnOrchestrator {
    pub rules: Vec<FirewallRule>,
    pub vpn_active: bool,
    pub vpn_endpoint: String,
}

impl UnifiedFirewallVpnOrchestrator {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            vpn_active: false,
            vpn_endpoint: String::from("pqc.sigmaos.org:51820"),
        }
    }

    pub fn add_rule(&mut self, id: u32, proto: &str, port: u16, action: &str) {
        self.rules.push(FirewallRule {
            rule_id: id,
            proto: proto.to_string(),
            port,
            action: action.to_string(),
        });
    }

    pub fn toggle_vpn(&mut self, enable: bool) {
        self.vpn_active = enable;
    }

    pub fn evaluate_packet(&self, port: u16) -> bool {
        for rule in &self.rules {
            if rule.port == port && rule.action == "drop" {
                return false;
            }
        }
        true
    }
}

impl Default for UnifiedFirewallVpnOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. ADAPTIVE WINDOW MANAGER OVERLAY CONTROLLER
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WmLayoutMode {
    Tiling,
    Stacking,
    Floating,
    ComplianceDashboard,
}

pub struct AdaptiveWmOverlayController {
    pub current_mode: WmLayoutMode,
    pub window_count: usize,
}

impl AdaptiveWmOverlayController {
    pub fn new() -> Self {
        Self {
            current_mode: WmLayoutMode::Tiling,
            window_count: 0,
        }
    }

    pub fn switch_layout(&mut self, mode: WmLayoutMode) {
        self.current_mode = mode;
    }

    pub fn auto_adapt_layout(&mut self, active_windows: usize) {
        self.window_count = active_windows;
        if active_windows > 4 {
            self.current_mode = WmLayoutMode::Tiling;
        } else if active_windows > 0 {
            self.current_mode = WmLayoutMode::Stacking;
        } else {
            self.current_mode = WmLayoutMode::ComplianceDashboard;
        }
    }
}

impl Default for AdaptiveWmOverlayController {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 6. GAMIFIED SYSTEM HEALTH TELEMETRY MONITOR
// =========================================================================

pub struct GamifiedSystemMonitor {
    pub cpu_health_score: u8,
    pub memory_health_score: u8,
    pub thermal_score: u8,
    pub system_rank: String,
    pub achievements_unlocked: Vec<String>,
}

impl GamifiedSystemMonitor {
    pub fn new() -> Self {
        Self {
            cpu_health_score: 100,
            memory_health_score: 100,
            thermal_score: 100,
            system_rank: String::from("Sovereign S-Class Node"),
            achievements_unlocked: vec![String::from("Zero Dependency Milestone")],
        }
    }

    pub fn calculate_overall_score(&self) -> u8 {
        ((self.cpu_health_score as u32
            + self.memory_health_score as u32
            + self.thermal_score as u32)
            / 3) as u8
    }

    pub fn update_telemetry(&mut self, cpu_usage: u8, temp_c: u8) {
        self.cpu_health_score = 100u8.saturating_sub(cpu_usage);
        self.thermal_score = if temp_c > 80 {
            50
        } else {
            100u8.saturating_sub(temp_c / 2)
        };

        if self.calculate_overall_score() > 90
            && !self
                .achievements_unlocked
                .contains(&"Peak Efficiency".to_string())
        {
            self.achievements_unlocked
                .push("Peak Efficiency".to_string());
        }
    }
}

impl Default for GamifiedSystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_event_scheduler() {
        let mut sched = PolicyAdaptiveEventScheduler::new();
        sched.register_task(101, "realtime_audio", WorkloadType::RealTime);
        assert!(sched.trigger_event(101, 20).is_ok());
        assert_eq!(sched.current_workload_mode, WorkloadType::RealTime);
        assert_eq!(sched.select_next_task().unwrap().pid, 101);
    }

    #[test]
    fn test_extensible_syscall_hook_gate() {
        let mut gate = ExtensibleSyscallHookGate::new();
        gate.block_syscall(59); // execve
        assert_eq!(gate.evaluate_syscall(10, 59), HookAction::Deny);
        assert_eq!(gate.evaluate_syscall(10, 1), HookAction::Allow);
    }

    #[test]
    fn test_visual_sandbox_policy_manager() {
        let mut mgr = VisualSandboxPolicyManager::new();
        mgr.set_policy(VisualPolicyRule {
            app_name: "untrusted_app".to_string(),
            allow_network: false,
            allow_filesystem_write: true,
            allow_camera_microphone: false,
            restricted_paths: vec!["/etc".to_string()],
        });

        assert!(!mgr.check_permission("untrusted_app", "network"));
        assert!(mgr.check_permission("untrusted_app", "fs_write"));
    }

    #[test]
    fn test_unified_firewall_vpn_orchestrator() {
        let mut fw = UnifiedFirewallVpnOrchestrator::new();
        fw.add_rule(1, "tcp", 80, "drop");
        assert!(!fw.evaluate_packet(80));
        assert!(fw.evaluate_packet(443));

        fw.toggle_vpn(true);
        assert!(fw.vpn_active);
    }

    #[test]
    fn test_adaptive_wm_overlay_controller() {
        let mut wm = AdaptiveWmOverlayController::new();
        wm.auto_adapt_layout(6);
        assert_eq!(wm.current_mode, WmLayoutMode::Tiling);

        wm.auto_adapt_layout(0);
        assert_eq!(wm.current_mode, WmLayoutMode::ComplianceDashboard);
    }

    #[test]
    fn test_gamified_system_monitor() {
        let mut monitor = GamifiedSystemMonitor::new();
        monitor.update_telemetry(0, 10); // Zero load, cool temp
        assert!(monitor.calculate_overall_score() >= 90);
        assert!(monitor
            .achievements_unlocked
            .contains(&"Peak Efficiency".to_string()));
    }
}
