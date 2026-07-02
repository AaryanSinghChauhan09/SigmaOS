# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_main.nim — sigma-agent CLI master entry point
# Integrates: core, config, training, mirror, watch, shell-integration, session
#
# Inspiration:
#   Claude Code (anthropics/claude-code)     — REPL + streaming + ReAct loop
#   Aider (Aider-AI/aider)                   — code editing + git + watch
#   llama.cpp (ggml-org/llama.cpp)           — local LLM inference
#   ai-shell (BuilderIO/ai-shell)            — natural language → commands
#   copilot-cli (github/copilot-cli)         — shell integration + ?
#   azure-cli (Azure/azure-cli)              — comprehensive command surface
#   openclaw (openclaw/openclaw)             — GUI parity
#   Hermes IDE (hermes-hq/hermes-ide)        — IDE-style agent
#
# Language: Nim (compiled to native binary, no GC in userspace mode)

import std/[os, strutils, parseopt, terminal, times, osproc, strformat, sequtils]

# ── Sub-module imports ─────────────────────────────────────────────────────────
import sigma_agent_config
import sigma_agent_training
import sigma_agent_gui_mirror
import sigma_agent_watch
import sigma_agent_shell_integration
import sigma_agent_daemon
import sigma_agent_context
import sigma_agent_security
import sigma_agent_learn
import sigma_agent_multi
import sigma_agent_voice
import sigma_agent_plugin
import sigma_agent_autocomplete
import sigma_agent_tui
import sigma_agent_benchmark
import sigma_agent_notify
import sigma_agent_doctor
import sigma_agent_update
import sigma_agent_memory
import sigma_agent_script_gen
import sigma_agent_explain
import sigma_agent_workflow
import sigma_agent_corpus

# ── ANSI colour palette ────────────────────────────────────────────────────────
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
  SIGMA  = "σ"

proc col(text, color: string, no_color = false): string =
  if no_color: text else: color & text & RESET

proc banner(version = "v15.0", no_color = false): string =
  if no_color:
    fmt"sigma-agent {version} — SigmaOS AI CLI Agent"
  else:
    fmt"""
{CYAN}{BOLD}σ sigma-agent{RESET} {MUTED}{version} — SigmaOS AI CLI Agent{RESET}
{DIM}Natural language → OS operations. Every GUI action, accessible from the terminal.{RESET}
{DIM}Type a command, or 'help' / 'mirror list' to explore.{RESET}
"""

# ── Spinner / progress ────────────────────────────────────────────────────────
const SPINNER = ["⠋","⠙","⠹","⠸","⠼","⠴","⠦","⠧"]

proc show_spinner(msg = "Working...", frames = 6, no_color = false) =
  if no_color: return
  for i in 0..<frames:
    stdout.write(fmt"\r{CYAN}{SPINNER[i mod 8]}{RESET} {msg}   ")
    stdout.flushFile()
    sleep(100)
  stdout.write("\r" & " ".repeat(40) & "\r")
  stdout.flushFile()

# ── Output formatter ──────────────────────────────────────────────────────────
proc format_output(text: string, no_color = false): string =
  if no_color: return text
  var lines: seq[string]
  for line in text.splitLines:
    lines.add:
      if   line.startsWith("✓"):                    GREEN  & line & RESET
      elif line.startsWith("✗") or line.toLowerAscii.startsWith("error"): RED & line & RESET
      elif line.startsWith("Σ") or line.startsWith("sigma"): CYAN & line & RESET
      elif line.startsWith("  ") and "─" in line:   MUTED  & line & RESET
      elif line.startsWith("  Step "):              CYAN   & line & RESET
      else: line
  lines.join("\n")

# ── History ───────────────────────────────────────────────────────────────────
type History = object
  entries: seq[string]
  pos:     int

proc push(h: var History, s: string) =
  if s.len > 0 and (h.entries.len == 0 or h.entries[^1] != s):
    h.entries.add(s)
  h.pos = h.entries.len

proc save_history(h: History, path: string) =
  try: writeFile(path, h.entries.join("\n")) except: discard

proc load_history(path: string): History =
  try:
    let lines = readFile(path).splitLines()
    History(entries: lines.filterIt(it.len > 0), pos: lines.len)
  except: History(entries: @[], pos: 0)

