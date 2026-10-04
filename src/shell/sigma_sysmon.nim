import os, strutils, json, net, asyncdispatch, posix, times

type
  SysMetrics* = object
    cpu_usage*: float
    mem_total*: uint64
    mem_free*: uint64
    net_rx_bytes*: uint64
    net_tx_bytes*: uint64
    battery_percent*: int
    disk_read_bytes*: uint64
    disk_write_bytes*: uint64

proc readCpuUsage(): float =
  try:
    let lines = readFile("/proc/stat").splitLines()
    if lines.len > 0:
      let parts = lines[0].splitWhitespace()
      if parts.len >= 5 and parts[0] == "cpu":
        let user = parseFloat(parts[1])
        let nice = parseFloat(parts[2])
        let system = parseFloat(parts[3])
        let idle = parseFloat(parts[4])
        let total = user + nice + system + idle
        if total > 0.0:
          return ((total - idle) / total) * 100.0
  except:
    discard
  return 0.0

proc readMemInfo(): tuple[total: uint64, free: uint64] =
  var total, free: uint64 = 0
  try:
    for line in lines("/proc/meminfo"):
      if line.startsWith("MemTotal:"):
        let parts = line.splitWhitespace()
        total = parseUInt(parts[1]) * 1024
      elif line.startsWith("MemAvailable:"):
        let parts = line.splitWhitespace()
        free = parseUInt(parts[1]) * 1024
  except:
    discard
  return (total, free)

proc readNetDev(): tuple[rx: uint64, tx: uint64] =
  var rx, tx: uint64 = 0
  try:
    for line in lines("/proc/net/dev"):
      if ":" in line:
        let parts = line.replace(":", " ").splitWhitespace()
        if parts.len >= 10:
          rx += parseUInt(parts[1])
          tx += parseUInt(parts[9])
  except:
    discard
  return (rx, tx)

proc readBattery(): int =
  var bat = -1
  try:
    if fileExists("/sys/class/power_supply/BAT0/capacity"):
      bat = parseInt(readFile("/sys/class/power_supply/BAT0/capacity").strip())
    elif fileExists("/sys/class/power_supply/BAT1/capacity"):
      bat = parseInt(readFile("/sys/class/power_supply/BAT1/capacity").strip())
  except:
    discard
  return bat

proc readDiskStats(): tuple[read: uint64, write: uint64] =
  var readB, writeB: uint64 = 0
  try:
    for line in lines("/proc/diskstats"):
      let parts = line.splitWhitespace()
      if parts.len >= 14:
        readB += parseUInt(parts[5]) * 512
        writeB += parseUInt(parts[9]) * 512
  except:
    discard
  return (readB, writeB)

proc getMetrics*(): SysMetrics =
  let mem = readMemInfo()
  let net_io = readNetDev()
  let disk = readDiskStats()
  result = SysMetrics(
    cpu_usage: readCpuUsage(),
    mem_total: mem.total,
    mem_free: mem.free,
    net_rx_bytes: net_io.rx,
    net_tx_bytes: net_io.tx,
    battery_percent: readBattery(),
    disk_read_bytes: disk.read,
    disk_write_bytes: disk.write
  )

proc printMetrics(m: SysMetrics) =
  echo "\e[36m--- SigmaOS System Monitor ---\e[0m"
  echo "CPU:        ", formatFloat(m.cpu_usage, ffDecimal, 1), "%"
  echo "Memory:     ", (m.mem_total - m.mem_free) div (1024*1024), " MB / ", m.mem_total div (1024*1024), " MB"
  echo "Network:    RX ", m.net_rx_bytes div 1024, " KB | TX ", m.net_tx_bytes div 1024, " KB"
  echo "Disk I/O:   Read ", m.disk_read_bytes div 1024, " KB | Write ", m.disk_write_bytes div 1024, " KB"
  if m.battery_percent >= 0:
    echo "Battery:    ", m.battery_percent, "%"
  echo "------------------------------\n"

# C-exported API for Rust FFI integration
proc sysmon_get_cpu_usage*(): cfloat {.exportc, dynlib.} =
  return cfloat(readCpuUsage())

proc sysmon_get_mem_total*(): culonglong {.exportc, dynlib.} =
  return culonglong(readMemInfo().total)

proc sysmon_get_mem_free*(): culonglong {.exportc, dynlib.} =
  return culonglong(readMemInfo().free)

# Unix Domain Socket Server
var sockPath = "/tmp/sigma_sysmon.sock"

proc serveUnixSocket() {.async.} =
  removeFile(sockPath)
  var server = newAsyncSocket(AF_UNIX, SOCK_STREAM, IPPROTO_IP)
  server.bindUnix(sockPath)
  server.listen()
  
  while true:
    let client = await server.accept()
    let metrics = getMetrics()
    let jsonStr = $(%*{
      "cpu_usage": metrics.cpu_usage,
      "mem_total": metrics.mem_total,
      "mem_free": metrics.mem_free,
      "net_rx": metrics.net_rx_bytes,
      "net_tx": metrics.net_tx_bytes,
      "battery": metrics.battery_percent,
      "disk_read": metrics.disk_read_bytes,
      "disk_write": metrics.disk_write_bytes
    }) & "\n"
    await client.send(jsonStr)
    client.close()

proc main() =
  let args = commandLineParams()
  var interval = 2000
  if args.len > 0:
    try:
      interval = parseInt(args[0])
    except:
      discard

  echo "Starting SigmaOS System Monitor daemon..."
  
  asyncCheck serveUnixSocket()
  
  while true:
    let metrics = getMetrics()
    printMetrics(metrics)
    poll(interval)

when isMainModule:
  main()
