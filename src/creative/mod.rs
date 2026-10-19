// SPDX-License-Identifier: MIT
// SigmaOS - Creative Suite Module
// Creative applications framework inspired by GIMP, Krita, Inkscape, Blender, etc.

pub mod anicraft;
pub mod anicraft_artcraft_parity_pr;
pub mod anicraft_missing_components_parity_pr;
pub mod artcraft;
pub mod audiocraft;
pub mod cadcraft;
pub mod cadcraft_artcraft_parity_pr;
pub mod cadcraft_missing_components_parity_pr;
pub mod craftlibs_missing_components_parity_pr;
pub mod deckcraft;
pub mod deckcraft_artcraft_parity_pr;
pub mod deckcraft_missing_components_parity_pr;
pub mod designcraft;
pub mod designcraft_artcraft_parity_pr;
pub mod designcraft_missing_components_parity_pr;
pub mod effectcraft;
pub mod effectcraft_artcraft_parity_pr;
pub mod effectcraft_missing_components_parity_pr;
pub mod figcraft;
pub mod figcraft_missing_components_parity_pr;
pub mod filmcraft;
pub mod filmcraft_artcraft_parity_pr;
pub mod filmcraft_missing_components_parity_pr;
pub mod fontcraft;
pub mod fontcraft_artcraft_parity_pr;
pub mod fontcraft_missing_components_parity_pr;
pub mod gridcraft;
pub mod gridcraft_artcraft_parity_pr;
pub mod gridcraft_missing_components_parity_pr;
pub mod lightcraft;
pub mod mediacraft;
pub mod model3d;
pub mod pdfcraft;
pub mod pdfcraft_artcraft_parity_pr;
pub mod pdfcraft_missing_components_parity_pr;
pub mod photocraft;
pub mod photocraft_missing_components_parity_pr;
pub mod soundcraft;
pub mod soundcraft_artcraft_parity_pr;
pub mod soundcraft_missing_components_parity_pr;
pub mod sovereign_artcraft_craftlibs_pr_engine;
pub mod threedcraft_artcraft_parity_pr;
pub mod threedcraft_missing_components_parity_pr;
pub mod vectoranimation;
pub mod vectorcraft;
pub mod voicecraft;
pub mod voicecraft_artcraft_parity_pr;
pub mod voicecraft_missing_components_parity_pr;
pub mod wordcraft;
pub mod wordcraft_artcraft_parity_pr;
pub mod wordcraft_missing_components_parity_pr;

