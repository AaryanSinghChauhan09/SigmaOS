// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// tools/sigma_sbom.rs — SBOM Generation Pipeline
//
// Implements Software Bill of Materials (SBOM) generation for SigmaOS packages.
// Inspired by: SPDX, CycloneDX, NixOS SBOM generation
// Language: Rust (std available for userland tools)

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

// ── Types ─────────────────────────────────────────────────────────────────────
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ParseError(String),
    CommandError(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

// ── SBOM Format ───────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub enum SbomFormat {
    /// SPDX 2.3 format
    Spdx,
    /// CycloneDX 1.4 format
    CycloneDx,
    /// JSON format
    Json,
}

// ── Component Type ───────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentType {
    Application,
    Library,
    Framework,
    Container,
    Firmware,
    File,
    Other(String),
}

// ── License Information ───────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct License {
    pub id: String,
    pub name: String,
    pub expression: String,
}

// ── Hash Information ─────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct Hash {
    pub algorithm: String, // SHA-256, SHA-512, etc.
    pub value: String,
}

// ── External Reference ───────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct ExternalReference {
    pub category: String, // SECURITY, PACKAGE-MANAGER, etc.
    pub url: String,
    pub comment: Option<String>,
}

// ── SBOM Component ───────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct Component {
    pub name: String,
    pub version: String,
    pub component_type: ComponentType,
    pub licenses: Vec<License>,
    pub hashes: Vec<Hash>,
    pub purl: Option<String>, // Package URL
    pub cpe: Option<String>, // Common Platform Enumeration
    pub description: Option<String>,
    pub external_references: Vec<ExternalReference>,
    pub supplier: Option<String>,
    pub author: Option<String>,
}

// ── SBOM Document ────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct SbomDocument {
    pub name: String,
    pub version: String,
    pub format: SbomFormat,
    pub components: Vec<Component>,
    pub dependencies: HashMap<String, Vec<String>>,
    pub creation_date: String,
    pub tools: Vec<String>,
}

// ── SBOM Generator ───────────────────────────────────────────────────────────
pub struct SbomGenerator {
    pub format: SbomFormat,
    pub include_hashes: bool,
    pub include_dependencies: bool,
    pub output_path: PathBuf,
}

impl SbomGenerator {
    pub fn new(format: SbomFormat, output_path: PathBuf) -> Self {
        Self {
            format,
            include_hashes: true,
            include_dependencies: true,
            output_path,
        }
    }

    // ── Public API ────────────────────────────────────────────────────────────

    /// Generate SBOM from Cargo.toml (Rust project)
    pub fn from_cargo(&self, project_path: &Path) -> Result<SbomDocument> {
        let cargo_toml = project_path.join("Cargo.toml");
        if !cargo_toml.exists() {
            return Err(Error::ParseError("Cargo.toml not found".to_string()));
        }

        let mut components = Vec::new();
        let mut dependencies = HashMap::new();

        // Parse Cargo.toml
        let file = File::open(&cargo_toml)?;
        let reader = BufReader::new(file);
        let mut current_section = String::new();

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();

            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                current_section = trimmed[1..trimmed.len()-1].to_string();
            } else if let Some((key, value)) = trimmed.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"');