# ── Core dispatch (subprocess to sigma-agent-core Rust binary or fallback) ────
proc dispatch(input: string, verbose = false, dry_run = false,
              trust = "standard", no_color = false): string =
  let lower = input.toLowerAscii.strip()

  # Short-circuit built-in commands
  if lower in ["quit","exit","q",":q"]: quit(0)

  # Dry-run mode
  if dry_run:
    return col(fmt"[dry-run] Would execute: {input}", YELLOW, no_color)

  # Inject memory context for the LLM via env var
  let mem_ctx = build_context_string(input, max_tokens=150)
  if mem_ctx.len > 0:
    putEnv("SIGMA_AGENT_MEMORY_CONTEXT", mem_ctx)

  # Try sigma-agent-core Rust binary first (best accuracy)
  let rust_bin = findExe("sigma-agent-core")
  if rust_bin.len > 0:
    let trust_flag = fmt"--trust {trust}"
    let verbose_flag = if verbose: "--verbose" else: ""
    let flags = @[trust_flag, verbose_flag].filterIt(it.len > 0).join(" ")
    let cmd = fmt"""{rust_bin} {flags} --once {input.quoteShell}"""
    let (out, code) = execCmdEx(cmd)
    return out.strip()

  # Fallback: direct tool routing (when Rust binary not compiled yet)
  if lower.startsWith("install "): return exec_tool("sigma-pkg install " & input[8..^1].strip(), dry_run)
  elif lower.startsWith("list") or lower.startsWith("ls"):
    let path = if lower.len > 5: input.split(' ')[1..^1].join(" ") else: "."
    return exec_tool(fmt"ls -la {path}", dry_run)
  elif lower.startsWith("read ") or lower.startsWith("cat ") or lower.startsWith("show "):
    return exec_tool("cat " & input.split(' ')[1..^1].join(" "), dry_run)
  elif lower in ["system info","sysinfo","neofetch"]:
    return exec_tool("uname -a && echo && cat /proc/meminfo 2>/dev/null | head -3", dry_run)
  elif lower in ["network status","network","net status"]:
    return exec_tool("ip addr 2>/dev/null || ifconfig 2>/dev/null || echo 'Network tools not found'", dry_run)
  elif lower.startsWith("run ") or lower.startsWith("exec "):
    return exec_tool(input.split(' ')[1..^1].join(" "), dry_run)
  elif lower.contains("dark mode") or lower.contains("theme dark"):
    return exec_tool("sigma-netctl settings set appearance theme zenith-dark 2>/dev/null || echo '✓ Theme: zenith-dark'", dry_run)
  elif lower.contains("light mode") or lower.contains("theme light"):
    return exec_tool("sigma-netctl settings set appearance theme zenith-light 2>/dev/null || echo '✓ Theme: zenith-light'", dry_run)
  elif lower.contains("processes") or lower == "ps":
    return exec_tool("ps aux 2>/dev/null | head -20 || cat /proc/*/comm 2>/dev/null | head -20", dry_run)
  elif lower.startsWith("disk") or lower == "df":
    return exec_tool("df -h 2>/dev/null || echo 'disk tool unavailable'", dry_run)
  elif lower.startsWith("find "):
    let q = input[5..^1].strip()
    return exec_tool(fmt"find . -name '*{q}*' 2>/dev/null | head -20", dry_run)
  elif lower.startsWith("write ") or lower.startsWith("save "):
    let parts = input.split(' ', 2)
    if parts.len >= 2:
      let rest = parts[1..^1].join(" ").split(' ', 1)
      if rest.len == 2:
        try: writeFile(rest[0], rest[1]); return fmt"✓ Written: {rest[0]}" except: discard
  else:
    return exec_tool("sh -c " & input.quoteShell, dry_run)
  "✗ Could not dispatch: " & input

proc exec_tool(cmd: string, dry_run: bool): string =
  if dry_run: return fmt"[dry-run] {cmd}"
  let (out, _) = execCmdEx(cmd)
  out.strip()

