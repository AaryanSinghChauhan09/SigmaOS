# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_capability_store.nim — Capability-based App Store
# Novel: Filter/search apps by which syscalls/capabilities they require.
# Transparency: users see EXACTLY what each app needs before installing.
#
# Inspired by: Android permission model, but for OS-level syscalls
# Goes beyond: No existing distro shows syscall-level transparency pre-install
#
# Language: Nim (stdlib only)

import std/[os, json, strutils, strformat, tables, sequtils, osproc]

# ── Capability categories ─────────────────────────────────────────────────
type
  CapCategory = enum
    CapNet      = "network"       # socket, connect, bind, sendto
    CapFiles    = "filesystem"    # open, read, write, unlink, mkdir
    CapProc     = "process"       # fork, exec, kill, wait
    CapDevice   = "device"        # ioctl, /dev access
    CapMem      = "memory"        # mmap large regions, /proc/mem
    CapSecurity = "security"      # setuid, capabilities, pledge
    CapGpu      = "gpu"           # DRM/KMS ioctl, GPU memory
    CapAudio    = "audio"         # /dev/snd, PipeWire socket
    CapCamera   = "camera"        # /dev/video
    CapBluetooth= "bluetooth"     # HCI socket
    CapCrypto   = "crypto"        # TPM2, hardware RNG
    CapAi       = "ai-inference"  # sigma-ai socket
    CapNone     = "none"

  AppCapabilities = object
    app_name:    string
    version:     string
    description: string
    # Pledge capabilities required
    pledge_caps: seq[string]      # "stdio", "rpath", "inet", etc.
    # Unveil paths required
    unveil_paths: seq[string]     # "/home:r", "/tmp:rw"
    # Syscalls explicitly used
    syscalls:    seq[string]
    # Human-readable categories
    categories:  seq[CapCategory]
    # Risk score 0-10
    risk_score:  int
    # Justification for each cap
    justifications: Table[string, string]

# ── Risk scorer ───────────────────────────────────────────────────────────
proc compute_risk(caps: AppCapabilities): int =
  var score = 0
  for pledge in caps.pledge_caps:
    case pledge.toLowerAscii
    of "exec":  score += 3
    of "proc":  score += 2
    of "inet":  score += 2
    of "wpath": score += 1
    of "dpath": score += 2
    of "cpath": score += 1
    else: discard
  if caps.pledge_caps.len > 6: score += 1
  if caps.unveil_paths.anyIt(it.endsWith(":rw") and it.startsWith("/home")): score += 1
  score.min(10)

# ── Static capability database (from sigpkg manifests) ───────────────────
proc load_app_caps(app_name: string): AppCapabilities =
  result.app_name   = app_name
  result.justifications = initTable[string, string]()

  # Try reading from installed sigpkg manifest
  let manifest_paths = [
    fmt"/usr/share/sigma/caps/{app_name}.json",
    getEnv("HOME","/tmp") / fmt".cache/sigma/caps/{app_name}.json",
    fmt"sigma_pkg_registry/caps/{app_name}.json",
  ]
  for path in manifest_paths:
    if fileExists(path):
      try:
        let j = parseJson(readFile(path))
        result.version      = j.getOrDefault("version").getStr("?")
        result.description  = j.getOrDefault("description").getStr("")
        result.pledge_caps  = j.getOrDefault("pledge").getElems.mapIt(it.getStr)
        result.unveil_paths = j.getOrDefault("unveil").getElems.mapIt(it.getStr)
        result.syscalls     = j.getOrDefault("syscalls").getElems.mapIt(it.getStr)
        if j.hasKey("justifications"):
          for k, v in j["justifications"]: result.justifications[k] = v.getStr()
        result.risk_score   = compute_risk(result)
        return
      except: discard

  # Fallback: scan binary with static analysis
  let bin_paths = [fmt"/usr/bin/{app_name}", fmt"/usr/local/bin/{app_name}"]
  for bin in bin_paths:
    if fileExists(bin):
      result.version = "installed"
      # Use nm/objdump to detect syscall wrappers
      let (syms_out, _) = execCmdEx(fmt"nm -D {bin.quoteShell} 2>/dev/null | grep -i 'socket\\|open\\|fork\\|exec'")
      if "socket" in syms_out: result.pledge_caps.add("inet")
      if "fork" in syms_out or "exec" in syms_out: result.pledge_caps.add("exec")
      if "open" in syms_out: result.pledge_caps.add("rpath")
      result.risk_score = compute_risk(result)
      return

