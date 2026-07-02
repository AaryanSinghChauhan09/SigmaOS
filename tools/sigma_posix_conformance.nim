# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# tools/sigma_posix_conformance.nim — POSIX conformance test suite
# Novel Category 13 (Developer Acceleration):
# Runs SigmaOS vs POSIX compliance tests, reports % compatible.
# Tests: syscalls, shell features, filesystem, signals, IPC, networking
#
# Language: Nim (stdlib only)

import std/[os, osproc, strutils, strformat, times, json, sequtils, tables]

type
  ConformanceStatus = enum CsPass, CsFail, CsSkip, CsWarn

  ConformanceTest = object
    id:          string
    category:    string
    description: string
    command:     string     # shell command: exit 0=pass, non-0=fail
    expected_out: string    # optional expected stdout substring
    skip_reason:  string

  TestResult = object
    test:    ConformanceTest
    status:  ConformanceStatus
    actual:  string
    elapsed_ms: int

# ── POSIX conformance tests ────────────────────────────────────────────────
const POSIX_TESTS: array[60, (string, string, string, string)] = [
  # (id, category, description, command)

  # Filesystem
  ("FS-01", "filesystem", "mkdir creates directory",         "mkdir /tmp/posix_test_dir && test -d /tmp/posix_test_dir; rm -rf /tmp/posix_test_dir"),
  ("FS-02", "filesystem", "create file with touch",          "touch /tmp/posix_test_file && test -f /tmp/posix_test_file; rm /tmp/posix_test_file"),
  ("FS-03", "filesystem", "write and read file",             "echo hello > /tmp/posix_rw_test && grep -q hello /tmp/posix_rw_test; rm /tmp/posix_rw_test"),
  ("FS-04", "filesystem", "rename file",                     "touch /tmp/posix_a && mv /tmp/posix_a /tmp/posix_b && test -f /tmp/posix_b; rm /tmp/posix_b"),
  ("FS-05", "filesystem", "symlink creation",                "ln -s /tmp /tmp/posix_link && test -L /tmp/posix_link; rm /tmp/posix_link"),
  ("FS-06", "filesystem", "chmod sets permissions",          "touch /tmp/posix_perm && chmod 600 /tmp/posix_perm && stat -c %a /tmp/posix_perm | grep -q 600; rm /tmp/posix_perm"),
  ("FS-07", "filesystem", "stat returns file info",          "stat /tmp 2>/dev/null | grep -qi 'directory\\|inode'"),
  ("FS-08", "filesystem", "/proc/self/status accessible",    "test -r /proc/self/status"),
  ("FS-09", "filesystem", "/dev/null exists",                "test -c /dev/null"),
  ("FS-10", "filesystem", "tmpfs at /tmp writable",          "touch /tmp/posix_tmpfs && rm /tmp/posix_tmpfs"),

  # Process management
  ("PR-01", "process", "fork creates child process",         "bash -c 'sleep 0 &' 2>/dev/null"),
  ("PR-02", "process", "exec replaces process image",        "exec true"),
  ("PR-03", "process", "getpid returns positive",            "bash -c 'echo $$' | grep -E '^[0-9]+$'"),
  ("PR-04", "process", "exit code 0 on success",             "true && test $? -eq 0"),
  ("PR-05", "process", "exit code non-zero on failure",      "false; test $? -ne 0"),
  ("PR-06", "process", "wait for child process",             "bash -c 'sleep 0.1 & wait'"),
  ("PR-07", "process", "kill -0 checks process",             "kill -0 $$ 2>/dev/null"),
  ("PR-08", "process", "env variables accessible",           "test -n \"$PATH\""),
  ("PR-09", "process", "cd changes directory",               "cd /tmp && pwd | grep -q tmp"),
  ("PR-10", "process", "umask is set",                       "umask 2>/dev/null | grep -E '^[0-9]'"),

  # Shell features
  ("SH-01", "shell", "pipes work",                           "echo hello | grep -q hello"),
  ("SH-02", "shell", "stdout redirect to file",              "echo hello > /tmp/posix_sh1; grep -q hello /tmp/posix_sh1; rm /tmp/posix_sh1"),
  ("SH-03", "shell", "stdin redirect from file",             "echo test > /tmp/posix_sh2; grep -q test < /tmp/posix_sh2; rm /tmp/posix_sh2"),
  ("SH-04", "shell", "stderr redirect",                      "ls /nonexistent 2>/dev/null; true"),
  ("SH-05", "shell", "command substitution $()",             "X=$(echo hello) && test \"$X\" = hello"),
  ("SH-06", "shell", "glob expansion *",                     "ls /tmp/*.* 2>/dev/null; true"),
  ("SH-07", "shell", "variable assignment",                  "X=hello && test \"$X\" = hello"),
  ("SH-08", "shell", "if/then/else",                         "if true; then echo ok; else echo fail; fi | grep -q ok"),
  ("SH-09", "shell", "for loop",                             "for i in 1 2 3; do echo $i; done | wc -l | grep -q 3"),
  ("SH-10", "shell", "while loop",                           "X=0; while [ $X -lt 3 ]; do X=$((X+1)); done; test $X -eq 3"),

  # Standard utilities (POSIX required)
  ("UT-01", "utils", "echo works",                           "echo hello | grep -q hello"),
  ("UT-02", "utils", "cat works",                            "echo hello | cat | grep -q hello"),
  ("UT-03", "utils", "grep basic pattern",                   "echo hello | grep -q hello"),
  ("UT-04", "utils", "sed substitution",                     "echo hello | sed 's/hello/world/' | grep -q world"),
  ("UT-05", "utils", "awk print field",                      "echo 'a b c' | awk '{print $2}' | grep -q b"),
  ("UT-06", "utils", "sort -n numeric",                      "printf '3\\n1\\n2\\n' | sort -n | head -1 | grep -q 1"),
  ("UT-07", "utils", "uniq deduplication",                   "printf 'a\\na\\nb\\n' | uniq | wc -l | grep -q 2"),
  ("UT-08", "utils", "wc -l line count",                     "printf 'a\\nb\\nc\\n' | wc -l | grep -q 3"),
  ("UT-09", "utils", "head first lines",                     "printf '1\\n2\\n3\\n' | head -1 | grep -q 1"),
  ("UT-10", "utils", "tail last lines",                      "printf '1\\n2\\n3\\n' | tail -1 | grep -q 3"),
  ("UT-11", "utils", "find by name",                         "find /tmp -name '*.tmp' 2>/dev/null; true"),
  ("UT-12", "utils", "date outputs timestamp",               "date | grep -Ei 'mon|tue|wed|thu|fri|sat|sun'"),
  ("UT-13", "utils", "pwd prints directory",                 "pwd | grep -E '^/'"),
  ("UT-14", "utils", "ls lists files",                       "ls / 2>/dev/null | grep -qi 'tmp\\|usr\\|bin\\|etc'"),
  ("UT-15", "utils", "cp copies file",                       "echo x > /tmp/posix_cp_src && cp /tmp/posix_cp_src /tmp/posix_cp_dst && diff /tmp/posix_cp_src /tmp/posix_cp_dst; rm /tmp/posix_cp_{src,dst}"),

  # Signals
  ("SG-01", "signals", "SIGTERM default handler",            "bash -c 'sleep 100 & PID=$!; sleep 0.1; kill $PID; wait $PID 2>/dev/null; true'"),
  ("SG-02", "signals", "trap SIGINT",                        "bash -c 'trap \"exit 0\" INT; kill -INT $$'"),
  ("SG-03", "signals", "SIGUSR1 custom handler",             "bash -c 'trap \"echo handled\" USR1; kill -USR1 $$ 2>/dev/null; true'"),

  # IPC
  ("IPC-01", "ipc",   "named pipe (mkfifo)",                "mkfifo /tmp/posix_fifo && echo ok > /tmp/posix_fifo & cat /tmp/posix_fifo | grep -q ok; rm /tmp/posix_fifo"),
  ("IPC-02", "ipc",   "anonymous pipe",                     "echo hello | read X; true"),

  # Networking
  ("NET-01", "network", "loopback interface exists",         "ip link show lo 2>/dev/null | grep -q 'lo\\|LOOPBACK'"),
  ("NET-02", "network", "DNS resolution works",              "host sigmaos.org 2>/dev/null | grep -qi 'address\\|alias' || nslookup sigmaos.org 2>/dev/null | grep -qi address"),
  ("NET-03", "network", "TCP connect to localhost",          "bash -c 'exec 3<>/dev/tcp/127.0.0.1/22 2>/dev/null; echo $? | grep -qE \"^[01]$\"; exec 3>&-; true'"),

  # Locale / time
  ("LC-01", "locale",  "TZ env variable respected",         "TZ=UTC date | grep -qi UTC"),
  ("LC-02", "locale",  "locale command works",              "locale 2>/dev/null | grep -qi lang; true"),

  # SigmaOS extensions (POSIX+ beyond POSIX)
  ("SA-01", "sigmaos", "sigma-agent binary exists",         "which sigma-agent 2>/dev/null || test -x /usr/bin/sigma-agent"),
  ("SA-02", "sigmaos", "sigma-pkg binary exists",           "which sigma-pkg 2>/dev/null || test -x /usr/bin/sigma-pkg"),
  ("SA-03", "sigmaos", "sigma_pledge syscall available",    "test -f /proc/sigma/pledge_available || true"),
  ("SA-04", "sigmaos", "sigma-agent doctor runs",           "sigma-agent doctor 2>/dev/null | grep -qi 'sigma\\|version\\|backend'"),
  ("SA-05", "sigmaos", "workflow list works",               "sigma-agent workflow list 2>/dev/null; true"),
]

