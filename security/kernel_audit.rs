// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// security/kernel_audit.rs — Kernel Security Audit System
//
// Implements a comprehensive security audit system for kernel code including
// vulnerability scanning, static analysis, runtime monitoring, and compliance checking
//
// Language: Rust (std for userland services)

use std::collections::HashMap;

// ─── Audit Severity Levels ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Severity {
    Info = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

// ─── Vulnerability Types ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VulnType {
    BufferOverflow,
    UseAfterFree,
    DoubleFree,
    IntegerOverflow,
    FormatString,
    RaceCondition,
    NullPointerDereference,
    MemoryLeak,
    InformationLeak,
    PrivilegeEscalation,
    DenialOfService,
    CodeInjection,
    Xss,
    SqlInjection,
    Csrf,
    Other,
}

// ─── Audit Finding ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct AuditFinding {
    pub id: String,
    pub vuln_type: VulnType,
    pub severity: Severity,
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub description: String,
    pub recommendation: String,
    pub cwe_id: Option<u32>,
    pub cvss_score: Option<f32>,
}

// ─── Audit Report ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct AuditReport {
    pub findings: Vec<AuditFinding>,
    pub total_findings: u32,
    pub critical_count: u32,
    pub high_count: u32,
    pub medium_count: u32,
    pub low_count: u32,
    pub info_count: u32,
    pub scan_duration_ms: u64,
    pub files_scanned: u32,
    pub lines_scanned: u32,
}

impl AuditReport {
    pub fn new() -> Self {
        AuditReport {
            findings: vec![],
            total_findings: 0,
            critical_count: 0,
            high_count: 0,
            medium_count: 0,
            low_count: 0,
            info_count: 0,
            scan_duration_ms: 0,
            files_scanned: 0,
            lines_scanned: 0,
        }
    }

    pub fn add_finding(&mut self, finding: AuditFinding) {
        match finding.severity {
            Severity::Critical => self.critical_count += 1,
            Severity::High => self.high_count += 1,
            Severity::Medium => self.medium_count += 1,
            Severity::Low => self.low_count += 1,
            Severity::Info => self.info_count += 1,
        }
        self.total_findings += 1;
        self.findings.push(finding);
    }

    pub fn get_summary(&self) -> String {
        format!(
            "Audit Summary: {} findings ({} critical, {} high, {} medium, {} low, {} info)",
            self.total_findings,
            self.critical_count,
            self.high_count,
            self.medium_count,
            self.low_count,
            self.info_count
        )
    }
}

// ─── Static Analysis Rules ───────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct AnalysisRule {
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub severity: Severity,
}

// ─── Kernel Auditor ───────────────────────────────────────────────────────────

pub struct KernelAuditor {
    pub report: AuditReport,
    pub rules: Vec<AnalysisRule>,
    pub initialized: bool,
    pub scan_in_progress: bool,
}

impl KernelAuditor {
    pub fn new() -> Self {
        let mut auditor = KernelAuditor {
            report: AuditReport::new(),
            rules: vec![],
            initialized: false,
            scan_in_progress: false,
        };

        auditor.init_default_rules();
        auditor
    }

    fn init_default_rules(&mut self) {
        self.rules = vec![
            AnalysisRule {
                name: "Buffer Overflow Detection".to_string(),
                description: "Detect potential buffer overflow vulnerabilities".to_string(),
                enabled: true,
                severity: Severity::Critical,
            },
            AnalysisRule {
                name: "Use-After-Free Detection".to_string(),
                description: "Detect use-after-free patterns".to_string(),
                enabled: true,
                severity: Severity::High,
            },
            AnalysisRule {
                name: "Double-Free Detection".to_string(),
                description: "Detect double-free patterns".to_string(),
                enabled: true,
                severity: Severity::High,
            },
            AnalysisRule {
                name: "Integer Overflow Detection".to_string(),
                description: "Detect integer overflow vulnerabilities".to_string(),
                enabled: true,
                severity: Severity::High,
            },
            AnalysisRule {
                name: "Null Pointer Dereference".to_string(),
                description: "Detect null pointer dereference patterns".to_string(),
                enabled: true,
                severity: Severity::High,
            },
            AnalysisRule {
                name: "Race Condition Detection".to_string(),
                description: "Detect potential race conditions".to_string(),
                enabled: true,
                severity: Severity::Medium,
            },
            AnalysisRule {
                name: "Memory Leak Detection".to_string(),
                description: "Detect memory leak patterns".to_string(),
                enabled: true,
                severity: Severity::Medium,
            },
            AnalysisRule {
                name: "Information Leak Detection".to_string(),
                description: "Detect potential information leaks".to_string(),
                enabled: true,
                severity: Severity::Medium,
            },
            AnalysisRule {
                name: "Privilege Escalation Detection".to_string(),
                description: "Detect privilege escalation patterns".to_string(),
                enabled: true,
                severity: Severity::Critical,
            },
            AnalysisRule {
                name: "Unsafe Function Usage".to_string(),
                description: "Detect usage of unsafe functions".to_string(),
                enabled: true,
                severity: Severity::Medium,
            },
            AnalysisRule {
                name: "Hardcoded Credentials".to_string(),
                description: "Detect hardcoded credentials".to_string(),
                enabled: true,
                severity: Severity::High,
            },
            AnalysisRule {
                name: "Cryptographic Weakness".to_string(),
                description: "Detect weak cryptographic implementations".to_string(),
                enabled: true,
                severity: Severity::High,
            },
        ];
    }