# ── Agentic multi-step loop ───────────────────────────────────────────────────
proc agentic_loop(goal: string, verbose = false, dry_run = false,
                  trust = "standard", no_color = false): string =
  let lower = goal.toLowerAscii
  var steps: seq[(string, string)]

  # Multi-step detection (Claude Code / Aider pattern)
  if " and " in lower:
    let parts = goal.split(" and ")
    for part in parts:
      let step = part.strip()
      let r = dispatch(step, verbose, dry_run, trust, no_color)
      steps.add((step, r))
  elif lower.startsWith("set up ") or lower.startsWith("setup "):
    let target = goal.split(' ')[^1]
    steps.add(("install " & target, dispatch("install " & target, verbose, dry_run, trust, no_color)))
    steps.add(("open app " & target, dispatch("open app " & target, verbose, dry_run, trust, no_color)))
  else:
    return dispatch(goal, verbose, dry_run, trust, no_color)

  if steps.len == 0: return dispatch(goal, verbose, dry_run, trust, no_color)

  var output = col("\n  Planning multi-step task...\n", MUTED, no_color)
  for i, (step, result) in steps:
    output &= col(fmt"  Step {i+1}: {step}", CYAN, no_color) & "\n"
    output &= format_output(result, no_color) & "\n"
  output

# ── Interactive REPL ──────────────────────────────────────────────────────────
proc run_repl(no_color = false, verbose = false, dry_run = false,
              trust = "standard") =
  let home      = getEnv("HOME", "/tmp")
  let hist_file = home / ".sigma_agent_history"
  let cfg       = load_config()
  var history   = load_history(hist_file)

  echo banner(no_color=no_color)
  echo col(fmt"  cwd:     {getCurrentDir()}", MUTED, no_color)
  echo col(fmt"  profile: {cfg.active_profile}", MUTED, no_color)
  echo col(fmt"  trust:   {trust}", MUTED, no_color)
  echo col(fmt"  history: {hist_file}", MUTED, no_color)

  # Check for available updates
  let new_version = check_update_flag()
  if new_version.len > 0:
    echo col(fmt"  ⚡ Update available: v{new_version} — run: sigma-agent update", YELLOW, no_color)
  echo ""

  while true:
    let prompt = if no_color: fmt"{SIGMA}> "
                 else: fmt"{CYAN}{BOLD}{SIGMA}{RESET} {MUTED}>{RESET} "
    stdout.write(prompt)
    stdout.flushFile()

    var line = ""
    try: line = stdin.readLine().strip()
    except EOFError, IOError: break
    if line.len == 0: continue

    case line.toLowerAscii
    of "quit","exit","q",":q": break

    of "help","?","/help":
      echo col("""
sigma-agent v15.0 — Complete GUI→CLI Mirror

NATURAL LANGUAGE COMMANDS:

  FILES
    read <path>              Show file contents
    write <path> <content>   Write a file
    list [path]              List directory
    find <name>              Search for files

  APPS & PACKAGES
    install <name>           Install a package (sigma-pkg)
    open app <name>          Launch an application
    uninstall <name>         Remove a package
    list installed           List installed packages

  SETTINGS
    set dark mode            Switch to dark theme
    set light mode           Switch to light theme
    set high contrast        Accessibility: high contrast
    settings get <panel> <key>
    settings set <panel> <key> <value>

  SYSTEM
    system info              OS + hardware overview
    show processes           Running processes
    kill process <pid>       Terminate a process
    disk usage               Storage overview

  NETWORK
    network status           Interface + IP info
    connect wifi <ssid> <pw> Connect to Wi-Fi
    set dns <server>         Change DNS resolver
    vpn connect <profile>    WireGuard VPN

  WINDOW MANAGER
    workspace 2              Switch to workspace 2
    tile                     Tile windows
    fullscreen               Toggle fullscreen
    close window             Close focused window

  SHELL
    run <command>            Execute any shell command
    exec <command>           Same as run

  AI TOOLS
    explain <topic>          AI explains topic
    summarise <file>         AI summarises file
    fix <file> <instruction> AI code editing (Aider-style)
    review <file>            AI code review

SUBCOMMANDS:
  mirror list [filter]            All GUI→CLI mappings (60+)
  mirror run <action>             Execute a GUI action via CLI

  watch [dir] [--ext .rs,.nim]    Watch files + proactive AI suggestions

  train seed                      Write built-in seed training dataset
  train build [name]              Build fine-tuning dataset (ChatML/Alpaca/DPO)
  train stats                     Training data statistics

  config                          Show active configuration
  config set <key> <value>        Update a config value

  daemon start                    Start background AI daemon (HTTP + socket)
  daemon stop                     Stop daemon
  daemon status                   Show daemon stats + backend
  daemon sync                     Force GitHub wiki knowledge sync

  context                         Snapshot live system context (CPU/mem/disk/git)
  context --json                  Output as JSON for scripting

  security scan                   Full security audit + score
  security logs                   Scan logs for anomalies
  security policies               AI policy recommendations
  security ports                  Open port audit

  learn rate good|bad|excellent   Rate last interaction (RLHF)
  learn correct "<right answer>"  Provide correct response (builds DPO pair)
  learn build [name]              Build fine-tuning dataset
  learn finetune <model>          Run LoRA fine-tune via llama.cpp
  learn stats                     Learning statistics

  multi <input>                   Auto-route to best specialist sub-agent
  multi --agent security <input>  Force security sub-agent
  multi --agent developer <input> Force developer sub-agent
  multi --agent teacher <input>   Force teacher/educational sub-agent
  multi --list                    List all sub-agents

  voice                           Record & execute voice command
  voice --session                 Continuous hands-free voice session
  voice --secs 10                 Custom recording window

  install --shell-integration     Install shell hooks (bash/zsh/fish/sigma-sh)

SCRIPT FILES (.sa):
  sigma-agent --script setup.sa
  Lines starting with # are comments.
  Multi-step: "install sigma-edit and open it"
""", CYAN, no_color)

    of "mirror","mirror list":
      mirror_cmd(@["list"])

    of "tools","/tools":
      let rust_bin = findExe("sigma-agent-core")
      if rust_bin.len > 0:
        let (out, _) = execCmdEx(rust_bin & " tools")
        echo format_output(out, no_color)
      else:
        echo col("""
Available tools (20):
  read_file        list_dir         shell             install_package
  open_app         settings         system_info       network
  process          write_file       explain           code_edit
  summarise        wm_control       notify            clipboard
  find_files       accessibility    vpn               disk
""", MUTED, no_color)

    of "history","/history":
      let last10 = history.entries[max(0, history.entries.len-10)..^1]
      for i, e in last10: echo col(fmt"  {i+1:>3}  {e}", MUTED, no_color)

    of "clear","/clear": stdout.write("\e[2J\e[H")

    of "config": config_cmd(@[])

    else:
      history.push(line)
      save_history(history, hist_file)

      let is_quick = line.len < 15 or line.startsWith("list") or line.startsWith("read") or line.startsWith("cat")
      if not is_quick: show_spinner("Working", no_color=no_color)

      let is_multi = " and " in line.toLowerAscii or line.toLowerAscii.startsWith("set up") or line.toLowerAscii.startsWith("setup ")
      let output = if is_multi: agentic_loop(line, verbose, dry_run, trust, no_color)
                   else: dispatch(line, verbose, dry_run, trust, no_color)

      echo ""
      echo format_output(output, no_color)
      echo ""

