# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_notify.nim — Desktop notification + alert system
# Send rich notifications from any agent action; subscribe to system events.
#
# Inspiration:
#   Claude Code — task-complete toasts
#   azure-cli   — long-running command completion alerts
#   Hermes IDE  — IDE event notifications
#   ai-shell    — inline success/error indicators
#
# Features:
#   - Send desktop notifications (sigma-notify → libnotify → wall fallback)
#   - Subscribe to system events (file changes, build completion, security alerts)
#   - Notification history log
#   - Per-category urgency rules
#   - Agent-hook: auto-notify on long-running command completion
#
# Language: Nim (stdlib only)

import std/[os, osproc, times, json, strutils, strformat, tables, sequtils]

# ── Types ─────────────────────────────────────────────────────────────────────
type
  Urgency = enum UrgLow, UrgNormal, UrgCritical

  Notification = object
    id:       string
    ts:       string
    title:    string
    body:     string
    urgency:  Urgency
    category: string
    icon:     string
    sent:     bool

# ── Storage ────────────────────────────────────────────────────────────────────
proc notif_log_path(): string =
  getEnv("HOME", "/tmp") / ".cache/sigma/notifications.jsonl"

proc append_log(n: Notification) =
  createDir(notif_log_path().parentDir())
  var f = open(notif_log_path(), fmAppend)
  f.writeLine($(%*{"id":n.id,"ts":n.ts,"title":n.title,"body":n.body,
                   "urgency":$n.urgency,"category":n.category,"sent":n.sent}))
  f.close()

proc load_history(last_n = 20): seq[Notification] =
  if not fileExists(notif_log_path()): return
  var all_lines: seq[string]
  for line in lines(notif_log_path()):
    if line.strip().len > 0: all_lines.add(line)
  let slice = all_lines[max(0, all_lines.len - last_n)..^1]
  for line in slice:
    try:
      let j = parseJson(line)
      result.add Notification(
        id:       j.getOrDefault("id").getStr,
        ts:       j.getOrDefault("ts").getStr,
        title:    j.getOrDefault("title").getStr,
        body:     j.getOrDefault("body").getStr,
        urgency:  case j.getOrDefault("urgency").getStr
                  of "UrgCritical": UrgCritical
                  of "UrgLow":      UrgLow
                  else:             UrgNormal,
        category: j.getOrDefault("category").getStr,
        sent:     j.getOrDefault("sent").getBool)
    except: discard

# ── Send notification via available backend ───────────────────────────────────
proc send*(title, body: string, urgency = UrgNormal,
           category = "agent", icon = "sigma-agent"): bool =
  let urg_str = case urgency
    of UrgLow:      "low"
    of UrgCritical: "critical"
    else:           "normal"

  let n = Notification(
    id:       $now().toTime.toUnix,
    ts:       $now(),
    title:    title,
    body:     body,
    urgency:  urgency,
    category: category,
    icon:     icon,
    sent:     false,
  )

  # Try backends in priority order
  var sent = false

  # 1. sigma-notify (SigmaOS native daemon)
  if not sent:
    let (_, code) = execCmdEx(
      fmt"sigma-notify --title {title.quoteShell} --body {body.quoteShell} --urgency {urg_str} 2>/dev/null")
    if code == 0: sent = true

  # 2. notify-send (freedesktop)
  if not sent:
    let (_, code) = execCmdEx(
      fmt"notify-send -u {urg_str} -i {icon.quoteShell} {title.quoteShell} {body.quoteShell} 2>/dev/null")
    if code == 0: sent = true

  # 3. Terminal bell + inline (always works)
  if not sent:
    let urgency_icon = case urgency
      of UrgCritical: "🔴"
      of UrgLow:      "ℹ"
      else:           "🔔"
    stdout.write(fmt"\a")  # terminal bell
    echo fmt"\n{urgency_icon} {title}: {body}\n"
    sent = true

  append_log(Notification(id: n.id, ts: n.ts, title: n.title, body: n.body,
                           urgency: n.urgency, category: n.category,
                           icon: n.icon, sent: sent))
  sent

# ── Event subscription (poll-based) ───────────────────────────────────────────
type EventRule = object
  name:      string
  condition: string   # shell command — if exit 0, trigger
  title:     string
  body_cmd:  string   # shell command whose stdout is the body
  urgency:   Urgency
  interval:  int      # check every N seconds

