# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_daemon.nim — sigma-agent background daemon
# Provides: Unix socket API, HTTP REST API, GitHub knowledge sync, keep-alive LLM
#
# Inspiration:
#   Claude Code daemon mode (stdio server)
#   llama.cpp llama-server (HTTP inference API)
#   azure-cli telemetry daemon
#   Hermes IDE language server (LSP-style)
#
# Protocol (Unix socket + HTTP):
#   POST /v1/chat        — one-shot inference
#   POST /v1/execute     — execute a tool call
#   GET  /v1/status      — daemon health + backend info
#   GET  /v1/context     — current system context snapshot
#   POST /v1/feedback    — record interaction quality (RLHF)
#   POST /v1/sync        — trigger GitHub knowledge sync
#   GET  /v1/tools       — list all 20+ tools
#   GET  /v1/history     — last N conversation turns
#
# Language: Nim (std only, no external libs)

import std/[os, osproc, times, json, tables, strutils, strformat,
            asyncdispatch, asynchttpserver, asyncnet, streams, hashes]

# ── Config ────────────────────────────────────────────────────────────────────
const
  SOCKET_PATH    = "/run/sigma/agent.sock"
  HTTP_PORT      = 11430
  SYNC_INTERVAL  = 3600  # GitHub sync every hour (seconds)
  MAX_HISTORY    = 200
  VERSION        = "15.0.0"
  GITHUB_REPO    = "AaryanSinghChauhan09/SigmaOS"
  WIKI_RAW_BASE  = "https://raw.githubusercontent.com/wiki/" & GITHUB_REPO

# ── Daemon state ──────────────────────────────────────────────────────────────
type
  BackendKind = enum BkSigmaAi, BkOllama, BkLlamaCpp, BkNone

  TurnRole = enum TrUser, TrAssistant, TrTool, TrSystem

  Turn = object
    role:      TurnRole
    content:   string
    ts:        int64      # unix timestamp
    tool_name: string
    success:   bool

  DaemonState = object
    started:      DateTime
    backend:      BackendKind
    backend_name: string
    history:      seq[Turn]
    sync_ts:      int64       # last GitHub sync timestamp
    knowledge:    Table[string, string]  # topic → summary (from Wiki)
    request_count: int
    pid:           int

# ── Knowledge base (from GitHub Wiki + repo docs) ─────────────────────────────
proc fetch_wiki_page(topic: string): string =
  ## Fetch a wiki page from GitHub (requires network)
  let url = fmt"{WIKI_RAW_BASE}/{topic}.md"
  let (out, code) = execCmdEx(fmt"""curl -sf --max-time 8 "{url}" """)
  if code == 0 and out.len > 50: return out[0..<min(2000, out.len)]
  ""

proc sync_knowledge(state: var DaemonState) =
  ## Sync key wiki pages from GitHub into in-memory knowledge base
  const PAGES = [
    "sigma-agent", "Architecture-Overview", "Zenith-Desktop",
    "Security-Model", "CLI-Reference", "Package-Manager-Spec",
    "Kernel", "Networking", "Getting-Started", "Shard-Development-Guide"
  ]
  echo "[daemon] Syncing knowledge from GitHub Wiki..."
  var synced = 0
  for page in PAGES:
    let content = fetch_wiki_page(page)
    if content.len > 0:
      state.knowledge[page.toLowerAscii.replace("-"," ")] = content
      synced += 1
  state.sync_ts = getTime().toUnix
  echo fmt"[daemon] Synced {synced}/{PAGES.len} wiki pages"

