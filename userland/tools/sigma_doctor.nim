# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
# userland/tools/sigma_doctor.nim — sigma-doctor: Self-Diagnostics + Repair
# Language: Nim — native, OOP via object + methods

import std/[os, strutils, osproc, strformat, tables, sets]

# ── Severity ──────────────────────────────────────────────────────────────────
type
  Severity = enum Ok, Info, Warn, Error, Critical

  DiagResult = object
    name:     string
    severity: Severity
    message:  string
    fix:      string  # suggested fix command

  DoctorReport = object
    checks:  seq[DiagResult]
    ok_cnt:  int
    warn_cnt: int
    err_cnt:  int

# ── Check Base (OOP) ──────────────────────────────────────────────────────────
type CheckFn = proc(): DiagResult {.nimcall.}

proc make_result(name: string, ok: bool, msg, fix: string): DiagResult =
  DiagResult(name: name, severity: if ok: Ok else: Error,
             message: msg, fix: fix)

# ── Individual Checks ─────────────────────────────────────────────────────────

proc check_kernel_version(): DiagResult =
  let (out, code) = execCmdEx("uname -r")
  let ver = out.strip()
  if code == 0 and ver.len > 0:
    DiagResult(name: "kernel", severity: Ok, message: "Kernel: " & ver, fix: "")
  else:
    DiagResult(name: "kernel", severity: Error, message: "Cannot read kernel version", fix: "")

proc check_disk_space(): DiagResult =
  let (out, _) = execCmdEx("df -h / 2>/dev/null | tail -1")
  let parts = out.splitWhitespace()
  if parts.len >= 5:
    let use_pct_str = parts[4].replace("%","")
    let use_pct = try: parseInt(use_pct_str) except: 0
    if use_pct >= 95:
      return DiagResult(name: "disk_space", severity: Critical,
        message: fmt"Root disk {use_pct}% full",
        fix: "sigma-pkg clean && rm -rf /tmp/*")
    elif use_pct >= 80:
      return DiagResult(name: "disk_space", severity: Warn,
        message: fmt"Root disk {use_pct}% full",
        fix: "sigma-pkg clean")
    return DiagResult(name: "disk_space", severity: Ok,
      message: fmt"Root disk {use_pct}% used", fix: "")
  DiagResult(name: "disk_space", severity: Info, message: "Could not check disk space", fix: "")

proc check_memory(): DiagResult =
  if not fileExists("/proc/meminfo"): return DiagResult(name: "memory", severity: Info, message: "No /proc/meminfo", fix: "")
  var total, avail: int
  for line in lines("/proc/meminfo"):
    let p = line.splitWhitespace()
    if p.len >= 2:
      if p[0] == "MemTotal:":   total = parseInt(p[1])
      if p[0] == "MemAvailable:": avail = parseInt(p[1])
  let used_pct = if total > 0: 100 * (total - avail) div total else: 0
  if used_pct >= 90:
    DiagResult(name: "memory", severity: Critical,
      message: fmt"Memory {used_pct}% used ({(total-avail) div 1024}MB / {total div 1024}MB)",
      fix: "sigma-monitor to find memory hogs; restart services")
  elif used_pct >= 75:
    DiagResult(name: "memory", severity: Warn,
      message: fmt"Memory {used_pct}% used", fix: "")
  else:
    DiagResult(name: "memory", severity: Ok,
      message: fmt"Memory {used_pct}% used ({(total-avail) div 1024}MB / {total div 1024}MB)", fix: "")

proc check_services(): DiagResult =
  let required = ["sigma-sh", "sigmad-health"]
  var missing: seq[string]
  for svc in required:
    let (_, code) = execCmdEx(fmt"pgrep -x {svc}")
    if code != 0: missing.add(svc)
  if missing.len > 0:
    DiagResult(name: "services", severity: Error,
      message: "Services not running: " & missing.join(", "),
      fix: "sigma-init start " & missing.join(" "))
  else:
    DiagResult(name: "services", severity: Ok, message: "All required services running", fix: "")

proc check_network(): DiagResult =
  let (_, code) = execCmdEx("ip route show default 2>/dev/null")
  if code != 0:
    return DiagResult(name: "network_route", severity: Warn,
      message: "No default route configured",
      fix: "sigma-netctl up <iface> && sigma-netctl dhcp <iface>")
  let (dns_out, _) = execCmdEx("cat /etc/resolv.conf 2>/dev/null")
  if not dns_out.contains("nameserver"):
    return DiagResult(name: "network_dns", severity: Warn,
      message: "No DNS nameserver configured",
      fix: "sigma-netctl dns 1.1.1.1")
  DiagResult(name: "network", severity: Ok, message: "Network connectivity OK", fix: "")

