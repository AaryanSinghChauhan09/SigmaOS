# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_security.nim — Security advisor sub-agent
# Scans logs for anomalies, suggests policies, audits telemetry,
# reviews firewall rules, and acts as a CLI-first security advisor.
#
# Inspiration:
#   azure-cli security commands   — policy + threat detection
#   Claude Code security analysis — file/code audit
#   Aider --lint                  — static analysis feedback loop
#
# Language: Nim (stdlib only)

import std/[os, osproc, strutils, strformat, times, json, tables, sequtils]

# ── Severity levels ───────────────────────────────────────────────────────────
type
  Severity = enum SevInfo, SevWarn, SevCritical

  Finding = object
    severity:    Severity
    category:    string
    description: string
    recommendation: string
    raw:         string

# ── ANSI helpers ──────────────────────────────────────────────────────────────
const
  C_RED    = "\e[38;2;248;113;113m"
  C_YELLOW = "\e[38;2;251;191;36m"
  C_GREEN  = "\e[38;2;52;211;153m"
  C_CYAN   = "\e[38;2;69;243;255m"
  C_MUTED  = "\e[38;2;107;114;128m"
  C_BOLD   = "\e[1m"
  C_RESET  = "\e[0m"

proc sev_color(s: Severity): string =
  case s
  of SevCritical: C_RED
  of SevWarn:     C_YELLOW
  of SevInfo:     C_MUTED

proc sev_icon(s: Severity): string =
  case s
  of SevCritical: "🔴"
  of SevWarn:     "🟡"
  of SevInfo:     "ℹ"

# ── Log anomaly scanner ────────────────────────────────────────────────────────
proc scan_logs*(lines = 200): seq[Finding] =
  ## Read system logs and surface anomalous entries
  const PATTERNS = [
    ("authentication failure", SevWarn,     "auth",     "Multiple auth failures detected. Consider enabling fail2ban or rate-limiting SSH."),
    ("segfault",               SevCritical, "crash",    "Segmentation fault detected. Check dmesg and the affected binary."),
    ("kernel: BUG",            SevCritical, "kernel",   "Kernel BUG detected. File an issue at github.com/AaryanSinghChauhan09/SigmaOS"),
    ("OOM",                    SevCritical, "memory",   "Out-of-memory kill detected. Increase swap or reduce memory pressure."),
    ("FAILED",                 SevWarn,     "service",  "Service failure detected. Run: sigma-agent 'show processes' to diagnose."),
    ("permission denied",      SevWarn,     "access",   "Permission denied errors. Verify file ownership and sigma_pledge config."),
    ("connection refused",     SevWarn,     "network",  "Connection refused. Check if the target service is running."),
    ("disk full",              SevCritical, "storage",  "Disk full. Run: sigma-agent 'disk usage' then clean up."),
    ("certificate",            SevWarn,     "tls",      "TLS certificate issue detected. Renew or verify certificate chain."),
    ("invalid user",           SevWarn,     "auth",     "Invalid SSH user attempts. Restrict sshd_config AllowUsers."),
    ("timeout",                SevWarn,     "network",  "Timeout events detected. Check network latency and DNS resolution."),
    ("out of memory",          SevCritical, "memory",   "Memory exhaustion. Consider increasing RAM or adjusting cgroups."),
  ]

  # Try multiple log sources
  var log_content = ""
  let sources = [
    fmt"sigma-log -n {lines} 2>/dev/null",
    fmt"journalctl -n {lines} --no-pager 2>/dev/null",
    fmt"tail -n {lines} /var/log/syslog 2>/dev/null",
    fmt"tail -n {lines} /var/log/messages 2>/dev/null",
    fmt"dmesg 2>/dev/null | tail -n {lines}",
  ]
  for src in sources:
    let (out, code) = execCmdEx(src)
    if code == 0 and out.strip().len > 10:
      log_content = out; break

  if log_content.len == 0:
    return @[Finding(severity: SevInfo, category: "logs",
      description: "No log sources available",
      recommendation: "Install sigma-log: sigma-pkg install sigma-log")]

  var counts: Table[string, int]
  for line_lower in log_content.toLowerAscii.splitLines():
    for (pattern, severity, category, recommendation) in PATTERNS:
      if pattern in line_lower:
        let key = pattern
        counts[key] = counts.getOrDefault(key, 0) + 1

  for (pattern, severity, category, recommendation) in PATTERNS:
    let count = counts.getOrDefault(pattern, 0)
    if count > 0:
      result.add Finding(
        severity:       severity,
        category:       category,
        description:    fmt"{count}x '{pattern}' in recent logs",
        recommendation: recommendation,
        raw:            pattern,
      )

