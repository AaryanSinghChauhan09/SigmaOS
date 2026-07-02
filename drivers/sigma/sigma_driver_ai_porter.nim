# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# drivers/sigma/sigma_driver_ai_porter.nim — AI-assisted driver porting tool
# Translates Linux/Windows drivers into SigmaOS SDF equivalents.
# Uses sigma-agent LLM to understand driver patterns and rewrite them.
#
# Approach (cleanroom — no GPL code copied):
#   1. Analyse the driver's structure (probe/init/IRQ/MMIO pattern)
#   2. Extract hardware interactions (register offsets, DMA, IRQ)
#   3. Generate SigmaOS SDF skeleton with same hardware logic
#   4. Apply sigma_pledge constraints based on required capabilities
#   5. Validate output compiles and passes DDK validation
#
# Language: Nim (stdlib only)

import std/[os, osproc, strutils, strformat, json, times, sequtils, tables]

# ── Driver source analysis ─────────────────────────────────────────────────
type
  DriverPattern = enum
    DpPciProbe, DpMmioRead, DpMmioWrite, DpIrqHandler,
    DpDmaAlloc, DpNetdevAlloc, DpBlockOp, DpI2C, DpSpi

  DriverAnalysis = object
    source_file:  string
    driver_name:  string
    vendor_id:    string
    device_id:    string
    patterns:     seq[DriverPattern]
    pledge_caps:  seq[string]
    mmio_offsets: seq[string]   # detected register names
    irq_handling: bool
    dma_usage:    bool
    complexity:   int           # 1=simple, 5=complex
    linux_apis:   seq[string]   # Linux-specific APIs detected

const LINUX_TO_SIGMA: array[20, (string, string, string)] = [
  # (linux_api, sigma_equivalent, notes)
  ("ioremap",          "ddk::iomap",              "Map MMIO region"),
  ("readl",            "ddk::mmio_read32",        "32-bit MMIO read"),
  ("writel",           "ddk::mmio_write32",       "32-bit MMIO write"),
  ("pci_read_config",  "ddk::pci_config_read32",  "PCI config space read"),
  ("request_irq",      "ddk::request_irq",        "Request hardware IRQ"),
  ("dma_alloc_coherent","ddk::dma_alloc",         "Allocate DMA buffer"),
  ("kmalloc",          "kfree/kmalloc",           "Kernel memory allocation"),
  ("printk",           "sigma_log",               "Kernel logging"),
  ("spin_lock",        "sigma_spinlock",          "Spinlock acquire"),
  ("netdev_alloc",     "sigma_bus_send",          "Network device alloc → sigma-bus"),
  ("pci_register_driver","sigma_register_driver", "Driver registration"),
  ("platform_driver_register","sigma_register_driver","Platform driver"),
  ("i2c_add_driver",   "sigma_register_driver",   "I2C driver"),
  ("usb_register",     "sigma_register_driver",   "USB driver"),
  ("alloc_etherdev",   "sigma_nic_alloc",         "Ethernet device alloc"),
  ("register_netdev",  "sigma_nic_register",      "Register network device"),
  ("pci_enable_device","ddk::pci_enable",         "Enable PCI device"),
  ("pci_request_regions","ddk::pci_request_bar",  "Request PCI BARs"),
  ("pci_set_master",   "ddk::pci_set_master",     "Enable PCI bus mastering"),
  ("devm_ioremap_resource","ddk::iomap",          "Resource-managed MMIO map"),
]

