# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_explain.nim — Explain-everything mode
# Answers "why", "how", "what does X do" with step-by-step breakdowns.
# Educational mode with interactive depth levels.
#
# Inspiration:
#   copilot-cli ??          — explain any command
#   Claude Code explain     — explain code / errors / concepts
#   ai-shell explain        — "what does this error mean?"
#   Aider /ask              — freeform Q&A about codebase
#   Hermes IDE explain      — context-aware explanation
#   ClaudeCode-Leak system  — structured explanation format
#
# Modes:
#   command:  explain what a command does before running it
#   error:    explain why a command failed and suggest fix
#   concept:  explain an OS/SigmaOS concept
#   code:     explain a code file/snippet
#   pipeline: explain a multi-step sequence
#
# Language: Nim (stdlib only)

import std/[os, osproc, strutils, strformat, json, sequtils, tables]

# ── Built-in knowledge base (offline explanations) ───────────────────────────
const BUILTIN_EXPLAINS: array[45, (string, string)] = [
  ("sigma_pledge",
   """sigma_pledge is SigmaOS's capability restriction syscall, inspired by OpenBSD pledge().

How it works:
  1. A process calls sigma_pledge(capabilities) at startup
  2. The kernel records the allowed capability set
  3. Any syscall outside the set is blocked (SIGKILL or SIGABRT)
  4. Once pledged, the process can only REDUCE capabilities, never gain more

Example: sigma_pledge("stdio rpath") allows file reads and stdio only.

Why it matters: Even if an attacker exploits a bug, they're confined to the
declared capabilities. A text editor that pledges "stdio rpath wpath" can never
open a network socket or execute other programs, even if compromised."""),

  ("sigma_unveil",
   """sigma_unveil restricts filesystem visibility — inspired by OpenBSD unveil().

How it works:
  1. Process calls sigma_unveil(path, permissions) for each allowed path
  2. All OTHER paths become invisible (ENOENT — as if they don't exist)
  3. Useful combined with sigma_pledge for double isolation

Example: sigma_unveil("/home/user/documents", "rw") makes only that directory accessible.

Why it matters: A compromised process can't read /etc/passwd or /home/other_user
even with full filesystem permissions, because those paths simply don't exist."""),

  ("sigma-pkg",
   """sigma-pkg is the SigmaOS sovereign package manager.

Key features:
  - .sigpkg format: signed with Dilithium-5 (post-quantum signature)
  - All packages verified before installation
  - Delta updates (only changed blocks downloaded)
  - Compatible with flatpak, AppImage, snap (via compat layer)
  - Atomic installs: package either installs fully or not at all

Commands:
  sigma-pkg install <name>   Install a package
  sigma-pkg remove <name>    Remove a package
  sigma-pkg update           Update all packages
  sigma-pkg list             List installed packages
  sigma-pkg search <query>   Search the registry
  sigma-pkg info <name>      Show package metadata"""),

  ("sigma-sh",
   """sigma-sh is the Sovereign Shell — SigmaOS's default interactive shell.

Built in Rust for safety. POSIX-compatible with extensions:
  - sigma_pledge integration (shell runs pledged by default)
  - Native sigma-agent integration (Ctrl+K → AI suggestions)
  - Variables, pipes, redirects, conditionals — standard POSIX
  - .sigma_profile (like .bashrc) and .sigma_rc

The shell is designed to be transparent about what it does:
  - Every command is logged to ~/.sigma_sh_history
  - `sigma-sh audit` shows what programs were run with what arguments"""),

  ("paging",
   """Virtual memory paging maps process addresses to physical RAM.

How it works:
  1. Each process sees a virtual address space (e.g. 0 → 48TB on x86_64)
  2. The MMU (Memory Management Unit) translates virtual → physical on each access
  3. The page table (managed by the kernel) holds these mappings
  4. If a virtual page isn't in RAM, a page fault fires → kernel loads it from disk

SigmaOS uses 4-level paging (PML4 → PDPT → PD → PT) with:
  - ASLR: 42-bit entropy per VMA (makes exploits harder)
  - W^X enforcement: no page is simultaneously writable AND executable
  - Per-process page tables: processes are isolated from each other

Why it matters: Without paging, all processes share RAM directly.
With paging, process A can't read/write process B's memory."""),

  ("shard",
   """A shard is SigmaOS's atomic capability module — the unit of modularity.

SigmaOS has 600+ shards, identified as S01_Genesis through S500+.
Each shard:
  - Has a single clear responsibility
  - Can be loaded/unloaded at runtime without reboot
  - Communicates via sigma-bus (typed IPC, no raw pointers)
  - Has its own test suite, CI job, and documentation

Shards replace monolithic subsystems. Instead of a huge "networking" module,
you have: S07_Network_IPv4, S07_Network_DNS, S07_Network_TLS, etc.

Key shards:
  S01 Genesis:       The minimal bootable core
  S02 ZenithUI:      Desktop environment
  S08 Security:      pledge/unveil/AVC
  S09 Intelligence:  sigma-ai LLM inference
  S36 SovereignPkg:  Package manager"""),

  ("buddy allocator",
   """The buddy allocator manages physical memory pages in the kernel.

How it works:
  1. Memory is divided into blocks that are powers of 2 in size (1, 2, 4, 8... pages)
  2. Each block size has a free list (linked list of available blocks)
  3. Allocate N pages → find the smallest 2^k ≥ N block → split if needed
  4. Free a block → check if its "buddy" (mirror block) is also free → merge

Why it's efficient:
  - Alloc: O(log N) — walk the free lists upward
  - Free: O(log N) — potentially merge with buddy
  - Low fragmentation: blocks naturally coalesce back into larger blocks

SigmaOS buddy allocator: kernel/memory/buddy.rs — handles 4KB–2MB page groups"""),

  ("sigma-bus",
   """sigma-bus is SigmaOS's typed IPC mechanism between shards.

Unlike raw Unix pipes or sockets, sigma-bus:
  - Is typed (both sides agree on message schema at compile time)
  - Is zero-copy for large messages (shared memory with capability tokens)
  - Has built-in backpressure (slow consumers don't crash fast producers)
  - Authenticates senders via sigma_pledge capabilities

Usage:
  // Send
  sigma_bus_send(BUS_NETWORK, &msg, sizeof(msg));
  // Receive
  sigma_bus_recv(BUS_NETWORK, &msg, TIMEOUT_MS);

Channels: BUS_NETWORK, BUS_AUDIO, BUS_INPUT, BUS_DISPLAY, BUS_AI, etc."""),

  ("mlfq",
   """MLFQ (Multi-Level Feedback Queue) is SigmaOS's default CPU scheduler.

How it works:
  1. Four queues with decreasing priority: Q0 (highest) → Q3 (lowest)
  2. New processes start in Q0 (highest priority)
  3. If a process uses its full time slice, it moves DOWN a queue (CPU-bound)
  4. If a process yields early (I/O wait), it stays or moves UP (interactive)
  5. Aging: processes in low queues are periodically bumped UP to prevent starvation

Why MLFQ?
  - Interactive tasks (UI, typing) get fast response (high queue)
  - Background tasks (compiling, backups) run in low queues
  - No need to know task behavior in advance — it learns automatically

SigmaOS also has: CFS (fair sharing), EDF (real-time), AI predictive pre-warming"""),

  ("sigma-agent",
   """sigma-agent is SigmaOS's native AI CLI agent.

It maps natural language → OS operations. Every GUI action has a CLI equivalent:
  GUI: Settings → Appearance → Dark Mode
  CLI: sigma-agent "set dark mode"

Architecture:
  User Input → IntentParser → ReAct Planner → LLM Backend → Tool Executor

LLM backends (priority order):
  1. sigma-ai daemon (/run/sigma/ai.sock) — sovereign, always-on
  2. Ollama (localhost:11434) — easy setup
  3. llama.cpp (GGUF model) — any model
  4. NullBackend — offline fallback with built-in responses

21 built-in tools: files, shell, packages, apps, settings, system, network,
processes, accessibility, VPN, disk, code editing, AI explain, notifications...

Run: sigma-agent doctor  — check all components are working"""),

  ("post-quantum cryptography",
   """Post-quantum cryptography (PQC) resists attacks from quantum computers.

Classical crypto (RSA, ECDH) can be broken by Shor's algorithm on a quantum computer.
PQC algorithms are hard even for quantum computers.

SigmaOS PQC stack:
  Key exchange: Kyber-1024 (KEM — Key Encapsulation Mechanism)
  Signatures:   Dilithium-5 (used for package signing, kernel attestation)
  Hybrid mode:  X25519 + Kyber-1024 in TLS 1.3 (classical + quantum-safe)

Where SigmaOS uses PQC:
  - TLS connections (HTTPS, sigma-bus encrypted channels)
  - Package signatures (every .sigpkg signed with Dilithium-5)
  - Boot attestation (TPM2 PCR chain + Dilithium)
  - sigma-vault secrets (encrypted with Kyber-1024 hybrid)"""),
]

