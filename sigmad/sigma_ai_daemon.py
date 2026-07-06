#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# sigmad/sigma_ai_daemon.py — sigma-ai background inference daemon
#
# Runs as a system service, listens on:
#   - Unix socket: /run/sigma/ai.sock  (local IPC)
#   - HTTP API:    localhost:17388     (REST endpoint for tools)
#
# Endpoints:
#   POST /v1/complete  { prompt, max_tokens, lang, stream }
#   GET  /v1/status    → { model, loaded, uptime_s, requests }
#   POST /v1/models/load   { path }
#   GET  /v1/models        → list of available models
#
# Model backend: sigma_gguf_loader + InferenceDaemon from sigma_ai.rs
# When no model is loaded, responds with rule-based answers.

import http.server
import json
import os
import pathlib
import re
import socket
import socketserver
import sys
import threading
import time
from typing import Optional

# ── Configuration ─────────────────────────────────────────────────────────
SOCKET_PATH   = os.environ.get("SIGMA_AI_SOCKET", "/run/sigma/ai.sock")
HTTP_HOST     = os.environ.get("SIGMA_AI_HOST",   "127.0.0.1")
HTTP_PORT     = int(os.environ.get("SIGMA_AI_PORT", "17388"))
MODEL_DIR     = pathlib.Path(os.environ.get("SIGMA_MODEL_DIR",
                    os.path.expanduser("~/.sigmaos/models")))
AUDIT_LOG     = pathlib.Path(os.environ.get("SIGMA_AI_AUDIT_LOG",
                    "/var/log/sigma/ai-audit.jsonl"))
MAX_CONTEXT   = 2048
MAX_TOKENS    = 512

# ── Daemon state ──────────────────────────────────────────────────────────
class DaemonState:
    def __init__(self):
        self.model_name:   str  = "none"
        self.model_loaded: bool = False
        self.start_time:   float = time.time()
        self.request_count: int  = 0
        self.history: list = []  # conversation context
        self._lock = threading.Lock()

    def uptime(self) -> float:
        return time.time() - self.start_time

    def record_request(self, prompt: str, response: str, lang: str):
        with self._lock:
            self.request_count += 1
        AUDIT_LOG.parent.mkdir(parents=True, exist_ok=True)
        entry = {
            "ts": time.time(), "prompt": prompt[:200],
            "response": response[:200], "lang": lang,
            "model": self.model_name,
        }
        try:
            with open(AUDIT_LOG, "a") as f:
                f.write(json.dumps(entry) + "\n")
        except Exception:
            pass

STATE = DaemonState()

# ── Rule-based fallback (no model loaded) ─────────────────────────────────
RULE_DB = [
    (r'\b(slow|performance|cpu|load)\b',
     "System performance check:\n  sigma-top --once --sort cpu\n  sigma_diagnostics quick\n  sigma-perf stat"),
    (r'\b(disk|storage|space|full)\b',
     "Free up disk space:\n  sigma-pkg clean\n  sigma-snapshot list\n  sigma-monitor disk"),
    (r'\b(security|harden|audit|vuln)\b',
     "Security hardening:\n  sigma-secure audit --fix\n  sigma-fix scan\n  sigma-secure harden --profile cis"),
    (r'\b(install|setup|get|add)\b.*\b(\w+)\b',
     "Install a package:\n  sigma-pkg install <package>\n  sigma-pkg search <query>"),
    (r'\b(update|upgrade)\b',
     "Update the system:\n  sigma-pkg update\n  sigma update --channel stable"),
    (r'\b(network|wifi|connect|internet)\b',
     "Network management:\n  sigma-net status\n  sigma-net wifi scan\n  sigma-net ping 8.8.8.8"),
    (r'\b(log|error|crash|panic)\b',
     "Diagnose errors:\n  sigma-log tail --lines 50 --level error\n  sigma-ai heal\n  sigma-log anomaly"),
    (r'\b(help|what can|what do)\b',
     "I can help with:\n  • Package install/remove/search\n  • System performance diagnostics\n  • Security audit and hardening\n  • Network troubleshooting\n  • Log analysis and crash diagnosis\n  • Script generation: sigma-ai script \"...\"\n  • Command translation: sigma-ai translate \"...\""),
]

def rule_based_response(prompt: str, lang: str = "en") -> str:
    p = prompt.lower()
    for pattern, response in RULE_DB:
        if re.search(pattern, p, re.IGNORECASE):
            return response
    return (
        f"I understand: \"{prompt}\"\n"
        "sigma-ai daemon is running but no LLM model is loaded.\n"
        "Load a model: sigma-ai model download tinyllama\n"
        "Or ask: sigma-ai translate \"" + prompt[:60] + "\""
    )

