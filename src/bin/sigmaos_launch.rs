// SPDX-License-Identifier: MIT OR Apache-2.0
//! SigmaOS Live Terminal Runtime & Interactive Controller
//! Inspired by Arch Linux's pacman/archinstall TUI and Omarchy's status dashboards.

use std::io::{self, Write};
use std::process::Command;

fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}

fn render_header(persona: &str) {
    println!("\x1b[1;36m┌────────────────────────────────────────────────────────────────────────────┐\x1b[0m");
    println!("\x1b[1;36m│\x1b[0m  \x1b[1;35m󰣇 SIGMAOS SOVEREIGN WORKSTATION\x1b[0m \x1b[33m(Arch Rolling + Omarchy Hyprland)\x1b[0m       \x1b[1;36m│\x1b[0m");
    println!("\x1b[1;36m├────────────────────────────────────────────────────────────────────────────┤\x1b[0m");
    println!("\x1b[1;36m│\x1b[0m Active Persona  : \x1b[1;32m{:<57}\x1b[0m\x1b[1;36m│\x1b[0m", persona);
    println!("\x1b[1;36m│\x1b[0m Substrate Core  : \x1b[34mArch Linux ALPM v7 Engine + Btrfs Subvolumes\x1b[0m              \x1b[1;36m│\x1b[0m");
    println!("\x1b[1;36m│\x1b[0m Desktop Aesthetic: \x1b[35mOmarchy Catppuccin Mocha · Wallust · Starship Prompt\x1b[0m     \x1b[1;36m│\x1b[0m");
    println!("\x1b[1;36m└────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    println!();
}

fn print_menu() {
    println!("\x1b[1mOperational Commands & Workflows:\x1b[0m");
    println!("  \x1b[1;32m1\x1b[0m) \x1b[1mOmarchy Persona\x1b[0m        - Switch to Omarchy Hyprland & Wallust aesthetic");
    println!("  \x1b[1;32m2\x1b[0m) \x1b[1mArch Persona\x1b[0m           - Switch to minimal Arch Linux rolling release environment");
    println!("  \x1b[1;32m3\x1b[0m) \x1b[1mPacman Update (Syu)\x1b[0m    - Simulate Arch rolling release package synchronization");
    println!("  \x1b[1;32m4\x1b[0m) \x1b[1mAUR Search\x1b[0m             - Query Arch User Repository (AUR v5 RPC)");
    println!("  \x1b[1;32m5\x1b[0m) \x1b[1mWallust Themes\x1b[0m         - List dynamic wallpaper & palette themes");
    println!("  \x1b[1;32m6\x1b[0m) \x1b[1mOmarchy Doctor\x1b[0m         - Run 14-point diagnostic system health checks");
    println!("  \x1b[1;32m7\x1b[0m) \x1b[1mLaunch Interactive Shell\x1b[0m- Enter live interactive shell with pacman/omarchy tools");
    println!("  \x1b[1;32m8\x1b[0m) \x1b[1mVerify Subsystems\x1b[0m      - Execute sovereign v36 complete readiness test");
    println!("  \x1b[1;33m9\x1b[0m) \x1b[1mExit\x1b[0m                   - Return to host shell");
    println!();
}

fn run_cmd(cmd: &str) {
    println!("\n\x1b[1;34m==> Executing: {}\x1b[0m", cmd);
    let _ = Command::new("bash").arg("-c").arg(cmd).status();
    println!("\n\x1b[2mPress Enter to return to menu...\x1b[0m");
    let mut discard = String::new();
    let _ = io::stdin().read_line(&mut discard);
}

fn main() {
    let mut current_persona = "Omarchy Aesthetic (Hyprland / Wallust)".to_string();

    loop {
        clear_screen();
        render_header(&current_persona);
        print_menu();

        print!("\x1b[1;35msigmaos>\x1b[0m Select an option [1-9]: ");
        let _ = io::stdout().flush();

        let mut choice = String::new();
        if io::stdin().read_line(&mut choice).is_err() {
            break;
        }

        match choice.trim() {
            "1" => {
                current_persona = "Omarchy Aesthetic (Hyprland / Wallust)".to_string();
                run_cmd("sigmaos --omarchy");
            }
            "2" => {
                current_persona = "Arch Linux Minimalist (Pacman / Rolling)".to_string();
                run_cmd("sigmaos --arch");
            }
            "3" => {
                run_cmd("sigpkg update");
            }
            "4" => {
                print!("\nEnter AUR package query: ");
                let _ = io::stdout().flush();
                let mut query = String::new();
                let _ = io::stdin().read_line(&mut query);
                let q = query.trim();
                if !q.is_empty() {
                    run_cmd(&format!("sigpkg aur {}", q));
                }
            }
            "5" => {
                run_cmd("omarchy theme list");
            }
            "6" => {
                run_cmd("omarchy doctor");
            }
            "7" => {
                println!("\n\x1b[1;32mEntering SigmaOS Interactive Live Shell. Type 'exit' to return.\x1b[0m");
                let _ = Command::new("sigmaos-interactive").status();
            }
            "8" => {
                run_cmd("bash /home/aaryansinghchauhan/SigmaOS/scripts/sovereign_arch_omarchy_v36_complete_readiness_test.sh");
            }
            "9" | "exit" | "quit" | "q" => {
                println!("\n\x1b[1;32mThank you for testing SigmaOS. Goodbye!\x1b[0m\n");
                break;
            }
            _ => {
                // Ignore invalid input
            }
        }
    }
}
