# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# tools/sigma_kernel_doc.nim — Kernel self-documentation API
# Novel Category 10 (Meta-System): Userspace can query "what does this syscall do?"
# and the kernel returns auto-generated documentation.
# Also: capability matrix diagram, shard graph, sigma-kernel-tour walkthrough.
#
# Language: Nim (stdlib only)

import std/[os, osproc, strutils, strformat, tables, json, sequtils]

# ── Built-in syscall documentation ───────────────────────────────────────
const SYSCALL_DOCS: array[35, (int, string, string, string, string)] = [
  # (nr, name, signature, description, pledge_required)
  (0,  "read",    "read(fd, buf, count) -> ssize_t",
   "Read up to count bytes from file descriptor fd into buffer buf. Returns bytes read, 0 on EOF, -1 on error.",
   "stdio rpath"),
  (1,  "write",   "write(fd, buf, count) -> ssize_t",
   "Write count bytes from buf to file descriptor fd. Returns bytes written, -1 on error.",
   "stdio wpath"),
  (2,  "open",    "open(path, flags, mode) -> int",
   "Open or create a file. flags: O_RDONLY=0, O_WRONLY=1, O_RDWR=2, O_CREAT=64, O_TRUNC=512. Returns fd or -1.",
   "rpath wpath"),
  (3,  "close",   "close(fd) -> int",
   "Close file descriptor fd, releasing its resources. Returns 0 on success.",
   "stdio"),
  (4,  "stat",    "stat(path, statbuf) -> int",
   "Get file metadata (size, permissions, timestamps) for path. Fills statbuf struct.",
   "rpath"),
  (9,  "mmap",    "mmap(addr, length, prot, flags, fd, offset) -> void*",
   "Map files or anonymous memory into process address space. prot: PROT_READ=1, PROT_WRITE=2, PROT_EXEC=4.",
   "vminfo"),
  (11, "munmap",  "munmap(addr, length) -> int",
   "Unmap memory previously mapped with mmap. addr must be page-aligned.",
   "vminfo"),
  (12, "brk",     "brk(addr) -> int",
   "Change the location of the program break (end of heap). Used by malloc internally.",
   "vminfo"),
  (39, "getpid",  "getpid() -> pid_t",
   "Return the process ID of the calling process. Fast: often implemented as vDSO.",
   "stdio"),
  (41, "socket",  "socket(domain, type, protocol) -> int",
   "Create a socket. domain: AF_INET=2, AF_INET6=10, AF_UNIX=1. type: SOCK_STREAM=1, SOCK_DGRAM=2.",
   "inet unix"),
  (42, "connect", "connect(fd, addr, addrlen) -> int",
   "Connect socket to address. For TCP: initiates 3-way handshake. Returns 0 on success.",
   "inet"),
  (57, "fork",    "fork() -> pid_t",
   "Create a child process (exact copy of parent). Returns child PID to parent, 0 to child, -1 on error.",
   "proc"),
  (59, "execve",  "execve(path, argv, envp) -> int",
   "Replace current process with new program at path. argv[0] = program name. envp = environment.",
   "exec"),
  (60, "exit",    "exit(status) -> noreturn",
   "Terminate current process with exit status. Calls atexit handlers. Parent gets status via wait().",
   "stdio"),
  (61, "wait4",   "wait4(pid, wstatus, options, rusage) -> pid_t",
   "Wait for child process to change state. pid=-1 = any child. Returns child PID on success.",
   "proc"),
  (62, "kill",    "kill(pid, sig) -> int",
   "Send signal sig to process pid. pid=0 = process group. Common signals: SIGTERM=15, SIGKILL=9.",
   "proc"),
  (63, "uname",   "uname(buf) -> int",
   "Return system information (OS name, hostname, kernel version, machine type) in buf.",
   "stdio"),
  (202,"futex",   "futex(addr, op, val, timeout, uaddr2, val3) -> int",
   "Fast userspace mutex. op: FUTEX_WAIT=0 (sleep if *addr==val), FUTEX_WAKE=1 (wake N waiters).",
   "stdio"),
  (228,"clock_gettime","clock_gettime(clkid, timespec) -> int",
   "Get current time. clkid: CLOCK_REALTIME=0, CLOCK_MONOTONIC=1. Often vDSO (no syscall overhead).",
   "stdio"),
  (231,"exit_group","exit_group(status) -> noreturn",
   "Terminate all threads in current thread group (the normal exit path for multi-threaded programs).",
   "stdio"),

  # SigmaOS-native syscalls
  (0x8001, "sigma_pledge", "sigma_pledge(promises, len) -> int",
   "Restrict process capabilities to declared promises (like OpenBSD pledge). Cannot be reversed. " &
   "Promises: 'stdio rpath wpath cpath inet unix exec proc vminfo audio video'. " &
   "Any syscall outside declared promises → SIGKILL.",
   ""),
  (0x8002, "sigma_unveil", "sigma_unveil(path, pathlen, perms, permlen) -> int",
   "Restrict filesystem visibility to declared paths (like OpenBSD unveil). " &
   "All other paths become ENOENT. perms: 'r'=read 'w'=write 'x'=exec 'c'=create. " &
   "Call with path=NULL to lock (no more unveil calls allowed).",
   ""),
  (0x8010, "sigma_bus_send", "sigma_bus_send(channel, data, len) -> int",
   "Send a typed message to a sigma-bus IPC channel. Zero-copy for large messages. " &
   "channel: BUS_NETWORK=0x01 BUS_STORAGE=0x02 BUS_SECURITY=0x03 BUS_AI=0x04. " &
   "Returns 0 on success, -EAGAIN if buffer full.",
   "stdio"),
  (0x8011, "sigma_bus_recv", "sigma_bus_recv(channel, buf, len, timeout_ms) -> int",
   "Receive a message from a sigma-bus IPC channel. Blocks until message available or timeout. " &
   "Returns bytes received, 0 on timeout, -1 on error.",
   "stdio"),
  (0x8040, "sigma_attest",  "sigma_attest(buf, len) -> int",
   "Generate TPM2 attestation quote (PCR values + signature) into buf. " &
   "Proves system state to remote verifier without revealing secrets. " &
   "Requires TPM2 hardware and sigma-trustd daemon.",
   ""),
  (0x8050, "sigma_ai_infer", "sigma_ai_infer(prompt, plen, out, outlen) -> int",
   "Route LLM inference request to sigma-ai daemon via /run/sigma/ai.sock. " &
   "Backends: sigma-ai → Ollama → llama.cpp → offline fallback. " &
   "Returns bytes written to out, -ENODEV if no backend available.",
   "inet unix"),
]

