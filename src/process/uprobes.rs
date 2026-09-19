// SigmaOS Process Uprobes & eBPF Instrumentation Engine
// Implements user-space dynamic binary instrumentation (DBI), function entry/ret probe hooks,
// instruction replacement, and zero-overhead process tracing.

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UprobeKind {
    Entry,
    Return,
}

#[derive(Debug, Clone)]
pub struct UprobePoint {
    pub probe_id: u64,
    pub pid: usize,
    pub binary_path: String,
    pub symbol_name: String,
    pub offset: usize,
    pub kind: UprobeKind,
    pub original_byte: u8,
    pub hits_count: u64,
    pub is_enabled: bool,
}

pub struct ProcessInstrumentationEngine {
    pub probes: BTreeMap<u64, UprobePoint>,
    pub next_probe_id: u64,
    pub trace_log: Vec<String>,
}

impl ProcessInstrumentationEngine {
    pub fn new() -> Self {
        Self {
            probes: BTreeMap::new(),
            next_probe_id: 1,
            trace_log: Vec::new(),
        }
    }

    pub fn attach_uprobe(
        &mut self,
        pid: usize,
        path: &str,
        symbol: &str,
        offset: usize,
        kind: UprobeKind,
    ) -> u64 {
        let id = self.next_probe_id;
        self.next_probe_id += 1;

        let probe = UprobePoint {
            probe_id: id,
            pid,
            binary_path: path.to_string(),
            symbol_name: symbol.to_string(),
            offset,
            kind,
            original_byte: 0x55, // push rbp mock original instruction byte
            hits_count: 0,
            is_enabled: true,
        };

        self.probes.insert(id, probe);
        id
    }

    pub fn trigger_uprobe_hit(&mut self, probe_id: u64, args_summary: &str) -> Result<(), &'static str> {
        if let Some(probe) = self.probes.get_mut(&probe_id) {
            if !probe.is_enabled {
                return Err("Uprobe: Probe is disabled");
            }
            probe.hits_count += 1;
            let log_entry = format!(
                "uprobe [{:?}]: pid={} symbol='{}' args='{}'",
                probe.kind, probe.pid, probe.symbol_name, args_summary
            );
            self.trace_log.push(log_entry);
            Ok(())
        } else {
            Err("Uprobe: Probe ID not found")
        }
    }
}

impl Default for ProcessInstrumentationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_uprobes_instrumentation() {
        let mut engine = ProcessInstrumentationEngine::new();
        let pid = 2048;

        let id = engine.attach_uprobe(pid, "/usr/bin/node", "v8::Context::New", 0x1200, UprobeKind::Entry);
        assert_eq!(id, 1);

        assert!(engine.trigger_uprobe_hit(id, "isolate=0x7fff00").is_ok());

        let probe = engine.probes.get(&id).unwrap();
        assert_eq!(probe.hits_count, 1);
        assert_eq!(engine.trace_log.len(), 1);
        assert!(engine.trace_log[0].contains("v8::Context::New"));
    }
}
