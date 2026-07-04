# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_perf_regression.nim — Built-in performance regression detector
# Novel: Every kernel build automatically benchmarked vs previous 3 builds.
# Alerts on regressions > threshold. Reports p50/p95/p99 per metric.
#
# Language: Nim (stdlib only)

import std/[os, json, times, osproc, strutils, strformat, math, tables, sequtils, algorithm]

# ── Benchmark types ───────────────────────────────────────────────────────
type
  BenchMetric = object
    name:        string
    unit:        string
    values:      seq[float64]   # raw samples
    p50, p95, p99: float64
    mean:        float64
    stddev:      float64

  BenchResult = object
    commit:      string
    timestamp:   string
    os_version:  string
    cpu_model:   string
    metrics:     Table[string, BenchMetric]

  RegressionAlert = object
    metric:      string
    prev_p95:    float64
    cur_p95:     float64
    pct_change:  float64
    is_regression: bool

# ── Statistics ────────────────────────────────────────────────────────────
proc percentile(vals: seq[float64], pct: float64): float64 =
  if vals.len == 0: return 0.0
  var sorted = vals.sorted()
  let idx = ((pct / 100.0) * float64(sorted.len - 1)).round.int.clamp(0, sorted.len - 1)
  sorted[idx]

proc compute_stats(m: var BenchMetric) =
  if m.values.len == 0: return
  m.p50    = percentile(m.values, 50.0)
  m.p95    = percentile(m.values, 95.0)
  m.p99    = percentile(m.values, 99.0)
  m.mean   = m.values.sum() / float64(m.values.len)
  let variance = m.values.mapIt((it - m.mean) ^ 2.0).sum() / float64(m.values.len)
  m.stddev = sqrt(variance)

# ── Built-in micro-benchmarks ─────────────────────────────────────────────
proc bench_syscall_latency(samples = 10_000): BenchMetric =
  result.name = "syscall_latency_us"
  result.unit = "µs"
  for _ in 0..<samples:
    let start = cpuTime()
    discard execCmdEx("true 2>/dev/null")
    result.values.add((cpuTime() - start) * 1_000_000.0)
  compute_stats(result)

proc bench_pipe_throughput(runs = 5): BenchMetric =
  result.name = "pipe_throughput_mbs"
  result.unit = "MB/s"
  for _ in 0..<runs:
    let (out, _) = execCmdEx("dd if=/dev/zero bs=64k count=512 2>&1 | grep copied | grep -oP '[0-9.]+ MB/s'")
    let val = try: parseFloat(out.strip().split()[0]) except: 0.0
    if val > 0: result.values.add(val)
  compute_stats(result)

proc bench_memory_alloc(iters = 1000): BenchMetric =
  result.name = "memory_alloc_ns"
  result.unit = "ns"
  for _ in 0..<iters:
    let start = cpuTime()
    var v = newSeq[byte](4096)
    v[0] = 1   # prevent optimization
    result.values.add((cpuTime() - start) * 1_000_000_000.0)
  compute_stats(result)

proc bench_sigma_agent_latency(runs = 50): BenchMetric =
  result.name = "sigma_agent_latency_ms"
  result.unit = "ms"
  for _ in 0..<runs:
    let start = cpuTime()
    discard execCmdEx("sigma-agent --no-color \"list .\" 2>/dev/null | head -1")
    result.values.add((cpuTime() - start) * 1000.0)
  compute_stats(result)

proc bench_fs_ops(iters = 500): BenchMetric =
  result.name = "fs_ops_us"
  result.unit = "µs"
  let tmp = "/tmp/sigma_bench_fs"
  for i in 0..<iters:
    let start = cpuTime()
    writeFile(fmt"{tmp}_{i}", "test")
    removeFile(fmt"{tmp}_{i}")
    result.values.add((cpuTime() - start) * 1_000_000.0)
  compute_stats(result)

