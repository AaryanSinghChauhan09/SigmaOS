// SigmaOS Open-Source Developer & System Tools Parity Suite
// Inspired by fastfetch, jq, duf, and dust
// Zero-dependency, #![no_std] compliant native Rust implementations

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::format;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec;
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// =========================================================================
// 1. FASTFETCH / NEOFETCH SYSTEM INFO BANNER ENGINE (fastfetch)
// =========================================================================

pub struct FastfetchSysinfo {
    pub os_name: String,
    pub host: String,
    pub kernel: String,
    pub uptime: String,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
}

pub struct ItsFossFastfetchSysinfoEngine;

impl ItsFossFastfetchSysinfoEngine {
    pub fn render_banner(info: &FastfetchSysinfo) -> String {
        let logo = [
            "   /\\   ",
            "  /  \\  ",
            " / /\\ \\ ",
            "/ /__\\ \\",
            "\\______/",
        ];

        let mem_percent = if info.memory_total_mb > 0 {
            (info.memory_used_mb * 100) / info.memory_total_mb
        } else {
            0
        };

        format!(
            "{}\nOS: {}\nHost: {}\nKernel: {}\nUptime: {}\nMemory: {}MB / {}MB ({}%)\n",
            logo.join("\n"),
            info.os_name,
            info.host,
            info.kernel,
            info.uptime,
            info.memory_used_mb,
            info.memory_total_mb,
            mem_percent
        )
    }
}

// =========================================================================
// 2. SIMPLE JQ JSON QUERY ENGINE (jq)
// =========================================================================

pub struct SimpleJqJsonQueryEngine;

impl SimpleJqJsonQueryEngine {
    /// Evaluates simple dot queries (e.g. `.user.name`) on JSON key-value strings
    pub fn extract_field(json: &str, query: &str) -> Option<String> {
        let field_key = query.trim_start_matches('.');
        let search_pattern = format!("\"{}\":", field_key);

        if let Some(pos) = json.find(&search_pattern) {
            let rest = &json[pos + search_pattern.len()..].trim();
            if rest.starts_with('"') {
                let start = 1;
                if let Some(end) = rest[start..].find('"') {
                    return Some(rest[start..start + end].to_string());
                }
            } else {
                let end = rest.find(|c: char| c == ',' || c == '}' || c == ']').unwrap_or(rest.len());
                return Some(rest[..end].trim().to_string());
            }
        }
        None
    }
}

// =========================================================================
// 3. DUF DISK USAGE FORMATTER ENGINE (duf)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DufDeviceUsage {
    pub device: String,
    pub mount_point: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
}

pub struct DufDiskUsageEngine;

impl DufDiskUsageEngine {
    /// Formats disk usage into a visual CLI bar chart
    pub fn render_usage_bar(usage: &DufDeviceUsage) -> String {
        let used_pct = if usage.total_bytes > 0 {
            (usage.used_bytes * 100) / usage.total_bytes
        } else {
            0
        };

        let bar_width = 20;
        let filled = (used_pct as usize * bar_width) / 100;
        let empty = bar_width.saturating_sub(filled);

        let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));
        format!(
            "{:<10} {:<15} {} {}%",
            usage.device, usage.mount_point, bar, used_pct
        )
    }
}

// =========================================================================
// 4. DUST RECURSIVE DIRECTORY TREE SIZE ANALYZER ENGINE (dust)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DustNode {
    pub name: String,
    pub size_bytes: u64,
    pub children: Vec<DustNode>,
}

pub struct DustDirectoryTreeEngine;

impl DustDirectoryTreeEngine {
    /// Renders recursive tree view with human-readable sizes
    pub fn render_tree(node: &DustNode, depth: usize) -> Vec<String> {
        let indent = "  ".repeat(depth);
        let size_kb = node.size_bytes / 1024;
        let mut lines = vec![format!("{}├── {} ({} KB)", indent, node.name, size_kb)];

        for child in &node.children {
            lines.extend(Self::render_tree(child, depth + 1));
        }
        lines
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fastfetch_sysinfo_engine() {
        let info = FastfetchSysinfo {
            os_name: "SigmaOS Sovereign".to_string(),
            host: "SovereignNode".to_string(),
            kernel: "6.10.0-sigma".to_string(),
            uptime: "2d 4h".to_string(),
            memory_used_mb: 2048,
            memory_total_mb: 8192,
        };

        let banner = ItsFossFastfetchSysinfoEngine::render_banner(&info);
        assert!(banner.contains("SigmaOS Sovereign"));
        assert!(banner.contains("25%"));
    }

    #[test]
    fn test_simple_jq_json_query() {
        let json_data = "{\"user\": \"sovereign\", \"status\": \"active\", \"version\": 1}";
        assert_eq!(
            SimpleJqJsonQueryEngine::extract_field(json_data, ".user"),
            Some("sovereign".to_string())
        );
        assert_eq!(
            SimpleJqJsonQueryEngine::extract_field(json_data, ".version"),
            Some("1".to_string())
        );
    }

    #[test]
    fn test_duf_disk_usage_engine() {
        let usage = DufDeviceUsage {
            device: "/dev/nvme0n1p2".to_string(),
            mount_point: "/".to_string(),
            total_bytes: 100_000_000_000,
            used_bytes: 50_000_000_000,
        };

        let bar = DufDiskUsageEngine::render_usage_bar(&usage);
        assert!(bar.contains("50%"));
        assert!(bar.contains("██████████░░░░░░░░░░"));
    }

    #[test]
    fn test_dust_directory_tree_engine() {
        let root = DustNode {
            name: "root".to_string(),
            size_bytes: 20480,
            children: vec![DustNode {
                name: "bin".to_string(),
                size_bytes: 10240,
                children: Vec::new(),
            }],
        };

        let lines = DustDirectoryTreeEngine::render_tree(&root, 0);
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("root"));
        assert!(lines[1].contains("bin"));
    }
}
