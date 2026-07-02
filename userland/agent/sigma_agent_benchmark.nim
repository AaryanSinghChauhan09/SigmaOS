# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_benchmark.nim — Agent response quality benchmarking
# Measures accuracy, latency, tool-call precision, and GUI parity coverage.
#
# Inspiration:
#   llama.cpp benchmark suite (llama-bench)
#   OpenAI evals framework
#   Aider benchmarks (aider --benchmark)
#   Claude Code accuracy tests
#
# Test categories:
#   accuracy   — does the agent map NL to the right tool/command?
#   latency    — response time per query (ms)
#   gui_parity — % of GUI actions correctly mapped to CLI
#   tools      — each of the 21 tools exercised and validated
#   regression — known-correct golden pairs checked after every change
#
# Language: Nim (stdlib only)

import std/[os, osproc, times, json, strutils, strformat, math, sequtils, tables]

# ── Test types ────────────────────────────────────────────────────────────────
type
  TestResult = enum TrPass, TrFail, TrSkip, TrError

  TestCase = object
    id:          string
    category:    string
    input:       string        # natural language input
    expected:    seq[string]   # expected keywords in output
    must_not:    seq[string]   # must NOT appear in output
    timeout_ms:  int           # max allowed latency
    trust:       string

  BenchResult = object
    test:        TestCase
    result:      TestResult
    actual:      string
    latency_ms:  int
    reason:      string

  BenchSuite = object
    results:  seq[BenchResult]
    started:  DateTime
    duration: int

