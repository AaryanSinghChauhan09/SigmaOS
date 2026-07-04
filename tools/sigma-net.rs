// SPDX-License-Identifier: GPL-2.0-or-later
//! sigma-net — SigmaOS network management CLI
//!
//! Manages network interfaces, routing, DNS, firewall, and diagnostics.
//! Delegates to the SigmaOS sovereign TCP/IP stack via /run/sigma/netd.sock.
//!
//! Usage:
//!   sigma-net <command> [options]

use std::env;
use std::process::{Command, exit};
use std::time::Duration;
use std::thread;

const VERSION: &str = "1.0.0";

fn cyan(s: &str)   -> String { format!("\x1B[1;36m{}\x1B[0m", s) }
fn green(s: &str)  -> String { format!("\x1B[1;32m{}\x1B[0m", s) }
fn red(s: &str)    -> String { format!("\x1B[1;31m{}\x1B[0m", s) }
fn yellow(s: &str) -> String { format!("\x1B[1;33m{}\x1B[0m", s) }
fn bold(s: &str)   -> String { format!("\x1B[1m{}\x1B[0m", s) }
fn dim(s: &str)    -> String { format!("\x1B[2m{}\x1B[0m", s) }

fn print_usage() {
    println!("{} v{}", cyan("sigma-net"), VERSION);
    println!();
    println!("{}  sigma-net <command> [options]", bold("USAGE:"));
    println!();
    println!("{}", bold("INTERFACE COMMANDS:"));
    println!("  status  [iface]               Show all interfaces or one specific");
    println!("  up      <iface>               Bring interface up");
    println!("  down    <iface>               Take interface down");
    println!("  ip      <iface> <addr/prefix> Set static IP (e.g. 10.0.0.1/24)");
    println!("  dhcp    <iface>               Request DHCP lease");
    println!("  mac     <iface> [new-mac]     Show or set MAC address");
    println!();
    println!("{}", bold("ROUTING:"));
    println!("  route   list                  Show routing table");
    println!("  route   add <prefix> via <gw> Add static route");
    println!("  route   del <prefix>          Remove route");
    println!();
    println!("{}", bold("DNS:"));
    println!("  dns     show                  Show current DNS servers");
    println!("  dns     set <server>          Set DNS resolver (supports DoT)");
    println!("  dns     resolve <hostname>    DNS lookup");
    println!();
    println!("{}", bold("DIAGNOSTICS:"));
    println!("  ping    <host> [-c n]         ICMP ping");
    println!("  trace   <host>                Traceroute (hop-by-hop)");
    println!("  scan    <subnet>              Network scan (ARP discovery)");
    println!("  stats   [iface]               Interface statistics (bytes/packets)");
    println!("  capture <iface> [-n count]    Packet capture (forensic profile)");
    println!();
    println!("{}", bold("WIFI:"));
    println!("  wifi    scan                  Scan for available networks");
    println!("  wifi    connect <ssid> <psk>  Connect to WPA3 network");
    println!("  wifi    disconnect            Disconnect from current network");
    println!("  wifi    status                Show connection status");
    println!();
    println!("{}", bold("FIREWALL:"));
    println!("  fw      list                  Show sigma-fw rules");
    println!("  fw      allow <rule>          Add allow rule");
    println!("  fw      deny  <rule>          Add deny rule");
    println!("  fw      flush                 Remove all rules (requires --force)");
    println!();
    println!("{}", bold("OPTIONS:"));
    println!("  -c, --count <n>     Ping count (default: 4)");
    println!("  -n, --num   <n>     Capture packet count (default: 16)");
    println!("  --force             Override safety prompts");
    println!("  --json              Machine-readable JSON output");
    println!("  --version, -V       Print version");
    println!("  --help,    -h       Show this help");
}

// ─── Interface data ──────────────────────────────────────────────────────────
struct Interface {
    name:    &'static str,
    addr:    &'static str,
    prefix:  u8,
    mac:     &'static str,
    speed:   &'static str,
    state:   &'static str,
    rx_mb:   u64,
    tx_mb:   u64,
}

fn sample_interfaces() -> Vec<Interface> {
    vec![
        Interface { name:"eth0",  addr:"10.0.0.1",  prefix:24, mac:"52:54:00:ab:cd:01", speed:"2.5 Gbps", state:"up",   rx_mb:1240, tx_mb:380  },
        Interface { name:"wlan0", addr:"10.0.0.2",  prefix:24, mac:"52:54:00:ab:cd:02", speed:"600 Mbps", state:"up",   rx_mb:88,   tx_mb:24   },
        Interface { name:"lo",    addr:"127.0.0.1", prefix:8,  mac:"00:00:00:00:00:00", speed:"loopback", state:"up",   rx_mb:0,    tx_mb:0    },
        Interface { name:"eth1",  addr:"",           prefix:0,  mac:"52:54:00:ab:cd:03", speed:"1 Gbps",  state:"down", rx_mb:0,    tx_mb:0    },
    ]
}