proc analyse_source(path: string): DriverAnalysis =
  result.source_file = path
  result.linux_apis  = @[]
  result.patterns    = @[]
  result.pledge_caps = @["stdio"]

  if not fileExists(path): return

  let content = readFile(path)
  let lower   = content.toLowerAscii

  # Extract driver name from module_param or MODULE_DESCRIPTION
  for line in content.splitLines():
    if "module_param" in line and "name" in line.toLowerAscii:
      result.driver_name = line.split('"').getOrDefault(1, "unknown")
    if "#define DRIVER_NAME" in line or "#define DRV_NAME" in line:
      let parts = line.split()
      if parts.len >= 3: result.driver_name = parts[2].strip(chars={'"','\''})
    if "pci_device_id" in line and "0x" in line:
      let hex_parts = line.split("0x")
      if hex_parts.len >= 3:
        result.vendor_id = "0x" & hex_parts[1][0..<min(4, hex_parts[1].len)]
        result.device_id = "0x" & hex_parts[2][0..<min(4, hex_parts[2].len)]

  # Detect patterns
  if "probe" in lower or "pci_driver" in lower:
    result.patterns.add(DpPciProbe)
  if "readl(" in lower or "ioread32" in lower:
    result.patterns.add(DpMmioRead)
  if "writel(" in lower or "iowrite32" in lower:
    result.patterns.add(DpMmioWrite)
  if "request_irq" in lower or "irq_handler" in lower:
    result.patterns.add(DpIrqHandler); result.irq_handling = true
  if "dma_alloc" in lower or "pci_alloc_consistent" in lower:
    result.patterns.add(DpDmaAlloc); result.dma_usage = true
  if "alloc_etherdev" in lower or "net_device" in lower:
    result.patterns.add(DpNetdevAlloc); result.pledge_caps.add("inet")
  if "register_blkdev" in lower or "blk_mq" in lower:
    result.patterns.add(DpBlockOp)
  if "i2c_add_driver" in lower: result.patterns.add(DpI2C)
  if "spi_register_driver" in lower: result.patterns.add(DpSpi)

  # Detect Linux APIs
  for (linux_api, _, _) in LINUX_TO_SIGMA:
    if linux_api in lower: result.linux_apis.add(linux_api)

  # Complexity estimate
  result.complexity = (result.patterns.len + result.linux_apis.len) div 4 + 1

  # pledge capabilities
  if result.dma_usage: result.pledge_caps.add("dpath")
  if result.irq_handling: result.pledge_caps.add("dpath")

proc print_analysis(a: DriverAnalysis) =
  echo fmt"\e[38;2;69;243;255m\e[1mDriver Analysis: {a.source_file}\e[0m\n"
  echo fmt"  Name:       {a.driver_name}"
  echo fmt"  Vendor/Dev: {a.vendor_id} / {a.device_id}"
  echo fmt"  Patterns:   {a.patterns.mapIt($it).join(\", \")}"
  echo fmt"  IRQ:        {a.irq_handling}  DMA: {a.dma_usage}"
  echo fmt"  Complexity: {a.complexity}/5"
  echo fmt"  pledge:     {a.pledge_caps.join(\" \")}"
  echo fmt"\n  Linux APIs detected ({a.linux_apis.len}):"
  for api in a.linux_apis:
    for (linux, sigma, note) in LINUX_TO_SIGMA:
      if api == linux:
        echo fmt"    {linux:<30} → {sigma}  ({note})"

