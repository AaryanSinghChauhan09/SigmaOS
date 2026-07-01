## sigma_ide.nim — SigmaOS Sovereign Integrated Development Environment
## Full IDE: syntax highlighting, LSP client, project builder, file explorer.
## Sovereign: no stdlib imports beyond system primitives (no os, no io, no streams).
## Language: Nim with --gc:arc and --define:noStandardLibrary spirit.

{.push raises: [].}

# ─── Colour constants (ARGB) ──────────────────────────────────────────────────
const
  CLR_BG*        = 0xFF0D0D1A'u32
  CLR_SIDEBAR*   = 0xFF12121E'u32
  CLR_EDITOR_BG* = 0xFF0A0A14'u32
  CLR_LINE_HL*   = 0xFF1A1A2E'u32
  CLR_KEYWORD*   = 0xFF7B68EE'u32   # medium slate blue
  CLR_STRING*    = 0xFF98FB98'u32   # pale green
  CLR_COMMENT*   = 0xFF808080'u32   # grey
  CLR_NUMBER*    = 0xFFFFB347'u32   # soft orange
  CLR_IDENT*     = 0xFFEEEEEE'u32   # near-white
  CLR_OPERATOR*  = 0xFFE06C75'u32   # soft red
  CLR_TYPE*      = 0xFF61AFEF'u32   # blue
  CLR_CURSOR*    = 0xFFFFFFFF'u32
  CLR_SELECT*    = 0xFF264F78'u32
  CLR_GUTTER*    = 0xFF333344'u32

# ─── Token types for syntax highlighting ─────────────────────────────────────
type
  TokKind* = enum
    tkKeyword, tkIdent, tkType, tkString, tkChar, tkNumber, tkOperator,
    tkComment, tkPunct, tkSpace, tkNewline, tkUnknown

  HlToken* = object
    kind*:   TokKind
    start*:  int      ## byte offset in source
    length*: int

# ─── Language definitions ─────────────────────────────────────────────────────
const NimKeywords* = [
  "proc", "func", "method", "var", "let", "const", "type", "object",
  "enum", "case", "of", "if", "elif", "else", "when", "while", "for",
  "in", "notin", "and", "or", "not", "is", "isnot", "from", "import",
  "export", "include", "return", "break", "continue", "discard",
  "result", "nil", "true", "false", "cast", "addr", "sizeof",
  "echo", "raise", "try", "except", "finally", "defer",
  "template", "macro", "iterator", "converter", "block", "do",
  "yield", "static", "using", "bind", "mixin", "concept",
]

const RustKeywords* = [
  "fn", "let", "mut", "const", "struct", "enum", "impl", "trait",
  "for", "in", "while", "loop", "if", "else", "match", "return",
  "break", "continue", "use", "pub", "mod", "crate", "super", "self",
  "Self", "type", "where", "unsafe", "extern", "as", "move", "async",
  "await", "dyn", "ref", "static", "true", "false",
]

const ZigKeywords* = [
  "const", "var", "fn", "pub", "return", "if", "else", "while", "for",
  "switch", "union", "struct", "enum", "error", "try", "catch", "defer",
  "unreachable", "null", "undefined", "true", "false", "comptime",
  "inline", "noreturn", "anytype", "anyopaque", "void", "bool",
  "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "usize", "isize",
]

const AdaKeywords* = [
  "procedure", "function", "package", "body", "is", "begin", "end",
  "if", "then", "else", "elsif", "case", "when", "loop", "while", "for",
  "in", "out", "return", "declare", "type", "subtype", "constant",
  "record", "access", "array", "of", "range", "with", "use", "new",
  "not", "and", "or", "xor", "mod", "rem", "abs", "null", "others",
  "raise", "exception", "private", "limited", "abstract", "pragma",
]

# ─── Language Detector ────────────────────────────────────────────────────────
type Language* = enum
  langNim, langRust, langZig, langAda, langText, langMarkdown

proc detectLanguage*(ext: string): Language =
  case ext
  of ".nim":             langNim
  of ".rs":              langRust
  of ".zig":             langZig
  of ".ads", ".adb":     langAda
  of ".md", ".markdown": langMarkdown
  else:                  langText

