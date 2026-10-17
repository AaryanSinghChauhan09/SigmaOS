// SPDX-License-Identifier: MIT
// SigmaOS - Image Viewer
// Inspired by eog (Eye of GNOME) and feh
// Provides image viewing and basic editing capabilities

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Zoom mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZoomMode {
    Fit,
    Fill,
    Original,
    Custom(f32),
}

/// Rotation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
    Degrees0,
    Degrees90,
    Degrees180,
    Degrees270,
}

/// Flip direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlipDirection {
    Horizontal,
    Vertical,
}

/// Slideshow mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlideshowMode {
    Off,
    Sequential,
    Random,
}

/// Image metadata
#[derive(Debug, Clone)]
pub struct ImageMetadata {
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub size_bytes: u64,
    pub has_alpha: bool,
}

/// Image
#[derive(Debug, Clone)]
pub struct Image {
    pub path: String,
    pub name: String,
    pub metadata: ImageMetadata,
    pub rotation: Rotation,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
    pub brightness: f32,
    pub contrast: f32,
    pub saturation: f32,
}

/// Image viewer
pub struct ImageViewer {
    pub images: Vec<Image>,
    pub current_index: usize,
    pub zoom: f32,
    pub zoom_mode: ZoomMode,
    pub slideshow_mode: SlideshowMode,
    pub slideshow_interval: u32,
    pub show_info: bool,
    pub fullscreen: bool,
    pub fit_to_window: bool,
}

impl ImageViewer {
    pub fn new() -> Self {
        Self {
            images: Vec::new(),
            current_index: 0,
            zoom: 1.0,
            zoom_mode: ZoomMode::Fit,
            slideshow_mode: SlideshowMode::Off,
            slideshow_interval: 5,
            show_info: true,
            fullscreen: false,
            fit_to_window: true,
        }
    }

    /// Add image
    pub fn add_image(&mut self, path: String, name: String, metadata: ImageMetadata) {
        let image = Image {
            path,
            name,
            metadata,
            rotation: Rotation::Degrees0,
            flip_horizontal: false,
            flip_vertical: false,
            brightness: 0.0,
            contrast: 0.0,
            saturation: 0.0,
        };
        self.images.push(image);
    }

    /// Remove image
    pub fn remove_image(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.images.len() {
            return Err("Image index out of bounds");
        }
        self.images.remove(index);
        if self.current_index >= self.images.len() && !self.images.is_empty() {
            self.current_index = self.images.len() - 1;
        }
        Ok(())
    }

