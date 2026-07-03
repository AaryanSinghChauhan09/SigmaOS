# sigmad-process/pty_server.nim
# SigmaOS PTY Server — WebSocket PTY bridge on port 17393 /pty
# Low-level POSIX only: posix + net modules, hand-rolled WS (RFC 6455)
# No external libraries.

import posix, net, os, strutils

# ---------------------------------------------------------------------------
# POSIX / Linux constants not in stdlib posix module
# ---------------------------------------------------------------------------
const
  TIOCSWINSZ  = 0x5414.cint
  TIOCGWINSZ  = 0x5413.cint
  O_RDWR      = 0x0002.cint
  O_NOCTTY    = 0x0400.cint
  TIOCSPTLCK  = 0x40045431.cint
  TIOCGPTN    = 0x80045430.cint

type
  WinSize = object
    ws_row, ws_col, ws_xpixel, ws_ypixel: cushort

# openpty wrapper via /dev/ptmx
proc openPtmx(masterFd: var cint): cint =
  masterFd = open("/dev/ptmx", O_RDWR or O_NOCTTY)
  if masterFd < 0:
    return -1
  # unlock slave
  var unlock: cint = 0
  if ioctl(masterFd, TIOCSPTLCK, addr unlock) != 0:
    discard close(masterFd)
    return -1
  return 0

proc getSlaveName(masterFd: cint, buf: var array[64, char]): bool =
  var n: cint = 0
  if ioctl(masterFd, TIOCGPTN, addr n) != 0:
    return false
  let s = "/dev/pts/" & $n
  for i, c in s:
    if i < buf.len: buf[i] = c
  return true

# ---------------------------------------------------------------------------
# PTY session
# ---------------------------------------------------------------------------
type
  PTYSession = object
    id:       int
    masterFd: cint
    pid:      Pid
    cols:     int
    rows:     int

var sessions: seq[PTYSession]
var nextId: int = 1

# ---------------------------------------------------------------------------
# SHA-1 for WebSocket handshake (RFC 6455 §4)
# Minimal hand-rolled implementation — no stdlib crypto
# ---------------------------------------------------------------------------
proc rotl32(v: uint32, n: int): uint32 {.inline.} =
  (v shl n) or (v shr (32 - n))