# ── Shard capability matrix ────────────────────────────────────────────────
const SHARD_CAPABILITIES: array[12, (string, string, seq[string])] = [
  ("sigma-sched", "CPU scheduling (MLFQ+CFS+EDF)",
   @["BUS_SCHED", "SCHED_TICK_IRQ", "PREEMPT_POINT"]),
  ("sigma-mm",    "Memory management (buddy+slab)",
   @["BUS_MEMORY", "PAGE_FAULT_HANDLER", "MMAP_SYSCALL"]),
  ("sigma-vfs",   "Virtual filesystem layer",
   @["BUS_FS", "OPEN_SYSCALL", "READ_SYSCALL", "WRITE_SYSCALL"]),
  ("sigma-net",   "Network stack (IPv4/IPv6/TLS)",
   @["BUS_NETWORK", "SOCKET_SYSCALL", "TCP_STATE_MACHINE"]),
  ("sigma-sec",   "Security (pledge/unveil/AVC)",
   @["BUS_SECURITY", "PLEDGE_SYSCALL", "UNVEIL_SYSCALL"]),
  ("sigma-pkg",   "Package manager",
   @["BUS_PACKAGES", "sigma_pkg_install", "sigma_pkg_verify"]),
  ("sigma-ai",    "Local LLM inference",
   @["BUS_AI", "sigma_ai_infer", "AI_SOCKET_UNIX"]),
  ("sigma-drv",   "Sovereign Driver Framework",
   @["BUS_DRIVERS", "SDF_PROBE", "SDF_INIT", "SDF_SHUTDOWN"]),
  ("sigma-ipc",   "Inter-shard message bus",
   @["BUS_IPC", "sigma_bus_send", "sigma_bus_recv"]),
  ("sigma-audit", "Blockchain audit log",
   @["BUS_AUDIT", "AUDIT_LOG_EVENT", "AUDIT_VERIFY_CHAIN"]),
  ("sigma-swarm", "Cluster load balancer",
   @["BUS_SWARM", "GOSSIP_PROTOCOL", "PHEROMONE_TRAIL"]),
  ("zenith-wm",   "Window manager (Zenith Desktop)",
   @["BUS_DISPLAY", "WM_TILE", "WM_WORKSPACE", "WM_FLOAT"]),
]