# ── System context snapshot ────────────────────────────────────────────────────
proc system_context(): JsonNode =
  ## Collect live OS state for injecting into LLM context
  var ctx = newJObject()
  ctx["ts"]       = %getTime().toUnix
  ctx["hostname"] = %execCmdEx("hostname")[0].strip()

  # Load average
  try:
    let la = readFile("/proc/loadavg").split()
    ctx["load_1m"]  = %la[0]
    ctx["load_5m"]  = %la[1]
    ctx["load_15m"] = %la[2]
  except: discard

  # Memory
  try:
    var total = 0'i64; var avail = 0'i64
    for line in readFile("/proc/meminfo").splitLines():
      let parts = line.split()
      if parts.len >= 2:
        if parts[0] == "MemTotal:":     total = parseInt(parts[1])
        elif parts[0] == "MemAvailable:": avail = parseInt(parts[1])
    ctx["mem_total_mb"] = %(total div 1024)
    ctx["mem_avail_mb"] = %(avail div 1024)
    ctx["mem_used_pct"]  = %((total - avail) * 100 div (if total > 0: total else: 1))
  except: discard

  # Top 5 processes by name
  let (ps_out, _) = execCmdEx("ps aux --sort=-%cpu 2>/dev/null | tail -n +2 | head -5 | awk '{print $11}'")
  ctx["top_procs"] = %ps_out.strip().splitLines()

  # Disk
  let (df_out, _) = execCmdEx("df -h / 2>/dev/null | tail -1")
  ctx["disk_root"] = %df_out.strip()

  # Uptime
  try:
    let up = readFile("/proc/uptime").split()[0]
    let secs = parseInt(up.split(".")[0])
    ctx["uptime_h"] = %(secs div 3600)
  except: discard

  # Active network interfaces
  let (ip_out, _) = execCmdEx("ip -brief addr 2>/dev/null | grep ' UP ' | awk '{print $1, $3}'")
  ctx["net_ifaces"] = %ip_out.strip()

  ctx

# ── Backend detection ──────────────────────────────────────────────────────────
proc detect_backend(): (BackendKind, string) =
  if fileExists("/run/sigma/ai.sock"):   return (BkSigmaAi, "sigma-ai daemon")
  let (_, code) = execCmdEx("curl -sf http://localhost:11434/api/tags --max-time 2")
  if code == 0:                          return (BkOllama,   "Ollama (localhost:11434)")
  let (_, code2) = execCmdEx("which llama-cli 2>/dev/null")
  if code2 == 0:                         return (BkLlamaCpp, "llama.cpp (llama-cli)")
  (BkNone, "none (offline fallback)")

# ── Inference call ─────────────────────────────────────────────────────────────
proc call_llm(state: DaemonState, messages: seq[JsonNode], max_tokens = 512): string =
  case state.backend
  of BkSigmaAi:
    # JSON over Unix socket → /run/sigma/ai.sock
    let payload = $ %*{"messages": messages, "max_tokens": max_tokens}
    let (out, code) = execCmdEx(
      fmt"""echo {payload.quoteShell} | nc -U /run/sigma/ai.sock 2>/dev/null""")
    if code == 0 and out.len > 0:
      try:
        let j = parseJson(out)
        return j.getOrDefault("content").getStr(out.strip())
      except: return out.strip()

  of BkOllama:
    let msgs_json = $ %messages
    let body = fmt"""{{ "model":"tinyllama","messages":{msgs_json},"stream":false,"options":{{"num_predict":{max_tokens}}} }}"""
    let (out, code) = execCmdEx(
      fmt"""curl -sf -X POST http://localhost:11434/api/chat -H 'Content-Type: application/json' -d {body.quoteShell} --max-time 30""")
    if code == 0:
      try:
        let j = parseJson(out)
        return j["message"]["content"].getStr()
      except: return out[0..<min(200, out.len)]

  of BkLlamaCpp:
    let last_user = messages.filterIt(it["role"].getStr == "user").mapIt(it["content"].getStr).join(" ")
    let (out, code) = execCmdEx(
      fmt"""llama-cli -p {last_user.quoteShell} --n-predict {max_tokens} --log-disable --no-display-prompt 2>/dev/null""")
    if code == 0: return out.strip()

  of BkNone:
    let last = messages.filterIt(it["role"].getStr == "user")
    if last.len > 0:
      return fmt"(offline) I understood: \"{last[^1][\"content\"].getStr[0..<min(60, last[^1][\"content\"].getStr.len)]}\" — install sigma-ai for live inference."
    return "(offline) No LLM backend available. Install: sigma-pkg install sigma-ai"

  "No response from backend"

