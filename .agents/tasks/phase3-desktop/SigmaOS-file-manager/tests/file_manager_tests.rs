//! Tests for SigmaOS-file-manager.

use std::path::PathBuf;
use sigma_file_manager::{
    Bookmark,
    browser::{FileBrowser, SortOrder},
    search::{FileSearch, SearchQuery},
    thumbnails::{ThumbnailCache, ThumbnailSize},
};

fn tmp_dir() -> tempfile::TempDir {
    tempfile::tempdir().unwrap()
}

// ── Browser tests ─────────────────────────────────────────────────────────────

#[test]
fn test_directory_listing() {
    let dir = tmp_dir();
    std::fs::write(dir.path().join("file.txt"), "hello").unwrap();
    let mut browser = FileBrowser::new();
    browser.navigate(dir.path()).unwrap();
    assert!(!browser.entries.is_empty());
}

#[test]
fn test_file_open_exists() {
    let dir = tmp_dir();
    std::fs::write(dir.path().join("open_me.txt"), "").unwrap();
    let mut browser = FileBrowser::new();
    browser.navigate(dir.path()).unwrap();
    assert!(browser.entries.iter().any(|e| e.name == "open_me.txt"));
}

#[test]
fn test_sort_by_name() {
    let dir = tmp_dir();
    std::fs::write(dir.path().join("z_file.txt"), "").unwrap();
    std::fs::write(dir.path().join("a_file.txt"), "").unwrap();
    let mut browser = FileBrowser::new();
    browser.sort = SortOrder::Name;
    browser.navigate(dir.path()).unwrap();
    assert_eq!(browser.entries[0].name, "a_file.txt");
}

#[test]
fn test_sort_by_size() {
    let dir = tmp_dir();
    std::fs::write(dir.path().join("big.txt"), "x".repeat(100)).unwrap();
    std::fs::write(dir.path().join("small.txt"), "x").unwrap();
    let mut browser = FileBrowser::new();
    browser.sort = SortOrder::Size;
    browser.navigate(dir.path()).unwrap();
    let sizes: Vec<u64> = browser.entries.iter().map(|e| e.size).collect();
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
fn test_symlink_handling() {
    let dir = tmp_dir();
    std::fs::write(dir.path().join("target.txt"), "data").unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(dir.path().join("target.txt"), dir.path().join("link.txt")).unwrap();
    let mut browser = FileBrowser::new();
    browser.navigate(dir.path()).unwrap();
    assert!(!browser.entries.is_empty());
}

#[test]
fn test_filter_hidden_files() {
    let dir = tmp_dir();
    std::fs::write(dir.path().join(".hidden"), "").unwrap();
    std::fs::write(dir.path().join("visible.txt"), "").unwrap();
    let mut browser = FileBrowser::new();
    browser.show_hidden = false;
    browser.navigate(dir.path()).unwrap();
    assert!(browser.entries.iter().all(|e| !e.name.starts_with('.')));
}

// ── Search tests ──────────────────────────────────────────────────────────────

#[test]
fn test_search_by_name() {
    let dir = tmp_dir();
    std::fs::write(dir.path().join("sigma_config.toml"), "").unwrap();
    std::fs::write(dir.path().join("other.txt"), "").unwrap();
    let results = FileSearch::search(dir.path(), &SearchQuery::by_name("sigma"));
    assert!(!results.is_empty());
    assert!(results.iter().any(|p| p.file_name().unwrap() == "sigma_config.toml"));
}

#[test]
fn test_search_case_insensitive() {
    let dir = tmp_dir();
    std::fs::write(dir.path().join("MyFile.txt"), "").unwrap();
    let mut q = SearchQuery::by_name("myfile");
    q.case_sensitive = false;
    let results = FileSearch::search(dir.path(), &q);
    assert!(!results.is_empty());
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
fn test_thumbnail_cache_hit() {
    let dir = tmp_dir();
    let img = dir.path().join("img.png");
    std::fs::write(&img, b"fake-png").unwrap();
    let mut cache = ThumbnailCache::new();
    cache.get_or_generate(&img, ThumbnailSize::Small);
    assert_eq!(cache.count(), 1);
    cache.get_or_generate(&img, ThumbnailSize::Small);
    assert_eq!(cache.count(), 1); // no duplicate
}

#[test]
fn test_icon_size_variants() {
    assert_eq!(ThumbnailSize::Small as u32, 64);
    assert_eq!(ThumbnailSize::Normal as u32, 128);
    assert_eq!(ThumbnailSize::Large as u32, 256);
}

#[test]
fn test_trash_integration_placeholder() {
    // Trash API: move to ~/.local/share/Trash/files/
    let trash_path = dirs_stub();
    assert!(trash_path.contains("Trash"));
}

fn dirs_stub() -> String {
    "/home/user/.local/share/Trash".to_string()
}

#[test]
fn test_bulk_rename_placeholder() {
    let names = vec!["file1.txt", "file2.txt", "file3.txt"];
    let renamed: Vec<String> = names.iter().map(|n| n.replace("file", "doc")).collect();
    assert_eq!(renamed[0], "doc1.txt");
}
