// SPDX-License-Identifier: MIT
// SigmaOS - Lightcraft (Lighting and Photo Library Framework)
// Inspired by ArtCraft LightCraft, Blender Cycles, Adobe Lightroom
// Provides photo library, RAW development, lighting, and rendering capabilities

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Light type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightType {
    Point,
    Directional,
    Spot,
    Area,
    Ambient,
}

/// Light
pub struct Light {
    pub id: u32,
    pub light_type: LightType,
    pub intensity: f32,
    pub color: (f32, f32, f32), // RGB
    pub position: (f32, f32, f32),
    pub enabled: AtomicBool,
}

/// Material
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Material {
    Lambert,
    Phong,
    Blinn,
    PBR,
    Emissive,
}

/// Scene
pub struct Scene {
    pub id: u32,
    pub name: String,
    pub lights: Vec<Light>,
    pub materials: Vec<Material>,
}

/// RAW format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawFormat {
    CR2,
    NEF,
    ARW,
    DNG,
    RAF,
    ORF,
    PEF,
    SRW,
}

/// Color space
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
    SRGB,
    AdobeRGB,
    ProPhotoRGB,
    DisplayP3,
    Rec2020,
}

/// Tone curve point
#[derive(Debug, Clone, Copy)]
pub struct ToneCurvePoint {
    pub input: f32,
    pub output: f32,
}

/// Curve type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurveType {
    Linear,
    Spline,
    Bezier,
}

/// Tone curve
#[derive(Debug, Clone)]
pub struct ToneCurve {
    pub points: Vec<ToneCurvePoint>,
    pub curve_type: CurveType,
}

/// Color mixer band
#[derive(Debug, Clone, Copy)]
pub struct ColorMixerBand {
    pub hue: f32,
    pub saturation: f32,
    pub luminance: f32,
}

/// Color mixer
#[derive(Debug, Clone)]
pub struct ColorMixer {
    pub shadows: ColorMixerBand,
    pub midtones: ColorMixerBand,
    pub highlights: ColorMixerBand,
}

/// Color grading wheel
#[derive(Debug, Clone, Copy)]
pub struct ColorGradingWheel {
    pub hue: f32,
    pub saturation: f32,
}

/// Three-way color grading
#[derive(Debug, Clone)]
pub struct ThreeWayColorGrading {
    pub shadows: ColorGradingWheel,
    pub midtones: ColorGradingWheel,
    pub highlights: ColorGradingWheel,
}

/// Mask type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskType {
    Brush,
    Gradient,
    Radial,
    ColorRange,
    Luminance,
}

/// Mask
#[derive(Debug, Clone)]
pub struct Mask {
    pub id: u32,
    pub mask_type: MaskType,
    pub enabled: bool,
    pub inverted: bool,
    pub feather: f32,
}

/// Rating
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rating {
    None,
    One,
    Two,
    Three,
    Four,
    Five,
}

/// Flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flag {
    None,
    Red,
    Yellow,
    Green,
    Blue,
    Purple,
}

/// Photo metadata
#[derive(Debug, Clone)]
pub struct PhotoMetadata {
    pub filename: String,
    pub raw_format: RawFormat,
    pub color_space: ColorSpace,
    pub iso: u32,
    pub shutter_speed: String,
    pub aperture: String,
    pub focal_length: String,
    pub date_captured: String,
}

/// Photo
#[derive(Debug, Clone)]
pub struct Photo {
    pub id: u32,
    pub metadata: PhotoMetadata,
    pub rating: Rating,
    pub flag: Flag,
    pub tone_curve: ToneCurve,
    pub color_mixer: ColorMixer,
    pub masks: Vec<Mask>,
    pub processed: bool,
}

/// Album
#[derive(Debug, Clone)]
pub struct Album {
    pub id: u32,
    pub name: String,
    pub photos: Vec<u32>, // photo IDs
    pub smart: bool,
    pub criteria: Option<String>, // for smart albums
}

/// Preset
#[derive(Debug, Clone)]
pub struct Preset {
    pub id: u32,
    pub name: String,
    pub tone_curve: ToneCurve,
    pub color_mixer: ColorMixer,
    pub three_way_grading: ThreeWayColorGrading,
}

/// Light engine
pub struct LightEngine {
    pub scenes: BTreeMap<u32, Scene>,
    pub photos: BTreeMap<u32, Photo>,
    pub albums: BTreeMap<u32, Album>,
    pub presets: BTreeMap<u32, Preset>,
    pub active: AtomicBool,
    pub gpu_pipeline_enabled: AtomicBool,
    pub current_color_space: ColorSpace,
}