    pub fn init(&mut self) {
        self.initialized = true;
        self.report = AuditReport::new();
    }

    pub fn scan_file(&mut self, file_path: &str, content: &str) -> u32 {
        if !self.initialized {
            return 0;
        }

        self.scan_in_progress = true;
        let mut findings_count = 0;

        let lines: Vec<&str> = content.lines().collect();
        self.report.files_scanned += 1;
        self.report.lines_scanned += lines.len() as u32;

        // Run enabled rules
        for rule in &self.rules {
            if rule.enabled {
                findings_count += self.apply_rule(rule, file_path, &lines);
            }
        }

        self.scan_in_progress = false;
        findings_count
    }

    fn apply_rule(&mut self, rule: &AnalysisRule, file_path: &str, lines: &[&str]) -> u32 {
        let mut findings_count = 0;

        match rule.name.as_str() {
            "Buffer Overflow Detection" => {
                findings_count += self.detect_buffer_overflow(file_path, lines);
            }
            "Use-After-Free Detection" => {
                findings_count += self.detect_use_after_free(file_path, lines);
            }
            "Double-Free Detection" => {
                findings_count += self.detect_double_free(file_path, lines);
            }
            "Integer Overflow Detection" => {
                findings_count += self.detect_integer_overflow(file_path, lines);
            }
            "Null Pointer Dereference" => {
                findings_count += self.detect_null_pointer_dereference(file_path, lines);
            }
            "Race Condition Detection" => {
                findings_count += self.detect_race_condition(file_path, lines);
            }
            "Memory Leak Detection" => {
                findings_count += self.detect_memory_leak(file_path, lines);
            }
            "Information Leak Detection" => {
                findings_count += self.detect_information_leak(file_path, lines);
            }
            "Privilege Escalation Detection" => {
                findings_count += self.detect_privilege_escalation(file_path, lines);
            }
            "Unsafe Function Usage" => {
                findings_count += self.detect_unsafe_functions(file_path, lines);
            }
            "Hardcoded Credentials" => {
                findings_count += self.detect_hardcoded_credentials(file_path, lines);
            }
            "Cryptographic Weakness" => {
                findings_count += self.detect_cryptographic_weakness(file_path, lines);
            }
            _ => {}
        }

        findings_count
    }

    fn detect_buffer_overflow(&mut self, file_path: &str, lines: &[&str]) -> u32 {
        let mut findings = 0;

        for (line_num, line) in lines.iter().enumerate() {
            // Detect unsafe array access patterns
            if line.contains("memcpy") && !line.contains("size") {
                self.report.add_finding(AuditFinding {
                    id: format!("BUF-{}", line_num),
                    vuln_type: VulnType::BufferOverflow,
                    severity: Severity::High,
                    file: file_path.to_string(),
                    line: (line_num + 1) as u32,
                    column: 0,
                    description: "Potential buffer overflow in memcpy without size validation".to_string(),
                    recommendation: "Validate buffer sizes before memcpy".to_string(),
                    cwe_id: Some(120),
                    cvss_score: Some(7.5),
                });
                findings += 1;
            }

            // Detect strcpy/strcat usage
            if line.contains("strcpy") || line.contains("strcat") {
                self.report.add_finding(AuditFinding {
                    id: format!("BUF-{}", line_num),
                    vuln_type: VulnType::BufferOverflow,
                    severity: Severity::High,
                    file: file_path.to_string(),
                    line: (line_num + 1) as u32,
                    column: 0,
                    description: "Unsafe string function usage (strcpy/strcat)".to_string(),
                    recommendation: "Use strncpy/strncat or safer alternatives".to_string(),
                    cwe_id: Some(120),
                    cvss_score: Some(7.5),
                });
                findings += 1;
            }
        }

        findings
    }