# ── Golden test cases ─────────────────────────────────────────────────────────
const GOLDEN_TESTS: array[40, TestCase] = [
  # ── Files ──────────────────────────────────────────────────────────────────
  TestCase(id:"F01", category:"files",   input:"list .",           expected:@["sigma_agent"], must_not:@["error"],   timeout_ms:3000, trust:"standard"),
  TestCase(id:"F02", category:"files",   input:"read README.md",   expected:@["sigma-agent"],  must_not:@["✗"],       timeout_ms:3000, trust:"standard"),
  TestCase(id:"F03", category:"files",   input:"find README",      expected:@["README"],        must_not:@[],          timeout_ms:5000, trust:"standard"),
  TestCase(id:"F04", category:"files",   input:"disk usage",       expected:@["/"],             must_not:@["✗","error"],timeout_ms:3000, trust:"standard"),

  # ── Settings ───────────────────────────────────────────────────────────────
  TestCase(id:"S01", category:"settings",input:"set dark mode",    expected:@["dark","zenith","theme","✓"], must_not:@["✗"], timeout_ms:3000, trust:"standard"),
  TestCase(id:"S02", category:"settings",input:"set light mode",   expected:@["light","✓"],  must_not:@["✗"],  timeout_ms:3000, trust:"standard"),
  TestCase(id:"S03", category:"settings",input:"settings get appearance theme", expected:@["appearance","theme"], must_not:@[], timeout_ms:3000, trust:"standard"),

  # ── System ─────────────────────────────────────────────────────────────────
  TestCase(id:"Y01", category:"system",  input:"system info",      expected:@["SigmaOS","CPU","Memory","Uptime","sigma"],must_not:@["✗"],timeout_ms:5000, trust:"standard"),
  TestCase(id:"Y02", category:"system",  input:"show processes",    expected:@["PID","NAME","sigma"],  must_not:@[],         timeout_ms:5000, trust:"standard"),

  # ── Network ────────────────────────────────────────────────────────────────
  TestCase(id:"N01", category:"network", input:"network status",   expected:@["eth","wlan","UP","DOWN","IP","addr","net"], must_not:@[], timeout_ms:5000, trust:"standard"),

  # ── Accessibility ──────────────────────────────────────────────────────────
  TestCase(id:"A01", category:"a11y",    input:"accessibility high-contrast on",  expected:@["high","contrast","✓","true"], must_not:@["✗"], timeout_ms:3000, trust:"standard"),
  TestCase(id:"A02", category:"a11y",    input:"accessibility large-text on",     expected:@["large","text","✓"],           must_not:@["✗"], timeout_ms:3000, trust:"standard"),
  TestCase(id:"A03", category:"a11y",    input:"accessibility screen-reader on",  expected:@["screen","reader","✓"],        must_not:@["✗"], timeout_ms:3000, trust:"standard"),
  TestCase(id:"A04", category:"a11y",    input:"accessibility reduce-motion on",  expected:@["reduce","motion","✓"],        must_not:@["✗"], timeout_ms:3000, trust:"standard"),

  # ── GUI parity ─────────────────────────────────────────────────────────────
  TestCase(id:"G01", category:"gui",     input:"open app sigma-terminal",    expected:@["sigma-terminal","launch","open"], must_not:@["✗"], timeout_ms:3000, trust:"standard"),
  TestCase(id:"G02", category:"gui",     input:"workspace 2",                expected:@["workspace","2","switch","wm"],    must_not:@[],    timeout_ms:3000, trust:"standard"),
  TestCase(id:"G03", category:"gui",     input:"tile",                       expected:@["tile","layout","wm"],             must_not:@[],    timeout_ms:3000, trust:"standard"),
  TestCase(id:"G04", category:"gui",     input:"fullscreen",                 expected:@["fullscreen","wm"],                must_not:@[],    timeout_ms:3000, trust:"standard"),
  TestCase(id:"G05", category:"gui",     input:"set dark mode",              expected:@["dark","theme"],                   must_not:@["✗"], timeout_ms:3000, trust:"standard"),
  TestCase(id:"G06", category:"gui",     input:"notify 'Test' --body 'body'",expected:@["Test","notification","notify"],  must_not:@["✗"], timeout_ms:3000, trust:"standard"),
  TestCase(id:"G07", category:"gui",     input:"clipboard read",             expected:@["clipboard","paste","read"],       must_not:@["error"], timeout_ms:3000, trust:"standard"),

  # ── Packages ───────────────────────────────────────────────────────────────
  TestCase(id:"P01", category:"packages",input:"install sigma-edit",         expected:@["install","sigma-edit"],           must_not:@[],    timeout_ms:8000, trust:"standard"),

  # ── NL understanding ───────────────────────────────────────────────────────
  TestCase(id:"L01", category:"nl",      input:"what's my disk space",       expected:@["/"],       must_not:@["error"],   timeout_ms:5000, trust:"standard"),
  TestCase(id:"L02", category:"nl",      input:"am I using too much memory",  expected:@["MB","memory","mem"], must_not:@[],timeout_ms:5000, trust:"standard"),
  TestCase(id:"L03", category:"nl",      input:"show me what's eating CPU",  expected:@["CPU","cpu","process","PID"], must_not:@[],timeout_ms:5000, trust:"standard"),
  TestCase(id:"L04", category:"nl",      input:"is my VPN connected",        expected:@["vpn","status","connect","disconnect"],must_not:@[],timeout_ms:5000,trust:"standard"),

  # ── Security ───────────────────────────────────────────────────────────────
  TestCase(id:"E01", category:"security",input:"security scan",              expected:@["scan","security","score","findings"], must_not:@["✗"],timeout_ms:15000,trust:"standard"),
  TestCase(id:"E02", category:"security",input:"security logs",              expected:@["log","scan","finding","anomaly","info","warn"], must_not:@[],timeout_ms:10000,trust:"standard"),
  TestCase(id:"E03", category:"security",input:"security policies",          expected:@["policy","recommend","firewall","ssh","sigma"],must_not:@[],timeout_ms:5000,trust:"standard"),

  # ── Multi-agent ─────────────────────────────────────────────────────────────
  TestCase(id:"M01", category:"multi",   input:"multi --list",               expected:@["sigma-security","sigma-sysadmin","sigma-developer"], must_not:@["error"],timeout_ms:5000,trust:"standard"),

  # ── Context ────────────────────────────────────────────────────────────────
  TestCase(id:"C01", category:"context", input:"context",                    expected:@["hostname","uptime","cpu","memory"], must_not:@["error"],timeout_ms:8000,trust:"standard"),

  # ── Subcommands ─────────────────────────────────────────────────────────────
  TestCase(id:"D01", category:"daemon",  input:"daemon status",              expected:@["daemon","running","not running","backend","sigma-agent"],must_not:@[],timeout_ms:5000,trust:"standard"),
  TestCase(id:"V01", category:"voice",   input:"voice --status",             expected:@["backend","voice","whisper","sigma-voice"],must_not:@["error"],timeout_ms:5000,trust:"standard"),
  TestCase(id:"W01", category:"watch",   input:"watch help",                 expected:@["watch","dir","ext","suggest","interval"],must_not:@[],timeout_ms:3000,trust:"standard"),
  TestCase(id:"R01", category:"learn",   input:"learn stats",                expected:@["Total","Good","Bad","Excellent"],must_not:@["error"],timeout_ms:5000,trust:"standard"),
  TestCase(id:"T01", category:"train",   input:"train seed",                 expected:@["seed","dataset","jsonl","sigma"],must_not:@["error"],timeout_ms:5000,trust:"standard"),
  TestCase(id:"B01", category:"mirror",  input:"mirror count",               expected:@["Total","action","mapped"],must_not:@["error"],timeout_ms:5000,trust:"standard"),
  TestCase(id:"Q01", category:"complete",input:"complete install sig",       expected:@["sigma-edit","sigma-terminal","sigma"],must_not:@["error"],timeout_ms:3000,trust:"standard"),
  TestCase(id:"Z01", category:"plugin",  input:"plugin list",                expected:@["plugin","directory","installed","sigma"],must_not:@["error"],timeout_ms:3000,trust:"standard"),
]