proc builtin_explain(query: string): string =
  let lower = query.toLowerAscii
  for (topic, explanation) in BUILTIN_EXPLAINS:
    if topic.toLowerAscii in lower or lower in topic.toLowerAscii:
      return explanation
  # Fuzzy: word overlap
  var best_score = 0; var best_expl = ""
  let q_words = lower.split()
  for (topic, explanation) in BUILTIN_EXPLAINS:
    let t_words = topic.toLowerAscii.split()
    let score = q_words.filterIt(it in t_words).len
    if score > best_score: best_score = score; best_expl = explanation
  if best_score > 0: return best_expl
  ""

# ── LLM-powered explanation ────────────────────────────────────────────────────
proc explain_with_llm(query, explain_type: string): string =
  let system = case explain_type
    of "command":
      "You are sigma-agent. Explain what this SigmaOS command does in 3-5 sentences. Be concrete and include an example output."
    of "error":
      "You are sigma-agent. Explain this error message clearly: what went wrong, why it happened, and how to fix it. Keep it under 150 words."
    of "concept":
      "You are sigma-agent, a SigmaOS expert. Explain this concept clearly with: 1) What it is, 2) How it works (briefly), 3) Why it matters for SigmaOS. Use concrete examples."
    of "code":
      "You are sigma-agent. Explain what this code does in plain English. Focus on the key logic and purpose."
    else:
      "You are sigma-agent, a SigmaOS expert. Answer this question concisely and accurately."

  # Try daemon
  let daemon_up = execCmdEx("curl -sf http://localhost:11430/v1/status --max-time 1 2>/dev/null")[1] == 0
  if daemon_up:
    let body = $ %*{"message": query, "max_tokens": 300, "include_context": false}
    let (out, code) = execCmdEx(
      fmt"""curl -sf -X POST http://localhost:11430/v1/chat -d {body.quoteShell} --max-time 10""")
    if code == 0:
      try: return parseJson(out).getOrDefault("response").getStr("") except: discard

  # Try Ollama
  let prompt = fmt"{system}\n\n{query}"
  let ollama_body = $ %*{"model":"tinyllama","prompt":prompt,"stream":false,
                          "options":{%*{"num_predict":300,"temperature":0.2}}}
  let (out, code) = execCmdEx(
    fmt"""curl -sf -X POST http://localhost:11434/api/generate -d {ollama_body.quoteShell} --max-time 15 2>/dev/null""")
  if code == 0:
    try: return parseJson(out).getOrDefault("response").getStr("") except: discard
  ""

