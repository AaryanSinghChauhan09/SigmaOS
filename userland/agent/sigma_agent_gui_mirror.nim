# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_gui_mirror.nim — GUI → CLI mapping (complete mirror)
# Inspiration: azure-cli (comprehensive command surface), openclaw (GUI parity)
# Language: Nim — every GUI panel has a CLI equivalent

import std/[os, strutils, osproc, strformat, tables]

# ── Complete GUI → CLI mapping table ─────────────────────────────────────────
const GUI_TO_CLI = {
  # ── Zenith Desktop ────────────────────────────────────────────────────────
  "open terminal":                   "sigma-agent \"open app sigma-terminal\"",
  "open files":                      "sigma-agent \"open app sigma-files\"",
  "open editor":                     "sigma-agent \"open app sigma-edit\"",
  "open browser":                    "sigma-agent \"open app sigma-browser\"",
  "open settings":                   "sigma-agent \"open app sigma-settings\"",
  "open app store":                  "sigma-agent \"open app sigma-appstore\"",
  "open system monitor":             "sigma-agent \"open app sigma-monitor\"",
  "open calculator":                 "sigma-agent \"open app sigma-calc\"",
  "switch workspace 1":              "sigma-agent \"workspace 1\"",
  "switch workspace 2":              "sigma-agent \"workspace 2\"",
  "tile windows":                    "sigma-agent \"tile\"",
  "float window":                    "sigma-agent \"float\"",
  "fullscreen":                      "sigma-agent \"fullscreen\"",
  "close window":                    "sigma-agent \"close window\"",
  "cycle layout":                    "sigma-agent \"cycle layout\"",

  # ── Settings → Appearance ─────────────────────────────────────────────────
  "set dark theme":                  "sigma-agent \"set dark mode\"",
  "set light theme":                 "sigma-agent \"set light mode\"",
  "set high contrast":               "sigma-agent \"set high contrast\"",
  "increase font size":              "sigma-agent \"accessibility large-text on\"",
  "decrease font size":              "sigma-agent \"accessibility large-text off\"",
  "change corner radius":            "sigma-agent \"settings set appearance corner_radius 8\"",
  "disable animations":              "sigma-agent \"settings set appearance animations false\"",
  "enable animations":               "sigma-agent \"settings set appearance animations true\"",

  # ── Settings → Network ────────────────────────────────────────────────────
  "view network interfaces":         "sigma-netctl list",
  "connect to wifi":                 "sigma-netctl wifi <iface> <ssid> <pass>",
  "disconnect wifi":                 "sigma-netctl down <iface>",
  "set static ip":                   "sigma-netctl static <iface> <ip> <mask> <gw>",
  "enable dhcp":                     "sigma-netctl dhcp <iface>",
  "change dns":                      "sigma-netctl dns <server1> <server2>",
  "enable firewall":                 "sigma-agent \"settings set network firewall true\"",
  "disable firewall":                "sigma-agent \"settings set network firewall false\"",
  "connect vpn":                     "sigma-vpn connect <profile>",
  "disconnect vpn":                  "sigma-vpn disconnect <profile>",
  "list vpn profiles":               "sigma-vpn list",

  # ── Settings → Privacy ────────────────────────────────────────────────────
  "disable telemetry":               "sigma-agent \"settings set privacy telemetry false\"",
  "enable crash reports":            "sigma-agent \"settings set privacy crash_reports true\"",
  "disable clipboard access":        "sigma-agent \"settings set privacy clipboard_guard true\"",
  "clear temp files":                "sigma-agent \"run rm -rf /tmp/*\"",

  # ── Settings → Accessibility ─────────────────────────────────────────────
  "enable high contrast":            "sigma-agent \"accessibility high-contrast on\"",
  "disable high contrast":           "sigma-agent \"accessibility high-contrast off\"",
  "enable screen reader":            "sigma-agent \"accessibility screen-reader on\"",
  "disable screen reader":           "sigma-agent \"accessibility screen-reader off\"",
  "enable reduce motion":            "sigma-agent \"accessibility reduce-motion on\"",
  "enable colour blind mode":        "sigma-agent \"accessibility colour-blind on\"",
  "enable sticky keys":              "sigma-agent \"accessibility sticky-keys on\"",
  "enable large text":               "sigma-agent \"accessibility large-text on\"",

  # ── App Store ─────────────────────────────────────────────────────────────
  "search packages":                 "sigma-pkg search <query>",
  "install package":                 "sigma-pkg install <name>",
  "remove package":                  "sigma-pkg remove <name>",
  "update all packages":             "sigma-pkg update",
  "list installed":                  "sigma-pkg list",
  "package info":                    "sigma-pkg info <name>",
  "verify package":                  "sigma-pkg verify <file.sigpkg>",

  # ── System Monitor ────────────────────────────────────────────────────────
  "view cpu usage":                  "sigma-monitor --once | grep CPU",
  "view memory usage":               "sigma-monitor --once | grep MEM",
  "view processes":                  "sigma-top -1",
  "kill process":                    "sigma-agent \"kill process <pid>\"",
  "view disk usage":                 "sigma-disks list",
  "view network speed":              "sigma-netctl show <iface>",

  # ── File Manager ─────────────────────────────────────────────────────────
  "browse home":                     "sigma-agent \"list ~\"",
  "browse root":                     "sigma-agent \"list /\"",
  "create folder":                   "sigma-agent \"run mkdir <name>\"",
  "delete file":                     "sigma-agent \"run rm <file>\"",
  "move file":                       "sigma-agent \"run mv <src> <dst>\"",
  "copy file":                       "sigma-agent \"run cp <src> <dst>\"",
  "view file":                       "sigma-agent \"read <file>\"",
  "search files":                    "sigma-agent \"find <query>\"",
  "show disk usage":                 "sigma-agent \"disk usage\"",

  # ── Notifications ─────────────────────────────────────────────────────────
  "send notification":               "sigma-agent \"notify '<title>' '<body>'\"",
  "enable do not disturb":           "sigma-agent \"settings set notifications dnd true\"",
  "disable do not disturb":          "sigma-agent \"settings set notifications dnd false\"",
  "clear notifications":             "sigma-notify --clear",

  # ── Desktop Controls ─────────────────────────────────────────────────────
  "lock screen":                     "sigma-lock",
  "suspend system":                  "sigma-agent \"run systemctl suspend 2>/dev/null || echo suspend\"",
  "shutdown":                        "sigma-agent \"run shutdown -h now\"",
  "restart":                         "sigma-agent \"run reboot\"",
  "screenshot":                      "sigma-screenshot",

  # ── Clipboard ─────────────────────────────────────────────────────────────
  "copy to clipboard":               "sigma-agent \"copy <text>\"",
  "paste from clipboard":            "sigma-agent \"paste\"",
  "clear clipboard":                 "sigma-agent \"clipboard clear\"",

  # ── Developer Tools ───────────────────────────────────────────────────────
  "view system logs":                "sigma-logs",
  "run diagnostics":                 "sigma-doctor",
  "performance benchmark":           "sigma-bench all",
  "strace process":                  "sigma-strace -p <pid>",
  "view system info":                "sigma-agent \"system info\"",
  "update system":                   "sigma-update apply",
}.toTable

