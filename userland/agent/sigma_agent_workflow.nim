# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_workflow.nim — n8n-style workflow automation
# YAML/JSON pipelines, event triggers, cron scheduling, external integrations.
#
# Inspiration:
#   n8n               — node-based workflow automation
#   Claude Code       — multi-step task execution
#   azure-cli         — az automation runbook
#   Aider             — automated code fix pipelines
#   OpenClaw          — event-driven agent actions
#
# Workflow format (.sigma-workflow.yaml):
#   name: weekly-backup
#   trigger: schedule=every friday 22:00
#   steps:
#     - name: backup
#       action: "run cp -r /home/user/Code /backup/"
#     - name: notify
#       action: "notify 'Backup done' 'Weekly backup complete'"
#     - name: security-check
#       action: "security scan --quick"
#       condition: "exit_code_of(backup) == 0"
#
# Language: Nim (stdlib only)

import std/[os, osproc, times, json, strutils, strformat,
            tables, sequtils, parseutils, hashes]

# ── Types ─────────────────────────────────────────────────────────────────────
type
  TriggerKind = enum
    TkManual, TkSchedule, TkEvent, TkFileChange, TkNetworkChange,
    TkCpuHigh, TkLowDisk, TkPkgUpdate, TkOnBoot, TkOnShutdown

  Trigger = object
    kind:       TriggerKind
    schedule:   string    # cron-like: "every friday 22:00", "daily 06:00", "*/5min"
    event_name: string    # system event name
    threshold:  float     # for CPU/disk triggers

  StepKind = enum SkAction, SkCondition, SkLoop, SkParallel

  WorkflowStep = object
    name:       string
    kind:       StepKind
    action:     string    # sigma-agent command or shell command
    condition:  string    # expression: "exit_code_of(stepname) == 0"
    on_fail:    string    # "continue" | "stop" | "notify"
    timeout_s:  int       # 0 = no timeout
    retries:    int

  Workflow = object
    name:       string
    description: string
    trigger:    Trigger
    steps:      seq[WorkflowStep]
    env:        Table[string, string]
    enabled:    bool
    last_run:   string
    run_count:  int
    created:    string

  StepResult = object
    name:       string
    success:    bool
    output:     string
    exit_code:  int
    duration_ms: int

  WorkflowRun = object
    workflow:   string
    started:    string
    finished:   string
    success:    bool
    steps:      seq[StepResult]
    trigger:    string

# ── Paths ─────────────────────────────────────────────────────────────────────
proc workflow_dir(): string =
  getEnv("HOME", "/tmp") / ".config/sigma/agent/workflows"

proc run_log_dir(): string =
  getEnv("HOME", "/tmp") / ".cache/sigma/agent/workflow_runs"

proc schedule_file(): string =
  getEnv("HOME", "/tmp") / ".cache/sigma/agent/workflow_schedule.json"

# ── YAML-lite parser (minimal subset for workflow files) ──────────────────────
proc parse_workflow_yaml(path: string): Workflow =
  result.enabled = true
  result.created = $now()
  result.env = initTable[string, string]()
  if not fileExists(path): return
  var current_step: WorkflowStep
  var in_step = false
  var in_env = false
  for raw in lines(path):
    let line = raw.expandTabs()
    let stripped = line.strip()
    if stripped.len == 0 or stripped.startsWith("#"): continue
    # Top-level keys
    if not line.startsWith("  ") and not line.startsWith("\t"):
      in_env = false
      if stripped.startsWith("name:"):
        result.name = stripped[5..^1].strip().strip(chars={'"','\''})
      elif stripped.startsWith("description:"):
        result.description = stripped[12..^1].strip().strip(chars={'"','\''})
      elif stripped.startsWith("enabled:"):
        result.enabled = "false" notin stripped.toLowerAscii
      elif stripped.startsWith("trigger:"):
        let tv = stripped[8..^1].strip()
        if tv.len > 0: result.trigger = parse_trigger(tv)
      elif stripped == "env:": in_env = true
      elif stripped == "steps:": discard
      elif stripped.startsWith("- name:") or stripped.startsWith("-name:"):
        if in_step and current_step.name.len > 0:
          result.steps.add(current_step)
        current_step = WorkflowStep(kind: SkAction, on_fail: "stop", timeout_s: 60, retries: 0)
        current_step.name = stripped.split(":")[1].strip().strip(chars={'"','\''})
        in_step = true
    elif in_env and ":" in stripped:
      let parts = stripped.split(":", 1)
      if parts.len == 2:
        result.env[parts[0].strip()] = parts[1].strip().strip(chars={'"','\''})
    elif in_step:
      if stripped.startsWith("action:"):
        current_step.action = stripped[7..^1].strip().strip(chars={'"','\''})
      elif stripped.startsWith("condition:"):
        current_step.condition = stripped[10..^1].strip().strip(chars={'"','\''})
      elif stripped.startsWith("on_fail:"):
        current_step.on_fail = stripped[8..^1].strip()
      elif stripped.startsWith("timeout:"):
        current_step.timeout_s = try: parseInt(stripped[8..^1].strip()) except: 60
      elif stripped.startsWith("retries:"):
        current_step.retries = try: parseInt(stripped[8..^1].strip()) except: 0
      elif stripped.startsWith("- name:"):
        result.steps.add(current_step)
        current_step = WorkflowStep(kind: SkAction, on_fail: "stop", timeout_s: 60)
        current_step.name = stripped[7..^1].strip().strip(chars={'"','\''})
  if in_step and current_step.name.len > 0:
    result.steps.add(current_step)