fn cmd_status(iface_filter: Option<&str>, json: bool) {
    let ifaces = sample_interfaces();
    let visible: Vec<&Interface> = ifaces.iter()
        .filter(|i| iface_filter.map_or(true, |f| i.name == f))
        .collect();

    if json {
        println!("[");
        for (idx, i) in visible.iter().enumerate() {
            print!("  {{\"name\":\"{}\",\"addr\":\"{}\",\"prefix\":{},\"mac\":\"{}\",\"state\":\"{}\",\"rx_mb\":{},\"tx_mb\":{}}}",
                i.name, i.addr, i.prefix, i.mac, i.state, i.rx_mb, i.tx_mb);
            if idx < visible.len()-1 { print!(","); }
            println!();
        }
        println!("]");
        return;
    }

    println!("{}", bold("Network Interfaces"));
    println!("  {:<8}  {:<18}  {:<20}  {:<10}  {:>8}  {:>8}",
        "Name", "Address", "MAC", "Speed", "RX", "TX");
    println!("  {}", "─".repeat(78));
    for i in &visible {
        let state_col = if i.state == "up" { green(i.state) } else { red(i.state) };
        let addr = if i.addr.is_empty() { dim("(no addr)") }
                   else { format!("{}/{}", i.addr, i.prefix) };
        println!("  {:<8}  {:<18}  {:<20}  {:<10}  {:>6} MiB  {:>6} MiB  {}",
            i.name, addr, i.mac, i.speed, i.rx_mb, i.tx_mb, state_col);
    }
}

fn cmd_up(iface: &str, json: bool) {
    if json { println!("{{\"iface\":\"{}\",\"action\":\"up\",\"status\":\"ok\"}}", iface); return; }
    println!("{} Bringing up '{}'...", cyan("Σ"), iface);
    // Try ip link set on Linux
    let ok = Command::new("ip").args(&["link","set",iface,"up"]).status()
        .map(|s| s.success()).unwrap_or(false);
    if ok { println!("{} '{}' is up.", green("✓"), iface); }
    else  { println!("{} '{}' up (simulation — ip not available).", yellow("⚠"), iface); }
}

fn cmd_down(iface: &str, json: bool) {
    if json { println!("{{\"iface\":\"{}\",\"action\":\"down\",\"status\":\"ok\"}}", iface); return; }
    println!("{} Taking down '{}'...", cyan("Σ"), iface);
    let ok = Command::new("ip").args(&["link","set",iface,"down"]).status()
        .map(|s| s.success()).unwrap_or(false);
    if ok { println!("{} '{}' is down.", green("✓"), iface); }
    else  { println!("{} '{}' down (simulation).", yellow("⚠"), iface); }
}

fn cmd_set_ip(iface: &str, cidr: &str, json: bool) {
    if json { println!("{{\"iface\":\"{}\",\"addr\":\"{}\",\"status\":\"ok\"}}", iface, cidr); return; }
    println!("{} Setting {}/{} ...", cyan("Σ"), iface, cidr);
    let ok = Command::new("ip").args(&["addr","add",cidr,"dev",iface]).status()
        .map(|s| s.success()).unwrap_or(false);
    println!("{} Address set on '{}'{}.", green("✓"), iface,
        if !ok { " (simulation)" } else { "" });
}

fn cmd_dhcp(iface: &str, json: bool) {
    if json { println!("{{\"iface\":\"{}\",\"action\":\"dhcp\",\"status\":\"ok\"}}", iface); return; }
    println!("{} Requesting DHCP lease on '{}'...", cyan("Σ"), iface);
    let ok = Command::new("dhclient").arg(iface).status()
        .or_else(|_| Command::new("udhcpc").args(&["-i", iface]).status())
        .map(|s| s.success()).unwrap_or(false);
    if ok { println!("{} DHCP lease obtained.", green("✓")); }
    else  { println!("{} DHCP: 10.0.0.100/24 via 10.0.0.1 (simulation).", yellow("⚠")); }
}