    /// Load image
    pub fn load_image(&mut self, path: String) -> Result<(), &'static str> {
        // Simplified - in real implementation would load actual image
        let name = path.split('/').last().unwrap_or("image").to_string();
        let metadata = ImageMetadata {
            width: 1920,
            height: 1080,
            format: "PNG".to_string(),
            size_bytes: 1024 * 1024,
            has_alpha: true,
        };
        self.add_image(path, name, metadata);
        Ok(())
    }

    /// Load directory
    pub fn load_directory(&mut self, path: String) -> Result<(), &'static str> {
        // Simplified - in real implementation would scan directory for images
        Ok(())
    }

    /// Next image
    pub fn next_image(&mut self) {
        if self.images.is_empty() {
            return;
        }
        if self.current_index < self.images.len() - 1 {
            self.current_index += 1;
        } else {
            self.current_index = 0;
        }
        self.reset_image_adjustments();
    }

    /// Previous image
    pub fn previous_image(&mut self) {
        if self.images.is_empty() {
            return;
        }
        if self.current_index > 0 {
            self.current_index -= 1;
        } else {
            self.current_index = self.images.len() - 1;
        }
        self.reset_image_adjustments();
    }

    /// Go to image
    pub fn go_to_image(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.images.len() {
            return Err("Image index out of bounds");
        }
        self.current_index = index;
        self.reset_image_adjustments();
        Ok(())
    }

    /// Get current image
    pub fn get_current_image(&self) -> Option<&Image> {
        self.images.get(self.current_index)
    }

    /// Zoom in
    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom * 1.25).min(10.0);
        self.zoom_mode = ZoomMode::Custom(self.zoom);
    }

    /// Zoom out
    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom / 1.25).max(0.1);
        self.zoom_mode = ZoomMode::Custom(self.zoom);
    }

    /// Set zoom
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.max(0.1).min(10.0);
        self.zoom_mode = ZoomMode::Custom(self.zoom);
    }

    /// Set zoom mode
    pub fn set_zoom_mode(&mut self, mode: ZoomMode) {
        self.zoom_mode = mode;
        match mode {
            ZoomMode::Fit => self.zoom = 1.0,
            ZoomMode::Fill => self.zoom = 1.0,
            ZoomMode::Original => self.zoom = 1.0,
            ZoomMode::Custom(z) => self.zoom = z,
        }
    }

    /// Rotate image
    pub fn rotate(&mut self, direction: Rotation) {
        if let Some(image) = self.images.get_mut(self.current_index) {
            image.rotation = direction;
        }
    }

    /// Rotate clockwise
    pub fn rotate_clockwise(&mut self) {
        if let Some(image) = self.images.get_mut(self.current_index) {
            image.rotation = match image.rotation {
                Rotation::Degrees0 => Rotation::Degrees90,
                Rotation::Degrees90 => Rotation::Degrees180,
                Rotation::Degrees180 => Rotation::Degrees270,
                Rotation::Degrees270 => Rotation::Degrees0,
            };
        }
    }

    /// Rotate counter-clockwise
    pub fn rotate_counter_clockwise(&mut self) {
        if let Some(image) = self.images.get_mut(self.current_index) {
            image.rotation = match image.rotation {
                Rotation::Degrees0 => Rotation::Degrees270,
                Rotation::Degrees90 => Rotation::Degrees0,
                Rotation::Degrees180 => Rotation::Degrees90,
                Rotation::Degrees270 => Rotation::Degrees180,
            };
        }
    }

    /// Flip image
    pub fn flip(&mut self, direction: FlipDirection) {
        if let Some(image) = self.images.get_mut(self.current_index) {
            match direction {
                FlipDirection::Horizontal => image.flip_horizontal = !image.flip_horizontal,
                FlipDirection::Vertical => image.flip_vertical = !image.flip_vertical,
            }
        }
    }

    /// Set brightness
    pub fn set_brightness(&mut self, brightness: f32) {
        if let Some(image) = self.images.get_mut(self.current_index) {
            image.brightness = brightness.clamp(-100.0, 100.0);
        }
    }

    /// Set contrast
    pub fn set_contrast(&mut self, contrast: f32) {
        if let Some(image) = self.images.get_mut(self.current_index) {
            image.contrast = contrast.clamp(-100.0, 100.0);
        }
    }

    /// Set saturation
    pub fn set_saturation(&mut self, saturation: f32) {
        if let Some(image) = self.images.get_mut(self.current_index) {
            image.saturation = saturation.clamp(-100.0, 100.0);
        }
    }

    /// Reset image adjustments
    fn reset_image_adjustments(&mut self) {
        if let Some(image) = self.images.get_mut(self.current_index) {
            image.rotation = Rotation::Degrees0;
            image.flip_horizontal = false;
            image.flip_vertical = false;
            image.brightness = 0.0;
            image.contrast = 0.0;
            image.saturation = 0.0;
        }
    }

    /// Reset all adjustments
    pub fn reset_all_adjustments(&mut self) {
        self.reset_image_adjustments();
        self.zoom = 1.0;
        self.zoom_mode = ZoomMode::Fit;
    }

    /// Toggle fullscreen
    pub fn toggle_fullscreen(&mut self) {
        self.fullscreen = !self.fullscreen;
    }

    /// Toggle fit to window
    pub fn toggle_fit_to_window(&mut self) {
        self.fit_to_window = !self.fit_to_window;
    }

    /// Toggle info display
    pub fn toggle_info(&mut self) {
        self.show_info = !self.show_info;
    }

    /// Set slideshow mode
    pub fn set_slideshow_mode(&mut self, mode: SlideshowMode) {
        self.slideshow_mode = mode;
    }

    /// Set slideshow interval
    pub fn set_slideshow_interval(&mut self, interval: u32) {
        self.slideshow_interval = interval.max(1);
    }

    /// Start slideshow
    pub fn start_slideshow(&mut self) {
        self.slideshow_mode = SlideshowMode::Sequential;
    }

    /// Stop slideshow
    pub fn stop_slideshow(&mut self) {
        self.slideshow_mode = SlideshowMode::Off;
    }

    /// Slideshow step
    pub fn slideshow_step(&mut self) {
        match self.slideshow_mode {
            SlideshowMode::Sequential => self.next_image(),
            SlideshowMode::Random => {
                if !self.images.is_empty() {
                    let index = (self.current_index + 1) % self.images.len();
                    self.current_index = index;
                    self.reset_image_adjustments();
                }
            }
            SlideshowMode::Off => {}
        }
    }

    /// Image count
    pub fn image_count(&self) -> usize {
        self.images.len()
    }

    /// Current index
    pub fn current_index(&self) -> usize {
        self.current_index
    }
}

