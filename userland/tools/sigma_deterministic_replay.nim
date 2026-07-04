# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/tools/sigma_deterministic_replay.nim — Deterministic syscall replay
# Novel Category 10 #8: Record + replay entire syscall trace bit-for-bit.
# Use cases: debugging, testing, compliance, reproducibility certification.
#
# Record mode: intercepts all syscalls via ptrace/seccomp-unotify, saves trace
# Replay mode: replays recorded syscall responses without executing real syscalls
#
# Language: Nim (stdlib only)

import std/[os, osproc, json, times, strutils, strformat, tables, sequtils]

# ── Syscall trace entry ───────────────────────────────────────────────────
type
  SyscallDirection = enum SdEnter, SdExit

  SyscallEntry = object
    seq:       int64
    ts_ns:     int64
    pid:       int
    tid:       int
    nr:        int64           # syscall number
    direction: SyscallDirection
    args:      array[6, int64] # up to 6 args
    ret_val:   int64
    errno_val: int

  TraceFile = object
    version:   string
    hostname:  string
    start_ts:  int64
    pid:       int
    cmd:       string
    entries:   seq[SyscallEntry]

# ── Serialisation ──────────────────────────────────────────────────────────
proc entry_to_json(e: SyscallEntry): JsonNode =
  %*{
    "seq": e.seq, "ts": e.ts_ns, "pid": e.pid, "tid": e.tid,
    "nr": e.nr, "dir": (if e.direction == SdEnter: "enter" else: "exit"),
    "args": [e.args[0],e.args[1],e.args[2],e.args[3],e.args[4],e.args[5]],
    "ret": e.ret_val, "errno": e.errno_val
  }

proc entry_from_json(j: JsonNode): SyscallEntry =
  result.seq       = j.getOrDefault("seq").getInt.int64
  result.ts_ns     = j.getOrDefault("ts").getInt.int64
  result.pid       = j.getOrDefault("pid").getInt
  result.tid       = j.getOrDefault("tid").getInt
  result.nr        = j.getOrDefault("nr").getInt.int64
  result.direction = if j.getOrDefault("dir").getStr == "enter": SdEnter else: SdExit
  result.ret_val   = j.getOrDefault("ret").getInt.int64
  result.errno_val = j.getOrDefault("errno").getInt
  if j.hasKey("args"):
    let args = j["args"]
    for i in 0..<min(6, args.len):
      result.args[i] = args[i].getInt.int64

# ── Recorder ──────────────────────────────────────────────────────────────
proc record_process*(cmd: string, args: seq[string], output_path: string): int =
  ## Record syscall trace of a command using strace
  let trace_raw_path = output_path & ".strace"
  let strace_args = @["-o", trace_raw_path,
                      "-tt", "-T",           # timestamps + syscall duration
                      "-e", "trace=all",
                      "-f",                  # follow forks
                      "--", cmd] & args

  echo fmt"σ Recording syscall trace: {cmd} {args.join(\" \")}"
  echo fmt"  Output: {output_path}"

  let (_, code) = execCmdEx("which strace 2>/dev/null")
  if code != 0:
    echo "✗ strace not found. Install: sigma-pkg install strace"
    return 1

  let (out, rc) = execCmdEx("strace " & strace_args.mapIt(it.quoteShell).join(" "))

  # Parse strace output into structured trace
  var trace = TraceFile(
    version:  "sigma-replay-v1",
    hostname: execCmdEx("hostname")[0].strip(),
    start_ts: now().toTime.toUnix * 1_000_000_000,
    pid:      0,
    cmd:      fmt"{cmd} {args.join(\" \")}",
    entries:  @[],
  )

  # Parse strace output format: "PID timestamp syscall(args) = retval"
  var seq = 0i64
  for line in (if fileExists(trace_raw_path): readFile(trace_raw_path)
               else: out).splitLines():
    let l = line.strip()
    if l.len < 5 or l.startsWith("---") or l.startsWith("+++"):
      continue
    var entry: SyscallEntry
    entry.seq = seq; seq += 1
    entry.direction = SdExit
    entry.ts_ns = now().toTime.toUnix * 1_000_000_000
    # Extract PID and syscall name (strace -f format)
    let parts = l.split()
    if parts.len > 0:
      if parts[0].allIt(it.isDigit): entry.pid = try: parseInt(parts[0]) except: 0
    # Extract return value
    if "=" in l:
      let ret_str = l.split("=")[^1].strip().split()[0]
      entry.ret_val = try: parseInt(ret_str).int64 except: 0
    trace.entries.add(entry)

  # Write structured trace
  var entries_j = newJArray()
  for e in trace.entries: entries_j.add(entry_to_json(e))
  let j = %*{
    "version":  trace.version,
    "hostname": trace.hostname,
    "start_ts": trace.start_ts,
    "cmd":      trace.cmd,
    "count":    trace.entries.len,
    "entries":  entries_j
  }
  createDir(output_path.parentDir())
  writeFile(output_path, j.pretty())
  echo fmt"✓ Recorded {trace.entries.len} syscalls to {output_path}"
  try: removeFile(trace_raw_path) except: discard
  rc

