//! SigmaOS Parity Distribution (Distro) Subsystem
//! Implements multi-channel release layers, modular installer pipelines,
//! internationalization localization engines, recovery tools, and compliance.

pub mod certification;
pub mod community;
pub mod developer;
pub mod enterprise;
pub mod i18n;
pub mod recovery;
pub mod specialized;

pub use certification::{CertificationSuite, TestResult};
pub use community::{CommunityRepository, RepoSecurityLevel};
pub use developer::{BuildTarget, DevelopmentPipeline};
pub use enterprise::EnterpriseManager;
pub use i18n::{Language, LocalizationEngine};
pub use recovery::{MountMode, RescueManager};
pub use specialized::{SpecializedProfile, VerticalPreset};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i18n_localization() {
        let hindi_engine = LocalizationEngine::new(Language::Hindi);
        assert_eq!(
            hindi_engine.get_translation("welcome"),
            "सिग्मा ओएस में आपका स्वागत है।"
        );
        assert_eq!(
            hindi_engine.get_translation("installer_title"),
            "सिग्मा ओएस स्थापना प्रबंधक"
        );

        let eng_engine = LocalizationEngine::new(Language::English);
        assert_eq!(eng_engine.get_translation("welcome"), "Welcome to SigmaOS.");
    }

    #[test]
    fn test_community_repositories() {
        let repo = CommunityRepository::new(
            "VettedApps",
            "https://p2p.sigmaos.org/vetted",
            RepoSecurityLevel::VettedCore,
        );
        assert_eq!(repo.verify_trust_score(), 100);

        let experimental = CommunityRepository::new(
            "Unstable",
            "https://p2p.sigmaos.org/experimental",
            RepoSecurityLevel::Experimental,
        );
        assert_eq!(experimental.verify_trust_score(), 30);
    }

    #[test]
    fn test_developer_pipeline() {
        let native_pipeline = DevelopmentPipeline::new(BuildTarget::Native);
        assert_eq!(native_pipeline.get_rustc_flags(), "-C opt-level=3");

        let arm_pipeline = DevelopmentPipeline::new(BuildTarget::CrossARM64);
        assert_eq!(
            arm_pipeline.get_rustc_flags(),
            "-C opt-level=3 --target=aarch64-unknown-none"
        );
    }

    #[test]
    fn test_certification_suite() {
        let mut suite = CertificationSuite::new();
        assert_eq!(suite.certify_driver(true, 64), TestResult::Passed);
        assert_eq!(suite.certify_driver(false, 256), TestResult::Failed);
        assert_eq!(suite.total_tests, 2);
        assert_eq!(suite.passed_tests, 1);
    }

    #[test]
    fn test_disaster_recovery() {
        let mut rescue = RescueManager::new();
        assert_eq!(rescue.active_mode, MountMode::ReadOnly);
        assert!(rescue.execute_rollback_restoration(42).is_ok());
        assert_eq!(rescue.checkpoint_id, 42);
        assert!(rescue.execute_rollback_restoration(0).is_err());
    }

    #[test]
    fn test_enterprise_configurations() {
        let mut manager = EnterpriseManager::new();
        assert!(!manager.is_domain_joined);
        let config_hash = [0x55; 32];
        assert!(manager.apply_declarative_config(config_hash));
        assert_eq!(manager.config_hash, config_hash);
    }

    #[test]
    fn test_specialized_profiles() {
        let automotive = SpecializedProfile::new(VerticalPreset::AutomotiveSafetyLoop);
        assert_eq!(automotive.get_process_priority_limit(), 255);

        let sandbox = SpecializedProfile::new(VerticalPreset::EducationalSandbox);
        assert_eq!(sandbox.get_process_priority_limit(), 32);
    }
}
