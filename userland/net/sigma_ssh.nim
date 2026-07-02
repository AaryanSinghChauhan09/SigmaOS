# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
# userland/net/sigma_ssh.nim — sigma-ssh: Sovereign SSH Client
# Language: Nim — native binary, OOP via object + methods, no third-party

import std/[net, strutils, parseopt, os, streams]

# ── SSH Protocol Constants ────────────────────────────────────────────────────

const
  SSH_MSG_DISCONNECT:          uint8 = 1
  SSH_MSG_SERVICE_REQUEST:     uint8 = 5
  SSH_MSG_SERVICE_ACCEPT:      uint8 = 6
  SSH_MSG_KEXINIT:             uint8 = 20
  SSH_MSG_NEWKEYS:             uint8 = 21
  SSH_MSG_USERAUTH_REQUEST:    uint8 = 50
  SSH_MSG_USERAUTH_SUCCESS:    uint8 = 52
  SSH_MSG_USERAUTH_FAILURE:    uint8 = 51
  SSH_MSG_CHANNEL_OPEN:        uint8 = 90
  SSH_MSG_CHANNEL_OPEN_CONFIRM: uint8 = 91
  SSH_MSG_CHANNEL_DATA:        uint8 = 94
  SSH_MSG_CHANNEL_EOF:         uint8 = 96
  SSH_MSG_CHANNEL_CLOSE:       uint8 = 97
  SSH_MSG_CHANNEL_REQUEST:     uint8 = 98
  SSH_MSG_CHANNEL_SUCCESS:     uint8 = 99
  SSH_VERSION = "SSH-2.0-SigmaOS_15.0"

# ── Packet Framing ────────────────────────────────────────────────────────────

type
  SshPacket = object
    payload: seq[uint8]

proc encode_uint32(v: uint32): seq[uint8] =
  result = @[
    uint8(v shr 24), uint8(v shr 16),
    uint8(v shr 8),  uint8(v and 0xFF)]

proc encode_string(s: string): seq[uint8] =
  result = encode_uint32(uint32(s.len))
  for c in s: result.add(uint8(c))

proc decode_uint32(data: seq[uint8], off: int): uint32 =
  (uint32(data[off]) shl 24) or (uint32(data[off+1]) shl 16) or
  (uint32(data[off+2]) shl 8) or uint32(data[off+3])

proc build_packet(payload: seq[uint8]): seq[uint8] =
  ## SSH binary packet: 4-byte len + 1-byte padding_len + payload + padding
  let payload_len = payload.len + 1  # +1 for padding_len byte
  var padding = 8 - (payload_len mod 8)
  if padding < 4: padding += 8
  let packet_len = uint32(payload_len + padding)
  result = encode_uint32(packet_len)
  result.add(uint8(padding))
  result.add(payload)
  for _ in 0..<padding: result.add(0)

proc send_packet(sock: Socket, payload: seq[uint8]) =
  let pkt = build_packet(payload)
  let data = cast[string](pkt)
  discard sock.send(data)

proc recv_packet(sock: Socket): seq[uint8] =
  var len_buf = newString(4)
  if sock.recv(len_buf, 4) != 4: return @[]
  let pkt_len = decode_uint32(cast[seq[uint8]](len_buf), 0)
  if pkt_len > 65536: return @[]
  var buf = newString(pkt_len.int)
  if sock.recv(buf, pkt_len.int) != pkt_len.int: return @[]
  let padding = uint8(buf[0])
  let payload_len = pkt_len.int - 1 - padding.int
  result = newSeq[uint8](payload_len)
  for i in 0..<payload_len: result[i] = uint8(buf[i + 1])

# ── SSH Session ───────────────────────────────────────────────────────────────

type
  SshState = enum
    Disconnected, VersionExchange, KeyExchange,
    Authenticated, ChannelOpen, Running

  SshSession = object
    sock:       Socket
    state:      SshState
    channel_id: uint32
    server_ver: string

proc new_session(host: string, port: int): SshSession =
  var sock = newSocket()
  sock.connect(host, Port(port))
  SshSession(sock: sock, state: SshState.VersionExchange,
             channel_id: 0, server_ver: "")

proc version_exchange(s: var SshSession): bool =
  ## Send our version, receive server version
  discard s.sock.send(SSH_VERSION & "\r\n")
  var line = ""
  s.sock.readLine(line)
  if not line.startsWith("SSH-2.0-"):
    stderr.writeLine("sigma-ssh: unsupported server version: " & line)
    return false
  s.server_ver = line.strip()
  s.state = SshState.KeyExchange
  return true

proc send_kexinit(s: var SshSession) =
  var payload: seq[uint8] = @[SSH_MSG_KEXINIT]
  # 16 bytes cookie (random in prod; placeholder here)
  for _ in 0..<16: payload.add(0xAB)
  # Algorithm name lists (simplified — comma-separated strings)
  let kex_algs   = "curve25519-sha256"
  let host_algs  = "ecdsa-sha2-nistp256"
  let enc_algs   = "aes256-gcm@openssh.com"
  let mac_algs   = "hmac-sha2-256"
  let comp_algs  = "none"
  for alg in [kex_algs, host_algs, enc_algs, enc_algs,
              mac_algs, mac_algs, comp_algs, comp_algs]:
    payload.add(encode_string(alg))
  payload.add(encode_string(""))  # languages c→s
  payload.add(encode_string(""))  # languages s→c
  payload.add(0)                  # first_kex_packet_follows
  payload.add(@[0u8,0,0,0])       # reserved
  send_packet(s.sock, payload)

