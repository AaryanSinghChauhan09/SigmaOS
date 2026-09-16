// SigmaOS DNF (Dandified YUM) Implementation
// Implements Fedora-style package management for SigmaOS
// Inspired by Fedora's DNF for modern package operations

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

/// DNF error types
#[derive(Debug, Clone)]
pub enum DnfError {
    PackageNotFound,
    PackageNotInstalled,
    DependencyResolutionFailed,
    TransactionProblems(Vec<String>),
    RepositoryError,
    DownloadFailed,
    InstallationFailed,
}

/// Package metadata
#[derive(Debug, Clone)]
pub struct DnfPackage {
    pub name: String,
    pub version: String,
    pub release: String,
    pub epoch: Option<u32>,
    pub architecture: String,
    pub summary: String,
    pub description: String,
    pub url: String,
    pub license: String,
    pub dependencies: Vec<String>,
    pub provides: Vec<String>,
    pub obsoletes: Vec<String>,
    pub conflicts: Vec<String>,
    pub requires: Vec<String>,
    pub size: u64,
    pub installed_size: u64,
    pub repository: String,
}

/// Transaction operation
#[derive(Debug, Clone)]
pub enum TransactionOperation {
    Install {
        package: String,
        version: String,
    },
    Remove {
        package: String,
    },
    Update {
        package: String,
        from_version: String,
        to_version: String,
    },
    Obsolete {
        package: String,
    },
    Reinstall {
        package: String,
        version: String,
    },
}

/// Transaction
#[derive(Debug, Clone)]
pub struct Transaction {
    pub operations: Vec<TransactionOperation>,
    pub dependencies: Vec<String>,
    pub problems: Vec<String>,
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
            dependencies: Vec::new(),
            problems: Vec::new(),
        }
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new()
    }
}

/// Repository
#[derive(Debug, Clone)]
pub struct Repository {
    pub name: String,
    pub baseurl: String,
    pub enabled: bool,
    pub gpgcheck: bool,
    pub gpgkey: Option<String>,
    pub priority: u32,
}

/// DNF package manager
pub struct SigmaDNF {
    pub database: BTreeMap<String, DnfPackage>,
    pub repositories: Vec<Repository>,
    pub installed: BTreeMap<String, String>,
}

impl SigmaDNF {
    pub fn new() -> Self {
        Self {
            database: BTreeMap::new(),
            repositories: Vec::new(),
            installed: BTreeMap::new(),
        }
    }

    /// Add repository
    pub fn add_repository(&mut self, repo: Repository) {
        self.repositories.push(repo);
    }

    /// Add package to database
    pub fn add_package(&mut self, package: DnfPackage) {
        self.database.insert(package.name.clone(), package);
    }

    /// Install packages
    pub fn install(&mut self, specs: Vec<String>) -> Result<(), DnfError> {
        let mut transaction = Transaction::new();

        // Resolve package specifications
        for spec in specs {
            if let Some(package) = self.database.get(&spec) {
                transaction.operations.push(TransactionOperation::Install {
                    package: package.name.clone(),
                    version: package.version.clone(),
                });

                // Add dependencies
                for dep in &package.dependencies {
                    if !transaction.dependencies.contains(dep) {
                        transaction.dependencies.push(dep.clone());
                    }
                }
            } else {
                return Err(DnfError::PackageNotFound);
            }
        }

        // Execute transaction
        self.execute_transaction(transaction)?;

        Ok(())
    }

    /// Update packages
    pub fn update(&mut self, specs: Vec<String>) -> Result<(), DnfError> {
        let mut transaction = Transaction::new();

        for spec in specs {
            if let Some(package) = self.database.get(&spec) {
                if let Some(current_version) = self.installed.get(&spec) {
                    if current_version != &package.version {
                        transaction.operations.push(TransactionOperation::Update {
                            package: package.name.clone(),
                            from_version: current_version.clone(),
                            to_version: package.version.clone(),
                        });
                    }
                }
            }
        }

        self.execute_transaction(transaction)?;
        Ok(())
    }

    /// Update all packages
    pub fn upgrade(&mut self) -> Result<(), DnfError> {
        let all_packages: Vec<String> = self.installed.keys().cloned().collect();
        self.update(all_packages)
    }

