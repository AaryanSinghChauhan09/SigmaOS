## SPDX-License-Identifier: MIT
## sigma_getopt.nim — reusable argument parser for SigmaOS CLI tools
## Provides POSIX-style getopt + GNU long options with no external dependencies.
##
## Features:
##   - Short flags: -v -j -n5 -n 5
##   - Long flags:  --verbose --json --count 5 --count=5
##   - Positional arguments
##   - Automatic --help flag detection
##   - Required / optional argument distinction
##   - Type-safe value conversion helpers

import std/[strutils, sequtils, tables, strformat]

# ─── Option definition ────────────────────────────────────────────────────────
type
  OptKind* = enum
    okFlag,       ## --verbose / -v  (no value)
    okOptional,   ## --output[=file] (value optional)
    okRequired    ## --output file   (value required)

  OptDef* = object
    long*:    string       ## long name without --
    short*:   char         ## single char, '\0' = none
    kind*:    OptKind
    help*:    string
    default_val*: string   ## default value for okOptional/okRequired

  ParsedArgs* = object
    flags*:       Table[string, bool]    ## flag_name → true if present
    values*:      Table[string, string]  ## opt_name  → value
    positional*:  seq[string]
    remainder*:   seq[string]            ## args after --
    errors*:      seq[string]

# ─── Builder ──────────────────────────────────────────────────────────────────
proc flag*(long: string, short: char = '\0', help = ""): OptDef =
  OptDef(long: long, short: short, kind: okFlag, help: help)

proc option*(long: string, short: char = '\0', help = "", default_val = ""): OptDef =
  OptDef(long: long, short: short, kind: okRequired, help: help, default_val: default_val)

proc optionOpt*(long: string, short: char = '\0', help = "", default_val = ""): OptDef =
  OptDef(long: long, short: short, kind: okOptional, help: help, default_val: default_val)

# ─── Parser ───────────────────────────────────────────────────────────────────
proc parseArgs*(args: seq[string], defs: seq[OptDef]): ParsedArgs =
  ## Parse args according to defs. Returns ParsedArgs.
  result = ParsedArgs(
    flags: initTable[string, bool](),
    values: initTable[string, string]()
  )

  # Apply defaults
  for d in defs:
    if d.kind != okFlag and d.default_val.len > 0:
      result.values[d.long] = d.default_val

  # Build lookup tables
  var longMap:  Table[string, OptDef]
  var shortMap: Table[char, OptDef]
  for d in defs:
    longMap[d.long] = d
    if d.short != '\0': shortMap[d.short] = d

  var i = 0
  var past_dashdash = false

  while i < args.len:
    let arg = args[i]

    if past_dashdash:
      result.remainder.add(arg); i += 1; continue

    if arg == "--":
      past_dashdash = true; i += 1; continue

    if arg.startsWith("--"):
      # Long option
      let body = arg[2..^1]
      if body.contains('='):
        let eq = body.find('=')
        let name = body[0..<eq]
        let val  = body[eq+1..^1]
        if name in longMap:
          let d = longMap[name]
          if d.kind == okFlag:
            result.flags[name] = true
          else:
            result.values[name] = val
        else:
          result.errors.add(fmt"unknown option: --{name}")
      else:
        if body in longMap:
          let d = longMap[body]
          if d.kind == okFlag:
            result.flags[body] = true
          elif d.kind == okRequired:
            i += 1
            if i < args.len and not args[i].startsWith("-"):
              result.values[body] = args[i]
            else:
              if d.default_val.len > 0: result.values[body] = d.default_val
              else: result.errors.add(fmt"--{body} requires a value")
              i -= 1
          else: # okOptional
            result.flags[body] = true
            if i+1 < args.len and not args[i+1].startsWith("-"):
              i += 1
              result.values[body] = args[i]
            elif d.default_val.len > 0:
              result.values[body] = d.default_val
        else:
          result.errors.add(fmt"unknown option: --{body}")

    elif arg.startsWith("-") and arg.len > 1 and arg[1] != '-':
      # Short option(s)
      var j = 1
      while j < arg.len:
        let c = arg[j]
        if c in shortMap:
          let d = shortMap[c]
          if d.kind == okFlag:
            result.flags[d.long] = true
          else:
            # Value: rest of arg or next arg
            let rest = arg[j+1..^1]
            if rest.len > 0:
              result.values[d.long] = rest
              j = arg.len  # consumed rest
            elif i+1 < args.len and not args[i+1].startsWith("-"):
              i += 1
              result.values[d.long] = args[i]
            else:
              if d.default_val.len > 0: result.values[d.long] = d.default_val
              else: result.errors.add(fmt"-{c} requires a value")
        else:
          result.errors.add(fmt"unknown flag: -{c}")
        j += 1

    else:
      result.positional.add(arg)

    i += 1

