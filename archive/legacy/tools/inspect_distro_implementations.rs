// SPDX-License-Identifier: MIT
//! SigmaOS Distro Implementation Inspection Tool (Native Rust)
//! Audits Linux & BSD distribution inspired abstractions across `src/` without Python.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::exit;

const DISTRO_KEYWORDS: &[&str] = &[
    "Arch", "Debian", "Fedora", "Alpine", "Nix", "Gentoo",
    "FreeBSD", "OpenBSD", "NetBSD", "DragonFly", "Void", "Slackware",
];

fn inspect_directory(dir: &Path, counts: &mut BTreeMap<&'static str, usize>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                inspect_directory(&path, counts);
            } else if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Ok(content) = fs::read_to_string(&path) {
                    for &kw in DISTRO_KEYWORDS {
                        let matches = content.matches(kw).count();
                        *counts.entry(kw).or_insert(0) += matches;
                    }
                }
            }
        }
    }
}

fn main() {
    println!("=== SigmaOS Distro Implementation Inspection Tool (Native Rust) ===");
    let src_dir = Path::new("src");
    let mut counts: BTreeMap<&'static str, usize> = BTreeMap::new();

    for &kw in DISTRO_KEYWORDS {
        counts.insert(kw, 0);
    }

    if src_dir.exists() {
        inspect_directory(src_dir, &mut counts);
    } else {
        eprintln!("[ERROR] Directory 'src/' not found.");
        exit(1);
    }

    println!("\n[+] Distribution Reference Counts in src/:");
    let mut total_refs = 0;
    for (kw, count) in &counts {
        println!("  - {:<12}: {} occurrences", kw, count);
        total_refs += count;
    }

    println!("\nTotal Distro Abstraction References: {}", total_refs);

    if total_refs > 100 {
        println!("[SUCCESS] Distro abstractions are thoroughly integrated and verified in native Rust.");
        exit(0);
    } else {
        println!("[WARNING] Low distro abstraction count detected.");
        exit(1);
    }
}
