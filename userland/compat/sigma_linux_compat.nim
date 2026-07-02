# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/compat/sigma_linux_compat.nim — Linux binary compatibility layer
# Provides syscall translation, FHS path mapping, and ELF loader shim
# so unmodified Linux binaries run on SigmaOS.
#
# Architecture:
#   Linux ELF binary → SigmaOS ELF loader → syscall translation table
#   → SigmaOS native syscalls → kernel
#
# Language: Nim + C interop (kernel compat layer uses Rust: kernel/compat/)

import std/[os, osproc, strutils, strformat, tables, json]

# ── Linux → SigmaOS FHS path mapping ─────────────────────────────────────────
# Linux binaries expect paths like /usr/lib, /etc, /proc that map to
# SigmaOS equivalents
const PATH_MAP: array[24, (string, string)] = [
  ("/usr/lib",          "/sigma/lib"),
  ("/usr/lib64",        "/sigma/lib64"),
  ("/usr/bin",          "/sigma/bin"),
  ("/usr/sbin",         "/sigma/sbin"),
  ("/usr/share",        "/sigma/share"),
  ("/usr/include",      "/sigma/include"),
  ("/usr/local/bin",    "/sigma/local/bin"),
  ("/usr/local/lib",    "/sigma/local/lib"),
  ("/etc/apt",          "/sigma/compat/apt"),
  ("/etc/yum.repos.d",  "/sigma/compat/yum"),
  ("/var/lib/dpkg",     "/sigma/compat/dpkg"),
  ("/lib/x86_64-linux-gnu", "/sigma/lib"),
  ("/lib64",            "/sigma/lib64"),
  ("/proc",             "/sigma/proc"),
  ("/sys",              "/sigma/sys"),
  ("/dev",              "/sigma/dev"),
  ("/run",              "/sigma/run"),
  ("/tmp",              "/sigma/tmp"),
  ("/home",             "/sigma/home"),
  ("/root",             "/sigma/root"),
  ("/opt",              "/sigma/opt"),
  ("/srv",              "/sigma/srv"),
  ("/snap",             "/sigma/snap"),
  ("/flatpak",          "/sigma/flatpak"),
]

proc translate_path*(linux_path: string): string =
  for (linux, sigma) in PATH_MAP:
    if linux_path.startsWith(linux):
      return sigma & linux_path[linux.len..^1]
  linux_path

# ── Linux syscall number → SigmaOS syscall mapping ───────────────────────────
# Linux x86_64 syscall numbers → SigmaOS equivalents
# Reference: linux/arch/x86/entry/syscalls/syscall_64.tbl
const SYSCALL_MAP: array[30, (int, int, string)] = [
  (0,  0,  "read"),
  (1,  1,  "write"),
  (2,  2,  "open"),
  (3,  3,  "close"),
  (4,  4,  "stat"),
  (5,  5,  "fstat"),
  (6,  6,  "lstat"),
  (7,  7,  "poll"),
  (8,  8,  "lseek"),
  (9,  9,  "mmap"),
  (10, 10, "mprotect"),
  (11, 11, "munmap"),
  (12, 12, "brk"),
  (13, 13, "rt_sigaction"),
  (14, 14, "rt_sigprocmask"),
  (39, 39, "getpid"),
  (57, 57, "fork"),
  (58, 58, "vfork"),
  (59, 59, "execve"),
  (60, 60, "exit"),
  (61, 61, "wait4"),
  (62, 62, "kill"),
  (63, 63, "uname"),
  (78, 78, "gettimeofday"),
  (79, 79, "settimeofday"),
  (87, 87, "unlink"),
  (89, 89, "readlink"),
  (102, 102, "getuid"),
  (104, 104, "getgid"),
  (231, 231, "exit_group"),
]

proc translate_syscall*(linux_syscall_nr: int): int =
  for (linux_nr, sigma_nr, _) in SYSCALL_MAP:
    if linux_nr == linux_syscall_nr: return sigma_nr
  -1  # unsupported syscall

# ── Compatibility environment setup ──────────────────────────────────────────
proc setup_compat_env*() =
  ## Set environment variables needed for Linux binaries
  let compat_dir = "/sigma/compat"
  for dir in ["/sigma/lib", "/sigma/lib64", "/sigma/bin", "/sigma/compat"]:
    try: createDir(dir) except: discard

  # LD_LIBRARY_PATH for Linux shared libraries
  let existing_ldpath = getEnv("LD_LIBRARY_PATH", "")
  putEnv("LD_LIBRARY_PATH",
    "/sigma/lib:/sigma/lib64:/sigma/local/lib" &
    (if existing_ldpath.len > 0: ":" & existing_ldpath else: ""))

  # XDG paths
  putEnv("XDG_DATA_DIRS", "/sigma/share:/sigma/local/share:/usr/share")
  putEnv("XDG_CONFIG_DIRS", "/sigma/etc/xdg:/etc/xdg")
  putEnv("PATH",
    "/sigma/bin:/sigma/sbin:/sigma/local/bin:" & getEnv("PATH",""))

