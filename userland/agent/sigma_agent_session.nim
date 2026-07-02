# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_session.nim — Session manager, multi-modal REPL
# Inspiration: Claude Code, azure-cli, openclaw, copilot-cli
# Language: Nim — OOP via object + methods, native binary

import std/[os, strutils, osproc, tables, json, times, strformat, terminal,
            parseopt, streams, hashes]

# ── Session State ─────────────────────────────────────────────────────────────
type
  SessionMode = enum Interactive, Script, Pipe, Server

  SessionConfig = object
    mode:       SessionMode
    model:      string        # llm model name
    no_color:   bool
    verbose:    bool
    dry_run:    bool
    cwd:        string
    history_file: string
    max_history:  int
    prompt:       string      # custom prompt prefix
    trust_level:  TrustLevel

  TrustLevel = enum
    Safe,       # only read operations
    Standard,   # reads + installs + settings changes
    Full        # everything including shell execution

  SessionContext = object
    cwd:        string
    env_vars:   Table[string, string]
    last_cmd:   string
    last_output: string
    last_exit:  int
    turn_count: int
    start_time: DateTime

# ── Conversation Memory ───────────────────────────────────────────────────────
type
  MemoryEntry = object
    ts:       string
    user:     string
    agent:    string
    tools:    seq[string]
    success:  bool

  Memory = object
    entries:  seq[MemoryEntry]
    max_size: int

proc new_memory(max_size = 100): Memory =
  Memory(entries: @[], max_size: max_size)

proc push(m: var Memory, user, agent: string, tools: seq[string], ok: bool) =
  m.entries.add MemoryEntry(
    ts: $now(), user: user, agent: agent, tools: tools, success: ok)
  if m.entries.len > m.max_size: m.entries.delete(0)

proc relevant(m: Memory, query: string, top_n = 3): seq[MemoryEntry] =
  ## Return most relevant past entries (simple keyword match)
  var scored: seq[(int, MemoryEntry)]
  let words = query.toLowerAscii.splitWhitespace.toHashSet
  for e in m.entries:
    let score = e.user.toLowerAscii.splitWhitespace.toHashSet * words
    if score.len > 0: scored.add((score.len, e))
  scored.sort(proc(a,b:(int,MemoryEntry)):int = b[0] - a[0])
  scored[0..<min(top_n, scored.len)].mapIt(it[1])

proc save(m: Memory, path: string) =
  var arr = newJArray()
  for e in m.entries:
    arr.add(%*{"ts": e.ts, "user": e.user, "agent": e.agent,
               "tools": e.tools, "success": e.success})
  try: writeFile(path, $(%*{"memory": arr})) except: discard

proc load_memory(path: string): Memory =
  result = new_memory()
  try:
    let j = parseJson(readFile(path))
    for e in j["memory"]:
      result.entries.add MemoryEntry(
        ts:      e["ts"].getStr,
        user:    e["user"].getStr,
        agent:   e["agent"].getStr,
        tools:   e["tools"].getElems.mapIt(it.getStr),
        success: e["success"].getBool)
  except: discard

# ── Output Rendering ──────────────────────────────────────────────────────────
type OutputRenderer = object
  no_color:  bool
  markdown:  bool

const
  C_RESET  = "\e[0m"
  C_CYAN   = "\e[38;2;69;243;255m"
  C_GREEN  = "\e[38;2;52;211;153m"
  C_YELLOW = "\e[38;2;251;191;36m"
  C_RED    = "\e[38;2;248;113;113m"
  C_PURPLE = "\e[38;2;168;85;247m"
  C_MUTED  = "\e[38;2;107;114;128m"
  C_BOLD   = "\e[1m"
  C_DIM    = "\e[2m"

proc col(r: OutputRenderer, text, color: string): string =
  if r.no_color: text else: color & text & C_RESET

proc render_step(r: OutputRenderer, kind, content: string) =
  case kind
  of "thought":     echo r.col("  💭 " & content, C_MUTED)
  of "action":      echo r.col("  ⚡ " & content, C_CYAN)
  of "observation": echo r.col("  📋 " & content, C_YELLOW)
  of "answer":      echo r.col(content, C_GREEN)
  of "error":       echo r.col("  ✗ " & content, C_RED)
  else:             echo content

proc render_output(r: OutputRenderer, text: string) =
  if r.no_color: echo text; return
  for line in text.splitLines():
    let l = line
    if   l.startsWith("✓"):     echo C_GREEN  & l & C_RESET
    elif l.startsWith("✗") or l.startsWith("Error"): echo C_RED & l & C_RESET
    elif l.startsWith("  ") and l.contains("─"): echo C_MUTED & l & C_RESET
    elif l.startsWith("Σ") or l.startsWith("sigma"): echo C_CYAN & l & C_RESET
    else: echo l

