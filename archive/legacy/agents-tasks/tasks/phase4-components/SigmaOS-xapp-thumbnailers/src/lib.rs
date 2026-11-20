//! SigmaOS XApp thumbnailers — system file thumbnail providers.

#![allow(dead_code)]

use std::path::Path;
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ThumbnailSize { Small, Normal, Large, XLarge }
impl ThumbnailSize { pub fn px(&self) -> u32 { match self { Self::Small=>48, Self::Normal=>128, Self::Large=>256, Self::XLarge=>512 } } }
#[derive(Debug,Clone)]
pub struct ThumbnailRequest { pub path: String, pub size: ThumbnailSize, pub mime_type: String }
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum ThumbnailResult { Hit(String), Miss, Unsupported }
pub trait Thumbnailer: Send + Sync {
    fn supported_mimes(&self) -> &[&str];
    fn generate(&self, req: &ThumbnailRequest) -> ThumbnailResult;
}
pub struct ImageThumbnailer;
impl Thumbnailer for ImageThumbnailer {
    fn supported_mimes(&self) -> &[&str] { &["image/png","image/jpeg","image/webp","image/gif","image/bmp"] }
    fn generate(&self, req: &ThumbnailRequest) -> ThumbnailResult {
        if self.supported_mimes().contains(&req.mime_type.as_str()) {
            ThumbnailResult::Hit(format!("/tmp/sigma-thumb-{}-{}.png", req.size.px(), std::collections::hash_map::DefaultHasher::new().finish_with(req.path.as_bytes())))
        } else { ThumbnailResult::Unsupported }
    }
}
use std::hash::{Hash,Hasher}; trait FinishWith { fn finish_with(&mut self, data: &[u8]) -> u64; }
impl FinishWith for std::collections::hash_map::DefaultHasher {
    fn finish_with(&mut self, data: &[u8]) -> u64 { data.iter().for_each(|b| self.write_u8(*b)); std::hash::Hasher::finish(self) }
}
#[cfg(test)] mod tests { use super::*;
  #[test] fn image_thumbnailer_hit() { let t = ImageThumbnailer; let req = ThumbnailRequest { path: "/home/user/photo.jpg".into(), size: ThumbnailSize::Normal, mime_type: "image/jpeg".into() }; assert!(matches!(t.generate(&req), ThumbnailResult::Hit(_))); }
  #[test] fn unsupported_mime() { let t = ImageThumbnailer; let req = ThumbnailRequest { path: "/home/user/doc.pdf".into(), size: ThumbnailSize::Normal, mime_type: "application/pdf".into() }; assert_eq!(t.generate(&req), ThumbnailResult::Unsupported); }
  #[test] fn thumbnail_sizes() { assert_eq!(ThumbnailSize::Normal.px(), 128); assert_eq!(ThumbnailSize::Large.px(), 256); }
}