proc parse_trigger(spec: string): Trigger =
  let lower = spec.toLowerAscii.strip()
  if lower.startsWith("schedule=") or lower.startsWith("cron="):
    let sched = spec.split("=")[1].strip()
    return Trigger(kind: TkSchedule, schedule: sched)
  elif lower == "manual" or lower == "":
    return Trigger(kind: TkManual)
  elif lower == "boot" or lower == "on_boot":
    return Trigger(kind: TkOnBoot)
  elif lower.startsWith("cpu>"):
    let thresh = try: parseFloat(lower[4..^1]) except: 90.0
    return Trigger(kind: TkCpuHigh, threshold: thresh)
  elif lower.startsWith("disk<"):
    let thresh = try: parseFloat(lower[5..^1]) except: 10.0
    return Trigger(kind: TkLowDisk, threshold: thresh)
  elif lower.startsWith("file:"):
    return Trigger(kind: TkFileChange, event_name: spec[5..^1].strip())
  elif lower.startsWith("network:"):
    return Trigger(kind: TkNetworkChange, event_name: spec[8..^1].strip())
  elif lower == "pkg_update":
    return Trigger(kind: TkPkgUpdate)
  Trigger(kind: TkManual)

# ── Schedule parser ────────────────────────────────────────────────────────────
proc next_run_secs(schedule: string): int64 =
  ## Parse human schedule → seconds until next run
  let now_ts = now().toTime.toUnix
  let lower = schedule.toLowerAscii.strip()
  # "*/5min", "*/30min", "*/1h"
  if lower.startsWith("*/"):
    let rest = lower[2..^1]
    if rest.endsWith("min"):
      let mins = try: parseInt(rest[0..^4]) except: 60
      return int64(mins * 60)
    elif rest.endsWith("h"):
      let hrs = try: parseInt(rest[0..^2]) except: 1
      return int64(hrs * 3600)
  # "daily HH:MM"
  if lower.startsWith("daily"):
    let time_str = lower.split(' ').getOrDefault(1, "06:00")
    let parts = time_str.split(':')
    let target_h = try: parseInt(parts[0]) except: 6
    let target_m = try: (if parts.len > 1: parseInt(parts[1]) else: 0) except: 0
    let n = now()
    var target = dateTime(n.year, n.month, n.monthday, target_h, target_m, 0, 0, utc())
    if target.toTime.toUnix <= now_ts: target = target + 1.days
    return target.toTime.toUnix - now_ts
  # "every friday HH:MM"
  if lower.startsWith("every"):
    let parts = lower.split(' ')
    let day_str = if parts.len > 1: parts[1] else: "friday"
    let time_str = if parts.len > 2: parts[2] else: "22:00"
    let target_dow = case day_str
      of "monday":    1
      of "tuesday":   2
      of "wednesday": 3
      of "thursday":  4
      of "friday":    5
      of "saturday":  6
      else:           7  # sunday
    let time_parts = time_str.split(':')
    let th = try: parseInt(time_parts[0]) except: 22
    let tm = try: (if time_parts.len > 1: parseInt(time_parts[1]) else: 0) except: 0
    var n = now()
    var days_ahead = (target_dow - n.weekday.ord + 7) mod 7
    if days_ahead == 0 and (n.hour * 60 + n.minute) >= (th * 60 + tm):
      days_ahead = 7
    let target = dateTime(n.year, n.month, n.monthday + days_ahead, th, tm, 0, 0, utc())
    return target.toTime.toUnix - now_ts
  # "hourly"
  if lower == "hourly": return 3600
  # "weekly"
  if lower == "weekly": return 7 * 86400
  # Default: 1 day
  86400'i64

# ── Condition evaluator ────────────────────────────────────────────────────────
proc eval_condition(cond: string, results: Table[string, StepResult]): bool =
  if cond.len == 0: return true
  let lower = cond.toLowerAscii.strip()
  # "exit_code_of(stepname) == 0"
  if "exit_code_of(" in lower:
    let s = lower.split("exit_code_of(")[1].split(")")[0].strip()
    let code = if s in results: results[s].exit_code else: -1
    if "== 0" in lower: return code == 0
    if "!= 0" in lower: return code != 0
    if "> 0" in lower:  return code > 0
  # "success_of(stepname)"
  if "success_of(" in lower:
    let s = lower.split("success_of(")[1].split(")")[0].strip()
    return if s in results: results[s].success else: false
  # "output_contains(stepname, 'text')"
  if "output_contains(" in lower:
    let inner = lower.split("output_contains(")[1].split(")")[0]
    let parts = inner.split(",")
    let step_name = parts[0].strip()
    let text = if parts.len > 1: parts[1].strip().strip(chars={'"','\''}) else: ""
    let out = if step_name in results: results[step_name].output.toLowerAscii else: ""
    return text in out
  # Default: true
  true