const DEFAULT_RULES: array[5, EventRule] = [
  EventRule(name: "build_done",
            condition: "test -f /tmp/sigma_build_done",
            title:     "Build Complete",
            body_cmd:  "cat /tmp/sigma_build_done 2>/dev/null || echo 'Build finished'",
            urgency:   UrgNormal, interval: 5),
  EventRule(name: "high_cpu",
            condition: "awk '{if($1>90) exit 0; exit 1}' /proc/loadavg 2>/dev/null",
            title:     "High CPU Load",
            body_cmd:  "awk '{print \"Load: \"$1\" (1m)  \"$2\" (5m)\"}' /proc/loadavg",
            urgency:   UrgCritical, interval: 30),
  EventRule(name: "low_disk",
            condition: "df / | awk 'NR==2{gsub(\"%\",\"\",$5); if($5>90) exit 0; exit 1}'",
            title:     "Low Disk Space",
            body_cmd:  "df -h / | tail -1 | awk '{print \"Root: \"$4\" free\"}'",
            urgency:   UrgCritical, interval: 60),
  EventRule(name: "pkg_updates",
            condition: "sigma-pkg check-updates 2>/dev/null | grep -q update",
            title:     "Updates Available",
            body_cmd:  "sigma-pkg check-updates 2>/dev/null | head -3",
            urgency:   UrgLow, interval: 3600),
  EventRule(name: "agent_daemon_down",
            condition: "! curl -sf http://localhost:11430/v1/status --max-time 1 >/dev/null 2>&1",
            title:     "sigma-agent daemon stopped",
            body_cmd:  "echo 'Run: sigma-agent daemon start'",
            urgency:   UrgLow, interval: 300),
]

proc watch_events*(rules: seq[EventRule] = @[], duration_secs = 0) =
  ## Poll event rules and send notifications when conditions are met
  let active_rules = if rules.len > 0: rules else: DEFAULT_RULES.toSeq
  var last_fired: Table[string, int64]
  let start = now().toTime.toUnix
  echo "\e[38;2;69;243;255mσ sigma-agent notify: watching for events (Ctrl+C to stop)\e[0m"
  for r in active_rules:
    echo fmt"  Monitoring: {r.name} (every {r.interval}s)"
  echo ""
  while true:
    let now_ts = now().toTime.toUnix
    if duration_secs > 0 and now_ts - start > duration_secs: break
    for r in active_rules:
      let last = last_fired.getOrDefault(r.name, 0)
      if now_ts - last < r.interval: continue
      let (_, code) = execCmdEx("sh -c " & r.condition.quoteShell & " 2>/dev/null")
      if code == 0:
        let (body_out, _) = execCmdEx("sh -c " & r.body_cmd.quoteShell & " 2>/dev/null")
        discard send(r.title, body_out.strip(), r.urgency, r.name)
        last_fired[r.name] = now_ts
        echo fmt"[{$now()}] Fired: {r.name} — {r.title}"
    sleep(2000)

# ── CLI ────────────────────────────────────────────────────────────────────────
proc notify_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "help":
    echo """sigma-agent notify — Desktop notifications + event subscriptions

Usage:
  sigma-agent notify "Title" "Body"            Send a notification
  sigma-agent notify "Title" "Body" --critical Send critical alert
  sigma-agent notify "Title" "Body" --low      Send low-priority info
  sigma-agent notify history                   Show notification history
  sigma-agent notify watch                     Start event watcher
  sigma-agent notify watch --duration 60       Watch for 60 seconds
  sigma-agent notify clear                     Clear notification history

Examples:
  sigma-agent notify "Build done" "cargo build completed"
  sigma-agent notify "Security alert" "Auth failure detected" --critical
  sigma-agent notify watch
  sigma-agent notify history
"""
    return

  case args[0].toLowerAscii
  of "history","log":
    let notifs = load_history(30)
    if notifs.len == 0:
      echo "No notification history"; return
    echo "\e[38;2;69;243;255mNotification history:\e[0m"
    for n in notifs:
      let urgency_icon = case n.urgency
        of UrgCritical: "🔴"
        of UrgLow:      "ℹ"
        else:           "🔔"
      let sent_str = if n.sent: "\e[38;2;52;211;153m✓\e[0m" else: "\e[38;2;248;113;113m✗\e[0m"
      echo fmt"  {sent_str} {urgency_icon} [{n.ts[0..<16]}] {n.title}: {n.body[0..<min(60,n.body.len)]}"
  of "watch":
    let dur_idx = args.find("--duration")
    let dur = if dur_idx >= 0 and dur_idx+1 < args.len:
                try: parseInt(args[dur_idx+1]) except: 0
              else: 0
    watch_events(duration_secs=dur)
  of "clear":
    removeFile(notif_log_path())
    echo "✓ Notification history cleared"
  else:
    # Positional: title body [--critical|--low]
    let title   = args[0]
    let body    = if args.len > 1 and not args[1].startsWith("-"): args[1] else: ""
    let urgency = if "--critical" in args: UrgCritical
                  elif "--low" in args: UrgLow
                  else: UrgNormal
    let cat     = args.filterIt(it.startsWith("--cat")).mapIt(it[6..^1]).getOrDefault("agent")
    if send(title, body, urgency, cat):
      echo fmt"✓ Notification sent: {title}"
    else:
      echo fmt"✗ Failed to send notification"