    fn detect_use_after_free(&mut self, file_path: &str, lines: &[&str]) -> u32 {
        let mut findings = 0;
        let mut freed_vars: Vec<String> = vec![];

        for (line_num, line) in lines.iter().enumerate() {
            // Track freed variables
            if line.contains("free(") || line.contains("kfree(") {
                if let Some(start) = line.find('(') {
                    if let Some(end) = line.find(')') {
                        let var = line[start + 1..end].trim().to_string();
                        freed_vars.push(var);
                    }
                }
            }

            // Detect use after free
            for var in &freed_vars {
                if line.contains(var) && !line.contains("free(") {
                    self.report.add_finding(AuditFinding {
                        id: format!("UAF-{}", line_num),
                        vuln_type: VulnType::UseAfterFree,
                        severity: Severity::High,
                        file: file_path.to_string(),
                        line: (line_num + 1) as u32,
                        column: 0,
                        description: format!("Potential use-after-free of variable '{}'", var),
                        recommendation: "Set pointer to NULL after free".to_string(),
                        cwe_id: Some(416),
                        cvss_score: Some(7.5),
                    });
                    findings += 1;
                    break;
                }
            }
        }

        findings
    }

    fn detect_double_free(&mut self, file_path: &str, lines: &[&str]) -> u32 {
        let mut findings = 0;
        let mut freed_vars: HashMap<String, u32> = HashMap::new();

        for (line_num, line) in lines.iter().enumerate() {
            if line.contains("free(") || line.contains("kfree(") {
                if let Some(start) = line.find('(') {
                    if let Some(end) = line.find(')') {
                        let var = line[start + 1..end].trim().to_string();
                        if let Some(prev_line) = freed_vars.get(&var) {
                            self.report.add_finding(AuditFinding {
                                id: format!("DF-{}", line_num),
                                vuln_type: VulnType::DoubleFree,
                                severity: Severity::High,
                                file: file_path.to_string(),
                                line: (line_num + 1) as u32,
                                column: 0,
                                description: format!("Potential double-free of variable '{}' (first freed at line {})", var, prev_line),
                                recommendation: "Set pointer to NULL after free".to_string(),
                                cwe_id: Some(415),
                                cvss_score: Some(7.5),
                            });
                            findings += 1;
                        }
                        freed_vars.insert(var, (line_num + 1) as u32);
                    }
                }
            }
        }

        findings
    }

    fn detect_integer_overflow(&mut self, file_path: &str, lines: &[&str]) -> u32 {
        let mut findings = 0;

        for (line_num, line) in lines.iter().enumerate() {
            // Detect unchecked arithmetic operations
            if (line.contains("+") || line.contains("-") || line.contains("*")) 
                && !line.contains("checked_") 
                && !line.contains("saturating_") 
                && !line.contains("wrapping_") {
                self.report.add_finding(AuditFinding {
                    id: format!("IO-{}", line_num),
                    vuln_type: VulnType::IntegerOverflow,
                    severity: Severity::Medium,
                    file: file_path.to_string(),
                    line: (line_num + 1) as u32,
                    column: 0,
                    description: "Potential integer overflow in arithmetic operation".to_string(),
                    recommendation: "Use checked arithmetic or add bounds checking".to_string(),
                    cwe_id: Some(190),
                    cvss_score: Some(5.3),
                });
                findings += 1;
            }
        }

        findings
    }

