# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_autocomplete.nim — LLM-powered smart tab completion
# Goes beyond static keyword lists — uses context + LLM to suggest next tokens.
#
# Inspiration:
#   Claude Code inline completions (ghost text)
#   copilot-cli ?? / ?? / ??! suggestion model
#   Aider /add completion
#   llama.cpp completion API
#
# Modes:
#   Static:  instant keyword-based completions (no LLM, always fast)
#   Dynamic: LLM-powered contextual completions (requires daemon or Ollama)
#   Ghost:   inline partial-completion display in REPL (like fish)
#
# Integration:
#   sigma-agent complete "<partial>"     → prints suggestions
#   sigma-agent complete --dynamic "<partial>"  → LLM-powered
#   sigma-agent complete --shell         → emit bash/zsh completion script
#   Daemon endpoint: GET /v1/complete?q=<partial>
#
# Language: Nim (stdlib only)

import std/[os, osproc, strutils, strformat, tables, json, sequtils, algorithm]

# ── Static completion database ─────────────────────────────────────────────────
# Maps prefix → list of completions (sorted by usage frequency)
const STATIC_COMPLETIONS: array[56, (string, string)] = [
  # Core commands
  ("install ",        "install <package>"),
  ("install sigma-",  "install sigma-edit\ninstall sigma-terminal\ninstall sigma-browser\ninstall sigma-files\ninstall sigma-calc"),
  ("open app ",       "open app sigma-terminal\nopen app sigma-edit\nopen app sigma-files\nopen app sigma-browser\nopen app sigma-appstore"),
  ("list ",           "list /home/user\nlist /usr/bin\nlist /etc\nlist ."),
  ("read ",           "read README.md\nread /etc/hostname\nread /proc/version"),
  ("find ",           "find <name>\nfind <name> in /home/user"),
  ("run ",            "run cargo build\nrun nimble build\nrun sigma-pkg update"),
  # Settings
  ("set ",            "set dark mode\nset light mode\nset high contrast\nset dns 1.1.1.1"),
  ("set dark",        "set dark mode"),
  ("set light",       "set light mode"),
  ("set high",        "set high contrast"),
  ("settings ",       "settings get appearance theme\nsettings set appearance theme zenith-dark\nsettings set network firewall true"),
  ("settings get ",   "settings get appearance theme\nsettings get network firewall\nsettings get privacy telemetry"),
  ("settings set ",   "settings set appearance theme zenith-dark\nsettings set network firewall true\nsettings set privacy telemetry false"),
  # System
  ("system",          "system info"),
  ("show ",           "show processes\nshow running processes\nshow disk usage"),
  ("kill ",           "kill process <pid>"),
  ("disk ",           "disk usage"),
  # Network
  ("network",         "network status"),
  ("connect ",        "connect wifi <ssid> <password>"),
  ("connect wifi ",   "connect wifi <ssid> <password>"),
  ("vpn ",            "vpn connect <profile>\nvpn disconnect\nvpn list"),
  ("vpn connect ",    "vpn connect work-vpn\nvpn connect home-vpn"),
  ("set dns ",        "set dns 1.1.1.1\nset dns 9.9.9.9\nset dns 8.8.8.8"),
  # Accessibility
  ("accessibility ",  "accessibility high-contrast on\naccessibility large-text on\naccessibility screen-reader on\naccessibility reduce-motion on"),
  ("access",          "accessibility high-contrast on\naccessibility large-text on"),
  # Window manager
  ("workspace ",      "workspace 1\nworkspace 2\nworkspace 3"),
  ("tile",            "tile"),
  ("full",            "fullscreen"),
  # AI tools
  ("explain ",        "explain what sigma_pledge does\nexplain how paging works\nexplain what sigma-pkg does"),
  ("fix ",            "fix <file> <instruction>"),
  ("summarise ",      "summarise README.md\nsummarise /home/user/notes.md"),
  ("review ",         "review <file>"),
  # Notifications / clipboard
  ("notify ",         "notify 'Build done' --body 'Compiled successfully'"),
  ("copy ",           "copy <text>"),
  # Sub-agent commands
  ("mirror ",         "mirror list\nmirror run <action>\nmirror count"),
  ("mirror list",     "mirror list\nmirror list network\nmirror list settings"),
  ("watch ",          "watch .\nwatch . --ext .rs,.nim\nwatch . --suggest"),
  ("security ",       "security scan\nsecurity logs\nsecurity ports\nsecurity policies\nsecurity telemetry"),
  ("security s",      "security scan"),
  ("learn ",          "learn rate good\nlearn rate bad\nlearn rate excellent\nlearn correct\nlearn build\nlearn stats"),
  ("multi ",          "multi \"<input>\"\nmulti --agent developer\nmulti --agent security\nmulti --list"),
  ("voice",           "voice\nvoice --session\nvoice --secs 10\nvoice --status"),
  ("daemon ",         "daemon start\ndaemon stop\ndaemon status\ndaemon sync"),
  ("config ",         "config set model auto\nconfig profile code\nconfig profiles\nconfig alias\nconfig models"),
  ("train ",          "train seed\ntrain build\ntrain stats\ntrain sync"),
  ("plugin ",         "plugin list\nplugin install <name>\nplugin create <name>\nplugin example"),
  ("context",         "context\ncontext --json"),
  ("benchmark",       "benchmark\nbenchmark quick\nbenchmark full"),
  # Package management
  ("sigma-pkg ",      "sigma-pkg install\nsigma-pkg list\nsigma-pkg search\nsigma-pkg update\nsigma-pkg remove"),
  ("uninstall ",      "uninstall <package>"),
  # Common NL patterns
  ("what ",           "what does <command> do\nwhat is sigma_pledge\nwhat's running"),
  ("how ",            "how do I install\nhow do I connect wifi\nhow does paging work"),
  ("why ",            "why is CPU high\nwhy is system slow"),
]

