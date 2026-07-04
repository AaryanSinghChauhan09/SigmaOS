// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/coreutils/src/main.rs — SigmaOS core utilities (multi-call binary)
//
// Implements: ls, cat, echo, pwd, mkdir, rm, cp, mv, touch, stat,
//             grep, head, tail, wc, true, false, uname, sleep, env, id
//
// Like BusyBox: one binary, many commands via argv[0] or first argument.
// All utilities use std — this is a userspace tool.

use std::env;
use std::fs;
use std::io::{self, Read, Write, BufRead, BufWriter};
use std::path::Path;
use std::os::unix::fs::MetadataExt;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Determine which command to run: argv[0] basename, or argv[1]
    let cmd = {
        let base = Path::new(&args[0])
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("sigma-coreutils");
        if base == "sigma-coreutils" || base == "coreutils" {
            args.get(1).map(|s| s.as_str()).unwrap_or("help")
        } else {
            base
        }
    };

    let sub_args: &[String] = if cmd == args.get(0).map(|s| s.as_str()).unwrap_or("") {
        &args[2..]
    } else {
        &args[1..]
    };

    let result = match cmd {
        "ls"     => cmd_ls(sub_args),
        "cat"    => cmd_cat(sub_args),
        "echo"   => cmd_echo(sub_args),
        "pwd"    => cmd_pwd(),
        "mkdir"  => cmd_mkdir(sub_args),
        "rm"     => cmd_rm(sub_args),
        "cp"     => cmd_cp(sub_args),
        "mv"     => cmd_mv(sub_args),
        "touch"  => cmd_touch(sub_args),
        "stat"   => cmd_stat(sub_args),
        "grep"   => cmd_grep(sub_args),
        "head"   => cmd_head(sub_args),
        "tail"   => cmd_tail(sub_args),
        "wc"     => cmd_wc(sub_args),
        "uname"  => cmd_uname(sub_args),
        "sleep"  => cmd_sleep(sub_args),
        "env"    => cmd_env(sub_args),
        "id"     => cmd_id(),
        "whoami" => cmd_whoami(),
        "true"   => Ok(()),
        "false"  => std::process::exit(1),
        "yes"    => cmd_yes(sub_args),
        "help" | "--help" | "-h" => {
            print_help(); Ok(())
        }
        unknown  => {
            eprintln!("sigma-coreutils: unknown command '{}'", unknown);
            print_help();
            std::process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("sigma-coreutils {}: {}", cmd, e);
        std::process::exit(1);
    }
}

// ── ls ─────────────────────────────────────────────────────────────────────
fn cmd_ls(args: &[String]) -> io::Result<()> {
    let long  = args.iter().any(|a| a == "-l" || a == "-la" || a == "-al");
    let all   = args.iter().any(|a| a == "-a" || a == "-la" || a == "-al");
    let paths: Vec<&str> = args.iter()
        .filter(|a| !a.starts_with('-'))
        .map(|a| a.as_str())
        .collect();
    let targets = if paths.is_empty() { vec!["."] } else { paths };

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    for target in targets {
        let meta = fs::metadata(target)?;
        if meta.is_dir() {
            let mut entries: Vec<_> = fs::read_dir(target)?
                .filter_map(|e| e.ok())
                .collect();
            entries.sort_by_key(|e| e.file_name());
            for entry in entries {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if !all && name_str.starts_with('.') { continue; }
                if long {
                    let m = entry.metadata()?;
                    let perms = format_perms(&m);
                    let size  = m.len();
                    write!(out, "{} {:>10} {}\n", perms, size, name_str)?;
                } else {
                    write!(out, "{}  ", name_str)?;
                }
            }
            if !long { writeln!(out)?; }
        } else {
            if long {
                let m = fs::metadata(target)?;
                writeln!(out, "{} {:>10} {}", format_perms(&m), m.len(), target)?;
            } else {
                writeln!(out, "{}", target)?;
            }
        }
    }
    Ok(())
}

fn format_perms(m: &fs::Metadata) -> String {
    let mode = m.mode();
    let kind  = if m.is_dir() { 'd' } else if m.file_type().is_symlink() { 'l' } else { '-' };
    let chars: Vec<char> = (0..9).rev().map(|i| {
        let bit = (mode >> i) & 1;
        match i % 3 {
            2 => if bit == 1 { 'r' } else { '-' },
            1 => if bit == 1 { 'w' } else { '-' },
            0 => if bit == 1 { 'x' } else { '-' },
            _ => '-',
        }
    }).collect();
    format!("{}{}", kind, chars.iter().collect::<String>())
}

