//! SigmaOS System Provisioning Manager
//!
//! Inspired by: Omarchy provisioning (omancy-provision-first-run, omancy-provision-owner, omancy-provision-user)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, GOVERNANCE_AND_DO_CRACY, MERITOCRACY_AND_INCLUSIVITY,
//! USER_FREEDOM_AND_CONTROL, VERSION_CONTROL_PRACTICES.

/// Provisioning stage
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvisioningStage {
    FirstRun,
    OwnerSetup,
    UserSetup,
    HardwareDetection,
    NetworkConfig,
    UserCreation,
    PackageInstall,
    Finalize,
}

/// Provisioning status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvisioningStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed(String),
    Skipped,
}

/// Provisioning step
#[derive(Debug, Clone)]
pub struct ProvisioningStep {
    pub stage: ProvisioningStage,
    pub status: ProvisioningStatus,
    pub completed_at: Option<u64>,
    pub error: Option<String>,
}

/// Provisioning manager
pub struct ProvisioningManager {
    pub stages: Vec<ProvisioningStep>,
    pub current_stage: Option<ProvisioningStage>,
    pub owner_name: Option<String>,
    pub owner_created: bool,
    pub users_created: Vec<String>,
    pub hardware_detected: bool,
    pub network_configured: bool,
}

impl ProvisioningManager {
    pub fn new() -> Self {
        let stages = vec![
            ProvisioningStep {
                stage: ProvisioningStage::FirstRun,
                status: ProvisioningStatus::NotStarted,
                completed_at: None,
                error: None,
            },
            ProvisioningStep {
                stage: ProvisioningStage::OwnerSetup,
                status: ProvisioningStatus::NotStarted,
                completed_at: None,
                error: None,
            },
            ProvisioningStep {
                stage: ProvisioningStage::UserSetup,
                status: ProvisioningStatus::NotStarted,
                completed_at: None,
                error: None,
            },
            ProvisioningStep {
                stage: ProvisioningStage::HardwareDetection,
                status: ProvisioningStatus::NotStarted,
                completed_at: None,
                error: None,
            },
            ProvisioningStep {
                stage: ProvisioningStage::NetworkConfig,
                status: ProvisioningStatus::NotStarted,
                completed_at: None,
                error: None,
            },
            ProvisioningStep {
                stage: ProvisioningStage::UserCreation,
                status: ProvisioningStatus::NotStarted,
                completed_at: None,
                error: None,
            },
            ProvisioningStep {
                stage: ProvisioningStage::PackageInstall,
                status: ProvisioningStatus::NotStarted,
                completed_at: None,
                error: None,
            },
            ProvisioningStep {
                stage: ProvisioningStage::Finalize,
                status: ProvisioningStatus::NotStarted,
                completed_at: None,
                error: None,
            },
        ];

        Self {
            stages,
            current_stage: None,
            owner_name: None,
            owner_created: false,
            users_created: Vec::new(),
            hardware_detected: false,
            network_configured: false,
        }
    }

    /// Start a stage
    pub fn start_stage(&mut self, stage: ProvisioningStage) -> Result<(), String> {
        if let Some(step) = self.stages.iter_mut().find(|s| s.stage == stage) {
            if step.status == ProvisioningStatus::Completed {
                return Err(format!("Stage {:?} already completed", stage));
            }
            step.status = ProvisioningStatus::InProgress;
            self.current_stage = Some(stage);
            Ok(())
        } else {
            Err(format!("Stage {:?} not found", stage))
        }
    }

    /// Complete a stage
    pub fn complete_stage(&mut self, stage: ProvisioningStage) -> Result<(), String> {
        let step = self
            .stages
            .iter_mut()
            .find(|s| s.stage == stage)
            .ok_or_else(|| format!("Stage {:?} not found", stage))?;

        if step.status != ProvisioningStatus::InProgress {
            return Err(format!("Stage {:?} not in progress", stage));
        }

        step.status = ProvisioningStatus::Completed;
        step.completed_at = Some(now_secs());

        if self.current_stage.as_ref() == Some(&stage) {
            self.current_stage = None;
        }

        Ok(())
    }

