# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_memory.nim — Persistent long-term memory
# Stores facts, preferences, and learned patterns across sessions.
#
# Inspiration:
#   Claude Code memory blocks (CLAUDE.md project memory)
#   Aider /remember command
#   chatgpt-cli conversation persistence
#   j178/chatgpt session replay
#
# Memory types:
#   facts:       user-stated facts ("my project is in ~/code/myapp")
#   preferences: learned preferences ("user prefers dark mode")
#   patterns:    recurring command patterns ("user always installs rust tools")
#   corrections: things the agent got wrong before
#   project:     per-directory .sigma_memory files (like CLAUDE.md)
#
# Language: Nim (stdlib only)

import std/[os, json, times, strutils, strformat, tables, sequtils, algorithm]

# ── Types ─────────────────────────────────────────────────────────────────────
type
  MemoryKind = enum MkFact, MkPref, MkPattern, MkCorrection, MkProject

  MemoryEntry = object
    id:        string
    kind:      MemoryKind
    content:   string
    context:   string     # cwd or project when added
    ts:        string
    access_count: int
    last_access:  string
    tags:      seq[string]
    pinned:    bool

# ── Paths ─────────────────────────────────────────────────────────────────────
proc global_memory_path(): string =
  getEnv("HOME", "/tmp") / ".config/sigma/agent/memory.json"

proc project_memory_path(cwd: string): string =
  ## Per-project memory file (like CLAUDE.md)
  cwd / ".sigma_memory"

# ── Serialisation ──────────────────────────────────────────────────────────────
proc to_json(e: MemoryEntry): JsonNode =
  %*{"id":e.id, "kind":$e.kind, "content":e.content, "context":e.context,
     "ts":e.ts, "access_count":e.access_count, "last_access":e.last_access,
     "tags":e.tags, "pinned":e.pinned}

proc from_json(j: JsonNode): MemoryEntry =
  MemoryEntry(
    id:           j.getOrDefault("id").getStr($now().toTime.toUnix),
    kind:         case j.getOrDefault("kind").getStr
                  of "MkFact":       MkFact
                  of "MkPref":       MkPref
                  of "MkPattern":    MkPattern
                  of "MkCorrection": MkCorrection
                  else:              MkProject,
    content:      j.getOrDefault("content").getStr,
    context:      j.getOrDefault("context").getStr,
    ts:           j.getOrDefault("ts").getStr($now()),
    access_count: j.getOrDefault("access_count").getInt(0),
    last_access:  j.getOrDefault("last_access").getStr,
    tags:         j.getOrDefault("tags").getElems.mapIt(it.getStr),
    pinned:       j.getOrDefault("pinned").getBool(false))

# ── Storage ────────────────────────────────────────────────────────────────────
proc load_memories(): seq[MemoryEntry] =
  let path = global_memory_path()
  if not fileExists(path): return @[]
  try:
    let j = parseJson(readFile(path))
    if j.kind == JArray:
      for item in j: result.add from_json(item)
  except: discard

proc save_memories(entries: seq[MemoryEntry]) =
  createDir(global_memory_path().parentDir())
  var arr = newJArray()
  for e in entries: arr.add(e.to_json())
  writeFile(global_memory_path(), arr.pretty())

proc load_project_memory(cwd = getCurrentDir()): string =
  ## Read project .sigma_memory file
  let path = project_memory_path(cwd)
  if fileExists(path): readFile(path).strip()
  else: ""

proc save_project_memory(content: string, cwd = getCurrentDir()) =
  writeFile(project_memory_path(cwd), content)

# ── Memory operations ─────────────────────────────────────────────────────────
proc remember*(content: string, kind = MkFact,
               tags: seq[string] = @[]): MemoryEntry =
  var entries = load_memories()
  # Check for duplicate
  for e in entries:
    if e.content.toLowerAscii == content.toLowerAscii:
      echo fmt"(Already in memory: {content[0..<min(50,content.len)]})"
      return e
  let entry = MemoryEntry(
    id:           $now().toTime.toUnix & "_" & $content.hash.abs,
    kind:         kind,
    content:      content,
    context:      getCurrentDir(),
    ts:           $now(),
    access_count: 0,
    last_access:  $now(),
    tags:         tags,
    pinned:       false)
  entries.add(entry)
  save_memories(entries)
  result = entry