// ── cat ────────────────────────────────────────────────────────────────────
fn cmd_cat(args: &[String]) -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    if args.is_empty() {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        write!(out, "{}", buf)?;
    } else {
        for path in args.iter().filter(|a| !a.starts_with('-')) {
            let mut f = fs::File::open(path)?;
            let mut buf = Vec::new();
            f.read_to_end(&mut buf)?;
            out.write_all(&buf)?;
        }
    }
    Ok(())
}

// ── echo ───────────────────────────────────────────────────────────────────
fn cmd_echo(args: &[String]) -> io::Result<()> {
    let no_newline = args.first().map(|a| a == "-n").unwrap_or(false);
    let start = if no_newline { 1 } else { 0 };
    let output = args[start..].join(" ");
    if no_newline {
        print!("{}", output);
    } else {
        println!("{}", output);
    }
    Ok(())
}

// ── pwd ────────────────────────────────────────────────────────────────────
fn cmd_pwd() -> io::Result<()> {
    let cwd = env::current_dir()?;
    println!("{}", cwd.display());
    Ok(())
}

// ── mkdir ──────────────────────────────────────────────────────────────────
fn cmd_mkdir(args: &[String]) -> io::Result<()> {
    let parents = args.iter().any(|a| a == "-p" || a == "--parents");
    for path in args.iter().filter(|a| !a.starts_with('-')) {
        if parents {
            fs::create_dir_all(path)?;
        } else {
            fs::create_dir(path)?;
        }
    }
    Ok(())
}

// ── rm ─────────────────────────────────────────────────────────────────────
fn cmd_rm(args: &[String]) -> io::Result<()> {
    let recursive = args.iter().any(|a| a == "-r" || a == "-rf" || a == "-fr" || a == "-R");
    let force     = args.iter().any(|a| a == "-f" || a == "-rf" || a == "-fr");
    for path in args.iter().filter(|a| !a.starts_with('-')) {
        let result = if recursive {
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        };
        if let Err(e) = result {
            if !force { return Err(e); }
        }
    }
    Ok(())
}

// ── cp ─────────────────────────────────────────────────────────────────────
fn cmd_cp(args: &[String]) -> io::Result<()> {
    let paths: Vec<&str> = args.iter().filter(|a| !a.starts_with('-'))
        .map(|s| s.as_str()).collect();
    if paths.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Usage: cp src dst"));
    }
    let dst = paths[paths.len() - 1];
    for src in &paths[..paths.len() - 1] {
        let dst_path = if Path::new(dst).is_dir() {
            let fname = Path::new(src).file_name().unwrap_or_default();
            Path::new(dst).join(fname).to_string_lossy().to_string()
        } else {
            dst.to_string()
        };
        fs::copy(src, &dst_path)?;
    }
    Ok(())
}

// ── mv ─────────────────────────────────────────────────────────────────────
fn cmd_mv(args: &[String]) -> io::Result<()> {
    let paths: Vec<&str> = args.iter().filter(|a| !a.starts_with('-'))
        .map(|s| s.as_str()).collect();
    if paths.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Usage: mv src dst"));
    }
    let dst = paths[paths.len() - 1];
    for src in &paths[..paths.len() - 1] {
        let dst_path = if Path::new(dst).is_dir() {
            let fname = Path::new(src).file_name().unwrap_or_default();
            Path::new(dst).join(fname).to_string_lossy().to_string()
        } else {
            dst.to_string()
        };
        fs::rename(src, &dst_path)?;
    }
    Ok(())
}

// ── touch ─────────────────────────────────────────────────────────────────
fn cmd_touch(args: &[String]) -> io::Result<()> {
    for path in args.iter().filter(|a| !a.starts_with('-')) {
        if !Path::new(path).exists() {
            fs::File::create(path)?;
        }
        // Update timestamps via open (simplified)
    }
    Ok(())
}

