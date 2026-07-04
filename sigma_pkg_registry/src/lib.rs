use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SigPkgManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub install: Vec<String>,
}

impl SigPkgManifest {
    pub fn new(name: String, version: String) -> Self {
        SigPkgManifest {
            name,
            version,
            description: String::new(),
            dependencies: Vec::new(),
            install: Vec::new(),
        }
    }
}
