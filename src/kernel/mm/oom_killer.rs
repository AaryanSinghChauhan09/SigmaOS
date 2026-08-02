#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

use crate::kernel::scheduler::Process;
/// SigmaOS OOM (Out Of Memory) Killer implementation
/// Calculates badness score of processes and kills the worst culprit
use crate::klib::HashMap;

pub struct OomKiller {
    oom_scores_adj: HashMap<u64, i32>,
}

impl OomKiller {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        OomKiller {
            oom_scores_adj: HashMap::new(),
        }
    }

    pub fn set_score_adj(&mut self, pid: u64, adj: i32) {
        // Range -1000 (never kill) to 1000 (always kill first)
        let safe_adj = adj.clamp(-1000, 1000);
        self.oom_scores_adj.insert(pid, safe_adj);
    }

    pub fn get_score_adj(&self, pid: u64) -> i32 {
        self.oom_scores_adj.get(&pid).copied().unwrap_or(0)
    }

    pub fn select_victim(
        &self,
        processes: &[Process],
        memory_usages: &HashMap<u64, u64>,
    ) -> Option<u64> {
        let mut worst_pid = None;
        let mut worst_points = -99999i64;

        for p in processes {
            let usage = memory_usages.get(&p.pid).copied().unwrap_or(0);

            // Base score is memory usage in kilobytes
            let mut points = (usage / 1024) as i64;

            // Adjust based on Priority
            points -= match p.priority {
                crate::kernel::scheduler::Priority::Idle => 0,
                crate::kernel::scheduler::Priority::Low => 50,
                crate::kernel::scheduler::Priority::Normal => 100,
                crate::kernel::scheduler::Priority::High => 500,
                crate::kernel::scheduler::Priority::Realtime => 1000,
            };

            // User-configured oom_score_adj adjustment
            let adj = self.get_score_adj(p.pid) as i64;
            if adj == -1000 {
                // Ignore completely
                continue;
            }
            points += adj;

            if points > worst_points {
                worst_points = points;
                worst_pid = Some(p.pid);
            }
        }

        worst_pid
    }
}

impl Default for OomKiller {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::scheduler::Priority;

    #[test]
    fn test_oom_killer_selection() {
        let mut oom = OomKiller::new();

        let p1 = Process::new(101, "browser".to_string(), Priority::Normal);
        let p2 = Process::new(102, "db".to_string(), Priority::High);

        let mut usages = HashMap::new();
        usages.insert(101, 1024 * 1024 * 50); // 50MB
        usages.insert(102, 1024 * 1024 * 100); // 100MB

        // By default, db uses more memory, but it has high priority. Let's see:
        let processes = vec![p1, p2];
        let victim = oom.select_victim(&processes, &usages).unwrap();
        // browser has Normal (-100 pts) while db has High (-500 pts).
        // browser pts = 50 * 1024 - 100 = 51100
        // db pts = 100 * 1024 - 500 = 101900 -> db still gets selected because memory is twice as large.
        assert_eq!(victim, 102);

        // Adjust DB adj to never kill
        oom.set_score_adj(102, -1000);
        let new_victim = oom.select_victim(&processes, &usages).unwrap();
        assert_eq!(new_victim, 101); // Now browser is chosen
    }
}
