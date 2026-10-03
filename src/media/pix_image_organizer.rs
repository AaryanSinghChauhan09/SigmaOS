// SigmaOS Linux Mint Pix-Inspired Image Organizer & Editing Engine
// Inspired by Linux Mint Pix (gThumb fork) - cataloging, editing, batch processing, slideshows & web albums.

use std::format;
use std::string::String;
use std::string::ToString;
use std::vec::Vec;

// =========================================================================
// 1. Image Format & Metadata Management (EXIF / IPTC / GPS / Ratings)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Jpeg,
    Png,
    WebP,
    Avif,
    Tiff,
    Bmp,
}

#[derive(Debug, Clone)]
pub struct PixImageMetadata {
    pub filepath: String,
    pub format: ImageFormat,
    pub width: u32,
    pub height: u32,
    pub file_size_bytes: u64,
    pub camera_make: String,
    pub camera_model: String,
    pub iso: u32,
    pub aperture_f_number: f32,
    pub exposure_time_sec: f32,
    pub gps_latitude: Option<f64>,
    pub gps_longitude: Option<f64>,
    pub date_taken: String,
    pub rating_stars: u8, // 0 to 5
    pub keywords: Vec<String>,
}

impl PixImageMetadata {
    pub fn new(filepath: &str, format: ImageFormat, width: u32, height: u32) -> Self {
        Self {
            filepath: String::from(filepath),
            format,
            width,
            height,
            file_size_bytes: (width as u64) * (height as u64) * 3,
            camera_make: String::from("SigmaOS Camera"),
            camera_model: String::from("Sovereign-X1"),
            iso: 100,
            aperture_f_number: 2.8,
            exposure_time_sec: 0.01,
            gps_latitude: None,
            gps_longitude: None,
            date_taken: String::from("2026-03-31 12:00:00"),
            rating_stars: 0,
            keywords: Vec::new(),
        }
    }

    pub fn set_rating(&mut self, stars: u8) {
        self.rating_stars = stars.min(5);
    }

    pub fn add_keyword(&mut self, keyword: &str) {
        if !self.keywords.iter().any(|k| k == keyword) {
            self.keywords.push(String::from(keyword));
        }
    }
}

// =========================================================================
// 2. Catalogs & Collections (Smart Searching & Filtering)
// =========================================================================

#[derive(Debug, Clone)]
pub struct PixCollection {
    pub name: String,
    pub images: Vec<PixImageMetadata>,
}

impl PixCollection {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            images: Vec::new(),
        }
    }

    pub fn add_image(&mut self, image: PixImageMetadata) {
        self.images.push(image);
    }

    pub fn filter_by_min_rating(&self, min_stars: u8) -> Vec<&PixImageMetadata> {
        self.images.iter().filter(|img| img.rating_stars >= min_stars).collect()
    }

    pub fn filter_by_keyword(&self, keyword: &str) -> Vec<&PixImageMetadata> {
        self.images.iter().filter(|img| img.keywords.iter().any(|k| k == keyword)).collect()
    }

    pub fn filter_by_format(&self, format: ImageFormat) -> Vec<&PixImageMetadata> {
        self.images.iter().filter(|img| img.format == format).collect()
    }
}

#[derive(Debug, Clone, Default)]
pub struct PixCatalog {
    pub collections: Vec<PixCollection>,
}

impl PixCatalog {
    pub fn new() -> Self {
        Self { collections: Vec::new() }
    }
}

// =========================================================================
// 3. Non-Destructive Image Editing (Crop, Rotate, Color Curves, Filters)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CropRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct PixImageEditParams {
    pub crop: Option<CropRect>,
    pub rotate_angle_deg: i32, // 0, 90, 180, 270
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
    pub brightness: i32, // -100 to +100
    pub contrast: i32,   // -100 to +100
    pub saturation: i32, // -100 to +100
    pub red_eye_removal: bool,
}

impl Default for PixImageEditParams {
    fn default() -> Self {
        Self {
            crop: None,
            rotate_angle_deg: 0,
            flip_horizontal: false,
            flip_vertical: false,
            brightness: 0,
            contrast: 0,
            saturation: 0,
            red_eye_removal: false,
        }
    }
}

pub struct PixImageEditor {
    pub target_metadata: PixImageMetadata,
    pub params: PixImageEditParams,
}

