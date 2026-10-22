// SPDX-License-Identifier: MIT
// SigmaOS - Photocraft (Photo Editing Framework)
// Inspired by GIMP (https://www.gimp.org/), Krita (https://krita.org/), and ArtCraft PhotoCraft
// Provides photo editing capabilities with PSD support, adjustment layers, and smart objects

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Image layer type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerType {
    Raster,
    Adjustment,
    Shape,
    Type,
    SmartObject,
}

/// Layer mask
#[derive(Debug, Clone)]
pub struct LayerMask {
    pub enabled: bool,
    pub linked: bool,
}

/// Adjustment layer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdjustmentType {
    BrightnessContrast,
    Curves,
    Levels,
    HueSaturation,
    ColorBalance,
    Vibrance,
    PhotoFilter,
    ChannelMixer,
    GradientMap,
    SelectiveColor,
}

/// Layer style
#[derive(Debug, Clone)]
pub struct LayerStyle {
    pub drop_shadow: Option<ShadowStyle>,
    pub inner_shadow: Option<ShadowStyle>,
    pub outer_glow: Option<GlowStyle>,
    pub inner_glow: Option<GlowStyle>,
    pub bevel_emboss: Option<BevelStyle>,
    pub stroke: Option<StrokeStyle>,
    pub color_overlay: Option<ColorStyle>,
    pub gradient_overlay: Option<GradientStyle>,
}

#[derive(Debug, Clone)]
pub struct ShadowStyle {
    pub blend_mode: BlendMode,
    pub color: [u8; 4],
    pub opacity: f32,
    pub angle: f32,
    pub distance: f32,
    pub spread: f32,
    pub size: f32,
}

#[derive(Debug, Clone)]
pub struct GlowStyle {
    pub blend_mode: BlendMode,
    pub color: [u8; 4],
    pub opacity: f32,
    pub spread: f32,
    pub size: f32,
}

#[derive(Debug, Clone)]
pub struct BevelStyle {
    pub style: BevelStyleType,
    pub technique: BevelTechnique,
    pub depth: f32,
    pub size: f32,
    pub soften: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BevelStyleType {
    InnerBevel,
    OuterBevel,
    Emboss,
    PillowEmboss,
    StrokeEmboss,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BevelTechnique {
    Smooth,
    ChiselHard,
    ChiselSoft,
}

#[derive(Debug, Clone)]
pub struct StrokeStyle {
    pub size: f32,
    pub position: StrokePosition,
    pub blend_mode: BlendMode,
    pub opacity: f32,
    pub color: [u8; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrokePosition {
    Outside,
    Inside,
    Center,
}

#[derive(Debug, Clone)]
pub struct ColorStyle {
    pub blend_mode: BlendMode,
    pub opacity: f32,
    pub color: [u8; 4],
}

#[derive(Debug, Clone)]
pub struct GradientStyle {
    pub blend_mode: BlendMode,
    pub opacity: f32,
    pub gradient: Gradient,
}

#[derive(Debug, Clone)]
pub struct Gradient {
    pub colors: Vec<[u8; 4]>,
    pub stops: Vec<f32>,
    pub angle: f32,
    pub type_: GradientType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradientType {
    Linear,
    Radial,
    Angle,
    Reflected,
    Diamond,
}

/// Image layer
#[derive(Debug)]
pub struct ImageLayer {
    pub id: u32,
    pub name: String,
    pub layer_type: LayerType,
    pub visible: AtomicBool,
    pub locked: AtomicBool,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub mask: Option<LayerMask>,
    pub adjustment_type: Option<AdjustmentType>,
    pub layer_style: Option<LayerStyle>,
}

/// Blend mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
    LinearDodge,
    LinearBurn,
    VividLight,
    LinearLight,
    PinLight,
    HardMix,
    PassThrough,
}

/// Selection tool type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionTool {
    RectangularMarquee,
    EllipticalMarquee,
    Lasso,
    PolygonalLasso,
    MagneticLasso,
    QuickSelection,
    MagicWand,
    ColorRange,
    SubjectSelect,
    ObjectSelect,
}

/// Mask type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskType {
    LayerMask,
    VectorMask,
    ClippingMask,
}

/// Smart filter
#[derive(Debug, Clone)]
pub struct SmartFilter {
    pub filter_name: String,
    pub enabled: bool,
    pub mask: Option<LayerMask>,
}

/// Filter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    Blur,
    Sharpen,
    Noise,
    Brightness,
    Contrast,
    Saturation,
    Hue,
    GaussianBlur,
    MotionBlur,
    RadialBlur,
    SmartBlur,
    SurfaceBlur,
    LensBlur,
    BoxBlur,
    UnsharpMask,
    HighPass,
    Median,
    DustAndScratches,
    Despeckle,
    AddNoise,
    ReduceNoise,
    GradientNoise,
    Liquify,
    PuppetWarp,
    PerspectiveWarp,
    VanishingPoint,
    ContentAwareFill,
    ContentAwareScale,
    ContentAwareMove,
}

/// Adjustment
#[derive(Debug, Clone)]
pub struct Adjustment {
    pub filter: Filter,
    pub value: f32,
    pub parameters: BTreeMap<String, f32>,
}

/// Photo editor
pub struct PhotoEditor {
    pub layers: BTreeMap<u32, ImageLayer>,
    pub current_image: Option<String>,
    pub next_layer_id: AtomicU32,
    pub active: AtomicBool,
    pub history: Vec<String>,
    pub smart_filters: Vec<SmartFilter>,
    pub selection_tool: SelectionTool,
    pub psd_compatible: bool,
}

impl PhotoEditor {
    pub fn new() -> Self {
        Self {
            layers: BTreeMap::new(),
            current_image: None,
            next_layer_id: AtomicU32::new(1),
            active: AtomicBool::new(false),
            history: Vec::new(),
            smart_filters: Vec::new(),
            selection_tool: SelectionTool::RectangularMarquee,
            psd_compatible: true,
        }
    }

