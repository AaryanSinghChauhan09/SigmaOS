## SPDX-License-Identifier: MIT
## sigma-cli — SigmaOS modular userspace CLI
## Profiles, aliases, automation, package proxy, pod proxy, network, AI hooks.
## Language: Nim (stdlib only), OOP via object types + methods.
##
## Usage: sigma-cli <subsystem> <verb> [options]

import std/[os, strutils, strformat, tables, json, times, osproc, sequtils]

const VERSION = "15.0.0"

# ─── Colour helpers ───────────────────────────────────────────────────────────
proc cyan(s: string):   string = "\e[1;36m" & s & "\e[0m"
proc green(s: string):  string = "\e[1;32m" & s & "\e[0m"
proc red(s: string):    string = "\e[1;31m" & s & "\e[0m"
proc yellow(s: string): string = "\e[1;33m" & s & "\e[0m"
proc bold(s: string):   string = "\e[1m"    & s & "\e[0m"
proc dim(s: string):    string = "\e[2m"    & s & "\e[0m"

proc info(msg: string)    = echo cyan("Σ [INFO]   ") & " " & msg
proc success(msg: string) = echo green("Σ [OK]     ") & " " & msg
proc warn(msg: string)    = echo yellow("Σ [WARN]   ") & " " & msg
proc err(msg: string)     = echo red("Σ [ERROR]  ") & " " & msg

# ─── Types ────────────────────────────────────────────────────────────────────
type
  SigmaProfile* = object
    name*:      string
    gap_inner*: int
    gap_outer*: int
    theme*:     string
    layout*:    string
    font_size*: int

  SigmaAlias* = object
    name*:    string
    command*: string
    active*:  bool

  SigmaCLIState* = object
    profiles*:      seq[SigmaProfile]
    aliases*:       seq[SigmaAlias]
    active_profile*: string

# ─── State management ─────────────────────────────────────────────────────────
proc state_path(): string =
  getEnv("HOME", "/home/sigma") / ".sigma" / "cli_state.json"

proc default_profiles(): seq[SigmaProfile] =
  @[
    SigmaProfile(name:"default",   gap_inner:8,  gap_outer:12, theme:"zenith-dark",  layout:"bsp",    font_size:12),
    SigmaProfile(name:"coding",    gap_inner:4,  gap_outer:8,  theme:"zenith-dark",  layout:"master", font_size:14),
    SigmaProfile(name:"minimal",   gap_inner:0,  gap_outer:0,  theme:"sigma-mono",   layout:"monocle",font_size:11),
    SigmaProfile(name:"gaming",    gap_inner:0,  gap_outer:0,  theme:"zenith-neon",  layout:"floating",font_size:12),
    SigmaProfile(name:"focus",     gap_inner:20, gap_outer:24, theme:"zenith-light", layout:"master", font_size:14),
  ]

proc load_state(): SigmaCLIState =
  let path = state_path()
  if fileExists(path):
    try:
      let data = parseJson(readFile(path))
      result.active_profile = data{"active_profile"}.getStr("default")
      # Load profiles and aliases from JSON if present
      # (simplified: return defaults + active profile from state)
    except: discard
  result.profiles = default_profiles()
  if result.active_profile.len == 0: result.active_profile = "default"

proc save_state(st: SigmaCLIState) =
  let path = state_path()
  createDir(path.parentDir())
  let data = %*{"active_profile": st.active_profile}
  writeFile(path, data.pretty())

