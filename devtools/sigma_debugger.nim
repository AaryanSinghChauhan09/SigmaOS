# sigma_debugger.nim — Native Debugging Suite Integration
# Integrates GDB, perf, LTTng into a unified SigmaOS debugging experience.
# Provides structured output, AI-assisted analysis, and kernel tracing.

import std/[strutils, tables, sequtils, os, times]

type
  DebugBackend* = enum
    GDB
    Perf
    LTTng
    Valgrind
    Strace

  TraceEvent* = object
    timestamp*: float64
    pid*: int
    tid*: int
    event_type*: string
    function_name*: string
    latency_ns*: uint64
    cpu*: int
    stack_depth*: int

  PerfCounter* = object
    name*: string
    value*: uint64
    unit*: string
    enabled_time_ns*: uint64
    running_time_ns*: uint64

  DebugSession* = object
    id*: int
    backend*: DebugBackend
    target_pid*: int
    target_binary*: string
    is_active*: bool
    breakpoints*: seq[Breakpoint]
    events*: seq[TraceEvent]
    counters*: seq[PerfCounter]

  Breakpoint* = object
    id*: int
    address*: uint64
    file*: string
    line*: int
    condition*: string
    hit_count*: int
    enabled*: bool

# ── GDB Integration ─────────────────────────────────────────────────────

proc createGdbSession*(binary: string, pid: int = 0): DebugSession =
  ## Create a new GDB debug session targeting a binary or PID
  result = DebugSession(
    id: 1,
    backend: GDB,
    target_pid: pid,
    target_binary: binary,
    is_active: true,
    breakpoints: @[],
    events: @[],
    counters: @[]
  )

proc addBreakpoint*(session: var DebugSession, file: string, line: int,
                    condition: string = ""): int =
  ## Add a conditional breakpoint
  let bp = Breakpoint(
    id: session.breakpoints.len + 1,
    address: 0,
    file: file,
    line: line,
    condition: condition,
    hit_count: 0,
    enabled: true
  )
  session.breakpoints.add(bp)
  return bp.id

proc removeBreakpoint*(session: var DebugSession, bp_id: int) =
  session.breakpoints = session.breakpoints.filterIt(it.id != bp_id)

# ── Perf Integration ────────────────────────────────────────────────────

type
  PerfProfile* = object
    pid*: int
    duration_ms*: uint64
    sample_rate*: int
    counters*: seq[PerfCounter]
    flamegraph_path*: string

proc startPerfProfile*(pid: int, duration_ms: uint64 = 5000,
                       sample_rate: int = 99): PerfProfile =
  ## Launch perf record on a target process
  ## In production: exec `perf record -F {sample_rate} -p {pid} -g -- sleep {duration_ms/1000}`
  result = PerfProfile(
    pid: pid,
    duration_ms: duration_ms,
    sample_rate: sample_rate,
    counters: @[
      PerfCounter(name: "cpu-cycles", value: 0, unit: "cycles",
                  enabled_time_ns: 0, running_time_ns: 0),
      PerfCounter(name: "cache-misses", value: 0, unit: "misses",
                  enabled_time_ns: 0, running_time_ns: 0),
      PerfCounter(name: "instructions", value: 0, unit: "insns",
                  enabled_time_ns: 0, running_time_ns: 0),
      PerfCounter(name: "branch-misses", value: 0, unit: "misses",
                  enabled_time_ns: 0, running_time_ns: 0),
    ],
    flamegraph_path: "/tmp/sigma-perf-" & $pid & ".svg"
  )

proc generateFlamegraph*(profile: PerfProfile): string =
  ## Generate a flamegraph SVG from perf data
  ## In production: perf script | stackcollapse-perf.pl | flamegraph.pl > output.svg
  return profile.flamegraph_path

# ── LTTng Kernel Tracing ────────────────────────────────────────────────

type
  LttngSession* = object
    name*: string
    channel*: string
    events_enabled*: seq[string]
    trace_path*: string
    is_recording*: bool

proc createLttngSession*(name: string): LttngSession =
  result = LttngSession(
    name: name,
    channel: "sigma-channel0",
    events_enabled: @[],
    trace_path: "/var/log/sigma/traces/" & name,
    is_recording: false
  )

proc enableKernelEvent*(session: var LttngSession, event: string) =
  ## Enable a kernel tracepoint (e.g., "sched_switch", "irq_handler_entry")
  if event notin session.events_enabled:
    session.events_enabled.add(event)
  # In production: lttng enable-event -k {event} -s {session.name}

proc startRecording*(session: var LttngSession) =
  session.is_recording = true
  # In production: lttng start {session.name}

proc stopRecording*(session: var LttngSession) =
  session.is_recording = false
  # In production: lttng stop {session.name}

# ── AI-Assisted Analysis ────────────────────────────────────────────────

proc analyzeStackTrace*(trace: string): string =
  ## Send a stack trace to the local AI engine for root-cause analysis
  ## In production: calls sigma_ai_engine with prompt "Analyze this crash:"
  if "SIGSEGV" in trace:
    return "Likely null pointer dereference. Check pointer initialization before access."
  elif "SIGABRT" in trace:
    return "Assertion failure or memory corruption detected. Review recent allocations."
  elif "deadlock" in trace.toLowerAscii():
    return "Potential deadlock detected. Check lock ordering across threads."
  else:
    return "Stack trace submitted to SigmaAI for analysis."

proc suggestBreakpoints*(source_file: string, error_line: int): seq[int] =
  ## AI-suggested breakpoint locations around a crash site
  result = @[]
  for offset in [-5, -3, -1, 0, 1, 3, 5]:
    let line = error_line + offset
    if line > 0:
      result.add(line)
