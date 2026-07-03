# sigmad-sandbox/main.nim
# SigmaOS Sandbox Manager
# Parses a JSON manifest, builds a bwrap argument array, execs bwrap.
# Logs capability violations to /var/log/sigma-audit.log
# Uses only the posix module — no high-level stdlib.

import posix, os

# ---------------------------------------------------------------------------
# Constants
# ---------------------------------------------------------------------------
const
  BWRAP_PATH = "/usr/bin/bwrap"
  AUDIT_LOG  = "/var/log/sigma-audit.log"

# Recognised capabilities and their bwrap flags
const CAP_MAP = [
  ("network",          "--share-net"),
  ("filesystem.read",  "--ro-bind /home /home"),
]

# ---------------------------------------------------------------------------
# Minimal hand-rolled JSON parser
# Only extracts: name (string), command (string), capabilities (array of strings)
# ---------------------------------------------------------------------------
type
  ManifestFields = object
    name:         string
    command:      string
    capabilities: seq[string]

proc skipWS(s: string, i: var int) =
  while i < s.len and s[i] in {' ', '\t', '\n', '\r'}: i += 1

proc parseString(s: string, i: var int): string =
  ## Parse a JSON quoted string starting at position i (which should be '"').
  result = ""
  if i >= s.len or s[i] != '"': return
  i += 1  # skip opening quote
  while i < s.len:
    let c = s[i]
    if c == '"':
      i += 1  # skip closing quote
      return
    elif c == '\\' and i + 1 < s.len:
      let esc = s[i + 1]
      case esc
      of '"':  result.add('"')
      of '\\': result.add('\\')
      of '/':  result.add('/')
      of 'n':  result.add('\n')
      of 'r':  result.add('\r')
      of 't':  result.add('\t')
      of 'b':  result.add('\b')
      of 'f':  result.add('\f')
      else:    result.add(esc)
      i += 2
    else:
      result.add(c)
      i += 1

proc skipValue(s: string, i: var int) =
  ## Skip over any JSON value (string, number, bool, null, object, array).
  skipWS(s, i)
  if i >= s.len: return
  case s[i]
  of '"':
    discard parseString(s, i)
  of '{':
    i += 1
    while i < s.len and s[i] != '}':
      skipWS(s, i)
      if i < s.len and s[i] == '"': discard parseString(s, i)
      skipWS(s, i)
      if i < s.len and s[i] == ':': i += 1
      skipWS(s, i)
      skipValue(s, i)
      skipWS(s, i)
      if i < s.len and s[i] == ',': i += 1
    if i < s.len: i += 1  # skip '}'
  of '[':
    i += 1
    while i < s.len and s[i] != ']':
      skipWS(s, i)
      if i < s.len and s[i] == ']': break
      skipValue(s, i)
      skipWS(s, i)
      if i < s.len and s[i] == ',': i += 1
    if i < s.len: i += 1  # skip ']'
  else:
    # number / bool / null: consume until delimiter
    while i < s.len and s[i] notin {',', '}', ']', '\n', '\r', ' ', '\t'}:
      i += 1

proc parseManifest(json: string): ManifestFields =
  result.capabilities = @[]
  var i = 0
  skipWS(json, i)
  if i >= json.len or json[i] != '{': return
  i += 1  # skip '{'

  while i < json.len and json[i] != '}':
    skipWS(json, i)
    if i >= json.len or json[i] == '}': break

    # Parse key
    if json[i] != '"': break
    let key = parseString(json, i)

    skipWS(json, i)
    if i >= json.len or json[i] != ':': break
    i += 1  # skip ':'
    skipWS(json, i)

    # Parse value based on key
    case key
    of "name":
      result.name = parseString(json, i)
    of "command":
      result.command = parseString(json, i)
    of "capabilities":
      # Parse array of strings
      if i < json.len and json[i] == '[':
        i += 1  # skip '['
        while i < json.len and json[i] != ']':
          skipWS(json, i)
          if i < json.len and json[i] == ']': break
          if i < json.len and json[i] == '"':
            result.capabilities.add(parseString(json, i))
          else:
            skipValue(json, i)
          skipWS(json, i)
          if i < json.len and json[i] == ',': i += 1
        if i < json.len: i += 1  # skip ']'
    else:
      skipValue(json, i)

    skipWS(json, i)
    if i < json.len and json[i] == ',': i += 1