proc run_full_benchmark(quick = false): BenchResult =
  result.timestamp = $now()
  result.metrics = initTable[string, BenchMetric]()
  let (commit_out, _) = execCmdEx("git rev-parse --short HEAD 2>/dev/null")
  result.commit = commit_out.strip().getOrDefault("unknown")
  let (cpu_out, _) = execCmdEx("grep 'model name' /proc/cpuinfo 2>/dev/null | head -1 | cut -d: -f2")
  result.cpu_model = cpu_out.strip()
  result.os_version = "SigmaOS v15.1"

  echo "\e[38;2;69;243;255mΣ Running performance benchmark...\e[0m"
  let benchmarks: seq[proc(): BenchMetric] = if quick: @[
    proc(): BenchMetric = bench_syscall_latency(1000),
    proc(): BenchMetric = bench_memory_alloc(100),
    proc(): BenchMetric = bench_sigma_agent_latency(10),
  ] else: @[
    proc(): BenchMetric = bench_syscall_latency(5000),
    proc(): BenchMetric = bench_pipe_throughput(3),
    proc(): BenchMetric = bench_memory_alloc(500),
    proc(): BenchMetric = bench_sigma_agent_latency(20),
    proc(): BenchMetric = bench_fs_ops(200),
  ]

  for bench_fn in benchmarks:
    let m = bench_fn()
    echo fmt"  {m.name:<30} p50={m.p50:.1f} p95={m.p95:.1f} p99={m.p99:.1f} {m.unit}"
    result.metrics[m.name] = m

# ── History store ─────────────────────────────────────────────────────────
proc history_dir(): string =
  getEnv("HOME", "/tmp") / ".cache/sigma/bench_history"

proc save_result(r: BenchResult) =
  createDir(history_dir())
  var metrics_json = newJObject()
  for name, m in r.metrics:
    metrics_json[name] = %*{"p50":m.p50,"p95":m.p95,"p99":m.p99,"mean":m.mean,"unit":m.unit}
  let j = %*{"commit":r.commit,"timestamp":r.timestamp,
              "cpu":r.cpu_model,"os":r.os_version,"metrics":metrics_json}
  writeFile(history_dir() / fmt"{$now().toTime.toUnix}_{r.commit}.json", j.pretty())

proc load_history(last_n = 5): seq[BenchResult] =
  var entries: seq[(string, BenchResult)]
  if not dirExists(history_dir()): return
  for _, path in walkDir(history_dir()):
    if not path.endsWith(".json"): continue
    try:
      let j = parseJson(readFile(path))
      var r = BenchResult(
        commit:    j.getOrDefault("commit").getStr,
        timestamp: j.getOrDefault("timestamp").getStr,
        cpu_model: j.getOrDefault("cpu").getStr,
        os_version:j.getOrDefault("os").getStr,
        metrics:   initTable[string, BenchMetric]())
      if j.hasKey("metrics"):
        for name, mj in j["metrics"]:
          r.metrics[name] = BenchMetric(
            name:  name,
            unit:  mj.getOrDefault("unit").getStr("?"),
            p50:   mj.getOrDefault("p50").getFloat,
            p95:   mj.getOrDefault("p95").getFloat,
            p99:   mj.getOrDefault("p99").getFloat,
            mean:  mj.getOrDefault("mean").getFloat)
      entries.add((path, r))
    except: discard
  entries.sortedByIt(it[0]).mapIt(it[1]).reversed[0..<min(last_n, entries.len)]

# ── Regression detector ───────────────────────────────────────────────────
proc detect_regressions*(current: BenchResult, threshold_pct = 10.0): seq[RegressionAlert] =
  let history = load_history(3)
  if history.len == 0: return

  # Compare against average of last 3 runs
  for metric_name, cur_m in current.metrics:
    var prev_p95_vals: seq[float64]
    for prev in history:
      if metric_name in prev.metrics:
        prev_p95_vals.add(prev.metrics[metric_name].p95)
    if prev_p95_vals.len == 0: continue
    let avg_prev_p95 = prev_p95_vals.sum() / float64(prev_p95_vals.len)
    if avg_prev_p95 == 0: continue
    let pct_change = (cur_m.p95 - avg_prev_p95) / avg_prev_p95 * 100.0
    # For latency metrics: increase = regression; for throughput: decrease = regression
    let is_latency = "latency" in metric_name or "ns" in cur_m.unit or "us" in cur_m.unit
    let is_regression = if is_latency: pct_change > threshold_pct
                        else: pct_change < -threshold_pct
    result.add RegressionAlert(
      metric:       metric_name,
      prev_p95:     avg_prev_p95,
      cur_p95:      cur_m.p95,
      pct_change:   pct_change,
      is_regression: is_regression)

