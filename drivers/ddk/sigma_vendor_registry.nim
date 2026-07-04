# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# drivers/ddk/sigma_vendor_registry.nim — Hardware vendor partnership registry
# Tracks: certified drivers, vendor contacts, hardware support status,
# transparency incentives (open source bonus), certification pipeline.
#
# Language: Nim (stdlib only)

import std/[os, json, times, strutils, strformat, tables, osproc]

# ── Vendor registry types ─────────────────────────────────────────────────
type
  VendorTier = enum VtCommunity, VtPartner, VtCertified, VtPlatinum

  DriverEntry = object
    vendor:       string
    device_name:  string
    vendor_id:    string
    device_ids:   seq[string]
    driver_name:  string
    source:       string    # "open" | "closed" | "ai-ported"
    status:       string    # "working" | "partial" | "wip" | "missing"
    file:         string
    notes:        string

  VendorEntry = object
    name:         string
    contact:      string
    tier:         VendorTier
    drivers:      seq[DriverEntry]
    open_pct:     int       # % of drivers that are open source
    score:        int       # transparency score 0-100

# ── Built-in registry (SigmaOS known hardware support) ───────────────────
const VENDOR_REGISTRY: array[12, DriverEntry] = [
  DriverEntry(vendor:"Intel",  device_name:"Ethernet (e1000/e1000e)",
              vendor_id:"8086", device_ids: @["0x100E","0x100F","0x10D3"],
              driver_name:"sigma-driver-intel-nic", source:"open",
              status:"working", file:"drivers/sovereignnic.rs", notes:""),
  DriverEntry(vendor:"Intel",  device_name:"Wi-Fi 6 AX200/AX201/AX210",
              vendor_id:"8086", device_ids: @["0x2723","0x02F0","0x2725"],
              driver_name:"sigma-driver-iwlwifi", source:"ai-ported",
              status:"partial", file:"drivers/net/sigma_wifi_driver.rs",
              notes:"Firmware loading stub; needs iwlwifi-*.ucode"),
  DriverEntry(vendor:"Intel",  device_name:"GPU (i915)",
              vendor_id:"8086", device_ids: @["0x3185","0x9BC5","0x9A49"],
              driver_name:"sigma-driver-i915", source:"open",
              status:"wip", file:"drivers/gpu/",
              notes:"Phase C — DRM/KMS mode setting needed"),
  DriverEntry(vendor:"AMD",    device_name:"Radeon GPU (amdgpu)",
              vendor_id:"1002", device_ids: @["0x687F","0x7310"],
              driver_name:"sigma-driver-amdgpu", source:"open",
              status:"wip", file:"drivers/gpu/",
              notes:"Phase C — amdgpu open source, needs porting"),
  DriverEntry(vendor:"NVIDIA", device_name:"GeForce (nouveau)",
              vendor_id:"10DE", device_ids: @["0x1C03","0x2206"],
              driver_name:"sigma-driver-nouveau", source:"ai-ported",
              status:"missing", file:"",
              notes:"NVIDIA blob not yet ported; nouveau reverse-eng needed"),
  DriverEntry(vendor:"Realtek",device_name:"RTL8111/8168 Ethernet",
              vendor_id:"10EC", device_ids: @["0x8111","0x8168"],
              driver_name:"sigma-driver-r8169", source:"ai-ported",
              status:"partial", file:"drivers/sovereignnic.rs",
              notes:"Basic init; full TX/RX path WIP"),
  DriverEntry(vendor:"Qualcomm",device_name:"Wi-Fi 6 QCA6390/WCN6855",
              vendor_id:"168C", device_ids: @["0x0034"],
              driver_name:"sigma-driver-ath11k", source:"ai-ported",
              status:"wip", file:"drivers/net/sigma_wifi_driver.rs",
              notes:"SDF struct done; firmware loading needed"),
  DriverEntry(vendor:"MediaTek",device_name:"Wi-Fi 6 MT7921/MT7922",
              vendor_id:"14C3", device_ids: @["0x7961","0x7922"],
              driver_name:"sigma-driver-mt76", source:"ai-ported",
              status:"wip", file:"drivers/net/sigma_wifi_driver.rs",
              notes:"Growing in Asia-Pacific market"),
  DriverEntry(vendor:"Samsung/SK Hynix",device_name:"NVMe SSD",
              vendor_id:"144D", device_ids: @["0xA804","0xA808"],
              driver_name:"sigma-driver-nvme", source:"open",
              status:"working", file:"drivers/sovereignnvme.rs", notes:""),
  DriverEntry(vendor:"Generic", device_name:"USB HID (keyboard/mouse)",
              vendor_id:"0000", device_ids: @["0x0000"],
              driver_name:"sigma-driver-hid", source:"open",
              status:"partial", file:"drivers/sovereignusb.rs",
              notes:"xHCI host works; HID parser WIP"),
  DriverEntry(vendor:"Realtek", device_name:"HD Audio",
              vendor_id:"10EC", device_ids: @["0x0215","0x0295"],
              driver_name:"sigma-driver-hda", source:"ai-ported",
              status:"missing", file:"drivers/audio/",
              notes:"Phase C — PipeWire integration needed"),
  DriverEntry(vendor:"Broadcom",device_name:"BCM4360 Wi-Fi",
              vendor_id:"14E4", device_ids: @["0x43A0"],
              driver_name:"sigma-driver-brcmfmac", source:"closed",
              status:"missing", file:"",
              notes:"Firmware blob required from Broadcom"),
]