impl LightEngine {
    pub fn new() -> Self {
        Self {
            scenes: BTreeMap::new(),
            photos: BTreeMap::new(),
            albums: BTreeMap::new(),
            presets: BTreeMap::new(),
            active: AtomicBool::new(false),
            gpu_pipeline_enabled: AtomicBool::new(true),
            current_color_space: ColorSpace::SRGB,
        }
    }

    /// Initialize light engine
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }

    /// Create scene
    pub fn create_scene(&mut self, name: String) -> u32 {
        let id = self.scenes.len() as u32;
        let scene = Scene {
            id,
            name,
            lights: Vec::new(),
            materials: Vec::new(),
        };
        self.scenes.insert(id, scene);
        id
    }

    /// Add light to scene
    pub fn add_light(
        &mut self,
        scene_id: u32,
        light_type: LightType,
        intensity: f32,
    ) -> Result<u32, &'static str> {
        let scene = self.scenes.get_mut(&scene_id).ok_or("Scene not found")?;
        let light_id = scene.lights.len() as u32;
        let light = Light {
            id: light_id,
            light_type,
            intensity,
            color: (1.0, 1.0, 1.0),
            position: (0.0, 0.0, 0.0),
            enabled: AtomicBool::new(true),
        };
        scene.lights.push(light);
        Ok(light_id)
    }

    /// Import photo
    pub fn import_photo(&mut self, metadata: PhotoMetadata) -> u32 {
        let id = self.photos.len() as u32;
        let photo = Photo {
            id,
            metadata,
            rating: Rating::None,
            flag: Flag::None,
            tone_curve: ToneCurve {
                points: vec![
                    ToneCurvePoint {
                        input: 0.0,
                        output: 0.0,
                    },
                    ToneCurvePoint {
                        input: 1.0,
                        output: 1.0,
                    },
                ],
                curve_type: CurveType::Linear,
            },
            color_mixer: ColorMixer {
                shadows: ColorMixerBand {
                    hue: 0.0,
                    saturation: 0.0,
                    luminance: 0.0,
                },
                midtones: ColorMixerBand {
                    hue: 0.0,
                    saturation: 0.0,
                    luminance: 0.0,
                },
                highlights: ColorMixerBand {
                    hue: 0.0,
                    saturation: 0.0,
                    luminance: 0.0,
                },
            },
            masks: Vec::new(),
            processed: false,
        };
        self.photos.insert(id, photo);
        id
    }

    /// Create album
    pub fn create_album(&mut self, name: String, smart: bool, criteria: Option<String>) -> u32 {
        let id = self.albums.len() as u32;
        let album = Album {
            id,
            name,
            photos: Vec::new(),
            smart,
            criteria,
        };
        self.albums.insert(id, album);
        id
    }

    /// Add photo to album
    pub fn add_photo_to_album(&mut self, album_id: u32, photo_id: u32) -> Result<(), &'static str> {
        let album = self.albums.get_mut(&album_id).ok_or("Album not found")?;
        if !album.photos.contains(&photo_id) {
            album.photos.push(photo_id);
        }
        Ok(())
    }

    /// Rate photo
    pub fn rate_photo(&mut self, photo_id: u32, rating: Rating) -> Result<(), &'static str> {
        let photo = self.photos.get_mut(&photo_id).ok_or("Photo not found")?;
        photo.rating = rating;
        Ok(())
    }

    /// Flag photo
    pub fn flag_photo(&mut self, photo_id: u32, flag: Flag) -> Result<(), &'static str> {
        let photo = self.photos.get_mut(&photo_id).ok_or("Photo not found")?;
        photo.flag = flag;
        Ok(())
    }

    /// Set tone curve
    pub fn set_tone_curve(
        &mut self,
        photo_id: u32,
        points: Vec<ToneCurvePoint>,
        curve_type: CurveType,
    ) -> Result<(), &'static str> {
        let photo = self.photos.get_mut(&photo_id).ok_or("Photo not found")?;
        photo.tone_curve = ToneCurve { points, curve_type };
        Ok(())
    }

    /// Set color mixer
    pub fn set_color_mixer(
        &mut self,
        photo_id: u32,
        shadows: ColorMixerBand,
        midtones: ColorMixerBand,
        highlights: ColorMixerBand,
    ) -> Result<(), &'static str> {
        let photo = self.photos.get_mut(&photo_id).ok_or("Photo not found")?;
        photo.color_mixer = ColorMixer {
            shadows,
            midtones,
            highlights,
        };
        Ok(())
    }

    /// Add mask
    pub fn add_mask(&mut self, photo_id: u32, mask_type: MaskType) -> Result<u32, &'static str> {
        let photo = self.photos.get_mut(&photo_id).ok_or("Photo not found")?;
        let mask_id = photo.masks.len() as u32;
        let mask = Mask {
            id: mask_id,
            mask_type,
            enabled: true,
            inverted: false,
            feather: 0.0,
        };
        photo.masks.push(mask);
        Ok(mask_id)
    }

    /// Create preset
    pub fn create_preset(
        &mut self,
        name: String,
        tone_curve: ToneCurve,
        color_mixer: ColorMixer,
        three_way_grading: ThreeWayColorGrading,
    ) -> u32 {
        let id = self.presets.len() as u32;
        let preset = Preset {
            id,
            name,
            tone_curve,
            color_mixer,
            three_way_grading,
        };
        self.presets.insert(id, preset);
        id
    }

    /// Apply preset to photo
    pub fn apply_preset(&mut self, photo_id: u32, preset_id: u32) -> Result<(), &'static str> {
        let preset = self.presets.get(&preset_id).ok_or("Preset not found")?;
        let photo = self.photos.get_mut(&photo_id).ok_or("Photo not found")?;
        photo.tone_curve = preset.tone_curve.clone();
        photo.color_mixer = preset.color_mixer.clone();
        Ok(())
    }

    /// Set color space
    pub fn set_color_space(&mut self, color_space: ColorSpace) {
        self.current_color_space = color_space;
    }

    /// Toggle GPU pipeline
    pub fn toggle_gpu_pipeline(&mut self) {
        let current = self.gpu_pipeline_enabled.load(Ordering::Relaxed);
        self.gpu_pipeline_enabled.store(!current, Ordering::Relaxed);
    }

    /// Search photos by rating
    pub fn search_by_rating(&self, rating: Rating) -> Vec<u32> {
        self.photos
            .iter()
            .filter(|(_, photo)| photo.rating == rating)
            .map(|(id, _)| *id)
            .collect()
    }

    /// Search photos by flag
    pub fn search_by_flag(&self, flag: Flag) -> Vec<u32> {
        self.photos
            .iter()
            .filter(|(_, photo)| photo.flag == flag)
            .map(|(id, _)| *id)
            .collect()
    }

    /// Disable light
    pub fn disable_light(&mut self, scene_id: u32, light_id: u32) -> Result<(), &'static str> {
        let scene = self.scenes.get_mut(&scene_id).ok_or("Scene not found")?;
        let light = scene
            .lights
            .get_mut(light_id as usize)
            .ok_or("Light not found")?;
        light.enabled.store(false, Ordering::Relaxed);
        Ok(())
    }

    /// Get scene
    pub fn get_scene(&self, id: u32) -> Option<&Scene> {
        self.scenes.get(&id)
    }

    /// Get photo
    pub fn get_photo(&self, id: u32) -> Option<&Photo> {
        self.photos.get(&id)
    }

    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for LightEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Global lightcraft
