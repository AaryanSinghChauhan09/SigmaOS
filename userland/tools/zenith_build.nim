## SPDX-License-Identifier: MIT
## zenith-build — SigmaOS Zenith Desktop build tool
## Compiles, bundles, and hot-reloads desktop components.
##
## Usage: zenith-build <command> [options]

import std/[os, osproc, strutils, strformat, times, json, sequtils]

const VERSION = "1.0.0"

proc cyan(s: string):   string = "\e[1;36m" & s & "\e[0m"
proc green(s: string):  string = "\e[1;32m" & s & "\e[0m"
proc red(s: string):    string = "\e[1;31m" & s & "\e[0m"
proc yellow(s: string): string = "\e[1;33m" & s & "\e[0m"
proc bold(s: string):   string = "\e[1m"    & s & "\e[0m"
proc dim(s: string):    string = "\e[2m"    & s & "\e[0m"

# ─── Component catalogue ──────────────────────────────────────────────────────
type
  Component = object
    name*:   string
    lang*:   string
    source*: string
    output*: string
    deps*:   seq[string]

proc catalogue(root: string): seq[Component] =
  @[
    Component(name:"zenith-wm",         lang:"rust", source:"zenith_desktop/wm",            output:"build/zenith-wm",            deps:@["sigma-core"]),
    Component(name:"zenith-compositor",  lang:"rust", source:"zenith_desktop/compositor",    output:"build/zenith-compositor",    deps:@["zenith-wm","sigma-gpu-hal"]),
    Component(name:"zenith-bar",         lang:"nim",  source:"zenith_desktop/modules/bar",   output:"build/zenith-bar",           deps:@["zenith-wm"]),
    Component(name:"zenith-notifications",lang:"nim", source:"zenith_desktop/notifications", output:"build/zenith-notifications", deps:@["zenith-wm"]),
    Component(name:"zenith-settings",    lang:"nim",  source:"zenith_desktop/settings",      output:"build/zenith-settings",      deps:@["zenith-wm"]),
    Component(name:"zenith-onboarding",  lang:"nim",  source:"zenith_desktop/onboarding",    output:"build/zenith-onboarding",    deps:@["zenith-wm"]),
    Component(name:"zenith-appstore",    lang:"js",   source:"zenith_desktop/appstore",      output:"build/zenith-appstore.js",   deps:@[]),
    Component(name:"zenith-neural",      lang:"rust", source:"zenith_desktop/neural",        output:"build/zenith-neural",        deps:@["sigma-ml"]),
  ]

# ─── Builder ──────────────────────────────────────────────────────────────────
proc build_component(c: Component, root: string, release: bool, verbose: bool): bool =
  let src = root / c.source
  if not dirExists(src) and not fileExists(src):
    echo yellow(fmt"  ⚠  {c.name:<28}") & dim(fmt" source not found: {c.source}")
    return true # not a fatal error, component may not exist yet

  let opts = if release: "--release" else: ""
  case c.lang
  of "rust":
    if verbose: echo dim(fmt"    cargo build {opts} --manifest-path {src}/Cargo.toml")
    let (out, code) = execCmdEx(fmt"cargo build {opts} --manifest-path {src}/Cargo.toml 2>&1")
    if code == 0:
      echo green(fmt"  ✓  {c.name:<28}") & dim(" (rust)")
      return true
    else:
      echo red(fmt"  ✗  {c.name:<28}") & dim(" cargo build failed")
      if verbose: echo out
      return false
  of "nim":
    let main = src / "main.nim"
    if not fileExists(main):
      echo yellow(fmt"  ─  {c.name:<28}") & dim(" (no main.nim, skipped)")
      return true
    let nim_opts = if release: "-d:release" else: ""
    let (out, code) = execCmdEx(fmt"nim compile {nim_opts} --out:{root / c.output} {main} 2>&1")
    if code == 0:
      echo green(fmt"  ✓  {c.name:<28}") & dim(" (nim)")
      return true
    else:
      echo red(fmt"  ✗  {c.name:<28}") & dim(" nim compile failed")
      if verbose: echo out
      return false
  of "js":
    let index = src / "index.js"
    if not fileExists(index):
      echo yellow(fmt"  ─  {c.name:<28}") & dim(" (no index.js, skipped)")
      return true
    # Use esbuild if available, fallback to cat
    let (_, code) = execCmdEx(fmt"esbuild {index} --bundle --outfile={root / c.output} 2>/dev/null")
    if code == 0:
      echo green(fmt"  ✓  {c.name:<28}") & dim(" (js/esbuild)")
    else:
      let _ = execCmdEx(fmt"cat {index} > {root / c.output}")
      echo yellow(fmt"  ─  {c.name:<28}") & dim(" (js/cat, no esbuild)")
    return true
  else:
    echo dim(fmt"  ─  {c.name:<28} unknown lang: {c.lang}")
    return true

