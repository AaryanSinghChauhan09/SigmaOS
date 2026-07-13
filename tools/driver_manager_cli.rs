// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// tools/driver_manager_cli.rs — CLI frontend for driver management

use std::env;
use crate::drivers::manager::driver_manager::DriverManager;
use crate::drivers::catalogue::driver_catalogue::CatalogueQuery;

pub fn main_cli() {
    let args: Vec<String> = env::args().collect();
    let catalogue_path = "data/driver_catalogue.json";
    
    let mut manager = match DriverManager::new(catalogue_path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error loading driver manager: {}", e);
            std::process::exit(1);
        }
    };

    let command = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match command {
        "scan" => {
            println!("Scanning local PCI/USB/ACPI hardware...");
            let matches = manager.match_drivers();
            if matches.is_empty() {
                println!("No matching drivers found.");
            } else {
                println!("Matched Drivers:");
                for (hw, driver_id) in matches {
                    let entry = manager.catalogue.get(&driver_id).unwrap();
                    println!("  Device: {} -> Driver: {} [{}] ({})", 
                        hw, entry.display_name, entry.status, entry.compat_status);
                }
            }
        }
        "list" => {
            let entries = manager.catalogue.query(&CatalogueQuery {
                include_removed: true,
                include_deprecated: true,
                ..Default::default()
            });
            println!("All catalogued drivers:");
            for e in entries {
                println!("  {:<20} | {:<20} | Era: {}..{} | {}", 
                    e.id, e.display_name, e.min_kernel, 
                    e.max_kernel.as_ref().map(|k| k.to_string()).unwrap_or_else(|| "Latest".to_string()),
                    e.status);
            }
        }
        "install" => {
            if let Some(name) = args.get(2) {
                println!("Attempting selective download for driver: {}...", name);
                match manager.install_driver(name) {
                    Ok(_) => println!("Driver '{}' installed successfully.", name),
                    Err(e) => eprintln!("Installation failed: {}", e),
                }
            } else {
                println!("Usage: sigma-drivers install <driver_id>");
            }
        }
        "help" | _ => {
            println!("sigma-drivers — Driver warehouse CLI manager");
            println!("Usage:");
            println!("  sigma-drivers scan             Scan hardware and match drivers");
            println!("  sigma-drivers list             List all available/deprecated drivers");
            println!("  sigma-drivers install <id>     Download and activate selective driver module");
        }
    }
}
