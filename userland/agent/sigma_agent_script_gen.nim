# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_script_gen.nim — Natural language → .sa script generator
# Converts a high-level goal into a runnable multi-step .sa script file.
#
# Inspiration:
#   ai-shell ??!           — "turn this into a script"
#   Claude Code /generate  — generate scripts from descriptions
#   copilot-cli git?        — "explain then do"
#   Aider /architect        — plan before implement
#   openai-cli              — streaming script generation
#
# Features:
#   - Parse goal into ordered steps using LLM or rule-based planner
#   - Output valid .sa script with comments
#   - Dry-run preview before writing
#   - Integrate with daemon for LLM-powered generation
#   - Library of built-in script templates (setup, backup, deploy, etc.)
#
# Language: Nim (stdlib only)

import std/[os, osproc, strutils, strformat, times, json, sequtils, tables]

# ── Built-in script templates ─────────────────────────────────────────────────
const TEMPLATES: array[12, (string, string, string)] = [
  ("dev-setup",     "Set up a development environment",
   """# sigma-agent script: dev-setup.sa
# Generated: {DATE}
# Goal: Set up development environment

install sigma-edit
install sigma-terminal
install sigma-browser
set dark mode
accessibility large-text off
notify "Dev setup" --body "Development environment ready"
"""),

  ("security-harden", "Harden system security",
   """# sigma-agent script: security-harden.sa
# Generated: {DATE}
# Goal: Harden system security

security scan
settings set network firewall true
settings set privacy telemetry false
accessibility sticky-keys off
notify "Security" --body "Hardening applied" --critical
"""),

  ("backup",        "Backup important files",
   """# sigma-agent script: backup.sa
# Generated: {DATE}
# Goal: Backup user files

run mkdir -p /backup/home
run cp -r /home/user/Documents /backup/home/
run cp -r /home/user/Code /backup/home/
run ls -la /backup/home/
notify "Backup" --body "Backup complete"
"""),

  ("update-system", "Update all packages and system",
   """# sigma-agent script: update-system.sa
# Generated: {DATE}
# Goal: Full system update

run sigma-pkg update
run sigma-pkg list
security scan --quick
notify "Update" --body "System updated successfully"
"""),

  ("network-setup", "Configure network settings",
   """# sigma-agent script: network-setup.sa
# Generated: {DATE}
# Goal: Configure network

network status
settings set network firewall true
settings set network dns 1.1.1.1
notify "Network" --body "Network configured"
"""),

  ("workspace-init","Initialize a new workspace",
   """# sigma-agent script: workspace-init.sa
# Generated: {DATE}
# Goal: Initialize workspace

run mkdir -p {PROJECT}/src
run mkdir -p {PROJECT}/tests
run mkdir -p {PROJECT}/docs
write {PROJECT}/README.md # {PROJECT}
memory project init
notify "Workspace" --body "Workspace initialized: {PROJECT}"
"""),

  ("install-tools", "Install developer tools",
   """# sigma-agent script: install-tools.sa
# Generated: {DATE}
# Goal: Install developer tools

install sigma-edit
install sigma-terminal
run sigma-pkg search sigma-debug
notify "Tools" --body "Developer tools installed"
"""),

  ("accessibility-setup", "Configure accessibility settings",
   """# sigma-agent script: accessibility-setup.sa
# Generated: {DATE}
# Goal: Configure accessibility

accessibility high-contrast on
accessibility large-text on
accessibility reduce-motion on
accessibility screen-reader off
settings set appearance animations false
notify "Accessibility" --body "Accessibility configured"
"""),

  ("dark-mode",     "Switch to dark mode",
   """# sigma-agent script: dark-mode.sa
# Generated: {DATE}

set dark mode
settings set appearance corner_radius 8
settings set appearance animations true
"""),

  ("privacy-mode",  "Enable privacy settings",
   """# sigma-agent script: privacy-mode.sa
# Generated: {DATE}
# Goal: Enable privacy mode

settings set privacy telemetry false
settings set privacy crash_reports false
settings set network firewall true
settings set privacy clipboard_guard true
notify "Privacy" --body "Privacy mode enabled"
"""),

  ("kiosk-mode",    "Set up kiosk / locked-down mode",
   """# sigma-agent script: kiosk-mode.sa
# Generated: {DATE}
# Goal: Set up locked kiosk

settings set accessibility kiosk true
settings set network firewall true
settings set privacy clipboard_guard true
accessibility reduce-motion on
"""),

  ("ai-setup",      "Set up sigma-agent AI backends",
   """# sigma-agent script: ai-setup.sa
# Generated: {DATE}
# Goal: Set up AI backends

run sigma-pkg install sigma-ai
run sigma-pkg install llama-cpp
run sigma-pkg install sigma-model-tinyllama
run sigma-pkg install whisper-cpp
run sigma-pkg install sigma-voice
sigma-agent daemon start
sigma-agent doctor
notify "AI setup" --body "sigma-agent fully configured"
"""),
]

