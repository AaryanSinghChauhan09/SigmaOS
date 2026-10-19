//! SigmaPhoto - Photo Editing and Management System
//!
//! Provides comprehensive photo editing and management inspired by Photo Craft.
//! Supports image processing, filters, adjustments, and photo organization.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Image format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    JPEG,
    PNG,
    WEBP,
    TIFF,
    BMP,
    RAW,
;

/// Color space
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
    RGB,
    CMYK,
    Grayscale,
    Lab,
;

/// Adjustment type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdjustmentType {
    Brightness,
    Contrast,
    Saturation,
    Exposure,
    Highlights,
    Shadows,
    Temperature,
    Tint,
;

/// Photo adjustment
#[derive(Debug, Clone)]
pub struct PhotoAdjustment {
    pub adjustment_type: AdjustmentType,
    pub value: i32, // -100 to 100
;

impl PhotoAdjustment {
    pub fn new(adjustment_type: AdjustmentType, value: i32) -> Self {
        Self {
            adjustment_type,
            value,
        }
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }
;

/// Photo filter
#[derive(Debug, Clone)]
pub struct PhotoFilter {
    pub id: String,
    pub name: String,
    pub adjustments: Vec<PhotoAdjustment>,
;

impl PhotoFilter {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            adjustments: Vec::new(),
        }
    }

    pub fn add_adjustment(&mut self, adjustment: PhotoAdjustment) {
        self.adjustments.push(adjustment);
    }

    pub fn remove_adjustment(&mut self, index: usize) -> bool {
        if index < self.adjustments.len() {
            self.adjustments.remove(index);
            true
        } else {
            false
        }
    }

    pub fn adjustment_count(&self) -> usize {
        self.adjustments.len()
    }
;

/// Photo
#[derive(Debug, Clone)]
pub struct Photo {
    pub id: String,
    pub name: String,
    pub file_path: String,
    pub format: ImageFormat,
    pub color_space: ColorSpace,
    pub width: u32,
    pub height: u32,
    pub adjustments: Vec<PhotoAdjustment>,
    pub filters: Vec<String>, // filter IDs
;

impl Photo {
    pub fn new(id: String, name: String, file_path: String, format: ImageFormat, width: u32, height: u32) -> Self {
        Self {
            id,
            name,
            file_path,
            format,
            color_space: ColorSpace::RGB,
            width,
            height,
            adjustments: Vec::new(),
            filters: Vec::new(),
        }
    }

    pub fn add_adjustment(&mut self, adjustment: PhotoAdjustment) {
        self.adjustments.push(adjustment);
    }

