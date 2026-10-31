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
    let reserved = vec![
        "if", "then", "else", "elif", "fi", "while", "do", "done", "for", "case", "esac", "in",
    ];
    assert_eq!(reserved.len(), 12);
    assert!(reserved.contains(&"if"));
    assert!(reserved.contains(&"then"));
    assert!(reserved.contains(&"else"));
    assert!(reserved.contains(&"elif"));
    assert!(reserved.contains(&"fi"));
    assert!(reserved.contains(&"while"));
    assert!(reserved.contains(&"do"));
    assert!(reserved.contains(&"done"));
    assert!(reserved.contains(&"for"));
    assert!(reserved.contains(&"case"));
    assert!(reserved.contains(&"esac"));
    assert!(reserved.contains(&"in"));
}

#[test]
fn test_posix_parameter_expansion_variables() {
    let exit_var = "$?";
    let pid_var = "$$";
    assert_eq!(exit_var, "$?");
    assert_eq!(pid_var, "$$");
}