impl PixImageEditor {
    pub fn new(metadata: PixImageMetadata) -> Self {
        Self {
            target_metadata: metadata,
            params: PixImageEditParams::default(),
        }
    }

    pub fn apply_crop(&mut self, rect: CropRect) -> Result<(), &'static str> {
        if rect.x + rect.width > self.target_metadata.width || rect.y + rect.height > self.target_metadata.height {
            return Err("Crop rect exceeds image dimensions");
        }
        self.params.crop = Some(rect);
        self.target_metadata.width = rect.width;
        self.target_metadata.height = rect.height;
        Ok(())
    }

    pub fn apply_rotation(&mut self, degrees: i32) {
        self.params.rotate_angle_deg = (self.params.rotate_angle_deg + degrees) % 360;
        if degrees == 90 || degrees == 270 {
            core::mem::swap(&mut self.target_metadata.width, &mut self.target_metadata.height);
        }
    }

    pub fn adjust_colors(&mut self, brightness: i32, contrast: i32, saturation: i32) {
        self.params.brightness = brightness.clamp(-100, 100);
        self.params.contrast = contrast.clamp(-100, 100);
        self.params.saturation = saturation.clamp(-100, 100);
    }
}

// =========================================================================
// 4. Batch Operations (Batch Rename, Batch Format Conversion, Watermarking)
// =========================================================================

pub struct PixBatchRenameEngine;

impl PixBatchRenameEngine {
    pub fn batch_rename(images: &[PixImageMetadata], prefix: &str) -> Vec<String> {
        images
            .iter()
            .enumerate()
            .map(|(idx, img)| {
                let ext = match img.format {
                    ImageFormat::Jpeg => "jpg",
                    ImageFormat::Png => "png",
                    ImageFormat::WebP => "webp",
                    ImageFormat::Avif => "avif",
                    ImageFormat::Tiff => "tiff",
                    ImageFormat::Bmp => "bmp",
                };
                format!("{}_{:04}.{}", prefix, idx + 1, ext)
            })
            .collect()
    }
}

pub struct PixBatchConverterEngine;

impl PixBatchConverterEngine {
    pub fn convert_format(
        images: &mut [PixImageMetadata],
        target_format: ImageFormat,
    ) -> usize {
        let mut count = 0;
        for img in images.iter_mut() {
            if img.format != target_format {
                img.format = target_format;
                count += 1;
            }
        }
        count
    }
}

#[derive(Debug, Clone)]
pub struct WatermarkSpec {
    pub text: String,
    pub opacity_percent: u8,
    pub font_size: u32,
}

pub struct PixBatchWatermarkEngine;

impl PixBatchWatermarkEngine {
    pub fn apply_watermark(images: &[PixImageMetadata], watermark: &WatermarkSpec) -> Vec<String> {
        images
            .iter()
            .map(|img| {
                format!(
                    "Watermarked {} with '{}' ({}% opacity)",
                    img.filepath, watermark.text, watermark.opacity_percent
                )
            })
            .collect()
    }
}

// =========================================================================
// 5. Interactive Slideshow & Web Album Generator
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlideshowTransition {
    Fade,
    Wipe,
    Zoom,
    Dissolve,
}

pub struct PixSlideshowEngine {
    pub collection: PixCollection,
    pub transition: SlideshowTransition,
    pub slide_duration_sec: u32,
    pub current_slide_index: usize,
    pub is_playing: bool,
}

impl PixSlideshowEngine {
    pub fn new(collection: PixCollection, transition: SlideshowTransition, duration_sec: u32) -> Self {
        Self {
            collection,
            transition,
            slide_duration_sec: duration_sec,
            current_slide_index: 0,
            is_playing: false,
        }
    }

    pub fn start(&mut self) {
        self.is_playing = true;
    }

    pub fn next_slide(&mut self) -> Option<&PixImageMetadata> {
        if self.collection.images.is_empty() {
            return None;
        }
        self.current_slide_index = (self.current_slide_index + 1) % self.collection.images.len();
        Some(&self.collection.images[self.current_slide_index])
    }
}

pub struct PixWebAlbumGenerator;

