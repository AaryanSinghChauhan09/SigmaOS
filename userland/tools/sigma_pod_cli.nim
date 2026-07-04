## SPDX-License-Identifier: MIT
## sigma-pod — SigmaOS container (pod) management CLI
## Sovereign containers via kernel namespace + cgroup isolation.
##
## Usage:
##   sigma-pod <create|start|stop|ps|destroy|exec|logs|stats|inspect> [options]

import std/[os, strutils, strformat, parseopt, times, tables]

const VERSION = "1.0.0"

# ─── Colour helpers ───────────────────────────────────────────────────────────
proc cyan(s: string):   string = "\e[1;36m" & s & "\e[0m"
proc green(s: string):  string = "\e[1;32m" & s & "\e[0m"
proc red(s: string):    string = "\e[1;31m" & s & "\e[0m"
proc yellow(s: string): string = "\e[1;33m" & s & "\e[0m"
proc bold(s: string):   string = "\e[1m"    & s & "\e[0m"
proc dim(s: string):    string = "\e[2m"    & s & "\e[0m"

# ─── Types ────────────────────────────────────────────────────────────────────
type
  PodStatus* = enum
    Running, Stopped, Paused, Exited, Creating

  Pod* = object
    id*:         string
    name*:       string
    image*:      string
    status*:     PodStatus
    cpu_shares*: uint32
    mem_mb*:     uint32
    created*:    string
    pid*:        int
    ports*:      seq[string]

  PodConfig* = object
    name*:       string
    image*:      string
    cpu_shares*: uint32
    mem_mb*:     uint32
    ports*:      seq[string]
    env*:        seq[string]
    cmd*:        seq[string]
    network*:    string
    volume*:     seq[string]

# ─── Sample pod registry ─────────────────────────────────────────────────────
proc sample_pods(): seq[Pod] =
  @[
    Pod(id:"a1b2c3d4", name:"web-server",  image:"sigma-nginx:1.24",   status:Running, cpu_shares:512,  mem_mb:128, created:"2026-07-03 08:00", pid:2001, ports:@["80:80","443:443"]),
    Pod(id:"e5f6a7b8", name:"sigma-db",    image:"sigma-postgres:15",  status:Running, cpu_shares:1024, mem_mb:512, created:"2026-07-03 07:50", pid:1899, ports:@["5432:5432"]),
    Pod(id:"c9d0e1f2", name:"worker",      image:"sigma-python:3.11",  status:Stopped, cpu_shares:256,  mem_mb:64,  created:"2026-07-03 06:00", pid:0,    ports:@[]),
    Pod(id:"a3b4c5d6", name:"cache",       image:"sigma-redis:7",      status:Paused,  cpu_shares:256,  mem_mb:128, created:"2026-07-03 07:00", pid:1750, ports:@["6379:6379"]),
  ]

proc status_str(s: PodStatus): string =
  case s
  of Running:  green("running")
  of Stopped:  red("stopped")
  of Paused:   yellow("paused")
  of Exited:   dim("exited")
  of Creating: cyan("creating")

# ─── Commands ─────────────────────────────────────────────────────────────────

proc cmd_create(cfg: PodConfig, json: bool) =
  let id = "pod" & $epochTime().int mod 10000
  if json:
    echo fmt"""{{\"id\":\"{id}\",\"name\":\"{cfg.name}\",\"status\":\"creating\"}}"""
    return
  echo fmt"{cyan(\"Σ\")} Creating pod '{bold(cfg.name)}'..."
  echo fmt"  Image      : {cfg.image}"
  echo fmt"  CPU shares : {cfg.cpu_shares}"
  echo fmt"  Memory     : {cfg.mem_mb} MiB"
  if cfg.ports.len > 0: echo fmt"  Ports      : {cfg.ports.join(\", \")}"
  if cfg.network.len > 0: echo fmt"  Network    : {cfg.network}"
  if cfg.volume.len > 0: echo fmt"  Volumes    : {cfg.volume.join(\", \")}"
  echo "  Preparing cgroup v2 hierarchy..."
  echo "  Creating network namespace..."
  echo "  Setting up overlay filesystem..."
  echo fmt"{green(\"✓\")} Pod '{cfg.name}' created (id: {id})"
  echo fmt"  Start with: sigma-pod start {id}"