# ── Command explainer (like copilot-cli ??) ────────────────────────────────────
proc explain_command*(cmd: string): string =
  ## Explain what a shell command does before running it
  let lower = cmd.toLowerAscii.strip()

  # Well-known sigma-* commands
  if lower.startsWith("sigma-pkg install"):
    let pkg = cmd.split().getOrDefault(2, "?")
    return fmt"Installs the package '{pkg}' using sigma-pkg, SigmaOS's package manager. The package will be downloaded, signature-verified (Dilithium-5), and installed atomically."

  if lower.startsWith("sigma-agent"):
    return "Runs sigma-agent, SigmaOS's AI CLI agent. It maps natural language to OS operations and can perform any task that the Zenith Desktop GUI can do."

  if lower.startsWith("sigma-netctl"):
    return "Manages network interfaces using sigma-netctl. Can list interfaces, connect to Wi-Fi, set DNS, configure static IPs, and check connection status."

  # Built-in knowledge
  let builtin = builtin_explain(cmd)
  if builtin.len > 0: return builtin

  # LLM fallback
  let llm = explain_with_llm(fmt"What does this command do: {cmd}", "command")
  if llm.len > 0: return llm

  fmt"Runs: {cmd}"

# ── Error explainer ────────────────────────────────────────────────────────────
proc explain_error*(cmd, error_output: string): string =
  ## Explain a command failure and suggest a fix
  let lower = error_output.toLowerAscii

  # Common error patterns
  if "permission denied" in lower:
    return fmt"""Permission denied running: {cmd}

Why: The current user doesn't have the required file/directory permissions.
Fix: 
  - Check file ownership: ls -la <path>
  - Run with elevated permissions: sudo {cmd}
  - Or adjust permissions: chmod +x <file>
  - If sigma_pledge is active, the process may lack the required capability."""

  if "command not found" in lower or "no such file" in lower:
    let missing = cmd.split()[0]
    return fmt"""Command not found: '{missing}'

Why: '{missing}' is not installed or not in PATH.
Fix:
  sigma-pkg install {missing}     # if it's a SigmaOS package
  sigma-agent "what is the sigma-os command for: {missing}"  # AI suggestion
  echo $PATH                      # check PATH"""

  if "connection refused" in lower:
    return fmt"""Connection refused while running: {cmd}

Why: The target service is not running or is blocked.
Fix:
  1. Check if service is running: sigma-agent "show processes"
  2. Check firewall: sigma-agent "settings get network firewall"  
  3. Verify the port: ss -tlnp | grep <port>
  4. Start the service: sigma-agent "open app <service-name>" """

  if "out of memory" in lower or "oom" in lower:
    return """Out of memory error.

Why: The system ran out of available RAM.
Fix:
  sigma-agent "system info"           # check memory usage
  sigma-agent "show processes"        # find memory-hungry processes
  sigma-agent "kill process <pid>"    # kill the biggest offender
  run swapon /swapfile                # enable swap if available"""

  # LLM fallback
  let llm = explain_with_llm(
    fmt"Command failed: {cmd}\nError: {error_output[0..<min(300,error_output.len)]}\nExplain what went wrong and how to fix it.",
    "error")
  if llm.len > 0: return llm
  fmt"Command '{cmd}' failed: {error_output[0..<min(200,error_output.len)]}"

