// SPDX-License-Identifier: GPL-2.0-or-later
//! sigma-hypervisor — SigmaOS hypervisor & VM management CLI
//!
//! Usage:
//!   sigma-hypervisor <list|create|start|stop|destroy|console|snapshot|info> [options]

use std::env;
use std::process::exit;

const VERSION: &str = "1.0.0";

fn cyan(s: &str)  -> String { format!("\x1B[1;36m{}\x1B[0m", s) }
fn green(s: &str) -> String { format!("\x1B[1;32m{}\x1B[0m", s) }
fn red(s: &str)   -> String { format!("\x1B[1;31m{}\x1B[0m", s) }
fn yellow(s: &str)-> String { format!("\x1B[1;33m{}\x1B[0m", s) }
fn bold(s: &str)  -> String { format!("\x1B[1m{}\x1B[0m", s) }

fn print_usage() {
    println!("{} v{}", cyan("sigma-hypervisor"), VERSION);
    println!();
    println!("{}  sigma-hypervisor <command> [options]", bold("USAGE:"));
    println!();
    println!("{}", bold("COMMANDS:"));
    println!("  list                          List all virtual machines");
    println!("  create  --name <n> [opts]     Create a new VM");
    println!("  start   --name <n>            Start a VM");
    println!("  stop    --name <n> [--force]  Stop a VM gracefully (or force)");
    println!("  destroy --name <n> [--force]  Permanently delete a VM");
    println!("  console --name <n>            Attach to VM serial console");
    println!("  snapshot --name <n> [--label] Take a VM checkpoint snapshot");
    println!("  info    --name <n>            Detailed VM information");
    println!();
    println!("{}", bold("CREATE OPTIONS:"));
    println!("  --name    <n>      VM name (required)");
    println!("  --mem     <MiB>    RAM in MiB (default: 512)");
    println!("  --cpus    <n>      vCPU count (default: 1)");
    println!("  --disk    <GiB>    Disk size in GiB (default: 8)");
    println!("  --image   <file>   Boot image (.iso or .img)");
    println!("  --arch    <arch>   Architecture (x86_64|aarch64|riscv64, default: x86_64)");
    println!("  --net     <mode>   Network mode (nat|bridge|none, default: nat)");
    println!();
    println!("{}", bold("GLOBAL OPTIONS:"));
    println!("  --json             Machine-readable JSON output");
    println!("  --version, -V      Print version");
    println!("  --help,    -h      Show this help");
}

struct VM {
    name:   &'static str,
    status: &'static str,
    arch:   &'static str,
    mem:    u32,
    cpus:   u8,
    disk:   u32,
    uptime: &'static str,
}

fn sample_vms() -> Vec<VM> {
    vec![
        VM { name: "sigma-dev",    status: "Running", arch: "x86_64",  mem: 2048, cpus: 2, disk: 20, uptime: "2h 15m" },
        VM { name: "sigma-test",   status: "Stopped", arch: "x86_64",  mem: 512,  cpus: 1, disk:  8, uptime: "—"      },
        VM { name: "sigma-arm64",  status: "Running", arch: "aarch64", mem: 1024, cpus: 2, disk: 16, uptime: "45m"    },
        VM { name: "sigma-riscv",  status: "Paused",  arch: "riscv64", mem: 512,  cpus: 1, disk:  8, uptime: "—"      },
    ]
}

fn cmd_list(json: bool) {
    let vms = sample_vms();
    if json {
        println!("{{\"vms\":[");
        for (i, v) in vms.iter().enumerate() {
            print!("  {{\"name\":\"{}\",\"status\":\"{}\",\"arch\":\"{}\",\"mem_mib\":{},\"cpus\":{},\"disk_gib\":{}}}",
                v.name, v.status, v.arch, v.mem, v.cpus, v.disk);
            if i < vms.len() - 1 { print!(","); }
            println!();
        }
        println!("]}}");
        return;
    }
    println!("{}", bold("Virtual Machines"));
    println!("  {:<18}  {:<10}  {:<8}  {:>7}  {:>5}  {:>7}  {}",
        "Name", "Status", "Arch", "RAM MiB", "vCPUs", "Disk GB", "Uptime");
    println!("  {}", "─".repeat(72));
    for v in &vms {
        let status_str = match v.status {
            "Running" => green(v.status),
            "Stopped" => red(v.status),
            _         => yellow(v.status),
        };
        println!("  {:<18}  {:<20}  {:<8}  {:>7}  {:>5}  {:>7}  {}",
            v.name, status_str, v.arch, v.mem, v.cpus, v.disk, v.uptime);
    }
    let running = vms.iter().filter(|v| v.status == "Running").count();
    println!("\n  {}/{} VMs running", running, vms.len());
}

fn cmd_create(name: &str, mem: u32, cpus: u8, disk: u32, arch: &str, net: &str, image: Option<&str>, json: bool) {
    if json {
        println!("{{\"create\":{{\"name\":\"{}\",\"mem\":{},\"cpus\":{},\"disk\":{},\"arch\":\"{}\",\"status\":\"created\"}}}}",
            name, mem, cpus, disk, arch);
        return;
    }
    println!("{} Creating VM '{}'...", cyan("Σ"), name);
    println!("  Architecture : {}", arch);
    println!("  RAM          : {} MiB", mem);
    println!("  vCPUs        : {}", cpus);
    println!("  Disk         : {} GiB", disk);
    println!("  Network      : {}", net);
    if let Some(img) = image { println!("  Boot image   : {}", img); }
    println!("  Allocating disk image...");
    println!("  Configuring virtual NIC...");
    println!("{} VM '{}' created. Start with: sigma-hypervisor start --name {}", green("✓"), name, name);
}