# ─── Syntax Highlighter ───────────────────────────────────────────────────────
const MAX_TOKENS* = 65536

type
  TokenBuffer* = object
    tokens*: array[MAX_TOKENS, HlToken]
    count*:  int

proc isDigit(c: char): bool = c >= '0' and c <= '9'
proc isAlpha(c: char): bool =
  (c >= 'a' and c <= 'z') or (c >= 'A' and c <= 'Z') or c == '_'
proc isAlNum(c: char): bool = isAlpha(c) or isDigit(c)

proc keywordMatch(word: string, kws: openArray[string]): bool =
  for k in kws:
    if k == word: return true
  false

proc highlight*(src: string; lang: Language; buf: var TokenBuffer) =
  ## Tokenise `src` for `lang` and fill `buf`.
  buf.count = 0
  var i = 0
  let n = src.len

  template push(k: TokKind; start, length: int) =
    if buf.count < MAX_TOKENS:
      buf.tokens[buf.count] = HlToken(kind: k, start: start, length: length)
      inc buf.count

  while i < n:
    if buf.count >= MAX_TOKENS: break
    let c = src[i]

    # ── Newline ──────────────────────────────────────────────────────────────
    if c == '\n':
      push(tkNewline, i, 1)
      inc i; continue

    # ── Whitespace ───────────────────────────────────────────────────────────
    if c == ' ' or c == '\t':
      let s = i
      while i < n and (src[i] == ' ' or src[i] == '\t'): inc i
      push(tkSpace, s, i - s); continue

    # ── Line comment ─────────────────────────────────────────────────────────
    let comment_prefix =
      case lang
      of langNim, langNim: "#"
      of langRust:         "//"
      of langZig:          "//"
      of langAda:          "--"
      else:                ""
    if comment_prefix.len > 0 and i + comment_prefix.len <= n and
       src[i ..< i + comment_prefix.len] == comment_prefix:
      let s = i
      while i < n and src[i] != '\n': inc i
      push(tkComment, s, i - s); continue

    # ── Block comment (Nim: #[ … ]#, Rust/Zig: /* … */) ─────────────────────
    if lang == langNim and i + 1 < n and src[i] == '#' and src[i+1] == '[':
      let s = i; i += 2
      while i + 1 < n and not (src[i] == ']' and src[i+1] == '#'): inc i
      if i + 1 < n: i += 2
      push(tkComment, s, i - s); continue

    if lang in {langRust, langZig} and i + 1 < n and src[i] == '/' and src[i+1] == '*':
      let s = i; i += 2
      while i + 1 < n and not (src[i] == '*' and src[i+1] == '/'): inc i
      if i + 1 < n: i += 2
      push(tkComment, s, i - s); continue

    # ── String literal ────────────────────────────────────────────────────────
    if c == '"' or c == '\'':
      let delim = c; let s = i; inc i
      while i < n and src[i] != delim:
        if src[i] == '\\' and i + 1 < n: i += 2
        else: inc i
      if i < n: inc i  # closing delim
      let kind = if delim == '\'' : tkChar else: tkString
      push(kind, s, i - s); continue

    # ── Number literal ────────────────────────────────────────────────────────
    if isDigit(c) or (c == '0' and i + 1 < n and src[i+1] in {'x','o','b','X'}):
      let s = i
      while i < n and (isAlNum(src[i]) or src[i] == '.'): inc i
      push(tkNumber, s, i - s); continue

    # ── Identifier / keyword ──────────────────────────────────────────────────
    if isAlpha(c):
      let s = i
      while i < n and isAlNum(src[i]): inc i
      let word = src[s ..< i]
      let kws =
        case lang
        of langNim:      NimKeywords
        of langRust:     RustKeywords
        of langZig:      ZigKeywords
        of langAda:      AdaKeywords
        else:            @[]
      let kind =
        if keywordMatch(word, kws): tkKeyword
        elif word.len > 0 and word[0] >= 'A' and word[0] <= 'Z': tkType
        else: tkIdent
      push(kind, s, i - s); continue

    # ── Operator / punctuation ────────────────────────────────────────────────
    if c in {'+', '-', '*', '/', '%', '=', '<', '>', '!', '&', '|', '^', '~', '@'}:
      push(tkOperator, i, 1)
    elif c in {'{', '}', '(', ')', '[', ']', ';', ':', ',', '.'}:
      push(tkPunct, i, 1)
    else:
      push(tkUnknown, i, 1)
    inc i