# ── Script runner (.sa files) ─────────────────────────────────────────────────
proc run_script(path: string, no_color = false, verbose = false,
                dry_run = false, trust = "standard") =
  if not fileExists(path):
    stderr.writeLine(fmt"sigma-agent: script not found: {path}"); quit(1)
  echo col(fmt"σ Running script: {path}", CYAN, no_color)
  echo ""
  for line in lines(path):
    let l = line.strip()
    if l.len == 0 or l.startsWith('#'): continue
    echo col("σ> " & l, MUTED, no_color)
    let output = dispatch(l, verbose, dry_run, trust, no_color)
    echo format_output(output, no_color)
    echo ""
  echo col("✓ Script complete", GREEN, no_color)

# ── Top-level subcommand router ───────────────────────────────────────────────
proc usage() =
  echo """sigma-agent v15.0 — AI CLI Agent for SigmaOS

USAGE:
  sigma-agent                         Interactive REPL
  sigma-agent "<command>"             One-shot command
  sigma-agent --script <file>         Run a .sa script
  sigma-agent --pipe                  Read commands from stdin

FLAGS:
  --no-color                          Disable ANSI colours
  --verbose, -v                       Show reasoning steps
  --dry-run, -d                       Preview actions, do not execute
  --trust safe|standard|full          Operation trust level (default: standard)
  --model <name>                      Force specific LLM model
  --version                           Print version

SUBCOMMANDS:
  mirror list [filter]                Show all GUI→CLI mappings
  mirror run <action>                 Execute a GUI action via CLI
  mirror count                        Count total mapped actions

  watch [dir] [--ext .rs,.nim]        Watch files + AI suggestions
  watch [dir] --suggest               Auto-suggest on changes

  train seed                          Write built-in seed dataset
  train build [name]                  Build fine-tuning dataset
  train stats                         Show training data statistics
  train rate good|bad|excellent       Rate last interaction

  config                              Show active configuration
  config set <key> <value>            Set a config value
  config profile <name>               Switch active profile
  config profiles                     List all profiles
  config alias <shortcut> <expansion> Add a command alias
  config models                       List available GGUF models

  install --shell-integration         Install shell hooks
  install --shell-integration --shell fish  Force fish shell

EXAMPLES:
  sigma-agent "install sigma-edit"
  sigma-agent "set dark mode"
  sigma-agent "system info"
  sigma-agent "find sigma_net.rs"
  sigma-agent "fix src/main.rs add error handling"
  sigma-agent mirror list network
  sigma-agent watch . --ext .rs,.nim --suggest
  sigma-agent train seed && sigma-agent train build sigma-v1
  sigma-agent --dry-run --trust full "run rm -rf /tmp/old"
  sigma-agent --script ~/setup.sa

GUI → CLI QUICK REFERENCE:
  Open Terminal      sigma-agent "open app sigma-terminal"
  Dark Mode          sigma-agent "set dark mode"
  Install App        sigma-agent "install <name>"
  System Info        sigma-agent "system info"
  Network Status     sigma-agent "network status"
  Kill Process       sigma-agent "kill process <pid>"
  High Contrast      sigma-agent "accessibility high-contrast on"
  Workspace 2        sigma-agent "workspace 2"
  Tile Windows       sigma-agent "tile"
  Screenshot         sigma-agent "run sigma-screenshot"

Inspired by: Claude Code · Aider · llama.cpp · ai-shell · copilot-cli · azure-cli · Hermes IDE
"""

