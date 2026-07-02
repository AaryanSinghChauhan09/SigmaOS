# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_context.nim — System context awareness module
# Collects live OS state, injects it into LLM prompts for tailored responses
#
# Inspiration:
#   Claude Code — workspace context (open files, git diff, diagnostics)
#   Hermes IDE — editor-state-aware prompts
#   Aider — repo-map context injection
#   Copilot CLI — shell context (last command, exit code, env)
#
# Context sources:
#   - CPU/memory/disk/network metrics (/proc, /sys)
#   - Running processes & daemons
#   - Installed packages (sigma-pkg list)
#   - Active kernel modules & loaded drivers
#   - Recent system logs (journald / sigma-log)
#   - Git repo state (branch, dirty files, last commit)
#   - Open editor files & recent changes
#   - Network topology & active connections
#   - Security posture (pledge/unveil state, active policies)
#
# Language: Nim (stdlib only)

import std/[os, osproc, strutils, strformat, tables, json, times, math]

# ── Context Categories ────────────────────────────────────────────────────────
type
  ContextCategory = enum
    CtxCpu, CtxMemory, CtxDisk, CtxNetwork, CtxProcesses,
    CtxPackages, CtxGit, CtxLogs, CtxSecurity, CtxDrivers,
    CtxEditor, CtxSystem

  ContextEntry = object
    category:   ContextCategory
    key:        string
    value:      string
    ts:         int64
    importance: int    # 1=low, 2=med, 3=high (for context budgeting)

  SystemContext = object
    entries:    seq[ContextEntry]
    collected:  DateTime
    token_est:  int   # rough token estimate for LLM budget

# ── Collectors ────────────────────────────────────────────────────────────────

proc collect_cpu(): seq[ContextEntry] =
  try:
    # Load average
    let la = readFile("/proc/loadavg").split()
    result.add ContextEntry(category: CtxCpu, key: "load_avg",
      value: fmt"{la[0]} {la[1]} {la[2]}", importance: 2)
    # CPU count
    let cpus = readFile("/proc/cpuinfo").count("processor\t:")
    result.add ContextEntry(category: CtxCpu, key: "cpu_count",
      value: $cpus, importance: 1)
    # CPU model
    for line in readFile("/proc/cpuinfo").splitLines():
      if line.startsWith("model name"):
        result.add ContextEntry(category: CtxCpu, key: "cpu_model",
          value: line.split(":")[1].strip(), importance: 1)
        break
  except: discard

proc collect_memory(): seq[ContextEntry] =
  try:
    var total = 0'i64; var avail = 0'i64; var cached = 0'i64
    for line in readFile("/proc/meminfo").splitLines():
      let p = line.split()
      if p.len >= 2:
        case p[0]
        of "MemTotal:":      total  = parseInt(p[1])
        of "MemAvailable:":  avail  = parseInt(p[1])
        of "Cached:":        cached = parseInt(p[1])
    if total > 0:
      let used_pct = (total - avail) * 100 div total
      result.add ContextEntry(category: CtxMemory, key: "mem_status",
        value: fmt"{(total-avail) div 1024}MB used / {total div 1024}MB total ({used_pct}% used)",
        importance: if used_pct > 80: 3 elif used_pct > 60: 2 else: 1)
  except: discard

proc collect_disk(): seq[ContextEntry] =
  let (out, code) = execCmdEx("df -h --output=target,pcent,used,size 2>/dev/null | tail -n +2 | head -5")
  if code == 0 and out.len > 0:
    for line in out.strip().splitLines():
      let p = line.split()
      if p.len >= 4:
        let pct_str = p[1].replace("%","")
        let pct = try: parseInt(pct_str) except: 0
        result.add ContextEntry(category: CtxDisk, key: fmt"disk_{p[0].replace('/','_')}",
          value: fmt"{p[0]}: {p[1]} used ({p[2]}/{p[3]})",
          importance: if pct > 90: 3 elif pct > 75: 2 else: 1)

proc collect_network(): seq[ContextEntry] =
  let (ip_out, _) = execCmdEx("ip -brief addr 2>/dev/null")
  if ip_out.len > 0:
    var ifaces: seq[string]
    for line in ip_out.strip().splitLines():
      let p = line.split()
      if p.len >= 3:
        ifaces.add fmt"{p[0]}({p[1]})"
    result.add ContextEntry(category: CtxNetwork, key: "interfaces",
      value: ifaces.join(", "), importance: 2)

  let (conn_out, _) = execCmdEx("ss -tnp 2>/dev/null | tail -n +2 | wc -l")
  result.add ContextEntry(category: CtxNetwork, key: "tcp_connections",
    value: conn_out.strip(), importance: 1)

  let (dns_out, _) = execCmdEx("cat /etc/resolv.conf 2>/dev/null | grep nameserver | awk '{print $2}'")
  result.add ContextEntry(category: CtxNetwork, key: "dns_servers",
    value: dns_out.strip().replace("\n", ", "), importance: 1)