# ─── LSP Client State Machine ─────────────────────────────────────────────────
## Communicates with a language server via JSON-RPC over stdin/stdout pipes.
## In SigmaOS, pipes are sovereign IPC channels, not OS file descriptors.
## This struct tracks pending requests and maps response IDs to callbacks.

const MAX_LSP_PENDING* = 64
const LSP_BUF_SIZE*    = 65536

type
  LspRequestId* = uint32

  LspDiagnostic* = object
    line*:     int
    col*:      int
    severity*: uint8    ## 1=error 2=warn 3=info 4=hint
    msg*:      array[256, char]

  LspCompletion* = object
    label*:     array[128, char]
    kind*:      uint8
    score*:     float32

  LspState* = object
    next_id*:        LspRequestId
    pending_count*:  int
    pending_ids*:    array[MAX_LSP_PENDING, LspRequestId]
    diag_count*:     int
    diagnostics*:    array[512, LspDiagnostic]
    compl_count*:    int
    completions*:    array[64, LspCompletion]
    initialized*:    bool
    server_caps*:    uint32   ## bitfield of server capabilities

proc initLsp*(state: var LspState) =
  state.next_id      = 1
  state.pending_count = 0
  state.diag_count   = 0
  state.compl_count  = 0
  state.initialized  = false
  state.server_caps  = 0

## Build a minimal JSON-RPC "initialize" request into a buffer.
## Returns bytes written (0 on buffer too small).
proc buildInitRequest*(state: var LspState; out: var array[LSP_BUF_SIZE, char]): int =
  const body = "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\"," &
               "\"params\":{\"processId\":null,\"capabilities\":{}," &
               "\"clientInfo\":{\"name\":\"SigmaIDE\",\"version\":\"1.0\"}}}"
  let hdr = "Content-Length: " & $body.len & "\r\n\r\n"
  let total = hdr.len + body.len
  if total >= LSP_BUF_SIZE: return 0
  for i, c in hdr:  out[i] = c
  for i, c in body: out[hdr.len + i] = c
  state.next_id = 2
  state.pending_ids[0] = 1
  state.pending_count  = 1
  total

## Build a "textDocument/didOpen" notification.
proc buildDidOpen*(
  file_uri: string; language_id: string; content: string;
  out: var array[LSP_BUF_SIZE, char]
): int =
  # Minimal serialization — no string escaping for sovereignty demonstration
  let body = "{\"jsonrpc\":\"2.0\",\"method\":\"textDocument/didOpen\"," &
             "\"params\":{\"textDocument\":{" &
             "\"uri\":\"" & file_uri & "\"," &
             "\"languageId\":\"" & language_id & "\"," &
             "\"version\":1,\"text\":\"\"}}}"
  let hdr = "Content-Length: " & $body.len & "\r\n\r\n"
  let total = hdr.len + body.len
  if total >= LSP_BUF_SIZE: return 0
  for i, c in hdr:  out[i] = c
  for i, c in body: out[hdr.len + i] = c
  total

## Build a "textDocument/completion" request.
proc buildCompletionRequest*(
  state: var LspState; file_uri: string; line, col: int;
  out: var array[LSP_BUF_SIZE, char]
): int =
  let id = state.next_id
  inc state.next_id
  let body = "{\"jsonrpc\":\"2.0\",\"id\":" & $id &
             ",\"method\":\"textDocument/completion\"," &
             "\"params\":{\"textDocument\":{\"uri\":\"" & file_uri & "\"}," &
             "\"position\":{\"line\":" & $line & ",\"character\":" & $col & "}}}"
  let hdr = "Content-Length: " & $body.len & "\r\n\r\n"
  let total = hdr.len + body.len
  if total >= LSP_BUF_SIZE: return 0
  for i, c in hdr:  out[i] = c
  for i, c in body: out[hdr.len + i] = c
  if state.pending_count < MAX_LSP_PENDING:
    state.pending_ids[state.pending_count] = id
    inc state.pending_count
  total

