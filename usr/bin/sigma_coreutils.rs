// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/bin/sigma_coreutils.rs — Core Utilities for SigmaOS
//
// Implements basic Unix-like utilities: ls, cp, mv, rm, mkdir
// Language: Rust (std for userland utilities)

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

// ─── ls: List directory contents ───────────────────────────────────────────────────

fn cmd_ls(args: &[String]) -> i32 {
    let path = args.get(1).map(|s| s.as_str()).unwrap_or(".");
    let path_obj = Path::new(path);

    if !path_obj.exists() {
        eprintln!("sigma-ls: {}: No such file or directory", path);
        return 1;
    }

    if path_obj.is_file() {
        // Single file: just print its name
        println!("{}", path);
        return 0;
    }

    // Directory: list contents
    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("sigma-ls: {}: {}", path, e);
            return 1;
        }
    };

    let mut names: Vec<String> = Vec::new();
    for entry in entries {
        match entry {
            Ok(e) => {
                if let Some(name) = e.file_name().to_str() {
                    names.push(name.to_string());
                }
            }
            Err(e) => {
                eprintln!("sigma-ls: error reading entry: {}", e);
            }
        }
    }

    names.sort();
    for name in names {
        // Add trailing slash for directories
        let full_path = path_obj.join(&name);
        if full_path.is_dir() {
            println!("{}/", name);
        } else {
            println!("{}", name);
        }
    }

    0
}

// ─── cp: Copy files ─────────────────────────────────────────────────────────────

fn cmd_cp(args: &[String]) -> i32 {
    if args.len() < 3 {
        eprintln!("sigma-cp: usage: cp <source> <destination>");
        return 1;
    }

    let src = &args[1];
    let dst = &args[2];

    let src_path = Path::new(src);
    let dst_path = Path::new(dst);

    if !src_path.exists() {
        eprintln!("sigma-cp: {}: No such file or directory", src);
        return 1;
    }

    // If destination is a directory, copy source into it
    if dst_path.is_dir() {
        let file_name = src_path.file_name().unwrap();
        let new_dst = dst_path.join(file_name);
        match fs::copy(src, &new_dst) {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("sigma-cp: {}: {}", src, e);
                1
            }
        }
    } else {
        match fs::copy(src, dst) {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("sigma-cp: {}: {}", src, e);
                1
            }
        }
    }
}

// ─── mv: Move/rename files ───────────────────────────────────────────────────────

fn cmd_mv(args: &[String]) -> i32 {
    if args.len() < 3 {
        eprintln!("sigma-mv: usage: mv <source> <destination>");
        return 1;
    }

    let src = &args[1];
    let dst = &args[2];

    let src_path = Path::new(src);
    let dst_path = Path::new(dst);

    if !src_path.exists() {
        eprintln!("sigma-mv: {}: No such file or directory", src);
        return 1;
    }

    // If destination is a directory, move source into it
    if dst_path.is_dir() {
        let file_name = src_path.file_name().unwrap();
        let new_dst = dst_path.join(file_name);
        match fs::rename(src, &new_dst) {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("sigma-mv: {}: {}", src, e);
                1
            }
        }
    } else {
        match fs::rename(src, dst) {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("sigma-mv: {}: {}", src, e);
                1
            }
        }
    }
}

// ─── rm: Remove files ─────────────────────────────────────────────────────────────

fn cmd_rm(args: &[String]) -> i32 {
    if args.len() < 2 {
        eprintln!("sigma-rm: usage: rm <file>...");
        return 1;
    }

    let mut exit_code = 0;
    for arg in &args[1..] {
        let path = Path::new(arg);
        if !path.exists() {
            eprintln!("sigma-rm: {}: No such file or directory", arg);
            exit_code = 1;
            continue;
        }

        if path.is_dir() {
            eprintln!("sigma-rm: {}: is a directory (use -r for recursive)", arg);
            exit_code = 1;
            continue;
        }

        match fs::remove_file(path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("sigma-rm: {}: {}", arg, e);
                exit_code = 1;
            }
        }
    }

    exit_code
}

// ─── mkdir: Create directories ───────────────────────────────────────────────────

fn cmd_mkdir(args: &[String]) -> i32 {
    if args.len() < 2 {
        eprintln!("sigma-mkdir: usage: mkdir <directory>...");
        return 1;
    }

    let mut exit_code = 0;
    for arg in &args[1..] {
        let path = Path::new(arg);
        if path.exists() {
            eprintln!("sigma-mkdir: {}: File exists", arg);
            exit_code = 1;
            continue;
        }

        match fs::create_dir(path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("sigma-mkdir: {}: {}", arg, e);
                exit_code = 1;
            }
        }
    }

    exit_code
}

// ─── cat: Concatenate and print files ───────────────────────────────────────────────

fn cmd_cat(args: &[String]) -> i32 {
    if args.len() < 2 {
        eprintln!("sigma-cat: usage: cat <file>...");
        return 1;
    }

    let mut exit_code = 0;
    for arg in &args[1..] {
        let path = Path::new(arg);
        match fs::read_to_string(path) {
            Ok(contents) => {
                print!("{}", contents);
            }
            Err(e) => {
                eprintln!("sigma-cat: {}: {}", arg, e);
                exit_code = 1;
            }
        }
    }

    exit_code
}

// ─── Main entry point ───────────────────────────────────────────────────────────

fn print_usage(program: &str) {
    println!("SigmaOS Core Utilities v1.0");
    println!();
    println!("Usage: {} <command> [args...]", program);
    println!();
    println!("Commands:");
    println!("  ls <dir>       List directory contents");
    println!("  cp <src> <dst>  Copy files");
    println!("  mv <src> <dst>  Move/rename files");
    println!("  rm <file>...   Remove files");
    println!("  mkdir <dir>...  Create directories");
    println!("  cat <file>...   Print file contents");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let command = &args[1];
    let exit_code = match command.as_str() {
        "ls" => cmd_ls(&args),
        "cp" => cmd_cp(&args),
        "mv" => cmd_mv(&args),
        "rm" => cmd_rm(&args),
        "mkdir" => cmd_mkdir(&args),
        "cat" => cmd_cat(&args),
        "--help" | "-h" => {
            print_usage(&args[0]);
            0
        }
        _ => {
            eprintln!("sigma-coreutils: unknown command '{}'", command);
            print_usage(&args[0]);
            1
        }
    };

    std::process::exit(exit_code);
}
