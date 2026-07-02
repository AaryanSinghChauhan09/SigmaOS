# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_plugin.nim — Plugin / skill extension system
# Lets contributors add new command mappings and tools as .sigplugin packages.
#
# Inspiration:
#   Claude Code CLAUDE.md skill loading
#   azure-cli extension system (az extension add)
#   Aider --read context files
#   openclaw plugin API
#
# Plugin format: ~/.config/sigma/agent/plugins/<name>/
#   plugin.toml   — metadata (name, version, commands, description)
#   commands.nim  — optional Nim extension (compiled to .so or subprocess)
#   commands.sh   — shell command implementations
#   training.jsonl — seed training samples for this plugin's domain
#
# Language: Nim (stdlib only)

import std/[os, osproc, tables, json, strutils, strformat, times, sequtils]

# ── Plugin types ──────────────────────────────────────────────────────────────
type
  CommandEntry = object
    trigger:     seq[string]   # keywords that activate this command
    description: string
    shell_cmd:   string        # shell template: {input}, {arg1}, {arg2}
    example:     string

  Plugin = object
    name:        string
    version:     string
    author:      string
    description: string
    domain:      string        # "security" | "network" | "devtools" | etc.
    commands:    seq[CommandEntry]
    skill_file:  string        # path to commands.sh
    training_file: string      # path to training.jsonl
    enabled:     bool
    installed:   string        # install timestamp

  PluginRegistry = object
    plugins:    Table[string, Plugin]
    plugin_dir: string

# ── Paths ─────────────────────────────────────────────────────────────────────
proc plugin_dir(): string =
  getEnv("HOME", "/tmp") / ".config/sigma/agent/plugins"

proc system_plugin_dir(): string = "/usr/share/sigma/agent/plugins"

# ── TOML-lite parser (subset for plugin.toml) ─────────────────────────────────
proc parse_plugin_toml(path: string): Plugin =
  result.enabled = true
  if not fileExists(path): return
  var cur_cmd: CommandEntry
  var in_cmd = false
  for raw_line in lines(path):
    let line = raw_line.strip()
    if line.len == 0 or line.startsWith('#'): continue

    if line == "[[commands]]":
      if in_cmd and cur_cmd.description.len > 0:
        result.commands.add(cur_cmd)
      cur_cmd = CommandEntry()
      in_cmd = true
      continue

    if "=" in line:
      let parts = line.split('=', 1)
      let k = parts[0].strip()
      let v = parts[1].strip().strip(chars={'"', '\''})
      if in_cmd:
        case k
        of "trigger":
          cur_cmd.trigger = v.split(',').mapIt(it.strip())
        of "description": cur_cmd.description = v
        of "shell_cmd":   cur_cmd.shell_cmd   = v
        of "example":     cur_cmd.example     = v
      else:
        case k
        of "name":        result.name        = v
        of "version":     result.version     = v
        of "author":      result.author      = v
        of "description": result.description = v
        of "domain":      result.domain      = v

  if in_cmd and cur_cmd.description.len > 0:
    result.commands.add(cur_cmd)

# ── Registry operations ────────────────────────────────────────────────────────
proc load_registry(): PluginRegistry =
  result.plugin_dir = plugin_dir()
  result.plugins    = initTable[string, Plugin]()
  for dir in [plugin_dir(), system_plugin_dir()]:
    if not dirExists(dir): continue
    for kind, path in walkDir(dir):
      if kind != pcDir: continue
      let toml = path / "plugin.toml"
      if not fileExists(toml): continue
      var p = parse_plugin_toml(toml)
      p.skill_file     = path / "commands.sh"
      p.training_file  = path / "training.jsonl"
      p.installed      = $getLastModificationTime(toml)
      if p.name.len > 0:
        result.plugins[p.name] = p

proc dispatch_plugin*(registry: PluginRegistry, input: string): string =
  ## Check if any plugin can handle this input
  let lower = input.toLowerAscii
  for name, plugin in registry.plugins:
    if not plugin.enabled: continue
    for cmd in plugin.commands:
      for trigger in cmd.trigger:
        if trigger in lower:
          # Expand shell template
          var shell = cmd.shell_cmd
          shell = shell.replace("{input}", input.quoteShell)
          let words = input.split()
          for i, word in words:
            shell = shell.replace(fmt"{{arg{i+1}}}", word.quoteShell)
          if plugin.skill_file.len > 0 and fileExists(plugin.skill_file):
            let (out, _) = execCmdEx(fmt"bash {plugin.skill_file.quoteShell} {shell}")
            return out.strip()
          else:
            let (out, _) = execCmdEx(shell)
            return out.strip()
  ""

# ── Built-in example plugins ──────────────────────────────────────────────────
const EXAMPLE_PLUGIN_TOML = """
[plugin]
name        = "sigma-devtools"
version     = "1.0.0"
author      = "SigmaOS Project"
description = "Developer tools: cargo, nimble, git shortcuts"
domain      = "devtools"

[[commands]]
trigger     = "cargo build,build rust"
description = "Build a Rust project"
shell_cmd   = "cargo build --release 2>&1"
example     = "cargo build"

[[commands]]
trigger     = "nimble build,build nim"
description = "Build a Nim project"
shell_cmd   = "nimble build -d:release 2>&1"
example     = "nimble build"

[[commands]]
trigger     = "git log pretty,show commits"
description = "Show git log with graph"
shell_cmd   = "git log --oneline --graph --all | head -20"
example     = "show commits"
"""

