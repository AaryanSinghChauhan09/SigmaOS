# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/shell/sigma_shell.nim — sigma-sh: Sovereign Shell
# Replaces: sigma_shell.cpp (C++ stub, removed)
#
# Language: Nim — compiles to native, ergonomic, no GC in kernel paths
# Pattern: OOP via object types + methods

import std/[strutils, os, parseopt, sequtils]

# ── Types ─────────────────────────────────────────────────────────────────────

type
  ShellState = object
    cwd:     string
    env:     seq[(string, string)]
    history: seq[string]
    running: bool

  ParsedCmd = object
    args:   seq[string]
    stdin_redir:  string
    stdout_redir: string
    append_redir: bool
    pipe_next:    bool

  ShellError = enum
    ErrNotFound   = "command not found"
    ErrPermDenied = "permission denied"
    ErrNoFile     = "no such file or directory"

# ── Shell State ───────────────────────────────────────────────────────────────

proc newShell(): ShellState =
  result.cwd     = getEnv("HOME", "/")
  result.running = true
  result.history = @[]
  result.env     = @[
    ("PATH",    "/usr/bin:/bin"),
    ("HOME",    "/home/sovereign"),
    ("SHELL",   "/bin/sigma-sh"),
    ("TERM",    "xterm-256color"),
    ("USER",    "sovereign"),
  ]

proc getEnvVar(sh: ShellState, key: string): string =
  for (k, v) in sh.env:
    if k == key: return v
  return ""

proc setEnvVar(sh: var ShellState, key, val: string) =
  for i in 0..<sh.env.len:
    if sh.env[i][0] == key:
      sh.env[i] = (key, val)
      return
  sh.env.add((key, val))

# ── Token Parser ──────────────────────────────────────────────────────────────

proc tokenise(line: string): seq[string] =
  ## Split line into tokens respecting single and double quotes
  var tokens: seq[string] = @[]
  var cur = ""
  var inSingle = false
  var inDouble = false

  for ch in line:
    case ch
    of '\'':
      if not inDouble: inSingle = not inSingle
      else: cur.add(ch)
    of '"':
      if not inSingle: inDouble = not inDouble
      else: cur.add(ch)
    of ' ', '\t':
      if inSingle or inDouble:
        cur.add(ch)
      elif cur.len > 0:
        tokens.add(cur); cur = ""
    else:
      cur.add(ch)

  if cur.len > 0: tokens.add(cur)
  return tokens

proc parseCmd(tokens: seq[string]): ParsedCmd =
  result.args        = @[]
  result.pipe_next   = false
  result.append_redir = false

  var i = 0
  while i < tokens.len:
    case tokens[i]
    of ">":
      i.inc
      if i < tokens.len: result.stdout_redir = tokens[i]
    of ">>":
      i.inc
      if i < tokens.len:
        result.stdout_redir = tokens[i]
        result.append_redir = true
    of "<":
      i.inc
      if i < tokens.len: result.stdin_redir = tokens[i]
    of "|":
      result.pipe_next = true
    else:
      result.args.add(tokens[i])
    i.inc

# ── Built-in Commands ─────────────────────────────────────────────────────────

proc builtin_cd(sh: var ShellState, args: seq[string]): int =
  let target = if args.len > 1: args[1] else: sh.getEnvVar("HOME")
  if dirExists(target):
    sh.cwd = target
    setCurrentDir(target)
    return 0
  else:
    stderr.writeLine("sigma-sh: cd: " & target & ": " & $ErrNoFile)
    return 1

proc builtin_echo(args: seq[string]): int =
  echo args[1..^1].join(" ")
  return 0

proc builtin_export(sh: var ShellState, args: seq[string]): int =
  for arg in args[1..^1]:
    let parts = arg.split('=', maxsplit=1)
    if parts.len == 2:
      sh.setEnvVar(parts[0], parts[1])
    else:
      sh.setEnvVar(parts[0], "")
  return 0

proc builtin_pwd(sh: ShellState): int =
  echo sh.cwd
  return 0

proc builtin_exit(code: int): int =
  quit(code)

proc builtin_history(sh: ShellState): int =
  for i, cmd in sh.history:
    echo "  ", i + 1, "  ", cmd
  return 0

proc builtin_help(): int =
  echo """sigma-sh — Sovereign Shell v15.0
Builtins: cd, echo, export, pwd, exit, history, help
External commands loaded from PATH"""
  return 0

# ── External Command Execution ────────────────────────────────────────────────

proc execExternal(sh: ShellState, cmd: ParsedCmd): int =
  if cmd.args.len == 0: return 0
  let exe = cmd.args[0]

  # Search PATH
  var found = ""
  for dir in sh.getEnvVar("PATH").split(':'):
    let candidate = dir / exe
    if fileExists(candidate):
      found = candidate
      break

  if found == "" and not fileExists(exe):
    stderr.writeLine("sigma-sh: " & exe & ": " & $ErrNotFound)
    return 127

  let actual = if found != "": found else: exe
  try:
    return execShellCmd(actual & " " & cmd.args[1..^1].join(" "))
  except:
    stderr.writeLine("sigma-sh: exec error: " & getCurrentExceptionMsg())
    return 1

# ── Main REPL ─────────────────────────────────────────────────────────────────

proc evalLine(sh: var ShellState, line: string): int =
  let stripped = line.strip()
  if stripped.len == 0 or stripped.startsWith("#"): return 0
  sh.history.add(stripped)

  # Handle semicolon-separated commands
  for part in stripped.split(';'):
    let tokens = tokenise(part.strip())
    if tokens.len == 0: continue
    let cmd = parseCmd(tokens)
    if cmd.args.len == 0: continue

    let exit_code = case cmd.args[0]
      of "cd":      builtin_cd(sh, cmd.args)
      of "echo":    builtin_echo(cmd.args)
      of "export":  builtin_export(sh, cmd.args)
      of "pwd":     builtin_pwd(sh)
      of "exit":    builtin_exit(if cmd.args.len > 1: parseInt(cmd.args[1]) else: 0)
      of "history": builtin_history(sh)
      of "help":    builtin_help()
      else:         execExternal(sh, cmd)

    if exit_code != 0:
      result = exit_code

proc main() =
  var sh = newShell()
  let isInteractive = isatty(stdin.getFileHandle())

  if isInteractive:
    echo "sigma-sh v15.0 — Sovereign Shell. Type 'help' for commands."

  while sh.running:
    if isInteractive:
      stdout.write("sovereign@sigma:" & sh.cwd & "$ ")
      stdout.flushFile()

    let line = try: stdin.readLine() except EOFError: break
    discard evalLine(sh, line)

main()