proc run_linux_binary*(binary_path: string, args: seq[string] = @[]): int =
  ## Run a Linux ELF binary under the SigmaOS compat layer
  if not fileExists(binary_path):
    echo fmt"✗ Binary not found: {binary_path}"; return 127

  setup_compat_env()

  # Check if we have binfmt_misc or sigma-compat-loader
  let loader = findExe("sigma-compat-loader")
  let cmd = if loader.len > 0:
    @[loader, binary_path] & args
  else:
    @[binary_path] & args  # Direct exec (works if ELF interp is available)

  let (out, code) = execCmdEx(cmd.join(" "))
  echo out
  code

# ── Container/OCI runtime integration ────────────────────────────────────────
proc run_oci_container*(image: string, cmd_args: seq[string] = @[]): int =
  ## Run an OCI container via sigma-pod (runc-compatible)
  let sigma_pod = findExe("sigma-pod")
  if sigma_pod.len > 0:
    let cmd = @[sigma_pod, "run", "--rm", image] & cmd_args
    let (out, code) = execCmdEx(cmd.join(" "))
    echo out; return code

  # Fallback: try docker / podman
  for runtime in ["podman", "docker", "ctr"]:
    if findExe(runtime).len > 0:
      echo fmt"(using {runtime} as OCI runtime)"
      let cmd = @[runtime, "run", "--rm", image] & cmd_args
      let (out, code) = execCmdEx(cmd.join(" "))
      echo out; return code

  echo "✗ No OCI runtime found. Install: sigma-pkg install sigma-pod"
  127

# ── Compat status report ──────────────────────────────────────────────────────
proc compat_status*() =
  echo "\e[38;2;69;243;255m\e[1mΣ SigmaOS Linux Compatibility Status\e[0m\n"

  let checks = [
    ("dpkg-deb",       "Debian package absorption",      "sigma-pkg install dpkg-tools"),
    ("rpm2cpio",       "RPM package absorption",         "sigma-pkg install rpm-tools"),
    ("flatpak",        "Flatpak app support",            "sigma-pkg install flatpak"),
    ("sigma-pod",      "OCI/Docker container runtime",  "sigma-pkg install sigma-pod"),
    ("sigma-compat-loader", "Linux ELF loader",         "sigma-pkg install sigma-compat"),
    ("bwrap",          "Bubblewrap sandbox",             "sigma-pkg install bubblewrap"),
  ]

  for (binary, desc, fix) in checks:
    let found = findExe(binary).len > 0
    let icon = if found: "\e[38;2;52;211;153m✓\e[0m" else: "\e[38;2;251;191;36m⚠\e[0m"
    echo fmt"  {icon}  {desc:<35}  {if found: binary else: \"Install: \" & fix}"

  echo fmt"\n  Path translation:  {PATH_MAP.len} Linux paths mapped to SigmaOS equivalents"
  echo fmt"  Syscall mapping:   {SYSCALL_MAP.len} Linux syscalls translated"
  echo "\n  For full binary compatibility: sigma-pkg install sigma-compat-layer"

# ── CLI ────────────────────────────────────────────────────────────────────────
proc compat_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "help":
    echo """sigma-compat — Linux binary & package compatibility layer

Usage:
  sigma-compat status              Show compatibility layer status
  sigma-compat run <binary>        Run a Linux ELF binary
  sigma-compat container <image>   Run an OCI/Docker container
  sigma-compat path <linux-path>   Translate a Linux path to SigmaOS
  sigma-compat absorb <pkg-file>   Absorb a Linux package (delegates to sigma-pkg absorb)

Examples:
  sigma-compat status
  sigma-compat run /usr/bin/vim
  sigma-compat container ubuntu:22.04
  sigma-compat path /usr/lib/x86_64-linux-gnu
  sigma-compat absorb firefox.deb
"""
    return

  case args[0].toLowerAscii
  of "status": compat_status()
  of "run":
    if args.len < 2: echo "Usage: sigma-compat run <binary>"; return
    let code = run_linux_binary(args[1], args[2..^1])
    if code != 0: echo fmt"Exit code: {code}"
  of "container","docker","oci":
    if args.len < 2: echo "Usage: sigma-compat container <image>"; return
    let code = run_oci_container(args[1], args[2..^1])
    if code != 0: echo fmt"Exit code: {code}"
  of "path":
    if args.len < 2: echo "Usage: sigma-compat path <linux-path>"; return
    echo translate_path(args[1])
  of "absorb":
    if args.len < 2: echo "Usage: sigma-compat absorb <pkg-file>"; return
    let (out, _) = execCmdEx(fmt"sigma-pkg absorb {args[1..^1].mapIt(it.quoteShell).join(\" \")} 2>&1")
    echo out
  else:
    echo fmt"Unknown command: {args[0]}"