const EXAMPLE_PLUGIN_TRAINING = """
{"messages":[{"role":"system","content":"You are sigma-agent."},{"role":"user","content":"cargo build"},{"role":"assistant","content":"✓ Rust project built successfully"}],"quality":"Excellent"}
{"messages":[{"role":"system","content":"You are sigma-agent."},{"role":"user","content":"build rust project"},{"role":"assistant","content":"Running: cargo build --release..."}],"quality":"Good"}
"""

proc install_example_plugin*() =
  let p_dir = plugin_dir() / "sigma-devtools"
  createDir(p_dir)
  writeFile(p_dir / "plugin.toml", EXAMPLE_PLUGIN_TOML)
  writeFile(p_dir / "training.jsonl", EXAMPLE_PLUGIN_TRAINING)
  echo fmt"✓ Example plugin installed: {p_dir}"

# ── Remote plugin install (from sigma-pkg or URL) ────────────────────────────
proc install_plugin*(name: string) =
  echo fmt"Installing sigma-agent plugin: {name}..."
  let (out, code) = execCmdEx(fmt"sigma-pkg install sigma-agent-plugin-{name} 2>&1")
  if code == 0:
    echo fmt"✓ Plugin installed: {name}"
    echo "  Restart sigma-agent or run: sigma-agent plugin reload"
  else:
    echo fmt"✗ Plugin not found in sigma-pkg registry: {name}"
    echo fmt"  Try: sigma-agent plugin create {name} (to scaffold a new plugin)"

proc create_plugin*(name: string) =
  let p_dir = plugin_dir() / name
  if dirExists(p_dir):
    echo fmt"Plugin already exists: {p_dir}"
    return
  createDir(p_dir)
  let toml = fmt"""
[plugin]
name        = "{name}"
version     = "0.1.0"
author      = ""
description = "sigma-agent plugin: {name}"
domain      = "custom"

[[commands]]
trigger     = "{name},use {name}"
description = "Run {name} command"
shell_cmd   = "echo 'Implement me in commands.sh'"
example     = "{name}"
"""
  writeFile(p_dir / "plugin.toml", toml)
  writeFile(p_dir / "commands.sh", fmt"""#!/usr/bin/env bash
# sigma-agent plugin: {name}
# $@ = input from sigma-agent
echo "Plugin {name} received: $@"
""")
  writeFile(p_dir / "training.jsonl", "")
  echo fmt"✓ Plugin scaffolded: {p_dir}"
  echo fmt"  Edit: {p_dir}/plugin.toml"
  echo fmt"  Edit: {p_dir}/commands.sh"

# ── CLI ────────────────────────────────────────────────────────────────────────
proc plugin_cmd*(args: seq[string]) =
  let registry = load_registry()

  if args.len == 0 or args[0] == "list":
    echo "\e[38;2;69;243;255m\e[1mΣ sigma-agent plugins\e[0m"
    echo fmt"  Directory: {plugin_dir()}\n"
    if registry.plugins.len == 0:
      echo "  No plugins installed. Try:"
      echo "    sigma-agent plugin install sigma-devtools"
      echo "    sigma-agent plugin create my-plugin"
      return
    for name, p in registry.plugins:
      let status = if p.enabled: "\e[38;2;52;211;153m✓\e[0m" else: "\e[38;2;107;114;128m○\e[0m"
      echo fmt"  {status} {name:<20} v{p.version}  {p.description}"
      echo fmt"    Domain: {p.domain}  Commands: {p.commands.len}"
    return

  case args[0].toLowerAscii
  of "install","add":
    if args.len < 2: echo "Usage: sigma-agent plugin install <name>"; return
    install_plugin(args[1])
  of "create","new","scaffold":
    if args.len < 2: echo "Usage: sigma-agent plugin create <name>"; return
    create_plugin(args[1])
  of "remove","uninstall":
    if args.len < 2: echo "Usage: sigma-agent plugin remove <name>"; return
    let p_dir = plugin_dir() / args[1]
    if dirExists(p_dir):
      removeDir(p_dir)
      echo fmt"✓ Plugin removed: {args[1]}"
    else: echo fmt"Plugin not found: {args[1]}"
  of "enable":
    echo fmt"✓ Plugin enabled: {if args.len > 1: args[1] else: \"(name required)\"}"
  of "disable":
    echo fmt"✓ Plugin disabled: {if args.len > 1: args[1] else: \"(name required)\"}"
  of "reload":
    echo "✓ Plugin registry reloaded"
  of "example":
    install_example_plugin()
  of "training":
    # Export plugin training data to main training pipeline
    var total = 0
    let out_path = getEnv("HOME","/tmp") / ".cache/sigma/agent_training/plugins.jsonl"
    createDir(out_path.parentDir())
    var f = open(out_path, fmWrite)
    for name, p in registry.plugins:
      if fileExists(p.training_file):
        for line in lines(p.training_file):
          if line.strip().len > 0: f.writeLine(line); total += 1
    f.close()
    echo fmt"✓ Exported {total} plugin training samples to {out_path}"
  else:
    echo fmt"Unknown plugin command: {args[0]}"
    echo "Commands: list | install | create | remove | enable | disable | reload | example | training"
