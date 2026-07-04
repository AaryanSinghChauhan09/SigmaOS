# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
# userland/tools/sigma_strace.nim — sigma-strace: Syscall Tracer
# Language: Nim — native, OOP via object + methods

import std/[os, strutils, parseopt, osproc, strformat, tables, streams]

# ── Syscall Table ─────────────────────────────────────────────────────────────
const SYSCALL_NAMES: array[33, string] = [
  "read","write","open","close","exit","fork","exec","wait",
  "mmap","munmap","brk","getpid","getppid","kill","signal",
  "stat","fstat","lseek","dup","dup2","pipe","chdir","getcwd",
  "mkdir","rmdir","unlink","rename","clock_gettime","nanosleep","ioctl",
  "sigma_pledge","sigma_unveil","sigma_attest"
]

# ── Trace Entry ───────────────────────────────────────────────────────────────
type
  TraceEntry = object
    pid:      int
    nr:       int
    args:     seq[uint64]
    ret:      int64
    dur_ns:   uint64
    name:     string

  Tracer = object
    pid:      int
    entries:  seq[TraceEntry]
    stats:    Table[int, (uint64, uint64)] # nr -> (count, total_ns)
    filter_nr: int  # -1 = all
    min_dur:   uint64

proc syscall_name(nr: int): string =
  if nr >= 0 and nr < SYSCALL_NAMES.len: SYSCALL_NAMES[nr]
  else: fmt"syscall_{nr}"

proc new_tracer(pid: int, filter_nr = -1, min_dur_ns: uint64 = 0): Tracer =
  Tracer(pid: pid, entries: @[], stats: initTable[int,(uint64,uint64)](),
         filter_nr: filter_nr, min_dur: min_dur_ns)

proc record(t: var Tracer, entry: TraceEntry) =
  if t.filter_nr >= 0 and entry.nr != t.filter_nr: return
  if entry.dur_ns < t.min_dur: return
  t.entries.add(entry)
  if entry.nr in t.stats:
    let (cnt, tot) = t.stats[entry.nr]
    t.stats[entry.nr] = (cnt+1, tot+entry.dur_ns)
  else:
    t.stats[entry.nr] = (1, entry.dur_ns)

proc format_entry(e: TraceEntry): string =
  let dur = if e.dur_ns > 1_000_000: fmt"{e.dur_ns div 1_000_000}ms"
            elif e.dur_ns > 1_000:   fmt"{e.dur_ns div 1_000}μs"
            else:                    fmt"{e.dur_ns}ns"
  let ret_str = if e.ret >= 0: fmt"{e.ret}" else: fmt"E{-e.ret}"
  fmt"[{e.pid}] {e.name}({e.args.mapIt($it).join(', ')}) = {ret_str} <{dur}>"

proc print_summary(t: Tracer) =
  echo "\n=== sigma-strace summary ==="
  echo fmt"{'Syscall':<20} {'Calls':>8} {'Total':>12} {'Avg':>10}"
  echo "-".repeat(52)
  var sorted = toSeq(t.stats.pairs)
  sorted.sort proc(a,b: (int,(uint64,uint64))): int =
    cmp(b[1][1], a[1][1])  # sort by total time desc
  for (nr, (cnt, tot)) in sorted:
    let avg = if cnt > 0: tot div cnt else: 0
    let tot_str = fmt"{tot div 1_000}μs"
    let avg_str = fmt"{avg div 1_000}μs"
    echo fmt"{syscall_name(nr):<20} {cnt:>8} {tot_str:>12} {avg_str:>10}"

# ── Linux ptrace-based tracing (simulated on non-Linux) ──────────────────────
when defined(linux):
  import std/posix

  proc trace_process(pid: int): Tracer =
    result = new_tracer(pid)
    # PTRACE_ATTACH
    let r = ptrace(PTRACE_ATTACH, Pid(pid), nil, nil)
    if r != 0: stderr.writeLine(fmt"sigma-strace: cannot attach to {pid}"); return
    var status: cint
    discard waitpid(Pid(pid), status, 0)
    # Set options for syscall tracing
    discard ptrace(PTRACE_SETOPTIONS, Pid(pid), nil, PTRACE_O_TRACESYSGOOD)
    while true:
      # Wait for next syscall entry
      if ptrace(PTRACE_SYSCALL, Pid(pid), nil, nil) != 0: break
      if waitpid(Pid(pid), status, 0) < 0: break
      if WIFEXITED(status): break
      # Read registers (simplified — architecture-dependent)
      var regs: UserRegs
      if ptrace(PTRACE_GETREGS, Pid(pid), nil, addr regs) != 0: continue
      let nr   = int(regs.orig_rax)
      let args = @[regs.rdi, regs.rsi, regs.rdx, regs.r10, regs.r8, regs.r9].mapIt(uint64(it))
      # Wait for syscall exit
      if ptrace(PTRACE_SYSCALL, Pid(pid), nil, nil) != 0: break
      if waitpid(Pid(pid), status, 0) < 0: break
      if WIFEXITED(status): break
      if ptrace(PTRACE_GETREGS, Pid(pid), nil, addr regs) != 0: continue
      let ret = int64(regs.rax)
      let e = TraceEntry(pid:pid, nr:nr, args:args, ret:ret, dur_ns:0, name:syscall_name(nr))
      result.record(e)
      echo format_entry(e)
    discard ptrace(PTRACE_DETACH, Pid(pid), nil, nil)

else:
  proc trace_process(pid: int): Tracer =
    result = new_tracer(pid)
    echo fmt"[sigma-strace] Attaching to PID {pid} (simulation mode)..."
    # Simulate a few syscall entries for demo
    for nr in 0..5:
      let e = TraceEntry(pid:pid, nr:nr, args: @[0'u64,4096'u64], ret:0,
                         dur_ns: uint64(nr * 100 + 50), name: syscall_name(nr))
      result.record(e)
      echo format_entry(e)

# ── Spawn + trace ─────────────────────────────────────────────────────────────
proc trace_cmd(args: seq[string]): Tracer =
  echo fmt"[sigma-strace] Running: {args.join(' ')}"
  let (_, pid) = startProcess(args[0], args: args[1..^1])
  result = trace_process(pid)

# ── CLI ───────────────────────────────────────────────────────────────────────
proc usage() =
  echo "sigma-strace — Sovereign Syscall Tracer v15.0"
  echo "Usage:"
  echo "  sigma-strace -p <pid>              Attach to running process"
  echo "  sigma-strace <cmd> [args...]       Run and trace command"
  echo "  sigma-strace -e <syscall> -p <pid> Filter by syscall name"
  echo "  sigma-strace -T -p <pid>           Show timing summary"

proc main() =
  var pid = -1; var cmd: seq[string]; var filter_name = ""; var show_summary = false
  var p = initOptParser()
  for kind, key, val in p.getopt():
    case kind
    of cmdOption:
      case key
      of "p": pid = parseInt(val)
      of "e": filter_name = val
      of "T": show_summary = true
      of "h","help": usage(); quit(0)
      else: discard
    of cmdArgument: cmd.add(key)
    else: discard

  var tracer: Tracer
  if pid > 0:
    tracer = trace_process(pid)
  elif cmd.len > 0:
    tracer = trace_cmd(cmd)
  else:
    usage(); quit(1)

  if show_summary: print_summary(tracer)

main()
