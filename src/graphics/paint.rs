extern crate alloc;
// SigmaOS Sovereign AI-Native Photo Editing Suite (SigmaPaint)
// Designed for high-performance raster image canvas and layer filtering

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Image processing error states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhotoError {
    Success = 0,
    InvalidDimensions = 1,
    LayerOutOfBounds = 2,
    NotSupported = 3,
    ProcessingFailed = 4,
}

/// Color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorRgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorRgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        ColorRgba { r, g, b, a }
    }
}

/// Layer blend modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
}

/// Base OOP interface representing any image processing filter
pub trait ImageFilter {
    fn apply_filter(
        &self,
        width: u32,
        height: u32,
        pixels: &mut [ColorRgba],
    ) -> Result<(), PhotoError>;
}

/// Base OOP interface representing a composite layer inside a Canvas
pub trait CanvasLayer {
    fn name(&self) -> &str;
    fn opacity(&self) -> f32; // 0.0 to 1.0
    fn blend_mode(&self) -> BlendMode;
    fn get_pixels(&self) -> &[ColorRgba];
    fn get_pixels_mut(&mut self) -> &mut [ColorRgba];
}

// ==========================================
// 1. Concrete Canvas Layer Implementation
// ==========================================

pub struct RasterLayer {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub pixels: Vec<ColorRgba>,
}

impl RasterLayer {
    pub fn new(name: String, width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let mut pixels = Vec::new();
        for _ in 0..size {
            pixels.push(ColorRgba::new(0, 0, 0, 0));
        }
        RasterLayer {
            name,
            width,
            height,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            pixels,
        }
    }
}

impl CanvasLayer for RasterLayer {
    fn name(&self) -> &str {
        &self.name
    }
    fn opacity(&self) -> f32 {
        self.opacity
    }
    fn blend_mode(&self) -> BlendMode {
        self.blend_mode
    }
    fn get_pixels(&self) -> &[ColorRgba] {
        &self.pixels
    }
    fn get_pixels_mut(&mut self) -> &mut [ColorRgba] {
        &mut self.pixels
    }
}

// ==========================================
// 2. Concrete Convolution Gaussian Blur Filter
// ==========================================

pub struct GaussianBlurFilter {
    pub radius: u32,
}

impl GaussianBlurFilter {
    pub fn new(radius: u32) -> Self {
        GaussianBlurFilter { radius }
    }
}

impl ImageFilter for GaussianBlurFilter {
    fn apply_filter(
        &self,
        width: u32,
        height: u32,
        pixels: &mut [ColorRgba],
    ) -> Result<(), PhotoError> {
        if width == 0 || height == 0 || pixels.len() != (width * height) as usize {
            return Err(PhotoError::InvalidDimensions);
        }

        // Simple box-blur representing convolution filter for valid no_std environments
        let mut temp_pixels = Vec::new();
        for &p in pixels.iter() {
            temp_pixels.push(p);
        }

        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                let idx = (y * width + x) as usize;

                // Average 3x3 surrounding pixels
                let mut sum_r: u32 = 0;
                let mut sum_g: u32 = 0;
                let mut sum_b: u32 = 0;
                let mut sum_a: u32 = 0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let offset_idx =
                            (((y as i32 + dy) * width as i32) + (x as i32 + dx)) as usize;
                        let p = temp_pixels[offset_idx];
                        sum_r += p.r as u32;
                        sum_g += p.g as u32;
                        sum_b += p.b as u32;
                        sum_a += p.a as u32;
                    }
                }

                pixels[idx] = ColorRgba::new(
                    (sum_r / 9) as u8,
                    (sum_g / 9) as u8,
                    (sum_b / 9) as u8,
                    (sum_a / 9) as u8,
                );
            }
        }

        Ok(())
    }
}

// ==========================================
// 3. Complete Color Space Conversion Filter
// ==========================================

pub struct GrayscaleConversionFilter;

