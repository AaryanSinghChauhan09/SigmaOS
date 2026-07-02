# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_watch.nim — File watcher + proactive AI suggestions
# Inspiration: Aider --watch-files, Hermes IDE file events, Claude Code context
# Language: Nim — OOP via FileWatcher + EventHandler

import std/[os, osproc, times, tables, strformat, strutils, hashes]

# ── Watch Event ───────────────────────────────────────────────────────────────
type
  WatchEventKind = enum Created, Modified, Deleted, Renamed

  WatchEvent = object
    kind:     WatchEventKind
    path:     string
    old_path: string   # for renames
    size:     int64
    mtime:    Time

  WatchConfig = object
    dirs:        seq[string]
    extensions:  seq[string]   # empty = all
    recursive:   bool
    interval_ms: int
    auto_suggest: bool         # call sigma-ai for suggestions on change
    ignore_dirs: seq[string]

# ── File State Tracker ────────────────────────────────────────────────────────
type FileState = object
  mtime: Time
  size:  int64

type FileWatcher = object
  config:  WatchConfig
  state:   Table[string, FileState]
  events:  seq[WatchEvent]

proc new_watcher(cfg: WatchConfig): FileWatcher =
  FileWatcher(config: cfg, state: initTable[string, FileState](), events: @[])

proc should_watch(w: FileWatcher, path: string): bool =
  # Skip ignored dirs
  for d in w.config.ignore_dirs:
    if path.contains(d): return false
  # Extension filter
  if w.config.extensions.len == 0: return true
  for ext in w.config.extensions:
    if path.endsWith(ext): return true
  false

proc scan(w: var FileWatcher, dir: string) =
  for kind, path in walkDir(dir):
    if not w.should_watch(path): continue
    if kind == pcDir and w.config.recursive:
      w.scan(path); continue
    if kind != pcFile: continue
    try:
      let info = getFileInfo(path)
      let cur  = FileState(mtime: info.lastWriteTime, size: info.size)
      if path in w.state:
        let prev = w.state[path]
        if cur.mtime != prev.mtime or cur.size != prev.size:
          w.events.add WatchEvent(kind: Modified, path: path, size: cur.size, mtime: cur.mtime)
      else:
        if w.state.len > 0:  # not first scan
          w.events.add WatchEvent(kind: Created, path: path, size: cur.size, mtime: cur.mtime)
      w.state[path] = cur
    except: discard

  # Detect deletions
  var deleted: seq[string]
  for path in w.state.keys:
    if not fileExists(path) and path.startsWith(dir):
      deleted.add(path)
      w.events.add WatchEvent(kind: Deleted, path: path)
  for p in deleted: w.state.del(p)

proc poll(w: var FileWatcher): seq[WatchEvent] =
  w.events.setLen(0)
  for dir in w.config.dirs: w.scan(dir)
  w.events

# ── AI Suggestions on file change ────────────────────────────────────────────
proc suggest_for_change(event: WatchEvent): string =
  case event.kind
  of Created:
    let ext = event.path.splitFile.ext
    return fmt"sigma-agent \"explain what I should do with the new {ext} file {event.path.extractFilename}\""
  of Modified:
    if event.path.endsWith(".rs") or event.path.endsWith(".nim"):
      return fmt"sigma-agent \"review {event.path} for issues\""
    elif event.path.endsWith(".md"):
      return fmt"sigma-agent \"summarise {event.path}\""
    else:
      return fmt"sigma-agent \"read {event.path}\""
  of Deleted:
    return fmt"File deleted: {event.path}"
  of Renamed:
    return fmt"File renamed: {event.old_path} → {event.path}"

# ── Watcher CLI ───────────────────────────────────────────────────────────────
const ANSI_CYAN  = "\e[38;2;69;243;255m"
const ANSI_GREEN = "\e[38;2;52;211;153m"
const ANSI_RESET = "\e[0m"

proc watch_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "help":
    echo """sigma-agent watch — Proactive AI file monitoring

Usage:
  sigma-agent watch [dir] [options]

Options:
  --ext <.rs,.nim>     Watch only these extensions (comma-separated)
  --recursive          Watch subdirectories (default: on)
  --suggest            Auto-run AI suggestions on changes
  --interval <ms>      Poll interval (default: 1000ms)
  --ignore <dir>       Ignore directory (repeatable)

Examples:
  sigma-agent watch .                      Watch current directory
  sigma-agent watch /home/user/code        Watch code directory
  sigma-agent watch . --ext .rs,.nim       Watch only Rust and Nim files
  sigma-agent watch . --suggest            Watch + auto-suggest on changes
"""
    return

  var cfg = WatchConfig(
    dirs:        @[if args[0] != "--" : args[0] else: "."],
    extensions:  @[],
    recursive:   true,
    interval_ms: 1000,
    auto_suggest: false,
    ignore_dirs: @[".git", "node_modules", "target", ".cache"],
  )

  var i = 1
  while i < args.len:
    case args[i]
    of "--ext":
      i.inc
      if i < args.len: cfg.extensions = args[i].split(',').mapIt(it.strip())
    of "--interval":
      i.inc
      if i < args.len: cfg.interval_ms = parseInt(args[i])
    of "--suggest":   cfg.auto_suggest = true
    of "--no-recursive": cfg.recursive = false
    of "--ignore":
      i.inc
      if i < args.len: cfg.ignore_dirs.add(args[i])
    else: discard
    i.inc

  if cfg.dirs.len == 0 or not dirExists(cfg.dirs[0]):
    cfg.dirs = @[getCurrentDir()]

  var watcher = new_watcher(cfg)
  # Initial scan (no events)
  for dir in cfg.dirs: watcher.scan(dir)

  echo ANSI_CYAN & fmt"σ Watching {cfg.dirs.join(', ')} (Ctrl+C to stop)" & ANSI_RESET
  if cfg.extensions.len > 0: echo fmt"  Extensions: {cfg.extensions.join(', ')}"
  if cfg.auto_suggest: echo "  Auto-suggest: enabled"
  echo ""

  while true:
    sleep(cfg.interval_ms)
    let events = watcher.poll()
    for ev in events:
      let icon = case ev.kind
        of Created:  "+"
        of Modified: "~"
        of Deleted:  "-"
        of Renamed:  "→"
      let color = case ev.kind
        of Created:  ANSI_GREEN
        of Modified: ANSI_CYAN
        else:        "\e[38;2;107;114;128m"
      echo fmt"{color}[{icon}] {ev.path.extractFilename:<30} {$ev.kind}" & ANSI_RESET
      if cfg.auto_suggest:
        let suggestion = suggest_for_change(ev)
        echo fmt"  → {suggestion}"
        let (out, _) = execCmdEx(suggestion)
        if out.strip().len > 0: echo "  " & out.strip().replace("\n", "\n  ")