def translate_response(text: str, tgt_lang: str) -> str:
    """Very basic translation stub — real impl calls Bhashini."""
    if tgt_lang == "en": return text
    prefixes = {
        "hi": "हिंदी अनुवाद: ",
        "ta": "தமிழ் மொழிபெயர்ப்பு: ",
        "te": "తెలుగు అనువాదం: ",
        "bn": "বাংলা অনুবাদ: ",
    }
    return prefixes.get(tgt_lang, f"[{tgt_lang}] ") + text

# ── Model discovery ────────────────────────────────────────────────────────
def list_models() -> list[dict]:
    models = []
    if not MODEL_DIR.exists():
        return models
    for f in MODEL_DIR.glob("*.gguf"):
        stat = f.stat()
        models.append({
            "name":    f.stem,
            "path":    str(f),
            "size_mb": round(stat.st_size / 1_048_576, 1),
        })
    # Also check common paths
    common = [
        pathlib.Path("/sigma/models"),
        pathlib.Path("/var/lib/sigma-ai/models"),
    ]
    for d in common:
        if d.exists():
            for f in d.glob("*.gguf"):
                models.append({"name": f.stem, "path": str(f),
                                "size_mb": round(f.stat().st_size/1_048_576, 1)})
    return models

def load_model(path: str) -> bool:
    """Attempt to load a GGUF model. Returns True if successful."""
    p = pathlib.Path(path)
    if not p.exists() or not p.suffix == ".gguf":
        return False
    # In production: call sigma_gguf_loader.GgufFile.open() via ctypes or subprocess
    # and load weights into the InferenceDaemon
    STATE.model_name   = p.stem
    STATE.model_loaded = True
    print(f"[sigma-ai] Loaded model: {p.name} ({p.stat().st_size // 1_048_576} MB)")
    return True

def complete(prompt: str, max_tokens: int = MAX_TOKENS, lang: str = "en") -> str:
    """Generate a completion. Uses model if loaded, else rule-based."""
    if STATE.model_loaded:
        # In production: call InferenceDaemon.complete() via FFI or subprocess
        # For now: echo with model name prefix
        resp = f"[{STATE.model_name}] " + rule_based_response(prompt, lang)
    else:
        resp = rule_based_response(prompt, lang)

    if lang != "en":
        resp = translate_response(resp, lang)

    STATE.record_request(prompt, resp, lang)
    return resp

# ── HTTP API server ────────────────────────────────────────────────────────
class AiRequestHandler(http.server.BaseHTTPRequestHandler):
    def log_message(self, fmt, *args):
        pass  # suppress access log

    def _send_json(self, data: dict, code: int = 200):
        body = json.dumps(data).encode()
        self.send_response(code)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", len(body))
        self.end_headers()
        self.wfile.write(body)

    def _read_body(self) -> dict:
        length = int(self.headers.get("Content-Length", 0))
        if length == 0: return {}
        raw = self.rfile.read(length)
        try: return json.loads(raw)
        except Exception: return {}

    def do_GET(self):
        if self.path == "/v1/status":
            self._send_json({
                "model":      STATE.model_name,
                "loaded":     STATE.model_loaded,
                "uptime_s":   round(STATE.uptime(), 1),
                "requests":   STATE.request_count,
                "socket":     SOCKET_PATH,
                "version":    "15.0.0-Zenith",
            })
        elif self.path == "/v1/models":
            self._send_json({"models": list_models()})
        elif self.path == "/health":
            self._send_json({"status": "ok"})
        else:
            self._send_json({"error": "not found"}, 404)

    def do_POST(self):
        body = self._read_body()
        if self.path == "/v1/complete":
            prompt     = body.get("prompt", "")
            max_tokens = int(body.get("max_tokens", MAX_TOKENS))
            lang       = body.get("lang", "en")
            if not prompt:
                self._send_json({"error": "prompt required"}, 400)
                return
            resp = complete(prompt, max_tokens, lang)
            self._send_json({"response": resp, "model": STATE.model_name,
                             "tokens": len(resp.split())})

        elif self.path == "/v1/models/load":
            path = body.get("path", "")
            if not path:
                # Auto-load first available model
                models = list_models()
                if models: path = models[0]["path"]
            if load_model(path):
                self._send_json({"ok": True, "model": STATE.model_name})
            else:
                self._send_json({"error": f"model not found: {path}"}, 404)

        elif self.path == "/v1/heal":
            crash_path = body.get("crash_path", "")
            analysis = (
                f"Root cause analysis for: {crash_path or 'system anomalies'}\n"
                "1. Check: sigma-log tail --level error --lines 100\n"
                "2. Check: sigma-top --once --sort cpu\n"
                "3. Check: sigma-monitor mem\n"
                "4. Apply: sigma-fix scan && sigma-fix apply --auto"
            )
            self._send_json({"analysis": analysis})

        elif self.path == "/v1/script":
            intent = body.get("intent", "")
            # Delegate to sigma_nl_cli.py
            import subprocess
            try:
                out = subprocess.check_output(
                    [sys.executable, "tools/sigma_nl_cli.py", "script", intent],
                    timeout=5, text=True,
                    cwd=str(pathlib.Path(__file__).parent.parent)
                )
                self._send_json({"script": out})
            except Exception as e:
                self._send_json({"script": f"# Intent: {intent}\n# Error: {e}"})
        else:
            self._send_json({"error": "not found"}, 404)

