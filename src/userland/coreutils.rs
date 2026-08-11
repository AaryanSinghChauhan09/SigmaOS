#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use core::cmp::Ordering;

pub enum CoreutilError {
    FileNotFound(String),
    PermissionDenied,
    InvalidArgument,
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
                // Mode, links, owner, group, size, date, name
                results.push(alloc::format!("-rw-r--r-- 1 root root {:>6} Jan 01 00:00 {}", 1024, f));
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
            Ok(alloc::format!("{}", lines))
        } else if args.iter().any(|a| a == "-w") {
            Ok(alloc::format!("{}", words))
        } else if args.iter().any(|a| a == "-c") {
            Ok(alloc::format!("{}", bytes))
        } else {
            Ok(alloc::format!("{:>8} {:>8} {:>8}", lines, words, bytes))
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

    pub fn chmod(mode: &str, file: &str) -> Result<(), CoreutilError> {
        // Octal parse
        if mode.chars().all(|c| c.is_digit(8)) {
            let _octal_val = u32::from_str_radix(mode, 8).map_err(|_| CoreutilError::InvalidArgument)?;
            // Apply octal_val
            return Ok(());
        }
        
        // Symbolic parse (e.g. u+x, g-w)
        let mut chars = mode.chars();
        let who = chars.next().unwrap_or('a');
        let op = chars.next().unwrap_or('+');
        let perm = chars.next().unwrap_or('x');
        
        if !['u','g','o','a'].contains(&who) || !['+','-','='].contains(&op) || !['r','w','x'].contains(&perm) {
            return Err(CoreutilError::InvalidArgument);
        }
        
        // Apply symbolic mode
        Ok(())
    }
}