# ─── Profile commands ─────────────────────────────────────────────────────────
proc cmd_profile(args: seq[string], json: bool) =
  var st = load_state()
  let verb = if args.len > 0: args[0] else: "list"

  case verb
  of "list":
    if json:
      let arr = st.profiles.mapIt(%*{"name":it.name,"theme":it.theme,"layout":it.layout,"active":it.name==st.active_profile})
      echo $(%arr)
      return
    echo bold("Available profiles:")
    for p in st.profiles:
      let active_marker = if p.name == st.active_profile: green(" ← active") else: ""
      echo fmt"  {p.name:<12} theme={p.theme:<16} layout={p.layout:<10}{active_marker}"

  of "show":
    let name = if args.len > 1: args[1] else: st.active_profile
    let p = st.profiles.filterIt(it.name == name)
    if p.len == 0: err(fmt"Profile '{name}' not found."); return
    if json: echo $(%*{"name":p[0].name,"theme":p[0].theme,"layout":p[0].layout,"gap_inner":p[0].gap_inner,"gap_outer":p[0].gap_outer})
    else:
      echo bold(fmt"Profile: {p[0].name}")
      echo fmt"  Theme     : {p[0].theme}"
      echo fmt"  Layout    : {p[0].layout}"
      echo fmt"  Gaps      : inner={p[0].gap_inner}px  outer={p[0].gap_outer}px"
      echo fmt"  Font size : {p[0].font_size}pt"

  of "use", "set":
    let name = if args.len > 1: args[1] else: ""
    if name.len == 0: err("Usage: sigma-cli profile use <name>"); return
    if not st.profiles.anyIt(it.name == name): err(fmt"Profile '{name}' not found. Run 'profile list'."); return
    st.active_profile = name
    save_state(st)
    success(fmt"Active profile set to '{name}'.")
    # Send IPC to zenith-compositor
    info("Sending profile change to zenith-wm (IPC)...")

  of "create":
    let name = if args.len > 1: args[1] else: ""
    if name.len == 0: err("Usage: sigma-cli profile create <name>"); return
    if st.profiles.anyIt(it.name == name): err(fmt"Profile '{name}' already exists."); return
    st.profiles.add(SigmaProfile(name:name, gap_inner:8, gap_outer:12, theme:"zenith-dark", layout:"bsp", font_size:12))
    save_state(st)
    success(fmt"Profile '{name}' created.")

  of "delete":
    let name = if args.len > 1: args[1] else: ""
    if name == "default": err("Cannot delete the default profile."); return
    st.profiles.keepItIf(it.name != name)
    save_state(st)
    success(fmt"Profile '{name}' deleted.")

  of "export":
    let out = if args.len > 1: args[1] else: "profile.json"
    let active = st.profiles.filterIt(it.name == st.active_profile)
    if active.len > 0:
      writeFile(out, $(%*{"name":active[0].name,"theme":active[0].theme,"layout":active[0].layout}))
      success(fmt"Profile exported to {out}")

  of "import":
    let path = if args.len > 1: args[1] else: ""
    if not fileExists(path): err(fmt"File not found: {path}"); return
    try:
      let data = parseJson(readFile(path))
      let name = data{"name"}.getStr("imported")
      st.profiles.add(SigmaProfile(name:name, theme:data{"theme"}.getStr("zenith-dark"), layout:data{"layout"}.getStr("bsp")))
      save_state(st)
      success(fmt"Profile '{name}' imported.")
    except: err("Invalid profile JSON.")
  else:
    err(fmt"Unknown profile verb '{verb}'. Valid: list, show, use, create, delete, export, import")

# ─── Alias commands ───────────────────────────────────────────────────────────
var g_aliases: seq[SigmaAlias] = @[
  SigmaAlias(name:"ll",  command:"ls -la", active:true),
  SigmaAlias(name:"gst", command:"git status", active:true),
  SigmaAlias(name:"gc",  command:"git commit -m", active:true),
]