# ── CLI ────────────────────────────────────────────────────────────────────
proc perf_regression_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "help":
    echo """sigma-perf — Performance regression detector

Usage:
  sigma-perf run                  Run full benchmark suite
  sigma-perf run --quick          Quick benchmark (fewer samples)
  sigma-perf history              Show recent benchmark history
  sigma-perf compare              Compare last 2 runs
  sigma-perf check                Run + check for regressions (CI mode)

Metrics tracked:
  syscall_latency_us      p50/p95/p99 µs per syscall
  pipe_throughput_mbs     MB/s through kernel pipe
  memory_alloc_ns         ns per 4K allocation
  sigma_agent_latency_ms  ms per sigma-agent invocation
  fs_ops_us               µs per file create+delete

Exit codes (for CI):
  0 = no regression
  1 = regression detected (> 10% on any p95 metric)
"""
    return

  let quick = "--quick" in args or "-q" in args

  case args[0].toLowerAscii
  of "run","bench","benchmark":
    let result = run_full_benchmark(quick)
    save_result(result)
    echo fmt"\n✓ Benchmark complete. Saved to {history_dir()}"

  of "history","log":
    let history = load_history(5)
    if history.len == 0: echo "No benchmark history found. Run: sigma-perf run"
    else:
      echo "\e[38;2;69;243;255m\e[1mΣ Benchmark history (last 5 runs)\e[0m\n"
      for r in history:
        echo fmt"  {r.timestamp[0..<16]}  {r.commit}"
        for name, m in r.metrics:
          echo fmt"    {name:<30} p95={m.p95:.1f} {m.unit}"

  of "check","ci":
    echo "Running benchmark for regression check..."
    let result = run_full_benchmark(quick)
    let alerts = detect_regressions(result)
    save_result(result)
    var has_regression = false
    for alert in alerts:
      if alert.is_regression:
        has_regression = true
        let arrow = if "latency" in alert.metric or "ns" in alert.metric: "↑" else: "↓"
        echo fmt"\e[38;2;248;113;113m✗ REGRESSION [{alert.metric}]: {arrow}{alert.pct_change:.1f}%  prev_p95={alert.prev_p95:.1f} cur_p95={alert.cur_p95:.1f}\e[0m"
      else:
        echo fmt"\e[38;2;52;211;153m✓ OK [{alert.metric}]: {alert.pct_change:+.1f}%\e[0m"
    if has_regression:
      echo "\n\e[38;2;248;113;113m✗ PERFORMANCE REGRESSION DETECTED\e[0m"
      quit(1)
    else:
      echo "\n\e[38;2;52;211;153m✓ No performance regressions\e[0m"

  of "compare":
    let history = load_history(2)
    if history.len < 2: echo "Need at least 2 benchmark runs. Run: sigma-perf run"; return
    let (a, b) = (history[1], history[0])
    echo fmt"\e[38;2;69;243;255m\e[1mΣ Comparison: {a.commit} → {b.commit}\e[0m\n"
    for name in a.metrics.keys:
      if name in b.metrics:
        let pa = a.metrics[name].p95; let pb = b.metrics[name].p95
        let pct = if pa > 0: (pb - pa) / pa * 100.0 else: 0.0
        let color = if abs(pct) < 5: "\e[38;2;107;114;128m"
                    elif pct > 5:    "\e[38;2;248;113;113m"
                    else:            "\e[38;2;52;211;153m"
        echo fmt"  {name:<30} {pa:.1f} → {pb:.1f}  {color}{pct:+.1f}%\e[0m  {a.metrics[name].unit}"

  else:
    echo fmt"Unknown command: {args[0]}"