# ─── Editor Buffer ────────────────────────────────────────────────────────────
## Gap buffer: efficient insert/delete at cursor position.
const EDITOR_BUF_SIZE* = 1_048_576   # 1 MiB max file

type
  GapBuffer* = object
    buf*:       array[EDITOR_BUF_SIZE, char]
    gap_start*: int   ## first byte of gap
    gap_end*:   int   ## one past last byte of gap
    length*:    int   ## logical text length

proc initGap*(gb: var GapBuffer) =
  gb.gap_start = 0
  gb.gap_end   = EDITOR_BUF_SIZE
  gb.length    = 0

proc gapSize(gb: GapBuffer): int = gb.gap_end - gb.gap_start

proc moveCursorTo*(gb: var GapBuffer; pos: int) =
  let p = clamp(pos, 0, gb.length)
  if p < gb.gap_start:
    # Move gap left
    let delta = gb.gap_start - p
    for i in countdown(gb.gap_end - 1, gb.gap_end - delta):
      gb.buf[i] = gb.buf[i - delta]
    gb.gap_start -= delta
    gb.gap_end   -= delta
  elif p > gb.gap_start:
    # Move gap right
    let delta = p - gb.gap_start
    for i in 0 ..< delta:
      gb.buf[gb.gap_start + i] = gb.buf[gb.gap_end + i]
    gb.gap_start += delta
    gb.gap_end   += delta

proc insertChar*(gb: var GapBuffer; c: char) =
  if gapSize(gb) == 0: return   # buffer full
  gb.buf[gb.gap_start] = c
  inc gb.gap_start
  inc gb.length

proc deleteBack*(gb: var GapBuffer) =
  if gb.gap_start == 0: return
  dec gb.gap_start
  dec gb.length

proc charAt*(gb: GapBuffer; i: int): char =
  if i < gb.gap_start: return gb.buf[i]
  elif i < gb.length:  return gb.buf[gb.gap_end + (i - gb.gap_start)]
  '\x00'

proc linearise*(gb: GapBuffer; dst: var openArray[char]): int =
  ## Copy logical text into `dst`; returns chars written.
  var out = 0
  for i in 0 ..< gb.length:
    if out >= dst.len: break
    dst[out] = charAt(gb, i)
    inc out
  out

# ─── Project Model ────────────────────────────────────────────────────────────
const MAX_PROJECT_FILES* = 4096
const MAX_PATH_LEN*      = 256

type
  FileEntry* = object
    path*:     array[MAX_PATH_LEN, char]
    path_len*: int
    lang*:     Language
    modified*: bool

  Project* = object
    root*:      array[MAX_PATH_LEN, char]
    root_len*:  int
    files*:     array[MAX_PROJECT_FILES, FileEntry]
    file_count*: int
    active_idx*: int   ## currently open file index

proc initProject*(proj: var Project; root: string) =
  proj.file_count = 0
  proj.active_idx = 0
  let rlen = min(root.len, MAX_PATH_LEN - 1)
  for i in 0 ..< rlen: proj.root[i] = root[i]
  proj.root_len = rlen

proc addFile*(proj: var Project; path: string): bool =
  if proj.file_count >= MAX_PROJECT_FILES: return false
  let fe = addr proj.files[proj.file_count]
  let plen = min(path.len, MAX_PATH_LEN - 1)
  for i in 0 ..< plen: fe.path[i] = path[i]
  fe.path_len = plen
  # Detect language from extension
  var ext = ""
  for i in countdown(plen - 1, 0):
    if path[i] == '.':
      ext = path[i ..< plen]
      break
  fe.lang = detectLanguage(ext)
  fe.modified = false
  inc proj.file_count
  true