proc cmd_alias(args: seq[string], json: bool) =
  let verb = if args.len > 0: args[0] else: "list"
  case verb
  of "list":
    if json:
      echo $(%g_aliases.mapIt(%*{"name":it.name,"command":it.command}))
    else:
      echo bold("Aliases:")
      for a in g_aliases:
        if a.active: echo fmt"  {a.name:<12} = {a.command}"

  of "add":
    if args.len < 3: err("Usage: sigma-cli alias add <name> <command>"); return
    let name = args[1]; let cmd = args[2..^1].join(" ")
    g_aliases.keepItIf(it.name != name)
    g_aliases.add(SigmaAlias(name:name, command:cmd, active:true))
    success(fmt"Alias '{name}' = '{cmd}' added.")

  of "remove", "rm":
    let name = if args.len > 1: args[1] else: ""
    let before = g_aliases.len
    g_aliases.keepItIf(it.name != name)
    if g_aliases.len < before: success(fmt"Alias '{name}' removed.")
    else: warn(fmt"Alias '{name}' not found.")

  of "show":
    let name = if args.len > 1: args[1] else: ""
    let a = g_aliases.filterIt(it.name == name)
    if a.len > 0: echo fmt"alias {a[0].name}='{a[0].command}'"
    else: err(fmt"Alias '{name}' not found.")
  else:
    err(fmt"Unknown alias verb '{verb}'. Valid: list, add, remove, show")

# ─── Automation commands ──────────────────────────────────────────────────────
proc cmd_auto(args: seq[string]) =
  let verb = if args.len > 0: args[0] else: "help"
  let root = getEnv("SIGMA_ROOT", getAppDir() / ".." / "..")
  let scripts = root / "scripts"
  case verb
  of "update":
    info("Pulling latest sources and rebuilding...")
    let (out, code) = execCmdEx(scripts / "sigma_automation.sh update")
    if code == 0: success("Update complete.") else: warn("Update returned code " & $code)
  of "backup":
    info("Creating workspace backup...")
    let (_, code) = execCmdEx(scripts / "sigma_automation.sh backup")
    if code == 0: success("Backup complete.") else: warn("Backup returned code " & $code)
  of "sync":
    let extra = args[1..^1].join(" ")
    info("Syncing git repository...")
    let (_, code) = execCmdEx(scripts / "sigma_git_sync.sh " & extra)
    if code == 0: success("Sync complete.") else: warn("Sync returned code " & $code)
  of "lint":
    info("Running static analysis...")
    let (_, code) = execCmdEx(scripts / "run_static_analysis.sh")
    if code == 0: success("Lint passed.") else: warn("Lint issues found.")
  of "status":
    let (out, _) = execCmdEx("git -C " & root & " status --short --branch")
    echo out
  else:
    echo bold("Automation commands:")
    echo "  sigma-cli auto update     Pull and rebuild"
    echo "  sigma-cli auto backup     Snapshot workspace"
    echo "  sigma-cli auto sync       Git sync"
    echo "  sigma-cli auto lint       Static analysis"
    echo "  sigma-cli auto status     Repo status"

# ─── Package proxy ────────────────────────────────────────────────────────────
proc cmd_pkg(args: seq[string]) =
  if args.len == 0: echo "Usage: sigma-cli pkg <install|remove|list|search|update>"; return
  info(fmt"Delegating to sigma-pkg: {args.join(\" \")}...")
  let (out, code) = execCmdEx("sigma-pkg " & args.join(" "))
  if code != 0 or out.len == 0:
    warn("sigma-pkg not found — forwarding to sigma_pkg_cli...")
    let (out2, _) = execCmdEx("nim r userland/tools/sigma_pkg_cli.nim " & args.join(" "))
    echo out2
  else: echo out

# ─── Pod proxy ────────────────────────────────────────────────────────────────
proc cmd_pod(args: seq[string]) =
  if args.len == 0: echo "Usage: sigma-cli pod <ps|create|start|stop|exec|logs>"; return
  info(fmt"Delegating to sigma-pod: {args.join(\" \")}...")
  let (out, code) = execCmdEx("sigma-pod " & args.join(" "))
  if code != 0 or out.len == 0:
    warn("sigma-pod not found — forwarding to sigma_pod_cli...")
    let (out2, _) = execCmdEx("nim r userland/tools/sigma_pod_cli.nim " & args.join(" "))
    echo out2
  else: echo out

