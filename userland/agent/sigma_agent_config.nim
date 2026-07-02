# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_config.nim — Configuration, profiles, persistence
# Inspiration: azure-cli profiles, Claude Code .claude config
# Language: Nim — OOP via object + methods

import std/[os, json, tables, strutils, strformat]

# ── Config Schema ─────────────────────────────────────────────────────────────
type
  LlmProvider = enum Auto, LlamaCpp, Ollama, SigmaAi, None

  AgentProfile = object
    name:         string
    model:        string
    provider:     LlmProvider
    temperature:  float
    max_tokens:   int
    system_prompt: string
    trust_level:  string
    no_color:     bool
    verbose:      bool
    dry_run:      bool
    aliases:      Table[string, string]  # custom command aliases

  AgentConfig = object
    active_profile: string
    profiles:       Table[string, AgentProfile]
    history_file:   string
    memory_file:    string
    log_file:       string
    config_dir:     string
    model_dir:      string
    auto_update:    bool
    telemetry:      bool  # always false — SigmaOS privacy-first

# ── Default config ────────────────────────────────────────────────────────────
proc default_profile(name: string): AgentProfile =
  AgentProfile(
    name:          name,
    model:         "auto",
    provider:      Auto,
    temperature:   0.7,
    max_tokens:    512,
    system_prompt: "",
    trust_level:   "standard",
    no_color:      false,
    verbose:       false,
    dry_run:       false,
    aliases:       initTable[string, string](),
  )

proc default_config(): AgentConfig =
  let home = getEnv("HOME", "/tmp")
  let cfg_dir = home / ".config/sigma/agent"
  result = AgentConfig(
    active_profile: "default",
    profiles:       initTable[string, AgentProfile](),
    history_file:   home / ".sigma_agent_history",
    memory_file:    home / ".cache/sigma/agent_memory.json",
    log_file:       home / ".cache/sigma/agent.log",
    config_dir:     cfg_dir,
    model_dir:      home / ".cache/sigma/models",
    auto_update:    false,
    telemetry:      false,
  )
  result.profiles["default"] = default_profile("default")
  result.profiles["code"]    = AgentProfile(
    name: "code", model: "auto", provider: Auto,
    temperature: 0.1, max_tokens: 1024,
    system_prompt: "You are a SigmaOS code assistant. Be precise and concise.",
    trust_level: "full", no_color: false, verbose: false, dry_run: false,
    aliases: initTable[string, string](),
  )
  result.profiles["safe"] = AgentProfile(
    name: "safe", model: "auto", provider: Auto,
    temperature: 0.5, max_tokens: 256,
    system_prompt: "You are a read-only SigmaOS assistant. Never suggest file modifications.",
    trust_level: "safe", no_color: false, verbose: false, dry_run: true,
    aliases: initTable[string, string](),
  )

# ── Serialisation ─────────────────────────────────────────────────────────────
proc profile_to_json(p: AgentProfile): JsonNode =
  var aliases = newJObject()
  for k, v in p.aliases: aliases[k] = %v
  %*{
    "name": p.name, "model": p.model,
    "provider": $p.provider, "temperature": p.temperature,
    "max_tokens": p.max_tokens, "system_prompt": p.system_prompt,
    "trust_level": p.trust_level, "no_color": p.no_color,
    "verbose": p.verbose, "dry_run": p.dry_run, "aliases": aliases
  }

proc profile_from_json(j: JsonNode): AgentProfile =
  result = default_profile(j["name"].getStr("default"))
  result.model         = j.getOrDefault("model").getStr("auto")
  result.temperature   = j.getOrDefault("temperature").getFloat(0.7)
  result.max_tokens    = j.getOrDefault("max_tokens").getInt(512)
  result.system_prompt = j.getOrDefault("system_prompt").getStr("")
  result.trust_level   = j.getOrDefault("trust_level").getStr("standard")
  result.no_color      = j.getOrDefault("no_color").getBool(false)
  result.verbose       = j.getOrDefault("verbose").getBool(false)
  result.dry_run       = j.getOrDefault("dry_run").getBool(false)
  if j.hasKey("aliases"):
    for k, v in j["aliases"]: result.aliases[k] = v.getStr()

proc save(cfg: AgentConfig) =
  createDir(cfg.config_dir)
  var profiles_j = newJObject()
  for name, p in cfg.profiles: profiles_j[name] = profile_to_json(p)
  let j = %*{
    "active_profile": cfg.active_profile,
    "profiles":       profiles_j,
    "history_file":   cfg.history_file,
    "memory_file":    cfg.memory_file,
    "model_dir":      cfg.model_dir,
    "auto_update":    cfg.auto_update,
    "telemetry":      false,
  }
  writeFile(cfg.config_dir / "config.json", j.pretty())

