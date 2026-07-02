# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_multi.nim — Multi-agent orchestration
# Routes tasks to specialised sub-agents: security, sysadmin, developer, teacher
# Each sub-agent has its own system prompt, tool subset, and trust level.
#
# Inspiration:
#   Claude Code multi-agent (sub-agents with tool access)
#   OpenClaw agent routing (intent → specialist)
#   Azure CLI service namespacing (az security / az network / az compute)
#   Hermes IDE — context-aware agent switching
#
# Architecture:
#   Orchestrator → intent classification → route to sub-agent
#   Sub-agents: security, sysadmin, developer, teacher, sys (default)
#   Each sub-agent: own system prompt + tool allowlist + trust level
#   Collaboration: sub-agents can call each other via orchestrator
#
# Language: Nim (stdlib only)

import std/[os, osproc, strutils, strformat, times, json, tables, sequtils]
import sigma_agent_security
import sigma_agent_context

# ── Sub-agent definitions ─────────────────────────────────────────────────────
type
  AgentRole = enum
    RoleSecurity, RoleSysAdmin, RoleDeveloper, RoleTeacher,
    RoleNetwork, RolePackage, RoleDefault

  SubAgent = object
    role:          AgentRole
    name:          string
    description:   string
    system_prompt: string
    tool_allowlist: seq[string]   # empty = all tools allowed
    trust_level:   string         # safe / standard / full
    temperature:   float
    keywords:      seq[string]    # routing keywords

  AgentTask = object
    input:       string
    agent:       AgentRole
    context_str: string
    response:    string
    tools_used:  seq[string]
    duration_ms: int
    success:     bool