# ── Open port audit ────────────────────────────────────────────────────────────
proc audit_ports*(): seq[Finding] =
  let (ss_out, code) = execCmdEx("ss -tlnp 2>/dev/null")
  if code != 0: return

  # Ports that should not be exposed on a typical SigmaOS desktop
  const SUSPICIOUS_PORTS = [
    (21,    "FTP — unencrypted file transfer. Use SFTP instead."),
    (23,    "Telnet — unencrypted shell. Disable immediately."),
    (25,    "SMTP — mail relay open. Restrict to localhost."),
    (135,   "RPC — Windows-style RPC. Should not be open."),
    (445,   "SMB — file sharing. Ensure authentication is enabled."),
    (3306,  "MySQL — database. Bind to localhost unless needed."),
    (5432,  "PostgreSQL — database. Bind to localhost unless needed."),
    (6379,  "Redis — no auth by default. Add requirepass in config."),
    (27017, "MongoDB — database. Enable authentication."),
  ]

  for line in ss_out.splitLines():
    for (port, advice) in SUSPICIOUS_PORTS:
      if fmt":{port} " in line or fmt":{port}\t" in line:
        result.add Finding(severity: SevWarn, category: "ports",
          description: fmt"Port {port} is listening ({line.strip()[0..<min(60,line.len)]})",
          recommendation: advice, raw: $port)

  # Count total open ports
  let total = ss_out.splitLines().len - 1
  if total > 15:
    result.add Finding(severity: SevWarn, category: "ports",
      description: fmt"{total} TCP ports listening — more than typical",
      recommendation: "Review with: ss -tlnp | grep LISTEN",
      raw: "port_count")

# ── File permission audit ──────────────────────────────────────────────────────
proc audit_permissions*(): seq[Finding] =
  const CRITICAL_FILES = [
    ("/etc/passwd",  "644"),
    ("/etc/shadow",  "640"),
    ("/etc/sudoers", "440"),
    ("/etc/ssh/sshd_config", "600"),
  ]
  for (path, expected_mode) in CRITICAL_FILES:
    if not fileExists(path): continue
    let (perm_out, code) = execCmdEx(fmt"stat -c '%a' {path} 2>/dev/null")
    if code != 0: continue
    let actual = perm_out.strip()
    if actual != expected_mode:
      result.add Finding(severity: SevWarn, category: "permissions",
        description: fmt"{path} has mode {actual}, expected {expected_mode}",
        recommendation: fmt"Fix with: chmod {expected_mode} {path}",
        raw: path)

  # World-writable files in sensitive dirs
  let (ww_out, _) = execCmdEx("find /etc /usr/bin /usr/sbin -perm -o+w -type f 2>/dev/null | head -5")
  for line in ww_out.strip().splitLines():
    if line.len > 0:
      result.add Finding(severity: SevCritical, category: "permissions",
        description: fmt"World-writable sensitive file: {line}",
        recommendation: fmt"Remove write bit: chmod o-w {line}",
        raw: line)

