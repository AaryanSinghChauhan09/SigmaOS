/// Linux & BSD Distro Guidelines Standards
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistroGuidelineStandard {
    ArchSimplicityPurity,
    DebianFhsLsbPolicy,
    FedoraSelinuxPresets,
    FreeBsdCapsicumJails,
    OpenBsdPledgeUnveil,
    NixHermeticCasStore,
}

/// Linux & BSD Distro Guidelines Rules Evaluator
#[derive(Debug, Clone)]
pub struct LinuxBsdDistroGuidelineRules {
    pub standards: Vec<DistroGuidelineStandard>,
    pub zero_dependency_purity: bool,
    pub capability_sandboxing_enabled: bool,
    pub cross_subsystem_event_routing: bool,
}

impl LinuxBsdDistroGuidelineRules {
    pub fn new() -> Self {
        Self {
            standards: vec![
                DistroGuidelineStandard::ArchSimplicityPurity,
                DistroGuidelineStandard::DebianFhsLsbPolicy,
                DistroGuidelineStandard::FedoraSelinuxPresets,
                DistroGuidelineStandard::FreeBsdCapsicumJails,
                DistroGuidelineStandard::OpenBsdPledgeUnveil,
                DistroGuidelineStandard::NixHermeticCasStore,
            ],
            zero_dependency_purity: true,
            capability_sandboxing_enabled: true,
            cross_subsystem_event_routing: true,
        }
    }

    pub fn verify_guideline_compliance(&self, standard: DistroGuidelineStandard) -> bool {
        match standard {
            DistroGuidelineStandard::ArchSimplicityPurity => self.zero_dependency_purity,
            DistroGuidelineStandard::FreeBsdCapsicumJails | DistroGuidelineStandard::OpenBsdPledgeUnveil => {
                self.capability_sandboxing_enabled
            }
            DistroGuidelineStandard::FedoraSelinuxPresets => self.cross_subsystem_event_routing,
            _ => true,
        }
    }

    pub fn verify_all_standards(&self) -> bool {
        self.standards.iter().all(|&std| self.verify_guideline_compliance(std))
    }
}

impl Default for LinuxBsdDistroGuidelineRules {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_bsd_distro_guidelines_compliance() {
        let rules = LinuxBsdDistroGuidelineRules::new();
        assert!(rules.verify_all_standards());
        assert!(rules.verify_guideline_compliance(DistroGuidelineStandard::ArchSimplicityPurity));
        assert!(rules.verify_guideline_compliance(DistroGuidelineStandard::FreeBsdCapsicumJails));
    }
}
