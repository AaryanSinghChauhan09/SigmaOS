# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_tui.nim — Terminal UI (interactive picker + panels)
# Provides: command picker, diff viewer, multi-select tool list, status dashboard
#
# Inspiration:
#   Claude Code interactive diffs (y/n per hunk)
#   Aider /editor mode
#   fzf-style fuzzy finder
#   azure-cli interactive mode (az interactive)
#
# Components:
#   CommandPicker  — fuzzy-search commands with preview (like fzf)
#   DiffViewer     — side-by-side diff with y/n/q hunk acceptance
#   StatusDashboard — live system metrics panel
#   MultiSelect     — checkbox list for batch operations
#
# Fallback: uses external fzf/peco if available, otherwise built-in renderer
# Language: Nim (stdlib only — raw terminal control)

import std/[os, osproc, terminal, strutils, strformat, sequtils, times, json]

# ── Terminal helpers ───────────────────────────────────────────────────────────
const
  ESC   = "\e"
  CLEAR = "\e[2J\e[H"
  CYAN  = "\e[38;2;69;243;255m"
  GREEN = "\e[38;2;52;211;153m"
  RED   = "\e[38;2;248;113;113m"
  YELLOW= "\e[38;2;251;191;36m"
  MUTED = "\e[38;2;107;114;128m"
  BOLD  = "\e[1m"
  RESET = "\e[0m"
  DIM   = "\e[2m"
  # Cursor
  HIDE_CURSOR = "\e[?25l"
  SHOW_CURSOR = "\e[?25h"
  UP    = "\e[A"
  DOWN  = "\e[B"

proc term_width(): int =
  let (w, _) = terminalSize()
  max(40, w)

proc term_height(): int =
  let (_, h) = terminalSize()
  max(10, h)

proc move_to(row, col: int) = stdout.write(fmt"\e[{row};{col}H")
proc clear_line() = stdout.write("\e[2K")
proc hide_cursor() = stdout.write(HIDE_CURSOR)
proc show_cursor() = stdout.write(SHOW_CURSOR); stdout.flushFile()

# ── Fuzzy matcher ─────────────────────────────────────────────────────────────
proc fuzzy_score(query, candidate: string): int =
  if query.len == 0: return 1
  let q = query.toLowerAscii
  let c = candidate.toLowerAscii
  if q in c: return 100 - (c.len - q.len)  # exact substring bonus
  var qi = 0; var score = 0
  for ch in c:
    if qi < q.len and ch == q[qi]: qi += 1; score += 10
  if qi == q.len: return score
  0

# ── Command Picker (fzf-style) ─────────────────────────────────────────────────
type PickerItem = object
  label:   string
  preview: string
  value:   string