proc sha1(msg: openArray[byte]): array[20, byte] =
  var h0: uint32 = 0x67452301'u32
  var h1: uint32 = 0xEFCDAB89'u32
  var h2: uint32 = 0x98BADCFE'u32
  var h3: uint32 = 0x10325476'u32
  var h4: uint32 = 0xC3D2E1F0'u32

  let origLen = msg.len
  var data = newSeq[byte](msg.len)
  for i, b in msg: data[i] = b

  # padding
  data.add(0x80'u8)
  while (data.len mod 64) != 56:
    data.add(0x00'u8)
  let bitLen = uint64(origLen) * 8
  for i in countdown(7, 0):
    data.add(byte((bitLen shr (i * 8)) and 0xFF))

  # process blocks
  var w: array[80, uint32]
  var i = 0
  while i < data.len:
    for j in 0 ..< 16:
      w[j] = (uint32(data[i + j*4]) shl 24) or
             (uint32(data[i + j*4+1]) shl 16) or
             (uint32(data[i + j*4+2]) shl 8)  or
              uint32(data[i + j*4+3])
    for j in 16 ..< 80:
      w[j] = rotl32(w[j-3] xor w[j-8] xor w[j-14] xor w[j-16], 1)

    var a = h0; var b = h1; var c = h2; var d = h3; var e = h4
    for j in 0 ..< 80:
      var f, k: uint32
      if j < 20:
        f = (b and c) or ((not b) and d); k = 0x5A827999'u32
      elif j < 40:
        f = b xor c xor d; k = 0x6ED9EBA1'u32
      elif j < 60:
        f = (b and c) or (b and d) or (c and d); k = 0x8F1BBCDC'u32
      else:
        f = b xor c xor d; k = 0xCA62C1D6'u32
      let temp = rotl32(a, 5) + f + e + k + w[j]
      e = d; d = c; c = rotl32(b, 30); b = a; a = temp

    h0 += a; h1 += b; h2 += c; h3 += d; h4 += e
    i += 64

  template put32(arr: var array[20, byte], off: int, v: uint32) =
    arr[off]   = byte(v shr 24)
    arr[off+1] = byte(v shr 16)
    arr[off+2] = byte(v shr 8)
    arr[off+3] = byte(v)
  result.put32(0,  h0)
  result.put32(4,  h1)
  result.put32(8,  h2)
  result.put32(12, h3)
  result.put32(16, h4)

# ---------------------------------------------------------------------------
# Base64 encode (RFC 4648)
# ---------------------------------------------------------------------------
const B64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"

proc base64Encode(data: openArray[byte]): string =
  result = newStringOfCap(((data.len + 2) div 3) * 4)
  var i = 0
  while i < data.len:
    let b0 = data[i]
    let b1 = if i+1 < data.len: data[i+1] else: 0'u8
    let b2 = if i+2 < data.len: data[i+2] else: 0'u8
    result.add(B64[b0 shr 2])
    result.add(B64[((b0 and 0x03) shl 4) or (b1 shr 4)])
    result.add(if i+1 < data.len: B64[((b1 and 0x0F) shl 2) or (b2 shr 6)] else: '=')
    result.add(if i+2 < data.len: B64[b2 and 0x3F] else: '=')
    i += 3

# ---------------------------------------------------------------------------
# WebSocket handshake
# ---------------------------------------------------------------------------
const WS_MAGIC = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"

proc wsHandshake(sock: SocketHandle): bool =
  ## Read HTTP upgrade request, send 101.  Returns true on success.
  var buf = newString(4096)
  let n = recv(sock, addr buf[0], 4095, 0)
  if n <= 0: return false
  buf.setLen(n)

  # extract Sec-WebSocket-Key
  var key = ""
  for line in buf.splitLines():
    if line.startsWith("Sec-WebSocket-Key:"):
      key = line[18..^1].strip()
      break
  if key.len == 0: return false

  # check path /pty
  var pathOk = false
  for line in buf.splitLines():
    if line.startsWith("GET /pty "): pathOk = true; break
  if not pathOk: return false

  let combined = key & WS_MAGIC
  var cbytes = newSeq[byte](combined.len)
  for i, c in combined: cbytes[i] = byte(c)
  let digest = sha1(cbytes)
  let accept = base64Encode(digest)

  let response = "HTTP/1.1 101 Switching Protocols\r\n" &
    "Upgrade: websocket\r\n" &
    "Connection: Upgrade\r\n" &
    "Sec-WebSocket-Accept: " & accept & "\r\n\r\n"
  discard send(sock, cast[pointer](unsafeAddr response[0]), response.len, 0)
  return true

# ---------------------------------------------------------------------------
# RFC 6455 framing
# ---------------------------------------------------------------------------
proc wsSend(sock: SocketHandle, data: openArray[byte], opcode: byte = 0x02) =
  ## Send a WebSocket frame (unmasked, server→client).
  var frame = newSeq[byte]()
  frame.add(0x80'u8 or opcode)   # FIN + opcode
  let dataLen = data.len
  if dataLen < 126:
    frame.add(byte(dataLen))
  elif dataLen < 65536:
    frame.add(0x7E'u8)
    frame.add(byte(dataLen shr 8))
    frame.add(byte(dataLen and 0xFF))
  else:
    frame.add(0x7F'u8)
    for i in countdown(7, 0):
      frame.add(byte((dataLen shr (i*8)) and 0xFF))
  for b in data: frame.add(b)
  discard send(sock, addr frame[0], frame.len, 0)

proc wsSendText(sock: SocketHandle, s: string) =
  var bytes = newSeq[byte](s.len)
  for i, c in s: bytes[i] = byte(c)
  wsSend(sock, bytes, 0x01)

type WsFrame = object
  opcode: byte
  payload: seq[byte]

proc wsRecvFrame(sock: SocketHandle, frame: var WsFrame): bool =
  ## Read exactly one WebSocket frame from client (masked).
  var header: array[2, byte]
  if recv(sock, addr header[0], 2, MSG_WAITALL) != 2: return false
  frame.opcode = header[0] and 0x0F
  let masked = (header[1] and 0x80) != 0
  var payLen = int(header[1] and 0x7F)
  if payLen == 126:
    var ext: array[2, byte]
    if recv(sock, addr ext[0], 2, MSG_WAITALL) != 2: return false
    payLen = (int(ext[0]) shl 8) or int(ext[1])
  elif payLen == 127:
    var ext: array[8, byte]
    if recv(sock, addr ext[0], 8, MSG_WAITALL) != 8: return false
    payLen = 0
    for i in 0 ..< 8: payLen = (payLen shl 8) or int(ext[i])

  var maskKey: array[4, byte]
  if masked:
    if recv(sock, addr maskKey[0], 4, MSG_WAITALL) != 4: return false

  frame.payload = newSeq[byte](payLen)
  if payLen > 0:
    if recv(sock, addr frame.payload[0], payLen, MSG_WAITALL) != payLen: return false
    if masked:
      for i in 0 ..< payLen:
        frame.payload[i] = frame.payload[i] xor maskKey[i mod 4]
  return true

# ---------------------------------------------------------------------------
# Minimal JSON helpers (no stdlib json)
# ---------------------------------------------------------------------------
proc jsonGetStr(json: string, key: string): string =
  ## Extract string value for key from flat JSON object.
  let needle = "\"" & key & "\":"
  let pos = json.find(needle)
  if pos < 0: return ""
  var i = pos + needle.len
  while i < json.len and json[i] in {' ', '\t'}: i += 1
  if i >= json.len or json[i] != '"': return ""
  i += 1
  var res = ""
  while i < json.len and json[i] != '"':
    res.add(json[i]); i += 1
  return res

proc jsonGetInt(json: string, key: string): int =
  let needle = "\"" & key & "\":"
  let pos = json.find(needle)
  if pos < 0: return 0
  var i = pos + needle.len
  while i < json.len and json[i] in {' ', '\t'}: i += 1
  var numStr = ""
  while i < json.len and json[i] in {'0'..'9', '-'}:
    numStr.add(json[i]); i += 1
  if numStr.len > 0: return parseInt(numStr) else: return 0

# ---------------------------------------------------------------------------
# Spawn shell in slave PTY
# ---------------------------------------------------------------------------
proc spawnShell(masterFd: cint, cols, rows: int): Pid =
  # Get slave name
  var slaveName: array[64, char]
  if not getSlaveName(masterFd, slaveName):
    return Pid(-1)

  let slaveStr = newString(64)
  var slen = 0
  for c in slaveName:
    if c == '\0': break
    slen += 1
  var slaveNameStr = newString(slen)
  for i in 0 ..< slen: slaveNameStr[i] = slaveName[i]

  # Set initial window size on master before fork
  var ws = WinSize(ws_row: cushort(rows), ws_col: cushort(cols),
                    ws_xpixel: 0, ws_ypixel: 0)
  discard ioctl(masterFd, TIOCSWINSZ, addr ws)

  let pid = fork()
  if pid < 0: return Pid(-1)

  if pid == 0:
    # child: become session leader, open slave as controlling terminal
    discard setsid()
    let slaveFd = open(slaveNameStr.cstring, O_RDWR)
    if slaveFd < 0: discard kill(getpid(), SIGKILL)

    # redirect stdio
    discard dup2(slaveFd, STDIN_FILENO)
    discard dup2(slaveFd, STDOUT_FILENO)
    discard dup2(slaveFd, STDERR_FILENO)
    if slaveFd > 2: discard close(slaveFd)
    discard close(masterFd)

    # exec /bin/sh
    var argv: array[2, cstring]
    argv[0] = "/bin/sh"
    argv[1] = nil
    discard execv("/bin/sh", cast[cstringArray](addr argv))
    discard kill(getpid(), SIGKILL)
  else:
    return pid

  return Pid(0)

# ---------------------------------------------------------------------------
# Handle a single WebSocket client connection
# ---------------------------------------------------------------------------
proc handleClient(sock: SocketHandle) =
  if not wsHandshake(sock):
    discard close(sock.cint)
    return

  # Allocate PTY
  var masterFd: cint
  if openPtmx(masterFd) != 0:
    wsSendText(sock, "{\"error\":\"PTY alloc failed\"}")
    discard close(sock.cint)
    return

  let pid = spawnShell(masterFd, 80, 24)
  if pid <= 0:
    discard close(masterFd)
    discard close(sock.cint)
    return

  var sess = PTYSession(id: nextId, masterFd: masterFd, pid: pid, cols: 80, rows: 24)
  nextId += 1
  sessions.add(sess)

  # I/O loop: multiplex masterFd <-> WebSocket using select
  var buf: array[4096, byte]
  var running = true

  while running:
    var rfds: TFdSet
    FD_ZERO(rfds)
    FD_SET(masterFd, rfds)
    FD_SET(sock.cint, rfds)
    let maxFd = max(masterFd, sock.cint)

    var tv: Timeval
    tv.tv_sec  = 0
    tv.tv_usec = 50_000  # 50 ms poll

    let sel = select(maxFd + 1, addr rfds, nil, nil, addr tv)
    if sel < 0:
      break

    # Data from PTY → WebSocket
    if FD_ISSET(masterFd, rfds):
      let n = read(masterFd, addr buf[0], buf.len)
      if n <= 0:
        running = false
      else:
        wsSend(sock, buf.toOpenArray(0, n - 1), 0x02)

    # Data from WebSocket → PTY
    if FD_ISSET(sock.cint, rfds):
      var frame: WsFrame
      if not wsRecvFrame(sock, frame):
        running = false
      elif frame.opcode == 0x08:  # close
        running = false
      elif frame.opcode == 0x01 or frame.opcode == 0x02:
        # Check for resize JSON: {"type":"resize","cols":N,"rows":M}
        let payload = cast[string](frame.payload)
        if payload.len > 0 and payload[0] == '{':
          let msgType = jsonGetStr(payload, "type")
          if msgType == "resize":
            let newCols = jsonGetInt(payload, "cols")
            let newRows = jsonGetInt(payload, "rows")
            if newCols > 0 and newRows > 0:
              var ws = WinSize(ws_row: cushort(newRows), ws_col: cushort(newCols),
                               ws_xpixel: 0, ws_ypixel: 0)
              discard ioctl(masterFd, TIOCSWINSZ, addr ws)
        else:
          # raw keystroke input → PTY
          if frame.payload.len > 0:
            discard write(masterFd, addr frame.payload[0], frame.payload.len)

  # Cleanup
  discard close(masterFd)
  discard close(sock.cint)
  discard kill(pid, SIGKILL)
  var status: cint
  discard waitpid(pid, status, WNOHANG)

  # Remove session
  for i in 0 ..< sessions.len:
    if sessions[i].pid == pid:
      sessions.del(i)
      break

# ---------------------------------------------------------------------------
# Main: TCP listener
# ---------------------------------------------------------------------------
proc main() =
  let serverFd = socket(AF_INET, SOCK_STREAM, 0)
  if serverFd < 0:
    quit("socket() failed", 1)

  var one: cint = 1
  discard setsockopt(serverFd.cint, SOL_SOCKET, SO_REUSEADDR, addr one, sizeof(one).SockLen)

  var saddr: Sockaddr_in
  saddr.sin_family = AF_INET.uint16
  saddr.sin_port   = htons(17393)
  saddr.sin_addr.s_addr = INADDR_ANY

  if bindSocket(serverFd, cast[ptr SockAddr](addr saddr), sizeof(saddr).SockLen) < 0:
    quit("bind() failed", 1)

  if listen(serverFd, 8) < 0:
    quit("listen() failed", 1)

  # Ignore SIGCHLD to auto-reap
  var sa: Sigaction
  sa.sa_handler = SIG_DFL
  sa.sa_flags   = SA_NOCLDWAIT
  discard sigaction(SIGCHLD, sa, nil)

  while true:
    var clientAddr: SockAddr
    var addrLen = sizeof(clientAddr).SockLen
    let clientFd = accept(serverFd, addr clientAddr, addr addrLen)
    if clientFd < 0: continue

    # Fork per client (simple model for a daemon)
    let cpid = fork()
    if cpid == 0:
      discard close(serverFd.cint)
      handleClient(clientFd.SocketHandle)
      quit(0)
    else:
      discard close(clientFd.cint)

main()