# ---------------------------------------------------------------------------
# Audit logging
# ---------------------------------------------------------------------------
proc logAudit(eventType, subject, action, result_: string) =
  ## Append one audit record: timestamp|event_type|subject|action|result
  let fd = open(AUDIT_LOG.cstring, O_WRONLY or O_CREAT or O_APPEND, 0o640)
  if fd < 0: return

  # Build ISO-8601-ish timestamp from clock_gettime
  var ts: Timespec
  discard clock_gettime(CLOCK_REALTIME, ts)
  let sec  = ts.tv_sec
  let usec = ts.tv_nsec div 1_000_000

  # Simple decimal timestamp: seconds.milliseconds (no stdlib time formatting)
  let line = $sec & "." & $usec & "|" & eventType & "|" &
             subject & "|" & action & "|" & result_ & "\n"
  discard write(fd, cast[pointer](unsafeAddr line[0]), line.len)
  discard close(fd)

# ---------------------------------------------------------------------------
# Read a file into a string (low-level)
# ---------------------------------------------------------------------------
proc readFile(path: string): string =
  let fd = open(path.cstring, O_RDONLY)
  if fd < 0: return ""
  var buf = newString(65536)
  var total = 0
  while true:
    let n = read(fd, addr buf[total], min(4096, buf.len - total))
    if n <= 0: break
    total += n
    if total >= buf.len: break
  discard close(fd)
  buf.setLen(total)
  return buf

# ---------------------------------------------------------------------------
# Build bwrap argv
# ---------------------------------------------------------------------------
proc buildBwrapArgs(manifest: ManifestFields): seq[string] =
  result = @[]

  # baseline sandbox flags
  result.add(BWRAP_PATH)
  result.add("--ro-bind"); result.add("/usr"); result.add("/usr")
  result.add("--ro-bind"); result.add("/lib"); result.add("/lib")

  # /lib64 if it exists (common on x86-64)
  result.add("--ro-bind-try"); result.add("/lib64"); result.add("/lib64")

  result.add("--tmpfs"); result.add("/tmp")
  result.add("--proc"); result.add("/proc")
  result.add("--dev"); result.add("/dev")
  result.add("--unshare-all")
  result.add("--die-with-parent")

  # hostname
  result.add("--hostname"); result.add("sigma-sandbox")

  # map capabilities
  for cap in manifest.capabilities:
    var matched = false
    for (capName, bwrapFlags) in CAP_MAP:
      if cap == capName:
        matched = true
        # bwrapFlags may be a multi-token string like "--ro-bind /home /home"
        for tok in bwrapFlags.splitWhitespace():
          result.add(tok)
        logAudit("capability-grant", manifest.name, "grant:" & cap, "allowed")
        break

    if not matched:
      logAudit("capability-violation", manifest.name, "request:" & cap, "denied")

  # The command to run inside the sandbox (split on spaces naively)
  # For robust splitting you'd want shell quoting; this matches the spec's simplicity
  for tok in manifest.command.splitWhitespace():
    result.add(tok)

# ---------------------------------------------------------------------------
# Exec bwrap
# ---------------------------------------------------------------------------
proc execBwrap(args: seq[string]) =
  # Build C argv array
  var argv = newSeq[cstring](args.len + 1)
  for i, a in args:
    argv[i] = a.cstring
  argv[args.len] = nil

  logAudit("exec", args[0], "execv", "attempt")
  discard execv(BWRAP_PATH, cast[cstringArray](addr argv[0]))

  # If we're here, execv failed
  logAudit("exec", args[0], "execv", "failed:errno=" & $errno)
  quit("execv failed: " & $strerror(errno), 1)

# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------
proc main() =
  let paramCount = paramCount()
  if paramCount < 1:
    quit("Usage: sigmad-sandbox <manifest.json>", 1)

  let manifestPath = paramStr(1)
  let json = readFile(manifestPath)
  if json.len == 0:
    logAudit("parse-error", manifestPath, "read", "failed")
    quit("Failed to read manifest: " & manifestPath, 1)

  let manifest = parseManifest(json)

  if manifest.name.len == 0:
    logAudit("parse-error", manifestPath, "parse:name", "missing")
    quit("Manifest missing 'name' field", 1)

  if manifest.command.len == 0:
    logAudit("parse-error", manifestPath, "parse:command", "missing")
    quit("Manifest missing 'command' field", 1)

  logAudit("sandbox-start", manifest.name, manifest.command, "starting")

  let args = buildBwrapArgs(manifest)
  execBwrap(args)

main()