# ── Step executor ──────────────────────────────────────────────────────────────
proc execute_step(step: WorkflowStep, env: Table[string, string],
                  dry_run = false, verbose = false): StepResult =
  let start = now().toTime.toUnix * 1000
  result.name = step.name

  if dry_run:
    echo fmt"  [dry-run] Step '{step.name}': {step.action}"
    result.success = true; result.exit_code = 0
    result.output  = fmt"[dry-run] {step.action}"
    return

  if verbose: echo fmt"\n  \e[38;2;69;243;255m→ Step: {step.name}\e[0m"
  if verbose: echo fmt"    Action: {step.action}"

  # Set env vars for this step
  for k, v in env: putEnv(k, v)

  # Dispatch: sigma-agent command or raw shell
  var cmd = step.action
  let is_agent_cmd = not cmd.startsWith("run ") and
                     not cmd.startsWith("bash ") and
                     not cmd.startsWith("sh ")

  let full_cmd = if is_agent_cmd:
    fmt"sigma-agent-core --trust standard --once {cmd.quoteShell} 2>&1"
  else:
    fmt"sh -c {cmd.split(' ', 1)[1].quoteShell} 2>&1"

  var attempt = 0
  while attempt <= step.retries:
    let (out, code) = execCmdEx(full_cmd)
    result.output    = out.strip()
    result.exit_code = code
    result.success   = code == 0
    if result.success or attempt >= step.retries: break
    if verbose: echo fmt"    Retry {attempt+1}/{step.retries}..."
    sleep(1000)
    attempt += 1

  result.duration_ms = int(now().toTime.toUnix * 1000 - start)

  if verbose:
    let icon = if result.success: "\e[38;2;52;211;153m✓\e[0m" else: "\e[38;2;248;113;113m✗\e[0m"
    echo fmt"    {icon} {result.output[0..<min(80,result.output.len)]}  ({result.duration_ms}ms)"

# ── Workflow runner ────────────────────────────────────────────────────────────
proc run_workflow*(wf: Workflow, trigger_src = "manual",
                   dry_run = false, verbose = false): WorkflowRun =
  result.workflow = wf.name
  result.started  = $now()
  result.trigger  = trigger_src
  var step_results: Table[string, StepResult]

  let CYAN  = "\e[38;2;69;243;255m"
  let GREEN = "\e[38;2;52;211;153m"
  let RED   = "\e[38;2;248;113;113m"
  let MUTED = "\e[38;2;107;114;128m"
  let RESET = "\e[0m"
  let BOLD  = "\e[1m"

  echo fmt"\n{CYAN}{BOLD}Σ Workflow: {wf.name}{RESET}  {MUTED}[{trigger_src}]{RESET}"
  if wf.description.len > 0: echo fmt"  {MUTED}{wf.description}{RESET}"
  echo fmt"  Steps: {wf.steps.len}"
  if dry_run: echo fmt"  {MUTED}[dry-run mode]{RESET}"
  echo ""

  var all_ok = true
  for i, step in wf.steps:
    # Evaluate condition
    if step.condition.len > 0 and not eval_condition(step.condition, step_results):
      echo fmt"  {MUTED}○ Skipped: {step.name} (condition not met){RESET}"
      step_results[step.name] = StepResult(name: step.name, success: true,
                                            output: "skipped", exit_code: 0)
      continue

    stdout.write(fmt"  [{i+1}/{wf.steps.len}] {step.name:<25} ")
    stdout.flushFile()

    let sr = execute_step(step, wf.env, dry_run, verbose)
    step_results[step.name] = sr
    result.steps.add(sr)

    if sr.success:
      echo fmt"{GREEN}✓{RESET}  {MUTED}{sr.duration_ms}ms{RESET}"
    else:
      echo fmt"{RED}✗{RESET}  {MUTED}{sr.output[0..<min(60,sr.output.len)]}{RESET}"
      all_ok = false
      case step.on_fail
      of "continue": discard
      of "notify":
        discard execCmdEx(fmt"sigma-agent notify \"Workflow failed\" \"Step '{step.name}' in '{wf.name}' failed\" --critical 2>/dev/null")
        break
      else: break  # "stop" is default

  result.finished = $now()
  result.success  = all_ok
  let status_str  = if all_ok: GREEN & "✓ PASS" & RESET else: RED & "✗ FAIL" & RESET
  echo fmt"\n  {status_str}  Workflow: {wf.name}"

# ── Workflow registry (load/save all .yaml files) ─────────────────────────────
proc load_workflows(): seq[Workflow] =
  let dir = workflow_dir()
  if not dirExists(dir): return
  for _, path in walkDir(dir):
    if path.endsWith(".yaml") or path.endsWith(".yml"):
      let wf = parse_workflow_yaml(path)
      if wf.name.len > 0: result.add(wf)

proc save_run_log(run: WorkflowRun) =
  createDir(run_log_dir())
  let path = run_log_dir() / fmt"{run.workflow}_{now().toTime.toUnix}.json"
  let j = %*{"workflow": run.workflow, "started": run.started,
              "finished": run.finished, "success": run.success,
              "trigger": run.trigger,
              "steps": run.steps.mapIt(%*{"name":it.name,"success":it.success,
                                          "exit_code":it.exit_code,"ms":it.duration_ms,
                                          "output":it.output[0..<min(200,it.output.len)]})}
  writeFile(path, $j)