// ── stat ───────────────────────────────────────────────────────────────────
fn cmd_stat(args: &[String]) -> io::Result<()> {
    for path in args.iter().filter(|a| !a.starts_with('-')) {
        let m = fs::metadata(path)?;
        println!("  File: {}", path);
        println!("  Size: {}  Type: {}", m.len(),
            if m.is_dir() { "directory" } else { "regular file" });
        println!(" Perms: {}", format_perms(&m));
        println!("  Ino:  {}", m.ino());
    }
    Ok(())
}

// ── grep ───────────────────────────────────────────────────────────────────
fn cmd_grep(args: &[String]) -> io::Result<()> {
    let ignore_case = args.iter().any(|a| a == "-i");
    let non_flags: Vec<&str> = args.iter().filter(|a| !a.starts_with('-'))
        .map(|s| s.as_str()).collect();
    if non_flags.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Usage: grep pattern [file...]"));
    }
    let pattern = non_flags[0];
    let pat_lower = pattern.to_lowercase();
    let files = &non_flags[1..];

    let do_grep = |reader: &mut dyn BufRead, prefix: &str| -> io::Result<()> {
        let mut line = String::new();
        let mut lineno = 0usize;
        loop {
            line.clear();
            let n = reader.read_line(&mut line)?;
            if n == 0 { break; }
            lineno += 1;
            let matches = if ignore_case {
                line.to_lowercase().contains(&pat_lower)
            } else {
                line.contains(pattern)
            };
            if matches {
                if prefix.is_empty() {
                    print!("{}", line);
                } else {
                    print!("{}:{}", prefix, line);
                }
            }
        }
        Ok(())
    };

    if files.is_empty() {
        let stdin = io::stdin();
        do_grep(&mut stdin.lock(), "")?;
    } else {
        for path in files {
            let f = fs::File::open(path)?;
            let mut r = io::BufReader::new(f);
            let prefix = if files.len() > 1 { path } else { "" };
            do_grep(&mut r, prefix)?;
        }
    }
    Ok(())
}

// ── head ───────────────────────────────────────────────────────────────────
fn cmd_head(args: &[String]) -> io::Result<()> {
    let n: usize = args.windows(2)
        .find(|w| w[0] == "-n")
        .and_then(|w| w[1].parse().ok())
        .unwrap_or(10);
    let files: Vec<&str> = args.iter().filter(|a| !a.starts_with('-') && a.parse::<usize>().is_err())
        .map(|s| s.as_str()).collect();

    let read_head = |reader: &mut dyn BufRead| -> io::Result<()> {
        let mut line = String::new();
        for _ in 0..n {
            line.clear();
            if reader.read_line(&mut line)? == 0 { break; }
            print!("{}", line);
        }
        Ok(())
    };

    if files.is_empty() {
        read_head(&mut io::stdin().lock())?;
    } else {
        for path in files {
            let f = fs::File::open(path)?;
            read_head(&mut io::BufReader::new(f))?;
        }
    }
    Ok(())
}

// ── tail ───────────────────────────────────────────────────────────────────
fn cmd_tail(args: &[String]) -> io::Result<()> {
    let n: usize = args.windows(2)
        .find(|w| w[0] == "-n")
        .and_then(|w| w[1].parse().ok())
        .unwrap_or(10);
    let files: Vec<&str> = args.iter().filter(|a| !a.starts_with('-'))
        .map(|s| s.as_str()).collect();

    let read_tail = |reader: &mut dyn BufRead| -> io::Result<()> {
        let mut ring: Vec<String> = Vec::with_capacity(n + 1);
        let mut line = String::new();
        loop {
            line.clear();
            if reader.read_line(&mut line)? == 0 { break; }
            if ring.len() >= n { ring.remove(0); }
            ring.push(line.clone());
        }
        for l in ring { print!("{}", l); }
        Ok(())
    };

    if files.is_empty() {
        read_tail(&mut io::stdin().lock())?;
    } else {
        for path in files {
            let f = fs::File::open(path)?;
            read_tail(&mut io::BufReader::new(f))?;
        }
    }
    Ok(())
}