    pub fn remove_adjustment(&mut self, index: usize) -> bool {
        if index < self.adjustments.len() {
            self.adjustments.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_filter(&mut self, filter_id: String) {
        self.filters.push(filter_id);
    }

    pub fn remove_filter(&mut self, index: usize) -> bool {
        if index < self.filters.len() {
            self.filters.remove(index);
            true
        } else {
            false
        }
    }

    pub fn set_color_space(&mut self, color_space: ColorSpace) {
        self.color_space = color_space;
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    pub fn adjustment_count(&self) -> usize {
        self.adjustments.len()
    }

    pub fn filter_count(&self) -> usize {
        self.filters.len()
    }
;

/// Photo album
#[derive(Debug, Clone)]
pub struct PhotoAlbum {
    pub id: String,
    pub name: String,
    pub photos: BTreeMap<String, Photo>,
;

impl PhotoAlbum {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            photos: BTreeMap::new(),
        }
    }

    pub fn add_photo(&mut self, photo: Photo) {
        self.photos.insert(photo.id.clone(), photo);
    }

    pub fn remove_photo(&mut self, id: &str) -> bool {
        self.photos.remove(id).is_some()
    }

    pub fn get_photo(&self, id: &str) -> Option<&Photo> {
        self.photos.get(id)
    }

    pub fn get_photo_mut(&mut self, id: &str) -> Option<&mut Photo> {
        self.photos.get_mut(id)
    }

    pub fn photo_count(&self) -> usize {
        self.photos.len()
    }
;

/// SigmaPhoto - Photo editing and management system
#[derive(Debug)]
pub struct SigmaPhoto {
    photos: BTreeMap<String, Photo>,
    albums: BTreeMap<String, PhotoAlbum>,
    filters: BTreeMap<String, PhotoFilter>,
    current_album: Option<String>,
;

impl SigmaPhoto {
    pub fn new() -> Self {
        Self {
            photos: BTreeMap::new(),
            albums: BTreeMap::new(),
            filters: BTreeMap::new(),
            current_album: None,
        }
    }

    pub fn add_photo(&mut self, photo: Photo) {
        self.photos.insert(photo.id.clone(), photo);
    }

    pub fn remove_photo(&mut self, id: &str) -> bool {
        self.photos.remove(id).is_some()
    }

    pub fn get_photo(&self, id: &str) -> Option<&Photo> {
        self.photos.get(id)
    }

    pub fn get_photo_mut(&mut self, id: &str) -> Option<&mut Photo> {
        self.photos.get_mut(id)
    }

    pub fn add_album(&mut self, album: PhotoAlbum) {
        self.albums.insert(album.id.clone(), album);
    }

    pub fn remove_album(&mut self, id: &str) -> bool {
        if self.current_album.as_ref() == Some(&id.to_string()) {
            self.current_album = None;
        }
        self.albums.remove(id).is_some()
    }

    pub fn get_album(&self, id: &str) -> Option<&PhotoAlbum> {
        self.albums.get(id)
    }

    pub fn add_filter(&mut self, filter: PhotoFilter) {
        self.filters.insert(filter.id.clone(), filter);
    }

    pub fn remove_filter(&mut self, id: &str) -> bool {
        self.filters.remove(id).is_some()
    }

    pub fn get_filter(&self, id: &str) -> Option<&PhotoFilter> {
        self.filters.get(id)
    }

    pub fn set_current_album(&mut self, id: &str) -> bool {
        if self.albums.contains_key(id) {
            self.current_album = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_album(&self) -> Option<&PhotoAlbum> {
        self.current_album.as_ref()
            .and_then(|id| self.albums.get(id))
    }

    pub fn list_photos(&self) -> Vec<&Photo> {
        self.photos.values().collect()
    }

    pub fn list_albums(&self) -> Vec<&PhotoAlbum> {
        self.albums.values().collect()
    }

    pub fn list_filters(&self) -> Vec<&PhotoFilter> {
        self.filters.values().collect()
    }

    pub fn photo_count(&self) -> usize {
        self.photos.len()
    }

    pub fn album_count(&self) -> usize {
        self.albums.len()
    }

    pub fn filter_count(&self) -> usize {
        self.filters.len()
    }
;

impl Default for SigmaPhoto {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaPhoto instance
static mut SIGMAPHOTO_INSTANCE: Option<SigmaPhoto> = None;

/// Initialize the global SigmaPhoto instance
pub fn init_sigmaphoto() {
    unsafe {
        SIGMAPHOTO_INSTANCE = Some(SigmaPhoto::new());
    }
;

/// Get the global SigmaPhoto instance
pub fn get_sigmaphoto() -> Option<&'static SigmaPhoto> {
    unsafe {
        SIGMAPHOTO_INSTANCE.as_ref()
    }
;

/// Get the global SigmaPhoto instance (mutable)
pub fn get_sigmaphoto_mut() -> Option<&'static mut SigmaPhoto> {
    unsafe {
        SIGMAPHOTO_INSTANCE.as_mut()
    }
;

/// Convenience function to add a photo
pub fn add_photo(photo: Photo) -> bool {
    if let Some(photo_sys) = get_sigmaphoto_mut() {
        photo_sys.add_photo(photo);
        true
    } else {
        false
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_photo_adjustment_creation() {
        let adj = PhotoAdjustment::new(AdjustmentType::Brightness, 50);
        assert_eq!(adj.adjustment_type, AdjustmentType::Brightness);
        assert_eq!(adj.value, 50);
    }

    #[test]
    fn test_photo_filter_creation() {
        let mut filter = PhotoFilter::new("filter1".to_string(), "Vintage".to_string());
        filter.add_adjustment(PhotoAdjustment::new(AdjustmentType::Saturation, -20));
        assert_eq!(filter.adjustment_count(), 1);
    }

    #[test]
    fn test_photo_creation() {
        let photo = Photo::new("photo1".to_string(), "Sunset".to_string(), "/path/to/image.jpg".to_string(), ImageFormat::JPEG, 1920, 1080);
        assert_eq!(photo.id, "photo1");
        assert_eq!(photo.format, ImageFormat::JPEG);
        assert_eq!(photo.get_aspect_ratio(), 1920.0 / 1080.0);
    }

    #[test]
    fn test_photo_modification() {
        let mut photo = Photo::new("photo1".to_string(), "Test".to_string(), "/path".to_string(), ImageFormat::JPEG, 1920, 1080);
        photo.add_adjustment(PhotoAdjustment::new(AdjustmentType::Brightness, 30));
        photo.add_filter("filter1".to_string());
        assert_eq!(photo.adjustment_count(), 1);
        assert_eq!(photo.filter_count(), 1);
    }

    #[test]
    fn test_photo_album_creation() {
        let album = PhotoAlbum::new("album1".to_string(), "Vacation".to_string());
        assert_eq!(album.id, "album1");
        assert_eq!(album.photo_count(), 0);
    }

    #[test]
    fn test_sigmaphoto_creation() {
        let photo_sys = SigmaPhoto::new();
        assert_eq!(photo_sys.photo_count(), 0);
        assert_eq!(photo_sys.album_count(), 0);
    }

    #[test]
    fn test_sigmaphoto_management() {
        let mut photo_sys = SigmaPhoto::new();
        let photo = Photo::new("photo1".to_string(), "Test".to_string(), "/path".to_string(), ImageFormat::JPEG, 1920, 1080);
        photo_sys.add_photo(photo);
        assert_eq!(photo_sys.photo_count(), 1);
    }
;
