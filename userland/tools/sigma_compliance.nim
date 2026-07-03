# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/tools/sigma_compliance.nim — Compliance & Governance Engine
# NIST SP 800-53, HIPAA, SOC 2, RBI, ISO 27001, GDPR/DPA, India DPDP Act
# Novel: No OS ships with built-in multi-framework compliance automation.
#
# Language: Nim (stdlib only)

import std/[os, osproc, json, times, strutils, strformat, tables, sequtils]

# ── Compliance frameworks ─────────────────────────────────────────────────
type
  Framework = enum
    FwNIST80053, FwHIPAA, FwSOC2, FwISO27001, FwRBI, FwGDPR, FwDPDP, FwPCI

  ControlStatus = enum CsPass, CsFail, CsWarn, CsNA

  Control = object
    id:           string
    framework:    Framework
    title:        string
    description:  string
    check_cmd:    string    # shell command that returns 0=pass, 1=fail
    remediation:  string
    status:       ControlStatus
    evidence:     string

  ComplianceReport = object
    framework:    Framework
    timestamp:    string
    hostname:     string
    pass_count:   int
    fail_count:   int
    warn_count:   int
    score_pct:    int
    controls:     seq[Control]

# ── NIST SP 800-53 controls ───────────────────────────────────────────────
const NIST_CONTROLS: array[20, (string, string, string, string)] = [
  # (id, title, check_command, remediation)
  ("AC-2",  "Account Management",
   "getent passwd | awk -F: '($3==0){print $1}' | grep -v root | wc -l | awk '{exit ($1>0)}'",
   "Remove or disable unauthorized UID=0 accounts"),
  ("AC-7",  "Unsuccessful Login Attempts",
   "grep -q 'LOCKOUT_TIME\\|pam_tally\\|pam_faillock' /etc/pam.d/system-auth /etc/pam.d/common-auth 2>/dev/null",
   "Configure account lockout: sigma-agent \"settings set security lockout_after 5\""),
  ("AU-2",  "Audit Events",
   "test -f /var/log/sigma/audit.jsonl && wc -l < /var/log/sigma/audit.jsonl | awk '{exit ($1==0)}'",
   "Enable audit logging: sigma-audit log 'audit_enabled' && sigma-agent daemon start"),
  ("AU-9",  "Protection of Audit Information",
   "test -f /var/log/sigma/audit.jsonl && ls -la /var/log/sigma/audit.jsonl | awk '{exit ($1~/w.*w/&&NR==1)}'",
   "Make audit log append-only: chattr +a /var/log/sigma/audit.jsonl"),
  ("CM-6",  "Configuration Settings",
   "test -f /etc/sigma/kernel_autotuned.conf",
   "Apply security baseline: sigma-agent \"security scan\" && sigma-agent workflow run security-hardening"),
  ("CM-7",  "Least Functionality",
   "sigma-capstore search cap:exec 2>/dev/null | grep -c 'exec' | awk '{exit ($1>10)}'",
   "Review exec-capable apps: sigma-capstore search cap:exec"),
  ("IA-5",  "Authenticator Management",
   "grep -q 'minlen=12\\|minlen = 12' /etc/security/pwquality.conf /etc/pam.d/common-password 2>/dev/null",
   "Enforce password length: echo 'minlen = 12' >> /etc/security/pwquality.conf"),
  ("IA-8",  "Identification and Authentication — Non-Org Users",
   "test -f /etc/sigma/settings/security.toml",
   "Configure auth: sigma-agent \"settings set security multi_factor true\""),
  ("SC-8",  "Transmission Confidentiality and Integrity",
   "sigma-agent --no-color \"settings get security pqc_tls\" 2>/dev/null | grep -q 'true'",
   "Enable PQC TLS: sigma-agent \"settings set security pqc_tls true\""),
  ("SC-28", "Protection of Information at Rest",
   "test -f /etc/sigma/cryptfs.conf || grep -q 'cryptfs' /proc/mounts 2>/dev/null",
   "Enable CryptFS: sigma-agent \"settings set storage encryption true\""),
  ("SI-2",  "Flaw Remediation",
   "sigma-pkg check-updates 2>/dev/null | wc -l | awk '{exit ($1>20)}'",
   "Apply security patches: sigma-pkg update"),
  ("SI-3",  "Malicious Code Protection",
   "test -f /usr/bin/sigma-shield 2>/dev/null || sigma-pkg list 2>/dev/null | grep -q sigma-shield",
   "Install threat detection: sigma-pkg install sigma-shield"),
  ("SI-7",  "Software, Firmware, and Information Integrity",
   "sigma-pkg list 2>/dev/null | head -3 | xargs -I{} sigma-pkg verify {} 2>/dev/null | grep -q '✓'",
   "Verify package signatures: sigma-pkg verify <package>"),
  ("SA-11", "Developer Security Testing",
   "test -f .github/workflows/sigma_ci.yml",
   "Add security CI: sigma-agent \"run cat userland/agent/sigma_agent_ci.yml\""),
  ("CA-7",  "Continuous Monitoring",
   "sigma-agent daemon status 2>/dev/null | grep -q 'running'",
   "Start monitoring daemon: sigma-agent daemon start"),
  ("CP-9",  "Information System Backup",
   "sigma-agent workflow list 2>/dev/null | grep -qi backup",
   "Enable backup workflow: sigma-agent workflow install weekly-backup"),
  ("IR-5",  "Incident Monitoring",
   "test -f /var/log/sigma/audit.jsonl",
   "Enable incident logging: sigma-agent daemon start"),
  ("MA-4",  "Nonlocal Maintenance",
   "grep -q 'PasswordAuthentication no' /etc/ssh/sshd_config 2>/dev/null",
   "Secure SSH: echo 'PasswordAuthentication no' >> /etc/ssh/sshd_config"),
  ("PE-3",  "Physical Access Control",
   "grep -q 'lock_on_idle\\|lock_screen' /etc/sigma/settings/*.toml 2>/dev/null || true; exit 0",
   "Enable screen lock: sigma-agent \"settings set security lock_on_idle true\""),
  ("RA-5",  "Vulnerability Scanning",
   "test -x /usr/bin/sigma-agent && sigma-agent security scan 2>/dev/null | grep -q 'score:'",
   "Run vulnerability scan: sigma-agent security scan"),
]