proc command_picker*(items: seq[PickerItem], prompt = "σ Command",
                     multi_select = false): seq[string] =
  ## Interactive fuzzy command picker
  ## Returns selected item values

  # Try fzf first (best UX)
  let fzf = findExe("fzf")
  if fzf.len > 0:
    let preview_cmd = "sigma-agent complete --ghost {}"
    let input_data = items.mapIt(it.label).join("\n")
    let fzf_cmd = if multi_select:
      fmt"{fzf} --multi --prompt '{prompt}> ' --height 40% --reverse --border"
    else:
      fmt"{fzf} --prompt '{prompt}> ' --height 40% --reverse --border"
    let (out, code) = execCmdEx(fmt"echo {input_data.quoteShell} | {fzf_cmd}")
    if code == 0:
      return out.strip().splitLines().filterIt(it.len > 0)
    return @[]

  # Built-in minimal picker
  var query   = ""
  var cursor  = 0
  var selected: seq[int]

  hide_cursor()
  defer: show_cursor()

  while true:
    # Filter items
    var filtered: seq[(int, PickerItem)]
    for i, item in items:
      let score = fuzzy_score(query, item.label)
      if score > 0 or query.len == 0:
        filtered.add((i, item))
    filtered.sort(proc(a,b:(int,PickerItem)):int = fuzzy_score(query, b[1].label) - fuzzy_score(query, a[1].label))

    cursor = min(cursor, max(0, filtered.len - 1))

    # Draw
    stdout.write(CLEAR)
    let w = term_width()
    let h = min(term_height() - 4, filtered.len)

    echo fmt"{CYAN}{BOLD}  {prompt}{RESET}"
    echo fmt"  {MUTED}Type to filter, ↑↓ to move, Enter to select, Ctrl+C to cancel{RESET}"
    echo fmt"  {CYAN}> {RESET}{query}{CYAN}█{RESET}"
    echo fmt"  {MUTED}{'─'.repeat(w - 4)}{RESET}"

    for i in 0..<h:
      let (orig_idx, item) = filtered[i]
      let is_sel = i == cursor
      let is_checked = orig_idx in selected
      let marker = if multi_select and is_checked: "●" elif multi_select: "○" else: " "
      let prefix = if is_sel: fmt"{CYAN}❯ {marker} " else: fmt"  {marker} "
      let label = if is_sel: BOLD & item.label & RESET else: item.label
      echo fmt"  {prefix}{label}{RESET}"

    stdout.flushFile()

    # Read key
    var key = ""
    try: key = stdin.readLine() except: break  # simplified key reading

    case key
    of "\r", "":   # Enter
      if filtered.len > 0:
        return @[filtered[cursor][1].value]
    of "\x1b[A":   # Up
      if cursor > 0: cursor -= 1
    of "\x1b[B":   # Down
      if cursor < filtered.len - 1: cursor += 1
    of "\x03":     # Ctrl+C
      break
    of " ":        # Space (multi-select)
      if multi_select and filtered.len > 0:
        let idx = filtered[cursor][0]
        if idx in selected: selected.del(selected.find(idx))
        else: selected.add(idx)
    else:
      query &= key

  @[]

# ── Diff viewer with hunk accept/reject ───────────────────────────────────────
type HunkDecision = enum HunkAccept, HunkReject, HunkSkip, HunkQuit

proc show_diff_hunk*(hunk_lines: seq[string], hunk_num, total_hunks: int): HunkDecision =
  ## Display a diff hunk and ask user to accept/reject
  ## Returns the user's decision
  let w = term_width()
  echo fmt"\n{MUTED}{'─'.repeat(w)}{RESET}"
  echo fmt"{CYAN}{BOLD}Hunk {hunk_num}/{total_hunks}{RESET}"
  for line in hunk_lines:
    let color = if line.startsWith("+"): GREEN
                elif line.startsWith("-"): RED
                elif line.startsWith("@@"): CYAN
                else: MUTED
    echo color & line & RESET
  echo fmt"{MUTED}{'─'.repeat(w)}{RESET}"
  stdout.write(fmt"{BOLD}Accept this hunk? {RESET}[{GREEN}y{RESET}/{RED}n{RESET}/{MUTED}s{RESET}=skip/{MUTED}q{RESET}=quit/all] ")
  stdout.flushFile()
  let answer = try: stdin.readLine().strip().toLowerAscii except: "q"
  case answer
  of "y","yes","a","all": HunkAccept
  of "n","no":            HunkReject
  of "s","skip":          HunkSkip
  else:                   HunkQuit

