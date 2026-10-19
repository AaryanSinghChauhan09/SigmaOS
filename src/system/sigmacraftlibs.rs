//! SigmaCraftLibs - Shared Libraries System
//!
//! Provides comprehensive shared libraries management inspired by CraftLibs.
//! Supports library loading, versioning, dependencies, and shared resources.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Library type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibraryType {
    Static,
    Dynamic,
    SharedObject,
    Framework,
}

/// Library version
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl LibraryVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Library dependency
#[derive(Debug, Clone)]
pub struct LibraryDependency {
    pub name: String,
    pub version: LibraryVersion,
    pub required: bool,
}

impl LibraryDependency {
    pub fn new(name: String, version: LibraryVersion, required: bool) -> Self {
        Self {
            name,
            version,
            required,
        }
    }
}

/// Shared library
#[derive(Debug, Clone)]
pub struct SharedLibrary {
    pub id: String,
    pub name: String,
    pub library_type: LibraryType,
    pub version: LibraryVersion,
    pub file_path: String,
    pub dependencies: Vec<LibraryDependency>,
    pub loaded: bool,
}

impl SharedLibrary {
    pub fn new(
        id: String,
        name: String,
        library_type: LibraryType,
        version: LibraryVersion,
        file_path: String,
    ) -> Self {
        Self {
            id,
            name,
            library_type,
            version,
            file_path,
            dependencies: Vec::new(),
            loaded: false,
        }
    }

    pub fn add_dependency(&mut self, dependency: LibraryDependency) {
        self.dependencies.push(dependency);
    }

    pub fn remove_dependency(&mut self, index: usize) -> bool {
        if index < self.dependencies.len() {
            self.dependencies.remove(index);
            true
        } else {
            false
        }
    }

    pub fn set_loaded(&mut self, loaded: bool) {
        self.loaded = loaded;
    }

    pub fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }
}

/// SigmaCraftLibs - Shared libraries system
#[derive(Debug)]
pub struct SigmaCraftLibs {
    libraries: BTreeMap<String, SharedLibrary>,
    loaded_libraries: Vec<String>, // library IDs in load order
}

impl SigmaCraftLibs {
    pub fn new() -> Self {
        Self {
            libraries: BTreeMap::new(),
            loaded_libraries: Vec::new(),
        }
    }

    pub fn add_library(&mut self, library: SharedLibrary) {
        self.libraries.insert(library.id.clone(), library);
    }

    pub fn remove_library(&mut self, id: &str) -> bool {
        if self.libraries.remove(id).is_some() {
            self.loaded_libraries.retain(|x| x != id);
            true
        } else {
            false
        }
    }

    pub fn get_library(&self, id: &str) -> Option<&SharedLibrary> {
        self.libraries.get(id)
    }

    pub fn get_library_mut(&mut self, id: &str) -> Option<&mut SharedLibrary> {
        self.libraries.get_mut(id)
    }

    pub fn load_library(&mut self, id: &str) -> bool {
        if let Some(lib) = self.libraries.get_mut(id) {
            if !lib.loaded {
                lib.loaded = true;
                self.loaded_libraries.push(id.to_string());
            }
            true
        } else {
            false
        }
    }

    pub fn unload_library(&mut self, id: &str) -> bool {
        if let Some(lib) = self.libraries.get_mut(id) {
            lib.loaded = false;
            self.loaded_libraries.retain(|x| x != id);
            true
        } else {
            false
        }
    }

