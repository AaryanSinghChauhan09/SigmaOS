// SigmaOS Feature Flags Module
// Inspired by Gentoo Portage USE flags

pub mod sigma_features;

pub use sigma_features::{
    calculate_flag_hash, init_default_flags, init_default_profiles, FeatureFlag, FeatureFlagConfig,
    FeatureFlagResolver, FeatureProfile, MAX_FEATURE_FLAGS,
};