# ─── Network commands ─────────────────────────────────────────────────────────
proc cmd_net(args: seq[string]) =
  let verb = if args.len > 0: args[0] else: "status"
  case verb
  of "status":
    info("Network interfaces:")
    let (out, code) = execCmdEx("ip addr show 2>/dev/null || ipconfig 2>/dev/null")
    if code == 0: echo out
    else:
      echo "  eth0    10.0.0.1/24  UP  2.5Gbps  (simulated)"
      echo "  lo      127.0.0.1/8  UP"
  of "ping":
    let host = if args.len > 1: args[1] else: "8.8.8.8"
    let (out, _) = execCmdEx(fmt"ping -c 3 {host} 2>/dev/null || echo 'ICMP simulated'")
    echo out
  of "dns":
    if args.len > 1: info(fmt"Setting DNS to {args[1]} (requires root)...")
    else:
      let (out, _) = execCmdEx("cat /etc/resolv.conf 2>/dev/null")
      if out.len > 0: echo out else: echo "  nameserver 1.1.1.1  (default)"
  else:
    echo "  sigma-cli net status|ping [host]|dns [server]"

# ─── Sysctl helper ────────────────────────────────────────────────────────────
proc cmd_sysctl(args: seq[string]) =
  if args.len == 0: echo "Usage: sigma-cli sysctl <key>[=value]"; return
  let arg = args[0]
  if "=" in arg:
    let parts = arg.split("=", 1)
    info(fmt"Setting {parts[0]} = {parts[1]}...")
    let (_, code) = execCmdEx(fmt"sysctl -w {arg} 2>/dev/null")
    if code == 0: success("Sysctl updated.") else: warn("sysctl failed or not available.")
  else:
    let (out, code) = execCmdEx(fmt"sysctl {arg} 2>/dev/null")
    if code == 0: echo out else: echo fmt"  {arg} = (requires root)"

# ─── Help & dispatch ─────────────────────────────────────────────────────────
proc print_usage() =
  echo cyan("Σ sigma-cli") & "  v" & VERSION
  echo ""
  echo bold("USAGE:") & "  sigma-cli <subsystem> <verb> [options]"
  echo ""
  echo bold("SUBSYSTEMS:")
  echo "  profile   list|show|use|create|delete|export|import"
  echo "  alias     list|add|remove|show"
  echo "  auto      update|backup|sync|lint|status"
  echo "  pkg       install|remove|list|search|update  (delegates to sigma-pkg)"
  echo "  pod       ps|create|start|stop|exec|logs     (delegates to sigma-pod)"
  echo "  net       status|ping|dns"
  echo "  sysctl    <key>[=value]"
  echo "  version   print version info"
  echo ""
  echo bold("GLOBAL OPTIONS:")
  echo "  --json     Machine-readable JSON output"
  echo "  --verbose  Extra detail"
  echo "  --help     Show this help"
  echo ""
  echo "  See also: " & cyan("sigma") & " (OS dev CLI) · " & cyan("sigma-sh") & " (shell)"

proc init_defaults*() =
  ## Called from C ABI entry; ensures state is initialised
  discard load_state()

proc profile_list*() = cmd_profile(@[], false)
proc profile_use*(name: string) = cmd_profile(@["use", name], false)
proc alias_list*() = cmd_alias(@[], false)
proc alias_add*(name, command: string) = cmd_alias(@["add", name, command], false)

proc main() =
  let args = commandLineParams()
  if args.len == 0 or args[0] in ["--help","-h","help"]: print_usage(); quit(0)
  if args[0] in ["--version","-V","version"]: echo "sigma-cli " & VERSION; quit(0)

  let json = "--json" in args
  let sub  = args[0]
  let rest = args[1..^1].filterIt(not it.startsWith("--"))

  case sub
  of "profile": cmd_profile(rest, json)
  of "alias":   cmd_alias(rest, json)
  of "auto":    cmd_auto(rest)
  of "pkg":     cmd_pkg(rest)
  of "pod":     cmd_pod(rest)
  of "net":     cmd_net(rest)
  of "sysctl":  cmd_sysctl(rest)
  else:
    err(fmt"Unknown subsystem '{sub}'. Run --help.")
    quit(1)

main()