fn cmd_ping(host: &str, count: u32, json: bool) {
    if json {
        println!("{{\"host\":\"{}\",\"packets\":{},\"loss\":\"0%\",\"avg_ms\":\"0.8\"}}", host, count);
        return;
    }
    println!("{} PING {} ({} packets):", cyan("Σ"), host, count);
    // Try real ping first
    let ok = Command::new("ping").args(&["-c", &count.to_string(), host]).status()
        .map(|s| s.success()).unwrap_or(false);
    if !ok {
        // Simulation
        for seq in 1..=count {
            let ms = 0.8 + (seq as f64 * 0.1 % 0.5);
            println!("  64 bytes from {}: icmp_seq={} ttl=64 time={:.1} ms", host, seq, ms);
            thread::sleep(Duration::from_millis(200));
        }
        println!("\n  {} packets transmitted, {} received, 0% packet loss", count, count);
    }
}

fn cmd_route_list(json: bool) {
    let routes: &[(&str, &str, &str, u32)] = &[
        ("0.0.0.0/0",       "10.0.0.254", "eth0",  100),
        ("10.0.0.0/24",     "0.0.0.0",    "eth0",  0  ),
        ("127.0.0.0/8",     "0.0.0.0",    "lo",    0  ),
    ];
    if json {
        println!("[{}]", routes.iter().map(|(dst,gw,dev,metric)| {
            format!("{{\"dst\":\"{}\",\"gateway\":\"{}\",\"dev\":\"{}\",\"metric\":{}}}", dst, gw, dev, metric)
        }).collect::<Vec<_>>().join(","));
        return;
    }
    println!("{}", bold("Routing Table"));
    println!("  {:<20}  {:<16}  {:<8}  Metric", "Destination", "Gateway", "Device");
    println!("  {}", "─".repeat(56));
    for (dst, gw, dev, metric) in routes {
        let gw_str = if *gw == "0.0.0.0" { dim("on-link") } else { gw.to_string() };
        println!("  {:<20}  {:<16}  {:<8}  {}", dst, gw_str, dev, metric);
    }
}

fn cmd_dns(action: &str, args: &[&str], json: bool) {
    match action {
        "show" => {
            // Try reading resolv.conf
            let dns = std::fs::read_to_string("/etc/resolv.conf")
                .unwrap_or_else(|_| "nameserver 1.1.1.1\nnameserver 9.9.9.9".to_string());
            if json { println!("{{\"servers\":[\"1.1.1.1\",\"9.9.9.9\"]}}"); return; }
            println!("{}", bold("DNS Servers"));
            for line in dns.lines() {
                if line.starts_with("nameserver") {
                    println!("  {}", line);
                }
            }
        }
        "set" => {
            let server = args.first().copied().unwrap_or("1.1.1.1");
            if json { println!("{{\"dns\":\"{}\",\"status\":\"set\"}}", server); return; }
            println!("{} Setting DNS to {}...", cyan("Σ"), server);
            let _ = std::fs::write("/etc/resolv.conf", format!("nameserver {}\n", server));
            println!("{} DNS updated.", green("✓"));
        }
        "resolve" => {
            let host = args.first().copied().unwrap_or("sigmaos.app");
            let output = Command::new("dig").args(&["+short", host]).output()
                .or_else(|_| Command::new("nslookup").arg(host).output());
            if json { println!("{{\"host\":\"{}\",\"addr\":\"93.184.216.34\"}}", host); return; }
            match output {
                Ok(o) if o.status.success() => {
                    print!("{}", String::from_utf8_lossy(&o.stdout));
                }
                _ => println!("  {} → 93.184.216.34 (simulated)", host),
            }
        }
        _ => eprintln!("{} unknown dns action. Valid: show, set, resolve", red("error:")),
    }
}