# ── SUID binary audit ─────────────────────────────────────────────────────────
proc audit_suid*(): seq[Finding] =
  let (suid_out, _) = execCmdEx("find /usr/bin /usr/sbin /bin /sbin -perm -4000 -type f 2>/dev/null")
  var suid_bins: seq[string]
  for line in suid_out.strip().splitLines():
    if line.len > 0: suid_bins.add(line.extractFilename)

  # Known safe SUID binaries
  const KNOWN_SAFE = ["sudo","su","passwd","ping","mount","umount","newgrp","chfn","chsh"]
  for bin in suid_bins:
    if bin notin KNOWN_SAFE:
      result.add Finding(severity: SevWarn, category: "suid",
        description: fmt"Unexpected SUID binary: {bin}",
        recommendation: fmt"Verify with: dpkg -S {bin} or rpm -qf {bin}",
        raw: bin)

# ── Telemetry / privacy audit ──────────────────────────────────────────────────
proc audit_telemetry*(): seq[Finding] =
  # Check SigmaOS telemetry settings
  let cfg_path = getEnv("HOME", "/tmp") / ".config/sigma/settings/privacy.toml"
  if fileExists(cfg_path):
    let cfg = readFile(cfg_path)
    if "telemetry" in cfg and "true" in cfg:
      result.add Finding(severity: SevInfo, category: "privacy",
        description: "Telemetry is enabled in SigmaOS privacy settings",
        recommendation: "Disable with: sigma-agent \"settings set privacy telemetry false\"",
        raw: "telemetry")
  else:
    result.add Finding(severity: SevInfo, category: "privacy",
      description: "Privacy settings not configured",
      recommendation: "Review with: sigma-agent \"settings get privacy\"",
      raw: "privacy_unconfigured")

  # Check for analytics processes
  const ANALYTICS_PROCS = ["telemetryd","analytics","crashreport","sentry","datadog"]
  let (ps_out, _) = execCmdEx("ps aux 2>/dev/null")
  for proc_name in ANALYTICS_PROCS:
    if proc_name in ps_out.toLowerAscii:
      result.add Finding(severity: SevWarn, category: "privacy",
        description: fmt"Analytics process running: {proc_name}",
        recommendation: fmt"Disable with: sigma-agent \"settings set privacy {proc_name} false\"",
        raw: proc_name)

# ── Policy suggestions ────────────────────────────────────────────────────────
proc suggest_policies*(context: string = ""): seq[string] =
  ## AI-style policy recommendations based on current system state
  var suggestions: seq[string]

  let (sshd_out, _)     = execCmdEx("systemctl is-active sshd 2>/dev/null")
  let (firewall_out, _) = execCmdEx("sigma-agent-core --once 'settings get network firewall' 2>/dev/null")

  if "active" in sshd_out.toLowerAscii:
    suggestions.add "SSH daemon is running. Consider: PasswordAuthentication no in sshd_config"
    suggestions.add "Restrict SSH access: sigma-agent \"settings set network ssh_allow_users user1,user2\""

  if "false" in firewall_out.toLowerAscii or firewall_out.strip() == "":
    suggestions.add "Firewall appears disabled. Enable with: sigma-agent \"settings set network firewall true\""

  let mem_ctx = collect_memory_pct()
  if mem_ctx > 80:
    suggestions.add fmt"Memory usage is {mem_ctx}%. Consider enabling zram: sigma-agent \"run modprobe zram\""

  suggestions.add "Enable PQC key exchange: sigma-agent \"settings set security pqc_tls true\""
  suggestions.add "Enable sigma_pledge for all apps: sigma-agent \"settings set security pledge_all true\""
  suggestions.add "Review audit trail: sigma-agent \"run sigma-log --type=security\""
  suggestions

proc collect_memory_pct(): int =
  try:
    var total = 0'i64; var avail = 0'i64
    for line in readFile("/proc/meminfo").splitLines():
      let p = line.split()
      if p.len >= 2:
        if p[0] == "MemTotal:":      total = parseInt(p[1])
        elif p[0] == "MemAvailable:": avail = parseInt(p[1])
    if total > 0: return int((total - avail) * 100 div total)
  except: discard
  0

