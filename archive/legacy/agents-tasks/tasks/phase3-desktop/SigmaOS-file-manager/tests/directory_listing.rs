//! Integration tests — directory listing.

use sigmaos_file_manager::{open_file_manager, DirectoryView, FileEntry, EntryKind};

#[test]
fn file_manager_opens_at_path() {
    let fm = open_file_manager("/home/sigma");
    assert_eq!(fm.root_path, "/home/sigma");
}

#[test]
fn directory_view_lists_all_entries() {
    let mut view = DirectoryView::default();
    view.load(
        "/etc",
        vec![
            FileEntry::file("passwd", 1024),
            FileEntry::file("hosts", 256),
            FileEntry::directory("ssl"),
        ],
    );
    assert_eq!(view.entries().len(), 3);
    assert_eq!(view.current_path, "/etc");
}

#[test]
fn directory_view_entry_kinds() {
    let mut view = DirectoryView::default();
    view.load(
        "/tmp",
        vec![
            FileEntry::file("a.txt", 10),
            FileEntry::directory("subdir"),
        ],
    );
    let entries = view.entries();
    assert_eq!(entries[0].kind, EntryKind::File);
    assert_eq!(entries[1].kind, EntryKind::Directory);
}

#[test]
fn directory_view_filter_case_insensitive() {
    let mut view = DirectoryView::default();
    view.load(
        "/home",
        vec![
            FileEntry::file("README.md", 500),
            FileEntry::file("cargo.toml", 300),
        ],
    );
    let results = view.filter("readme");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "README.md");
}
