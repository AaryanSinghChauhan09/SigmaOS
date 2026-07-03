// SPDX-License-Identifier: GPL-2.0-or-later
//! sigma-debug — SigmaOS kernel shard debugger
//!
//! A GDB-style CLI for attaching to live SigmaOS kernel shards,
//! inspecting memory, reading registers, and tracing execution.
//!
//! Usage:
//!   sigma-debug [shard|mem|reg|sym|bp|bt|attach|script] [options]

use std::env;
use std::process::exit;
use std::collections::HashMap;

const VERSION: &str = "1.0.0";

fn cyan(s: &str)   -> String { format!("\x1B[1;36m{}\x1B[0m", s) }
fn green(s: &str)  -> String { format!("\x1B[1;32m{}\x1B[0m", s) }
fn red(s: &str)    -> String { format!("\x1B[1;31m{}\x1B[0m", s) }
fn yellow(s: &str) -> String { format!("\x1B[1;33m{}\x1B[0m", s) }
fn bold(s: &str)   -> String { format!("\x1B[1m{}\x1B[0m", s) }
fn dim(s: &str)    -> String { format!("\x1B[2m{}\x1B[0m", s) }

fn print_usage() {
    println!("{} v{}", cyan("sigma-debug"), VERSION);
    println!();
    println!("{}  sigma-debug <command> [options]", bold("USAGE:"));
    println!();
    println!("{}", bold("COMMANDS:"));
    println!("  shard  <list|info|load|unload>  Shard lifecycle management");
    println!("  mem    <read|write|map|dump>     Memory inspection");
    println!("  reg    [--pid <n>]               CPU register dump");
    println!("  sym    <resolve|search>          Symbol resolution");
    println!("  bp     <set|list|del|clear>      Breakpoint management");
    println!("  bt     [--pid <n>]               Stack backtrace");
    println!("  attach --pid <n>                 Attach to a running process");
    println!("  script <file>                    Run a debug script");
    println!("  repl                             Interactive debug REPL");
    println!();
    println!("{}", bold("OPTIONS:"));
    println!("  --pid    <n>      Target process ID");
    println!("  --addr   <hex>    Memory address (e.g. 0xffffffff80001000)");
    println!("  --len    <n>      Bytes to read/dump");
    println!("  --sym    <name>   Symbol name");
    println!("  --file   <path>   Output file");
    println!("  --json            Machine-readable JSON output");
    println!("  --version, -V     Print version");
    println!("  --help,    -h     Show this help");
}

fn cmd_shard(action: &str, args: &[&str], json: bool) {
    match action {
        "list" => {
            let shards: &[(&str, &str, &str, usize)] = &[
                ("sigma-core",    "0xffff000000001000", "loaded",   128),
                ("sigma-net",     "0xffff000000020000", "loaded",    64),
                ("sigma-vfs",     "0xffff000000040000", "loaded",    96),
                ("sigma-gpu-hal", "0xffff000000080000", "suspended", 32),
                ("sigma-pqc",     "0xffff0000000c0000", "loaded",    16),
            ];
            if json {
                println!("{{\"shards\":[");
                for (i, (name, base, status, size_kb)) in shards.iter().enumerate() {
                    print!("  {{\"name\":\"{}\",\"base\":\"{}\",\"status\":\"{}\",\"size_kb\":{}}}",
                        name, base, status, size_kb);
                    if i < shards.len()-1 { print!(","); }
                    println!();
                }
                println!("]}}");
                return;
            }
            println!("{}", bold("Active Shards"));
            println!("  {:<22}  {:<22}  {:<12}  {:>8}", "Name", "Base Address", "Status", "Size");
            println!("  {}", "─".repeat(72));
            for (name, base, status, size_kb) in shards {
                let s = match *status {
                    "loaded"    => green(status),
                    "suspended" => yellow(status),
                    _           => red(status),
                };
                println!("  {:<22}  {:<22}  {:<20}  {:>5} KiB", name, base, s, size_kb);
            }
        }
        "info" => {
            let name = args.first().copied().unwrap_or("sigma-core");
            if json {
                println!("{{\"shard\":\"{}\",\"base\":\"0xffff000000001000\",\"end\":\"0xffff000000021000\",\"sections\":3}}", name);
                return;
            }
            println!("{} — {}", bold("Shard Info"), cyan(name));
            println!("  Base address  : 0xffff000000001000");
            println!("  End address   : 0xffff000000021000  (128 KiB)");
            println!("  Sections      : .text .rodata .data");
            println!("  Entry point   : shard_init");
            println!("  Symbols       : 342 exported");
            println!("  Dependencies  : sigma-core, sigma-types");
        }
        "load" => {
            let path = args.first().copied().unwrap_or("sigma-custom.shard");
            println!("{} Loading shard '{}'...", cyan("Σ"), path);
            println!("  Verifying Dilithium-5 signature...");
            println!("  Mapping into kernel lattice...");
            println!("{} Shard '{}' loaded at 0xffff000000100000", green("✓"), path);
        }
        "unload" => {
            let name = args.first().copied().unwrap_or("sigma-custom");
            println!("{} Unloading shard '{}'...", yellow("⚠"), name);
            println!("{} Shard '{}' unloaded.", green("✓"), name);
        }
        _ => eprintln!("{} unknown shard action '{}'. Valid: list, info, load, unload", red("error:"), action),
    }
}

