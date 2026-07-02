# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_doctor.nim — Self-diagnosis tool
# Checks that all sigma-agent components, backends, and integrations are working.
#
# Inspiration:
#   Claude Code `claude doctor` — environment health check
#   azure-cli `az self-test`    — SDK + extension validation
#   Aider diagnostics output    — model + repo + tool check
#
# Checks:
#   - Binary availability (sigma-agent, sigma-agent-core, sigma-pkg, sigma-netctl)
#   - LLM backends (sigma-ai, Ollama, llama.cpp, model files)
#   - Shell integration installation
#   - Daemon running status
#   - Training data presence
#   - Plugin directory
#   - Voice backend
#   - Rust engine compilation
#   - Configuration validity
#   - SigmaOS system compatibility
#
# Language: Nim (stdlib only)

import std/[os, osproc, strutils, strformat, json, times, sequtils]

# ── Check types ────────────────────────────────────────────────────────────────
type
  CheckStatus = enum ChkPass, ChkWarn, ChkFail, ChkSkip

  Check = object
    name:        string
    description: string
    status:      CheckStatus
    detail:      string
    fix:         string

const
  GREEN  = "\e[38;2;52;211;153m"
  YELLOW = "\e[38;2;251;191;36m"
  RED    = "\e[38;2;248;113;113m"
  CYAN   = "\e[38;2;69;243;255m"
  MUTED  = "\e[38;2;107;114;128m"
  BOLD   = "\e[1m"
  RESET  = "\e[0m"

proc status_icon(s: CheckStatus): string =
  case s
  of ChkPass: GREEN  & "✓" & RESET
  of ChkWarn: YELLOW & "⚠" & RESET
  of ChkFail: RED    & "✗" & RESET
  of ChkSkip: MUTED  & "○" & RESET

proc check_binary(name, fix: string): Check =
  let path = findExe(name)
  if path.len > 0:
    let (ver_out, _) = execCmdEx(fmt"{name} --version 2>/dev/null || {name} version 2>/dev/null || echo 'ok'")
    Check(name: name, description: fmt"Binary: {name}",
          status: ChkPass, detail: path & " " & ver_out.strip()[0..<min(40,ver_out.len)],
          fix: "")
  else:
    Check(name: name, description: fmt"Binary: {name}",
          status: ChkFail, detail: "not found in PATH",
          fix: fix)

# ── Individual checks ─────────────────────────────────────────────────────────
proc check_sigma_agent_nim(): Check =
  let path = findExe("sigma-agent")
  if path.len > 0:
    let (ver, _) = execCmdEx("sigma-agent --version 2>/dev/null")
    Check(name: "sigma-agent (Nim CLI)", description: "Main CLI entry point",
          status: ChkPass, detail: fmt"{path}  {ver.strip()}", fix: "")
  else:
    Check(name: "sigma-agent (Nim CLI)", description: "Main CLI entry point",
          status: ChkFail, detail: "Not found",
          fix: "cd userland/agent && nim c -d:release -o:sigma-agent sigma_agent_main.nim && cp sigma-agent /usr/bin/")

proc check_sigma_agent_core(): Check =
  let path = findExe("sigma-agent-core")
  if path.len > 0:
    Check(name: "sigma-agent-core (Rust)", description: "Rust tool executor engine",
          status: ChkPass, detail: path, fix: "")
  else:
    Check(name: "sigma-agent-core (Rust)", description: "Rust tool executor engine",
          status: ChkWarn, detail: "Optional — Nim fallback active",
          fix: "cargo build --release -p sigma-agent-core && cp target/release/sigma-agent-core /usr/bin/")

proc check_llm_backends(): seq[Check] =
  # 1. sigma-ai daemon
  let (_, sock_code) = execCmdEx("test -S /run/sigma/ai.sock 2>/dev/null")
  result.add Check(name: "sigma-ai daemon", description: "Sovereign local LLM daemon",
    status: if sock_code == 0: ChkPass else: ChkWarn,
    detail: if sock_code == 0: "/run/sigma/ai.sock connected" else: "Not running",
    fix: "sigma-pkg install sigma-ai && sigma-ai start")

  # 2. Ollama
  let (_, ollama_code) = execCmdEx("curl -sf http://localhost:11434/api/tags --max-time 2 2>/dev/null")
  result.add Check(name: "Ollama", description: "Ollama HTTP inference API",
    status: if ollama_code == 0: ChkPass else: ChkWarn,
    detail: if ollama_code == 0: "localhost:11434 reachable" else: "Not running",
    fix: "curl -fsSL https://ollama.ai/install.sh | sh && ollama pull tinyllama")

  # 3. llama.cpp
  let llama_bin = ["llama-cli","llama-cpp","sigma-llm"].filterIt(findExe(it).len > 0)
  let llama_found = llama_bin.len > 0
  result.add Check(name: "llama.cpp", description: "Local GGUF model inference",
    status: if llama_found: ChkPass else: ChkWarn,
    detail: if llama_found: llama_bin[0] else: "Not found",
    fix: "sigma-pkg install llama-cpp")

  # 4. GGUF model file
  let model_dir = getEnv("HOME","/tmp") / ".cache/sigma/models"
  var model_found = false; var model_name = ""
  if dirExists(model_dir):
    for _, path in walkDir(model_dir):
      if path.endsWith(".gguf"): model_found = true; model_name = path.extractFilename; break
  result.add Check(name: "GGUF model", description: "Local LLM model file",
    status: if model_found: ChkPass else: ChkWarn,
    detail: if model_found: fmt"{model_dir}/{model_name}" else: "No .gguf file in ~/.cache/sigma/models/",
    fix: "sigma-pkg install sigma-model-tinyllama  # or: ollama pull tinyllama")

