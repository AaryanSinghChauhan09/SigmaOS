use std::collections::HashMap;

/// Represents a runtime probe point, displacing eBPF programs.
#[derive(Debug, Clone)]
pub struct Probe {
    pub name: String,
    pub attach_point: String, // e.g., "syscall:openat", "kprobe:vfs_read"
    pub hit_count: u64,
}

/// ProbeManager manages runtime instrumentation probes in pure Rust.
pub struct ProbeManager {
    probes: HashMap<String, Probe>,
}

impl Default for ProbeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ProbeManager {
    pub fn new() -> Self {
        Self {
            probes: HashMap::new(),
        }
    }

    /// Attach a new probe to a kernel or userland attach point.
    pub fn attach_probe(&mut self, name: &str, attach_point: &str) -> Result<(), String> {
        if self.probes.contains_key(name) {
            return Err(format!("Probe '{}' already attached", name));
        }
        self.probes.insert(
            name.to_string(),
            Probe {
                name: name.to_string(),
                attach_point: attach_point.to_string(),
                hit_count: 0,
            },
        );
        Ok(())
    }

    /// Record a probe hit.
    pub fn record_hit(&mut self, name: &str) {
        if let Some(probe) = self.probes.get_mut(name) {
            probe.hit_count += 1;
        }
    }

    /// Detach a probe.
    pub fn detach_probe(&mut self, name: &str) -> Result<Probe, String> {
        self.probes
            .remove(name)
            .ok_or_else(|| format!("Probe '{}' not found", name))
    }
}
