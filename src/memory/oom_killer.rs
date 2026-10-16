// SigmaOS Cgroup-Aware Out-of-Memory (OOM) Killer
// Calculates oom_badness scores and selects victim processes during memory exhaustion.

#![allow(dead_code)]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub const OOM_SCORE_ADJ_MIN: i32 = -1000;
pub const OOM_SCORE_ADJ_MAX: i32 = 1000;

/// Process candidate for OOM consideration
#[derive(Debug, Clone)]
pub struct OomProcessCandidate {
    pub pid: u32,
    pub name: String,
    pub uid: u32,
    pub rss_pages: usize,
    pub swap_pages: usize,
    pub oom_score_adj: i32, // -1000 = unkillable, +1000 = always first victim
    pub cgroup_id: u32,
    pub is_kernel_task: bool,
}

/// Victim selected by the OOM killer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OomVictim {
    pub pid: u32,
    pub name: String,
    pub points: u64,
    pub freed_pages_estimate: usize,
}

/// Out-of-Memory Killer Engine
#[derive(Debug, Clone, Default)]
pub struct OomKiller {
    pub total_ram_pages: usize,
    pub oom_kill_count: u64,
    pub last_victim_pid: Option<u32>,
}

impl OomKiller {
    pub fn new(total_ram_pages: usize) -> Self {
        Self {
            total_ram_pages,
            oom_kill_count: 0,
            last_victim_pid: None,
        }
    }

    /// Calculate oom_badness score (0 to 1000 proportional to total RAM)
    pub fn calculate_badness(&self, proc: &OomProcessCandidate) -> u64 {
        // PID 1 (init) and kernel threads are strictly immune
        if proc.pid <= 1 || proc.is_kernel_task || proc.oom_score_adj <= OOM_SCORE_ADJ_MIN {
            return 0;
        }

        let memory_usage = proc.rss_pages + proc.swap_pages;
        if memory_usage == 0 {
            return 0;
        }

        // Base points: memory usage normalized against total RAM (in permille 0..1000)
        let base_points = if self.total_ram_pages > 0 {
            ((memory_usage as u64) * 1000) / (self.total_ram_pages as u64)
        } else {
            memory_usage as u64
        };

        // Root/system process discount (3% discount)
        let root_adjusted = if proc.uid == 0 {
            base_points.saturating_sub(30)
        } else {
            base_points
        };

        // Apply oom_score_adj offset
        let final_points = (root_adjusted as i64) + (proc.oom_score_adj as i64);
        final_points.max(1) as u64
    }

    /// Select the best victim from a candidate list
    pub fn select_victim(&mut self, candidates: &[OomProcessCandidate]) -> Option<OomVictim> {
        let mut best_victim: Option<OomVictim> = None;
        let mut max_points = 0u64;

        for proc in candidates {
            let points = self.calculate_badness(proc);
            if points > max_points {
                max_points = points;
                best_victim = Some(OomVictim {
                    pid: proc.pid,
                    name: proc.name.clone(),
                    points,
                    freed_pages_estimate: proc.rss_pages + proc.swap_pages,
                });
            }
        }

        if let Some(ref victim) = best_victim {
            self.last_victim_pid = Some(victim.pid);
            self.oom_kill_count += 1;
        }

        best_victim
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn test_oom_killer_victim_selection() {
        let mut killer = OomKiller::new(100_000); // 100k pages = 400MB
        let candidates = [
            OomProcessCandidate {
                pid: 1, // init
                name: "sigma-init".to_string(),
                uid: 0,
                rss_pages: 5000,
                swap_pages: 0,
                oom_score_adj: 0,
                cgroup_id: 0,
                is_kernel_task: false,
            },
            OomProcessCandidate {
                pid: 100, // Background daemon
                name: "syslogd".to_string(),
                uid: 0,
                rss_pages: 2000,
                swap_pages: 0,
                oom_score_adj: -500,
                cgroup_id: 1,
                is_kernel_task: false,
            },
            OomProcessCandidate {
                pid: 500, // Leaking app
                name: "leaky-browser".to_string(),
                uid: 1000,
                rss_pages: 60000,
                swap_pages: 10000,
                oom_score_adj: 200,
                cgroup_id: 2,
                is_kernel_task: false,
            },
        ];

        let victim = killer
            .select_victim(&candidates)
            .expect("Should pick victim");
        assert_eq!(victim.pid, 500);
        assert_eq!(victim.name, "leaky-browser");
    }
}