# ── Runner ─────────────────────────────────────────────────────────────────────
proc run_test(tc: TestCase, agent_bin = "sigma-agent"): BenchResult =
  let start = now()
  let cmd = fmt"{agent_bin} --trust {tc.trust} --no-color {tc.input.quoteShell} 2>&1"
  let (actual, code) = execCmdEx(cmd)
  let elapsed = int((now() - start).inMilliseconds)
  let lower = actual.toLowerAscii

  # Check expected keywords
  var missing: seq[string]
  for kw in tc.expected:
    if kw.toLowerAscii notin lower:
      missing.add(kw)

  # Check forbidden keywords
  var forbidden: seq[string]
  for kw in tc.must_not:
    if kw.toLowerAscii in lower:
      forbidden.add(kw)

  # Check latency
  let too_slow = elapsed > tc.timeout_ms

  let result_kind = if missing.len > 0 or forbidden.len > 0: TrFail
                    elif code != 0 and actual.contains("error"):        TrError
                    elif too_slow:                                       TrFail
                    else:                                                TrPass

  var reason = ""
  if missing.len > 0:    reason &= fmt"Missing: {missing.join(\", \")} "
  if forbidden.len > 0:  reason &= fmt"Forbidden: {forbidden.join(\", \")} "
  if too_slow:           reason &= fmt"Timeout: {elapsed}ms > {tc.timeout_ms}ms"

  BenchResult(test: tc, result: result_kind, actual: actual.strip()[0..<min(200,actual.len)],
              latency_ms: elapsed, reason: reason.strip())

proc run_suite*(filter_category = "", quick = false,
                agent_bin = "sigma-agent"): BenchSuite =
  result.started = now()

  let tests = block:
    var t: seq[TestCase]
    for tc in GOLDEN_TESTS:
      if filter_category.len > 0 and tc.category != filter_category: continue
      if quick and tc.category in ["security","daemon","packages"]: continue
      t.add(tc)
    t

  echo fmt"\n{CYAN}{BOLD}Σ sigma-agent benchmark{RESET}"
  echo fmt"  Tests: {tests.len}  |  Filter: {if filter_category.len > 0: filter_category else: \"all\"}"
  echo fmt"  Binary: {agent_bin}\n"

  var pass_count = 0; var fail_count = 0; var skip_count = 0; var error_count = 0
  var total_latency = 0

  for i, tc in tests:
    stdout.write(fmt"  [{i+1:>2}/{tests.len}] {tc.id:<5} {tc.category:<10} {tc.input[0..<min(40,tc.input.len)].ljust(40)} ")
    stdout.flushFile()
    let r = run_test(tc, agent_bin)
    result.results.add(r)
    total_latency += r.latency_ms
    case r.result
    of TrPass:
      echo fmt"{GREEN}✓ PASS{RESET} {MUTED}{r.latency_ms}ms{RESET}"
      pass_count += 1
    of TrFail:
      echo fmt"{RED}✗ FAIL{RESET} {MUTED}{r.latency_ms}ms  {r.reason}{RESET}"
      fail_count += 1
    of TrError:
      echo fmt"{YELLOW}! ERR {RESET} {MUTED}{r.latency_ms}ms{RESET}"
      error_count += 1
    of TrSkip:
      echo fmt"{MUTED}○ SKIP{RESET}"
      skip_count += 1

  result.duration = int((now() - result.started).inMilliseconds)

  let pass_pct = if tests.len > 0: pass_count * 100 div tests.len else: 0
  let avg_lat  = if tests.len > 0: total_latency div tests.len else: 0
  let score_color = if pass_pct >= 90: GREEN elif pass_pct >= 70: YELLOW else: RED

  echo fmt"""
{MUTED}{'─'.repeat(70)}{RESET}
{BOLD}Results:{RESET}
  {GREEN}Pass:{RESET}  {pass_count}/{tests.len}  ({score_color}{pass_pct}%{RESET})
  {RED}Fail:{RESET}  {fail_count}
  {YELLOW}Error:{RESET} {error_count}
  {MUTED}Skip:{RESET}  {skip_count}
  Avg latency:  {avg_lat}ms
  Total time:   {result.duration}ms
"""