proc static_complete*(partial: string, max = 5): seq[string] =
  ## Return instant static completions for a partial input
  let lower = partial.toLowerAscii.strip()
  var results: seq[string]
  # Exact prefix match first
  for (prefix, completions) in STATIC_COMPLETIONS:
    if lower.startsWith(prefix) or prefix.startsWith(lower):
      for line in completions.splitLines():
        if line.strip().len > 0 and line notin results:
          results.add(line.strip())
      if results.len >= max: break
  # Fuzzy: any word overlap
  if results.len < max:
    let words = lower.split()
    for (prefix, completions) in STATIC_COMPLETIONS:
      for w in words:
        if w in prefix and completions notin results:
          for line in completions.splitLines()[0..<min(2, completions.splitLines().len)]:
            if line.strip().len > 0 and line notin results:
              results.add(line.strip())
  results[0..<min(max, results.len)]

# ── Dynamic LLM-powered completion ────────────────────────────────────────────
proc dynamic_complete*(partial: string, max = 5, timeout_ms = 800): seq[string] =
  ## Ask the daemon or Ollama for contextual completions
  ## Falls back to static if not available within timeout

  # Try daemon first (fastest)
  let daemon_up = execCmdEx("curl -sf http://localhost:11430/v1/status --max-time 1")[1] == 0
  if daemon_up:
    let body = $ %*{"message": fmt"Complete this sigma-agent command (return top {max} completions, one per line, no explanation): {partial}",
                    "max_tokens": 80, "include_context": false}
    let (out, code) = execCmdEx(
      fmt"""curl -sf -X POST http://localhost:11430/v1/chat -d {body.quoteShell} --max-time 2""")
    if code == 0:
      try:
        let j = parseJson(out)
        let resp = j.getOrDefault("response").getStr("")
        let lines = resp.strip().splitLines().filterIt(it.strip().len > 0)
        if lines.len > 0: return lines[0..<min(max, lines.len)]
      except: discard

  # Fallback: Ollama
  let (ollama_ok, _) = execCmdEx("curl -sf http://localhost:11434/api/tags --max-time 1")
  if ollama_ok.len > 0:
    let prompt = fmt"""Complete this sigma-agent command. Output {max} completions, one per line, no explanation:
User typed: {partial}
Completions:"""
    let body = $ %*{"model":"tinyllama","prompt":prompt,"stream":false,"options":{%*{"num_predict":80}}}
    let (out, code) = execCmdEx(
      fmt"""curl -sf -X POST http://localhost:11434/api/generate -d {body.quoteShell} --max-time 3""")
    if code == 0:
      try:
        let j = parseJson(out)
        let resp = j.getOrDefault("response").getStr("")
        let lines = resp.strip().splitLines().filterIt(it.strip().len > 0)
        if lines.len > 0: return lines[0..<min(max, lines.len)]
      except: discard

  # Final fallback: static
  static_complete(partial, max)

# ── Ghost text (inline single-token prediction) ──────────────────────────────
proc ghost_text*(partial: string): string =
  ## Return a single ghost completion for inline display (like fish)
  ## Must be near-instant — static only
  let completions = static_complete(partial, 1)
  if completions.len == 0: return ""
  let full = completions[0]
  if full.len > partial.len and full.toLowerAscii.startsWith(partial.toLowerAscii):
    return full[partial.len..^1]
  ""

# ── Usage tracking (learns from what user actually runs) ─────────────────────
type UsageTracker = object
  counts: Table[string, int]
  path:   string