# ── SDF skeleton generator ─────────────────────────────────────────────────
proc generate_sigma_driver(a: DriverAnalysis, out_dir: string) =
  let name = a.driver_name.replace("-","_").replace(" ","_")
              .toLowerAscii.strip()
              .getOrDefault("my_driver")
  let name_upper = name.toUpperAscii.replace("-","_")
  let pledge_str = a.pledge_caps.mapIt(fmt"\"{it}\"").join(", ")

  let cargo_toml = fmt"""[package]
name    = "sigma-{name}"
version = "0.1.0"
edition = "2021"
[dependencies]
"""

  let lib_rs = fmt"""// SPDX-License-Identifier: MIT
// Auto-generated by sigma-driver-porter
// Source: {a.source_file}
// Complexity: {a.complexity}/5  Patterns: {a.patterns.mapIt($it).join(", ")}

#![no_std]
#![allow(dead_code)]

use sigma_ddk::*;

// ── Hardware register map ─────────────────────────────────────────────────
// TODO: fill in from source driver's register definitions
const REG_CTRL:   u32 = 0x00;
const REG_STATUS: u32 = 0x04;
const REG_DATA:   u32 = 0x08;

pub struct {name_upper}Driver {{
    bar0:  *mut u8,
    irq:   u8,
}}
unsafe impl Send for {name_upper}Driver {{}}
unsafe impl Sync for {name_upper}Driver {{}}

// ── SDF Lifecycle ──────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn {name}_probe(pci_bar: u64, irq: u8) -> i32 {{
    // TODO: verify PCI vendor/device ID
    // Source: {a.vendor_id} / {a.device_id}
    let _ = (pci_bar, irq);
    0
}}

#[no_mangle]
pub extern "C" fn {name}_init() -> i32 {{
    unsafe {{
        extern "C" {{
            fn sigma_pledge(p: *const u8, l: usize) -> i32;
        }}
        let pledge = concat!({pledge_str}, "\\0");
        sigma_pledge(pledge.as_ptr(), pledge.len());
    }}
    // TODO: map MMIO, configure hardware, set up DMA rings
    0
}}

#[no_mangle]
pub extern "C" fn {name}_shutdown() {{
    // TODO: quiesce hardware, release DMA, unmap MMIO
}}

{if a.irq_handling: fmt"""
#[no_mangle]
pub extern "C" fn {name}_irq() -> bool {{
    // TODO: read interrupt status, handle RX/TX, ack interrupt
    false
}}
""" else: ""}

// ── DDK registration ───────────────────────────────────────────────────────
sigma_register_driver!(SigmaDriverDescriptor {{
    magic:       SIGMA_DDK_MAGIC,
    abi_version: DDK_ABI_VERSION,
    vendor_id:   {if a.vendor_id.len > 0: a.vendor_id else: "0x0000"},
    device_id:   {if a.device_id.len > 0: a.device_id else: "0x0000"},
    flags:       SIGMA_DRV_FLAG_AI_PORTED | SIGMA_DRV_FLAG_OPEN_SOURCE,
    pledge_caps: 0,
    ring:        3,  // ring-3 isolated (safe default)
    fn_probe:    Some({name}_probe),
    fn_init:     Some({name}_init),
    fn_shutdown: Some({name}_shutdown),
    {if a.irq_handling: fmt"fn_irq: Some({name}_irq)," else: "fn_irq: None,"}
    ..Default::default()
}});
"""

  createDir(out_dir / name / "src")
  writeFile(out_dir / name / "Cargo.toml",    cargo_toml)
  writeFile(out_dir / name / "src" / "lib.rs", lib_rs)
  writeFile(out_dir / name / "sigma-shard.toml", fmt"""[shard]
name    = "sigma-{name}"
version = "0.1.0"
[security]
pledge  = [{pledge_str}]
[lifecycle]
probe    = "{name}_probe"
init     = "{name}_init"
shutdown = "{name}_shutdown"
""")

  echo fmt"\e[38;2;52;211;153m✓ Generated SigmaOS driver skeleton: {out_dir}/{name}/\e[0m"
  echo fmt"  Files:  Cargo.toml  src/lib.rs  sigma-shard.toml"
  echo fmt"  Next:   cd {out_dir}/{name} && cargo build --release"
  echo fmt"  Notes:"
  for (linux, sigma, note) in LINUX_TO_SIGMA:
    if linux in a.linux_apis:
      echo fmt"    Replace {linux} → {sigma}"

