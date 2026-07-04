# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
# userland/coreutils/sigma_coreutils.nim — Sovereign coreutils (busybox-style)
# Language: Nim — single binary, OOP via object + methods, no third-party

import std/[os, strutils, parseopt, posix, times]

# ── Utility Trait (OOP via method dispatch on a string) ──────────────────────

type CoreUtil = object
  name: string

proc dispatch(name: string, args: seq[string]): int

# ── ls ────────────────────────────────────────────────────────────────────────

proc cmd_ls(args: seq[string]): int =
  let path = if args.len > 0: args[0] else: "."
  if not dirExists(path):
    stderr.writeLine("ls: cannot access '" & path & "': No such file or directory")
    return 1
  for kind, entry in walkDir(path):
    let name = entry.extractFilename
    let prefix = case kind
      of pcDir:           "\e[34m"  # blue
      of pcFile:          "\e[0m"   # default
      of pcLinkToDir:     "\e[36m"  # cyan
      of pcLinkToFile:    "\e[36m"
    echo prefix & name & "\e[0m"
  return 0

# ── cat ───────────────────────────────────────────────────────────────────────

proc cmd_cat(args: seq[string]): int =
  if args.len == 0:
    # Read from stdin
    for line in stdin.lines: echo line
    return 0
  for path in args:
    if not fileExists(path):
      stderr.writeLine("cat: " & path & ": No such file or directory")
      result = 1; continue
    for line in lines(path): echo line
  return result

# ── cp ────────────────────────────────────────────────────────────────────────

proc cmd_cp(args: seq[string]): int =
  if args.len < 2:
    stderr.writeLine("cp: missing file operand"); return 1
  let src = args[0]; let dst = args[1]
  if not fileExists(src):
    stderr.writeLine("cp: cannot stat '" & src & "': No such file"); return 1
  copyFile(src, dst)
  return 0

# ── mv ────────────────────────────────────────────────────────────────────────

proc cmd_mv(args: seq[string]): int =
  if args.len < 2:
    stderr.writeLine("mv: missing operand"); return 1
  moveFile(args[0], args[1]); return 0

# ── rm ────────────────────────────────────────────────────────────────────────

proc cmd_rm(args: seq[string]): int =
  var recursive = false; var files: seq[string]
  for a in args:
    if a == "-r" or a == "-rf": recursive = true
    else: files.add(a)
  for f in files:
    if recursive and dirExists(f):  removeDir(f)
    elif fileExists(f):             removeFile(f)
    else: stderr.writeLine("rm: cannot remove '" & f & "': No such file")
  return 0

# ── mkdir ─────────────────────────────────────────────────────────────────────

proc cmd_mkdir(args: seq[string]): int =
  var parents = false; var dirs: seq[string]
  for a in args:
    if a == "-p": parents = true
    else: dirs.add(a)
  for d in dirs:
    if parents: createDir(d)
    else:
      if dirExists(d): stderr.writeLine("mkdir: cannot create '" & d & "': exists")
      else: createDir(d)
  return 0

# ── pwd ───────────────────────────────────────────────────────────────────────

proc cmd_pwd(args: seq[string]): int =
  echo getCurrentDir(); return 0

# ── echo ──────────────────────────────────────────────────────────────────────

proc cmd_echo(args: seq[string]): int =
  var no_nl = false; var words: seq[string]
  for a in args:
    if a == "-n": no_nl = true
    else: words.add(a)
  let line = words.join(" ")
  if no_nl: stdout.write(line)
  else:     echo line
  return 0

# ── head ──────────────────────────────────────────────────────────────────────

proc cmd_head(args: seq[string]): int =
  var n = 10; var files: seq[string]
  var i = 0
  while i < args.len:
    if args[i] == "-n" and i + 1 < args.len:
      n = parseInt(args[i+1]); i += 2
    else: files.add(args[i]); i += 1
  let paths = if files.len == 0: @["-"] else: files
  for path in paths:
    var count = 0
    if path == "-":
      for line in stdin.lines:
        if count >= n: break
        echo line; count += 1
    else:
      for line in lines(path):
        if count >= n: break
        echo line; count += 1
  return 0

# ── tail ──────────────────────────────────────────────────────────────────────