proc check_updates(): DiagResult =
  let (_, code) = execCmdEx("sigma-pkg check-updates 2>/dev/null")
  if code == 0:
    DiagResult(name: "updates", severity: Ok, message: "System is up to date", fix: "")
  else:
    DiagResult(name: "updates", severity: Info,
      message: "Updates may be available",
      fix: "sigma-pkg update")

proc check_security(): DiagResult =
  var issues: seq[string]
  # Check /tmp permissions
  let (ls_out, _) = execCmdEx("ls -ld /tmp 2>/dev/null")
  if ls_out.len > 0 and not ls_out.startsWith("drwxrwxrwt"):
    issues.add("/tmp permissions incorrect")
  # Check for world-writable files in /etc
  let (ww_out, _) = execCmdEx("find /etc -maxdepth 1 -perm -o+w 2>/dev/null")
  if ww_out.strip().len > 0:
    issues.add("World-writable files in /etc")
  if issues.len > 0:
    DiagResult(name: "security", severity: Warn,
      message: "Security issues: " & issues.join("; "),
      fix: "chmod 1777 /tmp && sigma-doctor fix-security")
  else:
    DiagResult(name: "security", severity: Ok, message: "Security checks passed", fix: "")

proc check_sigma_pkg(): DiagResult =
  let (_, code) = execCmdEx("sigma-pkg --version 2>/dev/null")
  if code == 0:
    DiagResult(name: "sigma_pkg", severity: Ok, message: "sigma-pkg is functional", fix: "")
  else:
    DiagResult(name: "sigma_pkg", severity: Error,
      message: "sigma-pkg not found or broken",
      fix: "reinstall sigma-pkg via live ISO")

proc check_cpu_temp(): DiagResult =
  when defined(linux):
    let thermal = "/sys/class/thermal/thermal_zone0/temp"
    if fileExists(thermal):
      let temp_str = readFile(thermal).strip()
      let temp_c = try: parseInt(temp_str) div 1000 except: 0
      if temp_c >= 90:
        return DiagResult(name: "cpu_temp", severity: Critical,
          message: fmt"CPU temperature critical: {temp_c}°C",
          fix: "Check cooling, reduce load")
      elif temp_c >= 75:
        return DiagResult(name: "cpu_temp", severity: Warn,
          message: fmt"CPU temperature elevated: {temp_c}°C", fix: "")
      return DiagResult(name: "cpu_temp", severity: Ok,
        message: fmt"CPU temperature: {temp_c}°C", fix: "")
  DiagResult(name: "cpu_temp", severity: Info, message: "CPU temperature: unavailable", fix: "")

# ── Auto-Fix ──────────────────────────────────────────────────────────────────
proc auto_fix(r: DiagResult): bool =
  if r.fix.len == 0: return false
  echo fmt"  → Attempting fix: {r.fix}"
  let (_, code) = execCmdEx(r.fix)
  code == 0

# ── Run All ───────────────────────────────────────────────────────────────────
proc run_all(auto_repair = false): DoctorReport =
  let checks: seq[CheckFn] = @[
    check_kernel_version, check_disk_space, check_memory,
    check_services, check_network, check_updates,
    check_security, check_sigma_pkg, check_cpu_temp
  ]
  for check in checks:
    let r = check()
    result.checks.add(r)
    case r.severity
    of Ok:       result.ok_cnt  += 1
    of Warn:     result.warn_cnt += 1
    of Error, Critical: result.err_cnt += 1
    else: discard
    if auto_repair and r.severity in {Error, Critical, Warn} and r.fix.len > 0:
      discard auto_fix(r)

proc print_report(rep: DoctorReport) =
  const icons = [Ok: "✓", Info: "ℹ", Warn: "⚠", Error: "✗", Critical: "🔴"]
  const colors = [Ok: "\e[32m", Info: "\e[36m", Warn: "\e[33m", Error: "\e[31m", Critical: "\e[35m"]
  const reset  = "\e[0m"
  echo "\n=== sigma-doctor Report ===\n"
  for r in rep.checks:
    echo fmt"{colors[r.severity]}{icons[r.severity]} [{r.name}] {r.message}{reset}"
    if r.fix.len > 0: echo fmt"    Fix: {r.fix}"
  echo fmt"\n{reset}Summary: {rep.ok_cnt} OK  {rep.warn_cnt} warnings  {rep.err_cnt} errors"
  if rep.err_cnt == 0 and rep.warn_cnt == 0:
    echo "\e[32m✓ System is healthy!\e[0m"

proc main() =
  import std/parseopt
  var auto_repair = false
  for kind, key, _ in getopt():
    case kind
    of cmdOption:
      if key in ["fix", "repair", "auto"]: auto_repair = true
      elif key in ["h","help"]:
        echo "sigma-doctor [--fix]  Run diagnostics, optionally auto-repair"
        quit(0)
    else: discard
  let rep = run_all(auto_repair)
  print_report(rep)
  quit(if rep.err_cnt > 0: 1 else: 0)

main()
