// SPDX-License-Identifier: MIT
// SigmaOS - Creative Suite Module
// Creative applications framework inspired by GIMP, Krita, Inkscape, Blender, etc.

pub mod photocraft;
pub mod filmcraft;
pub mod vectorcraft;
pub mod pdfcraft;
pub mod designcraft;
pub mod effectcraft;
pub mod lightcraft;
pub mod artcraft;

pub use photocraft::{
    PhotoEditor, ImageLayer, Filter, Adjustment, LayerType, LayerMask, AdjustmentType,
    LayerStyle, SelectionTool, SmartFilter, MaskType,
    init_photocraft, get_photocraft,
};
pub use filmcraft::{
    VideoEditor, VideoTrack, Timeline, Effect,
    VideoCodec, AudioCodec, Resolution, FrameRate,
    Clip, Transition, TransitionType,
    ColorGrading, ColorWheel,
    AudioMixer, AudioChannel,
    ExportSettings,
    init_filmcraft, get_filmcraft,
};
pub use vectorcraft::{
    VectorEditor, Shape, Path, VectorLayer, PathPoint, FillRule, BooleanOperation,
    PenToolMode, ShapeProperties,
    init_vectorcraft, get_vectorcraft,
};
pub use pdfcraft::{
    PdfEditor, PdfPage, PdfElement,
    init_pdfcraft, get_pdfcraft,
};
pub use designcraft::{
    DesignEditor, Canvas, Element, Color,
    init_designcraft, get_designcraft,
};
pub use effectcraft::{
    EffectEngine, Effect as EffectCraftEffect, Parameter,
    init_effectcraft, get_effectcraft,
};
pub use lightcraft::{
    LightEngine, Light, LightType, Material, Scene,
    RawFormat, ColorSpace, ToneCurve, ToneCurvePoint, CurveType,
    ColorMixer, ColorMixerBand, ColorGradingWheel, ThreeWayColorGrading,
    Mask, MaskType, Preset, PhotoMetadata, Photo, Album, Rating, Flag,
    init_lightcraft, get_lightcraft,
};
pub use artcraft::{
    ArtSuite, Project, Workspace,
    init_artcraft, get_artcraft,
};