impl ImageFilter for GrayscaleConversionFilter {
    fn apply_filter(
        &self,
        _width: u32,
        _height: u32,
        pixels: &mut [ColorRgba],
    ) -> Result<(), PhotoError> {
        for pixel in pixels.iter_mut() {
            // Standard NTSC Grayscale coefficients
            let gray =
                (0.299 * pixel.r as f32 + 0.587 * pixel.g as f32 + 0.114 * pixel.b as f32) as u8;
            pixel.r = gray;
            pixel.g = gray;
            pixel.b = gray;
        }
        Ok(())
    }
}

// ==========================================================
// 4. GIMP & Krita Inspired Non-Destructive Layer Mask Engine
// ==========================================================

/// Layer mask representation holding grayscale transparency values (0 = fully masked/hidden, 255 = fully visible)
pub struct SigmaLayerMask {
    pub width: u32,
    pub height: u32,
    pub mask_bytes: Vec<u8>,
    pub enabled: bool,
    pub inverted: bool,
}

impl SigmaLayerMask {
    pub fn new(width: u32, height: u32, initial_value: u8) -> Self {
        let size = (width * height) as usize;
        SigmaLayerMask {
            width,
            height,
            mask_bytes: alloc::vec![initial_value; size],
            enabled: true,
            inverted: false,
        }
    }

    pub fn get_value(&self, x: u32, y: u32) -> u8 {
        if x >= self.width || y >= self.height {
            return 0;
        }
        let idx = (y * self.width + x) as usize;
        let val = self.mask_bytes[idx];
        if self.inverted {
            255 - val
        } else {
            val
        }
    }

    pub fn set_value(&mut self, x: u32, y: u32, value: u8) {
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) as usize;
            self.mask_bytes[idx] = value;
        }
    }
}

/// GIMP/Krita-inspired Non-Destructive Layer Mask, Alpha Lock, & Clipping Mask Manager
pub struct SigmaLayerMaskEngine {
    pub alpha_locked: bool,
    pub is_clipping_mask: bool,
    pub mask: Option<SigmaLayerMask>,
}

impl SigmaLayerMaskEngine {
    pub fn new() -> Self {
        SigmaLayerMaskEngine {
            alpha_locked: false,
            is_clipping_mask: false,
            mask: None,
        }
    }

    pub fn attach_mask(&mut self, mask: SigmaLayerMask) {
        self.mask = Some(mask);
    }

    pub fn apply_mask_to_layer(&self, width: u32, height: u32, pixels: &mut [ColorRgba]) {
        if let Some(ref m) = self.mask {
            if !m.enabled {
                return;
            }
            for y in 0..height {
                for x in 0..width {
                    let idx = (y * width + x) as usize;
                    if idx < pixels.len() {
                        let mask_factor = m.get_value(x, y) as f32 / 255.0;
                        let curr_a = pixels[idx].a as f32;
                        pixels[idx].a = (curr_a * mask_factor) as u8;
                    }
                }
            }
        }
    }
}

// ==========================================================
// 5. MyPaint & Krita Inspired Pressure-Sensitive Brush Engine
// ==========================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BrushPoint {
    pub x: f32,
    pub y: f32,
    pub pressure: f32, // 0.0 to 1.0 (stylus/graphics tablet pressure)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrushType {
    Pencil,
    SoftRound,
    Airbrush,
    Calligraphy,
    PixelArt,
}

pub struct SovereignBrushEngine {
    pub brush_type: BrushType,
    pub base_size: f32,
    pub opacity: f32,
    pub hardness: f32, // 0.0 (soft) to 1.0 (hard edge)
    pub color: ColorRgba,
    pub spacing: f32,
}

impl SovereignBrushEngine {
    pub fn new(brush_type: BrushType, size: f32, color: ColorRgba) -> Self {
        SovereignBrushEngine {
            brush_type,
            base_size: size,
            opacity: 1.0,
            hardness: 0.8,
            color,
            spacing: 0.25,
        }
    }