proc check_shell_integration(): Check =
  let rc = getEnv("HOME","/tmp") / ".sigma_agent_rc"
  if fileExists(rc):
    let content = readFile(rc)
    let has_alias  = "alias ai=" in content
    let has_fn     = "ai_run()" in content
    let has_compl  = "_sigma_agent_complete" in content
    let ok = has_alias and has_fn and has_compl
    Check(name: "Shell integration", description: "bash/zsh/fish hooks + aliases",
          status: if ok: ChkPass else: ChkWarn,
          detail: fmt"~/.sigma_agent_rc  aliases={has_alias} functions={has_fn} completions={has_compl}",
          fix: "sigma-agent install --shell-integration && source ~/.sigma_agent_rc")
  else:
    Check(name: "Shell integration", description: "bash/zsh/fish hooks + aliases",
          status: ChkWarn, detail: "~/.sigma_agent_rc not found",
          fix: "sigma-agent install --shell-integration")

proc check_daemon(): Check =
  let (_, code) = execCmdEx("curl -sf http://localhost:11430/v1/status --max-time 2 2>/dev/null")
  let pid_file = "/run/sigma/agent.pid"
  if code == 0:
    let (status_out, _) = execCmdEx("curl -sf http://localhost:11430/v1/status --max-time 2")
    var detail = "Running on localhost:11430"
    try:
      let j = parseJson(status_out)
      detail = fmt"Running  backend={j[\"backend\"].getStr}  requests={j[\"requests\"].getInt}  knowledge={j[\"knowledge_pages\"].getInt} pages"
    except: discard
    Check(name: "sigma-agent daemon", description: "Background HTTP + socket service",
          status: ChkPass, detail: detail, fix: "")
  else:
    Check(name: "sigma-agent daemon", description: "Background HTTP + socket service",
          status: ChkWarn, detail: "Not running (optional but recommended)",
          fix: "sigma-agent daemon start")

proc check_training_data(): Check =
  let data_dir = getEnv("HOME","/tmp") / ".cache/sigma/agent_training"
  if dirExists(data_dir):
    var count = 0
    for _, path in walkDir(data_dir):
      if path.endsWith(".jsonl"):
        for line in lines(path):
          if line.strip().len > 0: count += 1
    Check(name: "Training data", description: "RLHF interaction history",
          status: if count > 0: ChkPass else: ChkWarn,
          detail: fmt"{count} samples in {data_dir}",
          fix: "sigma-agent train seed")
  else:
    Check(name: "Training data", description: "RLHF interaction history",
          status: ChkWarn, detail: "No training data yet",
          fix: "sigma-agent train seed")

proc check_plugins(): Check =
  let plugin_dir = getEnv("HOME","/tmp") / ".config/sigma/agent/plugins"
  let sys_dir    = "/usr/share/sigma/agent/plugins"
  var count = 0
  for d in [plugin_dir, sys_dir]:
    if dirExists(d):
      for kind, _ in walkDir(d):
        if kind == pcDir: count += 1
  Check(name: "Plugins", description: "Skill extension system",
        status: if count > 0: ChkPass else: ChkWarn,
        detail: fmt"{count} plugins installed",
        fix: "sigma-agent plugin example  # install the built-in example plugin")