proc render_prompt(r: OutputRenderer, cwd: string): string =
  let short_cwd = cwd.replace(getEnv("HOME",""), "~")
  if r.no_color: "sigma-agent " & short_cwd & "$ "
  else: C_CYAN & C_BOLD & "σ" & C_RESET & " " &
        C_MUTED & short_cwd & C_RESET & " " &
        C_CYAN & "›" & C_RESET & " "

# ── Shell Integration ─────────────────────────────────────────────────────────
proc run_tool_direct(cmd: string, ctx: SessionContext): (string, int) =
  ## Execute a tool command and return (output, exit_code)
  let full_cmd = "sigma-agent-core --once " & cmd.quoteShell
  let (output, code) = execCmdEx(full_cmd, workingDir=ctx.cwd)
  if code != 0:
    # Try direct execution
    let (out2, code2) = execCmdEx("sh -c " & cmd.quoteShell, workingDir=ctx.cwd)
    return (out2.strip(), code2)
  (output.strip(), code)

# ── Suggestion Engine ─────────────────────────────────────────────────────────
proc get_suggestions(partial: string): seq[string] =
  ## Fuzzy completions for the REPL (like copilot-cli suggest)
  const COMMANDS = [
    "install ", "open app ", "list ", "read ", "write ",
    "system info", "network status", "set dark mode", "set light mode",
    "show processes", "disk usage", "vpn connect ", "vpn disconnect",
    "accessibility high-contrast on", "accessibility large-text on",
    "find files ", "explain ", "summarise ", "fix ", "run ",
    "notify ", "copy ", "workspace ", "tile", "float",
  ]
  var result: seq[string]
  for cmd in COMMANDS:
    if cmd.startsWith(partial) and cmd != partial:
      result.add(cmd)
  result[0..<min(5, result.len)]

proc show_suggestions(partial: string, no_color: bool) =
  let suggestions = get_suggestions(partial)
  if suggestions.len == 0: return
  let prefix = if no_color: "  " else: C_MUTED & "  " & C_RESET
  for s in suggestions:
    let remainder = s[partial.len..^1]
    if no_color: echo "  " & partial & remainder
    else: echo "  " & partial & C_MUTED & remainder & C_RESET

# ── Streaming Output ──────────────────────────────────────────────────────────
proc stream_response(backend_cmd: string, no_color: bool) =
  ## Stream tokens from sigma-ai as they arrive (like Claude Code streaming)
  let (out, _) = execCmdEx(backend_cmd)
  let renderer = OutputRenderer(no_color: no_color)
  renderer.render_output(out)

# ── Main Session ──────────────────────────────────────────────────────────────
type Session = object
  config:   SessionConfig
  ctx:      SessionContext
  memory:   Memory
  renderer: OutputRenderer
  history:  seq[string]

proc new_session(cfg: SessionConfig): Session =
  let mem_file = getEnv("HOME","/tmp") / ".cache/sigma/agent_memory.json"
  Session(
    config:   cfg,
    ctx:      SessionContext(cwd: cfg.cwd, turn_count:0, start_time: now(),
                              env_vars: initTable[string,string]()),
    memory:   load_memory(mem_file),
    renderer: OutputRenderer(no_color: cfg.no_color),
    history:  @[],
  )

proc process_turn(s: var Session, input: string): bool =
  ## Process one user turn. Returns false if session should end.
  let trimmed = input.strip()
  if trimmed.len == 0: return true
  s.history.add(trimmed)
  s.ctx.last_cmd = trimmed
  s.ctx.turn_count += 1

  # Session commands
  case trimmed.toLowerAscii
  of "exit","quit","q","/quit",":q": return false
  of "clear","/clear":
    stdout.write("\e[2J\e[H"); return true
  of "history":
    for i, h in s.history[max(0,s.history.len-10)..^1]:
      echo s.renderer.col(fmt"  {i+1:>3}  {h}", C_MUTED)
    return true
  of "context","ctx":
    echo s.renderer.col(fmt"  cwd:   {s.ctx.cwd}", C_CYAN)
    echo s.renderer.col(fmt"  turns: {s.ctx.turn_count}", C_CYAN)
    echo s.renderer.col(fmt"  model: {s.config.model}", C_CYAN)
    echo s.renderer.col(fmt"  trust: {s.config.trust_level}", C_CYAN)
    return true
  of "memory":
    for e in s.memory.entries[max(0,s.memory.entries.len-5)..^1]:
      echo s.renderer.col(fmt"  [{e.ts[0..<16]}] {e.user[0..<60]}", C_MUTED)
    return true
  else: discard

  # Relevant memory context
  let relevant = s.memory.relevant(trimmed)
  var context_hint = ""
  if relevant.len > 0:
    context_hint = " (I recall: " & relevant[0].user[0..<40] & "...)"

  # Execute via sigma-agent-core
  let cmd = "sigma-agent-core --once " & trimmed.quoteShell
  let (output, exit_code) = execCmdEx(cmd, workingDir=s.ctx.cwd)
  s.ctx.last_output = output
  s.ctx.last_exit   = exit_code

  echo ""
  s.renderer.render_output(output)
  echo ""

  # Save to memory
  s.memory.push(trimmed, output[0..<min(200,output.len)], @[], exit_code == 0)
  let mem_file = getEnv("HOME","/tmp") / ".cache/sigma/agent_memory.json"
  s.memory.save(mem_file)
  true

