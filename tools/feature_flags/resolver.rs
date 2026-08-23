// SPDX-License-Identifier: MIT
// Feature Flag Resolution Engine for SigmaOS
// Location: tools/feature_flags/resolver.rs

#![no_std]
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::collections::BTreeMap;
use super::sigma_features::{FeatureFlag, FeatureFlagRegistry};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolutionError {
    CircularDependency(String),
    MissingDependency(String),
    ConflictDetected(String),
    RequiredUseViolation(String),
}

pub struct FeatureFlagResolver {
    registry: FeatureFlagRegistry,
    user_overrides: BTreeMap<String, bool>,
    profile_defaults: BTreeMap<String, bool>,
    required_use_exprs: Vec<(String, String)>, // (Condition flag, required flag)
}

impl FeatureFlagResolver {
    pub fn new(registry: FeatureFlagRegistry) -> Self {
        FeatureFlagResolver {
            registry,
            user_overrides: BTreeMap::new(),
            profile_defaults: BTreeMap::new(),
            required_use_exprs: Vec::new(),
        }
    }

    pub fn set_profile_default(&mut self, name: &str, enabled: bool) {
        self.profile_defaults.insert(String::from(name), enabled);
    }

    pub fn set_user_override(&mut self, name: &str, enabled: bool) {
        self.user_overrides.insert(String::from(name), enabled);
    }

    pub fn add_required_use_rule(&mut self, if_flag: &str, require_flag: &str) {
        self.required_use_exprs.push((String::from(if_flag), String::from(require_flag)));
    }

    pub fn resolve(&mut self) -> Result<BTreeMap<String, bool>, ResolutionError> {
        let mut resolved_state = BTreeMap::new();

        // 1. Apply Profile Defaults
        for (flag, &def) in &self.profile_defaults {
            resolved_state.insert(flag.clone(), def);
        }

        // 2. Apply User Overrides (takes precedence)
        for (flag, &override_val) in &self.user_overrides {
            resolved_state.insert(flag.clone(), override_val);
        }

        // 3. Resolve Dependencies and Cycle Detection
        let mut visited = Vec::new();

        for flag_name in resolved_state.keys().cloned().collect::<Vec<_>>() {
            self.check_cycles(&flag_name, &mut visited)?;
        }

        // 4. Validate REQUIRED_USE constraints
        for (if_flag, req_flag) in &self.required_use_exprs {
            let if_enabled = resolved_state.get(if_flag).copied().unwrap_or(false);
            if if_enabled {
                let req_enabled = resolved_state.get(req_flag).copied().unwrap_or(false);
                if !req_enabled {
                    return Err(ResolutionError::RequiredUseViolation(
                        alloc::format!("Flag '{}' requires '{}' to be enabled", if_flag, req_flag)
                    ));
                }
            }
        }

        Ok(resolved_state)
    }

    fn check_cycles(&self, current: &str, visited: &mut Vec<String>) -> Result<(), ResolutionError> {
        if visited.contains(&String::from(current)) {
            return Err(ResolutionError::CircularDependency(String::from(current)));
        }

        visited.push(String::from(current));
        // Check dependent flags if registered
        if let Some(flag) = self.registry.find_flag(current) {
            for i in 0..flag.dep_count as usize {
                let dep_id = flag.dependencies[i];
                let dep_name = alloc::format!("dep_{}", dep_id);
                self.check_cycles(&dep_name, visited)?;
            }
        }
        visited.pop();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_resolver_precedence_and_required_use() {
        let mut registry = FeatureFlagRegistry::new();
        registry.register_flag(FeatureFlag::new("wayland", "Wayland Support", false, true));
        registry.register_flag(FeatureFlag::new("opengl", "OpenGL Support", false, true));

        let mut resolver = FeatureFlagResolver::new(registry);
        resolver.set_profile_default("wayland", false);
        resolver.set_profile_default("opengl", true);
        resolver.set_user_override("wayland", true); // Overrides profile default

        resolver.add_required_use_rule("wayland", "opengl");

        let resolved = resolver.resolve().expect("Resolution should succeed");
        assert_eq!(resolved.get("wayland"), Some(&true));
        assert_eq!(resolved.get("opengl"), Some(&true));
    }

    #[test]
    fn test_required_use_violation() {
        let registry = FeatureFlagRegistry::new();
        let mut resolver = FeatureFlagResolver::new(registry);
        resolver.set_user_override("wayland", true);
        resolver.set_user_override("opengl", false);
        resolver.add_required_use_rule("wayland", "opengl");

        let res = resolver.resolve();
        assert!(matches!(res, Err(ResolutionError::RequiredUseViolation(_))));
    }
}