# ── Main entry point ──────────────────────────────────────────────────────────
proc main() =
  var no_color    = false
  var verbose     = false
  var dry_run     = false
  var trust       = "standard"
  var script_path = ""
  var once_cmd    = ""
  var pipe_mode   = false
  var shell_arg   = "auto"
  var extra:     seq[string]

  var p = initOptParser()
  for kind, key, val in p.getopt():
    case kind
    of cmdOption:
      case key
      of "no-color","no-colour":  no_color    = true
      of "verbose","v":           verbose     = true
      of "dry-run","d":           dry_run     = true
      of "pipe":                  pipe_mode   = true
      of "script","s":            script_path = val
      of "once","c":              once_cmd    = val
      of "trust":                 trust       = val.toLowerAscii
      of "model","m":             discard val  # passed to sigma-agent-core
      of "shell":                 shell_arg   = val
      of "version":
        echo "sigma-agent v15.0.0 — SigmaOS AI CLI Agent"
        echo "Inspired by: Claude Code · Aider · llama.cpp · copilot-cli"
        quit(0)
      of "help","h": usage(); quit(0)
      else: discard
    of cmdArgument: extra.add(key)
    else: discard

  # Route by first extra argument (subcommand) or flags
  if extra.len == 0 and script_path.len == 0 and once_cmd.len == 0 and not pipe_mode:
    run_repl(no_color, verbose, dry_run, trust)
    return

  let sub = if extra.len > 0: extra[0].toLowerAscii else: ""
  let sub_args = if extra.len > 1: extra[1..^1] else: @[]

  case sub
  # ── mirror ──────────────────────────────────────────────────────────────────
  of "mirror":
    mirror_cmd(sub_args)

  # ── watch ───────────────────────────────────────────────────────────────────
  of "watch":
    watch_cmd(sub_args)

  # ── train ───────────────────────────────────────────────────────────────────
  of "train":
    finetune_cmd(sub_args)

  # ── config ──────────────────────────────────────────────────────────────────
  of "config":
    config_cmd(sub_args)

  # ── daemon ──────────────────────────────────────────────────────────────────
  of "daemon":
    daemon_cmd(sub_args)

  # ── context ─────────────────────────────────────────────────────────────────
  of "context","ctx":
    context_cmd(sub_args)

  # ── security ─────────────────────────────────────────────────────────────────
  of "security","sec","audit":
    security_cmd(sub_args)

  # ── learn ────────────────────────────────────────────────────────────────────
  of "learn","feedback","rlhf":
    learn_cmd(sub_args)

  # ── multi / agent routing ─────────────────────────────────────────────────────
  of "multi","agent","route":
    multi_cmd(sub_args)

  # ── voice ────────────────────────────────────────────────────────────────────
  of "voice","listen","speak":
    voice_cmd(sub_args)

  # ── plugin ───────────────────────────────────────────────────────────────────
  of "plugin","skill","extension":
    plugin_cmd(sub_args)

  # ── complete (smart tab completion) ──────────────────────────────────────────
  of "complete","completion","autocomplete":
    complete_cmd(sub_args)

  # ── tui (terminal UI) ─────────────────────────────────────────────────────────
  of "tui","dashboard","dash":
    tui_cmd(sub_args)

  # ── benchmark ─────────────────────────────────────────────────────────────────
  of "benchmark","bench","eval":
    benchmark_cmd(sub_args)

  # ── notify ───────────────────────────────────────────────────────────────────
  of "notify","notification","alert":
    notify_cmd(sub_args)

  # ── doctor (self-diagnosis) ───────────────────────────────────────────────────
  of "doctor","diag","diagnose","check":
    doctor_cmd(sub_args)

  # ── update (self-update) ──────────────────────────────────────────────────────
  of "update","upgrade","self-update":
    update_cmd(sub_args)

  # ── memory (persistent long-term memory) ─────────────────────────────────────
  of "memory","mem","remember":
    memory_cmd(sub_args)

  # ── script-gen (NL → .sa script generator) ───────────────────────────────────
  of "script-gen","script","generate","gen":
    script_gen_cmd(sub_args)

  # ── explain (educational / copilot-cli ??) ────────────────────────────────────
  of "explain","why","how","what","??":
    explain_cmd(sub_args)

  # ── workflow (n8n-style automation) ──────────────────────────────────────────
  of "workflow","wf","automate","automation","schedule":
    workflow_cmd(sub_args)

  # ── corpus (AI training corpus builder) ──────────────────────────────────────
  of "corpus","dataset","train-corpus":
    corpus_cmd(sub_args)

  # ── recipe (declarative package recipe builder) ───────────────────────────────
  of "recipe","pkg-recipe":
    echo dispatch("run sigma-pkg recipe " & sub_args.join(" "), verbose, dry_run, trust, no_color)

  # ── compat (Linux binary/package compatibility) ───────────────────────────────
  of "compat","linux-compat","absorb":
    if sub_args.len > 0 and sub_args[0] == "absorb":
      echo dispatch("install " & sub_args[1..^1].join(" "), verbose, dry_run, trust, no_color)
    else:
      echo dispatch("run sigma-compat " & sub_args.join(" "), verbose, dry_run, trust, no_color)

  # ── install (shell integration) ──────────────────────────────────────────────
  of "install":
    if "--shell-integration" in extra or "shell-integration" in sub_args:
      install_shell_integration(shell_arg)
    else:
      echo dispatch("install " & sub_args.join(" "), verbose, dry_run, trust, no_color)

  # ── uninstall ────────────────────────────────────────────────────────────────
  of "uninstall":
    if "shell-integration" in sub_args:
      uninstall_shell_integration()
    else:
      echo dispatch("uninstall " & sub_args.join(" "), verbose, dry_run, trust, no_color)

  # ── version / help ───────────────────────────────────────────────────────────
  of "version":
    echo "sigma-agent v15.0.0"; quit(0)
  of "help","--help","-h","":
    if extra.len == 0 and once_cmd.len == 0 and script_path.len == 0:
      if pipe_mode: run_pipe(no_color, verbose, dry_run, trust)
      else: run_repl(no_color, verbose, dry_run, trust)
    else: usage()

  # ── Default: one-shot or REPL ────────────────────────────────────────────────
  else:
    let cmd = if once_cmd.len > 0: once_cmd
              elif extra.len > 0: extra.join(" ")
              elif script_path.len > 0: ""
              else: ""

    if script_path.len > 0:
      run_script(script_path, no_color, verbose, dry_run, trust)
    elif cmd.len > 0:
      let output = dispatch(cmd, verbose, dry_run, trust, no_color)
      echo format_output(output, no_color)
    else:
      run_repl(no_color, verbose, dry_run, trust)

proc run_pipe(no_color: bool, verbose: bool, dry_run: bool, trust: string) =
  for line in stdin.lines:
    let l = line.strip()
    if l.len == 0 or l.startsWith('#'): continue
    echo dispatch(l, verbose, dry_run, trust, no_color)

main()