proc load_config(): AgentConfig =
  result = default_config()
  let path = result.config_dir / "config.json"
  if not fileExists(path): return
  try:
    let j = parseJson(readFile(path))
    result.active_profile = j.getOrDefault("active_profile").getStr("default")
    result.history_file   = j.getOrDefault("history_file").getStr(result.history_file)
    result.memory_file    = j.getOrDefault("memory_file").getStr(result.memory_file)
    result.model_dir      = j.getOrDefault("model_dir").getStr(result.model_dir)
    if j.hasKey("profiles"):
      for name, pj in j["profiles"]:
        result.profiles[name] = profile_from_json(pj)
  except: discard

proc active_profile(cfg: AgentConfig): AgentProfile =
  cfg.profiles.getOrDefault(cfg.active_profile, default_profile("default"))

# ── Alias Expansion ───────────────────────────────────────────────────────────
proc expand_alias(cfg: AgentConfig, input: string): string =
  let profile = cfg.active_profile()
  let parts = input.splitWhitespace()
  if parts.len == 0: return input
  let cmd = parts[0]
  if cmd in profile.aliases:
    return profile.aliases[cmd] & " " & parts[1..^1].join(" ")
  input

# ── Model Management ──────────────────────────────────────────────────────────
proc list_models(cfg: AgentConfig): seq[string] =
  var models: seq[string]
  let model_dir = cfg.model_dir
  if dirExists(model_dir):
    for kind, path in walkDir(model_dir):
      if path.endsWith(".gguf"): models.add(path.extractFilename)
  models

proc download_model(cfg: AgentConfig, name: string): bool =
  ## Download a model from sigmaos package registry
  let (_, code) = execCmdEx(fmt"sigma-pkg install sigma-model-{name}")
  code == 0

# ── Config CLI ────────────────────────────────────────────────────────────────
proc config_cmd*(args: seq[string]) =
  var cfg = load_config()
  if args.len == 0:
    echo "sigma-agent config"
    echo fmt"  active profile: {cfg.active_profile}"
    echo fmt"  model:          {cfg.active_profile().model}"
    echo fmt"  trust:          {cfg.active_profile().trust_level}"
    echo fmt"  config dir:     {cfg.config_dir}"
    echo fmt"  profiles:       {toSeq(cfg.profiles.keys).join(', ')}"
    return

  case args[0]
  of "set":
    if args.len < 3: echo "Usage: config set <key> <value>"; return
    var p = cfg.active_profile()
    case args[1]
    of "model":       p.model       = args[2]
    of "temperature": p.temperature = parseFloat(args[2])
    of "trust":       p.trust_level = args[2]
    of "max-tokens":  p.max_tokens  = parseInt(args[2])
    of "verbose":     p.verbose     = args[2] == "true"
    of "dry-run":     p.dry_run     = args[2] == "true"
    cfg.profiles[cfg.active_profile] = p
    cfg.save()
    echo fmt"✓ {args[1]} = {args[2]}"
  of "profile":
    if args.len < 2: echo "Usage: config profile <name>"; return
    if args[1] notin cfg.profiles: echo fmt"Profile not found: {args[1]}"; return
    cfg.active_profile = args[1]
    cfg.save()
    echo fmt"✓ Active profile: {args[1]}"
  of "profiles":
    for name, p in cfg.profiles:
      let active = if name == cfg.active_profile: " ←" else: ""
      echo fmt"  {name:<12} model={p.model:<10} trust={p.trust_level}{active}"
  of "alias":
    if args.len < 3: echo "Usage: config alias <shortcut> <expansion>"; return
    var p = cfg.active_profile()
    p.aliases[args[1]] = args[2..^1].join(" ")
    cfg.profiles[cfg.active_profile] = p
    cfg.save()
    echo fmt"✓ alias {args[1]} → {p.aliases[args[1]]}"
  of "models":
    let models = cfg.list_models()
    if models.len == 0: echo "No models found in " & cfg.model_dir
    else:
      for m in models: echo "  " & m
  of "reset":
    cfg = default_config()
    cfg.save()
    echo "✓ Config reset to defaults"
  else:
    echo fmt"Unknown config command: {args[0]}"
    echo "Commands: set | profile | profiles | alias | models | reset"