# ── Kernel tour ────────────────────────────────────────────────────────────
const KERNEL_TOUR: array[10, (string, string, string)] = [
  ("Boot",       "0x0000_0000 – 0x0001_0000",
   "Bootloader (sigma-boot.efi) loads kernel, sets up GDT/IDT, enters 64-bit mode, calls kernel_main()"),
  ("IDT",        "kernel/arch/x86_64/idt.rs",
   "Interrupt Descriptor Table — 256 entries, maps IRQ/exception vectors to handler functions"),
  ("Scheduler",  "kernel/sched/sigma_mlfq.rs",
   "MLFQ+CFS+EDF scheduler. tick() called every 1ms by timer IRQ. schedule() picks next task."),
  ("Memory",     "kernel/memory/sigma_buddy.rs",
   "Buddy allocator manages physical pages in 2^n blocks. Slab allocator provides kmalloc()."),
  ("Syscalls",   "kernel/syscalls/sigma_syscall_table.rs",
   "50+ POSIX syscalls + SigmaOS-native (sigma_pledge, sigma_unveil, sigma_bus_*, sigma_ai_infer)."),
  ("Security",   "kernel/security/",
   "sigma_pledge: per-process capability bitmap. sigma_unveil: path visibility filter. AVC O(1) cache."),
  ("Filesystem", "fs/sigma_vfs.zig",
   "VFS layer: generic inode/dentry/file ops. Backends: Ext4, SigmaFS (CoW), Tmpfs, FAT32."),
  ("Network",    "kernel/net/sigma_tcp.rs",
   "IPv4/IPv6, TCP state machine (RFC 793), BBR congestion control, TLS 1.3 + Kyber-1024."),
  ("Shards",     "suites/",
   "600+ capability shards (S01–S500+). Each shard: probe()→init()→run()→shutdown() via sigma-bus."),
  ("sigma-ai",   "userland/ai/sigma_ai.rs",
   "On-device LLM daemon. Transformer inference via llama.cpp/Ollama/sigma-ai backends."),
]