proc get_template(name: string): string =
  for (n, _, tmpl) in TEMPLATES:
    if n.toLowerAscii == name.toLowerAscii: return tmpl
  ""

# ── LLM-powered script generation ────────────────────────────────────────────
proc generate_with_llm(goal: string): string =
  ## Ask daemon or Ollama to generate a .sa script for the goal
  let system_prompt = """You are sigma-agent, an AI CLI agent for SigmaOS.
Generate a .sa script (sigma-agent script format) for the given goal.
Each line is a sigma-agent natural language command.
Comments start with #.
Available commands: install <pkg>, open app <name>, set dark/light mode,
system info, network status, run <cmd>, settings set/get, accessibility <feature> on/off,
notify "title" "body", find <query>, read <file>, write <file> <content>,
disk usage, show processes, kill process <pid>, vpn connect <profile>.
Output ONLY the script content, no explanation."""

  # Try daemon
  let daemon_up = execCmdEx("curl -sf http://localhost:11430/v1/status --max-time 1 2>/dev/null")[1] == 0
  if daemon_up:
    let body = $ %*{"message": fmt"Generate a .sa script for: {goal}",
                    "max_tokens": 400, "include_context": false}
    let (out, code) = execCmdEx(
      fmt"""curl -sf -X POST http://localhost:11430/v1/chat -d {body.quoteShell} --max-time 10""")
    if code == 0:
      try:
        return parseJson(out).getOrDefault("response").getStr("")
      except: discard

  # Try Ollama
  let prompt = fmt"""{system_prompt}

Goal: {goal}
Script:"""
  let body = $ %*{"model":"tinyllama","prompt":prompt,"stream":false,
                   "options":{%*{"num_predict":400,"temperature":0.3}}}
  let (out, code) = execCmdEx(
    fmt"""curl -sf -X POST http://localhost:11434/api/generate -d {body.quoteShell} --max-time 15 2>/dev/null""")
  if code == 0:
    try: return parseJson(out).getOrDefault("response").getStr("") except: discard
  ""

# ── Rule-based script planner (offline fallback) ──────────────────────────────
proc plan_steps(goal: string): seq[string] =
  ## Map a goal to ordered .sa steps without LLM
  let lower = goal.toLowerAscii
  var steps: seq[string]

  steps.add(fmt"# sigma-agent script for: {goal}")
  steps.add(fmt"# Generated: {$now()}")
  steps.add("")

  if "install" in lower:
    let app = goal.split(' ').filterIt(it.len > 3 and it != "install").getOrDefault(0, "<package>")
    steps.add(fmt"install {app}")
    steps.add(fmt"open app {app}")

  elif "backup" in lower or "save" in lower:
    steps.add("run mkdir -p /backup")
    steps.add("run cp -r /home/user/Documents /backup/")
    steps.add("disk usage")
    steps.add("notify \"Backup\" --body \"Backup complete\"")

  elif "setup" in lower or "configure" in lower:
    steps.add("system info")
    steps.add("set dark mode")
    steps.add("settings set network firewall true")
    steps.add("sigma-agent doctor")
    steps.add(fmt"notify \"Setup\" --body \"{goal} complete\"")

  elif "secure" in lower or "harden" in lower or "security" in lower:
    steps.add("security scan")
    steps.add("settings set network firewall true")
    steps.add("settings set privacy telemetry false")
    steps.add("notify \"Security\" --body \"Hardening applied\" --critical")

  elif "update" in lower:
    steps.add("run sigma-pkg update")
    steps.add("security scan --quick")
    steps.add("notify \"Update\" --body \"System updated\"")

  elif "network" in lower or "wifi" in lower:
    steps.add("network status")
    steps.add("settings set network firewall true")
    steps.add("notify \"Network\" --body \"Network configured\"")

  elif "dark" in lower or "theme" in lower or "appearance" in lower:
    steps.add("set dark mode")
    steps.add("settings set appearance animations true")

  elif "accessibility" in lower or "a11y" in lower:
    steps.add("accessibility high-contrast on")
    steps.add("accessibility large-text on")
    steps.add("accessibility reduce-motion on")

  else:
    # Generic: system check + run + notify
    steps.add("system info")
    steps.add(fmt"run # TODO: add commands for: {goal}")
    steps.add(fmt"notify \"{goal[0..<min(30,goal.len)]}\" --body \"Done\"")

  steps