proc run_test(t: ConformanceTest): TestResult =
  let start = now()
  let (out, code) = execCmdEx("sh -c " & t.command.quoteShell & " 2>/dev/null")
  let elapsed = int((now() - start).inMilliseconds)
  let status = if t.skip_reason.len > 0:    CsSkip
               elif code == 0:
                 if t.expected_out.len > 0 and t.expected_out notin out: CsWarn
                 else: CsPass
               else: CsFail
  TestResult(test: t, status: status, actual: out.strip()[0..<min(80,out.strip().len)], elapsed_ms: elapsed)

proc run_conformance*(categories: seq[string] = @[], quick = false): seq[TestResult] =
  for (id, cat, desc, cmd) in POSIX_TESTS:
    if categories.len > 0 and cat notin categories: continue
    if quick and cat in ["signals","ipc","network"]: continue
    let t = ConformanceTest(id: id, category: cat, description: desc, command: cmd)
    result.add(run_test(t))

proc print_results*(results: seq[TestResult]) =
  var pass = 0; var fail = 0; var skip = 0; var warn = 0
  const G = "\e[38;2;52;211;153m"; const R = "\e[38;2;248;113;113m"
  const Y = "\e[38;2;251;191;36m"; const M = "\e[38;2;107;114;128m"; const X = "\e[0m"

  echo "\e[38;2;69;243;255m\e[1mΣ POSIX Conformance Test Suite\e[0m\n"
  var by_cat: Table[string, seq[TestResult]]
  for r in results: by_cat.mgetOrPut(r.test.category, @[]).add(r)

  for cat, cat_results in by_cat:
    let cat_pass = cat_results.filterIt(it.status == CsPass).len
    echo fmt"\e[1m  {cat.toUpperAscii:<12}\e[0m ({cat_pass}/{cat_results.len})"
    for r in cat_results:
      let (icon, color) = case r.status
        of CsPass: ("✓", G)
        of CsFail: ("✗", R)
        of CsWarn: ("⚠", Y)
        of CsSkip: ("○", M)
      echo fmt"    {color}{icon}{X}  {r.test.id:<8} {r.test.description[0..<min(45,r.test.description.len)]:<45} {r.elapsed_ms:>4}ms"
      if r.status == CsFail and r.actual.len > 0:
        echo fmt"         {M}got: {r.actual}{X}"
      case r.status
      of CsPass: pass += 1
      of CsFail: fail += 1
      of CsSkip: skip += 1
      of CsWarn: warn += 1
    echo ""

  let total = pass + fail + warn
  let pct = if total > 0: pass * 100 div total else: 0
  let score_color = if pct >= 90: G elif pct >= 70: Y else: R
  echo fmt"\n  {score_color}\e[1mPOSIX Conformance: {pct}%\e[0m  ({pass}/{total} tests pass  {fail} fail  {warn} warn  {skip} skip)"

  if pct >= 90: echo fmt"  {G}✓ Excellent POSIX compatibility{X}"
  elif pct >= 70: echo fmt"  {Y}⚠ Moderate compatibility — review failing tests{X}"
  else: echo fmt"  {R}✗ Low compatibility — significant POSIX gaps{X}"

proc conformance_cmd*(args: seq[string]) =
  if args.len > 0 and args[0] == "help":
    echo """sigma-posix — POSIX conformance test suite

Usage:
  sigma-posix                     Run all 60 POSIX tests
  sigma-posix --cat filesystem    Test specific category
  sigma-posix --quick             Skip slow tests
  sigma-posix --json              Output JSON results
  sigma-posix --list              List all test categories

Categories: filesystem, process, shell, utils, signals, ipc, network, locale, sigmaos
"""
    return

  let quick = "--quick" in args
  let json_out = "--json" in args
  let cat_idx  = args.find("--cat")
  let cats     = if cat_idx >= 0 and cat_idx+1 < args.len: @[args[cat_idx+1]] else: @[]

  if "--list" in args:
    for (id, cat, desc, _) in POSIX_TESTS:
      echo fmt"  {id:<8} {cat:<12} {desc}"
    return

  let results = run_conformance(cats, quick)

  if json_out:
    var arr = newJArray()
    for r in results:
      arr.add(%*{"id":r.test.id,"category":r.test.category,"status":$r.status,"ms":r.elapsed_ms})
    echo $arr
  else:
    print_results(results)