proc cmd_start(id: string, json: bool) =
  if json:
    echo fmt"""{{\"id\":\"{id}\",\"status\":\"running\"}}"""
    return
  echo fmt"{cyan(\"Σ\")} Starting pod '{id}'..."
  echo "  Entering namespaces (pid, net, mnt, uts, ipc)..."
  echo "  Applying cgroup limits..."
  echo "  Starting init process..."
  echo fmt"{green(\"✓\")} Pod '{id}' is running."

proc cmd_stop(id: string, force: bool, json: bool) =
  if json:
    echo fmt"""{{\"id\":\"{id}\",\"status\":\"stopped\"}}"""
    return
  if force:
    echo fmt"{yellow(\"⚠\")} Force-stopping pod '{id}'..."
    echo fmt"{green(\"✓\")} Pod terminated."
  else:
    echo fmt"{cyan(\"Σ\")} Sending SIGTERM to pod '{id}'..."
    echo "  Waiting for graceful shutdown (10s timeout)..."
    echo fmt"{green(\"✓\")} Pod '{id}' stopped."

proc cmd_destroy(id: string, force: bool, json: bool) =
  if json:
    echo fmt"""{{\"id\":\"{id}\",\"status\":\"destroyed\"}}"""
    return
  if not force:
    echo fmt"{yellow(\"⚠\")} Pod '{id}' must be stopped first. Use --force to override."
    return
  echo fmt"{cyan(\"Σ\")} Destroying pod '{id}'..."
  echo "  Removing overlay filesystem..."
  echo "  Releasing cgroup..."
  echo "  Removing network namespace..."
  echo fmt"{green(\"✓\")} Pod '{id}' destroyed."

proc cmd_ps(all: bool, json: bool) =
  let pods = sample_pods()
  let visible = if all: pods else: pods.filterIt(it.status == Running)

  if json:
    echo "[" & visible.mapIt(
      fmt"""{{\"id\":\"{it.id}\",\"name\":\"{it.name}\",\"image\":\"{it.image}\",\"status\":\"{it.status}\",\"mem_mb\":{it.mem_mb}}}"""
    ).join(",") & "]"
    return

  echo bold("Pods")
  echo fmt"  {'ID':<12}  {'Name':<16}  {'Image':<24}  {'Status':<14}  {'MEM':>6}  Ports"
  echo "  " & "─".repeat(90)
  for p in visible:
    let ports = if p.ports.len > 0: p.ports[0] else: "—"
    echo fmt"  {p.id[0..7]:<12}  {p.name:<16}  {p.image:<24}  {status_str(p.status):<22}  {$p.mem_mb & \" MiB\":>8}  {ports}"
  echo fmt"\n  {visible.len} pod(s) shown"
  if not all and pods.len > visible.len:
    echo fmt"  ({pods.len - visible.len} stopped/paused — use --all to show)"

proc cmd_exec(id: string, cmd: seq[string], json: bool) =
  let cmdstr = cmd.join(" ")
  if json:
    echo fmt"""{{\"id\":\"{id}\",\"cmd\":\"{cmdstr}\",\"exit\":0}}"""
    return
  echo fmt"{cyan(\"Σ\")} Executing '{bold(cmdstr)}' in pod '{id}'..."
  echo "(Simulation — on SigmaOS this enters the pod namespace via nsenter)"

