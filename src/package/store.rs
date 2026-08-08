// SigmaOS Polish-Parity Software Store & Update Manager (SigmaStore)
// Designed for software installation, package upgrades, and security auditing

use crate::klib::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreError {
    Success = 0,
    PackageNotFound = 1,
    InstallFailed = 2,
    InsecurePackage = 3,
}

pub struct StoreApp {
    pub name: String,
    pub version: String,
    pub category: String,
    pub is_installed: bool,
    pub safety_score: f32, // 0.0 to 1.0 (GDPR/Compliance check)
}

pub struct SigmaSoftwareStore {
    pub catalog: HashMap<String, StoreApp>,
    pub pending_updates: Vec<String>,
}

impl SigmaSoftwareStore {
    pub fn new() -> Self {
        let mut store = SigmaSoftwareStore {
            catalog: HashMap::new(),
            pending_updates: Vec::new(),
        };
        store.register_app(StoreApp {
            name: "sigma-browse".to_string(),
            version: "1.0.0".to_string(),
            category: "Internet".to_string(),
            is_installed: false,
            safety_score: 0.98,
        });
        store.register_app(StoreApp {
            name: "sigma-paint".to_string(),
            version: "1.2.0".to_string(),
            category: "Graphics".to_string(),
            is_installed: false,
            safety_score: 0.95,
        });
        store
    }

    pub fn register_app(&mut self, app: StoreApp) {
        self.catalog.insert(app.name.clone(), app);
    }

    pub fn install_app(&mut self, name: &str) -> Result<(), StoreError> {
        if let Some(app) = self.catalog.get_mut(name) {
            if app.safety_score < 0.5 {
                return Err(StoreError::InsecurePackage);
            }
            app.is_installed = true;
            Ok(())
        } else {
            Err(StoreError::PackageNotFound)
        }
    }

    pub fn check_for_updates(&mut self) -> usize {
        self.pending_updates.clear();
        for (name, app) in &self.catalog {
            if app.is_installed && app.version != "1.5.0" {
                // Assume latest stable is 1.5.0
                let name_str: String = name.clone();
                self.pending_updates.push(name_str);
            }
        }
        self.pending_updates.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_software_store_install() {
        let mut store = SigmaSoftwareStore::new();
        assert!(store.install_app("sigma-paint").is_ok());
        let app = store.catalog.get("sigma-paint").unwrap();
        assert!(app.is_installed);
    }

    #[test]
    fn test_software_store_updates() {
        let mut store = SigmaSoftwareStore::new();
        store.install_app("sigma-browse").unwrap();
        let update_count = store.check_for_updates();
        assert_eq!(update_count, 1);
        assert_eq!(store.pending_updates[0], "sigma-browse");
    }
}