proc collect_processes(top_n = 8): seq[ContextEntry] =
  let (ps_out, code) = execCmdEx("ps aux --sort=-%cpu 2>/dev/null | tail -n +2")
  if code == 0:
    var procs: seq[string]
    for line in ps_out.strip().splitLines()[0..<min(top_n, ps_out.strip().splitLines().len)]:
      let p = line.split()
      if p.len >= 11:
        procs.add fmt"{p[10]}(cpu={p[2]}%,mem={p[3]}%)"
    result.add ContextEntry(category: CtxProcesses, key: "top_processes",
      value: procs.join(", "), importance: 2)

  # Sigma daemons status
  let (sigma_daemons, _) = execCmdEx("ps aux 2>/dev/null | grep sigma | grep -v grep | awk '{print $11}'")
  if sigma_daemons.strip().len > 0:
    result.add ContextEntry(category: CtxProcesses, key: "sigma_daemons",
      value: sigma_daemons.strip().replace("\n", ", "), importance: 2)

proc collect_packages(): seq[ContextEntry] =
  let (pkg_out, code) = execCmdEx("sigma-pkg list 2>/dev/null | head -20")
  if code == 0 and pkg_out.len > 0:
    result.add ContextEntry(category: CtxPackages, key: "installed_packages",
      value: pkg_out.strip().replace("\n", ", ")[0..<min(300, pkg_out.len)],
      importance: 1)

  let (update_out, _) = execCmdEx("sigma-pkg check-updates 2>/dev/null | wc -l")
  let pending = try: parseInt(update_out.strip()) except: 0
  if pending > 0:
    result.add ContextEntry(category: CtxPackages, key: "pending_updates",
      value: fmt"{pending} updates available", importance: 2)

proc collect_git(): seq[ContextEntry] =
  let (is_git, code) = execCmdEx("git rev-parse --git-dir 2>/dev/null")
  if code != 0: return

  let (branch, _) = execCmdEx("git branch --show-current 2>/dev/null")
  result.add ContextEntry(category: CtxGit, key: "branch",
    value: branch.strip(), importance: 2)

  let (status, _) = execCmdEx("git status --short 2>/dev/null | head -10")
  if status.strip().len > 0:
    result.add ContextEntry(category: CtxGit, key: "dirty_files",
      value: status.strip().replace("\n", "; "), importance: 2)

  let (last_commit, _) = execCmdEx("git log -1 --pretty='%h %s' 2>/dev/null")
  result.add ContextEntry(category: CtxGit, key: "last_commit",
    value: last_commit.strip(), importance: 1)

  let (diff_stat, _) = execCmdEx("git diff --stat HEAD 2>/dev/null | tail -1")
  if diff_stat.strip().len > 0:
    result.add ContextEntry(category: CtxGit, key: "diff_stat",
      value: diff_stat.strip(), importance: 2)

proc collect_logs(lines = 20): seq[ContextEntry] =
  # Try sigma-log first, then journald
  let (sigma_log, code) = execCmdEx(fmt"sigma-log -n {lines} --priority err 2>/dev/null")
  if code == 0 and sigma_log.strip().len > 0:
    result.add ContextEntry(category: CtxLogs, key: "recent_errors",
      value: sigma_log.strip()[0..<min(500, sigma_log.len)], importance: 3)
    return

  let (jctl, code2) = execCmdEx(fmt"journalctl -n {lines} -p err --no-pager 2>/dev/null")
  if code2 == 0 and jctl.strip().len > 0:
    result.add ContextEntry(category: CtxLogs, key: "recent_errors",
      value: jctl.strip()[0..<min(500, jctl.len)], importance: 3)

proc collect_security(): seq[ContextEntry] =
  # sigma_pledge status (if kernel supports it)
  let (pledge_out, code) = execCmdEx("sigma-secctl status 2>/dev/null")
  if code == 0:
    result.add ContextEntry(category: CtxSecurity, key: "pledge_status",
      value: pledge_out.strip(), importance: 2)

  # Failed login attempts
  let (auth_fail, _) = execCmdEx("grep -c 'authentication failure' /var/log/auth.log 2>/dev/null")
  let fails = try: parseInt(auth_fail.strip()) except: 0
  if fails > 0:
    result.add ContextEntry(category: CtxSecurity, key: "auth_failures",
      value: fmt"{fails} authentication failures", importance: 3)

  # Open ports
  let (ports, _) = execCmdEx("ss -tlnp 2>/dev/null | tail -n +2 | awk '{print $4}' | cut -d: -f2")
  if ports.strip().len > 0:
    result.add ContextEntry(category: CtxSecurity, key: "listening_ports",
      value: ports.strip().replace("\n", ", "), importance: 2)