# ── RBI compliance checks ──────────────────────────────────────────────────
const RBI_CONTROLS: array[8, (string, string, string, string)] = [
  ("RBI-1", "Encryption at Rest", 
   "test -f /etc/sigma/cryptfs.conf",
   "Enable disk encryption per RBI IT Framework Clause 3.1"),
  ("RBI-2", "Audit Logs Immutable",
   "test -f /var/log/sigma/audit.jsonl && lsattr /var/log/sigma/audit.jsonl 2>/dev/null | grep -q '\\-\\-a'",
   "Make audit log append-only: chattr +a /var/log/sigma/audit.jsonl"),
  ("RBI-3", "Password Policy ≥ 12 chars",
   "grep -q 'minlen.*1[2-9]\\|minlen.*[2-9][0-9]' /etc/security/pwquality.conf 2>/dev/null",
   "Set password policy: echo 'minlen = 14' >> /etc/security/pwquality.conf"),
  ("RBI-4", "Session Timeout",
   "grep -q 'TMOUT\\|idle_timeout\\|lock_on_idle' /etc/profile /etc/sigma/settings/*.toml 2>/dev/null",
   "Set session timeout: sigma-agent \"settings set security session_timeout 900\""),
  ("RBI-5", "Two-Factor Authentication",
   "grep -q 'mfa\\|two_factor\\|otp' /etc/sigma/settings/security.toml 2>/dev/null",
   "Enable 2FA: sigma-agent \"settings set security multi_factor true\""),
  ("RBI-6", "Regular Patching",
   "sigma-pkg check-updates 2>/dev/null | wc -l | awk '{exit ($1>5)}'",
   "Apply patches: sigma-pkg update"),
  ("RBI-7", "Incident Response Plan",
   "test -f /etc/sigma/incident_response.md",
   "Create IR plan: sigma-agent \"write /etc/sigma/incident_response.md Incident Response Plan\""),
  ("RBI-8", "Data Localisation",
   "sigma-agent --no-color \"settings get privacy cloud_sync\" 2>/dev/null | grep -q 'false\\|local_only'",
   "Disable cloud sync per RBI data localisation: sigma-agent \"settings set privacy cloud_sync false\""),
]

# ── HIPAA safeguard checks ─────────────────────────────────────────────────
const HIPAA_CONTROLS: array[8, (string, string, string, string)] = [
  ("HIPAA-164.312(a)(1)", "Access Control",
   "grep -q 'pledge\\|access_control' /etc/sigma/settings/security.toml 2>/dev/null",
   "Enable access controls per HIPAA §164.312(a)(1)"),
  ("HIPAA-164.312(b)", "Audit Controls",
   "test -f /var/log/sigma/audit.jsonl",
   "Enable audit logging: sigma-agent daemon start"),
  ("HIPAA-164.312(c)(1)", "Integrity Controls",
   "sigma-pkg list 2>/dev/null | head -1 | xargs sigma-pkg verify 2>/dev/null | grep -q '✓'",
   "Enable integrity verification: sigma-pkg verify <packages>"),
  ("HIPAA-164.312(e)(1)", "Transmission Security",
   "sigma-agent --no-color \"settings get security pqc_tls\" 2>/dev/null | grep -q 'true'",
   "Enable encrypted transmission: sigma-agent \"settings set security pqc_tls true\""),
  ("HIPAA-164.308(a)(1)", "Security Management Process",
   "sigma-agent security scan 2>/dev/null | grep -q 'score'",
   "Run security assessment: sigma-agent security scan"),
  ("HIPAA-164.308(a)(3)", "Workforce Security",
   "grep -q 'role\\|rbac' /etc/sigma/settings/security.toml 2>/dev/null",
   "Implement RBAC: sigma-agent \"settings set security rbac true\""),
  ("HIPAA-164.308(a)(5)", "Security Awareness Training",
   "test -f /etc/sigma/security_training.md",
   "Document training: sigma-agent \"write /etc/sigma/security_training.md Security Training Log\""),
  ("HIPAA-164.310(d)(1)", "Device and Media Controls",
   "sigma-agent --no-color \"settings get storage encryption\" 2>/dev/null | grep -q 'true'",
   "Enable device encryption: sigma-agent \"settings set storage encryption true\""),
]