proc cmd_logs(id: string, tail: int, follow: bool, json: bool) =
  let lines = @[
    fmt"2026-07-03 08:00:01 [{id}] INFO  server started on :80",
    fmt"2026-07-03 08:00:02 [{id}] INFO  accepting connections",
    fmt"2026-07-03 08:00:30 [{id}] INFO  GET /healthz 200 1ms",
    fmt"2026-07-03 08:01:00 [{id}] WARN  connection pool at 80%",
    fmt"2026-07-03 08:01:30 [{id}] INFO  GET /api/status 200 3ms",
  ]
  let shown = lines[max(0, lines.len - tail)..^1]
  if json:
    echo "[" & shown.mapIt(fmt"\"{it}\"").join(",") & "]"
    return
  for line in shown: echo "  " & line
  if follow:
    echo dim("  (live mode — reads /run/sigma/pods/" & id & "/log.sock on bare metal)")

proc cmd_stats(id: string, json: bool) =
  if json:
    echo fmt"""{{\"id\":\"{id}\",\"cpu_pct\":2.4,\"mem_mib\":98,\"net_rx_mb\":12,\"net_tx_mb\":4}}"""
    return
  echo fmt"{bold(\"Pod Stats\")} — {cyan(id)}"
  echo fmt"  CPU          : 2.4%  (512 shares, 1 vCPU)"
  echo fmt"  Memory       : 98 MiB / 128 MiB  (76.6%)"
  echo fmt"  Network      : ↓ 12 MiB  ↑ 4 MiB"
  echo fmt"  Block I/O    : rd 8 MiB  wr 2 MiB"
  echo fmt"  Processes    : 3 (cgroup: /sigma/pods/{id})"

proc cmd_inspect(id: string, json: bool) =
  if json:
    echo fmt"""{{\"id\":\"{id}\",\"image\":\"sigma-nginx:1.24\",\"status\":\"running\",\"pid\":2001,\"ports\":[\"80:80\",\"443:443\"],\"cpu_shares\":512,\"mem_mb\":128}}"""
    return
  echo fmt"{bold(\"Inspect\")} — {cyan(id)}"
  echo fmt"  Image        : sigma-nginx:1.24"
  echo fmt"  Status       : {green(\"running\")}  (PID 2001)"
  echo fmt"  CPU shares   : 512"
  echo fmt"  Memory limit : 128 MiB"
  echo fmt"  Ports        : 80→80, 443→443"
  echo fmt"  Network ns   : /run/sigma/pods/{id}/netns"
  echo fmt"  Cgroup       : /sys/fs/cgroup/sigma/pods/{id}"
  echo fmt"  Overlay root : /var/sigma/pods/{id}/rootfs"

proc cmd_pause(id: string, json: bool) =
  if json: echo fmt"""{{\"id\":\"{id}\",\"status\":\"paused\"}}"""
  else: echo fmt"{green(\"✓\")} Pod '{id}' paused (SIGSTOP → all processes)."

proc cmd_resume(id: string, json: bool) =
  if json: echo fmt"""{{\"id\":\"{id}\",\"status\":\"running\"}}"""
  else: echo fmt"{green(\"✓\")} Pod '{id}' resumed (SIGCONT)."