// ── wc ─────────────────────────────────────────────────────────────────────
fn cmd_wc(args: &[String]) -> io::Result<()> {
    let l_only = args.iter().any(|a| a == "-l");
    let w_only = args.iter().any(|a| a == "-w");
    let c_only = args.iter().any(|a| a == "-c");
    let files: Vec<&str> = args.iter().filter(|a| !a.starts_with('-'))
        .map(|s| s.as_str()).collect();

    let count = |reader: &mut dyn Read| -> io::Result<(u64, u64, u64)> {
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        let lines = content.lines().count() as u64;
        let words = content.split_whitespace().count() as u64;
        let bytes = content.len() as u64;
        Ok((lines, words, bytes))
    };

    if files.is_empty() {
        let (l, w, c) = count(&mut io::stdin())?;
        if l_only { println!("{}", l); }
        else if w_only { println!("{}", w); }
        else if c_only { println!("{}", c); }
        else { println!("{:>8} {:>8} {:>8}", l, w, c); }
    } else {
        for path in files {
            let mut f = fs::File::open(path)?;
            let (l, w, c) = count(&mut f)?;
            if l_only { println!("{:>8} {}", l, path); }
            else if w_only { println!("{:>8} {}", w, path); }
            else if c_only { println!("{:>8} {}", c, path); }
            else { println!("{:>8} {:>8} {:>8} {}", l, w, c, path); }
        }
    }
    Ok(())
}

// ── uname ─────────────────────────────────────────────────────────────────
fn cmd_uname(args: &[String]) -> io::Result<()> {
    let all = args.iter().any(|a| a == "-a");
    let sys = args.iter().any(|a| a == "-s") || all;
    let nod = args.iter().any(|a| a == "-n") || all;
    let rel = args.iter().any(|a| a == "-r") || all;
    let ver = args.iter().any(|a| a == "-v") || all;
    let mac = args.iter().any(|a| a == "-m") || all;
    if args.is_empty() || all {
        println!("SigmaOS sigmaos 15.0.0-Zenith #1 SMP SigmaOS Zenith x86_64");
    } else {
        let mut parts = vec![];
        if sys { parts.push("SigmaOS"); }
        if nod { parts.push("sigmaos"); }
        if rel { parts.push("15.0.0-Zenith"); }
        if ver { parts.push("#1 SMP"); }
        if mac { parts.push("x86_64"); }
        println!("{}", parts.join(" "));
    }
    Ok(())
}

// ── sleep ─────────────────────────────────────────────────────────────────
fn cmd_sleep(args: &[String]) -> io::Result<()> {
    let secs: f64 = args.first()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1.0);
    std::thread::sleep(std::time::Duration::from_secs_f64(secs));
    Ok(())
}

// ── env ────────────────────────────────────────────────────────────────────
fn cmd_env(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        for (k, v) in env::vars() { println!("{}={}", k, v); }
    } else {
        // env VAR=val cmd args...
        let mut cmd_start = 0;
        let mut extra_env = vec![];
        for (i, arg) in args.iter().enumerate() {
            if arg.contains('=') {
                extra_env.push(arg.as_str());
            } else {
                cmd_start = i;
                break;
            }
        }
        if cmd_start < args.len() {
            let mut cmd = std::process::Command::new(&args[cmd_start]);
            cmd.args(&args[cmd_start + 1..]);
            for kv in extra_env {
                let mut parts = kv.splitn(2, '=');
                if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
                    cmd.env(k, v);
                }
            }
            let status = cmd.status()?;
            std::process::exit(status.code().unwrap_or(1));
        }
    }
    Ok(())
}

// ── id / whoami ────────────────────────────────────────────────────────────
fn cmd_id() -> io::Result<()> {
    println!("uid=0(root) gid=0(root) groups=0(root)");
    Ok(())
}

fn cmd_whoami() -> io::Result<()> {
    println!("root");
    Ok(())
}

// ── yes ────────────────────────────────────────────────────────────────────
fn cmd_yes(args: &[String]) -> io::Result<()> {
    let msg = if args.is_empty() { "y".to_string() } else { args.join(" ") };
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    loop { writeln!(out, "{}", msg)?; }
}

// ── help ───────────────────────────────────────────────────────────────────
fn print_help() {
    println!("Σ sigma-coreutils — SigmaOS core utilities");
    println!("Commands: ls cat echo pwd mkdir rm cp mv touch stat");
    println!("          grep head tail wc uname sleep env id whoami yes");
    println!("Usage: sigma-coreutils <cmd> [args...]");
    println!("   or: symlink to a command name");
}
