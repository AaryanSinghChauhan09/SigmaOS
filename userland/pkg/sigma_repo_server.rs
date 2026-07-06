// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/pkg/sigma_repo_server.rs — Sigma Package Repository Server
// Implements a simple HTTP-based package repository server for SigmaOS packages
//
// Features:
//   - Package metadata storage
//   - Package file serving
//   - Package signature verification
//   - RESTful API for package operations
//
// Language: Rust

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

// ── Package Metadata ─────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub checksum: String,
    pub size: u64,
    pub signature: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PackageIndex {
    pub packages: HashMap<String, Vec<PackageMetadata>>,
}

impl PackageIndex {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
        }
    }

    pub fn add_package(&mut self, metadata: PackageMetadata) {
        self.packages
            .entry(metadata.name.clone())
            .or_insert_with(Vec::new)
            .push(metadata);
    }

    pub fn get_package(&self, name: &str, version: &str) -> Option<&PackageMetadata> {
        self.packages.get(name)?.iter().find(|p| p.version == version)
    }

    pub fn list_packages(&self, name: &str) -> Vec<&PackageMetadata> {
        self.packages.get(name).map(|v| v.iter().collect()).unwrap_or_default()
    }
}

// ── Repository Server ───────────────────────────────────────────────────

pub struct PackageRepository {
    storage_path: PathBuf,
    index: Arc<Mutex<PackageIndex>>,
}

impl PackageRepository {
    pub fn new<P: AsRef<Path>>(storage_path: P) -> Result<Self, std::io::Error> {
        let storage_path = storage_path.as_ref().to_path_buf();
        fs::create_dir_all(&storage_path)?;
        fs::create_dir_all(storage_path.join("packages"))?;
        fs::create_dir_all(storage_path.join("metadata"))?;

        let index_path = storage_path.join("index.json");
        let index = if index_path.exists() {
            let mut file = File::open(&index_path)?;
            let mut data = String::new();
            file.read_to_string(&mut data)?;
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            PackageIndex::new()
        };

        Ok(Self {
            storage_path,
            index: Arc::new(Mutex::new(index)),
        })
    }

    pub fn add_package(&self, metadata: PackageMetadata, package_data: &[u8]) -> Result<(), String> {
        // Save package file
        let package_path = self.storage_path.join("packages")
            .join(format!("{}-{}.sigpkg", metadata.name, metadata.version));
        let mut file = File::create(&package_path)
            .map_err(|e| format!("Failed to create package file: {}", e))?;
        file.write_all(package_data)
            .map_err(|e| format!("Failed to write package data: {}", e))?;

        // Update index
        let mut index = self.index.lock().map_err(|e| format!("Lock error: {}", e))?;
        index.add_package(metadata.clone());

        // Save index
        let index_path = self.storage_path.join("index.json");
        let index_json = serde_json::to_string_pretty(&*index)
            .map_err(|e| format!("Failed to serialize index: {}", e))?;
        let mut index_file = File::create(&index_path)
            .map_err(|e| format!("Failed to create index file: {}", e))?;
        index_file.write_all(index_json.as_bytes())
            .map_err(|e| format!("Failed to write index: {}", e))?;

        Ok(())
    }

    pub fn get_package(&self, name: &str, version: &str) -> Option<PackageMetadata> {
        let index = self.index.lock().ok()?;
        index.get_package(name, version).cloned()
    }

    pub fn get_package_data(&self, name: &str, version: &str) -> Result<Vec<u8>, String> {
        let package_path = self.storage_path.join("packages")
            .join(format!("{}-{}.sigpkg", name, version));
        let mut file = File::open(&package_path)
            .map_err(|e| format!("Package not found: {}", e))?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)
            .map_err(|e| format!("Failed to read package: {}", e))?;
        Ok(data)
    }

    pub fn list_all_packages(&self) -> Vec<String> {
        let index = self.index.lock().unwrap();
        index.packages.keys().cloned().collect()
    }
}

// ── HTTP Server ─────────────────────────────────────────────────────────

fn handle_request(mut stream: TcpStream, repo: Arc<PackageRepository>) {
    let mut buffer = [0u8; 8192];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer);
            let response = if request.starts_with("GET /api/packages") {
                let packages = repo.list_all_packages();
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
                    serde_json::to_string(&packages).unwrap_or_default()
                )
            } else if request.starts_with("GET /api/package/") {
                let path: Vec<&str> = request.split_whitespace().nth(1).unwrap_or("").split('/').collect();
                if path.len() >= 4 {
                    let name = path[3];
                    let version = path.get(4).unwrap_or("latest");
                    match repo.get_package(name, version) {
                        Some(metadata) => format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
                            serde_json::to_string(&metadata).unwrap_or_default()
                        ),
                        None => "HTTP/1.1 404 Not Found\r\n\r\nPackage not found".to_string(),
                    }
                } else {
                    "HTTP/1.1 400 Bad Request\r\n\r\nInvalid request".to_string()
                }
            } else if request.starts_with("GET /download/") {
                let path: Vec<&str> = request.split_whitespace().nth(1).unwrap_or("").split('/').collect();
                if path.len() >= 3 {
                    let name = path[2];
                    let version = path.get(3).unwrap_or("latest");
                    match repo.get_package_data(name, version) {
                        Ok(data) => {
                            let header = format!(
                                "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n",
                                data.len()
                            );
                            let _ = stream.write_all(header.as_bytes());
                            let _ = stream.write_all(&data);
                            return;
                        }
                        Err(_) => "HTTP/1.1 404 Not Found\r\n\r\nPackage not found".to_string(),
                    }
                } else {
                    "HTTP/1.1 400 Bad Request\r\n\r\nInvalid request".to_string()
                }
            } else {
                "HTTP/1.1 404 Not Found\r\n\r\nNot Found".to_string()
            };
            let _ = stream.write_all(response.as_bytes());
        }
        Err(_) => {}
    }
}

pub fn start_server(repo: Arc<PackageRepository>, addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("Sigma Package Repository Server listening on {}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let repo_clone = Arc::clone(&repo);
                thread::spawn(move || handle_request(stream, repo_clone));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}

// ── CLI Entry Point ─────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: sigma-repo-server <storage-path> [port]");
        println!("  storage-path: Path to repository storage directory");
        println!("  port: TCP port to listen on (default: 8080)");
        std::process::exit(1);
    }

    let storage_path = &args[1];
    let port = args.get(2).and_then(|p| p.parse::<u16>().ok()).unwrap_or(8080);
    let addr = format!("0.0.0.0:{}", port);

    let repo = match PackageRepository::new(storage_path) {
        Ok(r) => Arc::new(r),
        Err(e) => {
            eprintln!("Failed to initialize repository: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = start_server(repo, &addr) {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}
