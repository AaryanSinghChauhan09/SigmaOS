# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
# userland/shell/sigma_scripting.nim — sigma-sh scripting: variables, funcs, loops
# Language: Nim — native, OOP via object + methods

import std/[os, strutils, tables, sequtils, parseutils, streams]

# ── Token Types ───────────────────────────────────────────────────────────────
type
  TokKind = enum
    TWord, TString, TSemi, TNewline, TPipe, TRedirOut, TRedirIn,
    TRedirAppend, TAnd, TOr, TIf, TElse, TFi, TFor, TDo, TDone,
    TWhile, TFunction, TLBrace, TRBrace, TLParen, TRParen, TEOF

  Token = object
    kind:  TokKind
    value: string

  Lexer = object
    src:   string
    pos:   int

# ── Lexer ─────────────────────────────────────────────────────────────────────
proc peek(l: Lexer): char =
  if l.pos < l.src.len: l.src[l.pos] else: '\0'

proc advance(l: var Lexer): char =
  result = l.peek(); l.pos += 1

proc skip_whitespace(l: var Lexer) =
  while l.peek() in {' ', '\t'}: discard l.advance()

proc read_word(l: var Lexer): Token =
  var w = ""
  while l.peek() notin {' ','\t','\n',';','|','&','<','>','{','}','(',')','\0'}:
    w.add(l.advance())
  result = Token(kind: TWord, value: w)

proc read_string(l: var Lexer, delim: char): Token =
  discard l.advance() # consume opening quote
  var s = ""
  while l.peek() != delim and l.peek() != '\0': s.add(l.advance())
  if l.peek() == delim: discard l.advance()
  Token(kind: TString, value: s)

proc next_token(l: var Lexer): Token =
  l.skip_whitespace()
  let c = l.peek()
  case c
  of '\0': Token(kind: TEOF, value: "")
  of '\n': discard l.advance(); Token(kind: TNewline, value: "\n")
  of ';':  discard l.advance(); Token(kind: TSemi, value: ";")
  of '|':
    discard l.advance()
    if l.peek() == '|': discard l.advance(); Token(kind: TOr, value: "||")
    else:               Token(kind: TPipe, value: "|")
  of '&':
    discard l.advance()
    if l.peek() == '&': discard l.advance(); Token(kind: TAnd, value: "&&")
    else:               Token(kind: TWord, value: "&")
  of '>':
    discard l.advance()
    if l.peek() == '>': discard l.advance(); Token(kind: TRedirAppend, value: ">>")
    else:               Token(kind: TRedirOut, value: ">")
  of '<':  discard l.advance(); Token(kind: TRedirIn, value: "<")
  of '{':  discard l.advance(); Token(kind: TLBrace, value: "{")
  of '}':  discard l.advance(); Token(kind: TRBrace, value: "}")
  of '(':  discard l.advance(); Token(kind: TLParen, value: "(")
  of ')':  discard l.advance(); Token(kind: TRParen, value: ")")
  of '"':  l.read_string('"')
  of '\'': l.read_string('\'')
  of '#':  # Comment: skip to end of line
    while l.peek() notin {'\n', '\0'}: discard l.advance()
    next_token(l)
  else:
    let w = l.read_word()
    case w.value
    of "if":       Token(kind: TIf,       value: "if")
    of "else":     Token(kind: TElse,     value: "else")
    of "fi":       Token(kind: TFi,       value: "fi")
    of "for":      Token(kind: TFor,      value: "for")
    of "do":       Token(kind: TDo,       value: "do")
    of "done":     Token(kind: TDone,     value: "done")
    of "while":    Token(kind: TWhile,    value: "while")
    of "function": Token(kind: TFunction, value: "function")
    else:          w

# ── Interpreter ───────────────────────────────────────────────────────────────
type
  ScriptEnv = object
    vars:     Table[string, string]
    funcs:    Table[string, string]  # name → body source
    cwd:      string
    last_exit: int
    args:     seq[string]

proc new_env(cwd: string, args: seq[string] = @[]): ScriptEnv =
  result.vars   = initTable[string, string]()
  result.funcs  = initTable[string, string]()
  result.cwd    = cwd
  result.args   = args
  result.vars["?"] = "0"
  result.vars["0"] = if args.len > 0: args[0] else: "sigma-sh"
  for i, a in args: result.vars[$i] = a