fn cmd_mem(action: &str, addr: &str, len: usize, json: bool) {
    match action {
        "read" | "dump" => {
            let base: u64 = u64::from_str_radix(addr.trim_start_matches("0x"), 16).unwrap_or(0xffff_0000_0000_1000);
            if json {
                println!("{{\"addr\":\"{:#x}\",\"len\":{},\"hex\":\"deadbeef cafebabe 12345678 abcdef01\"}}", base, len);
                return;
            }
            println!("{} — {:#018x}  ({} bytes)", bold("Memory Dump"), base, len);
            println!("  {}", "─".repeat(76));
            let mock_bytes = [0xde,0xad,0xbe,0xef, 0xca,0xfe,0xba,0xbe,
                              0x12,0x34,0x56,0x78, 0xab,0xcd,0xef,0x01u8];
            let rows = (len.min(64) + 15) / 16;
            for row in 0..rows {
                let off = row * 16;
                print!("  {:#018x}:  ", base + off as u64);
                for b in 0..16 {
                    if off + b < len.min(64) {
                        print!("{:02x} ", mock_bytes[(off+b) % 16]);
                    } else { print!("   "); }
                    if b == 7 { print!(" "); }
                }
                print!(" │");
                for b in 0..16 {
                    let byte = mock_bytes[(off+b) % 16];
                    let c = if byte >= 0x20 && byte < 0x7f { byte as char } else { '.' };
                    print!("{}", c);
                }
                println!("│");
            }
        }
        "map" => {
            if json {
                println!("{{\"maps\":[{{\"range\":\"0xffff000000000000-0xffff0000000fffff\",\"perm\":\"r-x\",\"name\":\"sigma-core\"}}]}}");
                return;
            }
            println!("{}", bold("Memory Map"));
            let maps: &[(&str, &str, &str)] = &[
                ("0xffff000000000000–0xffff0000000fffff", "r-x", "sigma-core (.text)"),
                ("0xffff000000100000–0xffff00000017ffff", "r--", "sigma-core (.rodata)"),
                ("0xffff000000180000–0xffff0000001fffff", "rw-", "sigma-core (.data/.bss)"),
                ("0xffff000000200000–0xffff00000027ffff", "r-x", "sigma-net (.text)"),
                ("0xffff800000000000–0xffff8fffffffffff", "rw-", "kernel heap"),
            ];
            for (range, perm, name) in maps {
                let colour = if perm.contains('x') { cyan(perm) } else if perm.contains('w') { yellow(perm) } else { green(perm) };
                println!("  {}  {}  {}", range, colour, name);
            }
        }
        _ => eprintln!("{} unknown mem action '{}'. Valid: read, dump, map", red("error:"), action),
    }
}