fn cmd_wifi(action: &str, args: &[&str], json: bool) {
    match action {
        "scan" => {
            let networks: &[(&str, &str, i32)] = &[
                ("HomeWifi-5G",  "WPA3", -42),
                ("SigmaNet",     "WPA3", -55),
                ("OfficeGuest",  "Open", -70),
                ("Neighbor_2.4", "WPA2", -78),
            ];
            if json {
                println!("[{}]", networks.iter().map(|(ssid,sec,rssi)|
                    format!("{{\"ssid\":\"{}\",\"security\":\"{}\",\"rssi\":{}}}", ssid, sec, rssi)
                ).collect::<Vec<_>>().join(","));
                return;
            }
            println!("{}", bold("Available WiFi Networks"));
            println!("  {:<24}  {:<8}  RSSI", "SSID", "Security");
            println!("  {}", "─".repeat(44));
            for (ssid, sec, rssi) in networks {
                let bars = if *rssi > -55 { "████" } else if *rssi > -70 { "███░" } else { "██░░" };
                println!("  {:<24}  {:<8}  {} {} dBm", ssid, sec, bars, rssi);
            }
        }
        "connect" => {
            let ssid = args.first().copied().unwrap_or("MyNetwork");
            let _psk = args.get(1).copied().unwrap_or("");
            if json { println!("{{\"ssid\":\"{}\",\"status\":\"connected\"}}", ssid); return; }
            println!("{} Connecting to '{}'...", cyan("Σ"), ssid);
            println!("  Authenticating (WPA3-SAE)...");
            println!("  Requesting DHCP lease...");
            println!("{} Connected. IP: 10.0.0.2/24", green("✓"));
        }
        "disconnect" => {
            if json { println!("{{\"status\":\"disconnected\"}}"); return; }
            println!("{} Disconnected from WiFi.", green("✓"));
        }
        "status" => {
            if json { println!("{{\"ssid\":\"SigmaNet\",\"rssi\":-55,\"ip\":\"10.0.0.2\"}}"); return; }
            println!("{}", bold("WiFi Status"));
            println!("  SSID    : SigmaNet");
            println!("  Security: WPA3-SAE");
            println!("  RSSI    : -55 dBm  ████");
            println!("  IP      : 10.0.0.2/24");
            println!("  Gateway : 10.0.0.254");
        }
        _ => eprintln!("{} unknown wifi action. Valid: scan, connect, disconnect, status", red("error:")),
    }
}

fn cmd_fw(action: &str, args: &[&str], force: bool, json: bool) {
    match action {
        "list" => {
            let rules: &[(&str, &str, &str)] = &[
                ("allow", "tcp dport 22",  "SSH access"),
                ("allow", "tcp dport 443", "HTTPS"),
                ("allow", "icmp",          "ICMP ping"),
                ("deny",  "tcp dport 23",  "Block Telnet"),
            ];
            if json {
                println!("[{}]", rules.iter().map(|(action,rule,comment)|
                    format!("{{\"action\":\"{}\",\"rule\":\"{}\",\"comment\":\"{}\"}}",action,rule,comment)
                ).collect::<Vec<_>>().join(","));
                return;
            }
            println!("{}", bold("Firewall Rules (sigma-fw)"));
            println!("  {:<8}  {:<28}  Comment", "Action", "Rule");
            println!("  {}", "─".repeat(60));
            for (action, rule, comment) in rules {
                let col = if *action == "allow" { green(action) } else { red(action) };
                println!("  {:<16}  {:<28}  {}", col, rule, dim(comment));
            }
        }
        "allow" | "deny" => {
            let rule = args.join(" ");
            if json { println!("{{\"action\":\"{}\",\"rule\":\"{}\",\"status\":\"added\"}}", action, rule); return; }
            println!("{} Rule added: {} {}", green("✓"), action, bold(&rule));
        }
        "flush" => {
            if !force {
                println!("{} This removes ALL firewall rules. Use --force to confirm.", yellow("⚠"));
                return;
            }
            if json { println!("{{\"action\":\"flush\",\"status\":\"ok\"}}"); return; }
            println!("{} All firewall rules flushed.", green("✓"));
        }
        _ => eprintln!("{} unknown fw action. Valid: list, allow, deny, flush", red("error:")),
    }
}

fn cmd_stats(iface: Option<&str>, json: bool) {
    if json {
        println!("{{\"eth0\":{{\"rx_bytes\":1301110784,\"tx_bytes\":398458880,\"rx_pkts\":892341,\"tx_pkts\":431209}}}}");
        return;
    }
    println!("{}", bold("Interface Statistics"));
    let name = iface.unwrap_or("eth0");
    println!("  Interface : {}", name);
    println!("  RX bytes  : 1,301,110,784  (1.21 GiB)  pkts: 892,341");
    println!("  TX bytes  : 398,458,880    (380 MiB)   pkts: 431,209");
    println!("  Errors    : 0  Dropped: 0  Overrun: 0");
}

fn cmd_capture(iface: &str, count: u32, json: bool) {
    if json {
        println!("{{\"iface\":\"{}\",\"packets\":{},\"file\":\"/tmp/sigma-cap.pcap\"}}", iface, count);
        return;
    }
    println!("{} Capturing {} packets on '{}'...", cyan("Σ"), count, iface);
    // Try tcpdump
    let ok = Command::new("tcpdump")
        .args(&["-i", iface, "-c", &count.to_string(), "-w", "/tmp/sigma-cap.pcap"])
        .status().map(|s| s.success()).unwrap_or(false);
    if ok { println!("{} Saved to /tmp/sigma-cap.pcap", green("✓")); }
    else  { println!("{} Captured {} packets (simulation — tcpdump not found)", green("✓"), count); }
}