# ── Sub-agent registry ─────────────────────────────────────────────────────────
proc make_agents(): Table[AgentRole, SubAgent] =
  result = initTable[AgentRole, SubAgent]()

  result[RoleSecurity] = SubAgent(
    role:        RoleSecurity,
    name:        "sigma-security",
    description: "Security advisor — audits, policies, threat detection",
    system_prompt: """You are sigma-security, a SigmaOS cybersecurity expert.
You specialise in: log analysis, port auditing, file permissions, firewall rules,
sigma_pledge/sigma_unveil policies, post-quantum cryptography (Kyber/Dilithium),
TPM2 attestation, and threat hunting.
Always prioritise security over convenience.
When suggesting commands, explain the security rationale.
Never suggest disabling security features without explaining the risk.""",
    tool_allowlist: @["read_file","list_dir","shell","settings","find_files",
                      "system_info","process","network"],
    trust_level:  "standard",
    temperature:  0.2,
    keywords: @["security","firewall","ssh","auth","permission","audit","threat","malware",
                "log","anomaly","vulnerability","cve","privilege","sandbox","pledge",
                "unveil","tpm","pqc","kyber","dilithium","encrypt","ssl","tls","cert",
                "password","hack","attack","intrusion","port scan"],
  )

  result[RoleSysAdmin] = SubAgent(
    role:        RoleSysAdmin,
    name:        "sigma-sysadmin",
    description: "System administrator — services, resources, maintenance",
    system_prompt: """You are sigma-sysadmin, a SigmaOS systems administrator expert.
You specialise in: process management, service control, cgroups, performance tuning,
disk management, kernel parameters, sigma-pkg, system updates, and troubleshooting.
Give precise, actionable commands. Explain what each step does.
For destructive actions, always show the command before explaining how to run it.""",
    tool_allowlist: @["shell","system_info","process","disk","network","install_package",
                      "read_file","list_dir","settings","notify"],
    trust_level:  "standard",
    temperature:  0.3,
    keywords: @["cpu","memory","ram","disk","storage","process","service","daemon","boot",
                "kernel","driver","sysctl","cgroup","namespace","mount","swap","load",
                "performance","slow","hang","crash","reboot","shutdown","update","upgrade",
                "sysadmin","admin","server","deploy","backup","restore","monitor","resource"],
  )

  result[RoleDeveloper] = SubAgent(
    role:        RoleDeveloper,
    name:        "sigma-developer",
    description: "Developer assistant — code editing, debugging, git, build tools",
    system_prompt: """You are sigma-developer, a SigmaOS software development expert.
You specialise in: Rust, Nim, Zig, Ada/SPARK code (SigmaOS languages), debugging,
git workflows, cargo/nimble builds, code review, refactoring, and test writing.
Follow SigmaOS language policy: Rust for kernel, Nim for userspace tools, Zig for HAL.
Show code diffs, not full file rewrites.
Explain why changes are needed, not just what to change.""",
    tool_allowlist: @["read_file","write_file","list_dir","shell","find_files",
                      "code_edit","explain","summarise"],
    trust_level:  "full",
    temperature:  0.1,
    keywords: @["code","rust","nim","zig","ada","spark","compile","build","cargo","nimble",
                "debug","error","fix","refactor","review","test","git","commit","push",
                "branch","diff","pr","patch","function","struct","trait","impl","module",
                "import","dependency","lint","clippy","format","rustfmt","nimpretty",
                "sdk","api","crate","library","ffi","abi","unsafe"],
  )

  result[RoleTeacher] = SubAgent(
    role:        RoleTeacher,
    name:        "sigma-teacher",
    description: "Educational mode — explains OS concepts, commands, architecture",
    system_prompt: """You are sigma-teacher, a SigmaOS educational AI assistant.
You explain OS internals, kernel concepts, and SigmaOS architecture in clear, accessible language.
Target audience: CS students, OS learners, and developers new to SigmaOS.
Structure explanations: concept → analogy → technical detail → example.
Always link to relevant wiki pages when appropriate.
Make learning interactive: after explaining, ask a comprehension question.""",
    tool_allowlist: @["read_file","list_dir","explain","summarise","find_files","system_info"],
    trust_level:  "safe",
    temperature:  0.7,
    keywords: @["explain","what is","how does","why does","teach","learn","understand",
                "concept","kernel","scheduler","mlfq","cfs","paging","virtual memory",
                "ipc","syscall","pledge","unveil","shard","sigma-bus","slab","buddy",
                "filesystem","sigmafs","ext4","vfs","process","thread","context switch",
                "interrupt","idt","gdt","tss","ring","privilege","aslr","wx","pqc"],
  )

  result[RoleNetwork] = SubAgent(
    role:        RoleNetwork,
    name:        "sigma-netops",
    description: "Network operations — interfaces, routing, VPN, DNS, firewall",
    system_prompt: """You are sigma-netops, a SigmaOS network operations expert.
You specialise in: network interfaces (sigma-netctl), Wi-Fi (WPA3/SAE),
VPN (WireGuard via sigma-vpn), DNS (DoH + DNSSEC), firewall rules,
TCP/IP diagnostics, and network performance.
Always show both the diagnostic command and the fix command together.""",
    tool_allowlist: @["network","vpn","shell","settings","system_info","notify"],
    trust_level:  "standard",
    temperature:  0.2,
    keywords: @["network","wifi","wi-fi","internet","ip","route","dns","dhcp","vpn",
                "wireguard","ping","traceroute","netstat","ss","interface","ethernet",
                "wlan","wpa","firewall","nat","proxy","bandwidth","latency","packet",
                "socket","tcp","udp","ipv4","ipv6","subnet","gateway","hostname",
                "nslookup","dig","curl","wget","http","https","port"],
  )

  result[RolePackage] = SubAgent(
    role:        RolePackage,
    name:        "sigma-pkgops",
    description: "Package management — install, update, search, audit packages",
    system_prompt: """You are sigma-pkgops, a SigmaOS package management expert.
You specialise in: sigma-pkg (native .sigpkg), flatpak, AppImage, snap compatibility,
package verification (Dilithium-5 signatures), dependency resolution,
package creation (.sigpkg recipes), and the sigma_pkg_registry.
Always verify package signatures and recommend pinned versions.""",
    tool_allowlist: @["install_package","shell","read_file","list_dir","find_files"],
    trust_level:  "standard",
    temperature:  0.3,
    keywords: @["install","package","pkg","app","application","software","update","upgrade",
                "remove","uninstall","search","list","sigpkg","flatpak","snap","appimage",
                "dependency","version","repository","registry","manifest","recipe","verify",
                "signature","checksum","source","binary"],
  )

  result[RoleDefault] = SubAgent(
    role:        RoleDefault,
    name:        "sigma-agent",
    description: "General-purpose SigmaOS AI CLI agent",
    system_prompt: """You are sigma-agent, the SigmaOS sovereign AI CLI assistant.
You can control the entire OS via natural language — every Zenith Desktop GUI action
has a CLI equivalent. Be concise. Show commands. Explain only when asked.""",
    tool_allowlist: @[],   # all tools
    trust_level:  "standard",
    temperature:  0.7,
    keywords: @[],
  )