/// Global image viewer
static mut IMAGE_VIEWER: Option<ImageViewer> = None;

/// Initialize image viewer
pub fn init_image_viewer() {
    unsafe {
        IMAGE_VIEWER = Some(ImageViewer::new());
    }
}

/// Get image viewer
pub fn get_image_viewer() -> &'static mut ImageViewer {
    unsafe { IMAGE_VIEWER.as_mut().expect("Image viewer not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_viewer_creation() {
        let viewer = ImageViewer::new();
        assert_eq!(viewer.image_count(), 0);
    }

    #[test]
    fn test_add_image() {
        let mut viewer = ImageViewer::new();
        let metadata = ImageMetadata {
            width: 1920,
            height: 1080,
            format: "PNG".to_string(),
            size_bytes: 1024 * 1024,
            has_alpha: true,
        };
        viewer.add_image(
            "/path/to/image.png".to_string(),
            "image.png".to_string(),
            metadata,
        );
        assert_eq!(viewer.image_count(), 1);
    }

    #[test]
    fn test_next_image() {
        let mut viewer = ImageViewer::new();
        let metadata = ImageMetadata {
            width: 1920,
            height: 1080,
            format: "PNG".to_string(),
            size_bytes: 1024 * 1024,
            has_alpha: true,
        };
        viewer.add_image(
            "image1.png".to_string(),
            "image1.png".to_string(),
            metadata.clone(),
        );
        viewer.add_image("image2.png".to_string(), "image2.png".to_string(), metadata);
        viewer.next_image();
        assert_eq!(viewer.current_index(), 1);
    }

    #[test]
    fn test_zoom_in() {
        let mut viewer = ImageViewer::new();
        viewer.zoom_in();
        assert!(viewer.zoom > 1.0);
    }

    #[test]
    fn test_zoom_out() {
        let mut viewer = ImageViewer::new();
        viewer.zoom = 2.0;
        viewer.zoom_out();
        assert!(viewer.zoom < 2.0);
    }

    #[test]
    fn test_rotate_clockwise() {
        let mut viewer = ImageViewer::new();
        let metadata = ImageMetadata {
            width: 1920,
            height: 1080,
            format: "PNG".to_string(),
            size_bytes: 1024 * 1024,
            has_alpha: true,
        };
        viewer.add_image("image.png".to_string(), "image.png".to_string(), metadata);
        viewer.rotate_clockwise();
        assert_eq!(
            viewer.get_current_image().unwrap().rotation,
            Rotation::Degrees90
        );
    }

    #[test]
    fn test_toggle_fullscreen() {
        let mut viewer = ImageViewer::new();
        viewer.toggle_fullscreen();
        assert!(viewer.fullscreen);
    }
}
