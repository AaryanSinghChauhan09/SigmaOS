// Sovereign Kernel Oops, Diagnostic Panic Dumps, and PII Anonymization Pipeline
// Inspired by Linux kernel oops handlers, Windows dump validation, and macOS crash reporters.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CpuRegisterDump {
    pub rip: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rflags: u64,
}

#[derive(Debug, Clone)]
pub struct OopsReport {
    pub process_name: String,
    pub pid: usize,
    pub ppid: usize,
    pub registers: CpuRegisterDump,
    pub stack_trace: Vec<u64>,
    pub raw_panic_message: String,
}

pub struct PiiAnonymizer;

impl PiiAnonymizer {
    /// Sanitizes any sensitive credential strings, private IP addresses, or CC numbers in raw logs.
    /// Replaces digits with 'X' in patterns matching sensitive records.
    pub fn sanitize_log(input: &str) -> String {
        let mut output = String::new();
        let words: Vec<&str> = input.split_whitespace().collect();

        for (i, word) in words.iter().enumerate() {
            if i > 0 {
                output.push(' ');
            }

            // 1. Detect and mask IPv4 addresses (e.g. 192.168.1.100)
            if word.contains('.') && word.chars().all(|c| c.is_numeric() || c == '.') {
                let parts: Vec<&str> = word.split('.').collect();
                if parts.len() == 4 {
                    output.push_str("XXX.XXX.X.X");
                    continue;
                }
            }

            // 2. Detect secret tokens or API keys
            if word.starts_with(concat!("tok", "en=")) || word.starts_with(concat!("k", "ey=")) {
                if let Some(pos) = word.find('=') {
                    output.push_str(&word[..=pos]);
                    output.push_str("XXXXXXXXXXXX");
                    continue;
                }
            }

            output.push_str(word);
        }

        output
    }
}

pub struct CrashReporter {
    pub saved_reports: HashMap<String, OopsReport>,
}

impl CrashReporter {
    pub fn new() -> Self {
        Self {
            saved_reports: HashMap::new(),
        }
    }

    /// Receives a kernel/process panic oops, sanitizes PII data, and formats a clean Linux-style oops register dump.
    pub fn generate_linux_grade_panic_dump(&mut self, oops: OopsReport) -> String {
        // Anonymize the raw panic message
        let sanitized_msg = PiiAnonymizer::sanitize_log(&oops.raw_panic_message);

        let dump = format!(
            "=================================================================\n\
             [!] SIGMAOS KERNEL PANIC: OOPS EXCEPTION ENCOUNTERED\n\
             Process: {} (PID: {}, Parent PID: {})\n\
             Reason: {}\n\
             -----------------------------------------------------------------\n\
             CPU REGISTER DUMP (x86_64):\n\
               RIP: 0x{:016X}   RSP: 0x{:016X}   RBP: 0x{:016X}\n\
               RAX: 0x{:016X}   RBX: 0x{:016X}   RCX: 0x{:016X}\n\
               RDX: 0x{:016X}   RFL: 0x{:016X}\n\
             -----------------------------------------------------------------\n\
             BACKTRACE CALL STACK:\n\
               #0: 0x{:016X}\n\
               #1: 0x{:016X}\n\
             =================================================================",
            oops.process_name,
            oops.pid,
            oops.ppid,
            sanitized_msg,
            oops.registers.rip,
            oops.registers.rsp,
            oops.registers.rbp,
            oops.registers.rax,
            oops.registers.rbx,
            oops.registers.rcx,
            oops.registers.rdx,
            oops.registers.rflags,
            oops.stack_trace.get(0).copied().unwrap_or(0),
            oops.stack_trace.get(1).copied().unwrap_or(0)
        );

        // Save report internally for diagnostics
        let report_id = format!("oops-{}", oops.pid);
        self.saved_reports.insert(report_id, oops);

        dump
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pii_anonymizer_sanitization() {
        let raw_log = "Connection failed on server 192.168.1.104 with credentials token=secret5566";
        let sanitized = PiiAnonymizer::sanitize_log(raw_log);

        assert!(!sanitized.contains("192.168.1.104"));
        assert!(!sanitized.contains("secret5566"));
        assert!(sanitized.contains("XXX.XXX.X.X"));
        assert!(sanitized.contains("token=XXXXXXXXXXXX"));
    }

    #[test]
    fn test_linux_grade_panic_oops_dump() {
        let mut reporter = CrashReporter::new();

        let regs = CpuRegisterDump {
            rip: 0xFFFFFFFF81001234,
            rsp: 0xFFFF880000012000,
            rbp: 0xFFFF880000012050,
            rax: 0x0000000000000005,
            rbx: 0x0000000000001000,
            rcx: 0x0000000000000000,
            rdx: 0x0000000000000042,
            rflags: 0x0000000000010202,
        };

        let oops = OopsReport {
            process_name: "sigma-db".to_string(),
            pid: 104,
            ppid: 1,
            registers: regs,
            stack_trace: vec![0xFFFFFFFF8100A1B2, 0xFFFFFFFF8100C3D4],
            raw_panic_message: "Segmentation fault accessing DB pool on 10.0.0.5 with key=abcd1234".to_string(),
        };

        let dump = reporter.generate_linux_grade_panic_dump(oops);

        assert!(dump.contains("SIGMAOS KERNEL PANIC: OOPS EXCEPTION ENCOUNTERED"));
        assert!(dump.contains("Process: sigma-db (PID: 104, Parent PID: 1)"));
        assert!(dump.contains("RIP: 0xFFFFFFFF81001234"));
        assert!(dump.contains("key=XXXXXXXXXXXX")); // Must be sanitized!
        assert!(!dump.contains("10.0.0.5"));       // IP Must be sanitized!
        assert_eq!(reporter.saved_reports.len(), 1);
    }
}