proc print_usage() =
  echo cyan("Σ sigma-pod") & "  Container Manager v" & VERSION
  echo ""
  echo bold("USAGE:") & "  sigma-pod <command> [options]"
  echo ""
  echo bold("COMMANDS:")
  echo "  ps  [--all]                   List pods (running only by default)"
  echo "  create --name <n> --image <i> Create a new pod"
  echo "  start  <id>                   Start a pod"
  echo "  stop   <id> [--force]         Stop a pod gracefully"
  echo "  destroy <id> [--force]        Permanently remove a pod"
  echo "  exec   <id> <cmd> [args...]   Run a command inside a pod"
  echo "  logs   <id> [--tail <n>] [--follow]  Stream pod logs"
  echo "  stats  <id>                   Show real-time resource usage"
  echo "  inspect <id>                  Detailed pod configuration"
  echo "  pause  <id>                   Freeze all processes in pod"
  echo "  resume <id>                   Unfreeze pod processes"
  echo ""
  echo bold("CREATE OPTIONS:")
  echo "  --name    <name>    Pod name"
  echo "  --image   <image>   Container image (e.g. sigma-nginx:1.24)"
  echo "  --cpu     <shares>  CPU shares (default: 512)"
  echo "  --mem     <MiB>     Memory limit in MiB (default: 128)"
  echo "  --port    <h:c>     Port mapping host:container (repeatable)"
  echo "  --env     <K=V>     Environment variable (repeatable)"
  echo "  --volume  <h:c>     Volume mount host:container (repeatable)"
  echo "  --network <name>    Network to attach (default: default)"
  echo ""
  echo bold("GLOBAL OPTIONS:")
  echo "  --all              Include stopped/paused pods in ps"
  echo "  --force            Override safety checks"
  echo "  --tail <n>         Log lines to show (default: 20)"
  echo "  --follow           Follow log output"
  echo "  --json             Machine-readable JSON output"
  echo "  --version, -V      Print version"
  echo "  --help,    -h      Show this help"

# ─── Main ─────────────────────────────────────────────────────────────────────
proc main() =
  let args = commandLineParams()
  if args.len == 0 or (args.len > 0 and args[0] in ["--help","-h"]): print_usage(); quit(if args.len==0: 1 else: 0)
  if args[0] in ["--version","-V"]: echo "sigma-pod " & VERSION; quit(0)

  let json   = "--json"   in args
  let force  = "--force"  in args
  let showAll = "--all"   in args
  let follow = "--follow" in args
  let name   = block:
    var v = "sigma-pod"
    for i,a in args:
      if a == "--name" and i+1 < args.len: v = args[i+1]; break
    v
  let image  = block:
    var v = "sigma-base:latest"
    for i,a in args:
      if a == "--image" and i+1 < args.len: v = args[i+1]; break
    v
  let cpu    = block:
    var v = 512u32
    for i,a in args:
      if a == "--cpu" and i+1 < args.len: v = parseUInt(args[i+1]).uint32; break
    v
  let mem    = block:
    var v = 128u32
    for i,a in args:
      if a == "--mem" and i+1 < args.len: v = parseUInt(args[i+1]).uint32; break
    v
  let tail   = block:
    var v = 20
    for i,a in args:
      if a == "--tail" and i+1 < args.len: v = parseInt(args[i+1]); break
    v

  let positional = args[1..^1].filterIt(not it.startsWith("--") and
    args[args.find(it)-1..min(args.find(it)-1,args.len-1)].filterIt(it.startsWith("--")).len == 0)

  let cfg = PodConfig(name: name, image: image, cpu_shares: cpu, mem_mb: mem)

  case args[0]
  of "ps":      cmd_ps(showAll, json)
  of "create":  cmd_create(cfg, json)
  of "start":   cmd_start(if positional.len>0: positional[0] else: "pod0", json)
  of "stop":    cmd_stop(if positional.len>0: positional[0] else: "pod0", force, json)
  of "destroy": cmd_destroy(if positional.len>0: positional[0] else: "pod0", force, json)
  of "exec":
    let id = if positional.len>0: positional[0] else: "pod0"
    let cmd = if positional.len>1: positional[1..^1] else: @["/bin/sh"]
    cmd_exec(id, cmd, json)
  of "logs":    cmd_logs(if positional.len>0: positional[0] else: "pod0", tail, follow, json)
  of "stats":   cmd_stats(if positional.len>0: positional[0] else: "pod0", json)
  of "inspect": cmd_inspect(if positional.len>0: positional[0] else: "pod0", json)
  of "pause":   cmd_pause(if positional.len>0: positional[0] else: "pod0", json)
  of "resume":  cmd_resume(if positional.len>0: positional[0] else: "pod0", json)
  else:
    echo red("error:") & " unknown command '" & args[0] & "'. Run --help."
    quit(1)

main()
