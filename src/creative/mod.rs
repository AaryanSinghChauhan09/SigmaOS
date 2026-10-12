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
    PhotoEditor, ImageLayer, Filter, Adjustment,
    init_photocraft, get_photocraft,
};
pub use filmcraft::{
    VideoEditor, VideoTrack, Timeline, Effect as FilmEffect,
    init_filmcraft, get_filmcraft,
};
pub use vectorcraft::{
    VectorEditor, Shape, Path, Layer as VectorLayer,
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
    EffectEngine, Effect, Parameter,
    init_effectcraft, get_effectcraft,
};
pub use lightcraft::{
    LightEngine, Light, LightType, Material, Scene,
    init_lightcraft, get_lightcraft,
};
pub use artcraft::{
    ArtSuite, Project, Workspace,
    init_artcraft, get_artcraft,
};