# ── Request handler ────────────────────────────────────────────────────────────
proc handle_request(state: var DaemonState, req: JsonNode): JsonNode =
  let path    = req.getOrDefault("path").getStr("/v1/chat")
  let method  = req.getOrDefault("method").getStr("POST").toUpperAscii
  let body    = req.getOrDefault("body")

  state.request_count += 1

  case path
  # ── GET /v1/status ────────────────────────────────────────────────────────
  of "/v1/status":
    return %*{
      "status":       "running",
      "version":      VERSION,
      "backend":      state.backend_name,
      "requests":     state.request_count,
      "history_len":  state.history.len,
      "knowledge_pages": state.knowledge.len,
      "last_sync":    state.sync_ts,
      "pid":          state.pid,
      "uptime_s":     (getTime() - state.started.toTime).inSeconds,
    }

  # ── GET /v1/context ───────────────────────────────────────────────────────
  of "/v1/context":
    return system_context()

  # ── GET /v1/tools ─────────────────────────────────────────────────────────
  of "/v1/tools":
    let (out, _) = execCmdEx("sigma-agent-core tools 2>/dev/null")
    return %*{"tools": out.strip()}

  # ── GET /v1/history ───────────────────────────────────────────────────────
  of "/v1/history":
    let n = body.getOrDefault("n").getInt(10)
    var arr = newJArray()
    for t in state.history[max(0, state.history.len - n)..^1]:
      arr.add(%*{"role": $t.role, "content": t.content[0..<min(200, t.content.len)], "ts": t.ts})
    return %*{"history": arr}

  # ── POST /v1/chat ─────────────────────────────────────────────────────────
  of "/v1/chat":
    let user_msg  = body.getOrDefault("message").getStr("")
    let sys_ctx   = body.getOrDefault("include_context").getBool(true)
    let max_tok   = body.getOrDefault("max_tokens").getInt(512)

    if user_msg.len == 0:
      return %*{"error": "message required"}

    # Build message list
    var messages: seq[JsonNode]
    let ctx_str = if sys_ctx:
      let c = system_context()
      fmt"System state: load={c[\"load_1m\"].getStr} mem_used={c[\"mem_used_pct\"].getInt}% disk={c[\"disk_root\"].getStr}"
    else: ""

    messages.add(%*{"role":"system","content":
      "You are sigma-agent, the SigmaOS sovereign AI CLI assistant. " &
      "Help users control their operating system via natural language. " &
      (if ctx_str.len > 0: "Current " & ctx_str else: "")})

    # Include relevant history (last 5 turns)
    for t in state.history[max(0, state.history.len-5)..^1]:
      messages.add(%*{"role": (if t.role == TrUser: "user" elif t.role == TrAssistant: "assistant" else: "tool"),
                       "content": t.content})

    messages.add(%*{"role":"user","content":user_msg})

    # Check knowledge base first
    let lower_msg = user_msg.toLowerAscii
    var knowledge_hint = ""
    for topic, content in state.knowledge:
      if topic in lower_msg:
        knowledge_hint = content[0..<min(500, content.len)]
        break

    if knowledge_hint.len > 0:
      messages.add(%*{"role":"system","content":
        fmt"Relevant documentation:\n{knowledge_hint}"})

    let response = call_llm(state, messages, max_tok)

    # Record in history
    state.history.add Turn(role: TrUser,      content: user_msg, ts: getTime().toUnix, success: true)
    state.history.add Turn(role: TrAssistant, content: response, ts: getTime().toUnix, success: true)
    if state.history.len > MAX_HISTORY: state.history.delete(0)

    return %*{"response": response, "backend": state.backend_name}

  # ── POST /v1/execute ──────────────────────────────────────────────────────
  of "/v1/execute":
    let cmd  = body.getOrDefault("command").getStr("")
    let dry  = body.getOrDefault("dry_run").getBool(false)
    if cmd.len == 0: return %*{"error": "command required"}
    if dry:  return %*{"dry_run": true, "command": cmd}
    let (out, code) = execCmdEx(fmt"sigma-agent-core --once {cmd.quoteShell} 2>&1")
    state.history.add Turn(role: TrTool, content: out, ts: getTime().toUnix, success: code == 0, tool_name: "execute")
    return %*{"output": out.strip(), "exit_code": code, "success": code == 0}

  # ── POST /v1/feedback ─────────────────────────────────────────────────────
  of "/v1/feedback":
    let quality  = body.getOrDefault("quality").getStr("good")  # good/bad/excellent
    let feedback = body.getOrDefault("feedback").getStr("")
    # Append to training JSONL
    let data_dir = getEnv("HOME", "/tmp") / ".cache/sigma/agent_training"
    createDir(data_dir)
    if state.history.len >= 2:
      let last_user  = state.history[^2].content
      let last_agent = state.history[^1].content
      let sample = %*{"messages":
        [%*{"role":"system","content":"You are sigma-agent, the SigmaOS AI CLI assistant."},
         %*{"role":"user","content":last_user},
         %*{"role":"assistant","content":last_agent}],
        "quality": quality, "feedback": feedback}
      let f = open(data_dir / "feedback.jsonl", fmAppend)
      f.writeLine($sample); f.close()
    return %*{"status": "recorded", "quality": quality}

  # ── POST /v1/sync ─────────────────────────────────────────────────────────
  of "/v1/sync":
    sync_knowledge(state)
    return %*{"status": "synced", "pages": state.knowledge.len, "ts": state.sync_ts}

  else:
    return %*{"error": fmt"Unknown path: {path}"}