fn cmd_start(name: &str, json: bool) {
    if json {
        println!("{{\"start\":{{\"name\":\"{}\",\"status\":\"running\"}}}}", name);
        return;
    }
    println!("{} Starting VM '{}'...", cyan("Σ"), name);
    println!("  Loading kernel image...");
    println!("  Attaching virtual devices...");
    println!("{} VM '{}' is now running.", green("✓"), name);
    println!("  Console: sigma-hypervisor console --name {}", name);
}

fn cmd_stop(name: &str, force: bool, json: bool) {
    if json {
        println!("{{\"stop\":{{\"name\":\"{}\",\"force\":{},\"status\":\"stopped\"}}}}", name, force);
        return;
    }
    if force {
        println!("{} Force-stopping VM '{}'...", yellow("⚠"), name);
        println!("{} VM '{}' terminated.", green("✓"), name);
    } else {
        println!("{} Sending ACPI shutdown to VM '{}'...", cyan("Σ"), name);
        println!("{} VM '{}' stopped gracefully.", green("✓"), name);
    }
}

fn cmd_destroy(name: &str, force: bool, json: bool) {
    if json {
        println!("{{\"destroy\":{{\"name\":\"{}\",\"status\":\"destroyed\"}}}}", name);
        return;
    }
    if !force {
        println!("{} VM '{}' is not stopped. Use --force or stop it first.", red("error:"), name);
        return;
    }
    println!("{} Destroying VM '{}' — this is irreversible.", yellow("⚠"), name);
    println!("  Removing disk image...");
    println!("  Releasing network allocation...");
    println!("{} VM '{}' destroyed.", green("✓"), name);
}

fn cmd_console(name: &str, json: bool) {
    if json {
        println!("{{\"console\":{{\"name\":\"{}\",\"socket\":\"/run/sigma/vms/{}.sock\"}}}}", name, name);
        return;
    }
    println!("{} Attaching to serial console of VM '{}'...", cyan("Σ"), name);
    println!("  (Press Ctrl+] to detach)");
    println!("  Socket: /run/sigma/vms/{}.sock", name);
    println!("  [Simulation — socat UNIX-CONNECT:/run/sigma/vms/{}.sock -,raw,echo=0]", name);
}

fn cmd_snapshot(name: &str, label: Option<&str>, json: bool) {
    let lbl = label.unwrap_or("checkpoint");
    if json {
        println!("{{\"snapshot\":{{\"vm\":\"{}\",\"label\":\"{}\",\"status\":\"ok\"}}}}", name, lbl);
        return;
    }
    println!("{} Snapshotting VM '{}' (label: {})...", cyan("Σ"), name, lbl);
    println!("  Freezing VM memory...");
    println!("  Writing checkpoint to /var/sigma/vms/{}/{}.snap...", name, lbl);
    println!("{} Snapshot '{}' saved.", green("✓"), lbl);
}

fn cmd_info(name: &str, json: bool) {
    if json {
        println!("{{\"vm\":{{\"name\":\"{}\",\"status\":\"Running\",\"arch\":\"x86_64\",\"mem\":2048,\"cpus\":2,\"disk\":20,\"uptime\":\"2h 15m\"}}}}", name);
        return;
    }
    println!("{} — {}", bold("VM Info"), cyan(name));
    println!("  Status       : {}", green("Running"));
    println!("  Architecture : x86_64");
    println!("  RAM          : 2048 MiB");
    println!("  vCPUs        : 2");
    println!("  Disk         : 20 GiB (/var/sigma/vms/{}/disk.img)", name);
    println!("  Network      : nat (virtio, 10.0.0.101)");
    println!("  Uptime       : 2h 15m");
    println!("  Console      : /run/sigma/vms/{}.sock", name);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" {
        print_usage();
        exit(if args.len() < 2 { 1 } else { 0 });
    }
    if args[1] == "--version" || args[1] == "-V" {
        println!("sigma-hypervisor {}", VERSION);
        exit(0);
    }

    let json  = args.iter().any(|a| a == "--json");
    let force = args.iter().any(|a| a == "--force");
    let name  = args.windows(2).find(|w| w[0] == "--name").map(|w| w[1].as_str()).unwrap_or("sigma-vm");
    let mem   = args.windows(2).find(|w| w[0] == "--mem").and_then(|w| w[1].parse().ok()).unwrap_or(512u32);
    let cpus  = args.windows(2).find(|w| w[0] == "--cpus").and_then(|w| w[1].parse().ok()).unwrap_or(1u8);
    let disk  = args.windows(2).find(|w| w[0] == "--disk").and_then(|w| w[1].parse().ok()).unwrap_or(8u32);
    let arch  = args.windows(2).find(|w| w[0] == "--arch").map(|w| w[1].as_str()).unwrap_or("x86_64");
    let net   = args.windows(2).find(|w| w[0] == "--net").map(|w| w[1].as_str()).unwrap_or("nat");
    let image = args.windows(2).find(|w| w[0] == "--image").map(|w| w[1].as_str());
    let label = args.windows(2).find(|w| w[0] == "--label").map(|w| w[1].as_str());

    match args[1].as_str() {
        "list"     => cmd_list(json),
        "create"   => cmd_create(name, mem, cpus, disk, arch, net, image, json),
        "start"    => cmd_start(name, json),
        "stop"     => cmd_stop(name, force, json),
        "destroy"  => cmd_destroy(name, force, json),
        "console"  => cmd_console(name, json),
        "snapshot" => cmd_snapshot(name, label, json),
        "info"     => cmd_info(name, json),
        _ => {
            eprintln!("{} unknown command '{}'. Run --help.", red("error:"), args[1]);
            exit(1);
        }
    }
}
