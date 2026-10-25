// SPDX-License-Identifier: GPL-2.0-or-later
// KarmaBSD DTrace-compatible observability layer for SigmaOS.
// Provides probe registration, enabling/firing, trace collection,
// and ZFS I/O observability for kernel and userspace instrumentation.

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// A four-tuple DTrace probe location identifier.
#[derive(Debug, Clone)]
pub struct ProbePoint {
    pub provider: String,
    pub module: String,
    pub function: String,
    pub name: String,
}

impl ProbePoint {
    pub fn new(provider: &str, module: &str, function: &str, name: &str) -> Self {
        Self {
            provider: provider.to_string(),
            module: module.to_string(),
            function: function.to_string(),
            name: name.to_string(),
        }
    }

    pub fn fqn(&self) -> String {
        alloc::format!(
            "{}:{}:{}:{}",
            self.provider,
            self.module,
            self.function,
            self.name
        )
    }
}

/// A runtime-registered DTrace probe instance.
#[derive(Debug, Clone)]
pub struct DTraceProbe {
    pub probe_id: u32,
    pub point: ProbePoint,
    pub enabled: bool,
}

/// ZFS pool / dataset I/O statistics.
#[derive(Debug, Clone, Default)]
pub struct ZfsIoStats {
    pub reads: u64,
    pub writes: u64,
    pub read_bytes: u64,
    pub write_bytes: u64,
}

/// ZFS observability provider — gathers pool, dataset, and I/O metrics.
pub struct ZfsObservabilityProvider {
    pub pool_stats: BTreeMap<String, ZfsIoStats>,
}

impl ZfsObservabilityProvider {
    pub fn new() -> Self {
        Self {
            pool_stats: BTreeMap::new(),
        }
    }

    pub fn pool_stats(&self, pool: &str) -> Option<&ZfsIoStats> {
        self.pool_stats.get(pool)
    }

    pub fn dataset_stats(&self, dataset: &str) -> Option<&ZfsIoStats> {
        // Datasets share the pool prefix; look up by pool name.
        let pool = dataset.split('/').next().unwrap_or(dataset);
        self.pool_stats.get(pool)
    }

    /// Returns a simplified latency histogram bucket count.
    pub fn io_latency_histogram(&self, pool: &str) -> Vec<u64> {
        match self.pool_stats.get(pool) {
            None => alloc::vec![0u64; 8],
            Some(s) => {
                // Simplified: distribute reads across 8 buckets.
                let bucket = s.reads / 8 + 1;
                alloc::vec![bucket; 8]
            }
        }
    }

    pub fn record_io(&mut self, pool: &str, reads: u64, writes: u64, rb: u64, wb: u64) {
        let stats = self.pool_stats.entry(pool.to_string()).or_default();
        stats.reads += reads;
        stats.writes += writes;
        stats.read_bytes += rb;
        stats.write_bytes += wb;
    }
}

impl Default for ZfsObservabilityProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// The KarmaInstrument engine — registers, enables, fires, and collects probes.
pub struct KarmaInstrument {
    next_id: u32,
    pub probes: Vec<DTraceProbe>,
    pub traces: Vec<String>,
}

impl KarmaInstrument {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            probes: Vec::new(),
            traces: Vec::new(),
        }
    }

    pub fn register_probe(&mut self, point: ProbePoint) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.probes.push(DTraceProbe {
            probe_id: id,
            point,
            enabled: false,
        });
        id
    }

    pub fn enable_probe(&mut self, probe_id: u32) -> Result<(), &'static str> {
        self.probes
            .iter_mut()
            .find(|p| p.probe_id == probe_id)
            .ok_or("Probe not found")
            .map(|p| {
                p.enabled = true;
            })
    }

    pub fn fire_probe(&mut self, probe_id: u32) -> Result<(), &'static str> {
        let probe = self
            .probes
            .iter()
            .find(|p| p.probe_id == probe_id)
            .ok_or("Probe not found")?;
        if !probe.enabled {
            return Err("Probe not enabled");
        }
        let trace = alloc::format!("FIRE probe_id={} fqn={}", probe_id, probe.point.fqn());
        self.traces.push(trace);
        Ok(())
    }

    pub fn collect_traces(&mut self) -> Vec<String> {
        core::mem::take(&mut self.traces)
    }
}

impl Default for KarmaInstrument {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probe_registration_and_firing() {
        let mut karma = KarmaInstrument::new();
        let pt = ProbePoint::new("zfs", "arc", "arc_read", "entry");
        let id = karma.register_probe(pt);
        assert!(karma.fire_probe(id).is_err()); // not enabled yet
        karma.enable_probe(id).unwrap();
        assert!(karma.fire_probe(id).is_ok());
        let traces = karma.collect_traces();
        assert_eq!(traces.len(), 1);
        assert!(traces[0].contains("zfs:arc:arc_read:entry"));
    }

    #[test]
    fn test_zfs_observability() {
        let mut zfs = ZfsObservabilityProvider::new();
        zfs.record_io("zroot", 100, 50, 4096 * 100, 4096 * 50);
        let stats = zfs.pool_stats("zroot").unwrap();
        assert_eq!(stats.reads, 100);
        let hist = zfs.io_latency_histogram("zroot");
        assert_eq!(hist.len(), 8);
    }
}