# ── Interactive "what does this do?" mode ─────────────────────────────────────
proc interactive_explain*(no_color = false) =
  let CYAN  = if no_color: "" else: "\e[38;2;69;243;255m"
  let MUTED = if no_color: "" else: "\e[38;2;107;114;128m"
  let RESET = if no_color: "" else: "\e[0m"
  echo fmt"{CYAN}σ sigma-agent explain mode{RESET}"
  echo fmt"{MUTED}Type a command, concept, or error to explain. 'quit' to exit.{RESET}\n"
  while true:
    stdout.write(fmt"{CYAN}explain>{RESET} ")
    stdout.flushFile()
    var line = ""
    try: line = stdin.readLine().strip() except EOFError, IOError: break
    if line.len == 0: continue
    if line.toLowerAscii in ["quit","exit","q"]: break
    let explanation = if "error" in line.toLowerAscii or line.startsWith("why"):
                        explain_with_llm(line, "error")
                      elif line.startsWith("what is") or line.startsWith("explain") or
                           line.startsWith("how does"):
                        let topic = line.split(' ')[2..^1].join(" ")
                        let b = builtin_explain(topic)
                        if b.len > 0: b else: explain_with_llm(line, "concept")
                      else:
                        explain_command(line)
    echo fmt"\n{explanation}\n"

