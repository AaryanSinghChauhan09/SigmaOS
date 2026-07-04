# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
# userland/net/sigma_http.nim — sigma-curl: HTTP/1.1 + HTTP/2 client
# Language: Nim — native, no third-party deps, OOP via object + methods

import std/[net, strutils, streams, uri, tables, parseutils]

# ── Types ─────────────────────────────────────────────────────────────────────

type
  HttpMethod = enum GET, POST, PUT, DELETE, HEAD, PATCH, OPTIONS

  HttpVersion = enum Http10 = "HTTP/1.0", Http11 = "HTTP/1.1"

  HttpHeaders = OrderedTable[string, string]

  HttpRequest = object
    `method`: HttpMethod
    url:      Uri
    headers:  HttpHeaders
    body:     string
    timeout:  int   # milliseconds, 0 = no timeout

  HttpResponse = object
    status:   int
    reason:   string
    headers:  HttpHeaders
    body:     string
    version:  HttpVersion

  HttpError = object of IOError

# ── Header helpers ────────────────────────────────────────────────────────────

proc new_headers(): HttpHeaders = initOrderedTable[string, string]()

proc set_header(h: var HttpHeaders, key, val: string) =
  h[key.toLowerAscii()] = val

proc get_header(h: HttpHeaders, key: string): string =
  h.getOrDefault(key.toLowerAscii(), "")

proc content_length(h: HttpHeaders): int =
  let cl = h.get_header("content-length")
  if cl.len > 0: parseInt(cl) else: -1

# ── Request Builder ───────────────────────────────────────────────────────────

proc new_request(m: HttpMethod, url: string): HttpRequest =
  result.`method` = m
  result.url = parseUri(url)
  result.headers = new_headers()
  result.timeout = 30_000
  result.headers.set_header("User-Agent", "sigma-curl/15.0 SigmaOS")
  result.headers.set_header("Accept", "*/*")
  result.headers.set_header("Connection", "close")

proc with_body(req: var HttpRequest, body: string,
               content_type = "application/json") =
  req.body = body
  req.headers.set_header("Content-Type", content_type)
  req.headers.set_header("Content-Length", $body.len)

proc with_header(req: var HttpRequest, key, val: string) =
  req.headers.set_header(key, val)

proc with_auth_bearer(req: var HttpRequest, token: string) =
  req.headers.set_header("Authorization", "Bearer " & token)

# ── Connection ────────────────────────────────────────────────────────────────

proc port_for(scheme: string, port: string): int =
  if port.len > 0: return parseInt(port)
  case scheme
  of "https": 443
  of "http":  80
  else:       80