proc has*(pa: ParsedArgs, name: string): bool =
  ## True if the flag was present OR the option was set.
  pa.flags.getOrDefault(name, false) or pa.values.hasKey(name)

proc getStr*(pa: ParsedArgs, name: string, default_val = ""): string =
  pa.values.getOrDefault(name, default_val)

proc getInt*(pa: ParsedArgs, name: string, default_val = 0): int =
  try: parseInt(pa.values.getOrDefault(name, $default_val))
  except: default_val

proc getBool*(pa: ParsedArgs, name: string): bool =
  pa.flags.getOrDefault(name, false)

proc isJson*(pa: ParsedArgs): bool = pa.getBool("json")
proc isDryRun*(pa: ParsedArgs): bool = pa.getBool("dry-run")
proc isVerbose*(pa: ParsedArgs): bool = pa.getBool("verbose")

# ─── Help formatter ───────────────────────────────────────────────────────────
proc formatHelp*(program: string, description: string, defs: seq[OptDef], usage = ""): string =
  var lines: seq[string]
  lines.add(fmt"  {program} — {description}")
  lines.add("")
  lines.add(fmt"  Usage: {if usage.len>0: usage else: program & \" [options] [args...]\"}")
  lines.add("")
  lines.add("  Options:")
  for d in defs:
    let short_part = if d.short != '\0': fmt"-{d.short}, " else: "    "
    let val_part = case d.kind
      of okFlag:     ""
      of okRequired: " <value>"
      of okOptional: " [value]"
    let default_part = if d.default_val.len > 0: fmt"  (default: {d.default_val})" else: ""
    lines.add(fmt"    {short_part}--{d.long:<20}{val_part:<10}  {d.help}{default_part}")
  lines.add("")
  lines.join("\n")

# ─── Standard CLI option set ──────────────────────────────────────────────────
proc stdOpts*(): seq[OptDef] =
  ## Common options included in all sigma CLI tools
  @[
    flag("help",    'h', "Show this help"),
    flag("version", 'V', "Print version"),
    flag("json",    'j', "Machine-readable JSON output"),
    flag("verbose", 'v', "Extra diagnostic output"),
    flag("dry-run",     help="Show what would happen without executing"),
    option("output", 'o', "Write output to file"),
  ]

when isMainModule:
  # Self-test
  let defs = stdOpts() & @[
    option("count", 'n', "Number of iterations", "10"),
    option("target",    help="Target arch", default_val="x86_64"),
    flag("release",     help="Release build"),
  ]
  let pa = parseArgs(commandLineParams(), defs)
  if pa.has("help"):
    echo formatHelp("sigma_getopt", "Argument parser library", defs)
    quit(0)
  echo "json=",    pa.isJson()
  echo "verbose=", pa.isVerbose()
  echo "count=",   pa.getInt("count")
  echo "target=",  pa.getStr("target")
  echo "positional=", pa.positional
  echo "errors=",  pa.errors
