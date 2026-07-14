use crate::policy::Policy;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone)]
pub struct Profile {
    pub id: u64,
    pub name: String,
    pub active_policy: Policy,
}

impl Profile {
    pub fn new(policy: Policy) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            name: policy.name.clone(),
            active_policy: policy,
        }
    }
}

/// Manages capability profiles, replacing AppArmor's text-based profile configurations.
pub struct ProfileSystem {
    profiles: Vec<Profile>,
}

impl Default for ProfileSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl ProfileSystem {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
        }
    }

    pub fn create(&mut self, policy: Policy) -> Profile {
        let profile = Profile::new(policy);
        self.profiles.push(profile.clone());
        profile
    }

    pub fn get_profile(&self, name: &str) -> Option<&Profile> {
        self.profiles.iter().find(|p| p.name == name)
    }
}
