//! Thumbnail cache for SigmaOS-file-manager.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Thumbnail size variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThumbnailSize { Small = 64, Normal = 128, Large = 256 }

/// An in-memory thumbnail (stub; real impl returns pixel data).
#[derive(Debug, Clone)]
pub struct Thumbnail {
    pub path: PathBuf,
    pub size: ThumbnailSize,
    pub width: u32,
    pub height: u32,
}

/// Async thumbnail cache keyed by (path, size).
#[derive(Debug, Default)]
pub struct ThumbnailCache {
    cache: HashMap<(PathBuf, u32), Thumbnail>,
}

impl ThumbnailCache {
    pub fn new() -> Self { Self::default() }

    /// Request a thumbnail. Returns cached entry or generates a stub.
    pub fn get_or_generate(&mut self, path: &Path, size: ThumbnailSize) -> Option<&Thumbnail> {
        let key = (path.to_path_buf(), size as u32);
        if !self.cache.contains_key(&key) {
            // Real impl: spawn async task, decode image, resize
            let thumb = Thumbnail {
                path: path.to_path_buf(),
                size,
                width: size as u32,
                height: size as u32,
            };
            self.cache.insert(key.clone(), thumb);
        }
        self.cache.get(&key)
    }

    pub fn evict(&mut self, path: &Path) {
        self.cache.retain(|(p, _), _| p != path);
    }

    pub fn count(&self) -> usize {
        self.cache.len()
    }
}
