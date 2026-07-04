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
        eprintln!("sigma-rm: usage: rm [-r] <file>...");
        return 1;
    }

    let mut recursive = false;
    let mut files: Vec<&String> = Vec::new();
    
    for arg in &args[1..] {
        if arg == "-r" || arg == "--recursive" {
            recursive = true;
        } else {
            files.push(arg);
        }
    }

    let mut exit_code = 0;
    for arg in files {
        let path = Path::new(arg);
        if !path.exists() {
            eprintln!("sigma-rm: {}: No such file or directory", arg);
            exit_code = 1;
            continue;
        }

        if path.is_dir() {
            if recursive {
                match fs::remove_dir_all(path) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("sigma-rm: {}: {}", arg, e);
                        exit_code = 1;
                    }
                }
            } else {
                eprintln!("sigma-rm: {}: is a directory (use -r for recursive)", arg);
                exit_code = 1;
                continue;
            }
        } else {
            match fs::remove_file(path) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("sigma-rm: {}: {}", arg, e);
                    exit_code = 1;
                }
            }
        }
    }

    exit_code
}

// ─── mkdir: Create directories ───────────────────────────────────────────────────

fn cmd_mkdir(args: &[String]) -> i32 {
    if args.len() < 2 {
        eprintln!("sigma-mkdir: usage: mkdir [-p] <directory>...");
        return 1;
    }

    let mut parents = false;
    let mut dirs: Vec<&String> = Vec::new();
    
    for arg in &args[1..] {
        if arg == "-p" || arg == "--parents" {
            parents = true;
        } else {
            dirs.push(arg);
        }
    }

    let mut exit_code = 0;
    for arg in dirs {
        let path = Path::new(arg);
        if path.exists() {
            // Don't error if directory already exists with -p
            if !parents {
                eprintln!("sigma-mkdir: {}: File exists", arg);
                exit_code = 1;
            }
            continue;
        }

        let result = if parents {
            fs::create_dir_all(path)
        } else {
            fs::create_dir(path)
        };
        
        match result {
            Ok(_) => {}
            Err(e) => {
                eprintln!("sigma-mkdir: {}: {}", arg, e);
                exit_code = 1;
            }
        }
    }

    exit_code
}

// ─── pwd: Print working directory ───────────────────────────────────────────────────

fn cmd_pwd() -> i32 {
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
            0
        }
        Err(e) => {
            eprintln!("sigma-pwd: {}", e);
            1
        }
    }
}

// ─── echo: Print text to stdout ────────────────────────────────────────────────────

fn cmd_echo(args: &[String]) -> i32 {
    if args.len() < 2 {
        println!();
        return 0;
    }
    
    let text = args[1..].join(" ");
    println!("{}", text);
    0
}

// ─── cat: Concatenate and print files ───────────────────────────────────────────────

fn cmd_cat(args: &[String]) -> i32 {
    if args.len() < 2 {
        eprintln!("sigma-cat: usage: cat [-n] <file>...");
        return 1;
    }

    let mut number_lines = false;
    let mut files: Vec<&String> = Vec::new();
    
    for arg in &args[1..] {
        if arg == "-n" || arg == "--number" {
            number_lines = true;
        } else {
            files.push(arg);
        }
    }

    let mut exit_code = 0;
    let mut line_num = 1;
    for arg in files {
        let path = Path::new(arg);
        match fs::read_to_string(path) {
            Ok(contents) => {
                if number_lines {
                    for line in contents.lines() {
                        println!("{:6}\t{}", line_num, line);
                        line_num += 1;
                    }
                } else {
                    print!("{}", contents);
                }
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
    println!("SigmaOS Core Utilities v1.1");
    println!();
    println!("Usage: {} <command> [args...]", program);
    println!();
    println!("Commands:");
    println!("  ls <dir>           List directory contents");
    println!("  cp <src> <dst>      Copy files");
    println!("  mv <src> <dst>      Move/rename files");
    println!("  rm [-r] <file>...   Remove files (-r for recursive)");
    println!("  mkdir [-p] <dir>... Create directories (-p for parents)");
    println!("  cat [-n] <file>...  Print file contents (-n for line numbers)");
    println!("  pwd                 Print working directory");
    println!("  echo <text>         Print text to stdout");
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
        "pwd" => cmd_pwd(),
        "echo" => cmd_echo(&args),
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
