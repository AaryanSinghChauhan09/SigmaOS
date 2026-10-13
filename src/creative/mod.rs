// SPDX-License-Identifier: MIT
// SigmaOS - Creative Suite Module
// Creative applications framework inspired by GIMP, Krita, Inkscape, Blender, etc.

pub mod photocraft;
pub mod filmcraft;
pub mod vectorcraft;
pub mod pdfcraft;
pub mod designcraft;
pub mod designcraft_missing_components_parity_pr;
pub mod effectcraft;
pub mod lightcraft;
pub mod mediacraft;
pub mod audiocraft;
pub mod vectoranimation;
pub mod model3d;
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
    FontFamily as PdfFontFamily, FontSize, TextAlignment as PdfTextAlignment,
    TextObject, ImageObject, ShapeObject,
    ShapeType, AnnotationType, Annotation,
    FormFieldType, FormField,
    Bookmark, Hyperlink,
    PdfMetadata, SecuritySettings, ExportFormat as PdfExportFormat,
    init_pdfcraft, get_pdfcraft,
};
pub use designcraft::{
    DesignEditor, Canvas, Element, Color,
    ColorModel, FontFamily, FontWeight, TextAlignment,
    ElementType, TextElement, Layer, DesignBlendMode,
    Guide, GuideOrientation, Grid,
    Component, Style, StyleType, ExportFormat as DesignExportFormat, Artboard,
    init_designcraft, get_designcraft,
};
pub use designcraft_missing_components_parity_pr::{
    DesigncraftPrCategory, DesigncraftPrProposalSpec,
    DesigncraftInteractivePrototypingPrEngine,
    DesigncraftAutoLayoutFlexEngine,
    DesigncraftDesignSystemVariablesEngine,
    DesigncraftVectorGeometryEngine,
    DesigncraftMultiplayerCrdtEngine,
    DesigncraftCodeExportEngine,
    SovereignDesigncraftArtcraftParityPrGateway,
};
pub use effectcraft::{
    EffectEngine, Effect as EffectCraftEffect, Parameter,
    BlendMode as EffectBlendMode, EffectCategory, ParameterType,
    Keyframe, InterpolationType, AnimatedParameter,
    EffectInstance, NodeType, CompositingNode, NodeGraph,
    init_effectcraft, get_effectcraft,
};
pub use lightcraft::{
    LightEngine, Light, LightType, Material, Scene,
    RawFormat, ColorSpace, ToneCurve, ToneCurvePoint, CurveType,
    ColorMixer, ColorMixerBand, ColorGradingWheel, ThreeWayColorGrading,
    Mask, MaskType as LightMaskType, Preset, PhotoMetadata, Photo, Album, Rating, Flag,
    init_lightcraft, get_lightcraft,
};
pub use mediacraft::{
    MediaPlayer, MediaItem, MediaLibrary, Playlist,
    MediaType, AudioCodec as MediaAudioCodec, VideoCodec as MediaVideoCodec, ContainerFormat,
    PlaybackState, RepeatMode, ShuffleMode,
    AudioTrack, SubtitleTrack, VideoTrack as MediaVideoTrack,
    EqualizerBand, EqualizerPreset, AudioSettings, VideoSettings,
    StreamProtocol, StreamSource,
    init_mediacraft, get_mediacraft,
};
pub use audiocraft::{
    AudioEditor, AudioClip, AudioTrack as AudioCraftTrack, AudioEffect,
    SampleFormat, SampleRate, ChannelConfig, TrackType,
    AudioEffectType, AutomationPoint, AutomationInterpolation, AutomationLane,
    AudioExportFormat,
    init_audiocraft, get_audiocraft,
};
pub use vectoranimation::{
    AnimationProject as VectorAnimationProject, AnimationLayer, AnimationLayerType,
    Keyframe, InterpolationMode, Bone, Rig, Timeline, OnionSkin,
    AnimationExportFormat,
    init_vectoranimation, get_vectoranimation,
};
pub use model3d::{
    Scene as Model3DScene, Object3D, Mesh, Vertex,
    MeshType, ObjectType, SculptBrushType, SculptBrush,
    Material, Light, LightType, Camera,
    ExportFormat as Model3DExportFormat,
    init_model3d, get_model3d,
};
pub use artcraft::{
    ArtSuite, Project, Workspace,
    init_artcraft, get_artcraft,
};