proc save_results*(suite: BenchSuite, path: string) =
  var arr = newJArray()
  for r in suite.results:
    arr.add(%*{
      "id":          r.test.id,
      "category":    r.test.category,
      "input":       r.test.input,
      "result":      $r.result,
      "latency_ms":  r.latency_ms,
      "reason":      r.reason,
      "actual":      r.actual,
    })
  let report = %*{
    "started":  $suite.started,
    "duration": suite.duration,
    "pass":     suite.results.filterIt(it.result == TrPass).len,
    "fail":     suite.results.filterIt(it.result == TrFail).len,
    "total":    suite.results.len,
    "results":  arr,
  }
  writeFile(path, report.pretty())
  echo fmt"✓ Benchmark report: {path}"

proc compare_results*(path_a, path_b: string) =
  ## Compare two benchmark runs (before/after a change)
  if not fileExists(path_a) or not fileExists(path_b):
    echo "✗ Both result files must exist"; return
  let a = parseJson(readFile(path_a))
  let b = parseJson(readFile(path_b))

  echo fmt"\n{CYAN}{BOLD}Σ Benchmark comparison{RESET}"
  echo fmt"  A: {path_a}  pass={a[\"pass\"].getInt}/{a[\"total\"].getInt}"
  echo fmt"  B: {path_b}  pass={b[\"pass\"].getInt}/{b[\"total\"].getInt}"

  let delta = b["pass"].getInt - a["pass"].getInt
  let delta_str = if delta > 0: fmt"{GREEN}+{delta}{RESET}" elif delta < 0: fmt"{RED}{delta}{RESET}" else: fmt"{MUTED}0{RESET}"
  echo fmt"\n  Change: {delta_str} passing tests"

  # Show regressions
  var reg_ids: seq[string]
  for r_b in b["results"]:
    if r_b["result"].getStr == "TrFail":
      for r_a in a["results"]:
        if r_a["id"].getStr == r_b["id"].getStr and r_a["result"].getStr == "TrPass":
          reg_ids.add(r_b["id"].getStr)
  if reg_ids.len > 0:
    echo fmt"\n  {RED}Regressions:{RESET} {reg_ids.join(\", \")}"
  else:
    echo fmt"  {GREEN}No regressions{RESET}"

# ── CLI ────────────────────────────────────────────────────────────────────────
proc benchmark_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "help":
    echo """sigma-agent benchmark — Response quality benchmarking

Usage:
  sigma-agent benchmark              Run full benchmark suite (40 tests)
  sigma-agent benchmark quick        Skip slow tests (packages, security)
  sigma-agent benchmark --cat files  Run only a category
  sigma-agent benchmark --save       Save results to JSON
  sigma-agent benchmark compare a.json b.json  Compare two runs

Categories: files, settings, system, network, a11y, gui, packages,
            nl, security, multi, context, daemon, voice, watch,
            learn, train, mirror, complete, plugin

Examples:
  sigma-agent benchmark
  sigma-agent benchmark quick
  sigma-agent benchmark --cat gui
  sigma-agent benchmark --save
  sigma-agent benchmark compare before.json after.json
"""
    return

  if args[0] == "compare":
    if args.len < 3: echo "Usage: sigma-agent benchmark compare <a.json> <b.json>"; return
    compare_results(args[1], args[2])
    return

  let quick    = "quick" in args
  let save_res = "--save" in args or "-s" in args
  let cat_idx  = args.find("--cat")
  let category = if cat_idx >= 0 and cat_idx + 1 < args.len: args[cat_idx+1] else: ""

  let suite = run_suite(category, quick)

  if save_res:
    let report_path = getEnv("HOME","/tmp") / fmt".cache/sigma/bench_{now().format(\"yyyyMMddHHmm\")}.json"
    createDir(report_path.parentDir())
    save_results(suite, report_path)
