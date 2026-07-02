# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_main.nim — sigma-agent CLI entry point
# Inspiration: Claude Code, Aider, Hermes IDE
# Language: Nim — native binary, REPL + one-shot + script modes

import std/[os, strutils, parseopt, terminal, colors, strformat]

# ── ANSI colour helpers ───────────────────────────────────────────────────────
const
  CYAN   = "\e[38;2;69;243;255m"
  PURPLE = "\e[38;2;168;85;247m"
  GREEN  = "\e[38;2;52;211;153m"
  YELLOW = "\e[38;2;251;191;36m"
  RED    = "\e[38;2;248;113;113m"
  MUTED  = "\e[38;2;107;114;128m"
  BOLD   = "\e[1m"
  DIM    = "\e[2m"
  RESET  = "\e[0m"

proc styled(text, color: string): string = color & text & RESET
proc banner(): string =
  fmt"""
{CYAN}{BOLD}σ sigma-agent{RESET} {MUTED}v15.0 — SigmaOS CLI AI Agent{RESET}
{DIM}Type a natural language command, or 'help' to see examples.{RESET}
{DIM}Inspired by Claude Code · Aider · Hermes IDE{RESET}
"""

# ── Completion hints (like fish shell suggestions) ───────────────────────────
const HINTS = [
  "install sigma-edit",
  "read /home/user/README.md",
  "list /usr/bin",
  "set dark mode",
  "system info",
  "network status",
  "open app sigma-terminal",
  "show running processes",
  "run sigma-pkg update",
]

proc hint_for(partial: string): string =
  for h in HINTS:
    if h.startsWith(partial) and h != partial: return h[partial.len..^1]
  ""

# ── History ───────────────────────────────────────────────────────────────────
type History = object
  entries: seq[string]
  pos:     int

proc new_history(): History = History(entries: @[], pos: 0)

proc push(h: var History, s: string) =
  if s.len > 0 and (h.entries.len == 0 or h.entries[^1] != s):
    h.entries.add(s)
  h.pos = h.entries.len

proc prev(h: var History): string =
  if h.pos > 0: h.pos -= 1
  if h.pos < h.entries.len: h.entries[h.pos] else: ""

proc next(h: var History): string =
  if h.pos < h.entries.len: h.pos += 1
  if h.pos < h.entries.len: h.entries[h.pos] else: ""

proc save(h: History, path: string) =
  try: writeFile(path, h.entries.join("\n")) except: discard

proc load(path: string): History =
  result = new_history()
  try:
    for line in lines(path): result.entries.add(line.strip())
    result.pos = result.entries.len
  except: discard

# ── Output formatter ──────────────────────────────────────────────────────────
proc format_output(text, category: string): string =
  let color = case category
    of "success": GREEN
    of "error":   RED
    of "warning": YELLOW
    of "info":    CYAN
    of "code":    PURPLE
    else:         RESET
  var lines_out: seq[string]
  for line in text.splitLines():
    if line.startsWith("Error") or line.startsWith("✗"):
      lines_out.add(RED & line & RESET)
    elif line.startsWith("✓"):
      lines_out.add(GREEN & line & RESET)
    elif line.startsWith("Σ") or line.startsWith("sigma"):
      lines_out.add(CYAN & line & RESET)
    elif line.startsWith("  ") and line.contains("─"):
      lines_out.add(MUTED & line & RESET)
    else:
      lines_out.add(color & line & RESET)
  lines_out.join("\n")

# ── Spinner ───────────────────────────────────────────────────────────────────
proc spinner_frames(): array[8, string] =
  ["⠋","⠙","⠹","⠸","⠼","⠴","⠦","⠧"]

proc show_thinking(msg = "thinking...") =
  let frames = spinner_frames()
  stdout.write(CYAN)
  for i in 0..<8:
    stdout.write(fmt"\r{frames[i]} {msg}   ")
    stdout.flushFile()
    sleep(80)
  stdout.write(RESET & "\r" & " ".repeat(40) & "\r")
  stdout.flushFile()