# ── Full security report ───────────────────────────────────────────────────────
proc run_security_scan*(verbose = false): tuple[findings: seq[Finding], score: int] =
  var all_findings: seq[Finding]
  echo C_CYAN & C_BOLD & "Σ sigma-agent security scan\e[0m\n"

  proc run_check(name: string, check: proc(): seq[Finding]) =
    echo C_MUTED & fmt"  Running {name}..." & C_RESET
    let f = check()
    all_findings.add(f)
    echo fmt"  {f.len} findings"

  run_check("log anomaly scan",    proc(): seq[Finding] = scan_logs())
  run_check("open port audit",     proc(): seq[Finding] = audit_ports())
  run_check("file permissions",    proc(): seq[Finding] = audit_permissions())
  run_check("SUID binaries",       proc(): seq[Finding] = audit_suid())
  run_check("telemetry audit",     proc(): seq[Finding] = audit_telemetry())

  # Score: 100 - (critical*20 + warn*5 + info*1)
  var score = 100
  for f in all_findings:
    score -= case f.severity
      of SevCritical: 20
      of SevWarn:     5
      of SevInfo:     1
  score = max(0, min(100, score))

  (all_findings, score)

# ── CLI command ───────────────────────────────────────────────────────────────
proc security_cmd*(args: seq[string]) =
  let sub = if args.len > 0: args[0].toLowerAscii else: "scan"

  case sub
  of "scan","audit":
    let verbose = "--verbose" in args or "-v" in args
    let (findings, score) = run_security_scan(verbose)
    echo ""

    # Print findings grouped by severity
    for sev in [SevCritical, SevWarn, SevInfo]:
      let sev_findings = findings.filterIt(it.severity == sev)
      if sev_findings.len == 0: continue
      echo sev_color(sev) & C_BOLD & $sev & " (" & $sev_findings.len & ")" & C_RESET
      for f in sev_findings:
        echo fmt"  {sev_icon(f.severity)} [{f.category}] {f.description}"
        echo fmt"     → {f.recommendation}"
      echo ""

    # Score
    let score_color = if score >= 80: C_GREEN elif score >= 60: C_YELLOW else: C_RED
    echo score_color & C_BOLD & fmt"Security score: {score}/100" & C_RESET
    if score >= 80: echo C_GREEN & "  ✓ System looks healthy" & C_RESET
    elif score >= 60: echo C_YELLOW & "  ⚠ Some issues need attention" & C_RESET
    else: echo C_RED & "  ✗ Critical issues detected — address immediately" & C_RESET

  of "logs":
    let findings = scan_logs(500)
    for f in findings:
      echo fmt"{sev_icon(f.severity)} {f.description}"
      echo fmt"   → {f.recommendation}"

  of "ports":
    let findings = audit_ports()
    if findings.len == 0: echo C_GREEN & "✓ No suspicious ports detected" & C_RESET
    else:
      for f in findings: echo fmt"{sev_icon(f.severity)} {f.description}\n   → {f.recommendation}"

  of "policies","suggest":
    echo C_CYAN & "\nΣ Security policy recommendations:\n" & C_RESET
    for s in suggest_policies():
      echo fmt"  • {s}"

  of "permissions":
    let findings = audit_permissions() & audit_suid()
    if findings.len == 0: echo C_GREEN & "✓ File permissions look correct" & C_RESET
    else:
      for f in findings: echo fmt"{sev_icon(f.severity)} {f.description}\n   → {f.recommendation}"

  of "telemetry","privacy":
    let findings = audit_telemetry()
    for f in findings: echo fmt"{sev_icon(f.severity)} {f.description}\n   → {f.recommendation}"

  else:
    echo """sigma-agent security — Security advisor sub-agent

Usage:
  sigma-agent security scan         Full security audit + score
  sigma-agent security logs         Scan logs for anomalies
  sigma-agent security ports        Audit open ports
  sigma-agent security permissions  Check file permissions + SUID
  sigma-agent security policies     Policy recommendations
  sigma-agent security telemetry    Privacy / telemetry audit

Examples:
  sigma-agent security scan
  sigma-agent security scan --verbose
  sigma-agent security policies
  sigma-agent security logs
"""
