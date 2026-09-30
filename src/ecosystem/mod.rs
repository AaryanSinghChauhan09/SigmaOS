// SigmaOS Ecosystem Module
pub mod integration;
pub mod technology;

pub use integration::{
    ArchTier, ArchitecturePort, EcosystemCertification, EcosystemManager, EcosystemPlatform,
    EnterprisePartner,
};
pub use technology::{
    KimiCodeAssistant, CodeSnippet, NDArray, numpy_mean, numpy_std_dev,
    CvImage, WinUiControl, WinUiState, WinUiPanel,
    SigmaGrpcEngine, GrpcServiceStub, MachMessageHeader, MachPort, MachZone,
    SigmaFreeTypeFont, UiRect, NavigationDirection, SpatialNavigationEngine,
};