# ─── Commands ─────────────────────────────────────────────────────────────────
proc cmd_build(args: seq[string], json: bool) =
  let root    = getEnv("SIGMA_ROOT", getCurrentDir() / ".." / "..")
  let release = "--release" in args
  let verbose = "--verbose" in args
  let target  = args.filterIt(not it.startsWith("--")).filterIt(it.len > 0)
  let comps   = catalogue(root)
  let to_build = if target.len > 0:
    comps.filterIt(it.name in target)
  else: comps

  if json:
    echo fmt"""{{\"action\":\"build\",\"components\":{to_build.len},\"release\":{release}}}"""
    return

  echo fmt"{cyan(\"Σ\")} Building Zenith Desktop ({to_build.len} components){if release: \" [release]\" else: \"\"}..."
  echo "─".repeat(60)
  let start_ms = now().toTime.toUnixFloat * 1000
  var ok_count = 0; var fail_count = 0
  for c in to_build:
    if build_component(c, root, release, verbose): ok_count += 1
    else: fail_count += 1
  let elapsed = (now().toTime.toUnixFloat * 1000 - start_ms) / 1000.0
  echo "─".repeat(60)
  if fail_count == 0:
    echo green(fmt"✓ {ok_count} components built in {elapsed:.1f}s")
  else:
    echo red(fmt"✗ {fail_count} failed, {ok_count} succeeded in {elapsed:.1f}s")

proc cmd_clean(root: string) =
  echo cyan("Σ") & " Cleaning build artefacts..."
  let build_dir = root / "build"
  if dirExists(build_dir):
    for _, f in walkDir(build_dir):
      if f.endsWith("-wm") or f.endsWith("-compositor") or f.endsWith(".js"):
        removeFile(f)
        echo dim(fmt"  removed: {f}")
  echo green("✓") & " Clean complete."

proc cmd_watch(root: string) =
  echo cyan("Σ") & " Watching for changes (Ctrl+C to stop)..."
  var mtimes: Table[string, int64]
  while true:
    for c in catalogue(root):
      let src = root / c.source
      if dirExists(src):
        for _, f in walkDirRec(src):
          let mtime = getFileInfo(f).lastWriteTime.toUnix
          if f notin mtimes or mtimes[f] != mtime:
            mtimes[f] = mtime
            echo cyan(fmt"Σ change: {f}") & "  rebuilding " & bold(c.name) & "..."
            discard build_component(c, root, false, false)
    sleep(500)

proc cmd_list(root: string, json: bool) =
  let comps = catalogue(root)
  if json:
    echo $(%comps.mapIt(%*{"name":it.name,"lang":it.lang,"source":it.source}))
    return
  echo bold("Zenith Desktop Components:")
  echo fmt"  {'Name':<28}  {'Lang':<6}  Source"
  echo "  " & "─".repeat(64)
  for c in comps:
    let built = fileExists(root / c.output) or dirExists(root / c.output)
    let status = if built: green("built") else: dim("not built")
    echo fmt"  {c.name:<28}  {c.lang:<6}  {dim(c.source)}  {status}"

proc cmd_hot_reload(component: string, root: string) =
  echo cyan("Σ") & fmt" Hot-reloading '{bold(component)}'..."
  let comps = catalogue(root).filterIt(it.name == component)
  if comps.len == 0: echo red("✗") & fmt" Component '{component}' not found."; return
  discard build_component(comps[0], root, false, false)
  # Send IPC to compositor to reload
  let (_, code) = execCmdEx(fmt"sigma-ipc zenith-compositor shard-reload {component} 2>/dev/null")
  if code == 0: echo green("✓") & fmt" '{component}' hot-reloaded in compositor."
  else: echo yellow("⚠") & " IPC not available — restart compositor to apply."

proc print_usage() =
  echo cyan("Σ zenith-build") & "  v" & VERSION
  echo ""
  echo bold("USAGE:") & "  zenith-build <command> [options]"
  echo ""
  echo bold("COMMANDS:")
  echo "  build [components...] [--release] [--verbose]"
  echo "  clean                  Remove build artefacts"
  echo "  watch                  Continuous rebuild on file change"
  echo "  list                   Show all components and build status"
  echo "  hot-reload <name>      Hot-swap a component in the running compositor"
  echo ""
  echo bold("OPTIONS:")
  echo "  --release   Optimised (release) build"
  echo "  --verbose   Show full build output"
  echo "  --json      Machine-readable JSON output"
  echo "  --version   Print version"
  echo "  --help      Show this help"

proc main() =
  let args = commandLineParams()
  if args.len == 0 or args[0] in ["--help","-h"]: print_usage(); quit(0)
  if args[0] in ["--version","-V"]: echo "zenith-build " & VERSION; quit(0)

  let root = getEnv("SIGMA_ROOT", getCurrentDir() / ".." / "..")
  let json = "--json" in args

  case args[0]
  of "build":     cmd_build(args[1..^1], json)
  of "clean":     cmd_clean(root)
  of "watch":     cmd_watch(root)
  of "list":      cmd_list(root, json)
  of "hot-reload":
    let name = if args.len > 1: args[1] else: ""
    if name.len == 0: echo "Usage: zenith-build hot-reload <component-name>"; quit(1)
    cmd_hot_reload(name, root)
  else:
    echo red("error:") & fmt" unknown command '{args[0]}'. Run --help."
    quit(1)

main()
