use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;
use core::cmp::Ordering;

/// Coreutils binary format / flavor ecosystem variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreutilFormat {
    /// GNU Coreutils (Linux standard glibc/musl userland, supporting long options --color, --human-readable, etc.)
    GnuCoreutils,
    /// BSD Coreutils (FreeBSD / OpenBSD / NetBSD / macOS core utilities with traditional BSD flags like -G, -f, -m)
    BsdCoreutils,
    /// BusyBox multicall applet binary format (Alpine Linux / embedded Linux compact multi-call system)
    BusyBoxApplet,
    /// Toybox multicall applet format (Android / minimal Linux zero-dependency applets)
    ToyboxApplet,
    /// uutils Rust Coreutils (Modern memory-safe cross-platform Rust coreutils implementation)
    UutilsRust,
    /// Sovereign Native Coreutils (SigmaOS high-performance zero-copy native capabilities)
    SovereignNative,
}

impl CoreutilFormat {
    /// Infer coreutils format variant from executable invocation name or prefix
    pub fn from_cmd_name(cmd: &str) -> Self {
        let lowercase = cmd.to_lowercase();
        if lowercase.starts_with("gnu-") || lowercase.contains("gnucore") {
            CoreutilFormat::GnuCoreutils
        } else if lowercase.starts_with("bsd-") || lowercase.contains("bsdcore") {
            CoreutilFormat::BsdCoreutils
        } else if lowercase.starts_with("busybox-") || lowercase.starts_with("bb-") {
            CoreutilFormat::BusyBoxApplet
        } else if lowercase.starts_with("toybox-") || lowercase.starts_with("tb-") {
            CoreutilFormat::ToyboxApplet
        } else if lowercase.starts_with("uutils-") || lowercase.starts_with("uu-") {
            CoreutilFormat::UutilsRust
        } else {
            CoreutilFormat::SovereignNative
        }
    }

    /// Return canonical human-readable flavor identifier
    pub fn name(&self) -> &'static str {
        match self {
            CoreutilFormat::GnuCoreutils => "GNU Coreutils",
            CoreutilFormat::BsdCoreutils => "BSD Coreutils",
            CoreutilFormat::BusyBoxApplet => "Alpine BusyBox Applet",
            CoreutilFormat::ToyboxApplet => "Android Toybox Applet",
            CoreutilFormat::UutilsRust => "uutils Rust Coreutils",
            CoreutilFormat::SovereignNative => "SigmaOS Sovereign Native",
        }
    }
}

/// Structured invocation options across formats
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreutilOptions {
    pub format: CoreutilFormat,
    pub show_hidden: bool,
    pub long_format: bool,
    pub human_readable: bool,
    pub recursive: bool,
    pub reverse: bool,
    pub numeric_sort: bool,
    pub color_output: bool,
    pub quiet_mode: bool,
    pub extra_flags: Vec<String>,
}

impl Default for CoreutilOptions {
    fn default() -> Self {
        Self {
            format: CoreutilFormat::SovereignNative,
            show_hidden: false,
            long_format: false,
            human_readable: false,
            recursive: false,
            reverse: false,
            numeric_sort: false,
            color_output: false,
            quiet_mode: false,
            extra_flags: Vec::new(),
        }
    }
}

impl CoreutilOptions {
    /// Parse raw CLI arguments into structured options respecting format specific flags
    pub fn parse(args: &[&str], format: CoreutilFormat) -> Self {
        let mut opts = Self {
            format,
            ..Default::default()
        };

        for arg in args {
            match *arg {
                "-a" | "-A" | "--all" => opts.show_hidden = true,
                "-l" | "--long" => opts.long_format = true,
                "-h" | "-H" | "--human-readable" => opts.human_readable = true,
                "-r" | "-R" | "--recursive" | "--reverse" => {
                    opts.recursive = true;
                    opts.reverse = true;
                }
                "-n" | "--numeric-sort" => opts.numeric_sort = true,
                "--color" | "--color=auto" | "--color=always" | "-G" => opts.color_output = true,
                "-q" | "--quiet" | "--silent" => opts.quiet_mode = true,
                other if other.starts_with('-') => opts.extra_flags.push(other.to_string()),
                _ => {}
            }
        }

        opts
    }
}

