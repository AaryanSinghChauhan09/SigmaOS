// SPDX-License-Identifier: GPL-2.0-or-later
// sigma_net_relay.js — WebSocket → TCP relay server
//
// Bridges browser-emulated SigmaOS (v86 / WASM) to the real internet.
// Browsers cannot open raw TCP sockets; this relay acts as the proxy.
//
// Protocol:
//   Client connects via WebSocket.
//   First message: JSON { "type": "connect", "host": "...", "port": N }
//   Subsequent messages: binary (raw TCP bytes)
//   Server relays bytes in both directions.
//
// Run:
//   node tools/sigma_net_relay.js --port 8765 --bind 0.0.0.0
//
// Inspired by: websockify (novnc), wstunnel

'use strict';

const WebSocket = require('ws');
const net       = require('net');
const http      = require('http');
const { URL }   = require('url');

// ── Config ────────────────────────────────────────────────────────────────────
const DEFAULT_PORT  = 8765;
const MAX_SESSIONS  = 500;
const IDLE_TIMEOUT  = 120 * 1000;  // 120s idle → close

// Blocked destination ranges (RFC1918 + loopback)
const BLOCKED_HOSTS = [/^127\./, /^10\./, /^192\.168\./, /^172\.(1[6-9]|2\d|3[01])\./];

const args = process.argv.slice(2);
let bindPort = DEFAULT_PORT;
for (let i = 0; i < args.length; i++) {
  if (args[i] === '--port' && args[i+1]) bindPort = parseInt(args[i+1]);
}

// ── Active session tracking ───────────────────────────────────────────────────
let sessionCount = 0;

// ── HTTP server for health check ──────────────────────────────────────────────
const httpServer = http.createServer((req, res) => {
  if (req.url === '/health') {
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({ status: 'ok', sessions: sessionCount }));
  } else {
    res.writeHead(404); res.end();
  }
});

// ── WebSocket server ─────────────────────────────────────────────────────────
const wss = new WebSocket.Server({ server: httpServer });

wss.on('connection', (ws, req) => {
  if (sessionCount >= MAX_SESSIONS) {
    ws.close(1013, 'Too many sessions');
    return;
  }

  const clientIp = req.socket.remoteAddress;
  let   tcpSocket   = null;
  let   connected   = false;
  let   bytesIn     = 0;
  let   bytesOut    = 0;
  let   idleTimer   = null;

  const resetIdle = () => {
    if (idleTimer) clearTimeout(idleTimer);
    idleTimer = setTimeout(() => {
      console.log(`[relay] idle timeout: ${clientIp}`);
      cleanup();
    }, IDLE_TIMEOUT);
  };

  const cleanup = () => {
    if (idleTimer) clearTimeout(idleTimer);
    if (tcpSocket) { tcpSocket.destroy(); tcpSocket = null; }
    if (ws.readyState === WebSocket.OPEN) ws.close();
    if (connected) { sessionCount--; connected = false; }
  };

  ws.on('message', (data) => {
    resetIdle();

    // First message: connection request JSON
    if (!tcpSocket) {
      let req;
      try { req = JSON.parse(data); } catch {
        ws.close(1003, 'Expected JSON connect message');
        return;
      }

      if (req.type !== 'connect' || !req.host || !req.port) {
        ws.close(1003, 'Invalid connect message');
        return;
      }

      // Block RFC1918 / loopback
      for (const pattern of BLOCKED_HOSTS) {
        if (pattern.test(req.host)) {
          ws.close(1008, 'Blocked host');
          return;
        }
      }

      console.log(`[relay] ${clientIp} → ${req.host}:${req.port}`);
      sessionCount++;
      connected = true;

      tcpSocket = net.connect(req.port, req.host, () => {
        ws.send(JSON.stringify({ type: 'connected' }));
        resetIdle();
      });

      tcpSocket.on('data', (chunk) => {
        bytesOut += chunk.length;
        if (ws.readyState === WebSocket.OPEN) {
          ws.send(chunk, { binary: true });
        }
      });

      tcpSocket.on('close',   () => { ws.close(1000, 'TCP closed'); cleanup(); });
      tcpSocket.on('error',   (e) => { ws.close(1011, e.message);  cleanup(); });
      return;
    }

    // Subsequent messages: raw TCP bytes to forward
    if (tcpSocket && !tcpSocket.destroyed) {
      tcpSocket.write(data);
      bytesIn += data.length;
    }
  });

  ws.on('close',   cleanup);
  ws.on('error',   cleanup);
  resetIdle();
});

httpServer.listen(bindPort, '0.0.0.0', () => {
  console.log(`[sigma-net-relay] Listening on ws://0.0.0.0:${bindPort}`);
  console.log(`[sigma-net-relay] Health: http://localhost:${bindPort}/health`);
  console.log(`[sigma-net-relay] Max sessions: ${MAX_SESSIONS}`);
});

// ── Graceful shutdown ─────────────────────────────────────────────────────────
process.on('SIGTERM', () => {
  console.log('[sigma-net-relay] Shutting down...');
  wss.close();
  httpServer.close();
});