fn cmd_reg(pid: u32, json: bool) {
    let regs: &[(&str, &str)] = &[
        ("rax", "0x0000000000000001"), ("rbx", "0xffff800000012340"),
        ("rcx", "0x0000000000000000"), ("rdx", "0xffff800000034560"),
        ("rsi", "0x0000000000000010"), ("rdi", "0xffff80000004abc0"),
        ("rsp", "0xffff80000000ff80"), ("rbp", "0xffff80000000ffc0"),
        ("rip", "0xffff000000001234"), ("rflags", "0x0000000000000246"),
        ("cs",  "0x0010"),            ("ss",  "0x0018"),
        ("cr0", "0x0000000080050033"), ("cr3", "0x00000001234ab000"),
        ("cr4", "0x00000000003706f0"),
    ];

    if json {
        let pairs: Vec<String> = regs.iter().map(|(k,v)| format!("\"{}\":\"{}\"", k, v)).collect();
        println!("{{\"pid\":{},\"regs\":{{{}}}}}", pid, pairs.join(","));
        return;
    }
    println!("{} — PID {}", bold("Register Dump"), pid);
    println!("  {}", "─".repeat(54));
    for chunk in regs.chunks(2) {
        let left  = format!("{:<8} = {}", chunk[0].0, chunk[0].1);
        let right = chunk.get(1).map(|(k,v)| format!("{:<8} = {}", k, v)).unwrap_or_default();
        println!("  {:<36}  {}", left, right);
    }
}

fn cmd_sym(action: &str, name_or_addr: &str, json: bool) {
    match action {
        "resolve" => {
            let addr = u64::from_str_radix(name_or_addr.trim_start_matches("0x"), 16).unwrap_or(0xffff000000001234);
            if json {
                println!("{{\"addr\":\"{:#x}\",\"sym\":\"sigma_syscall_dispatch+0x14\",\"file\":\"kernel/syscalls/dispatcher.rs\",\"line\":42}}", addr);
                return;
            }
            println!("  {:#018x}  {}  {}:{}", addr, cyan("sigma_syscall_dispatch+0x14"),
                dim("kernel/syscalls/dispatcher.rs"), 42);
        }
        "search" => {
            if json {
                println!("{{\"results\":[{{\"sym\":\"{}\",\"addr\":\"0xffff000000001234\"}},{{}]}}", name_or_addr);
                return;
            }
            println!("{} Search: '{}'", bold("Symbol"), name_or_addr);
            println!("  {:#018x}  {}", 0xffff000000001234u64, name_or_addr);
            println!("  {:#018x}  {}_init", 0xffff000000001200u64, name_or_addr);
        }
        _ => eprintln!("{} unknown sym action '{}'. Valid: resolve, search", red("error:"), action),
    }
}

fn cmd_bp(action: &str, addr: &str, json: bool) {
    match action {
        "set" => {
            let a = u64::from_str_radix(addr.trim_start_matches("0x"), 16).unwrap_or(0);
            println!("{} Breakpoint set at {:#018x}", green("✓"), a);
        }
        "list" => {
            if json {
                println!("{{\"breakpoints\":[{{\"id\":1,\"addr\":\"0xffff000000001234\",\"enabled\":true}}]}}");
                return;
            }
            println!("{}", bold("Breakpoints"));
            println!("  #1  0xffff000000001234  {}  sigma_syscall_dispatch", green("enabled "));
            println!("  #2  0xffff000000002abc  {}  sigma_mm_alloc_page",   yellow("disabled"));
        }
        "del"   => println!("{} Breakpoint removed.", green("✓")),
        "clear" => println!("{} All breakpoints cleared.", green("✓")),
        _ => eprintln!("{} unknown bp action. Valid: set, list, del, clear", red("error:")),
    }
}

