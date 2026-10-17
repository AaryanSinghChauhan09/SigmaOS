// SPDX-License-Identifier: MIT
// SigmaOS - Creative Suite Module
// Creative applications framework inspired by GIMP, Krita, Inkscape, Blender, etc.

pub mod artcraft;
pub mod audiocraft;
pub mod cadcraft;
pub mod deckcraft;
pub mod designcraft;
pub mod designcraft_missing_components_parity_pr;
pub mod figcraft;
pub mod figcraft_missing_components_parity_pr;
pub mod effectcraft;
pub mod effectcraft_missing_components_parity_pr;
pub mod filmcraft;
pub mod lightcraft;
pub mod mediacraft;
pub mod model3d;
pub mod pdfcraft;
pub mod pdfcraft_artcraft_parity_pr;
pub mod photocraft;
pub mod photocraft_missing_components_parity_pr;
pub mod soundcraft;
pub mod vectoranimation;
pub mod vectorcraft;
pub mod wordcraft;

pub use photocraft::{
    PhotoEditor, ImageLayer, Filter, Adjustment, LayerType, LayerMask, AdjustmentType,
    LayerStyle, SelectionTool, SmartFilter, MaskType,
    init_photocraft, get_photocraft,
};
pub use photocraft_missing_components_parity_pr::{
    PhotocraftPrProposalManifest, PhotocraftRawProcessingPrEngine,
    PhotocraftAiInpaintingPrEngine, PhotocraftColorGradingFrequencySeparationPrEngine,
    PhotocraftHdrExposureStackingPrEngine, PhotocraftVectorMaskPenPathPrEngine,
    PhotocraftBatchActionMacroPrEngine, SovereignPhotocraftArtcraftParityPrGateway,
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
pub use pdfcraft_artcraft_parity_pr::{
    PdfcraftArtcraftPrProposalManifest, PdfcraftVectorArtImportPrEngine,
    PdfcraftMultiWorkspaceProjectPrEngine, PdfcraftBatchExportRenderPrEngine,
    PdfcraftDigitalSignatureWatermarkPrEngine, PdfcraftOcrTextRecognitionPrEngine,
    PdfcraftInteractiveFormsXfaPrEngine, PdfcraftColorPreflightCmykPrEngine,
    PdfcraftRedactionSanitizationPrEngine, PdfcraftPageImpositionBookletPrEngine,
    PdfcraftArchivalValidationPrEngine, SovereignPdfcraftArtcraftParityPrGateway,
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
    EffectCraftPullRequestSpec, EffectCraftNodeCompositingPrDeployer,
    EffectCraftParticleShaderPrDeployer, EffectCraftMotionTrackingKeyingPrDeployer,
    EffectCraftMasterPrDeploymentSuite, fnv1a_effectcraft_pr_digest,
    init_effectcraft, get_effectcraft,
};
pub use effectcraft_missing_components_parity_pr::{
    EffectcraftPrProposalManifest, ShaderFilterGraphPrEngine,
    ParticlePhysicsEffectPrEngine, NeuralMatteSegmentationPrEngine,
    AudioReactiveVisualizerPrEngine, Spatial3DDepthCompositorPrEngine,
    ArtCraftOpenFxPluginPrEngine, EffectcraftOpticalFlowMotionVectorPrEngine,
    EffectcraftAcesOcioColorManagementPrEngine, EffectcraftCryptomatteAovExtractorPrEngine,
    EffectcraftExpressionScriptingRiggingPrEngine, VolumetricLightingAtmosphericRaymarchPrEngine,
    ProceduralVfxFractalFluidSimulationPrEngine, TemporalNoiseReductionMotionDegrainPrEngine,
    StereoscopicVr360SpatialVideoPrEngine, SovereignEffectcraftArtcraftParityPrGateway,
    EffectcraftMissingComponentsParityPrSuite,
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
    get_audiocraft, init_audiocraft, AudioClip, AudioEditor, AudioEffect, AudioEffectType,
    AudioExportFormat, AudioTrack as AudioCraftTrack, AutomationInterpolation, AutomationLane,
    AutomationPoint, ChannelConfig, SampleFormat, SampleRate, TrackType,
};
pub use deckcraft::{
    fnv1a_deckcraft_pr_digest, get_deckcraft, init_deckcraft,
    DeckCraftAnimationTransitionPrDeployer, DeckCraftEngine, DeckCraftMasterPrDeploymentSuite,
    DeckCraftPresenterZenithPrDeployer, DeckCraftPullRequestSpec,
    DeckCraftSlideLayoutMasterPrDeployer, DeckExportFormat, PresentationDeck, PresenterState,
    Slide, SlideElement, SlideExporterEngine, SlideLayoutType,
};
pub use model3d::{
    get_model3d, init_model3d, Camera, ExportFormat as Model3DExportFormat, Light as Model3DLight, LightType as Model3DLightType,
    Material as Model3DMaterial, Mesh, MeshType, Object3D, ObjectType, Scene as Model3DScene, SculptBrush,
    SculptBrushType, Vertex,
};
pub use soundcraft::{
    fnv1a_soundcraft_pr_digest, SoundClip, SoundCraftAudioEnginePrDeployer,
    SoundCraftDspSynthesizerPrDeployer, SoundCraftEngine, SoundCraftMasterPrDeploymentSuite,
    SoundCraftPullRequestSpec, SoundCraftSpatialMixingMasteringPrDeployer, SoundTrack,
    Spatial3DPosition, SynthesizerVoice, Waveform,
};
pub use vectoranimation::{
    get_vectoranimation, init_vectoranimation, AnimationExportFormat, AnimationLayer,
    AnimationLayerType, AnimationProject as VectorAnimationProject, Bone, InterpolationMode,
    Keyframe as VectorKeyframe, OnionSkin, Rig, Timeline as VectorTimeline,
};
pub use wordcraft::{
    fnv1a_wordcraft_pr_digest, get_wordcraft, init_wordcraft, Alignment as WordCraftAlignment,
    ChangeType as WordCraftChangeType, Comment as WordCraftComment,
    Document as WordCraftDocument, DocumentExportFormat as WordCraftDocumentExportFormat,
    FontFamily as WordCraftFontFamily, Paragraph as WordCraftParagraph,
    Section as WordCraftSection, Table as WordCraftTable, TableCell as WordCraftTableCell,
    TableRow as WordCraftTableRow, TextStyle as WordCraftTextStyle,
    TrackedChange as WordCraftTrackedChange,
    WordCraftCollaborationRevisionPrDeployer, WordCraftDocumentEnginePrDeployer,
    WordCraftExportConversionPrDeployer, WordCraftMasterPrDeploymentSuite,
    WordCraftPullRequestSpec,
};
pub use wordcraft_missing_components_parity_pr::{
    SovereignWordcraftArtcraftParityPrGateway, WordcraftLinguisticAiSpellGrammarPrEngine,
    WordcraftMailMergeAutomationPrEngine, WordcraftPrProposalManifest,
    WordcraftRealtimeCrdtCollabPrEngine, WordcraftReferenceCitationBibliographyPrEngine,
    WordcraftStylesTemplatesThemesPrEngine, WordcraftTypographyLayoutPrEngine,
};