proc load_usage(path: string): UsageTracker =
  result.path   = path
  result.counts = initTable[string, int]()
  if not fileExists(path): return
  try:
    let j = parseJson(readFile(path))
    for k, v in j: result.counts[k] = v.getInt(0)
  except: discard

proc record_usage*(tracker: var UsageTracker, command: string) =
  let key = command.split()[0..min(1, command.split().len-1)].join(" ").toLowerAscii
  tracker.counts[key] = tracker.counts.getOrDefault(key, 0) + 1
  try:
    var j = newJObject()
    for k, v in tracker.counts: j[k] = %v
    writeFile(tracker.path, $j)
  except: discard

proc top_commands*(tracker: UsageTracker, n = 10): seq[string] =
  var sorted = toSeq(tracker.counts.pairs).sortedByIt(-it[1])
  sorted[0..<min(n, sorted.len)].mapIt(it[0])

# ── Bash/zsh completion script generator ─────────────────────────────────────
const BASH_COMPLETION_SCRIPT = """
# sigma-agent bash completion — generated by sigma-agent complete --shell
# Source this or place in /etc/bash_completion.d/sigma-agent

_sigma_agent_dynamic_complete() {
    local cur="${COMP_WORDS[*]:1}"
    if [ -z "$cur" ]; then
        COMPREPLY=($(compgen -W "install open list read write find run set settings system show kill disk network connect vpn accessibility workspace tile fullscreen explain fix summarise mirror watch security learn multi voice daemon config train plugin context" -- ""))
        return
    fi
    # Try dynamic completions from daemon
    local completions
    completions=$(sigma-agent complete "$cur" 2>/dev/null | head -8)
    if [ -n "$completions" ]; then
        COMPREPLY=($(compgen -W "$completions" -- ""))
    fi
}

complete -F _sigma_agent_dynamic_complete sigma-agent
complete -F _sigma_agent_dynamic_complete ai
complete -F _sigma_agent_dynamic_complete ask
"""

const FISH_COMPLETION_SCRIPT = """
# sigma-agent fish completion — generated by sigma-agent complete --shell fish

function __sigma_agent_completions
    set -l cmd (commandline -cp)
    sigma-agent complete "$cmd" 2>/dev/null
end

complete -c sigma-agent -f -a '(__sigma_agent_completions)'
complete -c ai -f -a '(__sigma_agent_completions)'
"""

# ── CLI ────────────────────────────────────────────────────────────────────────
proc complete_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "help":
    echo """sigma-agent complete — LLM-powered smart tab completion

Usage:
  sigma-agent complete "<partial>"           Static completions (instant)
  sigma-agent complete --dynamic "<partial>" LLM-powered completions
  sigma-agent complete --ghost "<partial>"   Single ghost-text token
  sigma-agent complete --shell               Emit bash completion script
  sigma-agent complete --shell fish          Emit fish completion script
  sigma-agent complete --top                 Show your top commands
  sigma-agent complete --usage "<command>"   Record command usage

Examples:
  sigma-agent complete "install sigma"
  sigma-agent complete --dynamic "why is my"
  sigma-agent complete --ghost "set dark"
  sigma-agent complete --shell > /etc/bash_completion.d/sigma-agent
"""
    return

  let dynamic = "--dynamic" in args
  let ghost   = "--ghost"   in args

  if "--shell" in args:
    if "fish" in args: echo FISH_COMPLETION_SCRIPT
    else:              echo BASH_COMPLETION_SCRIPT
    return

  if "--top" in args:
    let tracker = load_usage(getEnv("HOME","/tmp") / ".cache/sigma/agent_usage.json")
    echo "Top commands:"
    for cmd in tracker.top_commands():
      echo fmt"  {cmd:<30} {tracker.counts.getOrDefault(cmd, 0)} uses"
    return

  if "--usage" in args:
    let cmd_idx = args.find("--usage")
    if cmd_idx + 1 < args.len:
      var tracker = load_usage(getEnv("HOME","/tmp") / ".cache/sigma/agent_usage.json")
      record_usage(tracker, args[cmd_idx+1])
      echo fmt"✓ Recorded: {args[cmd_idx+1]}"
    return

  # Get partial input
  let partial = block:
    let non_flags = args.filterIt(not it.startsWith("-"))
    if non_flags.len > 0: non_flags.join(" ") else: ""

  if partial.len == 0:
    echo "Usage: sigma-agent complete \"<partial input>\""
    return

  let completions = if ghost:
    let g = ghost_text(partial)
    if g.len > 0: @[g] else: @[]
  elif dynamic: dynamic_complete(partial)
  else:         static_complete(partial)

  for c in completions:
    echo c