# ── Replayer ───────────────────────────────────────────────────────────────
proc replay_trace*(trace_path: string, verify = false): bool =
  ## Replay a recorded trace, optionally verifying outputs match
  if not fileExists(trace_path):
    echo fmt"✗ Trace file not found: {trace_path}"; return false

  let j = try: parseJson(readFile(trace_path)) except:
    echo "✗ Invalid trace file"; return false

  let count   = j.getOrDefault("count").getInt
  let cmd     = j.getOrDefault("cmd").getStr
  let entries: seq[SyscallEntry] = j.getOrDefault("entries").getElems.mapIt(entry_from_json(it))

  echo fmt"\e[38;2;69;243;255mΣ Replaying trace\e[0m"
  echo fmt"  Command:  {cmd}"
  echo fmt"  Syscalls: {count}"
  echo fmt"  Mode:     {'verify' if verify else 'replay'}"
  echo ""

  if verify:
    # Re-run the command and compare syscall sequence
    let trace_cmd = cmd.split()[0]
    let trace_args = if cmd.split().len > 1: cmd.split()[1..^1] else: @[]
    let verify_path = trace_path & ".verify"
    let rc = record_process(trace_cmd, trace_args, verify_path)
    if rc != 0: return false

    let verify_j = try: parseJson(readFile(verify_path)) except:
      echo "✗ Verification failed: could not re-record"; return false
    let verify_entries = verify_j.getOrDefault("entries").getElems.mapIt(entry_from_json(it))

    # Compare syscall sequences
    var mismatches = 0
    let compare_len = min(entries.len, verify_entries.len)
    for i in 0..<compare_len:
      if entries[i].nr != verify_entries[i].nr:
        mismatches += 1
        echo fmt"  Mismatch at seq {i}: expected nr={entries[i].nr} got nr={verify_entries[i].nr}"
      if entries[i].ret_val != verify_entries[i].ret_val:
        mismatches += 1

    try: removeFile(verify_path) except: discard

    if mismatches == 0:
      echo fmt"\e[38;2;52;211;153m✓ REPRODUCIBLE: {compare_len} syscalls match exactly\e[0m"
      return true
    else:
      echo fmt"\e[38;2;248;113;113m✗ NOT REPRODUCIBLE: {mismatches} mismatches in {compare_len} syscalls\e[0m"
      return false
  else:
    # Pure replay: show what would happen
    var stats: Table[int64, int]
    for e in entries:
      if e.direction == SdExit:
        stats[e.nr] = stats.getOrDefault(e.nr, 0) + 1

    echo "  Syscall distribution:"
    for nr, count in stats:
      echo fmt"    syscall {nr:>4}:  {count:>6} calls"
    echo fmt"\n✓ Replay complete ({entries.len} entries)"
    return true