proc status_color(status: string): string =
  case status
  of "working": "\e[38;2;52;211;153m✅\e[0m"
  of "partial":  "\e[38;2;251;191;36m🔄\e[0m"
  of "wip":      "\e[38;2;107;114;128m⬜\e[0m"
  else:          "\e[38;2;248;113;113m❌\e[0m"

proc source_label(source: string): string =
  case source
  of "open":      "\e[38;2;52;211;153mopen\e[0m"
  of "closed":    "\e[38;2;248;113;113mclosed\e[0m"
  of "ai-ported": "\e[38;2;168;85;247mAI-ported\e[0m"
  else:           source

# ── Transparency incentive scoring ────────────────────────────────────────
proc compute_vendor_score(vendor: string): int =
  var open_count = 0; var total = 0
  for d in VENDOR_REGISTRY:
    if d.vendor == vendor:
      total += 1
      if d.source == "open": open_count += 1
  if total == 0: return 0
  let open_pct = open_count * 100 div total
  # Score: open% * 0.5 + working% * 0.3 + has_contact * 0.2
  var working_count = 0
  for d in VENDOR_REGISTRY:
    if d.vendor == vendor and d.status == "working": working_count += 1
  let working_pct = working_count * 100 div total
  open_pct * 50 div 100 + working_pct * 30 div 100 + 20

# ── CLI ────────────────────────────────────────────────────────────────────
proc vendor_registry_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "help":
    echo """sigma-ddk-vendors — Hardware vendor registry & transparency tracker

Usage:
  sigma-ddk-vendors list              List all hardware support status
  sigma-ddk-vendors list <vendor>     Filter by vendor name
  sigma-ddk-vendors missing           Show missing/WIP drivers (opportunities)
  sigma-ddk-vendors score             Show vendor transparency scores
  sigma-ddk-vendors contact           How to submit a driver for certification
"""
    return

  case args[0].toLowerAscii
  of "list":
    let filter = if args.len > 1: args[1].toLowerAscii else: ""
    echo "\e[38;2;69;243;255m\e[1mΣ SigmaOS Hardware Support Registry\e[0m\n"
    echo fmt"  {'VENDOR':<12} {'DEVICE':<30} {'STATUS':>8}  {'SOURCE':<12}  FILE"
    echo fmt"  {'─'.repeat(90)}"
    for d in VENDOR_REGISTRY:
      if filter.len > 0 and filter notin d.vendor.toLowerAscii and filter notin d.device_name.toLowerAscii: continue
      let sc = status_color(d.status)
      let sl = source_label(d.source)
      let fname = d.file[0..<min(35,d.file.len)]
      echo fmt"  {d.vendor:<12} {d.device_name[0..<28]:<30} {sc}  {sl:<12}  {fname}"
      if d.notes.len > 0: echo fmt"    \e[38;2;107;114;128m{d.notes}\e[0m"

  of "missing","gaps","todo":
    echo "\e[38;2;69;243;255m\e[1mMissing/WIP Drivers — Contribution Opportunities\e[0m\n"
    for d in VENDOR_REGISTRY:
      if d.status in ["missing","wip","partial"]:
        let sc = status_color(d.status)
        echo fmt"  {sc} {d.vendor:<10} {d.device_name}"
        if d.notes.len > 0: echo fmt"     {d.notes}"
        echo fmt"     Port with: sigma-driver-porter port <linux_driver.c>"
    echo fmt"\n  Contribute at: https://github.com/AaryanSinghChauhan09/SigmaOS"

  of "score","transparency":
    echo "\e[38;2;69;243;255m\e[1mVendor Transparency Scores\e[0m\n"
    var seen: seq[string]
    for d in VENDOR_REGISTRY:
      if d.vendor in seen: continue
      seen.add(d.vendor)
      let score = compute_vendor_score(d.vendor)
      let bar_len = score * 20 div 100
      let bar = "█".repeat(bar_len) & "░".repeat(20-bar_len)
      let color = if score >= 70: "\e[38;2;52;211;153m"
                  elif score >= 40: "\e[38;2;251;191;36m"
                  else: "\e[38;2;248;113;113m"
      echo fmt"  {d.vendor:<12} {color}{bar}\e[0m {score}/100"
    echo fmt"\n  Improve your score: open-source your drivers, get certified"
    echo fmt"  Certified vendors: https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Driver-Development-Guide"

  of "contact","certify","submit":
    echo """How to get your driver certified:

  1. Build driver using sigma-ddk (sigma-shard-new --template <class>)
  2. Test with: cargo test && sigma-ddk validate <driver.so>
  3. Open an issue: https://github.com/AaryanSinghChauhan09/SigmaOS/issues
     Title: "Driver Certification: <vendor> <device>"
     Include: source code link, hardware datasheet, test results
  4. SigmaOS team reviews, signs with Dilithium-5
  5. Driver listed as SIGMA_DRV_FLAG_CERTIFIED in registry

  Incentives for open-sourcing:
    ✓ Higher transparency score
    ✓ Included in SigmaOS ISO for supported hardware
    ✓ sigma-capstore shows ✓ Certified badge
    ✓ Community maintenance and bug fixes
"""

  else: echo fmt"Unknown command: {args[0]}"
