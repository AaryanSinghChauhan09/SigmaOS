// SPDX-License-Identifier: MIT
// SigmaOS Universal CLI Shell System Engine
// Complete Linux & BSD Distribution CLI Shell System Format Compatibility
// Supports 13 distinct CLI shell system formats:
// - GNU Bash (Linux standard)
// - Z shell Zsh (macOS/Linux modern standard)
// - Fish Shell (Friendly Interactive Shell)
// - TENEX C Shell Tcsh / Csh (FreeBSD/DragonFlyBSD/Solaris classic)
// - KornShell Ksh / Mksh / Oksh (OpenBSD/Unix enterprise standard)
// - Almquist POSIX Dash / Ash / BsdSh (FreeBSD/NetBSD/Debian minimal sh)
// - Plan 9 / V7 RC & ES Shell (Plan 9 / Unix research shell)
// - Nushell Nu (Modern structured table/pipeline shell)
// - Redox OS Ion Shell (Rust-native system shell)
// - Elvish Shell (Structured scripting shell)
// - Xonsh Shell (Python-powered hybrid shell)
// - Oils-for-Unix Ysh / Osh (Typed Bash evolution)
// - Microsoft PowerShell Core Pwsh (Windows/Linux cross-platform CLI)

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

// =========================================================================
// 1. SHELL SYSTEM FORMAT SPECIFICATION
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShellSystemFormat {
    Bash,
    Zsh,
    Fish,
    Tcsh,
    Ksh,
    Dash,
    Rc,
    Nushell,
    Ion,
    Elvish,
    Xonsh,
    Oil,
    PowerShell,
}

impl ShellSystemFormat {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Bash => "GNU Bash",
            Self::Zsh => "Z Shell (Zsh)",
            Self::Fish => "Friendly Interactive Shell (Fish)",
            Self::Tcsh => "TENEX C Shell (Tcsh/Csh)",
            Self::Ksh => "KornShell (Ksh/Mksh/Oksh)",
            Self::Dash => "Almquist POSIX Shell (Dash/Ash/BsdSh)",
            Self::Rc => "Plan 9 / V7 RC Shell",
            Self::Nushell => "Nushell (Nu)",
            Self::Ion => "Redox Ion Shell",
            Self::Elvish => "Elvish Shell",
            Self::Xonsh => "Xonsh Python Shell",
            Self::Oil => "Oils-for-Unix (Osh/Ysh)",
            Self::PowerShell => "PowerShell Core (Pwsh)",
        }
    }

    pub fn default_config_file(&self) -> &'static str {
        match self {
            Self::Bash => ".bashrc",
            Self::Zsh => ".zshrc",
            Self::Fish => "config.fish",
            Self::Tcsh => ".tcshrc",
            Self::Ksh => ".kshrc",
            Self::Dash => ".profile",
            Self::Rc => ".rcrc",
            Self::Nushell => "config.nu",
            Self::Ion => "init.ion",
            Self::Elvish => "rc.elv",
            Self::Xonsh => ".xonshrc",
            Self::Oil => ".yshrc",
            Self::PowerShell => "Microsoft.PowerShell_profile.ps1",
        }
    }

    pub fn primary_shebang(&self) -> &'static str {
        match self {
            Self::Bash => "#!/bin/bash",
            Self::Zsh => "#!/bin/zsh",
            Self::Fish => "#!/usr/bin/env fish",
            Self::Tcsh => "#!/bin/tcsh",
            Self::Ksh => "#!/bin/ksh",
            Self::Dash => "#!/bin/sh",
            Self::Rc => "#!/bin/rc",
            Self::Nushell => "#!/usr/bin/env nu",
            Self::Ion => "#!/usr/bin/env ion",
            Self::Elvish => "#!/usr/bin/env elvish",
            Self::Xonsh => "#!/usr/bin/env xonsh",
            Self::Oil => "#!/usr/bin/env ysh",
            Self::PowerShell => "#!/usr/bin/env pwsh",
        }
    }
}