type GuiMirror = object
  mappings: Table[string, string]

proc new_gui_mirror(): GuiMirror =
  GuiMirror(mappings: GUI_TO_CLI)

proc lookup(m: GuiMirror, action: string): string =
  let lower = action.toLowerAscii.strip()
  if lower in m.mappings: return m.mappings[lower]
  # Fuzzy: find best match
  var best_score = 0; var best_cmd = ""
  let words = lower.splitWhitespace.toHashSet
  for k, v in m.mappings:
    let kwords = k.toLowerAscii.splitWhitespace.toHashSet
    let score  = (kwords * words).len
    if score > best_score: best_score = score; best_cmd = v
  if best_score > 0: return best_cmd
  fmt"sigma-agent \"{action}\""

proc list_all(m: GuiMirror, filter = "") =
  var categories: Table[string, seq[(string,string)]]
  for action, cmd in m.mappings:
    if filter.len > 0 and not action.contains(filter.toLowerAscii): continue
    # Group by prefix word
    let cat = action.split(' ')[0]
    if cat notin categories: categories[cat] = @[]
    categories[cat].add((action, cmd))

  let cat_order = ["open","switch","set","view","list","install","connect","enable","disable","send","run"]
  for cat in cat_order:
    if cat notin categories: continue
    echo fmt"\n  {cat.toUpperAscii}:"
    for (action, cmd) in categories[cat]:
      echo fmt"    {action:<35} → {cmd}"
  for cat, pairs in categories:
    if cat in cat_order: continue
    echo fmt"\n  {cat.toUpperAscii}:"
    for (action, cmd) in pairs:
      echo fmt"    {action:<35} → {cmd}"

proc mirror_cmd*(args: seq[string]) =
  let m = new_gui_mirror()
  if args.len == 0:
    echo "sigma-agent mirror — Complete GUI → CLI mapping\n"
    echo "Usage:"
    echo "  sigma-agent mirror list              List all GUI actions"
    echo "  sigma-agent mirror list <filter>     Filter by keyword"
    echo "  sigma-agent mirror run <action>      Execute a GUI action via CLI"
    echo "  sigma-agent mirror count             Show total mapped actions"
    return

  case args[0]
  of "list":
    let filter = if args.len > 1: args[1] else: ""
    echo fmt"GUI → CLI Mapping ({m.mappings.len} actions):"
    m.list_all(filter)
  of "run":
    if args.len < 2: echo "Usage: mirror run <action>"; return
    let action = args[1..^1].join(" ")
    let cmd    = m.lookup(action)
    echo fmt"→ {cmd}"
    let (out, code) = execCmdEx(cmd)
    echo out
  of "count":
    echo fmt"Total GUI actions mapped: {m.mappings.len}"
  of "search":
    if args.len < 2: echo "Usage: mirror search <keyword>"; return
    let kw = args[1..^1].join(" ").toLowerAscii
    for action, cmd in m.mappings:
      if kw in action or kw in cmd:
        echo fmt"  {action:<35} → {cmd}"
  else:
    # Treat as direct action lookup
    let cmd = m.lookup(args.join(" "))
    echo cmd