static mut LIGHTCRAFT: Option<LightEngine> = None;

/// Initialize lightcraft
pub fn init_lightcraft() {
    unsafe {
        LIGHTCRAFT = Some(LightEngine::new());
    }
}

/// Get global lightcraft
pub fn get_lightcraft() -> Option<&'static mut LightEngine> {
    unsafe { LIGHTCRAFT.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lightcraft_init() {
        let mut engine = LightEngine::new();
        engine.init();
        assert!(engine.is_active());
    }

    #[test]
    fn test_create_scene() {
        let mut engine = LightEngine::new();
        engine.init();
        let id = engine.create_scene("Scene1".to_string());
        assert!(engine.get_scene(id).is_some());
    }

    #[test]
    fn test_add_light() {
        let mut engine = LightEngine::new();
        engine.init();
        let scene_id = engine.create_scene("Scene1".to_string());
        engine.add_light(scene_id, LightType::Point, 1.0).unwrap();
        assert_eq!(engine.get_scene(scene_id).unwrap().lights.len(), 1);
    }

    #[test]
    fn test_import_photo() {
        let mut engine = LightEngine::new();
        engine.init();
        let metadata = PhotoMetadata {
            filename: "test.cr2".to_string(),
            raw_format: RawFormat::CR2,
            color_space: ColorSpace::SRGB,
            iso: 100,
            shutter_speed: "1/60".to_string(),
            aperture: "f/2.8".to_string(),
            focal_length: "50mm".to_string(),
            date_captured: "2024-01-01".to_string(),
        };
        let photo_id = engine.import_photo(metadata);
        assert!(engine.get_photo(photo_id).is_some());
    }

    #[test]
    fn test_albums() {
        let mut engine = LightEngine::new();
        engine.init();
        let album_id = engine.create_album("Vacation".to_string(), false, None);
        let metadata = PhotoMetadata {
            filename: "test.cr2".to_string(),
            raw_format: RawFormat::CR2,
            color_space: ColorSpace::SRGB,
            iso: 100,
            shutter_speed: "1/60".to_string(),
            aperture: "f/2.8".to_string(),
            focal_length: "50mm".to_string(),
            date_captured: "2024-01-01".to_string(),
        };
        let photo_id = engine.import_photo(metadata);
        engine.add_photo_to_album(album_id, photo_id).unwrap();
        assert_eq!(engine.albums.get(&album_id).unwrap().photos.len(), 1);
    }

    #[test]
    fn test_rating_and_flagging() {
        let mut engine = LightEngine::new();
        engine.init();
        let metadata = PhotoMetadata {
            filename: "test.cr2".to_string(),
            raw_format: RawFormat::CR2,
            color_space: ColorSpace::SRGB,
            iso: 100,
            shutter_speed: "1/60".to_string(),
            aperture: "f/2.8".to_string(),
            focal_length: "50mm".to_string(),
            date_captured: "2024-01-01".to_string(),
        };
        let photo_id = engine.import_photo(metadata);
        engine.rate_photo(photo_id, Rating::Five).unwrap();
        assert_eq!(engine.get_photo(photo_id).unwrap().rating, Rating::Five);
        engine.flag_photo(photo_id, Flag::Red).unwrap();
        assert_eq!(engine.get_photo(photo_id).unwrap().flag, Flag::Red);
    }

    #[test]
    fn test_tone_curve() {
        let mut engine = LightEngine::new();
        engine.init();
        let metadata = PhotoMetadata {
            filename: "test.cr2".to_string(),
            raw_format: RawFormat::CR2,
            color_space: ColorSpace::SRGB,
            iso: 100,
            shutter_speed: "1/60".to_string(),
            aperture: "f/2.8".to_string(),
            focal_length: "50mm".to_string(),
            date_captured: "2024-01-01".to_string(),
        };
        let photo_id = engine.import_photo(metadata);
        let points = vec![
            ToneCurvePoint {
                input: 0.0,
                output: 0.0,
            },
            ToneCurvePoint {
                input: 0.5,
                output: 0.6,
            },
            ToneCurvePoint {
                input: 1.0,
                output: 1.0,
            },
        ];
        engine
            .set_tone_curve(photo_id, points, CurveType::Spline)
            .unwrap();
        assert_eq!(
            engine.get_photo(photo_id).unwrap().tone_curve.points.len(),
            3
        );
    }

    #[test]
    fn test_presets() {
        let mut engine = LightEngine::new();
        engine.init();
        let tone_curve = ToneCurve {
            points: vec![
                ToneCurvePoint {
                    input: 0.0,
                    output: 0.0,
                },
                ToneCurvePoint {
                    input: 1.0,
                    output: 1.0,
                },
            ],
            curve_type: CurveType::Linear,
        };
        let color_mixer = ColorMixer {
            shadows: ColorMixerBand {
                hue: 0.0,
                saturation: 0.0,
                luminance: 0.0,
            },
            midtones: ColorMixerBand {
                hue: 0.0,
                saturation: 0.0,
                luminance: 0.0,
            },
            highlights: ColorMixerBand {
                hue: 0.0,
                saturation: 0.0,
                luminance: 0.0,
            },
        };
        let three_way = ThreeWayColorGrading {
            shadows: ColorGradingWheel {
                hue: 0.0,
                saturation: 0.0,
            },
            midtones: ColorGradingWheel {
                hue: 0.0,
                saturation: 0.0,
            },
            highlights: ColorGradingWheel {
                hue: 0.0,
                saturation: 0.0,
            },
        };
        let preset_id =
            engine.create_preset("Cinematic".to_string(), tone_curve, color_mixer, three_way);
        assert!(engine.presets.contains_key(&preset_id));
    }

    #[test]
    fn test_color_space() {
        let mut engine = LightEngine::new();
        engine.init();
        engine.set_color_space(ColorSpace::AdobeRGB);
        assert_eq!(engine.current_color_space, ColorSpace::AdobeRGB);
    }

    #[test]
    fn test_gpu_pipeline() {
        let mut engine = LightEngine::new();
        engine.init();
        assert!(engine.gpu_pipeline_enabled.load(Ordering::Relaxed));
        engine.toggle_gpu_pipeline();
        assert!(!engine.gpu_pipeline_enabled.load(Ordering::Relaxed));
    }

    #[test]
    fn test_search() {
        let mut engine = LightEngine::new();
        engine.init();
        let metadata = PhotoMetadata {
            filename: "test.cr2".to_string(),
            raw_format: RawFormat::CR2,
            color_space: ColorSpace::SRGB,
            iso: 100,
            shutter_speed: "1/60".to_string(),
            aperture: "f/2.8".to_string(),
            focal_length: "50mm".to_string(),
            date_captured: "2024-01-01".to_string(),
        };
        let photo_id = engine.import_photo(metadata);
        engine.rate_photo(photo_id, Rating::Five).unwrap();
        let results = engine.search_by_rating(Rating::Five);
        assert_eq!(results.len(), 1);
        assert!(results.contains(&photo_id));
    }
}