                if current_section == "package" {
                    match key {
                        "name" => {
                            // Main package component
                            let component = Component {
                                name: value.to_string(),
                                version: String::new(),
                                component_type: ComponentType::Application,
                                licenses: Vec::new(),
                                hashes: Vec::new(),
                                purl: None,
                                cpe: None,
                                description: None,
                                external_references: Vec::new(),
                                supplier: None,
                                author: None,
                            };
                            components.push(component);
                        }
                        "version" => {
                            if let Some(comp) = components.last_mut() {
                                comp.version = value.to_string();
                            }
                        }
                        _ => {}
                    }
                } else if current_section == "dependencies" || current_section.starts_with("dependencies.") {
                    // Dependency
                    dependencies.insert(key.to_string(), vec![value.to_string()]);
                }
            }
        }

        // Add dependencies as components
        for (name, version) in &dependencies {
            let component = Component {
                name: name.clone(),
                version: version.first().unwrap_or(&String::new()).clone(),
                component_type: ComponentType::Library,
                licenses: Vec::new(),
                hashes: Vec::new(),
                purl: Some(format!("pkg:cargo/{}@{}", name, version.first().unwrap_or(&String::new()))),
                cpe: None,
                description: None,
                external_references: Vec::new(),
                supplier: None,
                author: None,
            };
            components.push(component);
        }

        Ok(SbomDocument {
            name: project_path.file_name().unwrap().to_string_lossy().to_string(),
            version: "1.0.0".to_string(),
            format: self.format.clone(),
            components,
            dependencies,
            creation_date: chrono::Utc::now().to_rfc3339(),
            tools: vec!["sigma-sbom 1.0.0".to_string()],
        })
    }

    /// Generate SBOM from package directory
    pub fn from_directory(&self, dir_path: &Path) -> Result<SbomDocument> {
        let mut components = Vec::new();

        // Scan directory for files
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let component = Component {
                    name: path.file_name().unwrap().to_string_lossy().to_string(),
                    version: String::new(),
                    component_type: ComponentType::File,
                    licenses: Vec::new(),
                    hashes: if self.include_hashes {
                        vec![self.compute_file_hash(&path)?]
                    } else {
                        Vec::new()
                    },
                    purl: None,
                    cpe: None,
                    description: None,
                    external_references: Vec::new(),
                    supplier: None,
                    author: None,
                };
                components.push(component);
            }
        }

        Ok(SbomDocument {
            name: dir_path.file_name().unwrap().to_string_lossy().to_string(),
            version: "1.0.0".to_string(),
            format: self.format.clone(),
            components,
            dependencies: HashMap::new(),
            creation_date: chrono::Utc::now().to_rfc3339(),
            tools: vec!["sigma-sbom 1.0.0".to_string()],
        })
    }

    /// Compute file hash
    fn compute_file_hash(&self, path: &Path) -> Result<Hash> {
        let output = Command::new("sha256sum")
            .arg(path)
            .output()
            .map_err(|e| Error::CommandError(e.to_string()))?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = output_str.split_whitespace().collect();

        Ok(Hash {
            algorithm: "SHA-256".to_string(),
            value: parts.first().unwrap_or(&"").to_string(),
        })
    }

    /// Export SBOM to file
    pub fn export(&self, sbom: &SbomDocument) -> Result<()> {
        let content = match self.format {
            SbomFormat::Spdx => self.export_spdx(sbom),
            SbomFormat::CycloneDx => self.export_cyclonedx(sbom),
            SbomFormat::Json => self.export_json(sbom),
        };

        let mut file = File::create(&self.output_path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    // ── Export Formats ────────────────────────────────────────────────────────

    fn export_spdx(&self, sbom: &SbomDocument) -> String {
        let mut output = String::new();
        
        output.push_str("SPDXVersion: SPDX-2.3\n");
        output.push_str(&format!("DataLicense: CC0-1.0\n"));
        output.push_str(&format!("SPDXID: SPDXRef-DOCUMENT\n"));
        output.push_str(&format!("DocumentName: {}\n", sbom.name));
        output.push_str(&format!("DocumentNamespace: https://sigmaos.org/sbom/{}\n", sbom.name));
        output.push_str(&format!("Created: {}\n", sbom.creation_date));
        output.push_str("\n");

        for tool in &sbom.tools {
            output.push_str(&format!("Creator: Tool: {}\n", tool));
        }
        output.push_str("\n");

        for component in &sbom.components {
            output.push_str(&format!("PackageName: {}\n", component.name));
            output.push_str(&format!("SPDXID: SPDXRef-{}\n", component.name.replace('-', "_")));
            output.push_str(&format!("PackageVersion: {}\n", component.version));
            output.push_str(&format!("PackageDownloadLocation: NOASSERTION\n"));
            output.push_str(&format!("FilesAnalyzed: false\n"));
            output.push_str(&format!("PackageVerificationCode: NOASSERTION\n"));
            output.push_str("PackageLicenseConcluded: NOASSERTION\n");
            output.push_str("PackageLicenseDeclared: NOASSERTION\n");
            output.push_str("PackageCopyrightText: NOASSERTION\n");
            output.push_str("\n");
        }

        output
    }

    fn export_cyclonedx(&self, sbom: &SbomDocument) -> String {
        // Simplified CycloneDX JSON output
        let mut output = String::new();
        
        output.push_str("{\n");
        output.push_str("  \"bomFormat\": \"CycloneDX\",\n");
        output.push_str("  \"specVersion\": \"1.4\",\n");
        output.push_str(&format!("  \"metadata\": {{\n    \"component\": {{\n      \"name\": \"{}\",\n      \"version\": \"{}\",\n      \"type\": \"application\"\n    }}\n  }},\n", sbom.name, sbom.version));
        output.push_str("  \"components\": [\n");

        for (i, component) in sbom.components.iter().enumerate() {
            if i > 0 {
                output.push_str(",\n");
            }
            output.push_str("    {\n");
            output.push_str(&format!("      \"name\": \"{}\",\n", component.name));
            output.push_str(&format!("      \"version\": \"{}\",\n", component.version));
            output.push_str(&format!("      \"type\": \"{}\"\n", self.component_type_to_string(&component.component_type)));
            output.push_str("    }");
        }

        output.push_str("\n  ]\n");
        output.push_str("}\n");

        output
    }

    fn export_json(&self, sbom: &SbomDocument) -> String {
        // Simple JSON output
        format!(
            r#"{{
  "name": "{}",
  "version": "{}",
  "format": "{:?}",
  "creationDate": "{}",
  "components": [
{}
  ]
}}"#,
            sbom.name,
            sbom.version,
            sbom.format,
            sbom.creation_date,
            sbom.components.iter()
                .map(|c| format!(
                    r#"    {{
      "name": "{}",
      "version": "{}",
      "type": "{:?}"
    }}"#,
                    c.name, c.version, c.component_type
                ))
                .collect::<Vec<_>>()
                .join(",\n")
        )
    }

    fn component_type_to_string(&self, ct: &ComponentType) -> &str {
        match ct {
            ComponentType::Application => "application",
            ComponentType::Library => "library",
            ComponentType::Framework => "framework",
            ComponentType::Container => "container",
            ComponentType::Firmware => "firmware",
            ComponentType::File => "file",
            ComponentType::Other(s) => s,
        }
    }
}

// ── CLI Interface ─────────────────────────────────────────────────────────────
pub fn run_sbom_generator(args: Vec<String>) -> Result<()> {
    if args.len() < 3 {
        eprintln!("Usage: sigma-sbom <format> <path> [output]");
        eprintln!("Formats: spdx, cyclonedx, json");
        std::process::exit(1);
    }

    let format = match args[1].as_str() {
        "spdx" => SbomFormat::Spdx,
        "cyclonedx" => SbomFormat::CycloneDx,
        "json" => SbomFormat::Json,
        _ => {
            eprintln!("Unknown format: {}", args[1]);
            std::process::exit(1);
        }
    };

    let path = Path::new(&args[2]);
    let output_path = if args.len() > 3 {
        PathBuf::from(&args[3])
    } else {
        path.join("sbom.txt")
    };

    let generator = SbomGenerator::new(format, output_path);

    let sbom = if path.join("Cargo.toml").exists() {
        generator.from_cargo(path)?
    } else if path.is_dir() {
        generator.from_directory(path)?
    } else {
        return Err(Error::ParseError("Invalid path".to_string()));
    };

    generator.export(&sbom)?;
    println!("SBOM generated successfully: {}", generator.output_path.display());

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Err(e) = run_sbom_generator(args) {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}
