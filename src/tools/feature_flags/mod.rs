// SigmaOS Feature Flags Module
// Inspired by Gentoo Portage USE flags

pub mod sigma_features;

pub use sigma_features::{
    FeatureFlag, FeatureFlagConfig, FeatureProfile, FeatureFlagResolver,
    MAX_FEATURE_FLAGS, init_default_flags, init_default_profiles, calculate_flag_hash,
};