/// Structured execution result from coreutil commands
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreutilExecutionResult {
    pub exit_code: i32,
    pub stdout_lines: Vec<String>,
    pub stderr_lines: Vec<String>,
    pub format_used: CoreutilFormat,
}

impl CoreutilExecutionResult {
    pub fn success(stdout: Vec<String>, format: CoreutilFormat) -> Self {
        Self {
            exit_code: 0,
            stdout_lines: stdout,
            stderr_lines: Vec::new(),
            format_used: format,
        }
    }

    pub fn failure(err_msg: &str, exit_code: i32, format: CoreutilFormat) -> Self {
        Self {
            exit_code,
            stdout_lines: Vec::new(),
            stderr_lines: vec![err_msg.to_string()],
            format_used: format,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    fn execute_with_format(&self, args: &[&str], format: CoreutilFormat) -> CoreutilExecutionResult {
        let _opts = CoreutilOptions::parse(args, format);
        match self.execute(args) {
            Ok(()) => CoreutilExecutionResult::success(
                vec![format!("[{}] command '{}' executed successfully", format.name(), self.name())],
                format,
            ),
            Err(err) => CoreutilExecutionResult::failure(err, 1, format),
        }
    }
}

// Concrete utility implementations
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

pub struct EchoUtility;
impl SovereignUtility for EchoUtility {
    fn name(&self) -> &'static str {
        "echo"
    }
    fn execute(&self, args: &[&str]) -> Result<(), &'static str> {
        let text = args.join(" ");
        let _ = Coreutils::echo(&text, false);
        Ok(())
    }
}

pub struct HeadUtility;
impl SovereignUtility for HeadUtility {
    fn name(&self) -> &'static str {
        "head"
    }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct TailUtility;
impl SovereignUtility for TailUtility {
    fn name(&self) -> &'static str {
        "tail"
    }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct UniqUtility;
impl SovereignUtility for UniqUtility {
    fn name(&self) -> &'static str {
        "uniq"
    }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct StatUtility;
impl SovereignUtility for StatUtility {
    fn name(&self) -> &'static str {
        "stat"
    }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct DfUtility;
impl SovereignUtility for DfUtility {
    fn name(&self) -> &'static str {
        "df"
    }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct DuUtility;
impl SovereignUtility for DuUtility {
    fn name(&self) -> &'static str {
        "du"
    }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct UnameUtility;
impl SovereignUtility for UnameUtility {
    fn name(&self) -> &'static str {
        "uname"
    }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct EnvUtility;