pub use anicraft_artcraft_parity_pr::*;
pub use cadcraft_artcraft_parity_pr::*;
pub use cadcraft_missing_components_parity_pr::*;
pub use craftlibs_missing_components_parity_pr::*;

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
pub use deckcraft_artcraft_parity_pr::{
    DeckcraftAiSlideGeneratorLayoutAutoBalancePrEngine, DeckcraftArtcraftParityPrSuite,
    DeckcraftArtcraftPrProposalManifest, DeckcraftCrdtMultiUserCoAuthoringPrEngine,
    DeckcraftLiveAudienceQnaPollingPrEngine, DeckcraftLiveDataDrivenChartsPrEngine,
    DeckcraftMasterTemplateThemeCascadePrEngine, DeckcraftMultiFormat4kVideoRendererPrEngine,
    DeckcraftPresenterZenithDualScreenPrEngine, DeckcraftRevealJsHtml5ExporterPrEngine,
    DeckcraftSmartDiagramConnectorFlowPrEngine, DeckcraftVectorMorphingKeyframeAnimationPrEngine,
    SovereignDeckcraftArtcraftParityPrGateway as SovereignDeckcraftArtcraftParityPrGatewayV2,
};
pub use deckcraft_missing_components_parity_pr::{
    DeckcraftAdvancedVectorAnimationTimelinePrEngine,
    DeckcraftAiSlideGeneratorLayoutSmartDesignPrEngine,
    DeckcraftInteractivePollAudienceQnaPrEngine, DeckcraftMultiFormatBatchExportRenderPrEngine,
    DeckcraftPrProposalManifest, DeckcraftRealtimeCrdtMultiUserCollabPrEngine,
    DeckcraftSmartDiagramDataChartPrEngine, SovereignDeckcraftArtcraftParityPrGateway,
};
pub use designcraft::{
    get_designcraft, init_designcraft, Artboard, Canvas, Color, ColorModel, Component,
    DesignBlendMode, DesignEditor, Element, ElementType, ExportFormat as DesignExportFormat,
    FontFamily, FontWeight, Grid, Guide, GuideOrientation, Layer, Style, StyleType, TextAlignment,
    TextElement,
};
pub use designcraft_missing_components_parity_pr::{
    DesigncraftAutoLayoutFlexEngine, DesigncraftCodeExportEngine,
    DesigncraftDesignSystemVariablesEngine, DesigncraftInteractivePrototypingPrEngine,
    DesigncraftMultiplayerCrdtEngine, DesigncraftPrCategory, DesigncraftPrProposalSpec,
    DesigncraftVectorGeometryEngine, SovereignDesigncraftArtcraftParityPrGateway,
};
pub use effectcraft::{
    fnv1a_effectcraft_pr_digest, get_effectcraft, init_effectcraft, AnimatedParameter,
    BlendMode as EffectBlendMode, CompositingNode, Effect as EffectCraftEffect, EffectCategory,
    EffectCraftMasterPrDeploymentSuite, EffectCraftMotionTrackingKeyingPrDeployer,
    EffectCraftNodeCompositingPrDeployer, EffectCraftParticleShaderPrDeployer,
    EffectCraftPullRequestSpec, EffectEngine, EffectInstance, InterpolationType, Keyframe,
    NodeGraph, NodeType, Parameter, ParameterType,
};
pub use effectcraft_artcraft_parity_pr::{
    EffectcraftArtcraftParityPrSuite, EffectcraftArtcraftPrProposalManifest,
    EffectcraftCameraTracker3dReconstructionPrEngine, EffectcraftCleanPlateWireRemovalPrEngine,
    EffectcraftColorMatchPytorchGradePrEngine, EffectcraftDeepCompositingExrChannelPrEngine,
    EffectcraftGpuFluidDynamicsSimulationPrEngine, EffectcraftProjectionMapping3dMeshPrEngine,
    EffectcraftStereoscopic3dDepthMapPrEngine, EffectcraftSubsurfaceScatteringSkinShaderPrEngine,
    EffectcraftTimeWarpRetimingOpticalFlowPrEngine,
    EffectcraftVolumetricLightingRaymarchingPrEngine,
    SovereignEffectcraftArtcraftParityPrGateway as SovereignEffectcraftArtcraftParityPrGatewayV2,
    Stereo3dMode, SubsurfaceProfile, SurveyPointTarget, VolumetricLightParams,
};
pub use effectcraft_missing_components_parity_pr::{
    ArtCraftOpenFxPluginPrEngine, AudioReactiveVisualizerPrEngine,
    EffectcraftAcesOcioColorManagementPrEngine, EffectcraftCryptomatteAovExtractorPrEngine,
    EffectcraftExpressionScriptingRiggingPrEngine, EffectcraftMissingComponentsParityPrSuite,
    EffectcraftOpticalFlowMotionVectorPrEngine, EffectcraftPrProposalManifest,
    NeuralMatteSegmentationPrEngine, ParticlePhysicsEffectPrEngine, ShaderFilterGraphPrEngine,
    SovereignEffectcraftArtcraftParityPrGateway as SovereignEffectcraftMissingComponentsParityPrGateway,
    Spatial3DDepthCompositorPrEngine,
};
pub use figcraft_missing_components_parity_pr::*;
pub use filmcraft::{
    get_filmcraft, init_filmcraft, AudioChannel, AudioCodec, AudioMixer, Clip, ColorGrading,
    ColorWheel, Effect, ExportSettings, FrameRate, Resolution, Timeline, Transition,
    TransitionType, VideoCodec, VideoEditor, VideoTrack,
};
pub use filmcraft_artcraft_parity_pr::*;
pub use filmcraft_missing_components_parity_pr::*;
pub use fontcraft::{
    FontMetrics, FontTableTag, GlyphContour, GlyphData, GlyphPoint, SovereignFontEngine,
};
pub use fontcraft_missing_components_parity_pr::{
    FontcraftColorVectorEmojiEnginePrEngine, FontcraftComplexScriptHarfbuzzShapingPrEngine,
    FontcraftMsdfGpuVectorRasterizerPrEngine, FontcraftPrProposalManifest,
    FontcraftVariableFontInterpolationPrEngine, FontcraftWoff2BrotliCompressionPrEngine, MsdfTexel,
    OpenTypeFeatureTag, ShapedGlyph, SovereignFontcraftArtcraftParityPrGateway, VariableFontAxis,
    Woff2Header,
};
pub use gridcraft::{
    get_gridcraft, init_gridcraft, AggregationType, CellFormat, CellValue, Chart, ChartType,
    DataVisualizationEngine, FormulaEngine, GridCell, GridcraftChartVisualizationPrDeployer,
    GridcraftEngine, GridcraftFormulaSpreadsheetPrDeployer, GridcraftMasterPrDeploymentSuite,
    GridcraftPivotTableAnalyticsPrDeployer, GridcraftPrPullRequestSpec, NumberFormat,
    PivotFieldOrientation, PivotTable, PivotTableEngine, Workbook, Worksheet,
};
pub use gridcraft_missing_components_parity_pr::{
    CellEditDelta, ConditionalFormatRule, FormulaValue, GoalSeekResult,
    GridcraftAdvancedFormulaXlookupPrEngine, GridcraftDataValidationConditionalFormattingPrEngine,
    GridcraftFinancialGoalSeekSolverPrEngine, GridcraftMacroScriptingUdfPrEngine,
    GridcraftMultiUserCrdtCollabPrEngine, GridcraftPrProposalManifest,
    GridcraftRealtimeDataStreamSpillArrayPrEngine, MonteCarloResult,
    SovereignGridcraftArtcraftParityPrGateway, TickerData, UserDefinedFunction, ValidationRule,
};
pub use lightcraft::{
    get_lightcraft, init_lightcraft, Album, ColorGradingWheel, ColorMixer, ColorMixerBand,
    ColorSpace, CurveType, Flag, Light, LightEngine, LightType, Mask, MaskType as LightMaskType,
    Material, Photo, PhotoMetadata, Preset, Rating, RawFormat, Scene, ThreeWayColorGrading,
    ToneCurve, ToneCurvePoint,
};
pub use mediacraft::{
    get_mediacraft, init_mediacraft, AudioCodec as MediaAudioCodec, AudioSettings, AudioTrack,
    ContainerFormat, EqualizerBand, EqualizerPreset, MediaItem, MediaLibrary, MediaPlayer,
    MediaType, PlaybackState, Playlist, RepeatMode, ShuffleMode, StreamProtocol, StreamSource,
    SubtitleTrack, VideoCodec as MediaVideoCodec, VideoSettings, VideoTrack as MediaVideoTrack,
};
pub use model3d::{
    get_model3d, init_model3d, Camera, ExportFormat as Model3DExportFormat, Light as Model3DLight,
    LightType as Model3DLightType, Material as Model3DMaterial, Mesh, MeshType, Object3D,
    ObjectType, Scene as Model3DScene, SculptBrush, SculptBrushType, Vertex,
};
pub use pdfcraft::{
    get_pdfcraft, init_pdfcraft, Annotation, AnnotationType, Bookmark,
    ExportFormat as PdfExportFormat, FontFamily as PdfFontFamily, FontSize, FormField,
    FormFieldType, Hyperlink, ImageObject, PdfEditor, PdfElement, PdfMetadata, PdfPage,
    SecuritySettings, ShapeObject, ShapeType, TextAlignment as PdfTextAlignment, TextObject,
};
pub use pdfcraft_artcraft_parity_pr::{
    PdfcraftArchivalValidationPrEngine, PdfcraftArtcraftPrProposalManifest,
    PdfcraftBatchExportRenderPrEngine, PdfcraftColorPreflightCmykPrEngine,
    PdfcraftDigitalSignatureWatermarkPrEngine, PdfcraftInteractiveFormsXfaPrEngine,
    PdfcraftMultiWorkspaceProjectPrEngine, PdfcraftOcrTextRecognitionPrEngine,
    PdfcraftPageImpositionBookletPrEngine, PdfcraftRedactionSanitizationPrEngine,
    PdfcraftVectorArtImportPrEngine, SovereignPdfcraftArtcraftParityPrGateway,
};
pub use pdfcraft_missing_components_parity_pr::{
    PdfcraftMissingComponentsParityPrSuite, PdfcraftPrCategory,
    PdfcraftPrProposalManifest as PdfcraftMissingComponentsPrProposalManifest,
    SovereignPdfcraftMissingComponentsParityPrGateway,
};
pub use photocraft::{
    get_photocraft, init_photocraft, Adjustment, AdjustmentType, Filter, ImageLayer, LayerMask,
    LayerStyle, LayerType, MaskType, PhotoEditor, SelectionTool, SmartFilter,
};
pub use photocraft_missing_components_parity_pr::{
    PhotocraftAiInpaintingPrEngine, PhotocraftAiSubjectMattingSelectionPrEngine,
    PhotocraftBatchActionMacroPrEngine, PhotocraftColorGradingFrequencySeparationPrEngine,
    PhotocraftHdrExposureStackingPrEngine, PhotocraftMissingComponentsParityPrSuite,
    PhotocraftPanoramaStitchingProjectionPrEngine, PhotocraftPerspectiveWarpMeshDeformPrEngine,
    PhotocraftPrProposalManifest, PhotocraftRawProcessingPrEngine,
    PhotocraftSmartObjectLayerStylePrEngine, PhotocraftVectorMaskPenPathPrEngine,
    SovereignPhotocraftArtcraftParityPrGateway,
};
pub use soundcraft::{
    fnv1a_soundcraft_pr_digest, SoundClip, SoundCraftAudioEnginePrDeployer,
    SoundCraftDspSynthesizerPrDeployer, SoundCraftEngine, SoundCraftMasterPrDeploymentSuite,
    SoundCraftPullRequestSpec, SoundCraftSpatialMixingMasteringPrDeployer, SoundTrack,
    Spatial3DPosition, SynthesizerVoice, Waveform,
};
pub use soundcraft_artcraft_parity_pr::*;
pub use soundcraft_missing_components_parity_pr::{
    ClipCrossfadeConfig, DawTrackDelta, FadeCurveShape, GranularGrain, LoudnessMeterResult,
    Position3D as SoundcraftPosition3D, SoundcraftAdvancedMultiTrackEnginePrEngine,
    SoundcraftMasteringLoudnessMeterPrEngine, SoundcraftMidiVst3ClapPluginPrEngine,
    SoundcraftPrProposalManifest, SoundcraftRealtimeCollabCrdtPrEngine,
    SoundcraftSpatial3dAmbisonicMasterPrEngine, SoundcraftSpectralSynthesisSamplerPrEngine,
    SovereignSoundcraftArtcraftParityPrGateway as SovereignSoundcraftMissingParityPrGateway,
};
pub use threedcraft_missing_components_parity_pr::{
    Aabb3D, AnimationChannelType, CsgBooleanOp, CsgMesh, KeyframeSample, PbrMaterialParams, Ray3D,
    SkeletalJoint, ThreeDCraftBvhRayTracerEngine, ThreeDCraftCsgBooleanMeshEngine,
    ThreeDCraftGltfAnimationParser, ThreeDCraftPbrShaderEngine, ThreeDCraftPrGatewayEngine,
    ThreeDCraftPrProposalManifest,
};
pub use vectoranimation::{
    get_vectoranimation, init_vectoranimation, AnimationExportFormat, AnimationLayer,
    AnimationLayerType, AnimationProject as VectorAnimationProject, Bone, InterpolationMode,
    Keyframe as VectorAnimationKeyframe, OnionSkin, Rig, Timeline as VectorAnimationTimeline,
};
pub use vectorcraft::{
    get_vectorcraft, init_vectorcraft, BooleanOperation, FillRule, Path, PathPoint, PenToolMode,
    Shape, ShapeProperties, VectorEditor, VectorLayer,
};
pub use wordcraft::{
    fnv1a_wordcraft_pr_digest, get_wordcraft, init_wordcraft, Alignment as WordCraftAlignment,
    ChangeType as WordCraftChangeType, Comment as WordCraftComment, Document as WordCraftDocument,
    DocumentExportFormat as WordCraftDocumentExportFormat, FontFamily as WordCraftFontFamily,
    Paragraph as WordCraftParagraph, Section as WordCraftSection, Table as WordCraftTable,
    TableCell as WordCraftTableCell, TableRow as WordCraftTableRow,
    TextStyle as WordCraftTextStyle, TrackedChange as WordCraftTrackedChange,
    WordCraftCollaborationRevisionPrDeployer, WordCraftDocumentEnginePrDeployer,
    WordCraftExportConversionPrDeployer, WordCraftMasterPrDeploymentSuite,
    WordCraftPullRequestSpec,
};
pub use wordcraft_artcraft_parity_pr::{
    SovereignWordcraftArtcraftParityPrGateway as SovereignWordcraftArtcraftParityPrGatewayV3,
    WordcraftArtcraftParityPrSuite, WordcraftArtcraftPrProposalManifest,
    WordcraftChapterManifestEntry, WordcraftCitationBibliographyCrossReferencePrEngine,
    WordcraftCitationSource, WordcraftCitationStyle, WordcraftColumnLayoutConfig,
    WordcraftCrdtMultiUserCoAuthoringPrEngine, WordcraftDocumentSecurityWatermarkPrEngine,
    WordcraftDocumentStyle, WordcraftDocumentTextDelta, WordcraftDocumentTheme,
    WordcraftEpubPdfBookPublisherPrEngine, WordcraftFormFieldEntry,
    WordcraftLinguisticAiSpellGrammarPrEngine, WordcraftLinguisticIssue,
    WordcraftMailMergeAutomationPrEngine, WordcraftMasterTemplateThemeCascadePrEngine,
    WordcraftOpenTypeTypographyLayoutPrEngine, WordcraftSelectionCursor,
    WordcraftStyleHierarchyThemeCascadePrEngine, WordcraftTableFormula,
    WordcraftTableFormulaChartPrEngine, WordcraftTocEntry,
};
pub use wordcraft_missing_components_parity_pr::{
    SovereignWordcraftArtcraftParityPrGateway,
    WordcraftDocumentSecurityWatermarkPrEngine as WordcraftDocumentSecurityWatermarkPrEngineLegacy,
    WordcraftEpubPdfBookPublisherPrEngine as WordcraftEpubPdfBookPublisherPrEngineLegacy,
    WordcraftLinguisticAiSpellGrammarPrEngine as WordcraftLinguisticAiSpellGrammarPrEngineLegacy,
    WordcraftMailMergeAutomationPrEngine as WordcraftMailMergeAutomationPrEngineLegacy,
    WordcraftMissingComponentsParityPrSuite, WordcraftPrProposalManifest,
    WordcraftRealtimeCrdtCollabPrEngine as WordcraftRealtimeCrdtCollabPrEngineLegacy,
    WordcraftReferenceCitationBibliographyPrEngine as WordcraftReferenceCitationBibliographyPrEngineLegacy,
    WordcraftStylesTemplatesThemesPrEngine,
    WordcraftTableFormulaChartPrEngine as WordcraftTableFormulaChartPrEngineLegacy,
    WordcraftTypographyLayoutPrEngine,
};