proc userauth_password(s: var SshSession, user, pass: string): bool =
  var payload: seq[uint8] = @[SSH_MSG_USERAUTH_REQUEST]
  payload.add(encode_string(user))
  payload.add(encode_string("ssh-connection"))
  payload.add(encode_string("password"))
  payload.add(0) # FALSE (not changing password)
  payload.add(encode_string(pass))
  send_packet(s.sock, payload)
  let resp = recv_packet(s.sock)
  if resp.len == 0: return false
  if resp[0] == SSH_MSG_USERAUTH_SUCCESS:
    s.state = SshState.Authenticated; return true
  false

proc open_session_channel(s: var SshSession): bool =
  var payload: seq[uint8] = @[SSH_MSG_CHANNEL_OPEN]
  payload.add(encode_string("session"))
  payload.add(encode_uint32(0))          # local channel ID
  payload.add(encode_uint32(1048576))    # initial window size
  payload.add(encode_uint32(32768))      # max packet size
  send_packet(s.sock, payload)
  let resp = recv_packet(s.sock)
  if resp.len == 0: return false
  if resp[0] == SSH_MSG_CHANNEL_OPEN_CONFIRM:
    s.channel_id = decode_uint32(resp, 5)
    s.state = SshState.ChannelOpen; return true
  false

proc request_shell(s: var SshSession): bool =
  var payload: seq[uint8] = @[SSH_MSG_CHANNEL_REQUEST]
  payload.add(encode_uint32(s.channel_id))
  payload.add(encode_string("shell"))
  payload.add(1) # want reply
  send_packet(s.sock, payload)
  let resp = recv_packet(s.sock)
  if resp.len == 0: return false
  resp[0] == SSH_MSG_CHANNEL_SUCCESS

proc run_interactive(s: var SshSession) =
  ## Simple interactive loop: read stdin, send; recv, print to stdout
  s.state = SshState.Running
  while true:
    var buf = newString(1024)
    let n = stdin.readBuffer(addr buf[0], 1024)
    if n <= 0: break
    var payload: seq[uint8] = @[SSH_MSG_CHANNEL_DATA]
    payload.add(encode_uint32(s.channel_id))
    payload.add(encode_uint32(uint32(n)))
    for i in 0..<n: payload.add(uint8(buf[i]))
    send_packet(s.sock, payload)
    let resp = recv_packet(s.sock)
    if resp.len == 0: break
    if resp[0] == SSH_MSG_CHANNEL_DATA:
      let data_len = decode_uint32(resp, 5).int
      for i in 9..<9+data_len:
        stdout.write(chr(resp[i]))
      stdout.flushFile()
    elif resp[0] == SSH_MSG_CHANNEL_EOF or resp[0] == SSH_MSG_CHANNEL_CLOSE:
      break

proc disconnect(s: var SshSession) =
  if s.state != SshState.Disconnected:
    var payload: seq[uint8] = @[SSH_MSG_DISCONNECT,0,0,0,11]
    payload.add(encode_string("Goodbye"))
    payload.add(encode_string(""))
    send_packet(s.sock, payload)
    s.sock.close()
    s.state = SshState.Disconnected

# ── CLI ───────────────────────────────────────────────────────────────────────

proc usage() =
  echo "sigma-ssh — Sovereign SSH Client v15.0"
  echo "Usage: sigma-ssh [-p port] [-l user] [-i identity] host [command]"

proc main() =
  var host = ""; var port = 22; var user = getCurrentUser()
  var password = ""; var args: seq[string]
  var p = initOptParser()
  for kind, key, val in p.getopt():
    case kind
    of cmdOption:
      case key
      of "p": port = parseInt(val)
      of "l": user = val
      of "h", "help": usage(); quit(0)
      else: discard
    of cmdArgument: args.add(key)
    else: discard

  if args.len == 0: usage(); quit(1)
  host = args[0]

  echo "sigma-ssh: connecting to " & user & "@" & host & ":" & $port
  var sess = new_session(host, port)
  if not version_exchange(sess):
    stderr.writeLine("sigma-ssh: version exchange failed"); quit(1)
  send_kexinit(sess)
  # Skip full kex for now (simplified) — get password
  password = readPasswordFromStdin("Password: ")
  if not userauth_password(sess, user, password):
    stderr.writeLine("sigma-ssh: authentication failed"); quit(1)
  if not open_session_channel(sess):
    stderr.writeLine("sigma-ssh: channel open failed"); quit(1)
  if not request_shell(sess):
    stderr.writeLine("sigma-ssh: shell request failed"); quit(1)
  run_interactive(sess)
  disconnect(sess)

main()
