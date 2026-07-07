# SigmaOS syscall_profiler — Nim implementation
# Replaces tools/syscall_profiler/profiler.py
# Compiles to native binary; zero Python/pip dependency.
# Reads /proc/<pid>/syscall or a strace-style trace file and tallies calls.

import std/[os, strutils, tables, algorithm]

type
  SyscallRecord = object
    name:  string
    count: int

proc parseProcSyscall(pid: int): string =
  let path = "/proc/" & $pid & "/syscall"
  if fileExists(path):
    return readFile(path).strip()
  return "(proc not available on this platform)"

proc parseSTrace(tracefile: string): seq[SyscallRecord] =
  var counts = initTable[string, int]()

  for line in lines(tracefile):
    let parts = line.splitWhitespace()
    if parts.len == 0: continue
    let name = parts[0].split("(")[0]
    if name.startsWith("---") or name.startsWith("+++"):
      continue
    counts[name] = counts.getOrDefault(name, 0) + 1

  for name, count in counts:
    result.add(SyscallRecord(name: name, count: count))

  result.sort(proc(a, b: SyscallRecord): int = cmp(b.count, a.count))

proc main =
  let args = commandLineParams()
  if args.len == 0:
    echo "Usage: syscall_profiler <tracefile.strace> | --pid <pid>"
    quit(1)

  if args[0] == "--pid" and args.len >= 2:
    let pid = parseInt(args[1])
    echo "Current syscall for PID ", pid, ":"
    echo parseProcSyscall(pid)
  else:
    let records = parseSTrace(args[0])
    echo "Syscall frequency report:"
    echo "─────────────────────────"
    for r in records:
      echo r.count, "\t", r.name

main()