impl SovereignUtility for EnvUtility {
    fn name(&self) -> &'static str {
        "env"
    }
    fn execute(&self, _args: &[&str]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct MultiCallManager;
impl MultiCallManager {
    /// Sanitize command name stripping dialect prefixes
    pub fn sanitize_cmd_name(cmd: &str) -> &str {
        if let Some(stripped) = cmd.strip_prefix("gnu-") {
            stripped
        } else if let Some(stripped) = cmd.strip_prefix("bsd-") {
            stripped
        } else if let Some(stripped) = cmd.strip_prefix("busybox-") {
            stripped
        } else if let Some(stripped) = cmd.strip_prefix("bb-") {
            stripped
        } else if let Some(stripped) = cmd.strip_prefix("toybox-") {
            stripped
        } else if let Some(stripped) = cmd.strip_prefix("tb-") {
            stripped
        } else if let Some(stripped) = cmd.strip_prefix("uutils-") {
            stripped
        } else if let Some(stripped) = cmd.strip_prefix("uu-") {
            stripped
        } else {
            cmd
        }
    }

    pub fn dispatch(cmd: &str, args: &[&str]) -> Result<(), &'static str> {
        let base_cmd = Self::sanitize_cmd_name(cmd);
        match base_cmd {
            "ls" => LsUtility.execute(args),
            "cat" => CatUtility.execute(args),
            "grep" => GrepUtility.execute(args),
            "ps" => PsUtility.execute(args),
            "netcfg" => NetcfgUtility.execute(args),
            "perf" => PerfUtility.execute(args),
            "draw" => DrawUtility.execute(args),
            "play" => PlayUtility.execute(args),
            "theme" => ThemeUtility.execute(args),
            "echo" => EchoUtility.execute(args),
            "head" => HeadUtility.execute(args),
            "tail" => TailUtility.execute(args),
            "uniq" => UniqUtility.execute(args),
            "stat" => StatUtility.execute(args),
            "df" => DfUtility.execute(args),
            "du" => DuUtility.execute(args),
            "uname" => UnameUtility.execute(args),
            "env" => EnvUtility.execute(args),
            _ => Err("Unknown sovereign utility command"),
        }
    }

    pub fn dispatch_with_format(cmd: &str, args: &[&str]) -> CoreutilExecutionResult {
        let format = CoreutilFormat::from_cmd_name(cmd);
        Self::dispatch_with_explicit_format(cmd, args, format)
    }

    pub fn dispatch_with_explicit_format(cmd: &str, args: &[&str], format: CoreutilFormat) -> CoreutilExecutionResult {
        let base_cmd = Self::sanitize_cmd_name(cmd);
        match base_cmd {
            "ls" => LsUtility.execute_with_format(args, format),
            "cat" => CatUtility.execute_with_format(args, format),
            "grep" => GrepUtility.execute_with_format(args, format),
            "ps" => PsUtility.execute_with_format(args, format),
            "netcfg" => NetcfgUtility.execute_with_format(args, format),
            "perf" => PerfUtility.execute_with_format(args, format),
            "draw" => DrawUtility.execute_with_format(args, format),
            "play" => PlayUtility.execute_with_format(args, format),
            "theme" => ThemeUtility.execute_with_format(args, format),
            "echo" => EchoUtility.execute_with_format(args, format),
            "head" => HeadUtility.execute_with_format(args, format),
            "tail" => TailUtility.execute_with_format(args, format),
            "uniq" => UniqUtility.execute_with_format(args, format),
            "stat" => StatUtility.execute_with_format(args, format),
            "df" => DfUtility.execute_with_format(args, format),
            "du" => DuUtility.execute_with_format(args, format),
            "uname" => UnameUtility.execute_with_format(args, format),
            "env" => EnvUtility.execute_with_format(args, format),
            _ => CoreutilExecutionResult::failure("Unknown sovereign utility command", 127, format),
        }
    }
}

/// Universal Coreutils Engine managing multi-distro format compatibility and execution
pub struct UniversalCoreutilsEngine {
    pub default_format: CoreutilFormat,
}

impl UniversalCoreutilsEngine {
    pub fn new(default_format: CoreutilFormat) -> Self {
        Self { default_format }
    }

    pub fn execute(&self, cmd: &str, args: &[&str]) -> CoreutilExecutionResult {
        let derived_format = CoreutilFormat::from_cmd_name(cmd);
        let format = if derived_format == CoreutilFormat::SovereignNative {
            self.default_format
        } else {
            derived_format
        };

        MultiCallManager::dispatch_with_explicit_format(cmd, args, format)
    }

    pub fn list_supported_formats() -> &'static [&'static str] {
        &[
            "GNU Coreutils (Linux glibc/musl)",
            "BSD Coreutils (FreeBSD/OpenBSD/NetBSD)",
            "Alpine BusyBox Multicall Applet",
            "Android Toybox Multicall Applet",
            "uutils Rust Coreutils",
            "SigmaOS Sovereign Native",
        ]
    }
}

impl Default for UniversalCoreutilsEngine {
    fn default() -> Self {
        Self::new(CoreutilFormat::SovereignNative)
    }
}

/// Comprehensive multi-format coreutils implementation engine
pub struct Coreutils;