# ── Simple line-delimited JSON protocol server ─────────────────────────────────
# Listens on Unix socket: /run/sigma/agent.sock
# Each request: one JSON line → one JSON response line

proc serve_unix_socket(state: var DaemonState) {.async.} =
  let sock_dir = SOCKET_PATH.parentDir()
  if not dirExists(sock_dir): createDir(sock_dir)
  if fileExists(SOCKET_PATH): removeFile(SOCKET_PATH)

  var server = newAsyncSocket(AF_UNIX, SOCK_STREAM, IPPROTO_IP)
  server.setSockOpt(OptReuseAddr, true)
  bindUnix(server, SOCKET_PATH)
  server.listen()
  echo fmt"[daemon] Unix socket: {SOCKET_PATH}"

  while true:
    let client = await server.accept()
    asyncCheck (proc() {.async.} =
      try:
        let line = await client.recvLine()
        if line.len > 0:
          let req  = parseJson(line)
          let resp = handle_request(state, req)
          await client.send($resp & "\n")
      except: discard
      client.close()
    )()

# ── HTTP server (for external integrations like IDE plugins) ───────────────────
proc serve_http(state: var DaemonState) {.async.} =
  var server = newAsyncHttpServer()
  echo fmt"[daemon] HTTP API: http://localhost:{HTTP_PORT}"

  proc cb(req: asynchttpserver.Request) {.async.} =
    let body_str = await req.body
    var req_j = newJObject()
    req_j["path"]   = %req.url.path
    req_j["method"] = %($req.reqMethod)
    if body_str.len > 0:
      try: req_j["body"] = parseJson(body_str) except: req_j["body"] = %body_str

    let resp = handle_request(state, req_j)
    let headers = newHttpHeaders([("Content-Type","application/json"),
                                   ("Access-Control-Allow-Origin","*")])
    await req.respond(Http200, $resp, headers)

  server.listen(Port(HTTP_PORT))
  while true: await server.acceptRequest(cb)

# ── Periodic background tasks ──────────────────────────────────────────────────
proc background_tasks(state: var DaemonState) {.async.} =
  var last_sync = 0'i64
  while true:
    await sleepAsync(60_000)  # tick every minute
    let now = getTime().toUnix
    # Hourly knowledge sync
    if now - last_sync > SYNC_INTERVAL:
      sync_knowledge(state)
      last_sync = now
    # Re-detect backend in case sigma-ai daemon came online
    let (bk, bk_name) = detect_backend()
    if bk != state.backend:
      echo fmt"[daemon] Backend changed: {state.backend_name} → {bk_name}"
      state.backend      = bk
      state.backend_name = bk_name

# ── PID file management ────────────────────────────────────────────────────────
proc write_pid(path: string) =
  let pid_dir = path.parentDir()
  if not dirExists(pid_dir): createDir(pid_dir)
  writeFile(path, $getCurrentProcessId())

proc remove_pid(path: string) =
  try: removeFile(path) except: discard