    /// Fail a stage
    pub fn fail_stage(&mut self, stage: ProvisioningStage, error: String) {
        if let Some(step) = self.stages.iter_mut().find(|s| s.stage == stage) {
            step.status = ProvisioningStatus::Failed(error);
            if self.current_stage.as_ref() == Some(&stage) {
                self.current_stage = None;
            }
        }
    }

    /// Skip a stage
    pub fn skip_stage(&mut self, stage: ProvisioningStage) -> Result<(), String> {
        let step = self
            .stages
            .iter_mut()
            .find(|s| s.stage == stage)
            .ok_or_else(|| format!("Stage {:?} not found", stage))?;
        step.status = ProvisioningStatus::Skipped;
        Ok(())
    }

    /// Set owner name
    pub fn set_owner(&mut self, name: String) {
        self.owner_name = Some(name);
        self.owner_created = true;
    }

    /// Add a user
    pub fn add_user(&mut self, username: String) {
        self.users_created.push(username);
    }

    /// Set hardware detected
    pub fn set_hardware_detected(&mut self, detected: bool) {
        self.hardware_detected = detected;
    }

    /// Set network configured
    pub fn set_network_configured(&mut self, configured: bool) {
        self.network_configured = configured;
    }

    /// Get stage status
    pub fn get_stage_status(&self, stage: ProvisioningStage) -> Option<&ProvisioningStatus> {
        self.stages.iter().find(|s| s.stage == stage).map(|s| &s.status)
    }

    /// Is provisioning complete
    pub fn is_complete(&self) -> bool {
        self.stages.iter().all(|s| matches!(s.status, ProvisioningStatus::Completed | ProvisioningStatus::Skipped))
    }

    /// Get progress percentage
    pub fn progress_percent(&self) -> f32 {
        let total = self.stages.len() as f32;
        let completed = self.stages.iter().filter(|s| matches!(s.status, ProvisioningStatus::Completed)).count() as f32;
        (completed / total) * 100.0
    }

    /// Stage count
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }
}

impl Default for ProvisioningManager {
    fn default() -> Self {
        Self::new()
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_complete_stage() {
        let mut mgr = ProvisioningManager::new();
        assert!(mgr.start_stage(ProvisioningStage::FirstRun).is_ok());
        assert_eq!(mgr.current_stage, Some(ProvisioningStage::FirstRun));
        assert!(mgr.complete_stage(ProvisioningStage::FirstRun).is_ok());
        assert_eq!(
            mgr.get_stage_status(ProvisioningStage::FirstRun),
            Some(&ProvisioningStatus::Completed)
        );
    }

    #[test]
    fn test_complete_without_start_fails() {
        let mut mgr = ProvisioningManager::new();
        assert!(mgr.complete_stage(ProvisioningStage::FirstRun).is_err());
    }

    #[test]
    fn test_set_owner() {
        let mut mgr = ProvisioningManager::new();
        mgr.set_owner("alice".to_string());
        assert_eq!(mgr.owner_name, Some("alice".to_string()));
        assert!(mgr.owner_created);
    }

    #[test]
    fn test_add_user() {
        let mut mgr = ProvisioningManager::new();
        mgr.add_user("bob".to_string());
        mgr.add_user("charlie".to_string());
        assert_eq!(mgr.users_created.len(), 2);
    }

    #[test]
    fn test_progress_percent() {
        let mut mgr = ProvisioningManager::new();
        assert_eq!(mgr.progress_percent(), 0.0);
        mgr.complete_stage(ProvisioningStage::FirstRun).ok();
        assert!((mgr.progress_percent() - 12.5).abs() < 0.01);
    }

    #[test]
    fn test_is_complete() {
        let mut mgr = ProvisioningManager::new();
        assert!(!mgr.is_complete());
        for stage in [
            ProvisioningStage::FirstRun,
            ProvisioningStage::OwnerSetup,
            ProvisioningStage::UserSetup,
            ProvisioningStage::HardwareDetection,
            ProvisioningStage::NetworkConfig,
            ProvisioningStage::UserCreation,
            ProvisioningStage::PackageInstall,
            ProvisioningStage::Finalize,
        ] {
            mgr.complete_stage(stage).ok();
        }
        assert!(mgr.is_complete());
    }
}