    /// Calculates Catmull-Rom spline interpolation between stroke control points for smooth curves
    pub fn interpolate_catmull_rom(p0: BrushPoint, p1: BrushPoint, p2: BrushPoint, p3: BrushPoint, t: f32) -> BrushPoint {
        let t2 = t * t;
        let t3 = t2 * t;

        let f0 = -0.5 * t3 + t2 - 0.5 * t;
        let f1 = 1.5 * t3 - 2.5 * t2 + 1.0;
        let f2 = -1.5 * t3 + 2.0 * t2 + 0.5 * t;
        let f3 = 0.5 * t3 - 0.5 * t2;

        let x = p0.x * f0 + p1.x * f1 + p2.x * f2 + p3.x * f3;
        let y = p0.y * f0 + p1.y * f1 + p2.y * f2 + p3.y * f3;
        let pressure = p0.pressure * f0 + p1.pressure * f1 + p2.pressure * f2 + p3.pressure * f3;

        BrushPoint {
            x,
            y,
            pressure: pressure.clamp(0.0, 1.0),
        }
    }

    /// Draws a dab stroke point onto raster layer pixels considering pressure sensitivity
    pub fn paint_dab(&self, point: BrushPoint, width: u32, height: u32, pixels: &mut [ColorRgba]) {
        let dynamic_radius = (self.base_size * point.pressure).max(1.0);
        let center_x = point.x;
        let center_y = point.y;

        let min_x = ((center_x - dynamic_radius).floor() as i32).max(0) as u32;
        let max_x = ((center_x + dynamic_radius).ceil() as i32).min(width as i32 - 1) as u32;
        let min_y = ((center_y - dynamic_radius).floor() as i32).max(0) as u32;
        let max_y = ((center_y + dynamic_radius).ceil() as i32).min(height as i32 - 1) as u32;

        for py in min_y..=max_y {
            for px in min_x..=max_x {
                let dx = px as f32 - center_x;
                let dy = py as f32 - center_y;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist <= dynamic_radius {
                    let idx = (py * width + px) as usize;
                    if idx < pixels.len() {
                        let falloff = if self.hardness >= 1.0 {
                            1.0
                        } else {
                            let inner = dynamic_radius * self.hardness;
                            if dist <= inner {
                                1.0
                            } else {
                                (1.0 - (dist - inner) / (dynamic_radius - inner)).clamp(0.0, 1.0)
                            }
                        };

                        let alpha_factor = self.opacity * point.pressure * falloff;
                        let src_a = (self.color.a as f32 * alpha_factor) as u8;

                        if src_a > 0 {
                            let dst = pixels[idx];
                            let blended_r = ((self.color.r as u32 * src_a as u32 + dst.r as u32 * (255 - src_a) as u32) / 255) as u8;
                            let blended_g = ((self.color.g as u32 * src_a as u32 + dst.g as u32 * (255 - src_a) as u32) / 255) as u8;
                            let blended_b = ((self.color.b as u32 * src_a as u32 + dst.b as u32 * (255 - src_a) as u32) / 255) as u8;
                            let blended_a = dst.a.saturating_add(src_a);

                            pixels[idx] = ColorRgba::new(blended_r, blended_g, blended_b, blended_a);
                        }
                    }
                }
            }
        }
    }
}

// ==========================================================
// 6. GIMP & Pinta Inspired Selection Engine
// ==========================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    Replace,
    Add,
    Subtract,
    Intersect,
}

/// Selection mask representing active pixel selection boundaries
pub struct SigmaSelectionEngine {
    pub width: u32,
    pub height: u32,
    pub selection_bytes: Vec<u8>, // 0 = unselected, 255 = fully selected
    pub active_mode: SelectionMode,
}