    pub fn check_dependencies(&self, id: &str) -> bool {
        if let Some(lib) = self.libraries.get(id) {
            for dep in &lib.dependencies {
                if dep.required {
                    if let Some(dep_lib) = self.libraries.get(&dep.name) {
                        if !dep_lib.loaded {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
            true
        } else {
            false
        }
    }

    pub fn list_libraries(&self) -> Vec<&SharedLibrary> {
        self.libraries.values().collect()
    }

    pub fn list_loaded(&self) -> Vec<&SharedLibrary> {
        self.loaded_libraries
            .iter()
            .filter_map(|id| self.libraries.get(id))
            .collect()
    }

    pub fn library_count(&self) -> usize {
        self.libraries.len()
    }

    pub fn loaded_count(&self) -> usize {
        self.loaded_libraries.len()
    }
}

impl Default for SigmaCraftLibs {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaCraftLibs instance
static mut SIGMACRAFTLIBS_INSTANCE: Option<SigmaCraftLibs> = None;

/// Initialize the global SigmaCraftLibs instance
pub fn init_sigmacraftlibs() {
    unsafe {
        SIGMACRAFTLIBS_INSTANCE = Some(SigmaCraftLibs::new());
    }
}

/// Get the global SigmaCraftLibs instance
pub fn get_sigmacraftlibs() -> Option<&'static SigmaCraftLibs> {
    unsafe { SIGMACRAFTLIBS_INSTANCE.as_ref() }
}

/// Get the global SigmaCraftLibs instance (mutable)
pub fn get_sigmacraftlibs_mut() -> Option<&'static mut SigmaCraftLibs> {
    unsafe { SIGMACRAFTLIBS_INSTANCE.as_mut() }
}

/// Convenience function to add a library
pub fn add_library(library: SharedLibrary) -> bool {
    if let Some(craftlibs) = get_sigmacraftlibs_mut() {
        craftlibs.add_library(library);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_version_creation() {
        let version = LibraryVersion::new(1, 2, 3);
        assert_eq!(version.to_string(), "1.2.3");
    }

    #[test]
    fn test_library_dependency_creation() {
        let version = LibraryVersion::new(1, 0, 0);
        let dep = LibraryDependency::new("lib1".to_string(), version, true);
        assert_eq!(dep.name, "lib1");
        assert!(dep.required);
    }

    #[test]
    fn test_shared_library_creation() {
        let version = LibraryVersion::new(1, 0, 0);
        let lib = SharedLibrary::new(
            "lib1".to_string(),
            "TestLib".to_string(),
            LibraryType::Dynamic,
            version,
            "/path/lib.so".to_string(),
        );
        assert_eq!(lib.id, "lib1");
        assert!(!lib.loaded);
    }

    #[test]
    fn test_shared_library_modification() {
        let version = LibraryVersion::new(1, 0, 0);
        let mut lib = SharedLibrary::new(
            "lib1".to_string(),
            "Test".to_string(),
            LibraryType::Dynamic,
            version,
            "/path".to_string(),
        );
        lib.add_dependency(LibraryDependency::new(
            "dep1".to_string(),
            LibraryVersion::new(1, 0, 0),
            true,
        ));
        lib.set_loaded(true);
        assert_eq!(lib.dependency_count(), 1);
        assert!(lib.loaded);
    }

    #[test]
    fn test_sigmacraftlibs_creation() {
        let craftlibs = SigmaCraftLibs::new();
        assert_eq!(craftlibs.library_count(), 0);
        assert_eq!(craftlibs.loaded_count(), 0);
    }

    #[test]
    fn test_sigmacraftlibs_management() {
        let mut craftlibs = SigmaCraftLibs::new();
        let version = LibraryVersion::new(1, 0, 0);
        let lib = SharedLibrary::new(
            "lib1".to_string(),
            "Test".to_string(),
            LibraryType::Dynamic,
            version,
            "/path".to_string(),
        );
        craftlibs.add_library(lib);
        assert_eq!(craftlibs.library_count(), 1);
    }

    #[test]
    fn test_library_loading() {
        let mut craftlibs = SigmaCraftLibs::new();
        let version = LibraryVersion::new(1, 0, 0);
        let lib = SharedLibrary::new(
            "lib1".to_string(),
            "Test".to_string(),
            LibraryType::Dynamic,
            version,
            "/path".to_string(),
        );
        craftlibs.add_library(lib);
        assert!(craftlibs.load_library("lib1"));
        assert_eq!(craftlibs.loaded_count(), 1);
    }
}
