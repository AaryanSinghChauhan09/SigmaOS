// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// tools/sigma_repo_sign.rs — Signed Package Repository Tooling
//
// Implements GPG signing and verification for SigmaOS package repositories.
// Inspired by: Fedora RPM signing, Debian apt signing, NixOS signing
// Language: Rust (std available for userland tools)

use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

// ── Types ─────────────────────────────────────────────────────────────────────
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    CommandError(String),
    SignatureError(String),
    KeyError(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

// ── Signature Algorithm ───────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SignatureAlgorithm {
    /// Ed25519 (default for SigmaOS)
    Ed25519,
    /// RSA 4096
    Rsa4096,
    /// ECDSA P-384
    EcdsaP384,
}

// ── Key Information ───────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct KeyInfo {
    pub key_id: String,
    pub key_type: SignatureAlgorithm,
    pub fingerprint: String,
    pub created: String,
    pub expires: Option<String>,
}

// ── Signature Information ─────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct Signature {
    pub file_path: PathBuf,
    pub signature_path: PathBuf,
    pub key_id: String,
    pub algorithm: SignatureAlgorithm,
    pub timestamp: String,
    pub valid: bool,
}

// ── Repository Metadata ───────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct RepositoryMetadata {
    pub name: String,
    pub url: String,
    pub key_id: String,
    pub last_update: String,
    pub packages: Vec<PackageInfo>,
}

#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub hash: String,
    pub signature: String,
}

// ── Repository Manager ───────────────────────────────────────────────────────
pub struct RepositoryManager {
    pub repo_path: PathBuf,
    pub keyring_path: PathBuf,
    pub gpg_home: PathBuf,
}

impl RepositoryManager {
    pub fn new(repo_path: PathBuf, keyring_path: PathBuf) -> Self {
        let gpg_home = keyring_path.join(".gnupg");
        Self {
            repo_path,
            keyring_path,
            gpg_home,
        }
    }

    // ── Key Management ────────────────────────────────────────────────────────

    /// Generate a new signing key
    pub fn generate_key(&self, key_type: SignatureAlgorithm, name: &str, email: &str) -> Result<KeyInfo> {
        let algo_str = match key_type {
            SignatureAlgorithm::Ed25519 => "ed25519",
            SignatureAlgorithm::Rsa4096 => "rsa4096",
            SignatureAlgorithm::EcdsaP384 => "ecdsa-p384",
        };

        let output = Command::new("gpg")
            .arg("--batch")
            .arg("--homedir")
            .arg(&self.gpg_home)
            .arg("--passphrase")
            .arg("")
            .arg("--quick-generate-key")
            .arg(&format!("{} <{}>", name, email))
            .arg(algo_str)
            .arg("default")
            .arg("0")
            .output()
            .map_err(|e| Error::CommandError(e.to_string()))?;

        if !output.status.success() {
            return Err(Error::KeyError(String::from_utf8_lossy(&output.stderr).to_string()));
        }

        // Extract key ID
        let key_id = self.list_keys()?.first()
            .ok_or_else(|| Error::KeyError("No key generated".to_string()))?
            .clone();

        Ok(KeyInfo {
            key_id: key_id.clone(),
            key_type,
            fingerprint: key_id.clone(),
            created: chrono::Utc::now().to_rfc3339(),
            expires: None,
        })
    }

    /// List all keys in keyring
    pub fn list_keys(&self) -> Result<Vec<String>> {
        let output = Command::new("gpg")
            .arg("--homedir")
            .arg(&self.gpg_home)
            .arg("--list-keys")
            .arg("--with-colons")
            .output()
            .map_err(|e| Error::CommandError(e.to_string()))?;

        let mut keys = Vec::new();
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() > 4 && parts[0] == "pub" {
                keys.push(parts[4].to_string());
            }
        }