proc interactive_diff*(old_content, new_content, file_path: string): bool =
  ## Show diff interactively, let user accept/reject hunks
  ## Returns true if any hunks were accepted and written
  type DiffLine = tuple[kind: char, content: string]  # '+' '-' ' ' '@'

  var diff_lines: seq[DiffLine]
  let old_lines = old_content.splitLines()
  let new_lines = new_content.splitLines()

  # Build unified diff (simplified LCS)
  var i = 0; var j = 0
  while i < old_lines.len or j < new_lines.len:
    if i >= old_lines.len:
      diff_lines.add(('+', new_lines[j])); j += 1
    elif j >= new_lines.len:
      diff_lines.add(('-', old_lines[i])); i += 1
    elif old_lines[i] == new_lines[j]:
      diff_lines.add((' ', old_lines[i])); i += 1; j += 1
    else:
      diff_lines.add(('-', old_lines[i])); i += 1
      diff_lines.add(('+', new_lines[j])); j += 1

  # Group into hunks (consecutive changed lines + 3 lines context)
  var hunks: seq[seq[string]]
  var cur_hunk: seq[string]
  var in_change = false
  for dl in diff_lines:
    if dl.kind != ' ':
      in_change = true
      cur_hunk.add((if dl.kind == '+': "+" else: "-") & dl.content)
    else:
      if in_change:
        cur_hunk.add(" " & dl.content)
        if cur_hunk.len > 6:  # end of context
          hunks.add(cur_hunk); cur_hunk = @[]; in_change = false
      else:
        cur_hunk.add(" " & dl.content)
        if cur_hunk.len > 3: cur_hunk.delete(0)

  if cur_hunk.filterIt(it[0] != ' ').len > 0:
    hunks.add(cur_hunk)

  if hunks.len == 0:
    echo fmt"{GREEN}✓ No changes{RESET}"
    return false

  echo fmt"\n{CYAN}{BOLD}σ Reviewing changes to: {file_path}{RESET}"
  echo fmt"  {hunks.len} hunks to review\n"

  var accepted_hunks: seq[seq[string]]
  for i, hunk in hunks:
    let decision = show_diff_hunk(hunk, i+1, hunks.len)
    case decision
    of HunkAccept: accepted_hunks.add(hunk); echo fmt"  {GREEN}✓ Accepted{RESET}"
    of HunkReject: echo fmt"  {RED}✗ Rejected{RESET}"
    of HunkSkip:   echo fmt"  {MUTED}○ Skipped{RESET}"
    of HunkQuit:   echo fmt"  {MUTED}Quit review{RESET}"; break

  if accepted_hunks.len == 0: return false

  # Apply accepted hunks — for now write new_content if any accepted
  # (full hunk-level patching would require a proper patch algorithm)
  writeFile(file_path, new_content)
  echo fmt"\n{GREEN}✓ Changes applied to {file_path}{RESET}"
  true

# ── Status Dashboard ──────────────────────────────────────────────────────────
proc status_dashboard*(refresh_secs = 2, duration_secs = 0) =
  ## Live system metrics panel — Ctrl+C to exit
  hide_cursor()
  defer: show_cursor(); stdout.write(CLEAR)

  let start = now()
  var ticks = 0
  while true:
    if duration_secs > 0 and (now() - start).inSeconds >= duration_secs: break
    let w = term_width()
    stdout.write(CLEAR)

    echo fmt"{CYAN}{BOLD}Σ sigma-agent system dashboard{RESET}  {MUTED}{$now()}{RESET}"
    echo MUTED & "─".repeat(w) & RESET

    # CPU
    let (la, _) = execCmdEx("cat /proc/loadavg 2>/dev/null")
    if la.len > 0:
      let parts = la.split()
      echo fmt"  {CYAN}CPU load:{RESET}   {parts[0]} (1m)  {parts[1]} (5m)  {parts[2]} (15m)"

    # Memory
    var total_kb = 0'i64; var avail_kb = 0'i64
    try:
      for line in readFile("/proc/meminfo").splitLines():
        let p = line.split()
        if p.len >= 2:
          if p[0] == "MemTotal:":      total_kb = parseInt(p[1])
          elif p[0] == "MemAvailable:": avail_kb = parseInt(p[1])
    except: discard
    if total_kb > 0:
      let used_pct = (total_kb - avail_kb) * 100 div total_kb
      let bar_len = 30
      let filled = used_pct * bar_len div 100
      let bar = "█".repeat(filled) & "░".repeat(bar_len - filled)
      let mem_color = if used_pct > 80: RED elif used_pct > 60: YELLOW else: GREEN
      echo fmt"  {CYAN}Memory:{RESET}     {mem_color}{bar}{RESET} {used_pct}%  ({(total_kb-avail_kb) div 1024}MB / {total_kb div 1024}MB)"

    # Disk
    let (df_out, _) = execCmdEx("df -h / 2>/dev/null | tail -1")
    if df_out.strip().len > 0:
      echo fmt"  {CYAN}Disk (/):{RESET}   {df_out.strip()}"

    # Network
    let (ip_out, _) = execCmdEx("ip -brief addr 2>/dev/null | grep UP | head -3")
    if ip_out.strip().len > 0:
      echo fmt"  {CYAN}Network:{RESET}    {ip_out.strip().replace(chr(10), \", \")}"

    # Top processes
    let (ps_out, _) = execCmdEx("ps aux --sort=-%cpu 2>/dev/null | tail -n +2 | head -5 | awk '{printf \"%-20s %5s%% CPU  %5s%% MEM\\n\", $11, $3, $4}'")
    if ps_out.strip().len > 0:
      echo fmt"\n  {CYAN}Top processes:{RESET}"
      for line in ps_out.strip().splitLines():
        echo fmt"    {line}"

    # sigma-agent daemon status
    let (daemon_out, daemon_code) = execCmdEx("curl -sf http://localhost:11430/v1/status --max-time 1 2>/dev/null")
    if daemon_code == 0:
      try:
        let j = parseJson(daemon_out)
        echo fmt"\n  {CYAN}sigma-agent daemon:{RESET}  {GREEN}running{RESET}  backend={j[\"backend\"].getStr}  requests={j[\"requests\"].getInt}"
      except: discard
    else:
      echo fmt"\n  {CYAN}sigma-agent daemon:{RESET}  {MUTED}not running (sigma-agent daemon start){RESET}"

    echo ""
    echo MUTED & "─".repeat(w) & RESET
    echo fmt"  {MUTED}Refresh: {refresh_secs}s  |  Ctrl+C to exit  |  ticks={ticks}{RESET}"
    stdout.flushFile()
    ticks += 1
    sleep(refresh_secs * 1000)