impl SigmaSelectionEngine {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        SigmaSelectionEngine {
            width,
            height,
            selection_bytes: alloc::vec![0; size],
            active_mode: SelectionMode::Replace,
        }
    }

    pub fn select_all(&mut self) {
        for val in self.selection_bytes.iter_mut() {
            *val = 255;
        }
    }

    pub fn clear_selection(&mut self) {
        for val in self.selection_bytes.iter_mut() {
            *val = 0;
        }
    }

    pub fn select_rectangle(&mut self, x0: u32, y0: u32, x1: u32, y1: u32) {
        let min_x = x0.min(x1).min(self.width);
        let max_x = x0.max(x1).min(self.width);
        let min_y = y0.min(y1).min(self.height);
        let max_y = y0.max(y1).min(self.height);

        if self.active_mode == SelectionMode::Replace {
            self.clear_selection();
        }

        for y in min_y..max_y {
            for x in min_x..max_x {
                let idx = (y * self.width + x) as usize;
                match self.active_mode {
                    SelectionMode::Replace | SelectionMode::Add => self.selection_bytes[idx] = 255,
                    SelectionMode::Subtract => self.selection_bytes[idx] = 0,
                    SelectionMode::Intersect => {
                        if self.selection_bytes[idx] == 0 {
                            self.selection_bytes[idx] = 0;
                        }
                    }
                }
            }
        }
    }

    pub fn is_selected(&self, x: u32, y: u32) -> bool {
        if x >= self.width || y >= self.height {
            false
        } else {
            let idx = (y * self.width + x) as usize;
            self.selection_bytes[idx] > 127
        }
    }
}

// ==========================================================
// 7. Inkscape & Krita Inspired Vector Path Engine
// ==========================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VectorControlPoint {
    pub x: f32,
    pub y: f32,
    pub handle_in_x: f32,
    pub handle_in_y: f32,
    pub handle_out_x: f32,
    pub handle_out_y: f32,
}

/// Vector Bezier curve spline path engine
pub struct SigmaVectorPathEngine {
    pub points: Vec<VectorControlPoint>,
    pub closed: bool,
    pub stroke_width: f32,
    pub stroke_color: ColorRgba,
    pub fill_color: Option<ColorRgba>,
}

impl SigmaVectorPathEngine {
    pub fn new() -> Self {
        SigmaVectorPathEngine {
            points: Vec::new(),
            closed: false,
            stroke_width: 2.0,
            stroke_color: ColorRgba::new(0, 0, 0, 255),
            fill_color: None,
        }
    }

    pub fn add_point(&mut self, pt: VectorControlPoint) {
        self.points.push(pt);
    }

    /// Evaluates cubic Bezier curve point given control handles
    pub fn evaluate_cubic_bezier(p0_x: f32, p0_y: f32, c0_x: f32, c0_y: f32, c1_x: f32, c1_y: f32, p1_x: f32, p1_y: f32, t: f32) -> (f32, f32) {
        let u = 1.0 - t;
        let tt = t * t;
        let uu = u * u;
        let uuu = uu * u;
        let ttt = tt * t;

        let x = uuu * p0_x + 3.0 * uu * t * c0_x + 3.0 * u * tt * c1_x + ttt * p1_x;
        let y = uuu * p0_y + 3.0 * uu * t * c0_y + 3.0 * u * tt * c1_y + ttt * p1_y;

        (x, y)
    }

    /// Rasterizes vector Bezier path onto target pixel canvas buffer
    pub fn stroke_path_onto_canvas(&self, width: u32, height: u32, pixels: &mut [ColorRgba]) {
        if self.points.len() < 2 {
            return;
        }

        let segment_count = self.points.len() - if self.closed { 0 } else { 1 };
        let brush = SovereignBrushEngine::new(BrushType::PixelArt, self.stroke_width, self.stroke_color);

        for i in 0..segment_count {
            let p0 = self.points[i];
            let p1 = self.points[(i + 1) % self.points.len()];

            let steps = 30;
            for s in 0..=steps {
                let t = s as f32 / steps as f32;
                let (bx, by) = Self::evaluate_cubic_bezier(
                    p0.x, p0.y,
                    p0.handle_out_x, p0.handle_out_y,
                    p1.handle_in_x, p1.handle_in_y,
                    p1.x, p1.y,
                    t,
                );

                let bp = BrushPoint { x: bx, y: by, pressure: 1.0 };
                brush.paint_dab(bp, width, height, pixels);
            }
        }
    }
}

// ==========================================================
// 8. GIMP Palette & Color Harmony Engine
// ==========================================================

pub struct SigmaPaletteManager {
    pub palette_name: String,
    pub colors: Vec<ColorRgba>,
    pub color_history: Vec<ColorRgba>,
}

