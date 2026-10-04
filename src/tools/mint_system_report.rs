//! Read-only host system information report, inspired by Linux Mint System Information.
//!
//! This is a hosted inspection utility, not kernel telemetry. It reports only
//! facts available from the host's Linux procfs and os-release files. Missing
//! sources are recorded as unavailable instead of being filled with examples.

#![allow(dead_code)]

use std::format;
use std::fs;
use std::path::Path;
use std::string::{String, ToString};
use std::time::{SystemTime, UNIX_EPOCH};
use std::vec::Vec;

/// System information category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemInfoCategory {
    /// General system information
    General,
    /// CPU information
    Cpu,
    /// Memory information
    Memory,
    /// Storage information
    Storage,
    /// Graphics information
    Graphics,
    /// Network information
    Network,
    /// Audio information
    Audio,
    /// USB devices
    Usb,
    /// PCI devices
    Pci,
    /// Kernel information
    Kernel,
    /// Software information
    Software,
    /// Security information
    Security,
}

/// System information item
#[derive(Debug, Clone)]
pub struct SystemInfoItem {
    /// Item name
    pub name: String,
    /// Item value
    pub value: String,
    /// Item category
    pub category: SystemInfoCategory,
}

/// System report - collects and displays system information
#[derive(Debug)]
pub struct MintSystemReport {
    /// System information items
    pub info_items: Vec<SystemInfoItem>,
    /// Report timestamp
    pub timestamp: u64,
    /// Report title
    pub title: String,
}

impl MintSystemReport {
    /// Create a new System Report
    pub fn new() -> Self {
        Self {
            info_items: Vec::new(),
            timestamp: unix_timestamp(),
            title: "SigmaOS System Information Report".to_string(),
        }
    }

    /// Add an information item
    pub fn add_info_item(&mut self, item: SystemInfoItem) {
        if let Some(existing) = self
            .info_items
            .iter_mut()
            .find(|existing| existing.category == item.category && existing.name == item.name)
        {
            *existing = item;
            return;
        }
        self.info_items.push(item);
    }

    /// Get items by category
    pub fn get_items_by_category(&self, category: SystemInfoCategory) -> Vec<&SystemInfoItem> {
        self.info_items
            .iter()
            .filter(|item| item.category == category)
            .collect()
    }

    /// Get item by name
    pub fn get_item_by_name(&self, name: &str) -> Option<&SystemInfoItem> {
        self.info_items.iter().find(|item| item.name == name)
    }

    /// Generate text report
    pub fn generate_text_report(&self) -> String {
        let mut report = String::new();

        report.push_str(&self.title);
        report.push_str("\n");
        report.push_str(&"=".repeat(self.title.len()));
        report.push_str("\n\n");

        // Group by category
        let mut categories = Vec::new();
        for item in &self.info_items {
            if !categories.contains(&item.category) {
                categories.push(item.category);
            }
        }

        for category in categories {
            report.push_str(&format!("{:?}\n", category));
            report.push_str(&"-".repeat(20));
            report.push_str("\n");

            for item in self.get_items_by_category(category) {
                report.push_str(&format!("{}: {}\n", item.name, item.value));
            }

            report.push_str("\n");
        }

        report
    }

    /// Generate JSON report
    pub fn generate_json_report(&self) -> String {
        let mut json = String::new();

        json.push_str("{\n");
        json.push_str(&format!("  \"title\": \"{}\",\n", escape_json(&self.title)));
        json.push_str(&format!("  \"timestamp\": {},\n", self.timestamp));
        json.push_str("  \"items\": [\n");

        for (i, item) in self.info_items.iter().enumerate() {
            if i > 0 {
                json.push_str(",\n");
            }
            json.push_str("    {\n");
            json.push_str(&format!(
                "      \"name\": \"{}\",\n",
                escape_json(&item.name)
            ));
            json.push_str(&format!(
                "      \"value\": \"{}\",\n",
                escape_json(&item.value)
            ));
            json.push_str(&format!("      \"category\": \"{:?}\"\n", item.category));
            json.push_str("    }");
        }

        json.push_str("\n  ]\n");
        json.push_str("}\n");

        json
    }

    /// Generate markdown report
    pub fn generate_markdown_report(&self) -> String {
        let mut md = String::new();

        md.push_str("# ");
        md.push_str(&escape_markdown_table_cell(&self.title));
        md.push_str("\n\n");

        // Group by category
        let mut categories = Vec::new();
        for item in &self.info_items {
            if !categories.contains(&item.category) {
                categories.push(item.category);
            }
        }

        for category in categories {
            md.push_str("## ");
            md.push_str(&format!("{:?}", category));
            md.push_str("\n\n");

            md.push_str("| Name | Value |\n");
            md.push_str("|------|-------|\n");

            for item in self.get_items_by_category(category) {
                md.push_str(&format!(
                    "| {} | {} |\n",
                    escape_markdown_table_cell(&item.name),
                    escape_markdown_table_cell(&item.value)
                ));
            }

            md.push_str("\n");
        }

        md
    }