# ── CLI ────────────────────────────────────────────────────────────────────────
proc tui_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "help":
    echo """sigma-agent tui — Terminal UI components

Usage:
  sigma-agent tui dashboard              Live system metrics panel
  sigma-agent tui dashboard --refresh 5  Custom refresh interval (seconds)
  sigma-agent tui pick                   Interactive command picker
  sigma-agent tui diff <file>            Review file changes interactively

Examples:
  sigma-agent tui dashboard
  sigma-agent tui dashboard --refresh 1
  sigma-agent tui pick
"""
    return

  case args[0].toLowerAscii
  of "dashboard","dash","status":
    var refresh = 2
    let ri = args.find("--refresh")
    if ri >= 0 and ri + 1 < args.len:
      refresh = try: parseInt(args[ri+1]) except: 2
    status_dashboard(refresh)

  of "pick","picker":
    # Build item list from commands
    let items = @[
      PickerItem(label: "install <package>",           value: "install "),
      PickerItem(label: "set dark mode",               value: "set dark mode"),
      PickerItem(label: "system info",                 value: "system info"),
      PickerItem(label: "security scan",               value: "security scan"),
      PickerItem(label: "network status",              value: "network status"),
      PickerItem(label: "show processes",              value: "show processes"),
      PickerItem(label: "disk usage",                  value: "disk usage"),
      PickerItem(label: "accessibility high-contrast on", value: "accessibility high-contrast on"),
      PickerItem(label: "explain <topic>",             value: "explain "),
      PickerItem(label: "fix <file> <instruction>",    value: "fix "),
      PickerItem(label: "daemon start",                value: "daemon start"),
      PickerItem(label: "learn rate good",             value: "learn rate good"),
      PickerItem(label: "multi diagnose <problem>",    value: "multi diagnose "),
      PickerItem(label: "voice",                       value: "voice"),
      PickerItem(label: "mirror list",                 value: "mirror list"),
    ]
    let selected = command_picker(items, "sigma-agent")
    if selected.len > 0:
      echo fmt"σ> {selected[0]}"
      let (out, _) = execCmdEx(fmt"sigma-agent-core --once {selected[0].quoteShell} 2>&1")
      echo out.strip()

  of "diff":
    let path = if args.len > 1: args[1] else: ""
    if path.len == 0:
      echo "Usage: sigma-agent tui diff <file>"
      return
    if not fileExists(path):
      echo fmt"✗ File not found: {path}"
      return
    let old = readFile(path)
    let new_content = old & "\n# Modified by sigma-agent tui diff demo"
    discard interactive_diff(old, new_content, path)

  else:
    echo fmt"Unknown tui command: {args[0]}"
    echo "Commands: dashboard | pick | diff"