# ── NL → Workflow generator ───────────────────────────────────────────────────
proc nl_to_workflow(goal: string): Workflow =
  ## Convert a natural language description into a Workflow struct
  ## Rule-based for offline; daemon/Ollama if available
  let lower = goal.toLowerAscii
  var wf = Workflow(name: "generated", enabled: true, created: $now(),
                    env: initTable[string, string]())

  # Try LLM first
  let llm_prompt = """Generate a sigma-agent workflow YAML for this goal.
Use this format:
name: <short-name>
description: <one line>
trigger: <manual|schedule=daily 06:00|cpu>90|pkg_update>
steps:
  - name: <step-name>
    action: <sigma-agent natural language command>
    on_fail: stop

Goal: """ & goal & "\nYAML:"

  let (daemon_ok, _) = execCmdEx("curl -sf http://localhost:11430/v1/status --max-time 1 2>/dev/null")
  if daemon_ok.len > 0:
    let body = $ %*{"message": llm_prompt, "max_tokens": 400, "include_context": false}
    let (out, code) = execCmdEx(
      fmt"""curl -sf -X POST http://localhost:11430/v1/chat -d {body.quoteShell} --max-time 15""")
    if code == 0:
      try:
        let resp = parseJson(out).getOrDefault("response").getStr("")
        if "name:" in resp and "steps:" in resp:
          let tmp = "/tmp/sigma_wf_gen.yaml"
          writeFile(tmp, resp)
          return parse_workflow_yaml(tmp)
      except: discard

  # Rule-based fallback
  wf.name = lower.split(' ')[0..min(2, lower.split(' ').len-1)].join("-")
                  .replace(" ","-")

  if "backup" in lower:
    wf.name = "auto-backup"
    wf.description = "Automated backup: " & goal
    wf.trigger = Trigger(kind: TkSchedule, schedule: "every friday 22:00")
    if "weekly" in lower or "friday" in lower:
      wf.trigger.schedule = "every friday 22:00"
    elif "daily" in lower:
      wf.trigger.schedule = "daily 02:00"
    wf.steps = @[
      WorkflowStep(name:"backup", action:"run cp -r /home/user/Code /backup/code/",
                   on_fail:"notify", timeout_s:120),
      WorkflowStep(name:"verify", action:"run ls -la /backup/code/",
                   on_fail:"continue", timeout_s:10),
      WorkflowStep(name:"notify", action:"notify 'Backup complete' 'Weekly backup done'",
                   on_fail:"continue", timeout_s:5)]

  elif "update" in lower or "upgrade" in lower:
    wf.name = "auto-update"
    wf.description = "Automated system update"
    wf.trigger = Trigger(kind: TkSchedule, schedule: "daily 06:00")
    wf.steps = @[
      WorkflowStep(name:"update", action:"run sigma-pkg update",
                   on_fail:"notify", timeout_s:300),
      WorkflowStep(name:"security", action:"security scan --quick",
                   on_fail:"continue", timeout_s:30),
      WorkflowStep(name:"notify", action:"notify 'Update done' 'System updated'",
                   on_fail:"continue", timeout_s:5)]

  elif "security" in lower or "audit" in lower or "harden" in lower:
    wf.name = "security-audit"
    wf.description = "Scheduled security audit"
    wf.trigger = Trigger(kind: TkSchedule, schedule: "daily 23:00")
    wf.steps = @[
      WorkflowStep(name:"scan", action:"security scan",
                   on_fail:"notify", timeout_s:60),
      WorkflowStep(name:"report", action:"security logs",
                   on_fail:"continue", timeout_s:30),
      WorkflowStep(name:"alert", action:"notify 'Security audit' 'Daily scan complete'",
                   on_fail:"continue", timeout_s:5)]

  elif "monitor" in lower or "cpu" in lower or "memory" in lower:
    wf.name = "resource-monitor"
    wf.description = "React when CPU/memory is high"
    wf.trigger = Trigger(kind: TkCpuHigh, threshold: 90.0)
    wf.steps = @[
      WorkflowStep(name:"diagnose", action:"multi \"why is CPU high\"",
                   on_fail:"continue", timeout_s:30),
      WorkflowStep(name:"alert", action:"notify 'High CPU' 'CPU usage critical' --critical",
                   on_fail:"continue", timeout_s:5)]

  elif "deploy" in lower or "ci" in lower or "build" in lower:
    wf.name = "auto-deploy"
    wf.description = "Build and deploy pipeline"
    wf.trigger = Trigger(kind: TkManual)
    wf.steps = @[
      WorkflowStep(name:"build", action:"run cargo build --release",
                   on_fail:"stop", timeout_s:300),
      WorkflowStep(name:"test", action:"run cargo test", on_fail:"stop", timeout_s:120),
      WorkflowStep(name:"notify", action:"notify 'Deploy' 'Build succeeded'",
                   on_fail:"continue", timeout_s:5)]

  else:
    wf.name = "custom-workflow"
    wf.description = goal
    wf.trigger = Trigger(kind: TkManual)
    wf.steps = @[
      WorkflowStep(name:"step1", action:goal, on_fail:"stop", timeout_s:60),
      WorkflowStep(name:"done",
                   action: fmt"notify 'Done' '{goal[0..<min(40,goal.len)]}'",
                   on_fail:"continue", timeout_s:5)]
  wf

