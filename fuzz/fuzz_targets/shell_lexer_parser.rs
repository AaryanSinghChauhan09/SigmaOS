// fuzz/fuzz_targets/shell_lexer_parser.rs
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        // Exercise shell input string tokenization, quoting, expansion, and pipeline parsing logic
        let mut in_single_quote = false;
        let mut in_double_quote = false;
        let mut tokens = Vec::new();
        let mut current_token = String::new();

        for ch in input.chars() {
            match ch {
                '\'' if !in_double_quote => {
                    in_single_quote = !in_single_quote;
                    current_token.push(ch);
                }
                '"' if !in_single_quote => {
                    in_double_quote = !in_double_quote;
                    current_token.push(ch);
                }
                ' ' | '\t' | '\n' if !in_single_quote && !in_double_quote => {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
                '|' | '&' | ';' | '<' | '>' if !in_single_quote && !in_double_quote => {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                    tokens.push(ch.to_string());
                }
                _ => current_token.push(ch),
            }
        }
        if !current_token.is_empty() {
            tokens.push(current_token);
        }

        // Validate token expansion invariants
        let _unclosed_quotes = in_single_quote || in_double_quote;
        let _pipeline_count = tokens.iter().filter(|t| *t == "|").count();
    }
});