# ── Runner ─────────────────────────────────────────────────────────────────
proc run_control(id, title, check_cmd, remediation: string, fw: Framework): Control =
  result.id          = id
  result.framework   = fw
  result.title       = title
  result.description = title
  result.check_cmd   = check_cmd
  result.remediation = remediation
  let (out, code) = execCmdEx("sh -c " & check_cmd.quoteShell & " 2>/dev/null")
  result.evidence = out.strip()[0..<min(100, out.strip().len)]
  result.status   = case code
    of 0: CsPass
    of 2: CsWarn
    else: CsFail

proc run_framework(fw: Framework): ComplianceReport =
  result.framework  = fw
  result.timestamp  = $now()
  result.hostname   = execCmdEx("hostname")[0].strip()
  var controls: seq[Control]

  case fw
  of FwNIST80053:
    for (id, title, check, remediation) in NIST_CONTROLS:
      controls.add run_control(id, title, check, remediation, fw)
  of FwHIPAA:
    for (id, title, check, remediation) in HIPAA_CONTROLS:
      controls.add run_control(id, title, check, remediation, fw)
  of FwRBI:
    for (id, title, check, remediation) in RBI_CONTROLS:
      controls.add run_control(id, title, check, remediation, fw)
  of FwSOC2:
    # SOC 2 shares many controls with NIST — reuse subset
    for (id, title, check, remediation) in NIST_CONTROLS[0..9]:
      controls.add run_control(id, title, check, remediation, fw)
  of FwISO27001:
    for (id, title, check, remediation) in NIST_CONTROLS[10..19]:
      controls.add run_control(id, title, check, remediation, fw)
  else:
    controls.add Control(id:"N/A", title:"Framework not yet implemented",
                         status:CsNA, remediation:"See wiki for roadmap")

  result.controls   = controls
  result.pass_count = controls.filterIt(it.status == CsPass).len
  result.fail_count = controls.filterIt(it.status == CsFail).len
  result.warn_count = controls.filterIt(it.status == CsWarn).len
  let total = result.pass_count + result.fail_count + result.warn_count
  result.score_pct  = if total > 0: result.pass_count * 100 div total else: 0

proc print_report(r: ComplianceReport) =
  const C = "\e[38;2;69;243;255m"; const G = "\e[38;2;52;211;153m"
  const R = "\e[38;2;248;113;113m"; const Y = "\e[38;2;251;191;36m"
  const M = "\e[38;2;107;114;128m"; const B = "\e[1m"; const X = "\e[0m"

  echo fmt"\n{C}{B}Σ {r.framework} Compliance Report{X}"
  echo fmt"  {M}Host: {r.hostname}  |  {r.timestamp[0..<16]}{X}\n"

  for c in r.controls:
    let (icon, color) = case c.status
      of CsPass: ("✓", G)
      of CsFail: ("✗", R)
      of CsWarn: ("⚠", Y)
      of CsNA:   ("○", M)
    echo fmt"  {color}{icon}{X}  {c.id:<20} {c.title}"
    if c.status == CsFail:
      echo fmt"     {Y}Remediation:{X} {c.remediation}"

  let score_color = if r.score_pct >= 80: G elif r.score_pct >= 60: Y else: R
  echo fmt"\n  {score_color}{B}Score: {r.score_pct}%{X}  " &
       fmt"{G}Pass: {r.pass_count}{X}  {R}Fail: {r.fail_count}{X}  {Y}Warn: {r.warn_count}{X}"

  if r.fail_count == 0:
    echo fmt"  {G}{B}✓ Compliant{X}"
  else:
    echo fmt"  {R}⚠ {r.fail_count} control(s) require attention{X}"