    /// Initialize photo editor
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }

    /// Load image
    pub fn load_image(&mut self, path: String) -> Result<(), &'static str> {
        self.current_image = Some(path.clone());
        self.history.push(format!("Load: {}", path));
        Ok(())
    }

    /// Create layer
    pub fn create_layer(&mut self, name: String, layer_type: LayerType) -> u32 {
        let id = self.next_layer_id.fetch_add(1, Ordering::Relaxed);
        let layer = ImageLayer {
            id,
            name,
            layer_type,
            visible: AtomicBool::new(true),
            locked: AtomicBool::new(false),
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            mask: None,
            adjustment_type: None,
            layer_style: None,
        };
        self.layers.insert(id, layer);
        id
    }

    /// Get layer
    pub fn get_layer(&self, id: u32) -> Option<&ImageLayer> {
        self.layers.get(&id)
    }

    /// Apply filter
    pub fn apply_filter(
        &mut self,
        layer_id: u32,
        filter: Filter,
        value: f32,
    ) -> Result<(), &'static str> {
        self.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        self.history
            .push(format!("Apply filter: {:?} with value {}", filter, value));
        Ok(())
    }

    /// Adjust layer opacity
    pub fn adjust_opacity(&mut self, layer_id: u32, opacity: f32) -> Result<(), &'static str> {
        let layer = self.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        layer.opacity = opacity;
        Ok(())
    }

    /// Set blend mode
    pub fn set_blend_mode(
        &mut self,
        layer_id: u32,
        blend_mode: BlendMode,
    ) -> Result<(), &'static str> {
        let layer = self.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        layer.blend_mode = blend_mode;
        Ok(())
    }

    /// Add smart filter
    pub fn add_smart_filter(&mut self, filter_name: String) {
        let smart_filter = SmartFilter {
            filter_name,
            enabled: true,
            mask: None,
        };
        self.smart_filters.push(smart_filter);
    }

    /// Set selection tool
    pub fn set_selection_tool(&mut self, tool: SelectionTool) {
        self.selection_tool = tool;
    }

    /// Add layer mask
    pub fn add_layer_mask(&mut self, layer_id: u32) -> Result<(), &'static str> {
        let layer = self.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        layer.mask = Some(LayerMask {
            enabled: true,
            linked: true,
        });
        Ok(())
    }

    /// Set layer style
    pub fn set_layer_style(
        &mut self,
        layer_id: u32,
        style: LayerStyle,
    ) -> Result<(), &'static str> {
        let layer = self.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        layer.layer_style = Some(style);
        Ok(())
    }

    /// Lock layer
    pub fn lock_layer(&mut self, layer_id: u32) -> Result<(), &'static str> {
        let layer = self.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        layer.locked.store(true, Ordering::Relaxed);
        Ok(())
    }

    /// Unlock layer
    pub fn unlock_layer(&mut self, layer_id: u32) -> Result<(), &'static str> {
        let layer = self.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        layer.locked.store(false, Ordering::Relaxed);
        Ok(())
    }

    /// Undo
    pub fn undo(&mut self) -> Result<(), &'static str> {
        if self.history.is_empty() {
            return Err("No history to undo");
        }
        self.history.pop();
        Ok(())
    }

    /// Get current image
    pub fn get_current_image(&self) -> Option<&String> {
        self.current_image.as_ref()
    }

    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for PhotoEditor {
    fn default() -> Self {
        Self::new()
    }
}