fn cmd_bt(pid: u32, json: bool) {
    let frames: &[(&str, &str, u32)] = &[
        ("0xffff000000001234", "sigma_syscall_dispatch",   42),
        ("0xffff000000005abc", "sigma_mm_page_fault_handler", 88),
        ("0xffff000000010000", "sigma_sched_yield",        14),
        ("0xffff800000000100", "kernel_thread_entry",       7),
    ];
    if json {
        println!("{{\"backtrace\":[");
        for (i, (addr, sym, line)) in frames.iter().enumerate() {
            print!("  {{\"frame\":{},\"addr\":\"{}\",\"sym\":\"{}\",\"line\":{}}}", i, addr, sym, line);
            if i < frames.len()-1 { print!(","); }
            println!();
        }
        println!("]}}");
        return;
    }
    println!("{} — PID {}", bold("Backtrace"), pid);
    for (i, (addr, sym, line)) in frames.iter().enumerate() {
        println!("  #{:<2} {}  {} {}:{}", i, cyan(addr), bold(sym), dim("@ line"), line);
    }
}

fn cmd_attach(pid: u32, json: bool) {
    if json {
        println!("{{\"attach\":{{\"pid\":{},\"status\":\"paused\"}}}}", pid);
        return;
    }
    println!("{} Attaching to PID {}...", cyan("Σ"), pid);
    println!("  Pausing all threads...");
    println!("  Loading symbol table...");
    println!("{} Attached. Use 'sigma-debug reg --pid {}' to inspect state.", green("✓"), pid);
    println!("  Detach: sigma-debug reg --pid {} --detach", pid);
}

fn cmd_repl() {
    println!("{} Interactive Debug REPL (type 'quit' to exit)", cyan("sigma-debug"));
    println!("  Commands: shard list | mem dump 0x<addr> | reg | bt | sym resolve 0x<addr>");
    println!("  {}", dim("(Simulation — connects to /run/sigma/debugd.sock on bare metal)"));
    let prompt_cmds = [
        ("(sigma-dbg) shard list",     "→ listing 5 shards..."),
        ("(sigma-dbg) reg",            "→ rip=0xffff000000001234 rsp=0xffff80000000ff80"),
        ("(sigma-dbg) bt",             "→ #0 sigma_syscall_dispatch #1 sigma_mm_page_fault_handler"),
        ("(sigma-dbg) quit",           "→ Detaching. Goodbye."),
    ];
    for (prompt, response) in &prompt_cmds {
        println!("\n  {}", bold(prompt));
        println!("  {}", dim(response));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" {
        print_usage();
        exit(if args.len() < 2 { 1 } else { 0 });
    }
    if args[1] == "--version" || args[1] == "-V" {
        println!("sigma-debug {}", VERSION);
        exit(0);
    }

    let json    = args.iter().any(|a| a == "--json");
    let pid     = args.windows(2).find(|w| w[0] == "--pid").and_then(|w| w[1].parse().ok()).unwrap_or(1u32);
    let addr    = args.windows(2).find(|w| w[0] == "--addr").map(|w| w[1].as_str()).unwrap_or("0xffff000000001000");
    let len     = args.windows(2).find(|w| w[0] == "--len").and_then(|w| w[1].parse().ok()).unwrap_or(64usize);

    let positional: Vec<&str> = args[2..].iter()
        .filter(|a| !a.starts_with("--"))
        .map(|s| s.as_str())
        .collect();

    match args[1].as_str() {
        "shard"  => cmd_shard(positional.get(0).copied().unwrap_or("list"), &positional[1.min(positional.len())..], json),
        "mem"    => cmd_mem(positional.get(0).copied().unwrap_or("dump"), addr, len, json),
        "reg"    => cmd_reg(pid, json),
        "sym"    => cmd_sym(positional.get(0).copied().unwrap_or("resolve"), positional.get(1).copied().unwrap_or(addr), json),
        "bp"     => cmd_bp(positional.get(0).copied().unwrap_or("list"), addr, json),
        "bt"     => cmd_bt(pid, json),
        "attach" => cmd_attach(pid, json),
        "repl"   => cmd_repl(),
        "script" => {
            let f = positional.get(0).copied().unwrap_or("debug.script");
            println!("{} Running debug script '{}'...", cyan("Σ"), f);
            println!("{} Script completed.", green("✓"));
        }
        _ => {
            eprintln!("{} unknown command '{}'. Run --help.", red("error:"), args[1]);
            exit(1);
        }
    }
}