# ── Capability filter ─────────────────────────────────────────────────────
proc matches_filter(caps: AppCapabilities, filter: string): bool =
  let lower = filter.toLowerAscii
  # Filter by capability
  if lower.startsWith("cap:"):
    let cap = lower[4..^1]
    return caps.pledge_caps.anyIt(it.toLowerAscii.contains(cap)) or
           caps.categories.anyIt($it == cap)
  # Filter by max risk
  if lower.startsWith("risk<"):
    let max_risk = try: parseInt(lower[5..^1]) except: 10
    return caps.risk_score < max_risk
  # Filter by syscall
  if lower.startsWith("syscall:"):
    let syscall = lower[8..^1]
    return caps.syscalls.anyIt(it.toLowerAscii.contains(syscall))
  # General search
  caps.app_name.toLowerAscii.contains(lower) or
    caps.description.toLowerAscii.contains(lower)

# ── App store search ──────────────────────────────────────────────────────
proc search_apps(query: string, max_results = 20): seq[AppCapabilities] =
  var results: seq[AppCapabilities]

  # Search installed packages
  let (pkg_list, _) = execCmdEx("sigma-pkg list 2>/dev/null")
  for line in pkg_list.splitLines():
    let app = line.split()[0].strip()
    if app.len == 0: continue
    let caps = load_app_caps(app)
    if query.len == 0 or matches_filter(caps, query):
      results.add(caps)
    if results.len >= max_results: break

  results.sortedByIt(it.risk_score)

# ── Display helpers ───────────────────────────────────────────────────────
proc risk_color(score: int): string =
  if score <= 2: "\e[38;2;52;211;153m"      # green
  elif score <= 5: "\e[38;2;251;191;36m"    # yellow
  else: "\e[38;2;248;113;113m"              # red

proc risk_label(score: int): string =
  if score <= 2: "Low"
  elif score <= 5: "Medium"
  elif score <= 7: "High"
  else: "Very High"

proc display_app(caps: AppCapabilities, verbose = false) =
  let rc = risk_color(caps.risk_score)
  let reset = "\e[0m"
  echo fmt"  \e[1m{caps.app_name}\e[0m  v{caps.version}"
  if caps.description.len > 0:
    echo fmt"    {caps.description}"
  echo fmt"    Risk: {rc}{risk_label(caps.risk_score)} ({caps.risk_score}/10){reset}"
  if caps.pledge_caps.len > 0:
    echo fmt"    Capabilities: {caps.pledge_caps.join(\", \")}"
  if caps.unveil_paths.len > 0:
    echo fmt"    File access:  {caps.unveil_paths.join(\", \")}"
  if verbose and caps.justifications.len > 0:
    echo "    Justifications:"
    for k, v in caps.justifications:
      echo fmt"      {k}: {v}"
  echo ""

# ── Interactive capability comparison ────────────────────────────────────
proc compare_apps*(apps: seq[string]) =
  echo "\e[38;2;69;243;255m\e[1mΣ Capability Comparison\e[0m\n"
  let all_caps: seq[AppCapabilities] = apps.mapIt(load_app_caps(it))

  # Print comparison table
  let max_name = all_caps.mapIt(it.app_name.len).foldl(max(a,b), 0)
  echo "  " & "App".ljust(max_name+2) & "Risk  Caps"
  echo "  " & "─".repeat(60)
  for caps in all_caps:
    let rc = risk_color(caps.risk_score)
    let reset = "\e[0m"
    let pledge_str = caps.pledge_caps.join(",")[0..<min(30, caps.pledge_caps.join(",").len)]
    echo fmt"  {caps.app_name.ljust(max_name+2)}{rc}{caps.risk_score:>3}{reset}   {pledge_str}"

