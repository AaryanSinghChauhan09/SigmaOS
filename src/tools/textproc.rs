//! Text Processing Tools (sed/awk/grep Inspiration)
//! Stream editor, text processor, and pattern search

#![no_std]

extern crate alloc;

use crate::klib::{Vec};
use alloc::string::{String, ToString};

/// Sed pattern
#[derive(Debug, Clone)]
pub struct SedPattern {
    pub pattern: String,
    pub replacement: String,
    pub flags: Vec<String>,
}

impl SedPattern {
    pub fn new(pattern: &str, replacement: &str) -> Self {
        Self {
            pattern: pattern.to_string(),
            replacement: replacement.to_string(),
            flags: Vec::new(),
        }
    }

    pub fn add_flag(&mut self, flag: &str) {
        self.flags.push(flag.to_string());
    }
}

/// Substitution rule
#[derive(Debug, Clone)]
pub struct SubstitutionRule {
    pub pattern: String,
    pub replacement: String,
    pub global: bool,
}

impl SubstitutionRule {
    pub fn new(pattern: &str, replacement: &str, global: bool) -> Self {
        Self {
            pattern: pattern.to_string(),
            replacement: replacement.to_string(),
            global,
        }
    }
}

/// Stream editor
pub struct StreamEditor {
    pub patterns: Vec<SedPattern>,
    pub substitution_rules: Vec<SubstitutionRule>,
}

impl StreamEditor {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            substitution_rules: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: SedPattern) {
        self.patterns.push(pattern);
    }

    pub fn add_substitution(&mut self, rule: SubstitutionRule) {
        self.substitution_rules.push(rule);
    }

    pub fn substitute(&self, text: &str) -> String {
        let mut result = text.to_string();
        for rule in &self.substitution_rules {
            if rule.global {
                result = result.replace(&rule.pattern, &rule.replacement);
            } else {
                if let Some(pos) = result.find(&rule.pattern) {
                    result.replace_range(pos..pos + rule.pattern.len(), &rule.replacement);
                }
            }
        }
        result
    }

    pub fn delete_line(&self, text: &str, line_number: usize) -> String {
        let lines: Vec<&str> = text.lines().collect();
        let result: Vec<&str> = lines.iter().enumerate()
            .filter(|(i, _)| *i != line_number - 1)
            .map(|(_, line)| *line)
            .collect();
        result.join("\n")
    }
}

/// Awk pattern
#[derive(Debug, Clone)]
pub struct AwkPattern {
    pub pattern: String,
    pub action: String,
}

impl AwkPattern {
    pub fn new(pattern: &str, action: &str) -> Self {
        Self {
            pattern: pattern.to_string(),
            action: action.to_string(),
        }
    }
}

/// Awk action
#[derive(Debug, Clone)]
pub struct AwkAction {
    pub script: String,
}

impl AwkAction {
    pub fn new(script: &str) -> Self {
        Self {
            script: script.to_string(),
        }
    }
}

/// Text processor
pub struct TextProcessor {
    pub patterns: Vec<AwkPattern>,
    pub actions: Vec<AwkAction>,
    pub field_separator: String,
}

impl TextProcessor {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            actions: Vec::new(),
            field_separator: " ".to_string(),
        }
    }

    pub fn add_pattern(&mut self, pattern: AwkPattern) {
        self.patterns.push(pattern);
    }

    pub fn add_action(&mut self, action: AwkAction) {
        self.actions.push(action);
    }

    pub fn set_field_separator(&mut self, separator: &str) {
        self.field_separator = separator.to_string();
    }

    pub fn process_line(&self, line: &str) -> Vec<String> {
        line.split(&self.field_separator).map(|s| s.to_string()).collect()
    }

    pub fn print_field(&self, line: &str, field: usize) -> Option<String> {
        let fields = self.process_line(line);
        fields.get(field).cloned()
    }
}

/// Grep options
#[derive(Debug, Clone)]
pub struct GrepOptions {
    pub recursive: bool,
    pub ignore_case: bool,
    pub line_numbers: bool,
    pub color: bool,
    pub context_lines: u32,
}

impl GrepOptions {
    pub fn new() -> Self {
        Self {
            recursive: false,
            ignore_case: false,
            line_numbers: false,
            color: false,
            context_lines: 0,
        }
    }

    pub fn set_recursive(&mut self, recursive: bool) {
        self.recursive = recursive;
    }

    pub fn set_ignore_case(&mut self, ignore_case: bool) {
        self.ignore_case = ignore_case;
    }
}

/// Pattern search
pub struct PatternSearch {
    pub patterns: Vec<String>,
    pub options: GrepOptions,
}

impl PatternSearch {
    pub fn new(pattern: &str) -> Self {
        Self {
            patterns: vec![pattern.to_string()],
            options: GrepOptions::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: &str) {
        self.patterns.push(pattern.to_string());
    }

    pub fn search(&self, text: &str) -> Vec<GrepMatch> {
        let mut matches = Vec::new();
        for (line_num, line) in text.lines().enumerate() {
            for pattern in &self.patterns {
                if self.options.ignore_case {
                    if line.to_lowercase().contains(&pattern.to_lowercase()) {
                        matches.push(GrepMatch {
                            line_number: line_num + 1,
                            line: line.to_string(),
                            pattern: pattern.clone(),
                        });
                    }
                } else {
                    if line.contains(pattern) {
                        matches.push(GrepMatch {
                            line_number: line_num + 1,
                            line: line.to_string(),
                            pattern: pattern.clone(),
                        });
                    }
                }
            }
        }
        matches
    }
}

#[derive(Debug, Clone)]
pub struct GrepMatch {
    pub line_number: usize,
    pub line: String,
    pub pattern: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextProcessingError {
    PatternInvalid,
    FileNotFound,
    ProcessingFailed,
}

impl Default for StreamEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TextProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PatternSearch {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sed_pattern() {
        let pattern = SedPattern::new("foo", "bar");
        assert_eq!(pattern.pattern, "foo");
    }

    #[test]
    fn test_stream_editor() {
        let mut editor = StreamEditor::new();
        let rule = SubstitutionRule::new("foo", "bar", true);
        editor.add_substitution(rule);
        let result = editor.substitute("foo bar foo");
        assert_eq!(result, "bar bar bar");
    }

    #[test]
    fn test_text_processor() {
        let mut processor = TextProcessor::new();
        let fields = processor.process_line("hello world");
        assert_eq!(fields.len(), 2);
    }

    #[test]
    fn test_pattern_search() {
        let grep = PatternSearch::new("test");
        let matches = grep.search("test line\ntest2 line");
        assert_eq!(matches.len(), 2);
    }
}