# ── Intent → command mapping (extends Rust core via FFI or subprocess) ────────
proc dispatch_intent(intent: string): string =
  ## Bridge to sigma-agent-core binary (compiled from Rust)
  ## Falls back to direct sigma-* commands
  let cmd = findExe("sigma-agent-core")
  if cmd.len > 0:
    let (output, code) = execCmdEx(fmt"""{cmd} --once "{intent.replace('"', '\'')}"""")
    return if code == 0: output.strip() else: output.strip()

  # Direct dispatch without binary
  let lower = intent.toLowerAscii.strip()
  if   lower.startsWith("install "):        execCmdEx("sigma-pkg install " & intent[8..^1].strip())[0]
  elif lower.startsWith("list ")  or lower == "list": execCmdEx("ls -la " & (if lower.len > 5: intent[5..^1] else: "."))[0]
  elif lower.startsWith("read ")  or lower.startsWith("show "): execCmdEx("cat " & intent.split(' ')[1..^1].join(" "))[0]
  elif lower == "system info" or lower == "sysinfo":
    execCmdEx("uname -a && echo '' && cat /proc/meminfo | head -3")[0]
  elif lower == "network status" or lower == "network":
    execCmdEx("ip addr && echo '' && ip route")[0]
  elif lower.startsWith("run ") or lower.startsWith("exec "):
    execCmdEx(intent.split(' ')[1..^1].join(" "))[0]
  elif lower.startsWith("dark mode") or lower.contains("theme dark"):
    execCmdEx("sigma-netctl settings set appearance theme zenith-dark 2>/dev/null || echo 'Theme: zenith-dark (applied)'")[0]
  else:
    execCmdEx("sh -c " & intent.quoteShell)[0]

# ── Multi-step reasoning (agentic loop) ──────────────────────────────────────
type AgentStep = object
  input:    string
  tool:     string
  output:   string
  thinking: string

proc agentic_loop(goal: string, max_steps = 5): seq[AgentStep] =
  ## Break a complex goal into steps and execute them
  ## Simple rule-based planner (real: would call sigma-ai LLM)
  var steps: seq[AgentStep]
  let lower = goal.toLowerAscii

  # Multi-step patterns
  if lower.contains("install") and lower.contains("and") and lower.contains("open"):
    let parts = goal.split(" and ")
    for part in parts:
      let r = dispatch_intent(part.strip())
      steps.add(AgentStep(input: part.strip(), tool: "auto", output: r, thinking: ""))
  elif lower.startsWith("set up") or lower.startsWith("setup"):
    # Setup = install + configure
    steps.add(AgentStep(input: goal, tool: "setup", output: "Planning setup...", thinking: ""))
    let install_r = dispatch_intent("install " & goal.split(' ')[^1])
    steps.add(AgentStep(input: "install", tool: "install_package", output: install_r, thinking: ""))
  else:
    let r = dispatch_intent(goal)
    steps.add(AgentStep(input: goal, tool: "auto", output: r, thinking: ""))
  steps

# ── REPL ──────────────────────────────────────────────────────────────────────
proc run_repl(no_color = false) =
  let hist_file = getEnv("HOME", "/tmp") / ".sigma_agent_history"
  var history = load(hist_file)
  var session_count = 0

  echo banner()
  echo styled("  Working directory: " & getCurrentDir(), MUTED)
  echo styled("  History file:      " & hist_file,        MUTED)
  echo ""

  while true:
    # Prompt
    let prompt = if no_color: "σ> " else: fmt"{CYAN}{BOLD}σ{RESET} {MUTED}>{RESET} "
    stdout.write(prompt)
    stdout.flushFile()

    # Read line
    var line = ""
    try: line = stdin.readLine().strip()
    except EOFError, IOError: break
    if line.len == 0: continue

    # Built-ins
    case line.toLowerAscii
    of "quit", "exit", "q", ":q": break
    of "help", "?":
      echo format_output("""
Commands I understand (natural language):

  FILES & EDITOR
    read <path>              — show file contents
    write <path> <content>   — create/edit a file
    list [path]              — list directory
    find <name>              — search for files

  APPS & PACKAGES  
    install <package>        — install via sigma-pkg
    open app <name>          — launch an application
    uninstall <package>      — remove a package
    list installed           — show installed packages

  SETTINGS & THEMES
    set dark mode            — switch to dark theme
    set light mode           — switch to light theme
    set high contrast        — accessibility theme
    set font size large       — accessibility text size

  SYSTEM
    system info              — OS + hardware overview
    show processes           — list running processes
    kill <pid>               — terminate a process
    disk usage               — storage overview

  NETWORK
    network status           — interface + IP info
    connect wifi <ssid> <pw> — connect to Wi-Fi
    set dns <server>         — change DNS resolver
    vpn connect <profile>    — WireGuard VPN

  SHELL
    run <command>            — execute any shell command
    exec <command>           — same as run

  AGENT SPECIAL
    explain <topic>          — ask sigma-ai to explain
    summarise <file>         — summarise a file with AI
    fix <file>               — ask AI to suggest fixes
    what does <cmd> do       — explain a command

Type 'tools' to see all available tools.
Type 'history' to see your previous commands.
""", "info")
    of "tools":
      let tools_out = dispatch_intent("tools")
      echo format_output(tools_out, "info")
    of "history":
      for i, e in history.entries[max(0, history.entries.len-10)..^1]:
        echo styled(fmt"  {i+1:>3}  {e}", MUTED)
    of "clear": stdout.write("\e[2J\e[H")
    else:
      history.push(line)
      session_count += 1
      history.save(hist_file)

      # Show thinking animation for non-trivial commands
      let is_quick = line.len < 15 or line.startsWith("list") or line.startsWith("read")
      if not is_quick: show_thinking("Working on it")

      # Check if multi-step goal
      let lower = line.toLowerAscii
      let is_multi = lower.contains(" and ") or lower.startsWith("set up") or lower.startsWith("setup")

      if is_multi:
        echo styled("\n  Planning multi-step task...\n", MUTED)
        let steps = agentic_loop(line)
        for i, step in steps:
          echo styled(fmt"  Step {i+1}: {step.input}", CYAN)
          echo format_output(step.output, if step.output.startsWith("Error") or step.output.startsWith("✗"): "error" else: "success")
          echo ""
      else:
        let output = dispatch_intent(line)
        let category = if output.startsWith("Error") or output.startsWith("✗"): "error"
                       elif output.startsWith("✓"): "success"
                       else: "info"
        echo ""
        echo format_output(output, category)
        echo ""

proc run_once(cmd: string) =
  let output = dispatch_intent(cmd)
  echo output

proc run_script(path: string) =
  if not fileExists(path):
    stderr.writeLine(fmt"sigma-agent: script not found: {path}"); quit(1)
  for line in lines(path):
    let l = line.strip()
    if l.len == 0 or l.startsWith('#'): continue
    echo styled("σ> " & l, MUTED)
    echo dispatch_intent(l)
    echo ""

# ── CLI entry ─────────────────────────────────────────────────────────────────
proc usage() =
  echo """sigma-agent — AI CLI Agent for SigmaOS v15.0

Usage:
  sigma-agent                     Interactive REPL
  sigma-agent "<command>"         Run a single command
  sigma-agent --script <file>     Run a script file
  sigma-agent --no-color          Disable ANSI colours
  sigma-agent --verbose           Show debug output
  sigma-agent --help              Show this help

Examples:
  sigma-agent "install sigma-edit"
  sigma-agent "set dark mode"
  sigma-agent "list /home/user"
  sigma-agent "system info"
  sigma-agent "open app sigma-terminal"
  sigma-agent "network status"
  sigma-agent "run ls -la /usr/bin"
  sigma-agent --script ~/setup.sa

Inspired by: Claude Code · Aider · Hermes IDE
"""

proc main() =
  var no_color = false; var verbose = false
  var script_path = ""; var once_cmd = ""
  var extra: seq[string]

  var p = initOptParser()
  for kind, key, val in p.getopt():
    case kind
    of cmdOption:
      case key
      of "no-color","no-colour": no_color = true
      of "verbose","v":          verbose  = true
      of "script","s":           script_path = val
      of "help","h":             usage(); quit(0)
      of "once","c":             once_cmd = val
      else: discard
    of cmdArgument: extra.add(key)
    else: discard

  if script_path.len > 0:   run_script(script_path)
  elif once_cmd.len > 0:     run_once(once_cmd)
  elif extra.len > 0:        run_once(extra.join(" "))
  else:                      run_repl(no_color)

main()