# ── Script writer ──────────────────────────────────────────────────────────────
proc generate_script*(goal: string, output_path: string,
                      use_llm = true, dry_run = false): string =
  ## Generate a .sa script for goal and write to output_path
  var content = ""

  # 1. Try template match
  let lower = goal.toLowerAscii
  for (name, desc, tmpl) in TEMPLATES:
    if name.toLowerAscii in lower or desc.toLowerAscii.splitWhitespace.anyIt(it in lower):
      content = tmpl.replace("{DATE}", $now().format("yyyy-MM-dd"))
                    .replace("{PROJECT}", goal.split(' ')[^1])
      break

  # 2. Try LLM
  if content.len == 0 and use_llm:
    content = generate_with_llm(goal)
    if content.len > 10:
      # Prepend comment header
      content = fmt"# sigma-agent script: generated.sa" & "\n" &
                fmt"# Goal: {goal}" & "\n" &
                fmt"# Generated: {$now().format(\"yyyy-MM-dd\")}" & "\n\n" & content

  # 3. Rule-based fallback
  if content.len == 0:
    content = plan_steps(goal).join("\n")

  if dry_run:
    return content

  writeFile(output_path, content)
  content

# ── CLI ────────────────────────────────────────────────────────────────────────
proc script_gen_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "help":
    echo """sigma-agent script-gen — Natural language → .sa script generator

Usage:
  sigma-agent script-gen "goal"              Generate script, print to stdout
  sigma-agent script-gen "goal" -o out.sa   Generate and save to file
  sigma-agent script-gen "goal" --dry-run   Preview without saving
  sigma-agent script-gen --list             List built-in templates
  sigma-agent script-gen --template <name>  Use a specific template
  sigma-agent script-gen --run "goal"       Generate + run immediately

Examples:
  sigma-agent script-gen "set up my development environment"
  sigma-agent script-gen "harden system security" -o harden.sa
  sigma-agent script-gen "backup home directory" --dry-run
  sigma-agent script-gen --template dev-setup -o ~/setup.sa
  sigma-agent script-gen --run "install sigma-edit and configure dark mode"
  sigma-agent script-gen --list

Script format (.sa files):
  Each line is a sigma-agent natural language command.
  Lines starting with # are comments.
  Run with: sigma-agent --script <file>
"""
    return

  if args[0] == "--list":
    echo "\e[38;2;69;243;255m\e[1mΣ Built-in script templates:\e[0m\n"
    for (name, desc, _) in TEMPLATES:
      echo fmt"  {name:<20} {desc}"
    echo fmt"\nUse: sigma-agent script-gen --template <name> -o output.sa"
    return

  let dry_run  = "--dry-run" in args or "-d" in args
  let do_run   = "--run" in args
  let no_llm   = "--no-llm" in args

  # --template override
  if "--template" in args:
    let ti = args.find("--template")
    if ti + 1 < args.len:
      let tmpl = get_template(args[ti+1])
      if tmpl.len == 0:
        echo fmt"Template not found: {args[ti+1]}"
        echo "Run: sigma-agent script-gen --list"
        return
      let content = tmpl.replace("{DATE}", $now().format("yyyy-MM-dd"))
      let oi = args.find("-o")
      if oi >= 0 and oi + 1 < args.len:
        writeFile(args[oi+1], content)
        echo fmt"✓ Script written to: {args[oi+1]}"
        echo fmt"  Run with: sigma-agent --script {args[oi+1]}"
      else:
        echo content
      return

  # Get goal from non-flag args
  let goal = args.filterIt(not it.startsWith("-") and it != args.getOrDefault(args.find("-o")+1,"__none__")).join(" ")
  if goal.len == 0:
    echo "Usage: sigma-agent script-gen \"your goal here\""; return

  # Find output path
  let oi = args.find("-o")
  let output_path = if oi >= 0 and oi+1 < args.len: args[oi+1]
                    else: "/tmp/sigma_generated.sa"

  let content = generate_script(goal, output_path, use_llm = not no_llm, dry_run = dry_run)

  if dry_run:
    echo "\e[38;2;107;114;128m[dry-run] Generated script:\e[0m"
    echo content
    return

  if oi < 0:
    # No -o flag: print to stdout
    echo content
  else:
    echo fmt"\e[38;2;52;211;153m✓ Script written to: {output_path}\e[0m"
    echo fmt"  Run with: sigma-agent --script {output_path}"
    echo fmt"  Preview:  sigma-agent --script {output_path} --dry-run"

  if do_run:
    echo "\n\e[38;2;69;243;255mRunning generated script...\e[0m\n"
    let (out, code) = execCmdEx(fmt"sigma-agent --script {output_path.quoteShell} 2>&1")
    echo out
    if code == 0: echo "\e[38;2;52;211;153m✓ Script complete\e[0m"
    else: echo fmt"\e[38;2;248;113;113m✗ Script exited with code {code}\e[0m"
