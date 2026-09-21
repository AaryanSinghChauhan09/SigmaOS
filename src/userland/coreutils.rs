use std::collections::HashMap;
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;
use core::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreutilFormat {
    GnuCoreutils,
    BsdCoreutils,
    BusyBoxApplet,
    ToyboxApplet,
    UutilsRust,
    SovereignNative,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreutilError {
    FileNotFound(String),
    PermissionDenied,
    InvalidArgument(String),
    ExecutionFailed(String),
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
    fn name(&self) -> &'static str {
        "ls"
    }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct CatUtility;
impl SovereignUtility for CatUtility {
    fn name(&self) -> &'static str {
        "cat"
    }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct GrepUtility;
impl SovereignUtility for GrepUtility {
    fn name(&self) -> &'static str {
        "grep"
    }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct PsUtility;
impl SovereignUtility for PsUtility {
    fn name(&self) -> &'static str {
        "ps"
    }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct NetcfgUtility;
impl SovereignUtility for NetcfgUtility {
    fn name(&self) -> &'static str {
        "netcfg"
    }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct PerfUtility;
impl SovereignUtility for PerfUtility {
    fn name(&self) -> &'static str {
        "perf"
    }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct DrawUtility;
impl SovereignUtility for DrawUtility {
    fn name(&self) -> &'static str {
        "draw"
    }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct PlayUtility;
impl SovereignUtility for PlayUtility {
    fn name(&self) -> &'static str {
        "play"
    }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct ThemeUtility;
impl SovereignUtility for ThemeUtility {
    fn name(&self) -> &'static str {
        "theme"
    }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreutilOptions {
    pub format: CoreutilFormat,
    pub flags: Vec<String>,
    pub key_value_options: HashMap<String, String>,
    pub positional_args: Vec<String>,
}

impl CoreutilOptions {
    pub fn parse(format: CoreutilFormat, args: &[String]) -> Self {
        let mut flags = Vec::new();
        let mut key_value_options = HashMap::new();
        let mut positional_args = Vec::new();

        let mut idx = 0;
        while idx < args.len() {
            let arg = &args[idx];
            if arg.starts_with("--") {
                if let Some(eq_pos) = arg.find('=') {
                    let key = arg[2..eq_pos].to_string();
                    let val = arg[eq_pos + 1..].to_string();
                    key_value_options.insert(key.clone(), val);
                    flags.push(key);
                } else {
                    let key = arg[2..].to_string();
                    if idx + 1 < args.len() && !args[idx + 1].starts_with('-') && ["lines", "delimiter", "fields"].contains(&key.as_str()) {
                        key_value_options.insert(key.clone(), args[idx + 1].clone());
                        flags.push(key);
                        idx += 1;
                    } else {
                        flags.push(key);
                    }
                }
            } else if arg.starts_with('-') && arg.len() > 1 {
                let opt_key = &arg[1..];
                if (opt_key == "d" || opt_key == "f") && idx + 1 < args.len() && !args[idx + 1].starts_with('-') {
                    key_value_options.insert(opt_key.to_string(), args[idx + 1].clone());
                    flags.push(opt_key.to_string());
                    idx += 1;
                } else if opt_key == "n" && idx + 1 < args.len() && args[idx + 1].parse::<usize>().is_ok() {
                    key_value_options.insert(opt_key.to_string(), args[idx + 1].clone());
                    flags.push(opt_key.to_string());
                    idx += 1;
                } else if opt_key.len() > 1 && (opt_key.starts_with('n') || opt_key.starts_with('d') || opt_key.starts_with('f')) {
                    let k = opt_key[0..1].to_string();
                    let v = opt_key[1..].to_string();
                    key_value_options.insert(k.clone(), v);
                    flags.push(k);
                } else {
                    for ch in arg[1..].chars() {
                        flags.push(ch.to_string());
                    }
                }
            } else {
                positional_args.push(arg.clone());
            }
            idx += 1;
        }

        Self {
            format,
            flags,
            key_value_options,
            positional_args,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreutilExecutionResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub struct UniversalCoreutilsEngine;

impl UniversalCoreutilsEngine {
    pub fn infer_format(cmd: &str) -> (CoreutilFormat, &str) {
        if let Some(stripped) = cmd.strip_prefix("gnu-") {
            (CoreutilFormat::GnuCoreutils, stripped)
        } else if let Some(stripped) = cmd.strip_prefix("bsd-") {
            (CoreutilFormat::BsdCoreutils, stripped)
        } else if let Some(stripped) = cmd.strip_prefix("busybox-") {
            (CoreutilFormat::BusyBoxApplet, stripped)
        } else if let Some(stripped) = cmd.strip_prefix("toybox-") {
            (CoreutilFormat::ToyboxApplet, stripped)
        } else if let Some(stripped) = cmd.strip_prefix("uutils-") {
            (CoreutilFormat::UutilsRust, stripped)
        } else {
            (CoreutilFormat::SovereignNative, cmd)
        }
    }

    pub fn execute(cmd: &str, args: &[String]) -> Result<CoreutilExecutionResult, CoreutilError> {
        let (format, base_cmd) = Self::infer_format(cmd);
        let opts = CoreutilOptions::parse(format, args);

        match base_cmd {
            "echo" => Self::execute_echo(&opts),
            "head" => Self::execute_head(&opts),
            "tail" => Self::execute_tail(&opts),
            "uniq" => Self::execute_uniq(&opts),
            "cut" => Self::execute_cut(&opts),
            "tr" => Self::execute_tr(&opts),
            "stat" => Self::execute_stat(&opts),
            "df" => Self::execute_df(&opts),
            "du" => Self::execute_du(&opts),
            "uname" => Self::execute_uname(&opts),
            "env" => Self::execute_env(&opts),
            "ls" => {
                let items = Coreutils::ls(args)?;
                Ok(CoreutilExecutionResult {
                    exit_code: 0,
                    stdout: items.join("\n"),
                    stderr: String::new(),
                })
            }
            "wc" => {
                let content = opts.positional_args.first().cloned().unwrap_or_default();
                let output = Coreutils::wc(args, &content)?;
                Ok(CoreutilExecutionResult {
                    exit_code: 0,
                    stdout: output,
                    stderr: String::new(),
                })
            }
            _ => Err(CoreutilError::InvalidArgument(format!("Unsupported coreutil command: {}", cmd))),
        }
    }

    fn execute_echo(opts: &CoreutilOptions) -> Result<CoreutilExecutionResult, CoreutilError> {
        let trailing_newline = !opts.flags.contains(&"n".to_string());
        let interpret_escapes = opts.flags.contains(&"e".to_string());
        let mut text = opts.positional_args.join(" ");

        if interpret_escapes {
            text = text
                .replace("\\n", "\n")
                .replace("\\t", "\t")
                .replace("\\\\", "\\");
        }

        if trailing_newline {
            text.push('\n');
        }

        Ok(CoreutilExecutionResult {
            exit_code: 0,
            stdout: text,
            stderr: String::new(),
        })
    }

    fn execute_head(opts: &CoreutilOptions) -> Result<CoreutilExecutionResult, CoreutilError> {
        let lines_cnt: usize = opts
            .key_value_options
            .get("n")
            .or_else(|| opts.key_value_options.get("lines"))
            .and_then(|v| v.parse().ok())
            .unwrap_or(10);

        let input = opts.positional_args.first().cloned().unwrap_or_default();
        let taken: Vec<&str> = input.lines().take(lines_cnt).collect();
        let mut stdout = taken.join("\n");
        if !stdout.is_empty() {
            stdout.push('\n');
        }

        Ok(CoreutilExecutionResult {
            exit_code: 0,
            stdout,
            stderr: String::new(),
        })
    }

    fn execute_tail(opts: &CoreutilOptions) -> Result<CoreutilExecutionResult, CoreutilError> {
        let lines_cnt: usize = opts
            .key_value_options
            .get("n")
            .or_else(|| opts.key_value_options.get("lines"))
            .and_then(|v| v.parse().ok())
            .unwrap_or(10);

        let input = opts.positional_args.first().cloned().unwrap_or_default();
        let all_lines: Vec<&str> = input.lines().collect();
        let start = if all_lines.len() > lines_cnt {
            all_lines.len() - lines_cnt
        } else {
            0
        };
        let mut stdout = all_lines[start..].join("\n");
        if !stdout.is_empty() {
            stdout.push('\n');
        }

        Ok(CoreutilExecutionResult {
            exit_code: 0,
            stdout,
            stderr: String::new(),
        })
    }

    fn execute_uniq(opts: &CoreutilOptions) -> Result<CoreutilExecutionResult, CoreutilError> {
        let count_flag = opts.flags.contains(&"c".to_string()) || opts.flags.contains(&"count".to_string());
        let input = opts.positional_args.first().cloned().unwrap_or_default();

        let mut lines_out = Vec::new();
        let lines: Vec<&str> = input.lines().collect();

        let mut idx = 0;
        while idx < lines.len() {
            let line = lines[idx];
            let mut count = 1;
            while idx + 1 < lines.len() && lines[idx + 1] == line {
                count += 1;
                idx += 1;
            }
            if count_flag {
                lines_out.push(format!("{:>7} {}", count, line));
            } else {
                lines_out.push(line.to_string());
            }
            idx += 1;
        }

        let mut stdout = lines_out.join("\n");
        if !stdout.is_empty() {
            stdout.push('\n');
        }

        Ok(CoreutilExecutionResult {
            exit_code: 0,
            stdout,
            stderr: String::new(),
        })
    }

    fn execute_cut(opts: &CoreutilOptions) -> Result<CoreutilExecutionResult, CoreutilError> {
        let delimiter = opts
            .key_value_options
            .get("d")
            .or_else(|| opts.key_value_options.get("delimiter"))
            .cloned()
            .unwrap_or_else(|| "\t".to_string());

        let field_idx: usize = opts
            .key_value_options
            .get("f")
            .or_else(|| opts.key_value_options.get("fields"))
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);

        let input = opts.positional_args.first().cloned().unwrap_or_default();
        let mut lines_out = Vec::new();

        for line in input.lines() {
            let parts: Vec<&str> = line.split(&delimiter).collect();
            if field_idx > 0 && field_idx <= parts.len() {
                lines_out.push(parts[field_idx - 1].to_string());
            } else {
                lines_out.push(String::new());
            }
        }

        let mut stdout = lines_out.join("\n");
        if !stdout.is_empty() {
            stdout.push('\n');
        }

        Ok(CoreutilExecutionResult {
            exit_code: 0,
            stdout,
            stderr: String::new(),
        })
    }

    fn execute_tr(opts: &CoreutilOptions) -> Result<CoreutilExecutionResult, CoreutilError> {
        let delete_flag = opts.flags.contains(&"d".to_string()) || opts.flags.contains(&"delete".to_string());
        if opts.positional_args.is_empty() {
            return Err(CoreutilError::InvalidArgument("tr requires set arguments".to_string()));
        }

        let set1 = &opts.positional_args[0];
        let mut input = if opts.positional_args.len() > 1 {
            opts.positional_args[1].clone()
        } else {
            String::new()
        };

        if delete_flag {
            for ch in set1.chars() {
                input = input.replace(ch, "");
            }
        } else if opts.positional_args.len() >= 2 {
            let set2 = &opts.positional_args[1];
            if opts.positional_args.len() >= 3 {
                input = opts.positional_args[2].clone();
            }
            let chars1: Vec<char> = set1.chars().collect();
            let chars2: Vec<char> = set2.chars().collect();
            let mut tr_map = HashMap::new();
            for (i, &c1) in chars1.iter().enumerate() {
                let c2 = if i < chars2.len() {
                    chars2[i]
                } else {
                    *chars2.last().unwrap_or(&c1)
                };
                tr_map.insert(c1, c2);
            }
            input = input
                .chars()
                .map(|ch| *tr_map.get(&ch).unwrap_or(&ch))
                .collect();
        }

        Ok(CoreutilExecutionResult {
            exit_code: 0,
            stdout: input,
            stderr: String::new(),
        })
    }

    fn execute_stat(opts: &CoreutilOptions) -> Result<CoreutilExecutionResult, CoreutilError> {
        let path = opts.positional_args.first().cloned().unwrap_or_else(|| ".".to_string());
        let is_bsd = opts.format == CoreutilFormat::BsdCoreutils;

        let output = if is_bsd {
            format!("16777220 12345678 -rw-r--r-- 1 root wheel 0 4096 0 \"{}\"", path)
        } else {
            format!("  File: {}\n  Size: 4096       Blocks: 8          IO Block: 4096   regular file\nDevice: 0/1   Inode: 12345678    Links: 1\nAccess: (0644/-rw-r--r--)  Uid: ( 0/  root)   Gid: ( 0/  root)", path)
        };

        Ok(CoreutilExecutionResult {
            exit_code: 0,
            stdout: output,
            stderr: String::new(),
        })
    }

    fn execute_df(opts: &CoreutilOptions) -> Result<CoreutilExecutionResult, CoreutilError> {
        let human = opts.flags.contains(&"h".to_string()) || opts.flags.contains(&"human-readable".to_string());
        let stdout = if human {
            "Filesystem     Type      Size  Used Avail Use% Mounted on\n/dev/sda1      sigmafs   100G   20G   80G  20% /\n"
        } else {
            "Filesystem     1K-blocks     Used Available Use% Mounted on\n/dev/sda1      104857600 20971520  83886080  20% /\n"
        }.to_string();

        Ok(CoreutilExecutionResult {
            exit_code: 0,
            stdout,
            stderr: String::new(),
        })
    }

    fn execute_du(opts: &CoreutilOptions) -> Result<CoreutilExecutionResult, CoreutilError> {
        let path = opts.positional_args.first().cloned().unwrap_or_else(|| ".".to_string());
        let human = opts.flags.contains(&"h".to_string()) || opts.flags.contains(&"human-readable".to_string());
        let summary = opts.flags.contains(&"s".to_string()) || opts.flags.contains(&"summarize".to_string());

        let stdout = match (human, summary) {
            (true, true) => format!("4.0M\t{}\n", path),
            (true, false) => format!("4.0K\t{}/file.txt\n4.0M\t{}\n", path, path),
            (false, true) => format!("4096\t{}\n", path),
            (false, false) => format!("4\t{}/file.txt\n4096\t{}\n", path, path),
        };

        Ok(CoreutilExecutionResult {
            exit_code: 0,
            stdout,
            stderr: String::new(),
        })
    }

    fn execute_uname(opts: &CoreutilOptions) -> Result<CoreutilExecutionResult, CoreutilError> {
        let all = opts.flags.contains(&"a".to_string()) || opts.flags.contains(&"all".to_string());
        let stdout = if all {
            format!(
                "SigmaOS sovereign-node 1.0.0-sigma #1 SMP x86_64 GNU/Linux ({:?})\n",
                opts.format
            )
        } else {
            "SigmaOS\n".to_string()
        };

        Ok(CoreutilExecutionResult {
            exit_code: 0,
            stdout,
            stderr: String::new(),
        })
    }

    fn execute_env(_opts: &CoreutilOptions) -> Result<CoreutilExecutionResult, CoreutilError> {
        let stdout = "PATH=/bin:/usr/bin:/sbin:/usr/sbin\nUSER=root\nSHELL=/bin/sigma-sh\nTERM=xterm-256color\n".to_string();
        Ok(CoreutilExecutionResult {
            exit_code: 0,
            stdout,
            stderr: String::new(),
        })
    }
}

pub struct MultiCallManager;
impl MultiCallManager {
    pub fn dispatch(cmd: &str, args: &[&str]) -> Result<(), &'static str> {
        let clean_cmd = cmd
            .strip_prefix("gnu-")
            .or_else(|| cmd.strip_prefix("bsd-"))
            .or_else(|| cmd.strip_prefix("busybox-"))
            .or_else(|| cmd.strip_prefix("toybox-"))
            .or_else(|| cmd.strip_prefix("uutils-"))
            .unwrap_or(cmd);

        let string_args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        if let Ok(_res) = UniversalCoreutilsEngine::execute(cmd, &string_args) {
            return Ok(());
        }

        match clean_cmd {
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
        let show_hidden = args.iter().any(|a| a == "-a" || a == "--all");
        let long_format = args.iter().any(|a| a == "-l");

        let mut files = vec!["file1.txt", "file2.txt", ".hidden"];
        files.sort();

        for f in files {
            if f.starts_with('.') && !show_hidden {
                continue;
            }
            if long_format {
                results.push(format!(
                    "-rw-r--r-- 1 root root {:>6} Jan 01 00:00 {}",
                    1024, f
                ));
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

        if args.iter().any(|a| a == "-l" || a == "--lines") {
            Ok(format!("{}", lines))
        } else if args.iter().any(|a| a == "-w" || a == "--words") {
            Ok(format!("{}", words))
        } else if args.iter().any(|a| a == "-c" || a == "--bytes") {
            Ok(format!("{}", bytes))
        } else {
            Ok(format!("{:>8} {:>8} {:>8}", lines, words, bytes))
        }
    }

    pub fn sort(lines: Vec<String>, args: &[String]) -> Result<Vec<String>, CoreutilError> {
        let mut sorted = lines;
        let reverse = args.iter().any(|a| a == "-r" || a == "--reverse");
        let numeric = args.iter().any(|a| a == "-n" || a == "--numeric-sort");

        sorted.sort_by(|a, b| {
            let cmp = if numeric {
                let num_a = a.trim().parse::<f64>().unwrap_or(0.0);
                let num_b = b.trim().parse::<f64>().unwrap_or(0.0);
                num_a.partial_cmp(&num_b).unwrap_or(Ordering::Equal)
            } else {
                a.cmp(b)
            };
            if reverse {
                cmp.reverse()
            } else {
                cmp
            }
        });

        Ok(sorted)
    }

    pub fn chmod(mode: &str, _file: &str) -> Result<(), CoreutilError> {
        if mode.chars().all(|c| c.is_digit(8)) {
            let _octal_val =
                u32::from_str_radix(mode, 8).map_err(|_| CoreutilError::InvalidArgument("Invalid octal mode".to_string()))?;
            return Ok(());
        }

        let mut chars = mode.chars();
        let who = chars.next().unwrap_or('a');
        let op = chars.next().unwrap_or('+');
        let perm = chars.next().unwrap_or('x');

        if !['u', 'g', 'o', 'a'].contains(&who)
            || !['+', '-', '='].contains(&op)
            || !['r', 'w', 'x'].contains(&perm)
        {
            return Err(CoreutilError::InvalidArgument("Invalid symbolic mode".to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_inference() {
        assert_eq!(UniversalCoreutilsEngine::infer_format("gnu-ls"), (CoreutilFormat::GnuCoreutils, "ls"));
        assert_eq!(UniversalCoreutilsEngine::infer_format("bsd-head"), (CoreutilFormat::BsdCoreutils, "head"));
        assert_eq!(UniversalCoreutilsEngine::infer_format("busybox-echo"), (CoreutilFormat::BusyBoxApplet, "echo"));
        assert_eq!(UniversalCoreutilsEngine::infer_format("toybox-tail"), (CoreutilFormat::ToyboxApplet, "tail"));
        assert_eq!(UniversalCoreutilsEngine::infer_format("uutils-cat"), (CoreutilFormat::UutilsRust, "cat"));
        assert_eq!(UniversalCoreutilsEngine::infer_format("uname"), (CoreutilFormat::SovereignNative, "uname"));
    }

    #[test]
    fn test_universal_echo_execution() {
        let res = UniversalCoreutilsEngine::execute("echo", &["-n".to_string(), "hello world".to_string()]).unwrap();
        assert_eq!(res.stdout, "hello world");

        let res_escapes = UniversalCoreutilsEngine::execute("gnu-echo", &["-e".to_string(), "hello\\nworld".to_string()]).unwrap();
        assert_eq!(res_escapes.stdout, "hello\nworld\n");
    }

    #[test]
    fn test_head_and_tail_execution() {
        let sample = "line1\nline2\nline3\nline4\nline5".to_string();
        let head_res = UniversalCoreutilsEngine::execute("head", &["-n".to_string(), "2".to_string(), sample.clone()]).unwrap();
        assert_eq!(head_res.stdout, "line1\nline2\n");

        let tail_res = UniversalCoreutilsEngine::execute("tail", &["-n".to_string(), "2".to_string(), sample]).unwrap();
        assert_eq!(tail_res.stdout, "line4\nline5\n");
    }

    #[test]
    fn test_uniq_and_cut_execution() {
        let sample_uniq = "apple\napple\nbanana".to_string();
        let uniq_res = UniversalCoreutilsEngine::execute("uniq", &["-c".to_string(), sample_uniq]).unwrap();
        assert!(uniq_res.stdout.contains("2 apple"));

        let sample_cut = "user:x:1000:1000".to_string();
        let cut_res = UniversalCoreutilsEngine::execute("cut", &["-d".to_string(), ":".to_string(), "-f".to_string(), "1".to_string(), sample_cut]).unwrap();
        assert_eq!(cut_res.stdout, "user\n");
    }

    #[test]
    fn test_multicall_dispatch() {
        assert!(MultiCallManager::dispatch("gnu-echo", &["hello"]).is_ok());
        assert!(MultiCallManager::dispatch("bsd-stat", &["/etc/passwd"]).is_ok());
        assert!(MultiCallManager::dispatch("ls", &[]).is_ok());
        assert!(MultiCallManager::dispatch("cat", &[]).is_ok());
        assert!(MultiCallManager::dispatch("unknown_cmd_xyz", &[]).is_err());
    }

    #[test]
    fn test_stat_df_du_uname_env_execution() {
        let stat_res = UniversalCoreutilsEngine::execute("bsd-stat", &["/etc/passwd".to_string()]).unwrap();
        assert!(stat_res.stdout.contains("wheel"));

        let df_res = UniversalCoreutilsEngine::execute("gnu-df", &["-h".to_string()]).unwrap();
        assert!(df_res.stdout.contains("Filesystem"));

        let du_res = UniversalCoreutilsEngine::execute("du", &["-sh".to_string(), "/var/log".to_string()]).unwrap();
        assert!(du_res.stdout.contains("/var/log"));

        let uname_res = UniversalCoreutilsEngine::execute("uname", &["-a".to_string()]).unwrap();
        assert!(uname_res.stdout.contains("SigmaOS"));

        let env_res = UniversalCoreutilsEngine::execute("env", &[]).unwrap();
        assert!(env_res.stdout.contains("PATH="));
    }
}
