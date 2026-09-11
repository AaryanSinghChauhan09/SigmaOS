use std::format;
use std::string::{String, ToString};

/// `fastfetch` / `neofetch` CLI System Information Banner Engine
#[derive(Debug, Clone)]
pub struct SystemInfoRecord {
    pub os_name: String,
    pub kernel_version: String,
    pub uptime_seconds: u64,
    pub shell: String,
    pub total_memory_mb: u64,
    pub used_memory_mb: u64,
}

#[derive(Debug, Clone)]
pub struct ItsFossFastfetchSysinfoEngine {
    pub info: SystemInfoRecord,
}

impl ItsFossFastfetchSysinfoEngine {
    pub fn new() -> Self {
        Self {
            info: SystemInfoRecord {
                os_name: "SigmaOS Sovereign Microkernel 1.0".to_string(),
                kernel_version: "6.9.0-sigmaos-rust".to_string(),
                uptime_seconds: 14400,
                shell: "sigma-sh 2.0".to_string(),
                total_memory_mb: 16384,
                used_memory_mb: 420,
            },
        }
    }

    /// Generates the Fastfetch ASCII logo and system info banner string
    pub fn render_banner(&self) -> String {
        let ascii_logo = r#"
   ███████╗██╗██████╗ ███╗   ███╗██████╗
   ██╔════╝██║██╔════╝ ████╗ ████║██╔══██╗
   ███████╗██║██║  ███╗██╔████╔██║██████╔╝
   ╚════██║██║██║   ██║██║╚██╔╝██║██╔══██╗
   ███████║██║╚██████╔╝██║ ╚═╝ ██║██║  ██║
   ╚══════╝╚═╝ ╚═════╝ ╚═╝     ╚═╝╚═╝  ╚═╝
"#;

        let mut output = String::new();
        output.push_str(ascii_logo);
        output.push_str(&format!("OS: {}\n", self.info.os_name));
        output.push_str(&format!("Kernel: {}\n", self.info.kernel_version));
        output.push_str(&format!("Uptime: {} hours\n", self.info.uptime_seconds / 3600));
        output.push_str(&format!("Shell: {}\n", self.info.shell));
        output.push_str(&format!(
            "Memory: {}MB / {}MB\n",
            self.info.used_memory_mb, self.info.total_memory_mb
        ));

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fastfetch_sysinfo_engine() {
        let sysinfo = ItsFossFastfetchSysinfoEngine::new();
        let banner = sysinfo.render_banner();
        assert!(banner.contains("SigmaOS Sovereign Microkernel"));
        assert!(banner.contains("Kernel: 6.9.0-sigmaos-rust"));
        assert!(banner.contains("Memory: 420MB / 16384MB"));
    }
}