    /// Remove packages
    pub fn remove(&mut self, specs: Vec<String>) -> Result<(), DnfError> {
        let mut transaction = Transaction::new();

        for spec in specs {
            if self.installed.contains_key(&spec) {
                transaction
                    .operations
                    .push(TransactionOperation::Remove { package: spec });
            } else {
                return Err(DnfError::PackageNotInstalled);
            }
        }

        self.execute_transaction(transaction)?;
        Ok(())
    }

    /// Search for packages
    pub fn search(&self, query: &str) -> Vec<&DnfPackage> {
        let query_lower = query.to_lowercase();
        self.database
            .values()
            .filter(|pkg| {
                pkg.name.to_lowercase().contains(&query_lower)
                    || pkg.summary.to_lowercase().contains(&query_lower)
                    || pkg.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Show package information
    pub fn info(&self, name: &str) -> Option<&DnfPackage> {
        self.database.get(name)
    }

    /// List installed packages
    pub fn list_installed(&self) -> Vec<(&String, &String)> {
        self.installed.iter().collect()
    }

    /// Execute transaction
    fn execute_transaction(&mut self, transaction: Transaction) -> Result<(), DnfError> {
        if !transaction.problems.is_empty() {
            return Err(DnfError::TransactionProblems(transaction.problems));
        }

        for operation in transaction.operations {
            match operation {
                TransactionOperation::Install { package, version } => {
                    println!("Installing {} ({})", package, version);
                    self.installed.insert(package, version);
                }
                TransactionOperation::Remove { package } => {
                    println!("Removing {}", package);
                    self.installed.remove(&package);
                }
                TransactionOperation::Update {
                    package,
                    from_version,
                    to_version,
                } => {
                    println!(
                        "Updating {} from {} to {}",
                        package, from_version, to_version
                    );
                    self.installed.insert(package, to_version);
                }
                TransactionOperation::Obsolete { package } => {
                    println!("Obsoleting {}", package);
                    self.installed.remove(&package);
                }
                TransactionOperation::Reinstall { package, version } => {
                    println!("Reinstalling {} ({})", package, version);
                    self.installed.insert(package, version);
                }
            }
        }

        Ok(())
    }

    /// Enable repository
    pub fn enable_repository(&mut self, name: &str) {
        if let Some(repo) = self.repositories.iter_mut().find(|r| r.name == name) {
            repo.enabled = true;
        }
    }

    /// Disable repository
    pub fn disable_repository(&mut self, name: &str) {
        if let Some(repo) = self.repositories.iter_mut().find(|r| r.name == name) {
            repo.enabled = false;
        }
    }
}

impl Default for SigmaDNF {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_dnf_install() {
        let mut dnf = SigmaDNF::new();

        let pkg = DnfPackage {
            name: "example-pkg".to_string(),
            version: "1.0.0".to_string(),
            release: "1".to_string(),
            epoch: None,
            architecture: "x86_64".to_string(),
            summary: "An example package".to_string(),
            description: "An example package for testing".to_string(),
            url: "https://example.com".to_string(),
            license: "MIT".to_string(),
            dependencies: vec![],
            provides: vec![],
            obsoletes: vec![],
            conflicts: vec![],
            requires: vec![],
            size: 1024,
            installed_size: 2048,
            repository: "fedora".to_string(),
        };

        dnf.add_package(pkg);
        let result = dnf.install(vec!["example-pkg".to_string()]);
        assert!(result.is_ok());
        assert!(dnf.installed.contains_key("example-pkg"));
    }

    #[test]
    fn test_dnf_search() {
        let mut dnf = SigmaDNF::new();

        let pkg = DnfPackage {
            name: "example-pkg".to_string(),
            version: "1.0.0".to_string(),
            release: "1".to_string(),
            epoch: None,
            architecture: "x86_64".to_string(),
            summary: "An example package".to_string(),
            description: "An example package for testing".to_string(),
            url: "https://example.com".to_string(),
            license: "MIT".to_string(),
            dependencies: vec![],
            provides: vec![],
            obsoletes: vec![],
            conflicts: vec![],
            requires: vec![],
            size: 1024,
            installed_size: 2048,
            repository: "fedora".to_string(),
        };

        dnf.add_package(pkg);
        let results = dnf.search("example");
        assert_eq!(results.len(), 1);
    }
}