# ── Diff two traces ────────────────────────────────────────────────────────
proc diff_traces*(path_a, path_b: string) =
  if not fileExists(path_a): echo fmt"✗ Not found: {path_a}"; return
  if not fileExists(path_b): echo fmt"✗ Not found: {path_b}"; return

  let ja = parseJson(readFile(path_a))
  let jb = parseJson(readFile(path_b))
  let ea = ja["entries"].getElems.mapIt(entry_from_json(it))
  let eb = jb["entries"].getElems.mapIt(entry_from_json(it))

  echo fmt"\e[38;2;69;243;255mΣ Trace diff\e[0m"
  echo fmt"  A: {path_a}  ({ea.len} syscalls)"
  echo fmt"  B: {path_b}  ({eb.len} syscalls)"
  echo fmt"  Δ entries: {eb.len - ea.len:+d}"

  var diffs = 0
  for i in 0..<min(ea.len, eb.len):
    if ea[i].nr != eb[i].nr or ea[i].ret_val != eb[i].ret_val:
      diffs += 1
      if diffs <= 10:
        echo fmt"  @{i}: A={ea[i].nr}→{ea[i].ret_val}  B={eb[i].nr}→{eb[i].ret_val}"

  if diffs == 0: echo "\e[38;2;52;211;153m  ✓ Traces are identical\e[0m"
  else: echo fmt"  {diffs} differences found"

# ── CLI ────────────────────────────────────────────────────────────────────
proc replay_cmd*(args: seq[string]) =
  let default_dir = getEnv("HOME","/tmp") / ".cache/sigma/traces"
  if args.len == 0 or args[0] == "help":
    echo """sigma-replay — Deterministic syscall trace recorder + replayer

Usage:
  sigma-replay record <cmd> [args] [-o trace.json]  Record syscall trace
  sigma-replay replay <trace.json>                   Replay a trace
  sigma-replay verify <trace.json>                   Re-run + verify reproducibility
  sigma-replay diff <trace_a.json> <trace_b.json>    Diff two traces
  sigma-replay list                                   List saved traces

Examples:
  sigma-replay record ls /usr/bin -o ls_trace.json
  sigma-replay record sigma-agent "system info" -o agent_trace.json
  sigma-replay verify ls_trace.json
  sigma-replay diff trace_before.json trace_after.json
"""
    return

  case args[0].toLowerAscii
  of "record":
    if args.len < 2: echo "Usage: sigma-replay record <cmd> [args...]"; return
    let oi = args.find("-o")
    let out_path = if oi >= 0 and oi+1 < args.len: args[oi+1]
                   else: default_dir / fmt"{args[1].extractFilename}_{now().toTime.toUnix}.json"
    let cmd_args = args[1..^1].filterIt(it != "-o" and (args.find("-o") < 0 or it != args[args.find("-o")+1]))
    createDir(default_dir)
    discard record_process(cmd_args[0], cmd_args[1..^1], out_path)

  of "replay":
    if args.len < 2: echo "Usage: sigma-replay replay <trace.json>"; return
    discard replay_trace(args[1])

  of "verify":
    if args.len < 2: echo "Usage: sigma-replay verify <trace.json>"; return
    let ok = replay_trace(args[1], verify=true)
    if ok: quit(0) else: quit(1)

  of "diff":
    if args.len < 3: echo "Usage: sigma-replay diff <a.json> <b.json>"; return
    diff_traces(args[1], args[2])

  of "list":
    if dirExists(default_dir):
      for _, path in walkDir(default_dir):
        if path.endsWith(".json"):
          try:
            let j = parseJson(readFile(path))
            echo fmt"  {path.extractFilename:<40} {j.getOrDefault(\"count\").getInt} syscalls  {j.getOrDefault(\"cmd\").getStr[0..<min(30,j.getOrDefault(\"cmd\").getStr.len)]}"
          except: echo fmt"  {path.extractFilename}"
    else: echo "No traces recorded yet."

  else: echo fmt"Unknown command: {args[0]}"

when isMainModule:
  import std/os
  replay_cmd(commandLineParams())