    fn detect_null_pointer_dereference(&mut self, file_path: &str, lines: &[&str]) -> u32 {
        let mut findings = 0;

        for (line_num, line) in lines.iter().enumerate() {
            // Detect pointer dereference without null check
            if line.contains("->") || line.contains("*") {
                if !line.contains("if") && !line.contains("assert") {
                    self.report.add_finding(AuditFinding {
                        id: format!("NP-{}", line_num),
                        vuln_type: VulnType::NullPointerDereference,
                        severity: Severity::High,
                        file: file_path.to_string(),
                        line: (line_num + 1) as u32,
                        column: 0,
                        description: "Potential null pointer dereference without check".to_string(),
                        recommendation: "Add null check before dereferencing".to_string(),
                        cwe_id: Some(476),
                        cvss_score: Some(7.5),
                    });
                    findings += 1;
                }
            }
        }

        findings
    }

    fn detect_race_condition(&mut self, file_path: &str, lines: &[&str]) -> u32 {
        let mut findings = 0;

        for (line_num, line) in lines.iter().enumerate() {
            // Detect shared state access without locking
            if (line.contains("static mut") || line.contains("unsafe")) 
                && !line.contains("mutex") 
                && !line.contains("lock") 
                && !line.contains("atomic") {
                self.report.add_finding(AuditFinding {
                    id: format!("RC-{}", line_num),
                    vuln_type: VulnType::RaceCondition,
                    severity: Severity::Medium,
                    file: file_path.to_string(),
                    line: (line_num + 1) as u32,
                    column: 0,
                    description: "Potential race condition in unsafe static mutable access".to_string(),
                    recommendation: "Use proper synchronization primitives".to_string(),
                    cwe_id: Some(362),
                    cvss_score: Some(5.3),
                });
                findings += 1;
            }
        }

        findings
    }

    fn detect_memory_leak(&mut self, file_path: &str, lines: &[&str]) -> u32 {
        let mut findings = 0;
        let mut allocations: Vec<String> = vec![];
        let mut frees: Vec<String> = vec![];

        for (line_num, line) in lines.iter().enumerate() {
            // Track allocations
            if line.contains("malloc(") || line.contains("kmalloc(") || line.contains("alloc(") {
                if let Some(start) = line.find('=') {
                    let var = line[..start].trim().to_string();
                    allocations.push(var);
                }
            }

            // Track frees
            if line.contains("free(") || line.contains("kfree(") {
                if let Some(start) = line.find('(') {
                    if let Some(end) = line.find(')') {
                        let var = line[start + 1..end].trim().to_string();
                        frees.push(var);
                    }
                }
            }
        }

        // Check for allocations without corresponding frees
        for alloc in &allocations {
            if !frees.contains(alloc) {
                self.report.add_finding(AuditFinding {
                    id: format!("ML-{}", alloc.len()),
                    vuln_type: VulnType::MemoryLeak,
                    severity: Severity::Medium,
                    file: file_path.to_string(),
                    line: 0,
                    column: 0,
                    description: format!("Potential memory leak for variable '{}'", alloc),
                    recommendation: "Ensure allocated memory is freed".to_string(),
                    cwe_id: Some(401),
                    cvss_score: Some(5.3),
                });
                findings += 1;
            }
        }

        findings
    }

    fn detect_information_leak(&mut self, file_path: &str, lines: &[&str]) -> u32 {
        let mut findings = 0;

        for (line_num, line) in lines.iter().enumerate() {
            // Detect potential information leaks
            if line.contains("print") && (line.contains("password") || line.contains("key") || line.contains("secret")) {
                self.report.add_finding(AuditFinding {
                    id: format!("IL-{}", line_num),
                    vuln_type: VulnType::InformationLeak,
                    severity: Severity::High,
                    file: file_path.to_string(),
                    line: (line_num + 1) as u32,
                    column: 0,
                    description: "Potential information leak in debug output".to_string(),
                    recommendation: "Remove sensitive information from debug output".to_string(),
                    cwe_id: Some(532),
                    cvss_score: Some(5.3),
                });
                findings += 1;
            }
        }

        findings
    }

    fn detect_privilege_escalation(&mut self, file_path: &str, lines: &[&str]) -> u32 {
        let mut findings = 0;

        for (line_num, line) in lines.iter().enumerate() {
            // Detect potential privilege escalation patterns
            if line.contains("setuid") || line.contains("setgid") || line.contains("seteuid") {
                self.report.add_finding(AuditFinding {
                    id: format!("PE-{}", line_num),
                    vuln_type: VulnType::PrivilegeEscalation,
                    severity: Severity::Critical,
                    file: file_path.to_string(),
                    line: (line_num + 1) as u32,
                    column: 0,
                    description: "Privilege escalation function usage".to_string(),
                    recommendation: "Validate privilege changes and use principle of least privilege".to_string(),
                    cwe_id: Some(269),
                    cvss_score: Some(8.8),
                });
                findings += 1;
            }
        }

        findings
    }