proc expand_var(env: ScriptEnv, name: string): string =
  case name
  of "?":  return $env.last_exit
  of "0":  return env.vars.getOrDefault("0", "sigma-sh")
  of "HOME", "PWD", "PATH", "USER", "SHELL":
    return getEnv(name, env.vars.getOrDefault(name, ""))
  else:
    return env.vars.getOrDefault(name, "")

proc expand(env: ScriptEnv, s: string): string =
  var out = ""
  var i = 0
  while i < s.len:
    if s[i] == '$':
      i += 1
      if i < s.len and s[i] == '{':
        i += 1
        var name = ""
        while i < s.len and s[i] != '}': name.add(s[i]); i += 1
        if i < s.len: i += 1
        out.add(expand_var(env, name))
      else:
        var name = ""
        while i < s.len and s[i] in {'a'..'z','A'..'Z','0'..'9','_'}: name.add(s[i]); i += 1
        out.add(expand_var(env, name))
    elif s[i] == '~' and (i == 0 or s[i-1] == ':'):
      out.add(getEnv("HOME", "/home/sovereign")); i += 1
    else:
      out.add(s[i]); i += 1
  out

proc exec_cmd(env: var ScriptEnv, args: seq[string]): int =
  if args.len == 0: return 0
  let cmd = args[0]
  let expanded: seq[string] = args.mapIt(expand(env, it))
  case expanded[0]
  of "cd":
    let target = if expanded.len > 1: expanded[1] else: getEnv("HOME", "/")
    if dirExists(target): env.cwd = target; setCurrentDir(target); return 0
    else: stderr.writeLine("cd: no such directory: " & target); return 1
  of "export":
    for a in expanded[1..^1]:
      let eq = a.find('=')
      if eq > 0: env.vars[a[0..<eq]] = a[eq+1..^1]
    return 0
  of "unset":
    for a in expanded[1..^1]: env.vars.del(a)
    return 0
  of "echo":
    echo expanded[1..^1].join(" "); return 0
  of "exit":
    quit(if expanded.len > 1: parseInt(expanded[1]) else: env.last_exit)
  of "true":  return 0
  of "false": return 1
  of "test", "[":
    # Basic test: -f file, -d dir, -z str, str1 = str2
    if expanded.len < 2: return 1
    if expanded[1] == "-f": return if expanded.len>2 and fileExists(expanded[2]):  0 else: 1
    if expanded[1] == "-d": return if expanded.len>2 and dirExists(expanded[2]):   0 else: 1
    if expanded[1] == "-z": return if expanded.len>2 and expanded[2].len == 0:    0 else: 1
    if expanded[1] == "-n": return if expanded.len>2 and expanded[2].len > 0:     0 else: 1
    if expanded.len >= 4 and expanded[2] == "=":  return if expanded[1] == expanded[3]: 0 else: 1
    if expanded.len >= 4 and expanded[2] == "!=": return if expanded[1] != expanded[3]: 0 else: 1
    return 1
  else:
    # External command
    try:
      return execShellCmd(expanded.join(" "))
    except:
      stderr.writeLine(expanded[0] & ": command not found"); return 127

proc run_script*(source: string, cwd: string = getCurrentDir(),
                 args: seq[string] = @[]): int =
  var env = new_env(cwd, args)
  var lex = Lexer(src: source, pos: 0)
  var line_words: seq[string]
  var last_exit = 0

  proc flush_line(words: var seq[string], e: var ScriptEnv): int =
    if words.len == 0: return 0
    let r = exec_cmd(e, words)
    e.last_exit = r
    e.vars["?"] = $r
    words = @[]
    r

  while true:
    let tok = next_token(lex)
    case tok.kind
    of TEOF:
      last_exit = flush_line(line_words, env)
      break
    of TNewline, TSemi:
      last_exit = flush_line(line_words, env)
    of TWord, TString:
      line_words.add(tok.value)
    of TAnd:
      last_exit = flush_line(line_words, env)
      if last_exit != 0: break # short-circuit
    of TOr:
      last_exit = flush_line(line_words, env)
      if last_exit == 0: break # short-circuit
    else: discard

  env.last_exit

proc run_file*(path: string, args: seq[string] = @[]): int =
  if not fileExists(path):
    stderr.writeLine("sigma-sh: " & path & ": No such file"); return 127
  let src = readFile(path)
  run_script(src, parentDir(path), args)

when isMainModule:
  import std/parseopt
  var args = commandLineParams()
  if args.len == 0:
    stderr.writeLine("Usage: sigma-sh-script <script.sh> [args...]"); quit(1)
  quit(run_file(args[0], args[1..^1]))