# ── Intent classifier → route to sub-agent ────────────────────────────────────
proc classify_intent*(input: string, agents: Table[AgentRole, SubAgent]): AgentRole =
  let lower = input.toLowerAscii
  var best_role  = RoleDefault
  var best_score = 0

  for role, agent in agents:
    if role == RoleDefault: continue
    var score = 0
    for kw in agent.keywords:
      if kw in lower: score += 1
    if score > best_score:
      best_score = score
      best_role  = role

  # Require at least 1 keyword match to route away from default
  if best_score == 0: return RoleDefault
  best_role

# ── Call sigma-agent-core with sub-agent system prompt ─────────────────────────
proc run_sub_agent(agent: SubAgent, input: string,
                   context_str: string = ""): AgentTask =
  let start_ms = now().toTime.toUnix * 1000

  # Build the command for sigma-agent-core
  let trust = agent.trust_level
  let temp_sys_prompt = fmt"/tmp/sigma_agent_prompt_{agent.role}.txt"

  # Write system prompt to temp file (agent-core reads from env)
  writeFile(temp_sys_prompt, agent.system_prompt &
    (if context_str.len > 0: "\n\nSystem context: " & context_str else: ""))

  let env_line = fmt"SIGMA_AGENT_SYSTEM_PROMPT={temp_sys_prompt.quoteShell}"
  let cmd = fmt"{env_line} sigma-agent-core --trust {trust} --once {input.quoteShell} 2>&1"

  let (response, code) = execCmdEx(cmd)
  removeFile(temp_sys_prompt)

  let dur_ms = int(now().toTime.toUnix * 1000 - start_ms)
  AgentTask(
    input:      input,
    agent:      agent.role,
    context_str: context_str,
    response:   response.strip(),
    tools_used: @[],
    duration_ms: dur_ms,
    success:    code == 0,
  )

# ── Orchestrator ──────────────────────────────────────────────────────────────
type Orchestrator = object
  agents:  Table[AgentRole, SubAgent]
  verbose: bool
  history: seq[AgentTask]

proc new_orchestrator*(verbose = false): Orchestrator =
  Orchestrator(agents: make_agents(), verbose: verbose, history: @[])

proc dispatch*(orch: var Orchestrator, input: string,
               force_role: AgentRole = RoleDefault,
               include_context = true): AgentTask =
  # Collect system context
  let ctx_str = if include_context:
    let ctx = collect_context(include_all=false)
    ctx.to_prompt_string(200)
  else: ""

  # Classify intent
  let role = if force_role != RoleDefault: force_role
             else: classify_intent(input, orch.agents)

  if orch.verbose:
    echo fmt"\e[38;2;107;114;128m  [orchestrator] routing to: {orch.agents[role].name}\e[0m"

  let agent = orch.agents[role]
  var task = run_sub_agent(agent, input, ctx_str)

  # If sub-agent failed or returned empty, fall back to default
  if not task.success or task.response.len < 5:
    if orch.verbose: echo "\e[38;2;107;114;128m  [orchestrator] fallback to default agent\e[0m"
    task = run_sub_agent(orch.agents[RoleDefault], input, ctx_str)

  orch.history.add(task)
  task