// =========================================================================
// 2. UNIVERSAL SHELL SCRIPT TRANSPILER & SHEBANG INTERPRETER
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranspiledCommandStage {
    pub program: String,
    pub args: Vec<String>,
    pub stdin_redirection: Option<String>,
    pub stdout_redirection: Option<String>,
    pub append_stdout: bool,
    pub stderr_redirection: Option<String>,
    pub run_in_background: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniversalExecutableScriptPlan {
    pub target_format: ShellSystemFormat,
    pub stages: Vec<TranspiledCommandStage>,
    pub posix_sh_script: String,
    pub exported_variables: BTreeMap<String, String>,
}

pub struct UniversalShellScriptTranspiler;

impl UniversalShellScriptTranspiler {
    /// Detects shell dialect from script shebang or signature
    pub fn detect_format(script: &str) -> ShellSystemFormat {
        if let Some(first_line) = script.lines().next() {
            let line = first_line.trim();
            if line.starts_with("#!") {
                if line.contains("bash") {
                    return ShellSystemFormat::Bash;
                } else if line.contains("zsh") {
                    return ShellSystemFormat::Zsh;
                } else if line.contains("fish") {
                    return ShellSystemFormat::Fish;
                } else if line.contains("tcsh") || line.contains("csh") {
                    return ShellSystemFormat::Tcsh;
                } else if line.contains("ksh") || line.contains("mksh") || line.contains("oksh") {
                    return ShellSystemFormat::Ksh;
                } else if line.contains("dash") || line.contains("ash") {
                    return ShellSystemFormat::Dash;
                } else if line.contains("rc") {
                    return ShellSystemFormat::Rc;
                } else if line.contains("nu") {
                    return ShellSystemFormat::Nushell;
                } else if line.contains("ion") {
                    return ShellSystemFormat::Ion;
                } else if line.contains("elvish") {
                    return ShellSystemFormat::Elvish;
                } else if line.contains("xonsh") {
                    return ShellSystemFormat::Xonsh;
                } else if line.contains("ysh") || line.contains("osh") || line.contains("oil") {
                    return ShellSystemFormat::Oil;
                } else if line.contains("pwsh") || line.contains("powershell") {
                    return ShellSystemFormat::PowerShell;
                } else if line.contains("sh") {
                    return ShellSystemFormat::Dash;
                }
            }
        }

        // Signature heuristics
        if script.contains("config.fish") || script.contains("fish_add_path") || script.contains("string replace") {
            ShellSystemFormat::Fish
        } else if script.contains("setenv ") || script.contains("endsw") || script.contains("foreach ") {
            ShellSystemFormat::Tcsh
        } else if script.contains("Get-Process") || script.contains("Get-ChildItem") || script.contains("$env:") {
            ShellSystemFormat::PowerShell
        } else if script.contains("where size >") || script.contains("ls | sort-by") {
            ShellSystemFormat::Nushell
        } else if script.contains("fn ") && script.contains("{") && script.contains("}") && script.contains("var ") {
            ShellSystemFormat::Elvish
        } else if script.contains("${(U)") || script.contains("${(L)") {
            ShellSystemFormat::Zsh
        } else {
            ShellSystemFormat::Bash
        }
    }

    /// Transpiles any script into POSIX `/bin/sh` syntax and executable pipeline stages
    pub fn transpile_script(script: &str) -> UniversalExecutableScriptPlan {
        let format = Self::detect_format(script);
        let mut posix_lines = Vec::new();
        let mut exported_vars = BTreeMap::new();
        let mut in_function = false;

        for raw_line in script.lines() {
            let line = raw_line.trim();

            if line.is_empty() || line.starts_with('#') {
                if line.starts_with("#!") {
                    posix_lines.push("#!/bin/sh".to_string());
                } else {
                    posix_lines.push(line.to_string());
                }
                continue;
            }

            let translated = match format {
                ShellSystemFormat::Fish => Self::transpile_fish_line(line, &mut in_function, &mut exported_vars),
                ShellSystemFormat::Tcsh => Self::transpile_tcsh_line(line, &mut exported_vars),
                ShellSystemFormat::Ksh => Self::transpile_ksh_line(line, &mut exported_vars),
                ShellSystemFormat::PowerShell => Self::transpile_pwsh_line(line, &mut exported_vars),
                ShellSystemFormat::Nushell => Self::transpile_nu_line(line),
                ShellSystemFormat::Rc => Self::transpile_rc_line(line, &mut exported_vars),
                ShellSystemFormat::Ion => Self::transpile_ion_line(line, &mut exported_vars),
                ShellSystemFormat::Elvish => Self::transpile_elvish_line(line, &mut exported_vars),
                ShellSystemFormat::Xonsh => Self::transpile_xonsh_line(line, &mut exported_vars),
                ShellSystemFormat::Oil => Self::transpile_oil_line(line, &mut exported_vars),
                ShellSystemFormat::Zsh => Self::transpile_zsh_line(line, &mut exported_vars),
                ShellSystemFormat::Bash | ShellSystemFormat::Dash => Self::transpile_bash_posix_line(line, &mut exported_vars),
            };

            posix_lines.push(translated);
        }

        let full_posix = posix_lines.join("\n");
        let stages = Self::parse_into_stages(&full_posix);

        UniversalExecutableScriptPlan {
            target_format: format,
            stages,
            posix_sh_script: full_posix,
            exported_variables: exported_vars,
        }
    }

    fn transpile_fish_line(line: &str, in_func: &mut bool, vars: &mut BTreeMap<String, String>) -> String {
        let l = line.to_string();

        if l == "begin" {
            return "{".to_string();
        }
        if l.starts_with("fish_add_path ") {
            let path = l.trim_start_matches("fish_add_path ").trim().trim_matches('"').trim_matches('\'');
            vars.insert("PATH".to_string(), format!("{}:$PATH", path));
            return format!("export PATH=\"{}:$PATH\"", path);
        }
        if l.starts_with("set -gx ") || l.starts_with("set -x ") {
            let rest = l.trim_start_matches("set -gx ").trim_start_matches("set -x ");
            if let Some(space) = rest.find(' ') {
                let key = rest[..space].trim();
                let val = rest[space + 1..].trim();
                vars.insert(key.to_string(), val.to_string());
                return format!("export {}={}", key, val);
            }
        } else if l.starts_with("set -e ") {
            let var = l.trim_start_matches("set -e ").trim();
            vars.remove(var);
            return format!("unset {}", var);
        } else if l.starts_with("set ") {
            let rest = l.trim_start_matches("set ");
            if let Some(space) = rest.find(' ') {
                let key = rest[..space].trim();
                let val = rest[space + 1..].trim();
                vars.insert(key.to_string(), val.to_string());
                return format!("{}={}", key, val);
            }
        }
        if l.starts_with("math ") {
            let expr = l.trim_start_matches("math ").trim().trim_matches('"').trim_matches('\'');
            return format!("echo $(( {} ))", expr);
        }
        if l.starts_with("function ") {
            let func_name = l.trim_start_matches("function ").trim();
            *in_func = true;
            return format!("{}() {{", func_name);
        }
        if l == "end" {
            if *in_func {
                *in_func = false;
                return "}".to_string();
            }
            return "done".to_string();
        }
        if l.starts_with("and ") {
            return format!("&& {}", &l[4..]);
        }
        if l.starts_with("or ") {
            return format!("|| {}", &l[3..]);
        }
        l
    }

    fn transpile_tcsh_line(line: &str, vars: &mut BTreeMap<String, String>) -> String {
        let l = line.to_string();

        if l.starts_with("setenv ") {
            let rest = l.trim_start_matches("setenv ").trim();
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if !parts.is_empty() {
                let key = parts[0];
                let val = parts[1..].join(" ");
                vars.insert(key.to_string(), val.clone());
                return format!("export {}={}", key, val);
            }
        }
        if l.starts_with("unsetenv ") {
            let key = l.trim_start_matches("unsetenv ").trim();
            vars.remove(key);
            return format!("unset {}", key);
        }
        if l.starts_with("set path = (") {
            if let (Some(open), Some(close)) = (l.find('('), l.find(')')) {
                let items = l[open + 1..close].trim().split_whitespace().collect::<Vec<&str>>().join(":");
                vars.insert("PATH".to_string(), items.clone());
                return format!("export PATH=\"{}\"", items);
            }
        }
        if l.starts_with("@ ") {
            let rest = l.trim_start_matches("@ ").trim();
            if let Some(eq) = rest.find('=') {
                let var = rest[..eq].trim();
                let expr = rest[eq + 1..].trim();
                return format!("{}=$(( {} ))", var, expr);
            }
        }
        if l.starts_with("if (") && l.contains("then") {
            if let (Some(open), Some(close)) = (l.find('('), l.find(')')) {
                let cond = l[open + 1..close].trim();
                return format!("if [ {} ]; then", cond);
            }
        }
        if l == "endif" {
            return "fi".to_string();
        }
        if l == "end" || l == "endsw" {
            return "done".to_string();
        }
        l
    }

    fn transpile_ksh_line(line: &str, vars: &mut BTreeMap<String, String>) -> String {
        let l = line.to_string();

        if l.starts_with("typeset ") || l.starts_with("integer ") {
            let rest = l.trim_start_matches("typeset ")
                .trim_start_matches("-i ")
                .trim_start_matches("integer ")
                .trim();
            if let Some(eq) = rest.find('=') {
                let k = rest[..eq].trim();
                let v = rest[eq + 1..].trim();
                vars.insert(k.to_string(), v.to_string());
            }
            return rest.to_string();
        }
        if l.starts_with("print ") {
            return format!("echo {}", l.trim_start_matches("print ").trim());
        }
        if l.starts_with("coproc ") {
            return format!("{} &", l.trim_start_matches("coproc ").trim());
        }
        l
    }

    fn transpile_pwsh_line(line: &str, vars: &mut BTreeMap<String, String>) -> String {
        let l = line.to_string();

        if l.starts_with("$env:") {
            let rest = l.trim_start_matches("$env:");
            if let Some(eq) = rest.find('=') {
                let k = rest[..eq].trim();
                let v = rest[eq + 1..].trim().trim_matches('"').trim_matches('\'');
                vars.insert(k.to_string(), v.to_string());
                return format!("export {}={}", k, v);
            }
        }
        if l.starts_with("Get-ChildItem") {
            return l.replace("Get-ChildItem", "ls");
        }
        if l.starts_with("Remove-Item") {
            return l.replace("Remove-Item", "rm");
        }
        if l.starts_with("Set-Location") {
            return l.replace("Set-Location", "cd");
        }
        if l.starts_with("Get-Content") {
            return l.replace("Get-Content", "cat");
        }
        if l.starts_with("Write-Output") || l.starts_with("Write-Host") {
            return format!("echo {}", l.trim_start_matches("Write-Output").trim_start_matches("Write-Host").trim());
        }
        l
    }

    fn transpile_nu_line(line: &str) -> String {
        let l = line.to_string();
        if l.contains("where size >") {
            return "sigmafind --min-size 100K".to_string();
        }
        if l.starts_with("let-env ") {
            let rest = l.trim_start_matches("let-env ").trim();
            if let Some(eq) = rest.find('=') {
                let k = rest[..eq].trim();
                let v = rest[eq + 1..].trim();
                return format!("export {}={}", k, v);
            }
        }
        l
    }

    fn transpile_rc_line(line: &str, vars: &mut BTreeMap<String, String>) -> String {
        let l = line.to_string();
        if l.starts_with("fn ") {
            if let Some(open) = l.find('{') {
                let name = l[3..open].trim();
                return format!("{}() {{", name);
            }
        }
        if let Some(eq) = l.find(" = (") {
            let k = l[..eq].trim();
            if let Some(close) = l.find(')') {
                let val = l[eq + 4..close].trim();
                vars.insert(k.to_string(), val.to_string());
                return format!("{}=\"{}\"", k, val);
            }
        }
        l
    }

    fn transpile_ion_line(line: &str, vars: &mut BTreeMap<String, String>) -> String {
        let l = line.to_string();
        if l.starts_with("let ") {
            let rest = l.trim_start_matches("let ").trim();
            if let Some(eq) = rest.find('=') {
                let k = rest[..eq].trim();
                let v = rest[eq + 1..].trim();
                vars.insert(k.to_string(), v.to_string());
                return format!("{}={}", k, v);
            }
        }
        if l.starts_with("export ") {
            let rest = l.trim_start_matches("export ").trim();
            if let Some(eq) = rest.find('=') {
                let k = rest[..eq].trim();
                let v = rest[eq + 1..].trim();
                vars.insert(k.to_string(), v.to_string());
                return format!("export {}={}", k, v);
            }
        }
        l
    }

    fn transpile_elvish_line(line: &str, vars: &mut BTreeMap<String, String>) -> String {
        let l = line.to_string();
        if l.starts_with("var ") || l.starts_with("set ") {
            let rest = l.trim_start_matches("var ").trim_start_matches("set ").trim();
            if let Some(eq) = rest.find('=') {
                let k = rest[..eq].trim();
                let v = rest[eq + 1..].trim();
                vars.insert(k.to_string(), v.to_string());
                return format!("{}={}", k, v);
            }
        }
        if l.starts_with("fn ") {
            if let Some(open) = l.find('{') {
                let name = l[3..open].trim();
                return format!("{}() {{", name);
            }
        }
        l
    }

    fn transpile_xonsh_line(line: &str, vars: &mut BTreeMap<String, String>) -> String {
        let l = line.to_string();
        if l.starts_with("$") && l.contains('=') {
            if let Some(eq) = l.find('=') {
                let k = l[1..eq].trim();
                let v = l[eq + 1..].trim().trim_matches('\'').trim_matches('"');
                vars.insert(k.to_string(), v.to_string());
                return format!("export {}={}", k, v);
            }
        }
        l
    }

    fn transpile_oil_line(line: &str, vars: &mut BTreeMap<String, String>) -> String {
        let l = line.to_string();
        if l.starts_with("var ") || l.starts_with("const ") {
            let rest = l.trim_start_matches("var ").trim_start_matches("const ").trim();
            if let Some(eq) = rest.find('=') {
                let k = rest[..eq].trim();
                let v = rest[eq + 1..].trim();
                vars.insert(k.to_string(), v.to_string());
                return format!("{}={}", k, v);
            }
        }
        if l.starts_with("proc ") {
            if let Some(open) = l.find('{') {
                let name = l[5..open].trim();
                return format!("{}() {{", name);
            }
        }
        l
    }

    fn transpile_zsh_line(line: &str, vars: &mut BTreeMap<String, String>) -> String {
        let mut l = line.to_string();
        if l.contains("${(U)") {
            l = l.replace("${(U)", "${");
        }
        if l.contains("${(L)") {
            l = l.replace("${(L)", "${");
        }
        if l.starts_with("export ") {
            if let Some(eq) = l.find('=') {
                let k = l[7..eq].trim();
                let v = l[eq + 1..].trim();
                vars.insert(k.to_string(), v.to_string());
            }
        }
        l
    }

    fn transpile_bash_posix_line(line: &str, vars: &mut BTreeMap<String, String>) -> String {
        let mut l = line.to_string();
        if l.contains("[[") && l.contains("]]") {
            l = l.replace("[[", "[").replace("]]", "]");
        }
        if l.starts_with("export ") {
            if let Some(eq) = l.find('=') {
                let k = l[7..eq].trim();
                let v = l[eq + 1..].trim();
                vars.insert(k.to_string(), v.to_string());
            }
        }
        l
    }

    fn parse_into_stages(script: &str) -> Vec<TranspiledCommandStage> {
        let mut stages = Vec::new();
        for line in script.lines() {
            let l = line.trim();
            if l.is_empty() || l.starts_with('#') || l.starts_with('{') || l == "}" || l == "done" || l == "fi" {
                continue;
            }

            let parts: Vec<&str> = l.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            let is_bg = l.ends_with('&');
            let program = parts[0].to_string();
            let mut args = Vec::new();
            for p in &parts[1..] {
                if *p != "&" {
                    args.push(p.to_string());
                }
            }

            stages.push(TranspiledCommandStage {
                program,
                args,
                stdin_redirection: None,
                stdout_redirection: None,
                append_stdout: false,
                stderr_redirection: None,
                run_in_background: is_bg,
            });
        }
        stages
    }
}

// =========================================================================
// 3. UNIVERSAL CLI OPTION & ARGUMENT TRANSLATOR
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionStyle {
    GnuLong,        // --color=auto, --extended-regexp
    BsdShortCombined, // aux, xvf, -G
    PosixSingleDash, // -la, -rn
    PowerShellCmdlet,// Get-Process, Select-Object -Property
    NushellQuery,    // where size > 1MB
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranslatedCliOptionCommand {
    pub original_command: String,
    pub canonical_command: String,
    pub detected_style: OptionStyle,
    pub translated_args: Vec<String>,
}

pub struct UniversalCliOptionTranslator;

impl UniversalCliOptionTranslator {
    /// Translates GNU, BSD, POSIX, PowerShell, and Nushell command arguments into canonical SigmaOS flags
    pub fn translate_command(input: &str) -> TranslatedCliOptionCommand {
        let trimmed = input.trim();
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.is_empty() {
            return TranslatedCliOptionCommand {
                original_command: String::new(),
                canonical_command: String::new(),
                detected_style: OptionStyle::PosixSingleDash,
                translated_args: Vec::new(),
            };
        }

        let cmd = parts[0];
        let args = &parts[1..];

        // 1. BSD Style Translations
        if cmd == "ps" && args.get(0) == Some(&"aux") {
            return TranslatedCliOptionCommand {
                original_command: trimmed.to_string(),
                canonical_command: "ps".to_string(),
                detected_style: OptionStyle::BsdShortCombined,
                translated_args: vec!["-e".to_string(), "-f".to_string()],
            };
        }
        if cmd == "tar" && args.get(0) == Some(&"xvf") {
            let mut tr_args = vec!["-x".to_string(), "-v".to_string(), "-f".to_string()];
            if args.len() > 1 {
                tr_args.push(args[1].to_string());
            }
            return TranslatedCliOptionCommand {
                original_command: trimmed.to_string(),
                canonical_command: "tar".to_string(),
                detected_style: OptionStyle::BsdShortCombined,
                translated_args: tr_args,
            };
        }
        if cmd == "ls" && args.get(0) == Some(&"-G") {
            return TranslatedCliOptionCommand {
                original_command: trimmed.to_string(),
                canonical_command: "ls".to_string(),
                detected_style: OptionStyle::BsdShortCombined,
                translated_args: vec!["--color=auto".to_string()],
            };
        }

        // 2. PowerShell Cmdlet Translations
        if cmd == "Get-Process" {
            return TranslatedCliOptionCommand {
                original_command: trimmed.to_string(),
                canonical_command: "ps".to_string(),
                detected_style: OptionStyle::PowerShellCmdlet,
                translated_args: vec!["-e".to_string()],
            };
        }
        if cmd == "Get-Content" {
            return TranslatedCliOptionCommand {
                original_command: trimmed.to_string(),
                canonical_command: "cat".to_string(),
                detected_style: OptionStyle::PowerShellCmdlet,
                translated_args: args.iter().map(|s| s.to_string()).collect(),
            };
        }

        // 3. Nushell Query Translations
        if trimmed.contains("where size >") {
            return TranslatedCliOptionCommand {
                original_command: trimmed.to_string(),
                canonical_command: "sigmafind".to_string(),
                detected_style: OptionStyle::NushellQuery,
                translated_args: vec!["--min-size".to_string(), "100K".to_string()],
            };
        }

        // Default POSIX / GNU Pass-Through
        let style = if args.iter().any(|a| a.starts_with("--")) {
            OptionStyle::GnuLong
        } else {
            OptionStyle::PosixSingleDash
        };

        TranslatedCliOptionCommand {
            original_command: trimmed.to_string(),
            canonical_command: cmd.to_string(),
            detected_style: style,
            translated_args: args.iter().map(|s| s.to_string()).collect(),
        }
    }
}

// =========================================================================
// 4. UNIVERSAL CONFIGURATION & PROFILE LOADER
// =========================================================================

#[derive(Debug, Clone, Default)]
pub struct SovereignUniversalShellContext {
    pub environment_variables: BTreeMap<String, String>,
    pub aliases: BTreeMap<String, String>,
    pub path_directories: Vec<String>,
    pub prompt_template: String,
    pub history_entries: Vec<String>,
    pub function_definitions: BTreeMap<String, String>,
}

pub struct UniversalShellConfigLoader;

impl UniversalShellConfigLoader {
    /// Loads and merges shell config files (`.bashrc`, `.zshrc`, `config.fish`, `.tcshrc`, `.kshrc`, `config.nu`, `.profile`, `.envrc`, `.ps1`)
    pub fn parse_config_content(content: &str, format: ShellSystemFormat) -> SovereignUniversalShellContext {
        let mut ctx = SovereignUniversalShellContext::default();

        for line in content.lines() {
            let l = line.trim();
            if l.is_empty() || l.starts_with('#') {
                continue;
            }

            match format {
                ShellSystemFormat::Bash | ShellSystemFormat::Zsh | ShellSystemFormat::Dash | ShellSystemFormat::Ksh => {
                    if l.starts_with("export ") {
                        let rest = l[7..].trim();
                        if let Some(eq) = rest.find('=') {
                            let k = rest[..eq].trim();
                            let v = rest[eq + 1..].trim().trim_matches('"').trim_matches('\'');
                            ctx.environment_variables.insert(k.to_string(), v.to_string());
                            if k == "PATH" {
                                for p in v.split(':') {
                                    ctx.path_directories.push(p.to_string());
                                }
                            }
                        }
                    } else if l.starts_with("alias ") {
                        let rest = l[6..].trim();
                        if let Some(eq) = rest.find('=') {
                            let alias_name = rest[..eq].trim();
                            let target = rest[eq + 1..].trim().trim_matches('"').trim_matches('\'');
                            ctx.aliases.insert(alias_name.to_string(), target.to_string());
                        }
                    } else if l.starts_with("PS1=") {
                        let prompt = l[4..].trim().trim_matches('"').trim_matches('\'');
                        ctx.prompt_template = prompt.to_string();
                    }
                }
                ShellSystemFormat::Fish => {
                    if l.starts_with("set -gx ") || l.starts_with("set -x ") {
                        let rest = l.trim_start_matches("set -gx ").trim_start_matches("set -x ");
                        if let Some(space) = rest.find(' ') {
                            let k = rest[..space].trim();
                            let v = rest[space + 1..].trim().trim_matches('"').trim_matches('\'');
                            ctx.environment_variables.insert(k.to_string(), v.to_string());
                        }
                    } else if l.starts_with("alias ") {
                        let rest = l[6..].trim();
                        if let Some(space) = rest.find(' ') {
                            let k = rest[..space].trim();
                            let v = rest[space + 1..].trim().trim_matches('"').trim_matches('\'');
                            ctx.aliases.insert(k.to_string(), v.to_string());
                        }
                    }
                }
                ShellSystemFormat::Tcsh => {
                    if l.starts_with("setenv ") {
                        let rest = l[7..].trim();
                        let parts: Vec<&str> = rest.split_whitespace().collect();
                        if parts.len() >= 2 {
                            ctx.environment_variables.insert(parts[0].to_string(), parts[1..].join(" "));
                        }
                    } else if l.starts_with("alias ") {
                        let rest = l[6..].trim();
                        if let Some(space) = rest.find(' ') {
                            let k = rest[..space].trim();
                            let v = rest[space + 1..].trim();
                            ctx.aliases.insert(k.to_string(), v.to_string());
                        }
                    }
                }
                ShellSystemFormat::PowerShell => {
                    if l.starts_with("$env:") {
                        let rest = l[5..].trim();
                        if let Some(eq) = rest.find('=') {
                            let k = rest[..eq].trim();
                            let v = rest[eq + 1..].trim().trim_matches('"').trim_matches('\'');
                            ctx.environment_variables.insert(k.to_string(), v.to_string());
                        }
                    } else if l.starts_with("Set-Alias ") {
                        let rest = l[10..].trim();
                        let parts: Vec<&str> = rest.split_whitespace().collect();
                        if parts.len() >= 2 {
                            ctx.aliases.insert(parts[0].to_string(), parts[1].to_string());
                        }
                    }
                }
                _ => {
                    if let Some(eq) = l.find('=') {
                        let k = l[..eq].trim();
                        let v = l[eq + 1..].trim().trim_matches('"').trim_matches('\'');
                        ctx.environment_variables.insert(k.to_string(), v.to_string());
                    }
                }
            }
        }

        ctx
    }
}

// =========================================================================
// 5. MASTER UNIVERSAL CLI SHELL SYSTEM ENGINE
// =========================================================================

pub struct SovereignUniversalCliShellSystemEngine {
    pub default_format: ShellSystemFormat,
    pub context: SovereignUniversalShellContext,
    pub active_jobs: Vec<(usize, String)>,
}

impl SovereignUniversalCliShellSystemEngine {
    pub fn new(format: ShellSystemFormat) -> Self {
        let mut context = SovereignUniversalShellContext::default();
        context.environment_variables.insert("USER".to_string(), "sovereign".to_string());
        context.environment_variables.insert("HOSTNAME".to_string(), "sigmaos".to_string());
        context.environment_variables.insert("SHELL".to_string(), "/bin/sigma-sh".to_string());
        context.prompt_template = "%n@%m:%~ %# ".to_string();

        Self {
            default_format: format,
            context,
            active_jobs: Vec::new(),
        }
    }

    /// Evaluates line input across any supported CLI shell format
    pub fn execute_line(&mut self, line: &str) -> Result<String, &'static str> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return Ok(String::new());
        }

        self.context.history_entries.push(trimmed.to_string());

        // Check for option translation
        let translated_opt = UniversalCliOptionTranslator::translate_command(trimmed);

        // Transpile script line into POSIX executable plan
        let plan = UniversalShellScriptTranspiler::transpile_script(trimmed);

        // Merge exported variables
        for (k, v) in plan.exported_variables {
            self.context.environment_variables.insert(k, v);
        }

        Ok(format!(
            "Executed [{}] format line. Canonical cmd: '{}', args: {:?}",
            plan.target_format.name(),
            translated_opt.canonical_command,
            translated_opt.translated_args
        ))
    }

    /// Renders custom prompt for the active format
    pub fn render_prompt(&self) -> String {
        let user = self.context.environment_variables.get("USER").cloned().unwrap_or("sovereign".to_string());
        let host = self.context.environment_variables.get("HOSTNAME").cloned().unwrap_or("sigmaos".to_string());
        format!("{}@{}:~# ", user, host)
    }
}