impl SigmaPaletteManager {
    pub fn new(name: &str) -> Self {
        SigmaPaletteManager {
            palette_name: name.to_string(),
            colors: Vec::new(),
            color_history: Vec::new(),
        }
    }

    /// Parses GIMP Palette (.gpl) text format catalog
    pub fn load_gpl_palette(&mut self, gpl_text: &str) -> usize {
        self.colors.clear();
        for line in gpl_text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("GIMP Palette") || trimmed.starts_with("Name:") || trimmed.starts_with("Columns:") {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 3 {
                if let (Ok(r), Ok(g), Ok(b)) = (parts[0].parse::<u8>(), parts[1].parse::<u8>(), parts[2].parse::<u8>()) {
                    self.colors.push(ColorRgba::new(r, g, b, 255));
                }
            }
        }
        self.colors.len()
    }

    /// Calculates complementary color harmony
    pub fn compute_complementary_color(color: ColorRgba) -> ColorRgba {
        ColorRgba::new(255 - color.r, 255 - color.g, 255 - color.b, color.a)
    }

    pub fn record_color_usage(&mut self, color: ColorRgba) {
        self.color_history.retain(|c| *c != color);
        self.color_history.insert(0, color);
        if self.color_history.len() > 16 {
            self.color_history.pop();
        }
    }
}

// ==========================================================
// 9. Sovereign Image Exporter (PPM, QOI, BMP)
// ==========================================================

pub struct SigmaImageExporter;

impl SigmaImageExporter {
    /// Exports canvas pixel buffer as Netpbm Portable Pixmap (PPM ASCII P3) image format
    pub fn export_ppm(width: u32, height: u32, pixels: &[ColorRgba]) -> Result<String, PhotoError> {
        if width == 0 || height == 0 || pixels.len() != (width * height) as usize {
            return Err(PhotoError::InvalidDimensions);
        }

        let mut ppm = alloc::format!("P3\n{} {}\n255\n", width, height);
        for pixel in pixels {
            ppm.push_str(&alloc::format!("{} {} {} ", pixel.r, pixel.g, pixel.b));
        }
        ppm.push('\n');
        Ok(ppm)
    }

