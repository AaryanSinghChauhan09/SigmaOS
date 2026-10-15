# SigmaOS Udev Netlink Kernel Event Listener & Device Node Synthesizer
# Written in Nim using standard libraries for hotplug events.
# Inspired by systemd-udevd and FreeBSD devd.

import std/[strformat, strutils, tables]

type
  NetlinkDeviceAction* = enum
    ndaAdd,
    ndaRemove,
    ndaChange,
    ndaBind

  UdevDeviceRecord* = object
    subsystem*: string
    devpath*: string
    devname*: string
    major*: int
    minor*: int
    symlinks*: seq[string]

  UdevRulesEngine* = object
    records*: Table[string, UdevDeviceRecord]
    rules_count*: int

proc initUdevRulesEngine*(): UdevRulesEngine =
  result.records = initTable[string, UdevDeviceRecord]()
  result.rules_count = 0

proc processKernelUevent*(engine: var UdevRulesEngine, raw_event: string): bool =
  var lines = raw_event.splitLines()
  if lines.len == 0:
    return false

  var rec = UdevDeviceRecord(
    subsystem: "unknown",
    devpath: "",
    devname: "",
    major: 0,
    minor: 0,
    symlinks: @[]
  )

  for line in lines:
    if line.startsWith("SUBSYSTEM="):
      rec.subsystem = line.substr(10)
    elif line.startsWith("DEVNAME="):
      rec.devname = line.substr(8)
    elif line.startsWith("DEVPATH="):
      rec.devpath = line.substr(8)

  if rec.devname.len > 0:
    if rec.subsystem == "block":
      rec.symlinks.add(&"/dev/disk/by-id/sigma-{rec.devname}")
    engine.records[rec.devname] = rec
    return true
  return false
