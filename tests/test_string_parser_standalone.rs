// Standalone test suite for string, config, and TOML parsing optimizations

#[path = "../src/klib/string_parser.rs"]
mod string_parser;

#[path = "../src/klib/hash.rs"]
pub mod hash;

#[path = "../src/klib/hashmap.rs"]
pub mod hashmap;

pub mod klib {
    pub use super::hashmap::HashMap;
}

#[path = "../src/klib/config_parser.rs"]
mod config_parser;

#[path = "../src/klib/vec.rs"]
pub mod vec;

#[path = "../src/klib/string.rs"]
mod string;

#[path = "../src/klib/toml.rs"]
mod toml_parser;

#[test]
fn test_split_and_trim_string() {
    let parts = string_parser::split_string("alpha,beta,gamma", ',');
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0], "alpha");
    assert_eq!(parts[1], "beta");
    assert_eq!(parts[2], "gamma");

    assert_eq!(string_parser::trim_string("   sovereign_os   "), "sovereign_os");
    assert_eq!(string_parser::trim_string("  hello world  "), "hello world");
}

#[test]
fn test_sigma_string_operations() {
    use string::SigmaString;

    let s = SigmaString::from_str("sovereign os microkernel");
    let parts: Vec<SigmaString> = s.split(' ').collect();
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0].as_str(), "sovereign");
    assert_eq!(parts[1].as_str(), "os");
    assert_eq!(parts[2].as_str(), "microkernel");

    assert!(s.contains("os"));
    assert!(!s.contains("nonexistent"));

    let replaced = s.replace("os", "kernel");
    assert_eq!(replaced.as_str(), "sovereign kernel microkernel");
}

#[test]
fn test_string_parser_methods() {
    let mut parser = string_parser::StringParser::new("sigma123");
    assert_eq!(parser.parse_letters(), "sigma");
    assert_eq!(parser.parse_digits(), "123");
    assert!(parser.is_end());
}

#[test]
fn test_toml_parser_performance_fastpaths() {
    let manifest = r#"
# Sovereign Manifest
name = "sigma-core"
version = 2026
description = "Sovereign AI-Native OS"

[settings]
fast_path = true
escaped_path = "C:\\SigmaOS\\boot"
targets = ["x86_64", "aarch64", "riscv64"]
"#;

    let doc = toml_parser::TomlDocument::parse(manifest).expect("Failed to parse TOML manifest");
    assert_eq!(doc.get_string("name"), Some("sigma-core"));
    assert_eq!(doc.get_int("version"), Some(2026));
    assert_eq!(doc.get_bool("settings.fast_path"), Some(true));
    assert_eq!(doc.get_string("settings.escaped_path"), Some("C:\\SigmaOS\\boot"));

    if let Some(toml_parser::TomlValue::Array(items)) = doc.get("settings.targets") {
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], "x86_64");
        assert_eq!(items[1], "aarch64");
        assert_eq!(items[2], "riscv64");
    } else {
        panic!("Expected array for settings.targets");
    }
}