# ── CLI ────────────────────────────────────────────────────────────────────────
proc explain_cmd*(args: seq[string]) =
  if args.len == 0:
    interactive_explain()
    return

  if args[0] == "help":
    echo """sigma-agent explain — Explain commands, concepts, and errors

Usage:
  sigma-agent explain                      Interactive explain mode
  sigma-agent explain "<command>"          Explain a command
  sigma-agent explain --error "<cmd>" "<err>"  Explain a failure
  sigma-agent explain --concept "<topic>"  Explain an OS concept
  sigma-agent explain --code "<file>"      Explain a code file
  sigma-agent explain --list               List built-in topics

Built-in topics (no LLM needed):
  sigma_pledge, sigma_unveil, sigma-pkg, sigma-sh, paging, shard,
  buddy allocator, sigma-bus, mlfq, sigma-agent, post-quantum cryptography

Examples:
  sigma-agent explain "sigma-pkg install sigma-edit"
  sigma-agent explain --concept "how paging works"
  sigma-agent explain --error "cargo build" "permission denied"
  sigma-agent explain --code src/main.rs
  sigma-agent explain "what does sigma_pledge do"
"""
    return

  if args[0] == "--list":
    echo "Built-in explanations:"
    for (topic, _) in BUILTIN_EXPLAINS:
      echo fmt"  {topic}"
    return

  if args[0] == "--error":
    let cmd = if args.len > 1: args[1] else: ""
    let err = if args.len > 2: args[2..^1].join(" ") else: ""
    echo explain_error(cmd, err)
    return

  if args[0] == "--concept":
    let topic = args[1..^1].join(" ")
    let b = builtin_explain(topic)
    if b.len > 0: echo b
    else:
      let llm = explain_with_llm(topic, "concept")
      if llm.len > 0: echo llm
      else: echo fmt"(No explanation found for: {topic}. Install sigma-ai for full explanations.)"
    return

  if args[0] == "--code":
    let path = if args.len > 1: args[1] else: ""
    if path.len == 0 or not fileExists(path):
      echo "Usage: sigma-agent explain --code <file>"; return
    let content = readFile(path)
    let llm = explain_with_llm(
      fmt"Explain this code file briefly:\n```\n{content[0..<min(2000,content.len)]}\n```",
      "code")
    if llm.len > 0: echo llm
    else: echo fmt"(sigma-ai not available. Install: sigma-pkg install sigma-ai)"
    return

  # Default: explain the query as a command or concept
  let query = args.filterIt(not it.startsWith("-")).join(" ")
  let lower = query.toLowerAscii

  # Check if it sounds like a concept question
  let is_concept = lower.startsWith("what") or lower.startsWith("how") or
                   lower.startsWith("why") or lower.startsWith("explain")

  let explanation = if is_concept:
    let b = builtin_explain(query)
    if b.len > 0: b else: explain_with_llm(query, "concept")
  else:
    let b = builtin_explain(query)
    if b.len > 0: b else: explain_command(query)

  if explanation.len > 0: echo explanation
  else: echo fmt"(No explanation available. Try: sigma-agent explain --concept \"{query}\")"
