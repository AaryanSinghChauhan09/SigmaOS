# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
# userland/tools/sigma_top.nim — sigma-top: Real-time Process Monitor
# Language: Nim — native, OOP via object + methods, ncurses-free

import std/[os, strutils, osproc, strformat, tables, algorithm, terminal, times]

type
  ProcInfo = object
    pid:      int
    ppid:     int
    name:     string
    state:    char
    cpu_pct:  float
    mem_kb:   uint64
    threads:  int
    user:     string
    cmd:      string
    cpu_prev: uint64
    cpu_total_prev: uint64

  TopState = object
    procs:       seq[ProcInfo]
    sort_by:     string
    reverse:     bool
    filter:      string
    refresh_ms:  int
    cpu_pct:     float
    mem_used_kb: uint64
    mem_total_kb: uint64
    uptime_s:    uint64
    load_avg:    array[3,float]

proc read_proc_info(pid: int): ProcInfo =
  result.pid = pid
  result.state = 'R'
  let stat_path = fmt"/proc/{pid}/stat"
  let comm_path = fmt"/proc/{pid}/comm"
  let status_path = fmt"/proc/{pid}/status"
  let cmd_path  = fmt"/proc/{pid}/cmdline"

  result.name = try: readFile(comm_path).strip() except: fmt"<{pid}>"
  result.cmd  = try: readFile(cmd_path).replace('\0',' ').strip() except: result.name

  if fileExists(status_path):
    for line in lines(status_path):
      let p = line.splitWhitespace()
      if p.len >= 2:
        case p[0]
        of "Pid:":     result.pid    = try: parseInt(p[1]) except: pid
        of "PPid:":    result.ppid   = try: parseInt(p[1]) except: 0
        of "State:":   result.state  = if p[1].len > 0: p[1][0] else: 'R'
        of "VmRSS:":   result.mem_kb = try: parseUInt(p[1]) except: 0
        of "Threads:": result.threads= try: parseInt(p[1]) except: 1
        else: discard

proc read_system_stats(st: var TopState) =
  if fileExists("/proc/stat"):
    for line in lines("/proc/stat"):
      if line.startsWith("cpu "):
        let nums = line.splitWhitespace()[1..^1].mapIt(try: parseUInt(it) except: 0'u64)
        if nums.len >= 4:
          let total: uint64 = nums.foldl(a+b, 0'u64)
          let idle = nums[3]
          st.cpu_pct = if total > 0: 100.0 * float(total - idle) / float(total) else: 0
        break

  if fileExists("/proc/meminfo"):
    for line in lines("/proc/meminfo"):
      let p = line.splitWhitespace()
      if p.len >= 2:
        case p[0]
        of "MemTotal:":     st.mem_total_kb = try: parseUInt(p[1]) except: 0
        of "MemAvailable:": st.mem_used_kb  = st.mem_total_kb - (try: parseUInt(p[1]) except: 0)
        else: discard

  if fileExists("/proc/loadavg"):
    let la = readFile("/proc/loadavg").splitWhitespace()
    for i in 0..<3.min(la.len):
      st.load_avg[i] = try: parseFloat(la[i]) except: 0

  if fileExists("/proc/uptime"):
    let up = readFile("/proc/uptime").splitWhitespace()
    if up.len > 0: st.uptime_s = try: parseUInt(up[0].split('.')[0]) except: 0

proc scan_procs(): seq[ProcInfo] =
  result = @[]
  for kind, path in walkDir("/proc"):
    let name = path.extractFilename
    if name.allIt(it.isDigit):
      let pid = try: parseInt(name) except: continue
      result.add(read_proc_info(pid))

proc format_mem(kb: uint64): string =
  if kb >= 1_000_000: fmt"{kb div 1_000_000}GB"
  elif kb >= 1_000:   fmt"{kb div 1_000}MB"
  else:               fmt"{kb}KB"

proc format_uptime(s: uint64): string =
  let h = s div 3600; let m = (s mod 3600) div 60; let sec = s mod 60
  fmt"{h:02}:{m:02}:{sec:02}"

proc draw_top(st: TopState, procs: seq[ProcInfo], rows: int) =
  # Header
  echo "\e[2J\e[H" # clear screen
  echo fmt"\e[32msigma-top\e[0m  up {format_uptime(st.uptime_s)}  load: {st.load_avg[0]:.2f} {st.load_avg[1]:.2f} {st.load_avg[2]:.2f}"
  let mem_pct = if st.mem_total_kb > 0: 100'u64 * st.mem_used_kb div st.mem_total_kb else: 0
  echo fmt"CPU: \e[33m{st.cpu_pct:5.1f}%\e[0m  MEM: \e[33m{format_mem(st.mem_used_kb)}/{format_mem(st.mem_total_kb)} ({mem_pct}%)\e[0m"
  echo ""
  echo fmt"\e[1m{'PID':>7} {'PPID':>7} {'S':>2} {'CPU%':>6} {'MEM':>8} {'THR':>4}  {'NAME':<20} CMD\e[0m"
  echo "-".repeat(80)
  var shown = 0
  for p in procs:
    if shown >= rows - 5: break
    let state_color = case p.state
      of 'R': "\e[32m" of 'S': "\e[36m" of 'D': "\e[31m" of 'Z': "\e[35m" else: "\e[37m"
    let cmd_short = if p.cmd.len > 30: p.cmd[0..29] & "…" else: p.cmd
    echo fmt"{p.pid:>7} {p.ppid:>7} {state_color}{p.state}\e[0m {p.cpu_pct:>5.1f}% {format_mem(p.mem_kb):>8} {p.threads:>4}  {p.name:<20} {cmd_short}"
    shown += 1
  echo "\e[90m  q=quit  s=sort  r=reverse  /=filter\e[0m"

proc sort_procs(procs: var seq[ProcInfo], by: string, rev: bool) =
  case by
  of "cpu": procs.sort(proc(a,b: ProcInfo): int = cmp(b.cpu_pct, a.cpu_pct))
  of "mem": procs.sort(proc(a,b: ProcInfo): int = cmp(b.mem_kb,  a.mem_kb))
  of "pid": procs.sort(proc(a,b: ProcInfo): int = cmp(a.pid, b.pid))
  of "name": procs.sort(proc(a,b: ProcInfo): int = cmp(a.name, b.name))
  else: discard
  if rev: procs.reverse()

proc run_top(interval_ms = 1000, once = false) =
  var st = TopState(sort_by: "mem", reverse: false, refresh_ms: interval_ms)
  while true:
    read_system_stats(st)
    var procs = scan_procs()
    sort_procs(procs, st.sort_by, st.reverse)
    let (cols, rows) = try: (terminalWidth(), terminalHeight()) except: (80, 24)
    draw_top(st, procs, rows)
    if once: break
    sleep(interval_ms)

proc main() =
  var once = false; var delay = 1000
  for kind, key, val in getopt():
    case kind
    of cmdOption:
      case key
      of "1","once","n": once = true
      of "d": delay = try: parseInt(val) * 1000 except: 1000
      else: discard
    else: discard
  run_top(delay, once)

main()