proc collect_drivers(): seq[ContextEntry] =
  let (lsmod_out, code) = execCmdEx("lsmod 2>/dev/null | tail -n +2 | awk '{print $1}' | head -10")
  if code == 0 and lsmod_out.strip().len > 0:
    result.add ContextEntry(category: CtxDrivers, key: "loaded_modules",
      value: lsmod_out.strip().replace("\n", ", "), importance: 1)

  # GPU
  let (gpu_out, _) = execCmdEx("lspci 2>/dev/null | grep -i 'vga\\|display\\|3d' | head -2")
  if gpu_out.strip().len > 0:
    result.add ContextEntry(category: CtxDrivers, key: "gpu",
      value: gpu_out.strip()[0..<min(100, gpu_out.len)], importance: 1)

proc collect_system(): seq[ContextEntry] =
  # OS version
  try:
    let os_rel = readFile("/etc/os-release")
    for line in os_rel.splitLines():
      if line.startsWith("PRETTY_NAME"):
        result.add ContextEntry(category: CtxSystem, key: "os_version",
          value: line.split("=")[1].strip().strip(chars={'"'}), importance: 2)
        break
  except:
    result.add ContextEntry(category: CtxSystem, key: "os_version",
      value: "SigmaOS v15.0", importance: 1)

  # Uptime
  try:
    let up_secs = readFile("/proc/uptime").split()[0].split(".")[0]
    let secs = parseInt(up_secs)
    let h = secs div 3600; let m = (secs mod 3600) div 60
    result.add ContextEntry(category: CtxSystem, key: "uptime",
      value: fmt"{h}h {m}m", importance: 1)
  except: discard

  # Hostname
  result.add ContextEntry(category: CtxSystem, key: "hostname",
    value: execCmdEx("hostname")[0].strip(), importance: 1)

# ── Context aggregator ─────────────────────────────────────────────────────────
proc collect_context*(categories: set[ContextCategory] = {},
                      include_all = false): SystemContext =
  result.collected = now()
  let include = if include_all or categories.len == 0:
                  {CtxCpu, CtxMemory, CtxDisk, CtxNetwork, CtxProcesses,
                   CtxPackages, CtxGit, CtxLogs, CtxSecurity, CtxDrivers,
                   CtxSystem}
                else: categories

  if CtxSystem     in include: result.entries.add collect_system()
  if CtxCpu        in include: result.entries.add collect_cpu()
  if CtxMemory     in include: result.entries.add collect_memory()
  if CtxDisk       in include: result.entries.add collect_disk()
  if CtxNetwork    in include: result.entries.add collect_network()
  if CtxProcesses  in include: result.entries.add collect_processes()
  if CtxPackages   in include: result.entries.add collect_packages()
  if CtxGit        in include: result.entries.add collect_git()
  if CtxLogs       in include: result.entries.add collect_logs()
  if CtxSecurity   in include: result.entries.add collect_security()
  if CtxDrivers    in include: result.entries.add collect_drivers()

  # Rough token estimate: ~4 chars per token
  result.token_est = result.entries.foldl(a + b.value.len, 0) div 4

proc to_prompt_string*(ctx: SystemContext, max_tokens = 300): string =
  ## Render context as a compact LLM-injectable string
  ## Prioritises high-importance entries within token budget
  var sorted = ctx.entries.sorted(proc(a,b:ContextEntry):int = b.importance - a.importance)
  var parts:  seq[string]
  var tokens  = 0
  for e in sorted:
    let part = fmt"{e.key}={e.value}"
    tokens += part.len div 4
    if tokens > max_tokens: break
    parts.add(part)
  "SigmaOS context: " & parts.join(" | ")

proc to_json*(ctx: SystemContext): JsonNode =
  var obj = newJObject()
  obj["collected"] = %($ctx.collected)
  obj["token_estimate"] = %ctx.token_est
  var entries = newJArray()
  for e in ctx.entries:
    entries.add(%*{"category": $e.category, "key": e.key,
                   "value": e.value, "importance": e.importance})
  obj["entries"] = entries
  obj

# ── CLI ────────────────────────────────────────────────────────────────────────
proc context_cmd*(args: seq[string]) =
  let fmt_json = "--json" in args
  let categories: set[ContextCategory] = {}  # empty = all

  let ctx = collect_context(include_all = true)

  if fmt_json:
    echo ctx.to_json().pretty()
  else:
    echo "\e[38;2;69;243;255m\e[1mΣ System Context\e[0m  (" & $ctx.entries.len & " entries, ~" & $ctx.token_est & " tokens)\n"
    var cur_cat = CtxSystem
    for e in ctx.entries:
      if e.category != cur_cat:
        cur_cat = e.category
        echo "\e[38;2;107;114;128m  ── " & $e.category & " ──\e[0m"
      let imp_color = case e.importance
        of 3: "\e[38;2;248;113;113m"   # red = high importance
        of 2: "\e[38;2;251;191;36m"    # yellow = medium
        else: "\e[38;2;107;114;128m"   # grey = low
      echo fmt"  {imp_color}{e.key:<22}\e[0m {e.value}"
    echo ""
    echo "\e[38;2;107;114;128mPrompt string (" & $ctx.token_est & " tokens):\e[0m"
    echo ctx.to_prompt_string(400)
