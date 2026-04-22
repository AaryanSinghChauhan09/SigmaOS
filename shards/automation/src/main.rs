//! sigma-daemon — SigmaOS auto-sync background process
//! Usage: sigma-daemon --interval 300 --root /path/to/SigmaOS [--self-heal]

use sigma_automation::AutomationDaemon;
use std::time::Duration;
use std::thread;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut interval = 300u64;
    let mut root = ".".to_string();
    let mut self_heal = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--interval" => { i += 1; interval = args[i].parse().unwrap_or(300); }
            "--root"     => { i += 1; root = args[i].clone(); }
            "--self-heal"=> { self_heal = true; }
            _ => {}
        }
        i += 1;
    }

    println!("\x1b[36m[Σ DAEMON]\x1b[0m Auto-sync started | interval={}s | root={}", interval, root);
    let daemon = AutomationDaemon::new(root, interval, self_heal);

    loop {
        thread::sleep(Duration::from_secs(daemon.interval_secs));
        let ok = daemon.sync_cycle();
        if ok {
            println!("\x1b[32m[DAEMON]\x1b[0m Sync OK");
        } else {
            eprintln!("\x1b[31m[DAEMON]\x1b[0m Sync failed — will retry next cycle");
        }
    }
}