proc cmd_tail(args: seq[string]): int =
  var n = 10; var files: seq[string]
  var i = 0
  while i < args.len:
    if args[i] == "-n" and i + 1 < args.len:
      n = parseInt(args[i+1]); i += 2
    else: files.add(args[i]); i += 1
  let paths = if files.len == 0: @["-"] else: files
  for path in paths:
    var buf: seq[string]
    let src = if path == "-": stdin else: open(path)
    for line in src.lines: buf.add(line)
    if path != "-": src.close()
    let start = max(0, buf.len - n)
    for j in start..<buf.len: echo buf[j]
  return 0

# ── wc ────────────────────────────────────────────────────────────────────────

proc cmd_wc(args: seq[string]): int =
  var files: seq[string]
  var lines_only, words_only, bytes_only = false
  for a in args:
    case a
    of "-l": lines_only = true
    of "-w": words_only = true
    of "-c": bytes_only = true
    else: files.add(a)
  let paths = if files.len == 0: @["-"] else: files
  for path in paths:
    var lc, wc, bc = 0
    let src = if path == "-": stdin else: open(path)
    for line in src.lines:
      lc += 1
      wc += line.splitWhitespace.len
      bc += line.len + 1
    if path != "-": src.close()
    if lines_only: echo lc
    elif words_only: echo wc
    elif bytes_only: echo bc
    else: echo lc, " ", wc, " ", bc, " ", path
  return 0

# ── chmod ─────────────────────────────────────────────────────────────────────

proc cmd_chmod(args: seq[string]): int =
  if args.len < 2:
    stderr.writeLine("chmod: missing operand"); return 1
  let mode_str = args[0]
  let mode = parseOctInt(mode_str)
  for path in args[1..^1]:
    setFilePermissions(path, cast[set[FilePermission]](mode))
  return 0

# ── touch ─────────────────────────────────────────────────────────────────────

proc cmd_touch(args: seq[string]): int =
  for f in args:
    if not fileExists(f): writeFile(f, "")
    # Update mtime — simplified: just re-write
  return 0

# ── grep ──────────────────────────────────────────────────────────────────────

proc cmd_grep(args: seq[string]): int =
  if args.len < 1: stderr.writeLine("grep: missing pattern"); return 1
  var pattern = ""; var files: seq[string]; var ignore_case = false
  var i = 0
  while i < args.len:
    case args[i]
    of "-i": ignore_case = true
    of "-n": discard
    else:
      if pattern.len == 0: pattern = args[i]
      else: files.add(args[i])
    i += 1
  let search = if ignore_case: pattern.toLowerAscii else: pattern
  let paths = if files.len == 0: @["-"] else: files
  var found = false
  for path in paths:
    var lineno = 0
    let src = if path == "-": stdin else: open(path)
    for line in src.lines:
      lineno += 1
      let hay = if ignore_case: line.toLowerAscii else: line
      if hay.contains(search):
        if files.len > 1: stdout.write(path & ":")
        echo lineno, ":", line
        found = true
    if path != "-": src.close()
  return if found: 0 else: 1

# ── Dispatch ──────────────────────────────────────────────────────────────────

proc dispatch(name: string, args: seq[string]): int =
  case name
  of "ls":    return cmd_ls(args)
  of "cat":   return cmd_cat(args)
  of "cp":    return cmd_cp(args)
  of "mv":    return cmd_mv(args)
  of "rm":    return cmd_rm(args)
  of "mkdir": return cmd_mkdir(args)
  of "pwd":   return cmd_pwd(args)
  of "echo":  return cmd_echo(args)
  of "head":  return cmd_head(args)
  of "tail":  return cmd_tail(args)
  of "wc":    return cmd_wc(args)
  of "chmod": return cmd_chmod(args)
  of "touch": return cmd_touch(args)
  of "grep":  return cmd_grep(args)
  else:
    stderr.writeLine("sigma-coreutils: unknown command: " & name)
    return 127

proc main() =
  let argv = commandLineParams()
  # When run as "sigma-coreutils <cmd> [args]"
  # or symlinked as the command name itself
  let prog = getAppFilename().extractFilename
  if prog == "sigma-coreutils" or prog == "sigma-coreutils.exe":
    if argv.len == 0:
      echo "sigma-coreutils: ls cat cp mv rm mkdir pwd echo head tail wc chmod touch grep"
      quit(0)
    quit(dispatch(argv[0], argv[1..^1]))
  else:
    quit(dispatch(prog, argv))

main()
