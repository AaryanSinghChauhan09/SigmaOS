//! Integration tests — search operations.

use sigmaos_file_manager::{SearchProvider, FileEntry};

#[test]
fn search_returns_matching_entries() {
    let mut sp = SearchProvider::default();
    sp.index("/home/user/docs", FileEntry::file("report.pdf", 4096));
    sp.index("/home/user/pics", FileEntry::file("vacation.jpg", 2_048_000));
    sp.index("/home/user/docs", FileEntry::file("notes.txt", 512));

    let results = sp.search("report");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].1.name, "report.pdf");
}

#[test]
fn search_case_insensitive() {
    let mut sp = SearchProvider::default();
    sp.index("/tmp", FileEntry::file("MyFile.txt", 100));
    let results = sp.search("myfile");
    assert_eq!(results.len(), 1);
}

#[test]
fn search_no_match_returns_empty() {
    let mut sp = SearchProvider::default();
    sp.index("/tmp", FileEntry::file("hello.rs", 200));
    let results = sp.search("zzznomatch");
    assert!(results.is_empty());
}

#[test]
fn search_multiple_matches() {
    let mut sp = SearchProvider::default();
    sp.index("/a", FileEntry::file("log_2024.txt", 10));
    sp.index("/b", FileEntry::file("log_2023.txt", 10));
    sp.index("/c", FileEntry::file("readme.md", 10));
    let results = sp.search("log");
    assert_eq!(results.len(), 2);
}