# ── Collaboration: security + sysadmin working together ───────────────────────
proc diagnose*(orch: var Orchestrator, problem: string): string =
  ## Multi-agent collaborative diagnosis
  ## sysadmin gathers data, security validates findings
  echo "\e[38;2;69;243;255m  Σ Multi-agent diagnosis\e[0m"
  echo "\e[38;2;107;114;128m  → sysadmin: gathering system state...\e[0m"

  let sysadmin_task = orch.dispatch(
    fmt"diagnose this problem and gather relevant system info: {problem}",
    force_role = RoleSysAdmin)

  echo "\e[38;2;107;114;128m  → security: checking for security implications...\e[0m"
  let sec_task = orch.dispatch(
    fmt"check security implications of: {problem}. Sysadmin found: {sysadmin_task.response[0..<min(200, sysadmin_task.response.len)]}",
    force_role = RoleSecurity)

  fmt"""Σ Diagnosis: {problem}

System analysis:
{sysadmin_task.response}

Security assessment:
{sec_task.response}
"""

# ── CLI ────────────────────────────────────────────────────────────────────────
proc multi_cmd*(args: seq[string]) =
  var orch = new_orchestrator(verbose = "--verbose" in args or "-v" in args)

  if args.len == 0 or args[0] == "help":
    echo """sigma-agent multi — Multi-agent orchestration

Sub-agents:
  security    Security advisor (log scan, policies, audit)
  sysadmin    System administrator (services, resources, maintenance)
  developer   Developer assistant (code edit, debug, git, build)
  teacher     Educational mode (explains OS internals and concepts)
  netops      Network operations (interfaces, VPN, DNS, firewall)
  pkgops      Package management (install, update, audit)

Usage:
  sigma-agent multi <input>             Auto-route to best sub-agent
  sigma-agent multi --agent security    Force security agent
  sigma-agent multi --agent developer   Force developer agent
  sigma-agent multi --list              List all sub-agents
  sigma-agent multi diagnose <problem>  Multi-agent collaborative diagnosis

Examples:
  sigma-agent multi "why is my CPU at 100%?"
  sigma-agent multi "explain how sigma_pledge works"
  sigma-agent multi "fix the auth failure in logs"
  sigma-agent multi --agent developer "refactor sigma_agent.rs extract trait"
  sigma-agent multi diagnose "system is slow and network drops"
"""
    return

  # --list
  if args[0] == "--list" or args[0] == "list":
    echo "\e[38;2;69;243;255m\e[1mΣ Available sub-agents:\e[0m\n"
    for role, agent in orch.agents:
      let kw_sample = agent.keywords[0..<min(5, agent.keywords.len)].join(", ")
      echo fmt"  \e[38;2;69;243;255m{agent.name:<20}\e[0m {agent.description}"
      echo fmt"    Trust: {agent.trust_level:<10} Temp: {agent.temperature:.1f}  Keywords: {kw_sample}..."
    return

  # --agent <name>
  var force_role = RoleDefault
  var input_start = 0
  if args[0] == "--agent" and args.len > 2:
    let agent_name = args[1].toLowerAscii
    force_role = case agent_name
      of "security":  RoleSecurity
      of "sysadmin","sys","admin": RoleSysAdmin
      of "developer","dev","code": RoleDeveloper
      of "teacher","learn","edu":  RoleTeacher
      of "network","net","netops": RoleNetwork
      of "package","pkg","pkgops": RolePackage
      else: RoleDefault
    input_start = 2

  # diagnose mode
  if args[input_start] == "diagnose" and args.len > input_start + 1:
    let problem = args[input_start+1..^1].join(" ")
    echo orch.diagnose(problem)
    return

  # Normal dispatch
  let input = args[input_start..^1].join(" ")
  if input.len == 0:
    echo "Usage: sigma-agent multi <input>"; return

  let task = orch.dispatch(input, force_role)
  let agent_name = orch.agents[task.agent].name
  if orch.verbose or force_role != RoleDefault:
    echo fmt"\e[38;2;107;114;128m[{agent_name}] {task.duration_ms}ms\e[0m\n"
  echo task.response
