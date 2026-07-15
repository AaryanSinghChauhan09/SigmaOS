// Community Build Infrastructure for SigmaOS
// Inspired by Open Build Service (OBS) from openSUSE
// Provides distributed build system for community packages

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildProject {
    pub name: String,
    pub description: String,
    pub repository: String,
    pub recipes: Vec<BuildRecipe>,
    pub architectures: Vec<String>,
    pub maintainers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildRecipe {
    pub name: String,
    pub version: String,
    pub source_url: String,
    pub build_system: BuildSystem,
    pub dependencies: Vec<String>,
    pub build_dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BuildSystem {
    Autotools,
    CMake,
    Meson,
    Cargo,
    Go,
    Python,
    Custom(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildResult {
    pub project: String,
    pub recipe: String,
    pub architecture: String,
    pub status: BuildStatus,
    pub build_time: i64,
    pub log_path: PathBuf,
    pub package_path: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BuildStatus {
    Pending,
    Building,
    Success,
    Failed,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildWorker {
    pub id: String,
    pub architecture: String,
    pub capabilities: Vec<String>,
    pub status: WorkerStatus,
    pub current_build: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum WorkerStatus {
    Idle,
    Busy,
    Offline,
}

pub struct CommunityBuild {
    build_dir: PathBuf,
    projects: HashMap<String, BuildProject>,
    workers: HashMap<String, BuildWorker>,
    results: Vec<BuildResult>,
}

impl CommunityBuild {
    pub fn new(build_dir: PathBuf) -> Result<Self, std::io::Error> {
        fs::create_dir_all(&build_dir)?;
        
        let projects = Self::load_projects(&build_dir)?;
        let workers = Self::load_workers(&build_dir)?;
        let results = Self::load_results(&build_dir)?;
        
        Ok(CommunityBuild {
            build_dir,
            projects,
            workers,
            results,
        })
    }

    /// Add a new build project
    pub fn add_project(&mut self, project: BuildProject) -> Result<(), std::io::Error> {
        let project_dir = self.build_dir.join("projects").join(&project.name);
        fs::create_dir_all(&project_dir)?;
        
        let project_file = project_dir.join("project.toml");
        let content = toml::to_string_pretty(&project)?;
        fs::write(&project_file, content)?;
        
        self.projects.insert(project.name.clone(), project);
        Ok(())
    }

    /// Submit a build request
    pub fn submit_build(&mut self, project_name: &str, recipe_name: &str) -> Result<String, std::io::Error> {
        if let Some(project) = self.projects.get(project_name) {
            let build_id = format!("{}_{}_{}", project_name, recipe_name, chrono::Utc::now().timestamp());
            
            for arch in &project.architectures {
                let result = BuildResult {
                    project: project_name.to_string(),
                    recipe: recipe_name.to_string(),
                    architecture: arch.clone(),
                    status: BuildStatus::Pending,
                    build_time: 0,
                    log_path: self.build_dir.join("logs").join(&build_id).join(format!("{}.log", arch)),
                    package_path: None,
                };
                
                self.results.push(result);
            }
            
            self.save_results()?;
            Ok(build_id)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Project {} not found", project_name),
            ))
        }
    }

    /// Register a build worker
    pub fn register_worker(&mut self, worker: BuildWorker) -> Result<(), std::io::Error> {
        let worker_file = self.build_dir.join("workers").join(format!("{}.toml", worker.id));
        let content = toml::to_string_pretty(&worker)?;
        fs::write(&worker_file, content)?;
        
        self.workers.insert(worker.id.clone(), worker);
        Ok(())
    }

    /// Process pending builds
    pub fn process_builds(&mut self) -> Result<usize, std::io::Error> {
        let mut processed = 0;
        
        for result in &mut self.results {
            if result.status == BuildStatus::Pending {
                if let Some(worker) = self.find_available_worker(&result.architecture) {
                    result.status = BuildStatus::Building;
                    result.build_time = chrono::Utc::now().timestamp();
                    
                    // Mark worker as busy
                    if let Some(w) = self.workers.get_mut(&worker.id) {
                        w.status = WorkerStatus::Busy;
                        w.current_build = Some(format!("{}:{}", result.project, result.recipe));
                    }
                    
                    processed += 1;
                    
                    // Simulate build
                    self.simulate_build(result, &worker)?;
                }
            }
        }
        
        self.save_results()?;
        self.save_workers()?;
        Ok(processed)
    }

    /// Get build results for a project
    pub fn get_results(&self, project_name: &str) -> Vec<&BuildResult> {
        self.results.iter()
            .filter(|r| r.project == project_name)
            .collect()
    }

    /// Get all projects
    pub fn list_projects(&self) -> Vec<&BuildProject> {
        self.projects.values().collect()
    }

    /// Get all workers
    pub fn list_workers(&self) -> Vec<&BuildWorker> {
        self.workers.values().collect()
    }

    fn find_available_worker(&self, architecture: &str) -> Option<&BuildWorker> {
        self.workers.values()
            .find(|w| w.architecture == architecture && w.status == WorkerStatus::Idle)
    }

    fn simulate_build(&self, result: &mut BuildResult, worker: &BuildWorker) -> Result<(), std::io::Error> {
        println!("Building {}:{} on {} ({})", 
            result.project, result.recipe, worker.id, result.architecture);
        
        // Create log directory
        fs::create_dir_all(result.log_path.parent().unwrap())?;
        
        // Write build log
        let log_content = format!(
            "Build started: {}\n\
             Project: {}\n\
             Recipe: {}\n\
             Architecture: {}\n\
             Worker: {}\n\
             Status: Building\n",
            chrono::Utc::now().to_rfc3339(),
            result.project,
            result.recipe,
            result.architecture,
            worker.id
        );
        
        fs::write(&result.log_path, log_content)?;
        
        // Simulate success
        result.status = BuildStatus::Success;
        result.package_path = Some(self.build_dir.join("packages")
            .join(&result.project)
            .join(format!("{}-{}.rpm", result.recipe, result.architecture)));
        
        Ok(())
    }

    fn load_projects(build_dir: &Path) -> Result<HashMap<String, BuildProject>, std::io::Error> {
        let projects_dir = build_dir.join("projects");
        let mut projects = HashMap::new();
        
        if projects_dir.exists() {
            for entry in fs::read_dir(&projects_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    let project_file = path.join("project.toml");
                    if project_file.exists() {
                        let content = fs::read_to_string(&project_file)?;
                        if let Ok(project) = toml::from_str::<BuildProject>(&content) {
                            projects.insert(project.name.clone(), project);
                        }
                    }
                }
            }
        }
        
        Ok(projects)
    }

    fn load_workers(build_dir: &Path) -> Result<HashMap<String, BuildWorker>, std::io::Error> {
        let workers_dir = build_dir.join("workers");
        let mut workers = HashMap::new();
        
        if workers_dir.exists() {
            for entry in fs::read_dir(&workers_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.extension().map(|e| e == "toml").unwrap_or(false) {
                    let content = fs::read_to_string(&path)?;
                    if let Ok(worker) = toml::from_str::<BuildWorker>(&content) {
                        workers.insert(worker.id.clone(), worker);
                    }
                }
            }
        }
        
        Ok(workers)
    }

    fn load_results(build_dir: &Path) -> Result<Vec<BuildResult>, std::io::Error> {
        let results_file = build_dir.join("results.json");
        
        if results_file.exists() {
            let content = fs::read_to_string(&results_file)?;
            let results: Vec<BuildResult> = serde_json::from_str(&content)?;
            Ok(results)
        } else {
            Ok(Vec::new())
        }
    }

    fn save_results(&self) -> Result<(), std::io::Error> {
        let results_file = self.build_dir.join("results.json");
        let content = serde_json::to_string_pretty(&self.results)?;
        fs::write(&results_file, content)?;
        Ok(())
    }

    fn save_workers(&self) -> Result<(), std::io::Error> {
        for worker in self.workers.values() {
            let worker_file = self.build_dir.join("workers").join(format!("{}.toml", worker.id));
            let content = toml::to_string_pretty(worker)?;
            fs::write(&worker_file, content)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_community_build_creation() {
        let temp_dir = tempdir().unwrap();
        let build_dir = temp_dir.path().to_path_buf();
        
        let build = CommunityBuild::new(build_dir).unwrap();
        assert_eq!(build.projects.len(), 0);
        assert_eq!(build.workers.len(), 0);
    }

    #[test]
    fn test_add_project() {
        let temp_dir = tempdir().unwrap();
        let build_dir = temp_dir.path().to_path_buf();
        
        let mut build = CommunityBuild::new(build_dir).unwrap();
        
        let project = BuildProject {
            name: "test-project".to_string(),
            description: "Test project".to_string(),
            repository: "https://github.com/test/project".to_string(),
            recipes: vec![],
            architectures: vec!["x86_64".to_string()],
            maintainers: vec!["test@example.com".to_string()],
        };
        
        build.add_project(project).unwrap();
        assert_eq!(build.projects.len(), 1);
    }

    #[test]
    fn test_register_worker() {
        let temp_dir = tempdir().unwrap();
        let build_dir = temp_dir.path().to_path_buf();
        
        let mut build = CommunityBuild::new(build_dir).unwrap();
        
        let worker = BuildWorker {
            id: "worker-1".to_string(),
            architecture: "x86_64".to_string(),
            capabilities: vec!["autotools".to_string(), "cmake".to_string()],
            status: WorkerStatus::Idle,
            current_build: None,
        };
        
        build.register_worker(worker).unwrap();
        assert_eq!(build.workers.len(), 1);
    }
}