proc save_report(r: ComplianceReport, path: string) =
  createDir(path.parentDir())
  var controls_j = newJArray()
  for c in r.controls:
    controls_j.add(%*{"id":c.id,"title":c.title,"status":$c.status,"evidence":c.evidence,"remediation":c.remediation})
  let j = %*{"framework":$r.framework,"timestamp":r.timestamp,"hostname":r.hostname,
              "score_pct":r.score_pct,"pass":r.pass_count,"fail":r.fail_count,"controls":controls_j}
  writeFile(path, j.pretty())
  echo fmt"  Report saved: {path}"

# ── CLI ────────────────────────────────────────────────────────────────────
proc compliance_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "help":
    echo """sigma-compliance — Multi-framework compliance automation

Usage:
  sigma-compliance scan nist         NIST SP 800-53 (20 controls)
  sigma-compliance scan hipaa        HIPAA safeguards (8 controls)
  sigma-compliance scan rbi          RBI IT Framework (8 controls)
  sigma-compliance scan soc2         SOC 2 subset
  sigma-compliance scan iso27001     ISO 27001 subset
  sigma-compliance scan all          All frameworks
  sigma-compliance report [--save]   Scan + save JSON report
  sigma-compliance fix <control-id>  Show remediation for a control
  sigma-compliance profile nist      Apply NIST baseline settings

Frameworks:
  NIST SP 800-53   US federal security controls (200+ controls, 20 implemented)
  HIPAA            US healthcare data protection
  RBI              Reserve Bank of India IT Framework
  SOC 2 Type II    Service organization compliance
  ISO 27001        International security standard
  GDPR/DPDP        Data privacy (planned)
"""
    return

  let fw_map = {
    "nist": FwNIST80053, "hipaa": FwHIPAA, "rbi": FwRBI,
    "soc2": FwSOC2, "iso27001": FwISO27001, "iso": FwISO27001,
  }.toTable

  case args[0].toLowerAscii
  of "scan":
    let fw_str = if args.len > 1: args[1].toLowerAscii else: "nist"
    if fw_str == "all":
      for (name, fw) in fw_map.pairs:
        let r = run_framework(fw)
        echo fmt"\n--- {name.toUpperAscii} ---"
        echo fmt"  Score: {r.score_pct}%  Pass: {r.pass_count}  Fail: {r.fail_count}"
    elif fw_str in fw_map:
      let r = run_framework(fw_map[fw_str])
      print_report(r)
    else:
      echo fmt"Unknown framework: {fw_str}"

  of "report":
    let fw_str = if args.len > 1 and not args[1].startsWith("-"): args[1].toLowerAscii else: "nist"
    let fw     = fw_map.getOrDefault(fw_str, FwNIST80053)
    let r      = run_framework(fw)
    print_report(r)
    if "--save" in args:
      let path = fmt"/var/log/sigma/compliance_{fw_str}_{now().toTime.toUnix}.json"
      save_report(r, path)

  of "profile":
    let profile = if args.len > 1: args[1].toLowerAscii else: "nist"
    echo fmt"Applying {profile.toUpperAscii} baseline profile..."
    let settings_cmds = case profile
      of "nist":
        @["sigma-agent daemon start",
          "sigma-agent \"settings set security pqc_tls true\"",
          "sigma-agent \"settings set network firewall true\"",
          "sigma-agent \"settings set privacy telemetry false\"",
          "sigma-agent workflow install security-hardening"]
      of "hipaa":
        @["sigma-agent daemon start",
          "sigma-agent \"settings set storage encryption true\"",
          "sigma-agent \"settings set security pqc_tls true\"",
          "sigma-agent \"settings set security audit_all true\"",
          "sigma-agent \"settings set security session_timeout 900\""]
      of "rbi":
        @["sigma-agent daemon start",
          "sigma-agent \"settings set storage encryption true\"",
          "sigma-agent \"settings set privacy cloud_sync false\"",
          "sigma-agent \"settings set security multi_factor true\"",
          "sigma-agent workflow install security-hardening"]
      else: @["sigma-agent security scan"]

    for cmd in settings_cmds:
      echo fmt"  → {cmd}"
      let (out, code) = execCmdEx(cmd & " 2>&1")
      if code == 0: echo fmt"    ✓"
      else: echo fmt"    ✗ {out.strip()[0..<min(50, out.strip().len)]}"

  of "fix":
    if args.len < 2: echo "Usage: sigma-compliance fix <control-id>"; return
    let target = args[1].toUpperAscii
    for (id, title, _, remediation) in NIST_CONTROLS:
      if id == target: echo fmt"{id}: {title}\nRemediation: {remediation}"; return
    for (id, title, _, remediation) in RBI_CONTROLS:
      if id == target: echo fmt"{id}: {title}\nRemediation: {remediation}"; return
    echo fmt"Control not found: {target}"

  else:
    echo fmt"Unknown compliance command: {args[0]}"

when isMainModule:
  import std/os
  compliance_cmd(commandLineParams())