proc forget*(query: string): int =
  ## Remove memories matching query. Returns count removed.
  var entries = load_memories()
  let before = entries.len
  entries = entries.filterIt(
    not (query.toLowerAscii in it.content.toLowerAscii) or it.pinned)
  save_memories(entries)
  before - entries.len

proc recall*(query: string = "", top_n = 5, kind: MemoryKind = MkFact,
             all_kinds = true): seq[MemoryEntry] =
  ## Retrieve most relevant memories
  var entries = load_memories()

  # Also include project memory
  let proj = load_project_memory()
  if proj.len > 0:
    entries.insert(MemoryEntry(
      id: "project", kind: MkProject, content: proj,
      context: getCurrentDir(), ts: "project", pinned: true), 0)

  # Filter by kind
  if not all_kinds:
    entries = entries.filterIt(it.kind == kind)

  # Filter by query
  if query.len > 0:
    let lower = query.toLowerAscii
    entries = entries.filterIt(lower in it.content.toLowerAscii or
                                it.tags.anyIt(lower in it.toLowerAscii))

  # Sort: pinned first, then by access_count, then recency
  entries.sort(proc(a,b:MemoryEntry):int =
    if a.pinned and not b.pinned: return -1
    if b.pinned and not a.pinned: return 1
    b.access_count - a.access_count)

  # Update access counts for returned entries
  var all_entries = load_memories()
  for returned in entries[0..<min(top_n, entries.len)]:
    for i in 0..<all_entries.len:
      if all_entries[i].id == returned.id:
        all_entries[i].access_count += 1
        all_entries[i].last_access = $now()
  save_memories(all_entries)

  entries[0..<min(top_n, entries.len)]

proc build_context_string*(query: string = "", max_tokens = 200): string =
  ## Build a compact memory string for injecting into LLM prompts
  let memories = recall(query, top_n=5)
  if memories.len == 0: return ""
  var parts: seq[string]
  for m in memories:
    let kind_tag = case m.kind
      of MkFact:       "fact"
      of MkPref:       "preference"
      of MkPattern:    "pattern"
      of MkCorrection: "correction"
      of MkProject:    "project"
    parts.add(fmt"[{kind_tag}] {m.content}")
  let result = "User memory: " & parts.join(" | ")
  result[0..<min(max_tokens * 4, result.len)]

proc pin*(query: string): int =
  var entries = load_memories()
  var count = 0
  for i in 0..<entries.len:
    if query.toLowerAscii in entries[i].content.toLowerAscii:
      entries[i].pinned = true; count += 1
  if count > 0: save_memories(entries)
  count

# ── Project memory (.sigma_memory file) ──────────────────────────────────────
proc init_project_memory*(cwd = getCurrentDir()) =
  let path = project_memory_path(cwd)
  if fileExists(path):
    echo fmt"Project memory already exists: {path}"
    return
  let template_content = fmt"""# sigma-agent project memory
# This file is read by sigma-agent for project-specific context.
# Edit freely — sigma-agent uses this to give better answers.

## Project
Name: {cwd.extractFilename}
Language: (e.g. Rust, Nim, Python)
Build: (e.g. cargo build, nimble build)
Test: (e.g. cargo test, nimble test)

## Key files
Main entry: (e.g. src/main.rs)
Config: (e.g. config.toml)

## Notes
(Add any project-specific notes here)
"""
  writeFile(path, template_content)
  echo fmt"✓ Created project memory: {path}"
  echo "  Edit this file to give sigma-agent project context."

