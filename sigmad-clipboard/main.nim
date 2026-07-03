# sigmad-clipboard/main.nim
# SigmaOS Clipboard Daemon
# Unix socket at /tmp/sigma-clipboard.sock
# Supports MIME: text/plain, text/html, image/png
# Multi-client broadcast on Write; Read returns latest within 100 ms
# All I/O via low-level posix: socket/bind/listen/accept/select/send/recv
# No external packages.

import posix, strutils, os

# ---------------------------------------------------------------------------
# Constants
# ---------------------------------------------------------------------------
const
  SOCK_PATH    = "/tmp/sigma-clipboard.sock"
  MAX_CLIENTS  = 64
  MAX_PAYLOAD  = 8 * 1024 * 1024  # 8 MiB hard cap per clipboard entry
  READ_TIMEOUT_MS = 100

# ---------------------------------------------------------------------------
# Wire protocol (simple framing over the Unix socket)
#
# Client → Daemon:
#   WRITE\n<mime_type>\n<base64_content>\n
#   READ\n
#
# Daemon → Client:
#   OK\n                                     (ack for WRITE)
#   DATA\n<mime_type>\n<base64_content>\n    (response for READ)
#   EMPTY\n                                  (no data yet, for READ)
#   EVENT\nclipboard-updated\n               (pushed to all on WRITE)
#   ERROR\n<message>\n
# ---------------------------------------------------------------------------

# ---------------------------------------------------------------------------
# ClipboardData
# ---------------------------------------------------------------------------
type
  ClipboardData = object
    mime_type: string
    content:   seq[byte]

var clipboard: ClipboardData   # current clipboard state
var hasData: bool = false

# ---------------------------------------------------------------------------
# Client tracking
# ---------------------------------------------------------------------------
var clientFds: array[MAX_CLIENTS, cint]
var clientCount: int = 0

proc addClient(fd: cint): bool =
  if clientCount >= MAX_CLIENTS: return false
  clientFds[clientCount] = fd
  clientCount += 1
  return true

proc removeClient(fd: cint) =
  for i in 0 ..< clientCount:
    if clientFds[i] == fd:
      discard close(fd)
      clientFds[i] = clientFds[clientCount - 1]
      clientCount -= 1
      return

# ---------------------------------------------------------------------------
# Base64 encode / decode (RFC 4648, no padding issues)
# ---------------------------------------------------------------------------
const B64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"

proc b64Encode(data: seq[byte]): string =
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

proc b64Decode(s: string): seq[byte] =
  const DEC: array[128, int8] = [
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,62,-1,-1,-1,63,
    52,53,54,55,56,57,58,59,60,61,-1,-1,-1,-1,-1,-1,
    -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,
    15,16,17,18,19,20,21,22,23,24,25,-1,-1,-1,-1,-1,
    -1,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,
    41,42,43,44,45,46,47,48,49,50,51,-1,-1,-1,-1,-1]
  result = @[]
  var acc: int = 0
  var bits: int = 0
  for c in s:
    if c == '=': break
    if ord(c) >= 128: continue
    let v = DEC[ord(c)]
    if v < 0: continue
    acc = (acc shl 6) or int(v)
    bits += 6
    if bits >= 8:
      bits -= 8
      result.add(byte((acc shr bits) and 0xFF))

# ---------------------------------------------------------------------------
# I/O helpers — non-blocking send of a full string
# ---------------------------------------------------------------------------
proc sendAll(fd: cint, s: string): bool =
  var sent = 0
  while sent < s.len:
    let n = send(fd, cast[pointer](unsafeAddr s[sent]), s.len - sent, MSG_NOSIGNAL)
    if n <= 0: return false
    sent += n
  return true

# Read a line (up to '\n') from fd, with 100 ms timeout via select
proc recvLine(fd: cint, line: var string): bool =
  line = ""
  while true:
    var rfds: TFdSet
    FD_ZERO(rfds)
    FD_SET(fd, rfds)
    var tv: Timeval
    tv.tv_sec  = 0
    tv.tv_usec = READ_TIMEOUT_MS * 1000
    let sel = select(fd + 1, addr rfds, nil, nil, addr tv)
    if sel <= 0: return false   # timeout or error
    var ch: char
    let n = recv(fd, addr ch, 1, 0)
    if n <= 0: return false
    if ch == '\n': return true
    line.add(ch)