# ── CLI ────────────────────────────────────────────────────────────────────
proc kernel_doc_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "help":
    echo """sigma-kernel-doc — Kernel self-documentation API

Usage:
  sigma-kernel-doc syscall <name|nr>     Document a syscall
  sigma-kernel-doc syscalls [category]   List all syscalls
  sigma-kernel-doc shards                Capability matrix diagram
  sigma-kernel-doc shard <name>          Shard details
  sigma-kernel-doc tour                  Kernel architecture walkthrough
  sigma-kernel-doc search <query>        Search documentation

Examples:
  sigma-kernel-doc syscall read
  sigma-kernel-doc syscall sigma_pledge
  sigma-kernel-doc syscall 0x8001
  sigma-kernel-doc shards
  sigma-kernel-doc tour
  sigma-kernel-doc search mmap
"""
    return

  case args[0].toLowerAscii
  of "syscall":
    let query = if args.len > 1: args[1].toLowerAscii else: ""
    var found = false
    for (nr, name, sig, desc, pledge) in SYSCALL_DOCS:
      let nr_match = try: parseInt(query) == nr except: false
      let hex_match = try: parseHexInt(query) == nr except: false
      if name == query or nr_match or hex_match or (nr_match or hex_match):
        echo fmt"\e[38;2;69;243;255m\e[1m{name}\e[0m  (nr={nr}  0x{nr:04X})"
        echo fmt"\e[38;2;168;85;247mSignature:\e[0m  {sig}"
        echo ""
        # Word-wrap description
        var line = ""; var words = desc.split()
        for word in words:
          if line.len + word.len > 80: echo fmt"  {line}"; line = word & " "
          else: line &= word & " "
        if line.len > 0: echo fmt"  {line}"
        echo ""
        if pledge.len > 0: echo fmt"\e[38;2;251;191;36mRequires pledge:\e[0m  {pledge}"
        found = true; break
    if not found: echo fmt"syscall '{query}' not found. Try: sigma-kernel-doc syscalls"

  of "syscalls":
    let filter = if args.len > 1: args[1].toLowerAscii else: ""
    echo "\e[38;2;69;243;255m\e[1mSigmaOS Syscall Reference\e[0m\n"
    echo fmt"  {'NR':>8}  {'NAME':<20}  DESCRIPTION"
    echo fmt"  {'─'.repeat(70)}"
    for (nr, name, _, desc, _) in SYSCALL_DOCS:
      if filter.len > 0 and filter notin name.toLowerAscii and filter notin desc.toLowerAscii: continue
      let is_sigma = nr >= 0x8000
      let color = if is_sigma: "\e[38;2;168;85;247m" else: "\e[38;2;52;211;153m"
      let tag   = if is_sigma: " [Σ]" else: ""
      echo fmt"  {color}{nr:>8X}\e[0m  {name:<20}  {desc[0..<min(45,desc.len)]}{tag}"

  of "shards":
    echo "\e[38;2;69;243;255m\e[1mSigmaOS Shard Capability Matrix\e[0m\n"
    echo fmt"  {'SHARD':<20} {'DESCRIPTION':<40}  CHANNELS"
    echo fmt"  {'─'.repeat(90)}"
    for (name, desc, channels) in SHARD_CAPABILITIES:
      echo fmt"  \e[38;2;69;243;255m{name:<20}\e[0m {desc[0..<min(38,desc.len)]:<40}  {channels.join(\" \")}"

  of "shard":
    let query = if args.len > 1: args[1].toLowerAscii else: ""
    for (name, desc, channels) in SHARD_CAPABILITIES:
      if query in name.toLowerAscii:
        echo fmt"\e[38;2;69;243;255m\e[1m{name}\e[0m"
        echo fmt"  {desc}"
        echo fmt"\n  Channels: {channels.join(\", \")}"
        return
    echo fmt"Shard '{query}' not found"

  of "tour":
    echo "\e[38;2;69;243;255m\e[1mΣ Kernel Architecture Walkthrough\e[0m\n"
    for (component, location, description) in KERNEL_TOUR:
      echo fmt"\e[38;2;168;85;247m\e[1m{component}\e[0m  \e[38;2;107;114;128m{location}\e[0m"
      echo fmt"  {description}\n"

  of "search":
    let query = if args.len > 1: args[1..^1].join(" ").toLowerAscii else: ""
    echo fmt"Search results for '{query}':\n"
    for (nr, name, sig, desc, _) in SYSCALL_DOCS:
      if query in name.toLowerAscii or query in desc.toLowerAscii:
        echo fmt"  syscall {name:<20} {desc[0..<min(50,desc.len)]}"
    for (shard, desc, _) in SHARD_CAPABILITIES:
      if query in shard.toLowerAscii or query in desc.toLowerAscii:
        echo fmt"  shard   {shard:<20} {desc[0..<min(50,desc.len)]}"
    for (comp, loc, desc) in KERNEL_TOUR:
      if query in comp.toLowerAscii or query in desc.toLowerAscii:
        echo fmt"  kernel  {comp:<20} {desc[0..<min(50,desc.len)]}"

  else:
    echo fmt"Unknown command: {args[0]}"