# ── CLI ────────────────────────────────────────────────────────────────────────
proc memory_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "help":
    echo """sigma-agent memory — Persistent long-term memory

Usage:
  sigma-agent memory list                  Show all memories
  sigma-agent memory list <query>          Search memories
  sigma-agent memory add "fact"            Add a fact
  sigma-agent memory add "pref" --pref     Add a preference
  sigma-agent memory add "pat" --pattern   Add a pattern
  sigma-agent memory forget <query>        Remove matching memories
  sigma-agent memory pin <query>           Pin (protect from forget)
  sigma-agent memory clear                 Clear all non-pinned memories

  sigma-agent memory project               Show project .sigma_memory
  sigma-agent memory project init          Create .sigma_memory template
  sigma-agent memory project show          Display project memory content

Examples:
  sigma-agent memory add "my code is in ~/code/myapp"
  sigma-agent memory add "I prefer dark mode" --pref
  sigma-agent memory add "always use cargo check before cargo build" --pattern
  sigma-agent memory list
  sigma-agent memory forget "dark mode"
  sigma-agent memory project init

How it works:
  - Memories are automatically included in every sigma-agent prompt
  - Project .sigma_memory files work like CLAUDE.md (project-specific context)
  - Use 'pin' to protect important memories from being removed
  - Run 'sigma-agent memory list' to see what the agent knows about you
"""
    return

  case args[0].toLowerAscii
  of "list","show","ls":
    let query = if args.len > 1: args[1..^1].join(" ") else: ""
    let entries = recall(query, top_n=20, all_kinds=true)
    let proj = load_project_memory()

    echo "\e[38;2;69;243;255m\e[1mΣ sigma-agent memory\e[0m"
    if proj.len > 0:
      echo fmt"\n  \e[38;2;168;85;247m[project memory]\e[0m  .sigma_memory\n{proj[0..<min(200,proj.len)]}...\n"

    if entries.len == 0:
      echo "  No memories yet. Add with: sigma-agent memory add \"your fact\""
      return
    for e in entries:
      let kind_color = case e.kind
        of MkFact:       "\e[38;2;52;211;153m"
        of MkPref:       "\e[38;2;69;243;255m"
        of MkPattern:    "\e[38;2;168;85;247m"
        of MkCorrection: "\e[38;2;251;191;36m"
        of MkProject:    "\e[38;2;107;114;128m"
      let pin_mark = if e.pinned: " 📌" else: ""
      echo fmt"  {kind_color}{$e.kind:<14}\e[0m {e.content[0..<min(70,e.content.len)]}{pin_mark}"
      if args.contains("--verbose") and e.tags.len > 0:
        echo fmt"    tags: {e.tags.join(\", \")}"

  of "add","remember","save":
    if args.len < 2: echo "Usage: sigma-agent memory add \"content\""; return
    let content = args[1..^1].filterIt(not it.startsWith("-")).join(" ")
    let kind = if "--pref" in args or "--preference" in args:   MkPref
               elif "--pattern" in args:                         MkPattern
               elif "--correction" in args or "--fix" in args:  MkCorrection
               else:                                             MkFact
    let tags = args.filterIt(it.startsWith("--tag:")).mapIt(it[6..^1])
    let entry = remember(content, kind, tags)
    let kind_str = $entry.kind
    echo fmt"✓ Remembered [{kind_str}]: {content[0..<min(60,content.len)]}"

  of "forget","remove","delete":
    if args.len < 2: echo "Usage: sigma-agent memory forget <query>"; return
    let query = args[1..^1].join(" ")
    let count = forget(query)
    if count > 0: echo fmt"✓ Forgot {count} memories matching: {query}"
    else: echo fmt"No memories found matching: {query}"

  of "pin":
    if args.len < 2: echo "Usage: sigma-agent memory pin <query>"; return
    let count = pin(args[1..^1].join(" "))
    echo fmt"✓ Pinned {count} memories"

  of "clear","reset":
    echo "This will delete all non-pinned memories. Are you sure? (yes/no)"
    let confirm = try: stdin.readLine().strip().toLowerAscii except: ""
    if confirm == "yes":
      let entries = load_memories().filterIt(it.pinned)
      save_memories(entries)
      echo fmt"✓ Cleared memories (kept {entries.len} pinned)"
    else: echo "Cancelled"

  of "project":
    let sub = if args.len > 1: args[1].toLowerAscii else: "show"
    case sub
    of "init","create","new": init_project_memory()
    of "show","cat","read":
      let content = load_project_memory()
      if content.len > 0: echo content
      else: echo "(No .sigma_memory in current directory. Run: sigma-agent memory project init)"
    else:
      let content = load_project_memory()
      if content.len > 0: echo content
      else: init_project_memory()

  of "context","debug":
    let ctx = build_context_string("", max_tokens=300)
    echo fmt"Memory context string ({ctx.len} chars):\n{ctx}"

  else:
    echo fmt"Unknown memory command: {args[0]}"
    echo "Run: sigma-agent memory help"