proc run_interactive(s: var Session) =
  echo s.renderer.col("Σ sigma-agent v15.0 — AI CLI for SigmaOS", C_CYAN & C_BOLD)
  echo s.renderer.col(fmt"  Backend: {s.config.model}  |  Trust: {s.config.trust_level}  |  cwd: {s.ctx.cwd}", C_MUTED)
  echo s.renderer.col("  Type 'help' for commands, 'quit' to exit.", C_MUTED)
  echo ""
  while true:
    stdout.write(s.renderer.render_prompt(s.ctx.cwd))
    stdout.flushFile()
    var line = ""
    try: line = stdin.readLine()
    except EOFError, IOError: break
    if not s.process_turn(line): break

proc run_once_mode(s: var Session, cmd: string) =
  let (output, _) = execCmdEx("sigma-agent-core --once " & cmd.quoteShell,
                               workingDir=s.ctx.cwd)
  echo output

proc run_script_mode(s: var Session, path: string) =
  if not fileExists(path): stderr.writeLine("File not found: " & path); quit(1)
  for line in lines(path):
    let l = line.strip()
    if l.len == 0 or l.startsWith('#'): continue
    echo s.renderer.col("σ> " & l, C_MUTED)
    if not s.process_turn(l): break

proc run_pipe_mode(s: var Session) =
  for line in stdin.lines:
    let l = line.strip()
    if l.len == 0: continue
    if not s.process_turn(l): break

# ── Entry Point ───────────────────────────────────────────────────────────────
proc main() =
  var cfg = SessionConfig(
    mode:         Interactive,
    model:        "auto",
    no_color:     false,
    verbose:      false,
    dry_run:      false,
    cwd:          getCurrentDir(),
    history_file: getEnv("HOME","/tmp") / ".sigma_agent_history",
    max_history:  1000,
    prompt:       "σ",
    trust_level:  Standard,
  )
  var extra: seq[string]
  var script_path = ""

  var p = initOptParser()
  for kind, key, val in p.getopt():
    case kind
    of cmdOption:
      case key
      of "no-color","no-colour": cfg.no_color = true
      of "verbose","v":          cfg.verbose  = true
      of "dry-run","d":          cfg.dry_run  = true
      of "model","m":            cfg.model    = val
      of "script","s":           script_path  = val; cfg.mode = Script
      of "pipe":                 cfg.mode     = Pipe
      of "cwd":                  cfg.cwd      = val
      of "trust":
        cfg.trust_level = case val.toLowerAscii
          of "safe":     Safe
          of "full":     Full
          else:          Standard
      of "help","h":
        echo """sigma-agent-session — Multi-modal AI CLI session manager

Usage:
  sigma-agent-session                    Interactive REPL
  sigma-agent-session "<command>"        Single command
  sigma-agent-session --script <file>    Run script
  sigma-agent-session --pipe             Read commands from stdin
  sigma-agent-session --model <name>     Use specific LLM model
  sigma-agent-session --trust full       Allow all operations
  sigma-agent-session --dry-run          Preview actions only
  sigma-agent-session --no-color         Disable ANSI colours
"""
        quit(0)
      else: discard
    of cmdArgument: extra.add(key)
    else: discard

  var session = new_session(cfg)

  case cfg.mode
  of Script:      session.run_script_mode(script_path)
  of Pipe:        session.run_pipe_mode()
  of Interactive:
    if extra.len > 0: session.run_once_mode(extra.join(" "))
    else:             session.run_interactive()
  else: discard

main()