# ── Workflow serializer (YAML output) ─────────────────────────────────────────
proc trigger_spec(t: Trigger): string =
  case t.kind
  of TkManual:        "manual"
  of TkSchedule:      "schedule=" & t.schedule
  of TkCpuHigh:       fmt"cpu>{t.threshold:.0f}"
  of TkLowDisk:       fmt"disk<{t.threshold:.0f}"
  of TkFileChange:    "file:" & t.event_name
  of TkNetworkChange: "network:" & t.event_name
  of TkPkgUpdate:     "pkg_update"
  of TkOnBoot:        "boot"
  of TkOnShutdown:    "shutdown"
  of TkEvent:         "event:" & t.event_name

proc to_yaml(wf: Workflow): string =
  var lines: seq[string]
  lines.add(fmt"name: {wf.name}")
  lines.add(fmt"description: \"{wf.description}\"")
  lines.add(fmt"enabled: {wf.enabled}")
  lines.add(fmt"trigger: {trigger_spec(wf.trigger)}")
  if wf.env.len > 0:
    lines.add("env:")
    for k, v in wf.env: lines.add(fmt"  {k}: \"{v}\"")
  lines.add("steps:")
  for step in wf.steps:
    lines.add(fmt"  - name: {step.name}")
    lines.add(fmt"    action: \"{step.action}\"")
    if step.condition.len > 0: lines.add(fmt"    condition: \"{step.condition}\"")
    if step.on_fail != "stop":  lines.add(fmt"    on_fail: {step.on_fail}")
    if step.timeout_s != 60:    lines.add(fmt"    timeout: {step.timeout_s}")
    if step.retries > 0:        lines.add(fmt"    retries: {step.retries}")
  lines.join("\n")

# ── Event trigger checker ──────────────────────────────────────────────────────
proc check_event_trigger(t: Trigger): bool =
  case t.kind
  of TkCpuHigh:
    try:
      let la = readFile("/proc/loadavg").split()[0]
      return parseFloat(la) * 100.0 > t.threshold
    except: return false
  of TkLowDisk:
    let (df, code) = execCmdEx("df / --output=pcent 2>/dev/null | tail -1")
    if code == 0:
      try:
        let pct = parseFloat(df.strip().replace("%",""))
        return (100.0 - pct) < t.threshold
      except: return false
    return false
  of TkPkgUpdate:
    let (out, code) = execCmdEx("sigma-pkg check-updates 2>/dev/null | wc -l")
    return code == 0 and (try: parseInt(out.strip()) > 0 except: false)
  of TkFileChange:
    return fileExists(t.event_name & ".changed")
  of TkNetworkChange:
    let (_, code) = execCmdEx("ip link show 2>/dev/null | grep -q DOWN")
    return code == 0
  else: return false

# ── Scheduler daemon (background loop) ────────────────────────────────────────
proc scheduler_loop*(once = false) =
  ## Main scheduling loop — checks triggers every 60s
  ## Run as background process or one-shot for manual trigger check
  var schedule: Table[string, int64]   # workflow_name → next_run unix timestamp

  proc load_schedule() =
    if fileExists(schedule_file()):
      try:
        let j = parseJson(readFile(schedule_file()))
        for k, v in j: schedule[k] = v.getInt(0)
      except: discard

  proc save_schedule() =
    createDir(schedule_file().parentDir())
    var j = newJObject()
    for k, v in schedule: j[k] = %v
    writeFile(schedule_file(), $j)

  load_schedule()
  echo "[workflow] Scheduler started. Watching " & workflow_dir()

  while true:
    let workflows = load_workflows()
    let now_ts = now().toTime.toUnix

    for wf in workflows:
      if not wf.enabled: continue
      let last = schedule.getOrDefault(wf.name, 0)

      case wf.trigger.kind
      of TkSchedule:
        let interval = next_run_secs(wf.trigger.schedule)
        if now_ts - last >= interval:
          echo fmt"[workflow] Triggering '{wf.name}' (schedule: {wf.trigger.schedule})"
          let run = run_workflow(wf, "schedule")
          save_run_log(run)
          schedule[wf.name] = now_ts
          save_schedule()

      of TkCpuHigh, TkLowDisk, TkPkgUpdate, TkFileChange, TkNetworkChange:
        # Debounce: don't re-trigger within 5 minutes
        if now_ts - last < 300: continue
        if check_event_trigger(wf.trigger):
          echo fmt"[workflow] Event trigger: '{wf.name}'"
          let run = run_workflow(wf, $wf.trigger.kind)
          save_run_log(run)
          schedule[wf.name] = now_ts
          save_schedule()

      of TkOnBoot:
        # Only run once per session (last run was > 23h ago)
        if now_ts - last > 82800:
          echo fmt"[workflow] Boot trigger: '{wf.name}'"
          let run = run_workflow(wf, "boot")
          save_run_log(run)
          schedule[wf.name] = now_ts
          save_schedule()

      else: discard

    if once: break
    sleep(60_000)  # check every 60 seconds

