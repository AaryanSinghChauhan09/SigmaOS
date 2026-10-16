// src/shell/diagnostics/mod.rs
// Diagnostics engine for SigmaOS shell

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
use std::format as alloc_format;
#[cfg(feature = "std")]
use std::string::{String, ToString};
#[cfg(feature = "std")]
use std::vec as alloc_vec;
#[cfg(feature = "std")]
use std::vec::Vec;

#[cfg(not(feature = "std"))]
use alloc::format as alloc_format;
#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};
#[cfg(not(feature = "std"))]
use alloc::vec as alloc_vec;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
}

#[derive(Debug, Clone)]
pub struct ShellDiagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub near_token: Option<String>,
    pub suggestion: Option<String>,
}

pub struct ShellDiagnosticsEngine;

impl ShellDiagnosticsEngine {
    pub fn format_parse_error(
        line: usize,
        col: usize,
        reason: &str,
        token: Option<&str>,
    ) -> String {
        let mut msg = alloc_format!(
            "sigma-sh: parse error at line {}, col {}: {}",
            line,
            col,
            reason
        );
        if let Some(tok) = token {
            msg.push_str(&alloc_format!(" Near token '{}'", tok));
        }
        msg
    }

    pub fn format_command_not_found(cmd: &str, candidates: &[&str]) -> String {
        let mut msg = alloc_format!("sigma-sh: {}: command not found", cmd);
        if let Some(best) = Self::find_closest_match(cmd, candidates) {
            msg.push_str(&alloc_format!(" (did you mean '{}'?)", best));
        }
        msg
    }

    pub fn format_redirection_error(target: &str, reason: &str) -> String {
        alloc_format!("sigma-sh: {}: {}", target, reason)
    }

    pub fn format_unclosed_block_error(block_type: &str, line: usize) -> String {
        alloc_format!(
            "sigma-sh: unclosed construct '{}' originating at line {}",
            block_type,
            line
        )
    }

    fn find_closest_match<'a>(cmd: &'a str, candidates: &[&'a str]) -> Option<&'a str> {
        let mut best_match: Option<(&'a str, usize)> = None;
        for &cand in candidates {
            let dist = Self::levenshtein_distance(cmd, cand);
            if dist <= 3 {
                match best_match {
                    None => best_match = Some((cand, dist)),
                    Some((_, min_dist)) if dist < min_dist => best_match = Some((cand, dist)),
                    _ => {}
                }
            }
        }
        best_match.map(|(c, _)| c)
    }

    fn levenshtein_distance(s1: &str, s2: &str) -> usize {
        let v1: Vec<char> = s1.chars().collect();
        let v2: Vec<char> = s2.chars().collect();
        let mut dp = alloc_vec![alloc_vec![0; v2.len() + 1]; v1.len() + 1];

        for i in 0..=v1.len() {
            dp[i][0] = i;
        }
        for j in 0..=v2.len() {
            dp[0][j] = j;
        }

        for i in 1..=v1.len() {
            for j in 1..=v2.len() {
                let cost = if v1[i - 1] == v2[j - 1] { 0 } else { 1 };
                dp[i][j] = core::cmp::min(
                    dp[i - 1][j] + 1,
                    core::cmp::min(dp[i][j - 1] + 1, dp[i - 1][j - 1] + cost),
                );
            }
        }

        dp[v1.len()][v2.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_parse_error() {
        let err = ShellDiagnosticsEngine::format_parse_error(1, 10, "unexpected EOF", Some("\""));
        assert_eq!(
            err,
            "sigma-sh: parse error at line 1, col 10: unexpected EOF Near token '\"'"
        );
    }

    #[test]
    fn test_format_command_not_found_with_suggestion() {
        let err = ShellDiagnosticsEngine::format_command_not_found("htep", &["http", "help", "ls"]);
        assert!(err.contains("command not found"));
        assert!(err.contains("did you mean 'help'?") || err.contains("did you mean 'http'?"));
    }

    #[test]
    fn test_redirection_error() {
        let err =
            ShellDiagnosticsEngine::format_redirection_error("/root/test.txt", "Permission denied");
        assert_eq!(err, "sigma-sh: /root/test.txt: Permission denied");
    }
}
