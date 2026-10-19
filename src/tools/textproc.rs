//! Text Processing Tools (sed/awk/grep Inspiration)
//! Stream editor, text processor, and pattern search

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
        // Optimization: build string directly with pre-allocated capacity instead of collecting into intermediate Vec<&str> vectors
        let mut result = String::with_capacity(text.len());
        let mut first = true;
        for (i, line) in text.lines().enumerate() {
            if i + 1 != line_number {
                if !first {
                    result.push('\n');
                }
                result.push_str(line);
                first = false;
            }
        }
        result
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
        line.split(&self.field_separator)
            .map(|s| s.to_string())
            .collect()
    }

    pub fn print_field(&self, line: &str, field: usize) -> Option<String> {
        // Optimization: avoid allocating a Vec<String> containing all split fields when only requesting field `field`
        line.split(&self.field_separator)
            .nth(field)
            .map(|s| s.to_string())
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

/// Zero-allocation case-insensitive substring search helper.
/// Uses window slice byte comparison for ASCII fast-path to avoid heap String allocations via `to_lowercase()`.
fn contains_ignore_case(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    if haystack.len() < needle.len() {
        return false;
    }
    if haystack.is_ascii() && needle.is_ascii() {
        let needle_bytes = needle.as_bytes();
        haystack
            .as_bytes()
            .windows(needle_bytes.len())
            .any(|window| {
                window
                    .iter()
                    .zip(needle_bytes.iter())
                    .all(|(&h, &n)| h.to_ascii_lowercase() == n.to_ascii_lowercase())
            })
    } else {
        haystack.to_lowercase().contains(&needle.to_lowercase())
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
        // Pre-allocate match buffer capacity to reduce vector growth reallocation overhead
        let mut matches = Vec::with_capacity(16);
        for (line_num, line) in text.lines().enumerate() {
            for pattern in &self.patterns {
                let matched = if self.options.ignore_case {
                    // Optimization: avoid allocating heap Strings via `to_lowercase()` per line/pattern check
                    contains_ignore_case(line, pattern)
                } else {
                    line.contains(pattern)
                };

                if matched {
                    matches.push(GrepMatch {
                        line_number: line_num + 1,
                        line: line.to_string(),
                        pattern: pattern.clone(),
                    });
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
        let processor = TextProcessor::new();
        let fields = processor.process_line("hello world");
        assert_eq!(fields.len(), 2);
    }

    #[test]
    fn test_pattern_search() {
        let grep = PatternSearch::new("test");
        let matches = grep.search("test line\ntest2 line");
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_contains_ignore_case_and_ignore_case_search() {
        assert!(contains_ignore_case("SIGMAOS LINUX", "sigmaos"));
        assert!(!contains_ignore_case("Alpha Beta", "gamma"));

        let mut grep = PatternSearch::new("SIGMA");
        grep.options.set_ignore_case(true);
        let matches = grep.search("sigma line\nOTHER LINE\nSigma2 line");
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_print_field_and_delete_line() {
        let processor = TextProcessor::new();
        assert_eq!(
            processor.print_field("hello world test", 1),
            Some("world".to_string())
        );

        let editor = StreamEditor::new();
        let result = editor.delete_line("line1\nline2\nline3", 2);
        assert_eq!(result, "line1\nline3");
    }
}
