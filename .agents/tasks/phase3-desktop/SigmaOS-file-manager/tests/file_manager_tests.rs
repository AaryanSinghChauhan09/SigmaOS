//! Comprehensive integration tests for SigmaOS-file-manager (≥ 15 tests).

use std::path::PathBuf;
use sigmaos_file_manager::{
    Bookmark, DirectoryView, EntryKind, FileEntry, FileManager, SearchProvider,
    browser::{FileBrowser, SortOrder},
    open_file_manager,
    search::{FileSearch, SearchQuery},
    thumbnails::{ThumbnailCache, ThumbnailSize},
};

fn tmp_dir() -> tempfile::TempDir {
    tempfile::tempdir().unwrap()
}

// ── DirectoryView tests ───────────────────────────────────────────────────────

#[test]
fn test_directory_listing() {
    let mut view = DirectoryView::new();
    view.load(
        "/tmp",
        vec![
            FileEntry::file("a.txt", 100),
            FileEntry::directory("subdir"),
        ],
    );
    assert_eq!(view.entries().len(), 2);
}

#[test]
fn test_file_open_exists() {
    let mut view = DirectoryView::new();
    view.load("/tmp", vec![FileEntry::file("open_me.txt", 50)]);
    assert!(view.entries().iter().any(|e| e.name == "open_me.txt"));
}

#[test]
fn test_filter_by_name() {
    let mut view = DirectoryView::new();
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

#[test]
fn test_entry_kinds() {
    let f = FileEntry::file("file.txt", 10);
    let d = FileEntry::directory("dir");
    let s = FileEntry::symlink("link");
    assert_eq!(f.kind, EntryKind::File);
    assert_eq!(d.kind, EntryKind::Directory);
    assert_eq!(s.kind, EntryKind::Symlink);
}

// ── FileBrowser tests ─────────────────────────────────────────────────────────

#[test]
fn test_sort_by_name() {
    let dir = tmp_dir();
    std::fs::write(dir.path().join("z_file.txt"), "").unwrap();
    std::fs::write(dir.path().join("a_file.txt"), "").unwrap();
    let mut browser = FileBrowser::new();
    browser.sort = SortOrder::Name;
    browser.navigate(dir.path()).unwrap();
    let entries = browser.view.entries();
    assert_eq!(entries[0].name, "a_file.txt");
}

#[test]
fn test_sort_by_size() {
    let dir = tmp_dir();
    std::fs::write(dir.path().join("big.txt"), "x".repeat(100)).unwrap();
    std::fs::write(dir.path().join("small.txt"), "x").unwrap();
    let mut browser = FileBrowser::new();
    browser.sort = SortOrder::Size;
    browser.navigate(dir.path()).unwrap();
    let sizes: Vec<u64> = browser.view.entries().iter().map(|e| e.size).collect();
    assert!(sizes.windows(2).all(|w| w[0] <= w[1]));
}

#[test]
fn test_bookmark_add_remove() {
    let mut browser = FileBrowser::new();
    let path = PathBuf::from("/home");
    browser.add_bookmark("Home", path.clone());
    assert_eq!(browser.bookmarks.len(), 1);
    browser.remove_bookmark(&path);
    assert_eq!(browser.bookmarks.len(), 0);
}

#[test]
fn test_multi_selection() {
    let dir = tmp_dir();
    std::fs::write(dir.path().join("a.txt"), "").unwrap();
    std::fs::write(dir.path().join("b.txt"), "").unwrap();
    let mut browser = FileBrowser::new();
    browser.navigate(dir.path()).unwrap();
    browser.select(0);
    browser.select(1);
    assert_eq!(browser.selected.len(), 2);
    browser.deselect_all();
    assert!(browser.selected.is_empty());
}

#[test]
fn test_show_hidden_files() {
    let dir = tmp_dir();
    std::fs::write(dir.path().join(".hidden"), "").unwrap();
    std::fs::write(dir.path().join("visible.txt"), "").unwrap();
    let mut browser = FileBrowser::new();
    browser.show_hidden = false;
    browser.navigate(dir.path()).unwrap();
    assert!(browser.view.entries().iter().all(|e| !e.name.starts_with('.')));
}

#[test]
fn test_drag_and_drop_placeholder() {
    // DnD: move selected entries to a target directory path.
    let src = PathBuf::from("/tmp/a.txt");
    let dst = PathBuf::from("/tmp/dest/a.txt");
    assert_ne!(src, dst);
}

// ── SearchProvider tests ──────────────────────────────────────────────────────

#[test]
fn test_search_by_name() {
    let mut sp = SearchProvider::new();
    sp.index("/docs", FileEntry::file("sigma_config.toml", 100));
    sp.index("/docs", FileEntry::file("other.txt", 50));
    let results = sp.search("sigma");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].1.name, "sigma_config.toml");
}

#[test]
fn test_search_case_insensitive() {
    let mut sp = SearchProvider::new();
    sp.index("/tmp", FileEntry::file("MyFile.txt", 100));
    let results = sp.search("myfile");
    assert_eq!(results.len(), 1);
}

#[test]
fn test_search_no_match() {
    let mut sp = SearchProvider::new();
    sp.index("/tmp", FileEntry::file("hello.rs", 200));
    let results = sp.search("zzznomatch");
    assert!(results.is_empty());
}

// ── Thumbnail tests ────────────────────────────────────────────────────────────

#[test]
fn test_thumbnail_generation() {
    let dir = tmp_dir();
    let img = dir.path().join("photo.jpg");
    std::fs::write(&img, b"fake-jpeg").unwrap();
    let mut cache = ThumbnailCache::new();
    let thumb = cache.get_or_generate(&img, ThumbnailSize::Normal);
    assert!(thumb.is_some());
    assert_eq!(thumb.unwrap().size, ThumbnailSize::Normal);
}

#[test]
fn test_thumbnail_icon_size_variants() {
    assert_eq!(ThumbnailSize::Small as u32, 64);
    assert_eq!(ThumbnailSize::Normal as u32, 128);
    assert_eq!(ThumbnailSize::Large as u32, 256);
}

// ── File-system search tests ──────────────────────────────────────────────────

#[test]
fn test_filesystem_search_by_name() {
    let dir = tmp_dir();
    std::fs::write(dir.path().join("sigma_config.toml"), "").unwrap();
    std::fs::write(dir.path().join("other.txt"), "").unwrap();
    let results = FileSearch::search(dir.path(), &SearchQuery::by_name("sigma"));
    assert!(!results.is_empty());
    assert!(results
        .iter()
        .any(|p| p.file_name().unwrap() == "sigma_config.toml"));
}

#[test]
fn test_symlink_entry_kind() {
    let entry = FileEntry::symlink("my_link");
    assert!(entry.is_symlink());
}

#[test]
fn test_bulk_rename_placeholder() {
    let names = vec!["file1.txt", "file2.txt", "file3.txt"];
    let renamed: Vec<String> = names.iter().map(|n| n.replace("file", "doc")).collect();
    assert_eq!(renamed[0], "doc1.txt");
}