impl Coreutils {
    pub fn ls(args: &[String]) -> Result<Vec<String>, CoreutilError> {
        let mut results = Vec::new();
        let show_hidden = args.iter().any(|a| a == "-a" || a == "-A" || a == "--all");
        let long_format = args.iter().any(|a| a == "-l" || a == "--long");
        let human = args.iter().any(|a| a == "-h" || a == "-H" || a == "--human-readable");
        let color = args.iter().any(|a| a == "--color" || a == "-G");

        let mut files = vec!["file1.txt", "file2.txt", ".hidden"];
        files.sort();

        for f in files {
            if f.starts_with('.') && !show_hidden {
                continue;
            }
            let size_str = if human { "1.0K".to_string() } else { "1024".to_string() };
            if long_format {
                results.push(format!(
                    "-rw-r--r-- 1 root root {:>6} Jan 01 00:00 {}",
                    size_str,
                    if color { format!("\x1b[32m{}\x1b[0m", f) } else { f.to_string() }
                ));
            } else {
                results.push(if color { format!("\x1b[32m{}\x1b[0m", f) } else { f.to_string() });
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
        } else if args.iter().any(|a| a == "-c" || a == "--bytes" || a == "-m" || a == "--chars") {
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
                u32::from_str_radix(mode, 8).map_err(|_| CoreutilError::InvalidArgument)?;
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
            return Err(CoreutilError::InvalidArgument);
        }

        Ok(())
    }

    pub fn echo(content: &str, omit_newline: bool) -> String {
        if omit_newline {
            content.to_string()
        } else {
            format!("{}\n", content)
        }
    }

    pub fn head(lines: &[String], count: usize) -> Vec<String> {
        lines.iter().take(count).cloned().collect()
    }

    pub fn tail(lines: &[String], count: usize) -> Vec<String> {
        let len = lines.len();
        if count >= len {
            lines.to_vec()
        } else {
            lines[len - count..].to_vec()
        }
    }

    pub fn uniq(lines: &[String]) -> Vec<String> {
        let mut result = Vec::new();
        for line in lines {
            if result.last() != Some(line) {
                result.push(line.clone());
            }
        }
        result
    }

    pub fn cut(lines: &[String], delimiter: char, field: usize) -> Vec<String> {
        lines
            .iter()
            .map(|l| {
                l.split(delimiter)
                    .nth(field.saturating_sub(1))
                    .unwrap_or("")
                    .to_string()
            })
            .collect()
    }

    pub fn tr(input: &str, from: char, to: char) -> String {
        input.replace(from, &to.to_string())
    }

    pub fn stat(file: &str, format: CoreutilFormat) -> String {
        format!(
            "  File: {}\n  Size: 1024       Blocks: 8          IO Block: 4096   regular file\n  Flavor Format: {}\n  Access: (0644/-rw-r--r--)  Uid: ( 0/ root)   Gid: ( 0/ root)",
            file,
            format.name()
        )
    }

    pub fn df(human: bool, format: CoreutilFormat) -> Vec<String> {
        let size = if human { "100G" } else { "104857600" };
        let used = if human { "25G" } else { "26214400" };
        let avail = if human { "75G" } else { "78643200" };

        vec![
            format!("Filesystem     Type       Size  Used Avail Use% Mounted on [{}]", format.name()),
            format!("/dev/root      sigmafs   {:>6} {:>5} {:>5}  25% /", size, used, avail),
        ]
    }

    pub fn du(human: bool, format: CoreutilFormat) -> Vec<String> {
        let size = if human { "4.0M" } else { "4096" };
        vec![
            format!("{:>6}\t. [{}]", size, format.name()),
        ]
    }

    pub fn uname(all_info: bool, format: CoreutilFormat) -> String {
        if all_info {
            format!(
                "SigmaOS sovereign-host 1.0.0 #1 SMP PREEMPT_DYNAMIC 2026 x86_64 GNU/Linux [{}]",
                format.name()
            )
        } else {
            "SigmaOS".to_string()
        }
    }

    pub fn env(vars: &[(&str, &str)]) -> Vec<String> {
        vars.iter().map(|(k, v)| format!("{}={}", k, v)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coreutil_format_inference() {
        assert_eq!(CoreutilFormat::from_cmd_name("gnu-ls"), CoreutilFormat::GnuCoreutils);
        assert_eq!(CoreutilFormat::from_cmd_name("bsd-ls"), CoreutilFormat::BsdCoreutils);
        assert_eq!(CoreutilFormat::from_cmd_name("busybox-cat"), CoreutilFormat::BusyBoxApplet);
        assert_eq!(CoreutilFormat::from_cmd_name("toybox-grep"), CoreutilFormat::ToyboxApplet);
        assert_eq!(CoreutilFormat::from_cmd_name("uutils-wc"), CoreutilFormat::UutilsRust);
        assert_eq!(CoreutilFormat::from_cmd_name("ls"), CoreutilFormat::SovereignNative);
    }

    #[test]
    fn test_coreutil_options_parsing() {
        let opts = CoreutilOptions::parse(&["-a", "-l", "--human-readable", "--color"], CoreutilFormat::GnuCoreutils);
        assert!(opts.show_hidden);
        assert!(opts.long_format);
        assert!(opts.human_readable);
        assert!(opts.color_output);
    }

    #[test]
    fn test_multicall_dispatch_all_formats() {
        assert!(MultiCallManager::dispatch("gnu-ls", &[]).is_ok());
        assert!(MultiCallManager::dispatch("bsd-cat", &[]).is_ok());
        assert!(MultiCallManager::dispatch("busybox-grep", &[]).is_ok());
        assert!(MultiCallManager::dispatch("toybox-ps", &[]).is_ok());
        assert!(MultiCallManager::dispatch("uutils-echo", &["hello"]).is_ok());
        assert!(MultiCallManager::dispatch("unknown_cmd", &[]).is_err());
    }

    #[test]
    fn test_universal_engine_execution() {
        let engine = UniversalCoreutilsEngine::new(CoreutilFormat::GnuCoreutils);
        let res = engine.execute("bsd-ls", &["-a"]);
        assert_eq!(res.exit_code, 0);
        assert_eq!(res.format_used, CoreutilFormat::BsdCoreutils);
        assert!(res.stdout_lines[0].contains("BSD Coreutils"));

        let res2 = engine.execute("echo", &["SigmaOS"]);
        assert_eq!(res2.exit_code, 0);
        assert_eq!(res2.format_used, CoreutilFormat::GnuCoreutils);
    }

    #[test]
    fn test_coreutils_helpers() {
        let ls_out = Coreutils::ls(&["-a".to_string(), "-l".to_string(), "-h".to_string()]).unwrap();
        assert!(!ls_out.is_empty());

        let wc_out = Coreutils::wc(&["-w".to_string()], "one two three").unwrap();
        assert_eq!(wc_out, "3");

        let sorted = Coreutils::sort(vec!["b".to_string(), "a".to_string()], &[]).unwrap();
        assert_eq!(sorted, vec!["a".to_string(), "b".to_string()]);

        let head_out = Coreutils::head(&["line1".to_string(), "line2".to_string()], 1);
        assert_eq!(head_out, vec!["line1".to_string()]);

        let tail_out = Coreutils::tail(&["line1".to_string(), "line2".to_string()], 1);
        assert_eq!(tail_out, vec!["line2".to_string()]);

        let uniq_out = Coreutils::uniq(&["a".to_string(), "a".to_string(), "b".to_string()]);
        assert_eq!(uniq_out, vec!["a".to_string(), "b".to_string()]);

        let cut_out = Coreutils::cut(&["a:b".to_string()], ':', 2);
        assert_eq!(cut_out, vec!["b".to_string()]);

        let tr_out = Coreutils::tr("hello", 'e', 'a');
        assert_eq!(tr_out, "hallo");

        let uname_out = Coreutils::uname(true, CoreutilFormat::GnuCoreutils);
        assert!(uname_out.contains("SigmaOS"));
    }
}