# ── Generate sigpkg capability manifest ──────────────────────────────────
proc generate_manifest*(app_name, binary_path: string): string =
  var caps = AppCapabilities(app_name: app_name, justifications: initTable[string,string]())

  # Static analysis via objdump/nm
  let (sym_out, _) = execCmdEx(fmt"nm -D {binary_path.quoteShell} 2>/dev/null")
  if "socket"  in sym_out: caps.pledge_caps.add("inet")
  if "fork"    in sym_out: caps.pledge_caps.add("proc"); caps.pledge_caps.add("exec")
  if "open"    in sym_out: caps.pledge_caps.add("rpath")
  if "write"   in sym_out and "open" in sym_out: caps.pledge_caps.add("wpath")
  if "ioctl"   in sym_out: caps.pledge_caps.add("dpath")
  caps.pledge_caps = caps.pledge_caps.deduplicate()
  caps.risk_score  = compute_risk(caps)

  let pledge_json = caps.pledge_caps.mapIt(fmt"\"{it}\"").join(",")
  fmt"""{{
  "name":    "{app_name}",
  "version": "0.1.0",
  "pledge":  [{pledge_json}],
  "unveil":  [],
  "risk":    {caps.risk_score},
  "justifications": {{}}
}}"""

# ── CLI ────────────────────────────────────────────────────────────────────
proc capability_store_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "help":
    echo """sigma-capstore — Capability-based App Store

Usage:
  sigma-capstore search [query]           Search apps by name/description
  sigma-capstore search cap:network       Apps that use network
  sigma-capstore search risk<3            Apps with risk score < 3 (safest)
  sigma-capstore search syscall:fork      Apps that call fork()
  sigma-capstore show <app>               Show capability details
  sigma-capstore compare <app1> <app2>    Compare two apps' capabilities
  sigma-capstore manifest <binary>        Generate capability manifest for a binary
  sigma-capstore install <app>            Install with capability review

Risk scores:
  0-2  Low     (read-only, no network, no process spawning)
  3-5  Medium  (network or file writes, typical)
  6-7  High    (process spawning, device access)
  8-10 Very High (exec, setuid, device + network combined)

Examples:
  sigma-capstore search                   # list all installed apps with caps
  sigma-capstore search cap:network       # network-using apps only
  sigma-capstore search risk<3            # safest apps only
  sigma-capstore show sigma-edit
  sigma-capstore compare sigma-edit vim
  sigma-capstore manifest /usr/bin/curl
"""
    return

  case args[0].toLowerAscii
  of "search","list":
    let query = if args.len > 1: args[1..^1].join(" ") else: ""
    echo fmt"\e[38;2;69;243;255m\e[1mΣ App Store — Capability View\e[0m"
    if query.len > 0: echo fmt"  Filter: {query}\n"
    else: echo "  Showing all installed apps by capability\n"
    let results = search_apps(query)
    if results.len == 0: echo "  No apps found."
    else:
      for caps in results: display_app(caps)
      echo fmt"  {results.len} apps found."

  of "show","info":
    if args.len < 2: echo "Usage: sigma-capstore show <app>"; return
    let caps = load_app_caps(args[1])
    display_app(caps, verbose=true)
    echo fmt"  Pledge manifest:\n    sigma_pledge({caps.pledge_caps.mapIt(\"\\\"\" & it & \"\\\"\").join(\", \")})"
    echo fmt"  Unveil manifest:\n    {caps.unveil_paths.join(\", \")}"

  of "compare":
    if args.len < 3: echo "Usage: sigma-capstore compare <app1> <app2>"; return
    compare_apps(args[1..^1])

  of "manifest","generate":
    if args.len < 2: echo "Usage: sigma-capstore manifest <binary>"; return
    let binary = args[1]
    let app_name = binary.extractFilename
    echo generate_manifest(app_name, binary)

  of "install":
    if args.len < 2: echo "Usage: sigma-capstore install <app>"; return
    let caps = load_app_caps(args[1])
    echo fmt"\e[38;2;69;243;255m\e[1mCapability Review — {args[1]}\e[0m\n"
    display_app(caps, verbose=true)
    stdout.write("Install this app? (yes/no) ")
    stdout.flushFile()
    let answer = stdin.readLine().strip().toLowerAscii
    if answer in ["yes","y"]:
      let (out, code) = execCmdEx(fmt"sigma-pkg install {args[1].quoteShell} 2>&1")
      echo out
      if code == 0: echo fmt"✓ Installed {args[1]}"
      else: echo fmt"✗ Install failed"
    else: echo "Cancelled."

  else:
    echo fmt"Unknown command: {args[0]}"
