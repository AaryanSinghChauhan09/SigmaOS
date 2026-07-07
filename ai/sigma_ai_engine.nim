# sigma_ai_engine.nim
# SigmaOS AI Intent Engine — Nim implementation
# Replaces Python AI daemon stubs (sigmad/sigma_ai_daemon.py).
# Compiles to a native binary with --gc:none (zero GC, zero Python dependency).
# Provides: NL→CLI intent parsing, dry-run sandbox, policy enforcement.

import std/[strutils, tables, os]

# ── Types ─────────────────────────────────────────────────────────────────

type
  RiskLevel = enum
    rlSafe, rlModerate, rlDestructive

  Intent = object
    phrase:      string
    command:     string
    description: string
    risk:        RiskLevel

# ── Intent Database ───────────────────────────────────────────────────────
# Hard-coded intent table — zero runtime library needed.

const INTENTS: array[20, Intent] = [
  Intent(phrase: "install",        command: "sigpkg install",      description: "Install package",         risk: rlSafe),
  Intent(phrase: "remove",         command: "sigpkg remove",       description: "Remove package",          risk: rlModerate),
  Intent(phrase: "update",         command: "sigpkg update",       description: "Update all packages",     risk: rlModerate),
  Intent(phrase: "rollback",       command: "sigpkg rollback",     description: "Rollback transaction",    risk: rlModerate),
  Intent(phrase: "list packages",  command: "sigpkg list",         description: "List packages",           risk: rlSafe),
  Intent(phrase: "disk usage",     command: "df -h",               description: "Show disk usage",         risk: rlSafe),
  Intent(phrase: "memory",         command: "free -h",             description: "Show memory",             risk: rlSafe),
  Intent(phrase: "cpu",            command: "cat /proc/cpuinfo",   description: "Show CPU info",           risk: rlSafe),
  Intent(phrase: "processes",      command: "ps aux",              description: "List processes",          risk: rlSafe),
  Intent(phrase: "network",        command: "ip addr",             description: "Show network interfaces", risk: rlSafe),
  Intent(phrase: "firewall",       command: "sigpkg list firewall",description: "Check firewall rules",    risk: rlSafe),
  Intent(phrase: "shutdown",       command: "shutdown -h now",     description: "Shutdown system",         risk: rlDestructive),
  Intent(phrase: "reboot",         command: "reboot",              description: "Reboot system",           risk: rlDestructive),
  Intent(phrase: "format",         command: "mkfs.ext4",           description: "Format filesystem",       risk: rlDestructive),
  Intent(phrase: "delete",         command: "rm -rf",              description: "Delete files",            risk: rlDestructive),
  Intent(phrase: "kernel version", command: "uname -r",            description: "Print kernel version",    risk: rlSafe),
  Intent(phrase: "uptime",         command: "uptime",              description: "Show system uptime",      risk: rlSafe),
  Intent(phrase: "logs",           command: "journalctl -n 50",    description: "Show recent logs",        risk: rlSafe),
  Intent(phrase: "sandbox",        command: "sigma-sandbox create",description: "Create app sandbox",      risk: rlModerate),
  Intent(phrase: "help",           command: "sigpkg --help",       description: "Show help",               risk: rlSafe),
]

# ── Matcher ───────────────────────────────────────────────────────────────

proc matchIntent(input: string): ptr Intent =
  let lower = input.toLowerAscii()
  for i in 0 ..< INTENTS.len:
    if lower.contains(INTENTS[i].phrase):
      return unsafeAddr INTENTS[i]
  return nil

# ── Policy Enforcement ─────────────────────────────────────────────────────

proc enforcePolicy(intent: ptr Intent): bool =
  if intent.risk == rlDestructive:
    stdout.write("\e[33m[SIGMA-AI] ⚠ Destructive action detected: " & intent.description & "\e[0m\n")
    stdout.write("\e[33m[SIGMA-AI] Confirm execution? [y/N]: \e[0m")
    let answer = stdin.readLine()
    return answer.toLowerAscii() == "y"
  return true

# ── Dry-Run Sandbox ───────────────────────────────────────────────────────

proc dryRun(intent: ptr Intent) =
  stdout.write("\e[36m[DRY-RUN] Would execute: \e[1m" & intent.command & "\e[0m\n")
  stdout.write("\e[36m[SIGMA-AI] Description: " & intent.description & "\e[0m\n")

# ── Main REPL ──────────────────────────────────────────────────────────────

proc main =
  stdout.write("\e[1;36mSigmaOS AI Intent Engine v0.1\e[0m\n")
  stdout.write("Type in plain English. Type 'exit' to quit.\n\n")

  while true:
    stdout.write("\e[32mσ-ai\e[0m> ")
    flushFile(stdout)

    let line = stdin.readLine()
    if line == "exit" or line == "quit": break
    if line.len == 0: continue

    let intent = matchIntent(line)
    if intent == nil:
      stdout.write("\e[31m[SIGMA-AI] Intent not recognized. Try rephrasing.\e[0m\n")
    else:
      if enforcePolicy(intent):
        dryRun(intent)

main()