    /// Collect host OS release and architecture facts when those sources exist.
    pub fn collect_general_info(&mut self) {
        self.add_info_item(SystemInfoItem {
            name: "Report scope".to_string(),
            value: "Hosted Linux inspection; this is not SigmaOS kernel telemetry".to_string(),
            category: SystemInfoCategory::General,
        });
        self.add_info_item(SystemInfoItem {
            name: "Host architecture".to_string(),
            value: std::env::consts::ARCH.to_string(),
            category: SystemInfoCategory::General,
        });

        let os_release = Path::new("/etc/os-release");
        match fs::read_to_string(os_release) {
            Ok(contents) => {
                let fields = parse_os_release(&contents);
                if let Some(name) = fields.get("PRETTY_NAME").or_else(|| fields.get("NAME")) {
                    self.add_info_item(SystemInfoItem {
                        name: "Host operating system".to_string(),
                        value: name.clone(),
                        category: SystemInfoCategory::General,
                    });
                }
                if let Some(version) = fields.get("VERSION_ID") {
                    self.add_info_item(SystemInfoItem {
                        name: "Host OS version".to_string(),
                        value: version.clone(),
                        category: SystemInfoCategory::General,
                    });
                }
            }
            Err(error) => self.add_collection_warning("/etc/os-release", &error.to_string()),
        }

        match fs::read_to_string("/proc/sys/kernel/osrelease") {
            Ok(version) => self.add_info_item(SystemInfoItem {
                name: "Host kernel release".to_string(),
                value: version.trim().to_string(),
                category: SystemInfoCategory::Kernel,
            }),
            Err(error) => {
                self.add_collection_warning("/proc/sys/kernel/osrelease", &error.to_string())
            }
        }
    }

    /// Collect CPU model and logical processor count from Linux procfs.
    pub fn collect_cpu_info(&mut self) {
        match fs::read_to_string("/proc/cpuinfo") {
            Ok(contents) => {
                if let Some(model) = parse_cpu_model(&contents) {
                    self.add_info_item(SystemInfoItem {
                        name: "CPU model".to_string(),
                        value: model,
                        category: SystemInfoCategory::Cpu,
                    });
                }
                let processors = contents
                    .lines()
                    .filter(|line| {
                        line.starts_with("processor\t") || line.starts_with("processor :")
                    })
                    .count();
                if processors > 0 {
                    self.add_info_item(SystemInfoItem {
                        name: "Logical processors".to_string(),
                        value: processors.to_string(),
                        category: SystemInfoCategory::Cpu,
                    });
                }
            }
            Err(error) => self.add_collection_warning("/proc/cpuinfo", &error.to_string()),
        }
    }

    /// Collect total and available memory from Linux procfs.
    pub fn collect_memory_info(&mut self) {
        match fs::read_to_string("/proc/meminfo") {
            Ok(contents) => {
                for (source_key, label) in [
                    ("MemTotal:", "Total memory"),
                    ("MemAvailable:", "Available memory"),
                ] {
                    if let Some(value) = parse_meminfo_kib(&contents, source_key) {
                        self.add_info_item(SystemInfoItem {
                            name: label.to_string(),
                            value: format_kib(value),
                            category: SystemInfoCategory::Memory,
                        });
                    }
                }
            }
            Err(error) => self.add_collection_warning("/proc/meminfo", &error.to_string()),
        }
    }

    /// Collect all available host facts without inventing values.
    pub fn collect_all_info(&mut self) {
        self.timestamp = unix_timestamp();
        self.collect_general_info();
        self.collect_cpu_info();
        self.collect_memory_info();
    }

    fn add_collection_warning(&mut self, source: &str, reason: &str) {
        self.add_info_item(SystemInfoItem {
            name: format!("Unavailable source: {source}"),
            value: reason.to_string(),
            category: SystemInfoCategory::General,
        });
    }