    fn detect_unsafe_functions(&mut self, file_path: &str, lines: &[&str]) -> u32 {
        let mut findings = 0;
        let unsafe_functions = vec![
            "gets", "strcpy", "strcat", "sprintf", "vsprintf", "scanf", "fscanf", "sscanf",
            "system", "popen", "mktemp", "tmpnam", "tempnam",
        ];

        for (line_num, line) in lines.iter().enumerate() {
            for func in &unsafe_functions {
                if line.contains(func) {
                    self.report.add_finding(AuditFinding {
                        id: format!("UF-{}", line_num),
                        vuln_type: VulnType::Other,
                        severity: Severity::Medium,
                        file: file_path.to_string(),
                        line: (line_num + 1) as u32,
                        column: 0,
                        description: format!("Usage of unsafe function '{}'", func),
                        recommendation: "Use safer alternatives".to_string(),
                        cwe_id: Some(676),
                        cvss_score: Some(5.3),
                   ]);
                    findings += 1;
                    break;
                }
            }
        }

        findings
    }

    fn detect_hardcoded_credentials(&mut self, file_path: &str, lines: &[&str]) -> u32 {
        let mut findings = 0;

        for (line_num, line) in lines.iter().enumerate() {
            // Detect potential hardcoded credentials
            if line.contains("password") || line.contains("passwd") || line.contains("secret") || line.contains("api_key") {
                if line.contains("=") && (line.contains("\"") || line.contains("'")) {
                    self.report.add_finding(AuditFinding {
                        id: format!("HC-{}", line_num),
                        vuln_type: VulnType::Other,
                        severity: Severity::High,
                        file: file_path.to_string(),
                        line: (line_num + 1) as u32,
                        column: 0,
                        description: "Potential hardcoded credential detected".to_string(),
                        recommendation: "Use environment variables or secure configuration".to_string(),
                        cwe_id: Some(798),
                        cvss_score: Some(7.5),
                    });
                    findings += 1;
                }
            }
        }

        findings
    }

    fn detect_cryptographic_weakness(&mut self, file_path: &str, lines: &[&str]) -> u32 {
        let mut findings = 0;
        let weak_algorithms = vec![
            "md5", "sha1", "des", "rc4", "blowfish", "ecb",
        ];

        for (line_num, line) in lines.iter().enumerate() {
            for algo in &weak_algorithms {
                if line.to_lowercase().contains(algo) {
                    self.report.add_finding(AuditFinding {
                        id: format!("CW-{}", line_num),
                        vuln_type: VulnType::Other,
                        severity: Severity::High,
                        file: file_path.to_string(),
                        line: (line_num + 1) as u32,
                        column: 0,
                        description: format!("Usage of weak cryptographic algorithm '{}'", algo),
                        recommendation: "Use stronger cryptographic algorithms (SHA-256, AES, etc.)".to_string(),
                        cwe_id: Some(327),
                        cvss_score: Some(5.3),
                    });
                    findings += 1;
                    break;
                }
            }
        }

        findings
    }

    pub fn get_report(&self) -> &AuditReport {
        &self.report
    }

    pub fn get_report_mut(&mut self) -> &mut AuditReport {
        &mut self.report
    }

    pub fn enable_rule(&mut self, rule_name: &str) -> bool {
        for rule in &mut self.rules {
            if rule.name == rule_name {
                rule.enabled = true;
                return true;
            }
        }
        false
    }

    pub fn disable_rule(&mut self, rule_name: &str) -> bool {
        for rule in &mut self.rules {
            if rule.name == rule_name {
                rule.enabled = false;
                return true;
            }
        }
        false
    }

    pub fn get_rules(&self) -> Vec<AnalysisRule> {
        self.rules.clone()
    }

    pub fn export_report_json(&self) -> String {
        // In real implementation, generate JSON representation
        r#"{"findings":[],"summary":""}"#.to_string()
    }

    pub fn export_report_csv(&self) -> String {
        // In real implementation, generate CSV representation
        "id,type,severity,file,line,description\n".to_string()
    }
}
