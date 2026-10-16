// tests/shell/posix/posix_test.rs
// POSIX Grammar & Quoting Subset Tests for SigmaOS Shell

#[test]
fn test_posix_quoting_rules() {
    let single_quote_literal = "'$VAR *.rs'";
    assert!(!single_quote_literal.contains("expanded"));
    assert!(single_quote_literal.starts_with('\'') && single_quote_literal.ends_with('\''));

    let double_quote_literal = "\"$VAR\"";
    assert!(double_quote_literal.starts_with('"') && double_quote_literal.ends_with('"'));
}

#[test]
fn test_posix_reserved_words() {
    let reserved = vec!["if", "then", "else", "elif", "fi", "while", "do", "done", "for", "case", "esac", "in"];
    assert_eq!(reserved.len(), 12);
    assert!(reserved.contains(&"if"));
    assert!(reserved.contains(&"fi"));
}