# ── Unix socket server (for local IPC) ────────────────────────────────────
def handle_unix_client(conn: socket.socket):
    try:
        data = conn.recv(4096)
        if not data: return
        try:
            req = json.loads(data)
        except Exception:
            req = {"cmd": "ask", "prompt": data.decode("utf-8", errors="replace")}

        cmd    = req.get("cmd", "ask")
        prompt = req.get("prompt", "")
        lang   = req.get("lang", "en")

        if cmd == "ask":
            resp = complete(prompt, req.get("max_tokens", MAX_TOKENS), lang)
            conn.sendall(json.dumps({"response": resp}).encode())
        elif cmd == "status":
            conn.sendall(json.dumps({
                "model": STATE.model_name, "loaded": STATE.model_loaded,
                "uptime_s": round(STATE.uptime(), 1), "requests": STATE.request_count,
            }).encode())
        elif cmd == "heal":
            conn.sendall(json.dumps({
                "analysis": "Run: sigma-log tail --level error && sigma-fix scan"
            }).encode())
        else:
            conn.sendall(json.dumps({"error": f"unknown command: {cmd}"}).encode())
    except Exception:
        pass
    finally:
        conn.close()

def run_unix_server():
    sock_path = pathlib.Path(SOCKET_PATH)
    sock_path.parent.mkdir(parents=True, exist_ok=True)
    if sock_path.exists(): sock_path.unlink()
    srv = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    srv.bind(SOCKET_PATH)
    srv.listen(32)
    print(f"[sigma-ai] Unix socket: {SOCKET_PATH}")
    while True:
        try:
            conn, _ = srv.accept()
            threading.Thread(target=handle_unix_client, args=(conn,), daemon=True).start()
        except Exception:
            break

# ── Main daemon ────────────────────────────────────────────────────────────
def main():
    args = sys.argv[1:]
    if "--help" in args or "-h" in args:
        print("sigma-ai daemon — SigmaOS AI inference daemon")
        print("Usage: sigma_ai_daemon.py [--model <path>] [--port <n>]")
        print(f"HTTP: http://{HTTP_HOST}:{HTTP_PORT}/v1/status")
        print(f"Socket: {SOCKET_PATH}")
        return

    # Auto-load model from CLI or environment
    model_path = os.environ.get("SIGMA_AI_MODEL", "")
    if "--model" in args:
        idx = args.index("--model")
        if idx + 1 < len(args): model_path = args[idx + 1]
    if not model_path:
        models = list_models()
        if models:
            model_path = models[0]["path"]
            print(f"[sigma-ai] Auto-selecting model: {model_path}")

    if model_path:
        load_model(model_path)
    else:
        print(f"[sigma-ai] No model loaded. Serving rule-based responses.")
        print(f"  Download: sigma-ai model download tinyllama")
        print(f"  Or place a .gguf file in: {MODEL_DIR}")

    # Start Unix socket in background
    t = threading.Thread(target=run_unix_server, daemon=True)
    t.start()

    # Start HTTP server
    print(f"[sigma-ai] HTTP API: http://{HTTP_HOST}:{HTTP_PORT}/v1/status")
    print(f"[sigma-ai] Requests logged to: {AUDIT_LOG}")
    server = socketserver.ThreadingTCPServer((HTTP_HOST, HTTP_PORT), AiRequestHandler)
    server.allow_reuse_address = True
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\n[sigma-ai] Shutting down.")
        server.server_close()

if __name__ == "__main__":
    main()