# ── Entry point ────────────────────────────────────────────────────────────────
proc daemon_main() =
  let pid_file   = "/run/sigma/agent.pid"
  let log_file   = getEnv("HOME", "/tmp") / ".cache/sigma/agent_daemon.log"

  var state = DaemonState(
    started:   now(),
    pid:       getCurrentProcessId(),
    history:   @[],
    knowledge: initTable[string, string](),
  )
  let (bk, bk_name) = detect_backend()
  state.backend      = bk
  state.backend_name = bk_name

  echo fmt"Σ sigma-agent daemon v{VERSION} starting..."
  echo fmt"  PID:     {state.pid}"
  echo fmt"  Backend: {state.backend_name}"
  echo fmt"  Socket:  {SOCKET_PATH}"
  echo fmt"  HTTP:    localhost:{HTTP_PORT}"

  write_pid(pid_file)
  createDir(log_file.parentDir())

  # Initial knowledge sync (background, don't block startup)
  proc init_sync() {.async.} =
    await sleepAsync(2000)
    sync_knowledge(state)
  asyncCheck init_sync()

  # Start servers and background tasks
  asyncCheck serve_unix_socket(state)
  asyncCheck serve_http(state)
  asyncCheck background_tasks(state)

  echo "[daemon] Ready. Listening for requests."
  runForever()

# ── CLI entry ──────────────────────────────────────────────────────────────────
proc daemon_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "start":
    daemon_main()
  elif args[0] == "stop":
    let pid_file = "/run/sigma/agent.pid"
    if fileExists(pid_file):
      let pid = readFile(pid_file).strip()
      let (_, code) = execCmdEx(fmt"kill {pid}")
      if code == 0: echo fmt"✓ sigma-agent daemon stopped (PID {pid})"
      else: echo fmt"✗ Could not stop daemon (PID {pid})"
      removeFile(pid_file)
    else: echo "sigma-agent daemon is not running"
  elif args[0] == "status":
    let (out, code) = execCmdEx(
      fmt"""curl -sf http://localhost:{HTTP_PORT}/v1/status --max-time 2""")
    if code == 0:
      try:
        let j = parseJson(out)
        echo fmt"✓ sigma-agent daemon running"
        echo fmt"  Version:  {j[\"version\"].getStr}"
        echo fmt"  Backend:  {j[\"backend\"].getStr}"
        echo fmt"  Requests: {j[\"requests\"].getInt}"
        echo fmt"  History:  {j[\"history_len\"].getInt} turns"
        echo fmt"  Knowledge:{j[\"knowledge_pages\"].getInt} pages"
        echo fmt"  Uptime:   {j[\"uptime_s\"].getInt}s"
      except: echo out
    else: echo "✗ sigma-agent daemon is not running (start with: sigma-agent daemon start)"
  elif args[0] == "sync":
    let (out, code) = execCmdEx(
      fmt"""curl -sf -X POST http://localhost:{HTTP_PORT}/v1/sync --max-time 30""")
    if code == 0: echo "✓ Knowledge synced from GitHub"
    else: echo "✗ Sync failed (daemon not running?)"
  elif args[0] == "logs":
    let log_file = getEnv("HOME", "/tmp") / ".cache/sigma/agent_daemon.log"
    if fileExists(log_file): echo readFile(log_file)
    else: echo "No daemon log file found"
  else:
    echo """sigma-agent daemon — background AI service

Usage:
  sigma-agent daemon start    Start the daemon
  sigma-agent daemon stop     Stop the daemon
  sigma-agent daemon status   Show daemon status + stats
  sigma-agent daemon sync     Force GitHub knowledge sync
  sigma-agent daemon logs     Show daemon log

HTTP API (localhost:11430):
  GET  /v1/status     Daemon health + backend info
  GET  /v1/context    Live system context snapshot
  POST /v1/chat       Inference with context injection
  POST /v1/execute    Execute a sigma-agent command
  POST /v1/feedback   Rate last interaction (RLHF)
  POST /v1/sync       Trigger GitHub knowledge sync
  GET  /v1/tools      List all 20+ tools
  GET  /v1/history    Conversation history

Unix socket: /run/sigma/agent.sock (same protocol)
"""

when isMainModule:
  daemon_main()