# ---------------------------------------------------------------------------
# Broadcast clipboard-updated event to all connected clients (non-blocking)
# ---------------------------------------------------------------------------
proc broadcastUpdated() =
  let msg = "EVENT\nclipboard-updated\n"
  var toRemove: seq[cint]
  for i in 0 ..< clientCount:
    let fd = clientFds[i]
    if not sendAll(fd, msg):
      toRemove.add(fd)
  for fd in toRemove:
    removeClient(fd)

# ---------------------------------------------------------------------------
# Handle a single command from a connected client fd
# Returns false when client should be closed.
# ---------------------------------------------------------------------------
proc handleCommand(fd: cint): bool =
  var cmd: string
  if not recvLine(fd, cmd): return false

  case cmd
  of "WRITE":
    var mimeType: string
    var b64Content: string
    if not recvLine(fd, mimeType): return false
    if not recvLine(fd, b64Content): return false

    # Validate mime type
    if mimeType notin ["text/plain", "text/html", "image/png"]:
      discard sendAll(fd, "ERROR\nunsupported MIME type\n")
      return true  # keep connection

    let content = b64Decode(b64Content)
    if content.len > MAX_PAYLOAD:
      discard sendAll(fd, "ERROR\npayload too large\n")
      return true

    clipboard.mime_type = mimeType
    clipboard.content   = content
    hasData = true

    discard sendAll(fd, "OK\n")
    broadcastUpdated()
    return true

  of "READ":
    var rfds: TFdSet
    FD_ZERO(rfds)
    FD_SET(fd, rfds)
    var tv: Timeval
    tv.tv_sec  = 0
    tv.tv_usec = READ_TIMEOUT_MS * 1000
    # Use select to respect the 100 ms read deadline on our end too
    let sel = select(fd + 1, addr rfds, nil, nil, addr tv)
    if sel < 0:
      discard sendAll(fd, "ERROR\nselect failed\n")
      return false

    if not hasData:
      discard sendAll(fd, "EMPTY\n")
      return true

    let encoded = b64Encode(clipboard.content)
    let response = "DATA\n" & clipboard.mime_type & "\n" & encoded & "\n"
    if not sendAll(fd, response): return false
    return true

  else:
    discard sendAll(fd, "ERROR\nunknown command\n")
    return true

# ---------------------------------------------------------------------------
# Main event loop
# ---------------------------------------------------------------------------
proc main() =
  # Remove stale socket
  discard unlink(SOCK_PATH.cstring)

  let serverFd = socket(AF_UNIX, SOCK_STREAM, 0)
  if serverFd < 0: quit("socket() failed", 1)

  var addr_un: Sockaddr_un
  addr_un.sun_family = AF_UNIX.uint16
  let path = SOCK_PATH
  let copyLen = min(path.len, sizeof(addr_un.sun_path) - 1)
  for i in 0 ..< copyLen:
    addr_un.sun_path[i] = path[i]

  if bindSocket(serverFd, cast[ptr SockAddr](addr addr_un), sizeof(addr_un).SockLen) < 0:
    quit("bind() failed on " & SOCK_PATH, 1)

  if listen(serverFd, 32) < 0:
    quit("listen() failed", 1)

  # Set server socket permissions
  discard chmod(SOCK_PATH.cstring, 0o600)

  while true:
    # Build fd_set for select: server + all clients
    var rfds: TFdSet
    FD_ZERO(rfds)
    FD_SET(serverFd, rfds)
    var maxFd = serverFd

    for i in 0 ..< clientCount:
      FD_SET(clientFds[i], rfds)
      if clientFds[i] > maxFd: maxFd = clientFds[i]

    var tv: Timeval
    tv.tv_sec  = 5
    tv.tv_usec = 0

    let sel = select(maxFd + 1, addr rfds, nil, nil, addr tv)
    if sel < 0:
      if errno == EINTR: continue
      break

    # New connection
    if FD_ISSET(serverFd, rfds):
      var clientAddr: SockAddr
      var addrLen = sizeof(clientAddr).SockLen
      let clientFd = accept(serverFd, addr clientAddr, addr addrLen)
      if clientFd >= 0:
        if not addClient(clientFd):
          discard sendAll(clientFd, "ERROR\ntoo many clients\n")
          discard close(clientFd)

    # Service existing clients
    var toClose: seq[cint]
    for i in 0 ..< clientCount:
      let fd = clientFds[i]
      if FD_ISSET(fd, rfds):
        if not handleCommand(fd):
          toClose.add(fd)

    for fd in toClose:
      removeClient(fd)

  discard close(serverFd)
  discard unlink(SOCK_PATH.cstring)

main()