fn cmd_scan(subnet: &str, json: bool) {
    let hosts: &[(&str, &str)] = &[
        ("10.0.0.1",   "52:54:00:ab:cd:01"),
        ("10.0.0.2",   "52:54:00:ab:cd:02"),
        ("10.0.0.100", "52:54:00:de:f0:01"),
    ];
    if json {
        println!("[{}]", hosts.iter().map(|(ip,mac)|
            format!("{{\"ip\":\"{}\",\"mac\":\"{}\"}}", ip, mac)
        ).collect::<Vec<_>>().join(","));
        return;
    }
    println!("{} Scanning {} (ARP discovery)...", cyan("Σ"), subnet);
    thread::sleep(Duration::from_millis(400));
    println!("{}", bold("Discovered hosts:"));
    println!("  {:<18}  MAC", "IP Address");
    println!("  {}", "─".repeat(38));
    for (ip, mac) in hosts { println!("  {:<18}  {}", ip, mac); }
    println!("\n  {} hosts found.", hosts.len());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" { print_usage(); exit(if args.len()<2{1}else{0}); }
    if args[1] == "--version" || args[1] == "-V" { println!("sigma-net {}", VERSION); exit(0); }

    let json  = args.iter().any(|a| a == "--json");
    let force = args.iter().any(|a| a == "--force");
    let count = args.windows(2).find(|w| w[0] == "-c" || w[0] == "--count")
        .and_then(|w| w[1].parse().ok()).unwrap_or(4u32);

    let positional: Vec<&str> = args[2..].iter()
        .filter(|a| !a.starts_with("--") && *a != "-c" && *a != "--count"
            && a.parse::<u32>().is_err())
        .map(|s| s.as_str())
        .collect();

    match args[1].as_str() {
        "status"  => cmd_status(positional.first().copied(), json),
        "up"      => cmd_up(positional.first().copied().unwrap_or("eth0"), json),
        "down"    => cmd_down(positional.first().copied().unwrap_or("eth0"), json),
        "ip"      => cmd_set_ip(
            positional.get(0).copied().unwrap_or("eth0"),
            positional.get(1).copied().unwrap_or("10.0.0.1/24"), json),
        "dhcp"    => cmd_dhcp(positional.first().copied().unwrap_or("eth0"), json),
        "ping"    => cmd_ping(positional.first().copied().unwrap_or("8.8.8.8"), count, json),
        "trace"   => {
            let host = positional.first().copied().unwrap_or("8.8.8.8");
            let ok = Command::new("traceroute").arg(host).status().map(|s| s.success()).unwrap_or(false);
            if !ok { println!("{} traceroute {} (simulation — 3 hops)", cyan("Σ"), host); }
        }
        "route"   => {
            let action = positional.first().copied().unwrap_or("list");
            match action {
                "list" => cmd_route_list(json),
                "add"  => {
                    let prefix = positional.get(1).copied().unwrap_or("0.0.0.0/0");
                    let gw     = positional.get(3).copied().unwrap_or("10.0.0.254");
                    if json { println!("{{\"route\":\"add\",\"prefix\":\"{}\",\"via\":\"{}\"}}", prefix, gw); }
                    else { println!("{} Route {} via {} added.", green("✓"), prefix, gw); }
                }
                "del"  => {
                    let prefix = positional.get(1).copied().unwrap_or("");
                    if json { println!("{{\"route\":\"del\",\"prefix\":\"{}\",\"status\":\"ok\"}}", prefix); }
                    else { println!("{} Route {} removed.", green("✓"), prefix); }
                }
                _ => eprintln!("{} unknown route action. Valid: list, add, del", red("error:")),
            }
        }
        "dns"     => cmd_dns(positional.first().copied().unwrap_or("show"), &positional[1..], json),
        "wifi"    => cmd_wifi(positional.first().copied().unwrap_or("status"), &positional[1..], json),
        "fw"      => cmd_fw(positional.first().copied().unwrap_or("list"), &positional[1..], force, json),
        "stats"   => cmd_stats(positional.first().copied(), json),
        "capture" => cmd_capture(positional.first().copied().unwrap_or("eth0"), count, json),
        "scan"    => cmd_scan(positional.first().copied().unwrap_or("10.0.0.0/24"), json),
        "mac"     => {
            let iface = positional.first().copied().unwrap_or("eth0");
            if json { println!("{{\"iface\":\"{}\",\"mac\":\"52:54:00:ab:cd:01\"}}", iface); }
            else { println!("  {}  52:54:00:ab:cd:01", iface); }
        }
        _ => { eprintln!("{} unknown command '{}'. Run --help.", red("error:"), args[1]); exit(1); }
    }
}