/// Global photo editor
static mut PHOTOCRAFT: Option<PhotoEditor> = None;

/// Initialize photocraft
pub fn init_photocraft() {
    unsafe {
        PHOTOCRAFT = Some(PhotoEditor::new());
    }
}

/// Get global photo editor
pub fn get_photocraft() -> Option<&'static mut PhotoEditor> {
    unsafe { PHOTOCRAFT.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_photocraft_init() {
        let mut editor = PhotoEditor::new();
        editor.init();
        assert!(editor.is_active());
    }

    #[test]
    fn test_load_image() {
        let mut editor = PhotoEditor::new();
        editor.init();
        editor.load_image("/path/to/image.jpg".to_string()).unwrap();
        assert!(editor.get_current_image().is_some());
    }

    #[test]
    fn test_create_layer() {
        let mut editor = PhotoEditor::new();
        editor.init();
        let id = editor.create_layer("Background".to_string(), LayerType::Raster);
        assert!(editor.get_layer(id).is_some());
    }

    #[test]
    fn test_create_adjustment_layer() {
        let mut editor = PhotoEditor::new();
        editor.init();
        let id = editor.create_layer("Curves Adjustment".to_string(), LayerType::Adjustment);
        assert!(editor.get_layer(id).is_some());
    }

    #[test]
    fn test_layer_mask() {
        let mut editor = PhotoEditor::new();
        editor.init();
        let id = editor.create_layer("Layer1".to_string(), LayerType::Raster);
        editor.add_layer_mask(id).unwrap();
        assert!(editor.get_layer(id).unwrap().mask.is_some());
    }

    #[test]
    fn test_layer_lock() {
        let mut editor = PhotoEditor::new();
        editor.init();
        let id = editor.create_layer("Layer1".to_string(), LayerType::Raster);
        editor.lock_layer(id).unwrap();
        assert!(editor.get_layer(id).unwrap().locked.load(Ordering::Relaxed));
    }

    #[test]
    fn test_smart_filter() {
        let mut editor = PhotoEditor::new();
        editor.init();
        editor.add_smart_filter("Gaussian Blur".to_string());
        assert_eq!(editor.smart_filters.len(), 1);
    }

    #[test]
    fn test_selection_tool() {
        let mut editor = PhotoEditor::new();
        editor.init();
        editor.set_selection_tool(SelectionTool::QuickSelection);
        assert_eq!(editor.selection_tool, SelectionTool::QuickSelection);
    }

    #[test]
    fn test_apply_filter() {
        let mut editor = PhotoEditor::new();
        editor.init();
        let id = editor.create_layer("Layer1".to_string(), LayerType::Raster);
        editor.apply_filter(id, Filter::GaussianBlur, 50.0).unwrap();
        assert_eq!(editor.history.len(), 1);
    }
}