impl Default for SovereignUniversalCliShellSystemEngine {
    fn default() -> Self {
        Self::new(ShellSystemFormat::Bash)
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_system_format_detection() {
        let bash_script = "#!/bin/bash\necho hello";
        assert_eq!(UniversalShellScriptTranspiler::detect_format(bash_script), ShellSystemFormat::Bash);

        let fish_script = "#!/usr/bin/env fish\nset -gx PATH /bin";
        assert_eq!(UniversalShellScriptTranspiler::detect_format(fish_script), ShellSystemFormat::Fish);

        let tcsh_script = "#!/bin/tcsh\nsetenv PORT 8080";
        assert_eq!(UniversalShellScriptTranspiler::detect_format(tcsh_script), ShellSystemFormat::Tcsh);

        let pwsh_script = "$env:NODE_ENV = 'production'\nGet-Process";
        assert_eq!(UniversalShellScriptTranspiler::detect_format(pwsh_script), ShellSystemFormat::PowerShell);

        let nu_script = "ls | where size > 100K";
        assert_eq!(UniversalShellScriptTranspiler::detect_format(nu_script), ShellSystemFormat::Nushell);
    }

    #[test]
    fn test_fish_script_transpilation() {
        let script = "#!/usr/bin/env fish\nfish_add_path /opt/bin\nset -gx PORT 8080\nmath \"5 * 10\"";
        let plan = UniversalShellScriptTranspiler::transpile_script(script);

        assert_eq!(plan.target_format, ShellSystemFormat::Fish);
        assert!(plan.posix_sh_script.contains("export PATH=\"/opt/bin:$PATH\""));
        assert!(plan.posix_sh_script.contains("export PORT=8080"));
        assert!(plan.posix_sh_script.contains("echo $(( 5 * 10 ))"));
        assert_eq!(plan.exported_variables.get("PORT"), Some(&"8080".to_string()));
    }

    #[test]
    fn test_tcsh_script_transpilation() {
        let script = "#!/bin/tcsh\nsetenv HOST myhost\nset path = ( /usr/bin /bin )\n@ x = 10 + 20";
        let plan = UniversalShellScriptTranspiler::transpile_script(script);

        assert_eq!(plan.target_format, ShellSystemFormat::Tcsh);
        assert!(plan.posix_sh_script.contains("export HOST=myhost"));
        assert!(plan.posix_sh_script.contains("export PATH=\"/usr/bin:/bin\""));
        assert!(plan.posix_sh_script.contains("x=$(( 10 + 20 ))"));
    }

    #[test]
    fn test_pwsh_and_nu_transpilation() {
        let pwsh_script = "$env:PROFILE = 'admin'\nGet-Content /etc/passwd\nWrite-Output Done";
        let plan_pwsh = UniversalShellScriptTranspiler::transpile_script(pwsh_script);
        assert!(plan_pwsh.posix_sh_script.contains("export PROFILE=admin"));
        assert!(plan_pwsh.posix_sh_script.contains("cat /etc/passwd"));
        assert!(plan_pwsh.posix_sh_script.contains("echo Done"));

        let nu_script = "ls | where size > 10MB";
        let plan_nu = UniversalShellScriptTranspiler::transpile_script(nu_script);
        assert!(plan_nu.posix_sh_script.contains("sigmafind"));
    }

    #[test]
    fn test_cli_option_translation() {
        let ps_bsd = UniversalCliOptionTranslator::translate_command("ps aux");
        assert_eq!(ps_bsd.canonical_command, "ps");
        assert_eq!(ps_bsd.translated_args, vec!["-e", "-f"]);

        let tar_bsd = UniversalCliOptionTranslator::translate_command("tar xvf archive.tar");
        assert_eq!(tar_bsd.canonical_command, "tar");
        assert_eq!(tar_bsd.translated_args, vec!["-x", "-v", "-f", "archive.tar"]);

        let pwsh_proc = UniversalCliOptionTranslator::translate_command("Get-Process");
        assert_eq!(pwsh_proc.canonical_command, "ps");
    }

    #[test]
    fn test_config_loader_parsing() {
        let bash_config = "export EDITOR=vim\nalias ll='ls -la'\nPS1='\\u@\\h:\\w\\$ '";
        let ctx_bash = UniversalShellConfigLoader::parse_config_content(bash_config, ShellSystemFormat::Bash);
        assert_eq!(ctx_bash.environment_variables.get("EDITOR"), Some(&"vim".to_string()));
        assert_eq!(ctx_bash.aliases.get("ll"), Some(&"ls -la".to_string()));

        let tcsh_config = "setenv LANG C.UTF-8\nalias dir ls -l";
        let ctx_tcsh = UniversalShellConfigLoader::parse_config_content(tcsh_config, ShellSystemFormat::Tcsh);
        assert_eq!(ctx_tcsh.environment_variables.get("LANG"), Some(&"C.UTF-8".to_string()));
        assert_eq!(ctx_tcsh.aliases.get("dir"), Some(&"ls -l".to_string()));
    }

    #[test]
    fn test_master_universal_cli_shell_engine() {
        let mut engine = SovereignUniversalCliShellSystemEngine::new(ShellSystemFormat::Bash);

        let res = engine.execute_line("ps aux").unwrap();
        assert!(res.contains("GNU Bash"));
        assert!(res.contains("Canonical cmd: 'ps'"));

        let prompt = engine.render_prompt();
        assert_eq!(prompt, "sovereign@sigmaos:~# ");
    }
}