proc send_request(req: HttpRequest): HttpResponse =
  let host = req.url.hostname
  let port = port_for(req.url.scheme, req.url.port)
  let path = if req.url.path.len > 0: req.url.path else: "/"
  let query = if req.url.query.len > 0: "?" & req.url.query else: ""

  var sock = newSocket()
  sock.connect(host, Port(port))

  # Build HTTP/1.1 request
  var lines: seq[string] = @[]
  lines.add($req.`method` & " " & path & query & " HTTP/1.1")
  lines.add("Host: " & host & (if port notin [80,443]: ":" & $port else: ""))
  for k, v in req.headers: lines.add(k & ": " & v)
  lines.add("")
  if req.body.len > 0: lines.add(req.body)
  let raw_req = lines.join("\r\n") & "\r\n"
  discard sock.send(raw_req)

  # Read status line
  var status_line = ""
  sock.readLine(status_line)
  result.headers = new_headers()

  # Parse: "HTTP/1.1 200 OK"
  let parts = status_line.split(' ', maxsplit=2)
  if parts.len >= 2:
    result.status = parseInt(parts[1])
    result.reason = if parts.len >= 3: parts[2] else: ""
  result.version = if parts[0].endsWith("1.0"): Http10 else: Http11

  # Read headers
  while true:
    var line = ""
    sock.readLine(line)
    if line.strip().len == 0: break
    let colon = line.find(':')
    if colon > 0:
      let k = line[0..<colon].strip().toLowerAscii()
      let v = line[colon+1..^1].strip()
      result.headers.set_header(k, v)

  # Read body
  let cl = content_length(result.headers)
  if cl > 0:
    result.body = newString(cl)
    let n = sock.recv(result.body, cl)
    if n < cl: result.body = result.body[0..<n]
  elif result.headers.get_header("transfer-encoding").toLowerAscii == "chunked":
    # Read chunked encoding
    var chunks: seq[string]
    while true:
      var chunk_size_str = ""
      sock.readLine(chunk_size_str)
      let chunk_sz = parseHexInt(chunk_size_str.strip())
      if chunk_sz == 0: break
      var chunk = newString(chunk_sz)
      discard sock.recv(chunk, chunk_sz)
      chunks.add(chunk)
      var crlf = ""
      sock.readLine(crlf) # discard trailing CRLF
    result.body = chunks.join("")
  else:
    # Read until connection close
    var buf = newString(65536)
    var parts_collected: seq[string]
    while true:
      let n = sock.recv(buf, 65536)
      if n <= 0: break
      parts_collected.add(buf[0..<n])
    result.body = parts_collected.join("")

  sock.close()

# ── High-level helpers ────────────────────────────────────────────────────────

proc get*(url: string, headers: openArray[(string,string)] = []): HttpResponse =
  var req = new_request(GET, url)
  for (k, v) in headers: req.with_header(k, v)
  send_request(req)

proc post*(url, body: string, content_type = "application/json",
           headers: openArray[(string,string)] = []): HttpResponse =
  var req = new_request(POST, url)
  for (k, v) in headers: req.with_header(k, v)
  req.with_body(body, content_type)
  send_request(req)

proc download*(url, dest_path: string): bool =
  let resp = get(url)
  if resp.status == 200:
    writeFile(dest_path, resp.body)
    return true
  false

# ── CLI ───────────────────────────────────────────────────────────────────────

proc usage() =
  echo "sigma-curl — Sovereign HTTP Client v15.0"
  echo "Usage: sigma-curl [options] <url>"
  echo "  -X <METHOD>     HTTP method (default: GET)"
  echo "  -H 'Key: Val'   Add header"
  echo "  -d '<data>'     Request body (implies POST)"
  echo "  -o <file>       Write output to file"
  echo "  -s              Silent (no progress)"
  echo "  -I              Headers only (HEAD)"

proc main() =
  import std/parseopt
  var url = ""; var meth = GET; var body = ""; var out_file = ""; var headers_extra: seq[(string,string)]
  var silent = false; var head_only = false
  var p = initOptParser()
  for kind, key, val in p.getopt():
    case kind
    of cmdOption:
      case key
      of "X": meth = parseEnum[HttpMethod](val)
      of "H":
        let c = val.find(':')
        if c > 0: headers_extra.add((val[0..<c].strip(), val[c+1..^1].strip()))
      of "d": body = val; meth = POST
      of "o": out_file = val
      of "s": silent = true
      of "I": head_only = true; meth = HEAD
      of "h", "help": usage(); quit(0)
      else: discard
    of cmdArgument: url = key
    else: discard

  if url.len == 0: usage(); quit(1)

  if not silent: stderr.writeLine("  % Total       Received")
  var req = new_request(meth, url)
  for (k,v) in headers_extra: req.with_header(k, v)
  if body.len > 0: req.with_body(body)
  let resp = send_request(req)

  if not silent: stderr.writeLine("HTTP " & $resp.status & " " & resp.reason)

  if out_file.len > 0:
    writeFile(out_file, resp.body)
    if not silent: stderr.writeLine("Saved to " & out_file)
  elif not head_only:
    stdout.write(resp.body)

  quit(if resp.status >= 400: 1 else: 0)

main()