        Ok(keys)
    }

    /// Export public key
    pub fn export_public_key(&self, key_id: &str, output_path: &Path) -> Result<()> {
        let output = Command::new("gpg")
            .arg("--homedir")
            .arg(&self.gpg_home)
            .arg("--armor")
            .arg("--export")
            .arg(key_id)
            .output()
            .map_err(|e| Error::CommandError(e.to_string()))?;

        let mut file = File::create(output_path)?;
        file.write_all(&output.stdout)?;
        Ok(())
    }

    /// Import public key
    pub fn import_public_key(&self, key_path: &Path) -> Result<String> {
        let output = Command::new("gpg")
            .arg("--homedir")
            .arg(&self.gpg_home)
            .arg("--import")
            .arg(key_path)
            .output()
            .map_err(|e| Error::CommandError(e.to_string()))?;

        if !output.status.success() {
            return Err(Error::KeyError(String::from_utf8_lossy(&output.stderr).to_string()));
        }

        // Extract imported key ID
        let keys = self.list_keys()?;
        keys.last().cloned().ok_or_else(|| Error::KeyError("No key imported".to_string()))
    }

    // ── Signing Operations ────────────────────────────────────────────────────

    /// Sign a file
    pub fn sign_file(&self, file_path: &Path, key_id: &str) -> Result<Signature> {
        let sig_path = file_path.with_extension(file_path.extension().unwrap_or(&std::ffi::OsStr::new("sig")).to_str().unwrap().to_string() + ".asc");

        let output = Command::new("gpg")
            .arg("--homedir")
            .arg(&self.gpg_home)
            .arg("--batch")
            .arg("--yes")
            .arg("--detach-sign")
            .arg("--default-key")
            .arg(key_id)
            .arg("--armor")
            .arg("--output")
            .arg(&sig_path)
            .arg(file_path)
            .output()
            .map_err(|e| Error::CommandError(e.to_string()))?;

        if !output.status.success() {
            return Err(Error::SignatureError(String::from_utf8_lossy(&output.stderr).to_string()));
        }

        Ok(Signature {
            file_path: file_path.to_path_buf(),
            signature_path: sig_path,
            key_id: key_id.to_string(),
            algorithm: SignatureAlgorithm::Ed25519,
            timestamp: chrono::Utc::now().to_rfc3339(),
            valid: true,
        })
    }

    /// Sign a package
    pub fn sign_package(&self, package_path: &Path, key_id: &str) -> Result<Signature> {
        self.sign_file(package_path, key_id)
    }

    /// Sign repository metadata
    pub fn sign_metadata(&self, metadata_path: &Path, key_id: &str) -> Result<Signature> {
        self.sign_file(metadata_path, key_id)
    }

    // ── Verification Operations ────────────────────────────────────────────────

    /// Verify a file signature
    pub fn verify_file(&self, file_path: &Path, signature_path: &Path) -> Result<bool> {
        let output = Command::new("gpg")
            .arg("--homedir")
            .arg(&self.gpg_home)
            .arg("--verify")
            .arg(signature_path)
            .arg(file_path)
            .output()
            .map_err(|e| Error::CommandError(e.to_string()))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Check for "Good signature" in output
        let is_valid = stdout.contains("Good signature") || stderr.contains("Good signature");
        Ok(is_valid)
    }

    /// Verify a package
    pub fn verify_package(&self, package_path: &Path) -> Result<bool> {
        let sig_path = package_path.with_extension(package_path.extension().unwrap_or(&std::ffi::OsStr::new("sig")).to_str().unwrap().to_string() + ".asc");
        self.verify_file(package_path, &sig_path)
    }

    /// Verify repository metadata
    pub fn verify_metadata(&self, metadata_path: &Path) -> Result<bool> {
        let sig_path = metadata_path.with_extension(metadata_path.extension().unwrap_or(&std::ffi::OsStr::new("json")).to_str().unwrap().to_string() + ".asc");
        self.verify_file(metadata_path, &sig_path)
    }

    // ── Repository Operations ─────────────────────────────────────────────────

    /// Create repository metadata
    pub fn create_metadata(&self, packages: &[PathBuf], key_id: &str) -> Result<RepositoryMetadata> {
        let mut package_infos = Vec::new();

        for pkg_path in packages {
            if pkg_path.exists() {
                let hash = self.compute_file_hash(pkg_path)?;
                let name = pkg_path.file_name().unwrap().to_string_lossy().to_string();
                let version = self.extract_version(&name);

                // Sign the package
                let sig = self.sign_package(pkg_path, key_id)?;

                package_infos.push(PackageInfo {
                    name,
                    version,
                    hash,
                    signature: sig.signature_path.to_string_lossy().to_string(),
                });
            }
        }

        let metadata = RepositoryMetadata {
            name: "sigmaos-main".to_string(),
            url: "https://repo.sigmaos.org".to_string(),
            key_id: key_id.to_string(),
            last_update: chrono::Utc::now().to_rfc3339(),
            packages: package_infos,
        };

        // Save metadata
        let metadata_path = self.repo_path.join("metadata.json");
        let metadata_json = serde_json::to_string_pretty(&metadata)?;
        fs::write(&metadata_path, metadata_json)?;

        // Sign metadata
        self.sign_metadata(&metadata_path, key_id)?;

        Ok(metadata)
    }

    /// Load repository metadata
    pub fn load_metadata(&self) -> Result<RepositoryMetadata> {
        let metadata_path = self.repo_path.join("metadata.json");
        let content = fs::read_to_string(&metadata_path)?;
        let metadata: RepositoryMetadata = serde_json::from_str(&content)?;
        Ok(metadata)
    }

    /// Verify repository
    pub fn verify_repository(&self) -> Result<bool> {
        let metadata = self.load_metadata()?;
        
        // Verify metadata signature
        let metadata_path = self.repo_path.join("metadata.json");
        if !self.verify_metadata(&metadata_path)? {
            return Ok(false);
        }

        // Verify all packages
        for pkg in &metadata.packages {
            let pkg_path = self.repo_path.join(&pkg.name);
            if pkg_path.exists() {
                if !self.verify_package(&pkg_path)? {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    // ── Helper Functions ─────────────────────────────────────────────────────

    fn compute_file_hash(&self, path: &Path) -> Result<String> {
        let output = Command::new("sha256sum")
            .arg(path)
            .output()
            .map_err(|e| Error::CommandError(e.to_string()))?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = output_str.split_whitespace().collect();
        Ok(parts.first().unwrap_or(&"").to_string())
    }

    fn extract_version(&self, filename: &str) -> String {
        // Simple version extraction from filename
        if let Some(idx) = filename.find('-') {
            filename[idx+1..].split('.').take(3).collect::<Vec<_>>().join(".")
        } else {
            "0.0.0".to_string()
        }
    }
}

// ── CLI Interface ─────────────────────────────────────────────────────────────
pub fn run_repo_sign(args: Vec<String>) -> Result<()> {
    if args.len() < 2 {
        eprintln!("Usage: sigma-repo-sign <command> [args]");
        eprintln!("Commands: gen-key, sign, verify, create-metadata, verify-repo");
        std::process::exit(1);
    }

    let repo_path = PathBuf::from("repo");
    let keyring_path = PathBuf::from(".keyring");
    let manager = RepositoryManager::new(repo_path, keyring_path);

    match args[1].as_str() {
        "gen-key" => {
            if args.len() < 5 {
                eprintln!("Usage: sigma-repo-sign gen-key <name> <email> [algorithm]");
                std::process::exit(1);
            }
            let algo = if args.len() > 4 {
                match args[4].as_str() {
                    "rsa" => SignatureAlgorithm::Rsa4096,
                    "ecdsa" => SignatureAlgorithm::EcdsaP384,
                    _ => SignatureAlgorithm::Ed25519,
                }
            } else {
                SignatureAlgorithm::Ed25519
            };
            let key = manager.generate_key(algo, &args[2], &args[3])?;
            println!("Generated key: {}", key.key_id);
        }
        "sign" => {
            if args.len() < 4 {
                eprintln!("Usage: sigma-repo-sign sign <file> <key-id>");
                std::process::exit(1);
            }
            let sig = manager.sign_file(Path::new(&args[2]), &args[3])?;
            println!("Signed: {} -> {}", sig.file_path.display(), sig.signature_path.display());
        }
        "verify" => {
            if args.len() < 3 {
                eprintln!("Usage: sigma-repo-sign verify <file>");
                std::process::exit(1);
            }
            let valid = manager.verify_file(Path::new(&args[2]), &Path::new(&format!("{}.asc", args[2])))?;
            println!("Signature valid: {}", valid);
        }
        "create-metadata" => {
            if args.len() < 3 {
                eprintln!("Usage: sigma-repo-sign create-metadata <key-id>");
                std::process::exit(1);
            }
            let packages = vec![PathBuf::from("package1.sigpkg"), PathBuf::from("package2.sigpkg")];
            let metadata = manager.create_metadata(&packages, &args[2])?;
            println!("Created metadata with {} packages", metadata.packages.len());
        }
        "verify-repo" => {
            let valid = manager.verify_repository()?;
            println!("Repository valid: {}", valid);
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Err(e) = run_repo_sign(args) {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}