    /// Exports canvas pixel buffer as Quite OK Image (QOI) lightweight binary byte stream
    pub fn export_qoi_bytes(width: u32, height: u32, pixels: &[ColorRgba]) -> Result<Vec<u8>, PhotoError> {
        if width == 0 || height == 0 || pixels.len() != (width * height) as usize {
            return Err(PhotoError::InvalidDimensions);
        }

        let mut bytes = Vec::new();
        // QOI Magic "qoif"
        bytes.extend_from_slice(b"qoif");
        bytes.extend_from_slice(&width.to_be_bytes());
        bytes.extend_from_slice(&height.to_be_bytes());
        bytes.push(4); // RGBA channels
        bytes.push(0); // sRGB colorspace

        // Raw RGBA pixels stream encoding
        for pixel in pixels {
            bytes.push(254); // QOI_OP_RGBA tag
            bytes.push(pixel.r);
            bytes.push(pixel.g);
            bytes.push(pixel.b);
            bytes.push(pixel.a);
        }

        // QOI End of stream marker (7 0x00 bytes + 0x01)
        bytes.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 1]);
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raster_layer_creation() {
        let layer = RasterLayer::new("Background".to_string(), 10, 10);
        assert_eq!(layer.name(), "Background");
        assert_eq!(layer.get_pixels().len(), 100);
    }

    #[test]
    fn test_grayscale_filter() {
        let mut layer = RasterLayer::new("Layer 1".to_string(), 2, 2);
        layer.get_pixels_mut()[0] = ColorRgba::new(100, 150, 200, 255);
        let filter = GrayscaleConversionFilter;
        filter.apply_filter(2, 2, layer.get_pixels_mut()).unwrap();
        let p = layer.get_pixels()[0];
        assert_eq!(p.r, p.g);
        assert_eq!(p.g, p.b);
    }

    #[test]
    fn test_layer_mask_engine() {
        let mut mask_engine = SigmaLayerMaskEngine::new();
        assert!(!mask_engine.alpha_locked);

        let mut mask = SigmaLayerMask::new(2, 2, 255);
        mask.set_value(0, 0, 128); // 50% opacity
        mask_engine.attach_mask(mask);

        let mut pixels = alloc::vec![ColorRgba::new(255, 0, 0, 200); 4];
        mask_engine.apply_mask_to_layer(2, 2, &mut pixels);

        // Pixel (0,0) alpha scaled down by ~50%
        assert_eq!(pixels[0].a, 100);
        // Pixel (1,0) alpha unchanged because mask is 255
        assert_eq!(pixels[1].a, 200);
    }

    #[test]
    fn test_brush_engine_and_spline_interpolation() {
        let brush = SovereignBrushEngine::new(BrushType::SoftRound, 10.0, ColorRgba::new(0, 0, 255, 255));
        let p0 = BrushPoint { x: 0.0, y: 0.0, pressure: 0.5 };
        let p1 = BrushPoint { x: 10.0, y: 10.0, pressure: 0.8 };
        let p2 = BrushPoint { x: 20.0, y: 10.0, pressure: 0.8 };
        let p3 = BrushPoint { x: 30.0, y: 0.0, pressure: 0.5 };

        let mid = SovereignBrushEngine::interpolate_catmull_rom(p0, p1, p2, p3, 0.5);
        assert!(mid.x > 10.0 && mid.x < 20.0);

        let mut pixels = alloc::vec![ColorRgba::new(0, 0, 0, 0); 400]; // 20x20 canvas
        brush.paint_dab(p1, 20, 20, &mut pixels);

        let center_idx = (10 * 20 + 10) as usize;
        assert!(pixels[center_idx].b > 0);
    }

    #[test]
    fn test_selection_engine() {
        let mut sel = SigmaSelectionEngine::new(10, 10);
        assert!(!sel.is_selected(5, 5));

        sel.select_rectangle(2, 2, 8, 8);
        assert!(sel.is_selected(5, 5));
        assert!(!sel.is_selected(0, 0));

        sel.clear_selection();
        assert!(!sel.is_selected(5, 5));
    }

    #[test]
    fn test_vector_path_engine() {
        let mut path = SigmaVectorPathEngine::new();
        path.add_point(VectorControlPoint {
            x: 2.0, y: 2.0,
            handle_in_x: 2.0, handle_in_y: 2.0,
            handle_out_x: 5.0, handle_out_y: 2.0,
        });
        path.add_point(VectorControlPoint {
            x: 8.0, y: 8.0,
            handle_in_x: 5.0, handle_in_y: 8.0,
            handle_out_x: 8.0, handle_out_y: 8.0,
        });

        let mut pixels = alloc::vec![ColorRgba::new(255, 255, 255, 0); 100]; // 10x10 canvas
        path.stroke_path_onto_canvas(10, 10, &mut pixels);

        // Ensure rasterization hit pixels along path
        let hit_count = pixels.iter().filter(|p| p.a > 0).count();
        assert!(hit_count > 0);
    }

    #[test]
    fn test_palette_manager() {
        let mut palette = SigmaPaletteManager::new("Tango");
        let gpl_data = "GIMP Palette\nName: Tango\n# Comment\n252 233 79 Butter\n138 226 52 Chameleon\n";

        let count = palette.load_gpl_palette(gpl_data);
        assert_eq!(count, 2);
        assert_eq!(palette.colors[0], ColorRgba::new(252, 233, 79, 255));

        let comp = SigmaPaletteManager::compute_complementary_color(ColorRgba::new(200, 100, 50, 255));
        assert_eq!(comp, ColorRgba::new(55, 155, 205, 255));
    }

    #[test]
    fn test_image_exporter() {
        let pixels = alloc::vec![ColorRgba::new(255, 0, 0, 255); 4]; // 2x2 red canvas
        let ppm = SigmaImageExporter::export_ppm(2, 2, &pixels).unwrap();
        assert!(ppm.starts_with("P3\n2 2\n255\n"));

        let qoi = SigmaImageExporter::export_qoi_bytes(2, 2, &pixels).unwrap();
        assert_eq!(&qoi[0..4], b"qoif");
    }
}