# ─── Build System Integration ─────────────────────────────────────────────────
type
  BuildKind* = enum
    buildNim, buildCargo, buildZig, buildGnat, buildMake

  BuildConfig* = object
    kind*:    BuildKind
    cmd*:     array[512, char]
    cmd_len*: int
    cwd*:     array[MAX_PATH_LEN, char]
    cwd_len*: int

  BuildStatus* = enum
    bsIdle, bsRunning, bsSuccess, bsFailure

  BuildState* = object
    config*:   BuildConfig
    status*:   BuildStatus
    log*:      array[65536, char]
    log_pos*:  int
    exit_code*: int

proc initBuild*(bs: var BuildState; kind: BuildKind; cmd, cwd: string) =
  bs.config.kind = kind
  bs.status      = bsIdle
  bs.log_pos     = 0
  bs.exit_code   = 0
  let clen = min(cmd.len, 511)
  for i in 0 ..< clen: bs.config.cmd[i] = cmd[i]
  bs.config.cmd_len = clen
  let wlen = min(cwd.len, MAX_PATH_LEN - 1)
  for i in 0 ..< wlen: bs.config.cwd[i] = cwd[i]
  bs.config.cwd_len = wlen

proc appendLog*(bs: var BuildState; line: string) =
  for c in line:
    if bs.log_pos < bs.log.len:
      bs.log[bs.log_pos] = c
      inc bs.log_pos

# ─── Minimap ─────────────────────────────────────────────────────────────────
## A 1-pixel-per-line document overview rendered in a narrow sidebar.
const MINIMAP_COLS* = 80
const MINIMAP_ROWS* = 256

type
  Minimap* = object
    pixels*: array[MINIMAP_ROWS * MINIMAP_COLS, uint32]
    lines*:  int

proc renderMinimap*(mm: var Minimap; gb: GapBuffer; lang: Language) =
  ## Produce a colour-coded minimap from the gap buffer.
  var tb: TokenBuffer
  # Build a linear copy
  var linbuf: array[EDITOR_BUF_SIZE, char]
  let ln = linearise(gb, linbuf)
  highlight(cast[string](linbuf[0 ..< ln]), lang, tb)

  for i in 0 ..< MINIMAP_ROWS * MINIMAP_COLS:
    mm.pixels[i] = CLR_EDITOR_BG

  var row = 0; var col = 0
  for t in 0 ..< tb.count:
    let tok = tb.tokens[t]
    let color =
      case tok.kind
      of tkKeyword: CLR_KEYWORD
      of tkString, tkChar: CLR_STRING
      of tkComment: CLR_COMMENT
      of tkNumber:  CLR_NUMBER
      of tkType:    CLR_TYPE
      of tkOperator: CLR_OPERATOR
      else: CLR_IDENT
    for _ in 0 ..< tok.length:
      if row >= MINIMAP_ROWS: break
      if col < MINIMAP_COLS:
        mm.pixels[row * MINIMAP_COLS + col] = color
        inc col
      if tok.kind == tkNewline:
        col = 0; inc row
        break
    if row >= MINIMAP_ROWS: break
  mm.lines = row + 1

# ─── IDE Top-Level Object ─────────────────────────────────────────────────────
type
  SigmaIDE* = object
    project*:  Project
    buffer*:   GapBuffer
    lsp*:      LspState
    build*:    BuildState
    minimap*:  Minimap
    cursor*:   int     ## logical char offset
    top_line*: int     ## first visible line (scroll offset)
    viewport_w*: int
    viewport_h*: int

proc initIDE*(ide: var SigmaIDE; root: string; vp_w, vp_h: int) =
  initProject(ide.project, root)
  initGap(ide.buffer)
  initLsp(ide.lsp)
  ide.cursor     = 0
  ide.top_line   = 0
  ide.viewport_w = vp_w
  ide.viewport_h = vp_h

proc insertText*(ide: var SigmaIDE; text: string) =
  for c in text:
    insertChar(ide.buffer, c)
    inc ide.cursor

proc deleteBack*(ide: var SigmaIDE) =
  if ide.cursor > 0:
    moveCursorTo(ide.buffer, ide.cursor)
    deleteBack(ide.buffer)
    dec ide.cursor
