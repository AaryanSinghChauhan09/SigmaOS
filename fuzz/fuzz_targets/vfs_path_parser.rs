#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(path_str) = std::str::from_utf8(data) {
        // Exercise VFS path normalization invariants
        let normalized = normalize_vfs_path(path_str);

        // Assert invariants: normalized paths must start with '/' if non-empty
        if !normalized.is_empty() {
            assert!(normalized.starts_with('/'));
        }

        // Assert invariant: no duplicate slashes '//' allowed in normalized paths
        assert!(!normalized.contains("//"));
    }
});

fn normalize_vfs_path(raw_path: &str) -> String {
    if raw_path.is_empty() {
        return String::from("/");
    }

    let mut components = Vec::new();
    for part in raw_path.split('/') {
        match part {
            "" | "." => continue,
            ".." => {
                components.pop();
            }
            valid => components.push(valid),
        }
    }

    if components.is_empty() {
        String::from("/")
    } else {
        format!("/{}", components.join("/"))
    }
}