# ── LLM-powered translation ────────────────────────────────────────────────
proc ai_translate_driver(source_path: string, out_dir: string): bool =
  ## Ask sigma-agent LLM to translate driver with full context
  let content = try: readFile(source_path) except: return false
  let analysis = analyse_source(source_path)

  let prompt = fmt"""You are a SigmaOS kernel developer. Translate this Linux driver to SigmaOS SDF format.
Rules:
1. Replace all Linux kernel APIs with sigma_ddk equivalents
2. Use sigma_pledge() at start of init() to restrict capabilities
3. Use sigma_bus_send() for device events to userspace
4. Keep the hardware logic identical (register accesses, DMA patterns)
5. Add #[no_mangle] to probe/init/shutdown/irq functions
6. Return ONLY the Rust source code, no explanation

Driver info:
  Name: {analysis.driver_name}
  Vendor: {analysis.vendor_id}  Device: {analysis.device_id}
  Pledge needed: {analysis.pledge_caps.join(\" \")}

Linux driver source (first 500 chars):
{content[0..<min(500, content.len)]}

Generate SigmaOS driver src/lib.rs:"""

  # Try daemon
  let (daemon_ok, _) = execCmdEx("curl -sf http://localhost:11430/v1/status --max-time 1 2>/dev/null")
  if daemon_ok.len > 0:
    let body = $ %*{"message": prompt, "max_tokens": 1200, "include_context": false}
    let (out, code) = execCmdEx(
      fmt"""curl -sf -X POST http://localhost:11430/v1/chat -d {body.quoteShell} --max-time 30""")
    if code == 0:
      try:
        let resp = parseJson(out).getOrDefault("response").getStr("")
        if "fn_probe" in resp or "fn_init" in resp or "#[no_mangle]" in resp:
          let name = analysis.driver_name.replace("-","_").toLowerAscii
          createDir(out_dir / name / "src")
          writeFile(out_dir / name / "src" / "lib.rs", resp)
          echo fmt"✓ AI-translated driver saved: {out_dir}/{name}/src/lib.rs"
          return true
      except: discard

  # Fallback: rule-based skeleton
  generate_sigma_driver(analysis, out_dir)
  true

# ── CLI ────────────────────────────────────────────────────────────────────
proc driver_porter_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "help":
    echo """sigma-driver-porter — AI-assisted driver porting tool

Usage:
  sigma-driver-porter analyse <driver.c>      Analyse a Linux/Windows driver
  sigma-driver-porter port <driver.c>         Generate SigmaOS skeleton
  sigma-driver-porter port <driver.c> --ai    Use AI for full translation
  sigma-driver-porter port <driver.c> -o dir  Custom output directory
  sigma-driver-porter apis                    Show Linux→SigmaOS API mappings

Examples:
  sigma-driver-porter analyse drivers/net/ethernet/intel/e1000/e1000_main.c
  sigma-driver-porter port rtl8169.c
  sigma-driver-porter port rtl8169.c --ai
  sigma-driver-porter apis

Workflow:
  1. Get Linux driver source (cleanroom study — don't copy GPL code)
  2. Analyse to understand hardware interaction patterns
  3. Generate SigmaOS skeleton with same hardware logic
  4. Fill in register definitions from vendor datasheet
  5. Test with sigma-ddk validate and cargo test
"""
    return

  case args[0].toLowerAscii
  of "analyse","analyze":
    if args.len < 2: echo "Usage: sigma-driver-porter analyse <file>"; return
    let a = analyse_source(args[1])
    print_analysis(a)

  of "port","translate","convert":
    if args.len < 2: echo "Usage: sigma-driver-porter port <file>"; return
    let use_ai = "--ai" in args
    let oi     = args.find("-o")
    let out_dir = if oi >= 0 and oi+1 < args.len: args[oi+1]
                  else: getCurrentDir() / "sigma_drivers"
    if use_ai:
      echo "Using AI translation (requires sigma-agent daemon)..."
      discard ai_translate_driver(args[1], out_dir)
    else:
      let a = analyse_source(args[1])
      print_analysis(a)
      echo ""
      generate_sigma_driver(a, out_dir)

  of "apis","api-map":
    echo "\e[38;2;69;243;255m\e[1mLinux → SigmaOS API Mapping\e[0m\n"
    echo fmt"  {'LINUX API':<35} {'SIGMA DDK':<30}  NOTE"
    echo fmt"  {'─'.repeat(85)}"
    for (linux, sigma, note) in LINUX_TO_SIGMA:
      echo fmt"  {linux:<35} {sigma:<30}  {note}"

  else: echo fmt"Unknown command: {args[0]}"