    /// Search information items
    pub fn search(&self, query: &str) -> Vec<&SystemInfoItem> {
        let query_lower = query.to_lowercase();
        self.info_items
            .iter()
            .filter(|item| {
                item.name.to_lowercase().contains(&query_lower)
                    || item.value.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
}

/// Backwards-compatible name for consumers that used the old tool type.
pub type SystemInformationReport = MintSystemReport;

fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn parse_os_release(contents: &str) -> std::collections::BTreeMap<String, String> {
    contents
        .lines()
        .filter_map(|line| {
            let (key, value) = line.split_once('=')?;
            let key = key.trim();
            if key.is_empty()
                || !key
                    .bytes()
                    .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
            {
                return None;
            }
            let value = value.trim();
            let value = if value.len() >= 2
                && ((value.starts_with('"') && value.ends_with('"'))
                    || (value.starts_with('\'') && value.ends_with('\'')))
            {
                &value[1..value.len() - 1]
            } else {
                value
            };
            Some((key.to_string(), value.to_string()))
        })
        .collect()
}

fn parse_cpu_model(contents: &str) -> Option<String> {
    contents.lines().find_map(|line| {
        let (key, value) = line.split_once(':')?;
        matches!(key.trim(), "model name" | "Hardware")
            .then(|| value.trim().to_string())
            .filter(|value| !value.is_empty())
    })
}

fn parse_meminfo_kib(contents: &str, key: &str) -> Option<u64> {
    contents.lines().find_map(|line| {
        let (field, value) = line.split_once(':')?;
        if field.trim() != key.trim_end_matches(':') {
            return None;
        }
        value.split_whitespace().next()?.parse().ok()
    })
}

fn format_kib(kib: u64) -> String {
    let tenths = kib.saturating_mul(10) / 1_048_576;
    format!("{}.{:01} GiB", tenths / 10, tenths % 10)
}

fn escape_json(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            character if character.is_control() => escaped.push(' '),
            character => escaped.push(character),
        }
    }
    escaped
}

fn escape_markdown_table_cell(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '\\' | '|' | '[' | ']' | '(' | ')' | '*' | '_' | '`' | '#' => {
                escaped.push('\\');
                escaped.push(character);
            }
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '\n' | '\r' => escaped.push(' '),
            character if character.is_control() => escaped.push(' '),
            character => escaped.push(character),
        }
    }
    escaped
}

impl Default for MintSystemReport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_report_creation() {
        let report = MintSystemReport::new();
        assert_eq!(report.info_items.len(), 0);
    }

    #[test]
    fn test_add_info_item() {
        let mut report = MintSystemReport::new();

        let item = SystemInfoItem {
            name: "Test".to_string(),
            value: "Value".to_string(),
            category: SystemInfoCategory::General,
        };

        report.add_info_item(item);
        assert_eq!(report.info_items.len(), 1);
    }

    #[test]
    fn test_get_items_by_category() {
        let mut report = MintSystemReport::new();

        report.add_info_item(SystemInfoItem {
            name: "Test1".to_string(),
            value: "Value1".to_string(),
            category: SystemInfoCategory::General,
        });

        report.add_info_item(SystemInfoItem {
            name: "Test2".to_string(),
            value: "Value2".to_string(),
            category: SystemInfoCategory::Cpu,
        });

        let general_items = report.get_items_by_category(SystemInfoCategory::General);
        assert_eq!(general_items.len(), 1);
    }

    #[test]
    fn test_generate_text_report() {
        let mut report = MintSystemReport::new();

        report.add_info_item(SystemInfoItem {
            name: "Test".to_string(),
            value: "Value".to_string(),
            category: SystemInfoCategory::General,
        });

        let text = report.generate_text_report();
        assert!(text.contains("SigmaOS System Information Report"));
        assert!(text.contains("Test: Value"));
    }

    #[test]
    fn test_collect_general_info() {
        let mut report = MintSystemReport::new();
        report.collect_general_info();

        assert!(report.get_item_by_name("Report scope").is_some());
        assert_eq!(
            report.get_item_by_name("Host architecture").unwrap().value,
            std::env::consts::ARCH
        );
    }

    #[test]
    fn parsers_read_reported_values_without_fabricating_defaults() {
        let release = parse_os_release("NAME=\"Example OS\"\nVERSION_ID=1.2\ninvalid line\n");
        assert_eq!(release.get("NAME").map(String::as_str), Some("Example OS"));
        assert_eq!(release.get("VERSION_ID").map(String::as_str), Some("1.2"));
        assert_eq!(
            parse_cpu_model("processor : 0\nmodel name : Example CPU\n").as_deref(),
            Some("Example CPU")
        );
        assert_eq!(
            parse_meminfo_kib("MemTotal: 2048 kB\n", "MemTotal:"),
            Some(2048)
        );
        assert_eq!(
            parse_meminfo_kib("MemTotal: unavailable\n", "MemTotal:"),
            None
        );
    }

    #[test]
    fn report_exports_escape_untrusted_text() {
        let mut report = MintSystemReport::new();
        report.title = "Report\n| [link](target)".to_string();
        report.add_info_item(SystemInfoItem {
            name: "driver|name".to_string(),
            value: "line one\n\"quoted\"".to_string(),
            category: SystemInfoCategory::Graphics,
        });

        let json = report.generate_json_report();
        assert!(json.contains("Report\\n| [link](target)"));
        assert!(json.contains("driver|name"));
        assert!(json.contains("line one\\n\\\"quoted\\\""));
        let markdown = report.generate_markdown_report();
        assert!(markdown.contains("Report \\| \\[link\\]\\(target\\)"));
        assert!(markdown.contains("driver\\|name"));
        assert!(markdown.contains("line one \"quoted\""));
    }

    #[test]
    fn test_search() {
        let mut report = MintSystemReport::new();

        report.add_info_item(SystemInfoItem {
            name: "CPU Model".to_string(),
            value: "Intel i7".to_string(),
            category: SystemInfoCategory::Cpu,
        });

        let results = report.search("intel");
        assert_eq!(results.len(), 1);
    }
}