# ── Built-in workflow templates ────────────────────────────────────────────────
const WORKFLOW_TEMPLATES: array[8, (string, string)] = [
  ("weekly-backup", """name: weekly-backup
description: "Backup Code and Documents every Friday night"
enabled: true
trigger: schedule=every friday 22:00
steps:
  - name: backup-code
    action: "run cp -r /home/user/Code /backup/code/"
    on_fail: notify
    timeout: 120
  - name: backup-docs
    action: "run cp -r /home/user/Documents /backup/docs/"
    on_fail: continue
    timeout: 60
  - name: disk-check
    action: "disk usage"
    on_fail: continue
  - name: done
    action: "notify 'Backup complete' 'Weekly backup finished successfully'"
    on_fail: continue
"""),
  ("daily-update", """name: daily-update
description: "Update all packages and run security scan daily"
enabled: true
trigger: schedule=daily 06:00
steps:
  - name: update-packages
    action: "run sigma-pkg update"
    on_fail: notify
    timeout: 300
    retries: 1
  - name: security-scan
    action: "security scan"
    on_fail: continue
    timeout: 60
  - name: notify
    action: "notify 'Daily update' 'System updated and secured'"
    on_fail: continue
"""),
  ("cpu-alert", """name: cpu-alert
description: "Alert and diagnose when CPU load exceeds 90%"
enabled: true
trigger: cpu>90
steps:
  - name: diagnose
    action: "multi --agent sysadmin 'diagnose high CPU usage'"
    on_fail: continue
    timeout: 30
  - name: alert
    action: "notify 'High CPU' 'CPU usage exceeded 90% - check processes' --critical"
    on_fail: continue
"""),
  ("low-disk-alert", """name: low-disk-alert
description: "Alert when root filesystem is more than 90% full"
enabled: true
trigger: disk<10
steps:
  - name: check
    action: "disk usage"
    on_fail: continue
  - name: alert
    action: "notify 'Low Disk Space' 'Root filesystem is nearly full' --critical"
    on_fail: continue
  - name: suggest
    action: "multi --agent sysadmin 'suggest how to free disk space'"
    on_fail: continue
"""),
  ("dev-workflow", """name: dev-workflow
description: "Build, test, and notify on completion"
enabled: true
trigger: manual
steps:
  - name: build
    action: "run cargo build --release"
    on_fail: stop
    timeout: 300
  - name: test
    action: "run cargo test"
    on_fail: notify
    timeout: 120
    condition: "exit_code_of(build) == 0"
  - name: review
    action: "security scan --quick"
    on_fail: continue
    condition: "exit_code_of(test) == 0"
  - name: done
    action: "notify 'Build complete' 'All tests passed, security checked'"
    on_fail: continue
    condition: "success_of(test)"
"""),
  ("security-hardening", """name: security-hardening
description: "Apply security hardening and generate report"
enabled: true
trigger: manual
steps:
  - name: scan
    action: "security scan"
    on_fail: continue
    timeout: 60
  - name: firewall
    action: "settings set network firewall true"
    on_fail: continue
  - name: privacy
    action: "settings set privacy telemetry false"
    on_fail: continue
  - name: policies
    action: "security policies"
    on_fail: continue
  - name: report
    action: "notify 'Security hardening complete' 'System hardened and audited'"
    on_fail: continue
"""),
  ("on-boot-setup", """name: on-boot-setup
description: "Run doctor and sync knowledge on every boot"
enabled: true
trigger: boot
steps:
  - name: doctor
    action: "doctor"
    on_fail: continue
    timeout: 30
  - name: sync-knowledge
    action: "daemon sync"
    on_fail: continue
    timeout: 30
  - name: context
    action: "context"
    on_fail: continue
"""),
  ("pkg-update-notify", """name: pkg-update-notify
description: "Notify when package updates are available"
enabled: true
trigger: pkg_update
steps:
  - name: list-updates
    action: "run sigma-pkg check-updates"
    on_fail: continue
  - name: notify
    action: "notify 'Updates available' 'New packages available - run sigma-pkg update'"
    on_fail: continue
"""),
]

proc install_template*(name: string, dest_dir: string): bool =
  for (tname, content) in WORKFLOW_TEMPLATES:
    if tname == name or tname.replace("-","") == name.replace("-",""):
      createDir(dest_dir)
      writeFile(dest_dir / fmt"{tname}.yaml", content)
      return true
  false

# ── Run history viewer ─────────────────────────────────────────────────────────
proc show_history*(workflow_name = "", last_n = 10) =
  let run_dir = run_log_dir()
  if not dirExists(run_dir): echo "No workflow runs yet."; return
  var runs: seq[(string, JsonNode)]
  for _, path in walkDir(run_dir):
    if not path.endsWith(".json"): continue
    if workflow_name.len > 0 and workflow_name notin path: continue
    try:
      let j = parseJson(readFile(path))
      runs.add((path, j))
    except: discard
  runs.sort(proc(a,b:(string,JsonNode)):int = cmp(b[0],a[0]))
  let slice = runs[0..<min(last_n, runs.len)]
  echo fmt"\e[38;2;69;243;255m\e[1mΣ Workflow run history ({slice.len} runs)\e[0m\n"
  for (_, j) in slice:
    let ok     = j.getOrDefault("success").getBool
    let icon   = if ok: "\e[38;2;52;211;153m✓\e[0m" else: "\e[38;2;248;113;113m✗\e[0m"
    let name   = j.getOrDefault("workflow").getStr("?")
    let ts     = j.getOrDefault("started").getStr("?")[0..<min(16,j.getOrDefault("started").getStr.len)]
    let trig   = j.getOrDefault("trigger").getStr("?")
    let steps  = j.getOrDefault("steps").getElems.len
    echo fmt"  {icon}  {name:<25} {ts}  trigger={trig}  steps={steps}"

