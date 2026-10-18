// SPDX-License-Identifier: MIT
// SigmaOS - Artcraft (Creative Suite Integration)
// Inspired by Adobe Creative Cloud, Serif Affinity Suite
// Provides unified creative suite management

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Project type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectType {
    Photo,
    Video,
    Vector,
    PDF,
    Design,
    Effect,
    Scene,
    Deck,
    Presentation,
    Grid,
    Spreadsheet,
    GridCraft,
    WordCraft,
    Document,
    TextDocument,
    Word,
    Light,
    Audio,
    Media,
    Model3D,
    Drafting,
    SoundCraft,
    DeckCraft,
    CadCraft,
    Cad,
    SolidModel,
    Figcraft,
    Fig,
    UiUx,
    Prototype,
    Animation,
    LightCraft,
    MediaCraft,
    AudioCraft,
}

/// Project
#[derive(Debug, Clone)]
pub struct Project {
    pub id: u32,
    pub name: String,
    pub project_type: ProjectType,
    pub path: Option<String>,
    pub created: bool,
}

/// Workspace
#[derive(Debug, Clone)]
pub struct Workspace {
    pub id: u32,
    pub name: String,
    pub projects: Vec<Project>,
}

/// Art suite manager
pub struct ArtSuite {
    pub workspaces: BTreeMap<u32, Workspace>,
    pub next_workspace_id: AtomicU32,
    pub next_project_id: AtomicU32,
    pub active: AtomicBool,
}

impl ArtSuite {
    pub fn new() -> Self {
        Self {
            workspaces: BTreeMap::new(),
            next_workspace_id: AtomicU32::new(1),
            next_project_id: AtomicU32::new(1),
            active: AtomicBool::new(false),
        }
    }

    /// Initialize art suite
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }

    /// Create workspace
    pub fn create_workspace(&mut self, name: String) -> u32 {
        let id = self.next_workspace_id.fetch_add(1, Ordering::Relaxed);
        let workspace = Workspace {
            id,
            name,
            projects: Vec::new(),
        };
        self.workspaces.insert(id, workspace);
        id
    }

    /// Create project
    pub fn create_project(
        &mut self,
        workspace_id: u32,
        name: String,
        project_type: ProjectType,
    ) -> Result<u32, &'static str> {
        let workspace = self
            .workspaces
            .get_mut(&workspace_id)
            .ok_or("Workspace not found")?;
        let id = self.next_project_id.fetch_add(1, Ordering::Relaxed);
        let project = Project {
            id,
            name,
            project_type,
            path: None,
            created: true,
        };
        workspace.projects.push(project);
        Ok(id)
    }

    /// Get workspace
    pub fn get_workspace(&self, id: u32) -> Option<&Workspace> {
        self.workspaces.get(&id)
    }

    /// Get project
    pub fn get_project(&self, workspace_id: u32, project_id: u32) -> Option<&Project> {
        let workspace = self.workspaces.get(&workspace_id)?;
        workspace.projects.iter().find(|p| p.id == project_id)
    }

    /// Close / remove workspace by ID
    pub fn close_workspace(&mut self, workspace_id: u32) -> bool {
        self.workspaces.remove(&workspace_id).is_some()
    }

    /// Get all projects in a workspace of a specific type
    pub fn get_projects_by_type(
        &self,
        workspace_id: u32,
        project_type: ProjectType,
    ) -> Vec<&Project> {
        if let Some(workspace) = self.workspaces.get(&workspace_id) {
            workspace
                .projects
                .iter()
                .filter(|p| p.project_type == project_type)
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Export PR proposal manifest for a given workspace
    pub fn export_workspace_pr_manifest(&self, workspace_id: u32) -> Result<String, &'static str> {
        let workspace = self
            .workspaces
            .get(&workspace_id)
            .ok_or("Workspace not found")?;

        let mut manifest = format!(
            "### ArtCraft Workspace PR Manifest: {}\n- **Workspace ID**: {}\n- **Total Projects**: {}\n\n#### Included Projects:\n",
            workspace.name, workspace.id, workspace.projects.len()
        );

        for proj in &workspace.projects {
            manifest.push_str(&format!(
                "- **ID**: {} | **Name**: {} | **Type**: {:?}\n",
                proj.id, proj.name, proj.project_type
            ));
        }

        Ok(manifest)
    }

    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for ArtSuite {
    fn default() -> Self {
        Self::new()
    }
}

/// Global artcraft
static mut ARTCRAFT: Option<ArtSuite> = None;

/// Initialize artcraft
pub fn init_artcraft() {
    unsafe {
        ARTCRAFT = Some(ArtSuite::new());
    }
}

/// Get global artcraft
pub fn get_artcraft() -> Option<&'static mut ArtSuite> {
    unsafe { ARTCRAFT.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artcraft_init() {
        let mut suite = ArtSuite::new();
        suite.init();
        assert!(suite.is_active());
    }

    #[test]
    fn test_create_workspace() {
        let mut suite = ArtSuite::new();
        suite.init();
        let id = suite.create_workspace("Workspace1".to_string());
        assert!(suite.get_workspace(id).is_some());
    }

    #[test]
    fn test_create_project() {
        let mut suite = ArtSuite::new();
        suite.init();
        let workspace_id = suite.create_workspace("Workspace1".to_string());
        suite
            .create_project(
                workspace_id,
                "Photo Project".to_string(),
                ProjectType::Photo,
            )
            .unwrap();
        assert_eq!(suite.get_workspace(workspace_id).unwrap().projects.len(), 1);
    }

    #[test]
    fn test_close_workspace() {
        let mut suite = ArtSuite::new();
        suite.init();
        let id = suite.create_workspace("Workspace1".to_string());
        assert!(suite.close_workspace(id));
        assert!(suite.get_workspace(id).is_none());
        assert!(!suite.close_workspace(id));
    }

    #[test]
    fn test_get_projects_by_type() {
        let mut suite = ArtSuite::new();
        suite.init();
        let w_id = suite.create_workspace("Workspace1".to_string());
        suite
            .create_project(w_id, "Photo 1".to_string(), ProjectType::Photo)
            .unwrap();
        suite
            .create_project(w_id, "Video 1".to_string(), ProjectType::Video)
            .unwrap();
        suite
            .create_project(w_id, "Photo 2".to_string(), ProjectType::Photo)
            .unwrap();

        let photos = suite.get_projects_by_type(w_id, ProjectType::Photo);
        assert_eq!(photos.len(), 2);
    }

    #[test]
    fn test_export_workspace_pr_manifest() {
        let mut suite = ArtSuite::new();
        suite.init();
        let w_id = suite.create_workspace("Design Studio".to_string());
        suite
            .create_project(w_id, "Icon Set".to_string(), ProjectType::Vector)
            .unwrap();

        let manifest = suite.export_workspace_pr_manifest(w_id).unwrap();
        assert!(manifest.contains("Design Studio"));
        assert!(manifest.contains("Icon Set"));
        assert!(manifest.contains("Vector"));
    }
}