proc check_voice(): Check =
  # sigma-voice daemon
  let (_, sock_ok) = execCmdEx("test -S /run/sigma/voice.sock 2>/dev/null")
  if sock_ok == 0:
    return Check(name: "Voice backend", description: "Speech-to-text for voice commands",
                 status: ChkPass, detail: "sigma-voice daemon at /run/sigma/voice.sock", fix: "")
  # whisper.cpp
  let whisper = ["whisper-cpp","whisper"].filterIt(findExe(it).len > 0)
  if whisper.len > 0:
    return Check(name: "Voice backend", description: "Speech-to-text",
                 status: ChkPass, detail: fmt"whisper.cpp: {whisper[0]}", fix: "")
  Check(name: "Voice backend", description: "Speech-to-text for voice commands",
        status: ChkWarn, detail: "No STT backend found",
        fix: "sigma-pkg install whisper-cpp && sigma-pkg install whisper-model-base-en")

proc check_config(): Check =
  let cfg_file = getEnv("HOME","/tmp") / ".config/sigma/agent/config.json"
  if fileExists(cfg_file):
    try:
      let j = parseJson(readFile(cfg_file))
      let profile = j.getOrDefault("active_profile").getStr("default")
      Check(name: "Configuration", description: "Agent config + profiles",
            status: ChkPass,
            detail: fmt"Profile: {profile}  {cfg_file}", fix: "")
    except:
      Check(name: "Configuration", description: "Agent config + profiles",
            status: ChkWarn, detail: "Config file invalid JSON",
            fix: "sigma-agent config reset")
  else:
    Check(name: "Configuration", description: "Agent config + profiles",
          status: ChkWarn, detail: "Not yet configured (defaults will be used)",
          fix: "sigma-agent config  # view defaults")

proc check_sigmaos_tools(): seq[Check] =
  const TOOLS = [
    ("sigma-pkg",    "Package manager",       "sigma-pkg install sigma-agent"),
    ("sigma-netctl", "Network control",       "sigma-pkg install sigma-netctl"),
    ("sigma-vpn",    "VPN management",        "sigma-pkg install sigma-vpn"),
    ("sigma-disks",  "Disk management",       "sigma-pkg install sigma-disks"),
    ("sigma-notify", "Desktop notifications", "sigma-pkg install sigma-notify"),
    ("sigma-log",    "Log viewer",            "sigma-pkg install sigma-log"),
  ]
  for (bin, desc, fix) in TOOLS:
    let path = findExe(bin)
    result.add Check(name: bin, description: desc,
      status: if path.len > 0: ChkPass else: ChkWarn,
      detail: if path.len > 0: path else: "not found (optional)",
      fix: if path.len > 0: "" else: fix)

# ── Full diagnosis ─────────────────────────────────────────────────────────────
proc run_doctor*(verbose = false): int =
  ## Returns exit code: 0=all pass, 1=warnings, 2=failures
  var all_checks: seq[Check]
  echo fmt"\n{CYAN}{BOLD}Σ sigma-agent doctor\e[0m  {MUTED}v15.0 environment check{RESET}\n"

  proc section(name: string, checks: seq[Check]) =
    echo BOLD & fmt"  {name}" & RESET
    for c in checks:
      all_checks.add(c)
      let icon = status_icon(c.status)
      echo fmt"  {icon}  {c.name:<30} {MUTED}{c.detail}{RESET}"
      if verbose and c.fix.len > 0 and c.status != ChkPass:
        echo fmt"        {YELLOW}Fix:{RESET} {c.fix}"
    echo ""

  section("Core binaries", @[check_sigma_agent_nim(), check_sigma_agent_core()])
  section("LLM backends",  check_llm_backends())
  section("Integration",   @[check_shell_integration(), check_daemon()])
  section("Data & plugins",@[check_training_data(), check_plugins(), check_voice(), check_config()])
  section("SigmaOS tools", check_sigmaos_tools())

  let pass  = all_checks.filterIt(it.status == ChkPass).len
  let warn  = all_checks.filterIt(it.status == ChkWarn).len
  let fail  = all_checks.filterIt(it.status == ChkFail).len
  let total = all_checks.len

  let score_color = if fail == 0 and warn <= 2: GREEN elif fail == 0: YELLOW else: RED
  echo fmt"  {score_color}{BOLD}Results: {pass}/{total} passed{RESET}  {YELLOW}{warn} warnings{RESET}  {RED}{fail} failures{RESET}"

  if fail == 0 and warn == 0:
    echo fmt"\n  {GREEN}{BOLD}✓ sigma-agent is fully configured and ready{RESET}"
  elif fail == 0:
    echo fmt"\n  {YELLOW}⚠ Some optional components are missing. Run with --verbose for fix commands.{RESET}"
  else:
    echo fmt"\n  {RED}✗ Critical components missing. Run: sigma-agent doctor --verbose{RESET}"

  if fail > 0: 2 elif warn > 0: 1 else: 0

# ── CLI ────────────────────────────────────────────────────────────────────────
proc doctor_cmd*(args: seq[string]) =
  let verbose = "--verbose" in args or "-v" in args
  let code = run_doctor(verbose)
  quit(code)