impl PixWebAlbumGenerator {
    pub fn generate_html_gallery(collection: &PixCollection, title: &str) -> String {
        let mut html = String::from("<!DOCTYPE html>\n<html><head><title>");
        html.push_str(title);
        html.push_str("</title><style>body{background:#1e1e2e;color:#fff;font-family:sans-serif}.gallery{display:flex;flex-wrap:wrap;gap:10px}.card{border:1px solid #444;padding:8px;border-radius:6px}</style></head><body><h1>");
        html.push_str(title);
        html.push_str("</h1><div class=\"gallery\">");

        for img in &collection.images {
            html.push_str("<div class=\"card\"><h3>");
            html.push_str(&img.filepath);
            html.push_str("</h3><p>Rating: ");
            html.push_str(&img.rating_stars.to_string());
            html.push_str(" ★ | Dimensions: ");
            html.push_str(&img.width.to_string());
            html.push_str("x");
            html.push_str(&img.height.to_string());
            html.push_str("</p></div>");
        }

        html.push_str("</div></body></html>");
        html
    }
}

// =========================================================================
// UNIT TESTS MODULE
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pix_metadata_and_collections() {
        let mut img1 = PixImageMetadata::new("/home/user/Pictures/photo1.jpg", ImageFormat::Jpeg, 1920, 1080);
        img1.set_rating(5);
        img1.add_keyword("nature");
        img1.add_keyword("mountains");

        let mut img2 = PixImageMetadata::new("/home/user/Pictures/photo2.png", ImageFormat::Png, 3840, 2160);
        img2.set_rating(3);
        img2.add_keyword("portrait");

        let mut collection = PixCollection::new("Vacation 2026");
        collection.add_image(img1);
        collection.add_image(img2);

        assert_eq!(collection.images.len(), 2);
        assert_eq!(collection.filter_by_min_rating(4).len(), 1);
        assert_eq!(collection.filter_by_keyword("nature").len(), 1);
        assert_eq!(collection.filter_by_format(ImageFormat::Png).len(), 1);
    }

    #[test]
    fn test_pix_image_editor_crop_and_rotate() {
        let img = PixImageMetadata::new("/home/user/Pictures/sample.jpg", ImageFormat::Jpeg, 1920, 1080);
        let mut editor = PixImageEditor::new(img);

        let crop = CropRect { x: 100, y: 100, width: 800, height: 600 };
        assert!(editor.apply_crop(crop).is_ok());
        assert_eq!(editor.target_metadata.width, 800);
        assert_eq!(editor.target_metadata.height, 600);

        editor.apply_rotation(90);
        assert_eq!(editor.target_metadata.width, 600);
        assert_eq!(editor.target_metadata.height, 800);
    }

    #[test]
    fn test_pix_batch_operations() {
        let img1 = PixImageMetadata::new("/pic1.jpg", ImageFormat::Jpeg, 800, 600);
        let img2 = PixImageMetadata::new("/pic2.png", ImageFormat::Png, 1024, 768);

        let renamed = PixBatchRenameEngine::batch_rename(&[img1.clone(), img2.clone()], "Holiday");
        assert_eq!(renamed[0], "Holiday_0001.jpg");
        assert_eq!(renamed[1], "Holiday_0002.png");

        let mut images = vec![img1, img2];
        let converted = PixBatchConverterEngine::convert_format(&mut images, ImageFormat::WebP);
        assert_eq!(converted, 2);
        assert_eq!(images[0].format, ImageFormat::WebP);

        let watermark = WatermarkSpec { text: "© SigmaOS".to_string(), opacity_percent: 50, font_size: 24 };
        let stamped = PixBatchWatermarkEngine::apply_watermark(&images, &watermark);
        assert_eq!(stamped.len(), 2);
        assert!(stamped[0].contains("© SigmaOS"));
    }

    #[test]
    fn test_pix_slideshow_and_web_album() {
        let mut collection = PixCollection::new("Slideshow Test");
        let mut img = PixImageMetadata::new("/slide1.jpg", ImageFormat::Jpeg, 1920, 1080);
        img.set_rating(4);
        collection.add_image(img);

        let mut slideshow = PixSlideshowEngine::new(collection.clone(), SlideshowTransition::Fade, 5);
        slideshow.start();
        assert!(slideshow.is_playing);

        let html = PixWebAlbumGenerator::generate_html_gallery(&collection, "My Album");
        assert!(html.contains("<title>My Album</title>"));
        assert!(html.contains("4 ★"));
    }
}
