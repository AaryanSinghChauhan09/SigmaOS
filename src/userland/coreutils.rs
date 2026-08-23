extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;
use core::cmp::Ordering;

pub enum CoreutilError {
    FileNotFound(String),
    PermissionDenied,
    InvalidArgument,
}

pub struct ConsoleOut;

impl ConsoleOut {
    pub fn write_str(&self, _s: &str) {}
}

/// Base abstract trait representing a single executable system utility (OOP Abstraction)
pub trait SovereignUtility {
    fn name(&self) -> &'static str;
    fn execute(&self, args: &[&str]) -> Result<(), &'static str>;
}

pub struct LsUtility;
impl SovereignUtility for LsUtility {
    fn name(&self) -> &'static str { "ls" }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct CatUtility;
impl SovereignUtility for CatUtility {
    fn name(&self) -> &'static str { "cat" }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct GrepUtility;
impl SovereignUtility for GrepUtility {
    fn name(&self) -> &'static str { "grep" }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct PsUtility;
impl SovereignUtility for PsUtility {
    fn name(&self) -> &'static str { "ps" }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct NetcfgUtility;
impl SovereignUtility for NetcfgUtility {
    fn name(&self) -> &'static str { "netcfg" }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct PerfUtility;
impl SovereignUtility for PerfUtility {
    fn name(&self) -> &'static str { "perf" }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct DrawUtility;
impl SovereignUtility for DrawUtility {
    fn name(&self) -> &'static str { "draw" }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct PlayUtility;
impl SovereignUtility for PlayUtility {
    fn name(&self) -> &'static str { "play" }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct ThemeUtility;
impl SovereignUtility for ThemeUtility {
    fn name(&self) -> &'static str { "theme" }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct MultiCallManager;
impl MultiCallManager {
    pub fn dispatch(cmd: &str, args: &[&str]) -> Result<(), &'static str> {
        match cmd {
            "ls" => LsUtility.execute(args),
            "cat" => CatUtility.execute(args),
            "grep" => GrepUtility.execute(args),
            "ps" => PsUtility.execute(args),
            "netcfg" => NetcfgUtility.execute(args),
            "perf" => PerfUtility.execute(args),
            "draw" => DrawUtility.execute(args),
            "play" => PlayUtility.execute(args),
            "theme" => ThemeUtility.execute(args),
            _ => Err("Unknown sovereign utility command"),
        }
    }
}

pub struct Coreutils;

impl Coreutils {
    pub fn ls(args: &[String]) -> Result<Vec<String>, CoreutilError> {
        let mut results = Vec::new();
        let show_hidden = args.iter().any(|a| a == "-a");
        let long_format = args.iter().any(|a| a == "-l");
        
        let mut files = vec!["file1.txt", "file2.txt", ".hidden"];
        files.sort();

        for f in files {
            if f.starts_with('.') && !show_hidden { continue; }
            if long_format {
                results.push(format!("-rw-r--r-- 1 root root {:>6} Jan 01 00:00 {}", 1024, f));
            } else {
                results.push(f.to_string());
            }
        }
        
        Ok(results)
    }

    pub fn wc(args: &[String], content: &str) -> Result<String, CoreutilError> {
        let lines = content.lines().count();
        let words = content.split_whitespace().count();
        let bytes = content.len();
        
        if args.iter().any(|a| a == "-l") {
            Ok(format!("{}", lines))
        } else if args.iter().any(|a| a == "-w") {
            Ok(format!("{}", words))
        } else if args.iter().any(|a| a == "-c") {
            Ok(format!("{}", bytes))
        } else {
            Ok(format!("{:>8} {:>8} {:>8}", lines, words, bytes))
        }
    }

    pub fn sort(lines: Vec<String>, args: &[String]) -> Result<Vec<String>, CoreutilError> {
        let mut sorted = lines;
        let reverse = args.iter().any(|a| a == "-r");
        let numeric = args.iter().any(|a| a == "-n");

        sorted.sort_by(|a, b| {
            let cmp = if numeric {
                let num_a = a.trim().parse::<f64>().unwrap_or(0.0);
                let num_b = b.trim().parse::<f64>().unwrap_or(0.0);
                num_a.partial_cmp(&num_b).unwrap_or(Ordering::Equal)
            } else {
                a.cmp(b)
            };
            if reverse { cmp.reverse() } else { cmp }
        });

        Ok(sorted)
    }

    pub fn chmod(mode: &str, _file: &str) -> Result<(), CoreutilError> {
        if mode.chars().all(|c| c.is_digit(8)) {
            let _octal_val = u32::from_str_radix(mode, 8).map_err(|_| CoreutilError::InvalidArgument)?;
            return Ok(());
        }
        
        let mut chars = mode.chars();
        let who = chars.next().unwrap_or('a');
        let op = chars.next().unwrap_or('+');
        let perm = chars.next().unwrap_or('x');
        
        if !['u','g','o','a'].contains(&who) || !['+','-','='].contains(&op) || !['r','w','x'].contains(&perm) {
            return Err(CoreutilError::InvalidArgument);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multicall_dispatch() {
        assert!(MultiCallManager::dispatch("ls", &[]).is_ok());
        assert!(MultiCallManager::dispatch("cat", &[]).is_ok());
        assert!(MultiCallManager::dispatch("unknown", &[]).is_err());
    }
}
