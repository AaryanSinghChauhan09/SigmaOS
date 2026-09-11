// SigmaOS System Snapshot Schedule Module
// Zero-dependency #![no_std] automated snapshot scheduler & retention policy manager

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotFrequency {
    Hourly,
    Daily,
    Weekly,
    Monthly,
    OnBoot,
    CustomIntervalSeconds(u64),
}

impl SnapshotFrequency {
    pub fn interval_seconds(&self) -> u64 {
        match self {
            SnapshotFrequency::Hourly => 3600,
            SnapshotFrequency::Daily => 86400,
            SnapshotFrequency::Weekly => 604800,
            SnapshotFrequency::Monthly => 2592000,
            SnapshotFrequency::OnBoot => 0,
            SnapshotFrequency::CustomIntervalSeconds(sec) => *sec,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetentionPolicy {
    pub max_hourly: usize,
    pub max_daily: usize,
    pub max_weekly: usize,
    pub max_monthly: usize,
    pub min_free_disk_space_mb: u64,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            max_hourly: 24,
            max_daily: 7,
            max_weekly: 4,
            max_monthly: 12,
            min_free_disk_space_mb: 1024,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotJob {
    pub id: u64,
    pub name: String,
    pub target_volume: String,
    pub frequency: SnapshotFrequency,
    pub last_run_timestamp: u64,
    pub next_run_timestamp: u64,
    pub retention: RetentionPolicy,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatedSnapshotRecord {
    pub job_id: u64,
    pub snapshot_id: String,
    pub volume: String,
    pub timestamp: u64,
    pub size_bytes: u64,
}

pub struct ScheduledSnapshotEngine {
    jobs: Vec<SnapshotJob>,
    history: Vec<CreatedSnapshotRecord>,
    next_job_id: u64,
}

impl ScheduledSnapshotEngine {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            history: Vec::new(),
            next_job_id: 1,
        }
    }

    pub fn add_job(
        &mut self,
        name: &str,
        target_volume: &str,
        frequency: SnapshotFrequency,
        retention: RetentionPolicy,
        current_time: u64,
    ) -> u64 {
        let id = self.next_job_id;
        self.next_job_id += 1;

        let next_run = current_time + frequency.interval_seconds();
        let job = SnapshotJob {
            id,
            name: String::from(name),
            target_volume: String::from(target_volume),
            frequency,
            last_run_timestamp: current_time,
            next_run_timestamp: next_run,
            retention,
            active: true,
        };

        self.jobs.push(job);
        id
    }

    pub fn remove_job(&mut self, job_id: u64) -> bool {
        if let Some(pos) = self.jobs.iter().position(|j| j.id == job_id) {
            self.jobs.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn list_jobs(&self) -> &[SnapshotJob] {
        &self.jobs
    }

    pub fn get_job_mut(&mut self, job_id: u64) -> Option<&mut SnapshotJob> {
        self.jobs.iter_mut().find(|j| j.id == job_id)
    }

    pub fn evaluate_and_run_schedules(&mut self, current_time: u64) -> Vec<CreatedSnapshotRecord> {
        let mut created = Vec::new();

        for job in self.jobs.iter_mut() {
            if !job.active {
                continue;
            }

            if current_time >= job.next_run_timestamp {
                let snap_id = format!("snap_{}_{}_{}", job.id, job.target_volume, current_time);
                let record = CreatedSnapshotRecord {
                    job_id: job.id,
                    snapshot_id: snap_id,
                    volume: job.target_volume.clone(),
                    timestamp: current_time,
                    size_bytes: 1024 * 1024 * 10, // Simulated 10MB CoW snapshot metadata
                };

                job.last_run_timestamp = current_time;
                let interval = job.frequency.interval_seconds();
                job.next_run_timestamp = if interval > 0 {
                    current_time + interval
                } else {
                    current_time + 86400
                };

                created.push(record);
            }
        }

        for record in &created {
            self.history.push(record.clone());
        }

        created
    }

    pub fn prune_expired_snapshots(&mut self, job_id: u64) -> usize {
        let job = match self.jobs.iter().find(|j| j.id == job_id) {
            Some(j) => j,
            None => return 0,
        };

        let max_keep = job.retention.max_daily;
        let mut job_snaps: Vec<usize> = self
            .history
            .iter()
            .enumerate()
            .filter(|(_, r)| r.job_id == job_id)
            .map(|(idx, _)| idx)
            .collect();

        if job_snaps.len() <= max_keep {
            return 0;
        }

        let remove_count = job_snaps.len() - max_keep;
        job_snaps.truncate(remove_count);

        let mut pruned = 0;
        for &idx in job_snaps.iter().rev() {
            self.history.remove(idx);
            pruned += 1;
        }

        pruned
    }

    pub fn snapshot_history(&self) -> &[CreatedSnapshotRecord] {
        &self.history
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduled_snapshot_engine() {
        let mut engine = ScheduledSnapshotEngine::new();
        let current_time = 1700000000;

        let job_id = engine.add_job(
            "Root Btrfs Hourly",
            "rootfs",
            SnapshotFrequency::Hourly,
            RetentionPolicy::default(),
            current_time,
        );

        assert_eq!(engine.list_jobs().len(), 1);

        // Advance time by 3601 seconds
        let new_time = current_time + 3601;
        let created = engine.evaluate_and_run_schedules(new_time);

        assert_eq!(created.len(), 1);
        assert_eq!(created[0].job_id, job_id);
        assert_eq!(engine.snapshot_history().len(), 1);
    }
}