# ── Audit trail ────────────────────────────────────────────────────────────────
proc audit_log*(action, workflow, details: string) =
  ## Append every workflow action to the audit log
  let log_path = getEnv("HOME", "/tmp") / ".cache/sigma/agent/workflow_audit.log"
  createDir(log_path.parentDir())
  var f = open(log_path, fmAppend)
  f.writeLine(fmt"[{$now()}] [{action}] workflow={workflow} {details}")
  f.close()

# ── CLI ────────────────────────────────────────────────────────────────────────
proc workflow_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "help":
    echo """sigma-agent workflow — n8n-style automation engine

USAGE:
  sigma-agent workflow list                     List all workflows
  sigma-agent workflow run <name>               Run a workflow now
  sigma-agent workflow run <name> --dry-run     Preview steps without executing
  sigma-agent workflow run <name> --verbose     Show step output live
  sigma-agent workflow create "<goal>"          NL → new workflow YAML
  sigma-agent workflow create "<goal>" -o file  Save workflow to file
  sigma-agent workflow install <template>       Install a built-in template
  sigma-agent workflow install --all            Install all 8 templates
  sigma-agent workflow templates                List built-in templates
  sigma-agent workflow edit <name>              Open workflow YAML in editor
  sigma-agent workflow enable <name>            Enable a workflow
  sigma-agent workflow disable <name>           Disable a workflow
  sigma-agent workflow delete <name>            Delete a workflow
  sigma-agent workflow history [name]           Show run history
  sigma-agent workflow audit                    Show audit log
  sigma-agent workflow scheduler                Start background scheduler
  sigma-agent workflow check                    Check which workflows should run now

TRIGGER FORMATS:
  manual                  Run on demand only
  schedule=daily 06:00    Every day at 06:00
  schedule=*/30min        Every 30 minutes
  schedule=every friday 22:00  Every Friday at 22:00
  cpu>90                  When CPU load exceeds 90%
  disk<10                 When free disk < 10%
  pkg_update              When package updates available
  boot                    On every system boot
  file:/path/to/watch     When a file changes

STEP FORMAT (.yaml):
  steps:
    - name: my-step
      action: "sigma-agent natural language command"
      condition: "exit_code_of(prev-step) == 0"
      on_fail: stop|continue|notify
      timeout: 60
      retries: 1

EXAMPLES:
  sigma-agent workflow install weekly-backup
  sigma-agent workflow run weekly-backup --dry-run
  sigma-agent workflow create "run security scan every night at 23:00"
  sigma-agent workflow create "back up home folder every Friday" -o backup.yaml
  sigma-agent workflow list
  sigma-agent workflow history
  sigma-agent workflow scheduler
"""
    return

  let wf_dir   = workflow_dir()
  let dry_run  = "--dry-run" in args or "-d" in args
  let verbose  = "--verbose" in args or "-v" in args

  case args[0].toLowerAscii

  of "list","ls":
    let wfs = load_workflows()
    echo fmt"\e[38;2;69;243;255m\e[1mΣ Workflows\e[0m  ({wfs.len} found in {wf_dir})\n"
    if wfs.len == 0:
      echo "  No workflows yet. Try:"
      echo "    sigma-agent workflow install weekly-backup"
      echo "    sigma-agent workflow create \"backup home folder every Friday\""
      return
    for wf in wfs:
      let status = if wf.enabled: "\e[38;2;52;211;153m●\e[0m" else: "\e[38;2;107;114;128m○\e[0m"
      let trig   = trigger_spec(wf.trigger)
      echo fmt"  {status}  {wf.name:<25} trigger={trig}"
      if wf.description.len > 0:
        echo fmt"     \e[38;2;107;114;128m{wf.description}\e[0m"
      echo fmt"     Steps: {wf.steps.len}"

  of "run","exec","execute":
    let name = if args.len > 1: args[1] else: ""
    if name.len == 0: echo "Usage: sigma-agent workflow run <name>"; return
    let wfs = load_workflows()
    var found = false
    for wf in wfs:
      if wf.name == name or wf.name.replace("-","") == name.replace("-",""):
        found = true
        audit_log("RUN", wf.name, fmt"trigger=manual dry_run={dry_run}")
        let run = run_workflow(wf, "manual", dry_run, verbose)
        save_run_log(run)
        break
    if not found:
      echo fmt"✗ Workflow not found: '{name}'"
      echo "  Run: sigma-agent workflow list"

  of "create","new","generate","gen":
    let goal = args[1..^1].filterIt(not it.startsWith("-")).join(" ")
    if goal.len == 0: echo "Usage: sigma-agent workflow create \"your goal\""; return
    let wf = nl_to_workflow(goal)
    let yaml_content = to_yaml(wf)
    let oi = args.find("-o")
    if oi >= 0 and oi+1 < args.len:
      let out_path = args[oi+1]
      writeFile(out_path, yaml_content)
      echo fmt"\e[38;2;52;211;153m✓ Workflow saved: {out_path}\e[0m"
      echo fmt"  Run with: sigma-agent workflow run {wf.name}"
      echo fmt"  Or install: cp {out_path} {wf_dir}/{wf.name}.yaml"
    else:
      echo yaml_content

  of "install":
    if args.len < 2: echo "Usage: sigma-agent workflow install <template|--all>"; return
    createDir(wf_dir)
    if args[1] == "--all":
      for (name, _) in WORKFLOW_TEMPLATES:
        discard install_template(name, wf_dir)
        echo fmt"  ✓ {name}"
      echo fmt"\n✓ Installed {WORKFLOW_TEMPLATES.len} workflow templates to {wf_dir}"
    else:
      if install_template(args[1], wf_dir):
        echo fmt"✓ Workflow installed: {args[1]}"
        echo fmt"  Run with: sigma-agent workflow run {args[1]}"
        echo fmt"  Edit at:  {wf_dir}/{args[1]}.yaml"
      else:
        echo fmt"✗ Template not found: {args[1]}"
        echo "  Run: sigma-agent workflow templates"

  of "templates","list-templates":
    echo "\e[38;2;69;243;255m\e[1mΣ Built-in workflow templates:\e[0m\n"
    for (name, content) in WORKFLOW_TEMPLATES:
      let trigger_line = content.splitLines().filterIt(it.startsWith("trigger:")).getOrDefault(0,"")
      let desc_line    = content.splitLines().filterIt(it.startsWith("description:")).getOrDefault(0,"")
      echo fmt"  {name:<25}  {trigger_line[8..^1].strip()}  {desc_line[13..^1].strip(chars={'\"',' '})}"
    echo fmt"\nInstall one: sigma-agent workflow install <name>"
    echo fmt"Install all: sigma-agent workflow install --all"

  of "enable":
    if args.len < 2: echo "Usage: sigma-agent workflow enable <name>"; return
    let path = wf_dir / args[1] & ".yaml"
    if fileExists(path):
      let content = readFile(path).replace("enabled: false","enabled: true")
      writeFile(path, content)
      echo fmt"✓ Workflow enabled: {args[1]}"
    else: echo fmt"✗ Workflow not found: {args[1]}"

  of "disable":
    if args.len < 2: echo "Usage: sigma-agent workflow disable <name>"; return
    let path = wf_dir / args[1] & ".yaml"
    if fileExists(path):
      let content = readFile(path).replace("enabled: true","enabled: false")
      writeFile(path, content)
      echo fmt"✓ Workflow disabled: {args[1]}"
    else: echo fmt"✗ Workflow not found: {args[1]}"

  of "delete","remove","rm":
    if args.len < 2: echo "Usage: sigma-agent workflow delete <name>"; return
    let path = wf_dir / args[1] & ".yaml"
    if fileExists(path):
      removeFile(path)
      echo fmt"✓ Workflow deleted: {args[1]}"
    else: echo fmt"✗ Workflow not found: {args[1]}"

  of "edit":
    if args.len < 2: echo "Usage: sigma-agent workflow edit <name>"; return
    let path = wf_dir / args[1] & ".yaml"
    if not fileExists(path): echo fmt"✗ Not found: {path}"; return
    let editor = getEnv("EDITOR", "sigma-edit")
    discard execCmdEx(fmt"{editor} {path.quoteShell}")

  of "history","log","runs":
    let name = if args.len > 1 and not args[1].startsWith("-"): args[1] else: ""
    let n = block:
      let ni = args.find("-n")
      if ni >= 0 and ni+1 < args.len: try: parseInt(args[ni+1]) except: 10
      else: 20
    show_history(name, n)

  of "audit":
    let log_path = getEnv("HOME","/tmp") / ".cache/sigma/agent/workflow_audit.log"
    if fileExists(log_path):
      let lines_all = readFile(log_path).splitLines()
      for line in lines_all[max(0,lines_all.len-30)..^1]:
        echo line
    else: echo "No audit log yet."

  of "scheduler","daemon","watch":
    echo "\e[38;2;69;243;255mΣ sigma-agent workflow scheduler starting...\e[0m"
    echo fmt"  Watching: {wf_dir}"
    echo "  Press Ctrl+C to stop.\n"
    scheduler_loop()

  of "check","trigger-check":
    let wfs = load_workflows()
    echo fmt"Checking {wfs.len} workflow triggers...\n"
    for wf in wfs:
      if not wf.enabled: continue
      let would_fire = check_event_trigger(wf.trigger)
      let status = if would_fire: "\e[38;2;52;211;153m● FIRE\e[0m"
                   else:          "\e[38;2;107;114;128m○ wait\e[0m"
      echo fmt"  {status}  {wf.name:<25} trigger={trigger_spec(wf.trigger)}"

  else:
    # Try as workflow name (run it directly)
    let wfs = load_workflows()
    var found = false
    for wf in wfs:
      if wf.name == args[0]:
        found = true
        let run = run_workflow(wf, "manual", dry_run, verbose)
        save_run_log(run)
        break
    if not found:
      echo fmt"Unknown workflow command: '{args[0]}'"
      echo "Run: sigma-agent workflow help"
