# SigmaOS Reproducible Builds Specification

## Overview

SigmaOS enforces 100% deterministic builds across its entire package ecosystem. By neutralizing variables like timestamps, compile paths, and filesystem order during compilation, we guarantee that compiling a specific Git commit results in bit-for-bit identical binaries. Each package contains cryptographically signed Software Bills of Materials (SBOMs) to verify package provenance.

### Key Principles

- **Deterministic**: Same source + same environment = identical binary
- **Bit-for-Bit Verification**: Verify builds match expected outputs
- **SBOM Generation**: Generate Software Bill of Materials for all packages
- **Build Isolation**: Use controlled build environments
- **Transparency**: Make build process auditable and verifiable
- **Cross-Verification**: Multiple independent builds for verification

## Deterministic Pipeline Flow

```
 [Source Code (Git Commit)] ──► [Neutralize Timestamps & Paths]
                                         │
                                         ▼
 [Isolated Container Sandbox] ──► [Deterministic Compilation]
                                         │
                                         ▼
 [Cryptographic Signature] ◄──► [Bit-for-Bit Parity Check]
         │
         ▼
 [Staged Repository Package + SBOM]
```

## Build Environment

### Container Isolation

**Base Image**:
```dockerfile
FROM sigmaos/build-base:6.1.0

# Pin toolchain versions
ENV GCC_VERSION=12.2.0
ENV BINUTILS_VERSION=2.40
ENV KERNEL_HEADERS=6.1.0

# Set deterministic environment
ENV LANG=C.UTF-8
ENV LC_ALL=C.UTF-8
ENV SOURCE_DATE_EPOCH=1704067200
ENV TZ=UTC

# Clear build environment
ENV PATH=/usr/bin:/bin
ENV HOME=/build
```

### Build Configuration

**File**: `build.toml`

```toml
[build]
reproducible = true
env_clear = ["PATH", "LANG", "TZ", "USER", "HOME"]
timezone = "UTC"
timestamp = 1704067200 # Fixed epoch timestamp for build determinism
locale = "C.UTF-8"

[toolchain]
gcc = "12.2.0"
binutils = "2.40"
kernel_headers = "6.1.0"

[source]
git_commit = "abc123..."
source_hash = "sha256:..."
patches = ["patch1.patch", "patch2.patch"]

[compilation]
optimization = "O2"
debug_symbols = false
strip = true
deterministic_flags = true

[sbom]
format = "SPDX"
hash_algorithm = "sha256"
include_sources = true
sign_sbom = true
```

## Technical Implementation

### Binary Verification

```rust
// tools/sigma_iso_builder.rs
use std::fs;
use std::path::Path;
use sha2::{Sha256, Digest};

pub fn verify_binary_determinism(path_a: &Path, path_b: &Path) -> Result<bool, io::Error> {
    let bytes_a = fs::read(path_a)?;
    let bytes_b = fs::read(path_b)?;
    
    if bytes_a.len() != bytes_b.len() {
        return Ok(false);
    }
    
    Ok(bytes_a == bytes_b)
}

pub fn compute_file_hash(path: &Path) -> Result<String, io::Error> {
    let data = fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn compare_builds(build_a: &Path, build_b: &Path) -> Result<BuildComparison, BuildError> {
    let files_a = collect_files(build_a)?;
    let files_b = collect_files(build_b)?;
    
    let mut comparison = BuildComparison::new();
    
    for file in files_a.keys() {
        if let Some(hash_a) = files_a.get(file) {
            if let Some(hash_b) = files_b.get(file) {
                if hash_a != hash_b {
                    comparison.add_mismatch(file.clone());
                }
            } else {
                comparison.add_missing_in_b(file.clone());
            }
        }
    }
    
    for file in files_b.keys() {
        if !files_a.contains_key(file) {
            comparison.add_missing_in_a(file.clone());
        }
    }
    
    Ok(comparison)
}
```

### Deterministic Compilation

```rust
// tools/sigpkg-build/src/compiler.rs
use std::process::Command;

pub struct DeterministicCompiler {
    source_date_epoch: u64,
    locale: String,
    timezone: String,
}

impl DeterministicCompiler {
    pub fn new() -> Self {
        DeterministicCompiler {
            source_date_epoch: 1704067200,
            locale: "C.UTF-8".to_string(),
            timezone: "UTC".to_string(),
        }
    }
    
    pub fn compile(&self, source: &Path, output: &Path) -> Result<(), BuildError> {
        let mut cmd = Command::new("gcc");
        
        // Set deterministic flags
        cmd.env("SOURCE_DATE_EPOCH", self.source_date_epoch.to_string());
        cmd.env("LANG", &self.locale);
        cmd.env("LC_ALL", &self.locale);
        cmd.env("TZ", &self.timezone);
        
        // Deterministic compilation flags
        cmd.arg("-O2")
           .arg("-fno-strict-aliasing")
           .arg("-fno-common")
           .arg("-fno-delete-null-pointer-checks")
           .arg("-fno-stack-protector-strong")
           .arg("-Wl,--no-as-needed")
           .arg("-Wl,--build-id=sha1")
           .arg("-ffile-prefix-map=./=")
           .arg("-g0") // No debug symbols
           .arg("-s");  // Strip
        
        cmd.arg("-o").arg(output);
        cmd.arg(source);
        
        let output = cmd.output()?;
        
        if !output.status.success() {
            return Err(BuildError::CompilationFailed(String::from_utf8_lossy(&output.stderr).to_string()));
        }
        
        Ok(())
    }
}
```

### SBOM Generation

```rust
// tools/sigpkg-build/src/sbom.rs
use serde_json::json;

pub struct SBOMGenerator {
    format: SBOMFormat,
    hash_algorithm: String,
}

impl SBOMGenerator {
    pub fn new(format: SBOMFormat) -> Self {
        SBOMGenerator {
            format,
            hash_algorithm: "sha256".to_string(),
        }
    }
    
    pub fn generate(&self, package: &Package) -> Result<String, SBOMError> {
        let sbom = match self.format {
            SBOMFormat::SPDX => self.generate_spdx(package)?,
            SBOMFormat::CycloneDX => self.generate_cyclonedx(package)?,
        };
        
        Ok(serde_json::to_string_pretty(&sbom)?)
    }
    
    fn generate_spdx(&self, package: &Package) -> Result<serde_json::Value, SBOMError> {
        let sbom = json!({
            "SPDXID": "SPDXRef-DOCUMENT",
            "spdxVersion": "SPDX-2.3",
            "name": format!("{}-{}", package.name, package.version),
            "documentNamespace": format!("https://sigmaos.org/sbom/{}-{}", package.name, package.version),
            "creationInfo": {
                "created": chrono::Utc::now().to_rfc3339(),
                "creators": ["Tool: sigpkg-sbom-generator-1.0"]
            },
            "packages": [{
                "SPDXID": format!("SPDXRef-Package-{}", package.name),
                "name": package.name,
                "versionInfo": package.version,
                "downloadLocation": package.source_url,
                "filesAnalyzed": false,
                "licenseConcluded": package.license,
                "externalRefs": [{
                    "referenceCategory": "PACKAGE-MANAGER",
                    "referenceLocator": format!("pkg:sigmaos/{}@{}", package.name, package.version),
                    "referenceType": "purl"
                }]
            }]
        });
        
        Ok(sbom)
    }
}
```

## Build Farm Architecture

### Build Farm Components

```
┌─────────────────┐
│  Build Queue    │
│  (RabbitMQ)     │
└────────┬────────┘
         │
         ├─────────────────────────────────┐
         │                                 │
┌────────▼────────┐              ┌────────▼────────┐
│  Build Worker 1 │              │  Build Worker 2 │
│  (Container)    │              │  (Container)    │
└────────┬────────┘              └────────┬────────┘
         │                                 │
         └─────────────────────────────────┘
                           │
                  ┌────────▼────────┐
                  │  Artifact Store │
                  │  (S3/Nexus)    │
                  └────────┬────────┘
                           │
                  ┌────────▼────────┐
                  │  SBOM Database  │
                  │  (PostgreSQL)   │
                  └─────────────────┘
```

### Build Worker Implementation

```rust
// build-farm/src/worker.rs
use tokio::sync::mpsc;

pub struct BuildWorker {
    id: String,
    build_queue: mpsc::Receiver<BuildTask>,
    artifact_store: ArtifactStore,
    sbom_database: SBOMDatabase,
}

impl BuildWorker {
    pub async fn run(&mut self) -> Result<(), WorkerError> {
        while let Some(task) = self.build_queue.recv().await {
            self.process_build_task(task).await?;
        }
        Ok(())
    }
    
    async fn process_build_task(&mut self, task: BuildTask) -> Result<(), WorkerError> {
        // Create isolated build environment
        let build_env = self.create_build_environment(&task)?;
        
        // Build package
        let build_result = self.build_package(&task, build_env).await?;
        
        // Verify determinism
        self.verify_determinism(&build_result)?;
        
        // Generate SBOM
        let sbom = self.generate_sbom(&task, &build_result)?;
        
        // Store artifacts
        self.store_artifacts(&task, &build_result, &sbom).await?;
        
        Ok(())
    }
}
```

## Verification Infrastructure

### Cross-Verification

**Multiple Independent Builds**:
- Build same package on different workers
- Compare binary hashes
- Verify SBOM consistency
- Detect build non-determinism

```rust
// build-farm/src/verifier.rs
pub struct BuildVerifier {
    workers: Vec<BuildWorker>,
}

impl BuildVerifier {
    pub async fn cross_verify(&self, package: &Package) -> Result<VerificationResult, VerificationError> {
        let mut build_results = Vec::new();
        
        // Build on multiple workers
        for worker in &self.workers {
            let result = worker.build_package(package).await?;
            build_results.push(result);
        }
        
        // Compare results
        let mut all_match = true;
        for i in 1..build_results.len() {
            if build_results[0].hash != build_results[i].hash {
                all_match = false;
                break;
            }
        }
        
        Ok(VerificationResult {
            package: package.clone(),
            all_match,
            build_results,
        })
    }
}
```

### Diffoscope Integration

```bash
# Compare builds
diffoscope build1/ build2/ --html diff.html

# Detailed comparison
diffoscope --max-depth 10 build1/ build2/

# Text output
diffoscope --text diff.txt build1/ build2/
```

## Best Practices

### Development

1. **Version Pinning**: Pin all toolchain and dependency versions
2. **Deterministic Configuration**: Use fixed compiler flags and options
3. **Build Isolation**: Build in isolated containers
4. **Verification**: Always verify builds against references

### CI/CD

1. **Automated Verification**: Verify reproducibility in CI
2. **SBOM Generation**: Generate SBOMs for all builds
3. **Artifact Storage**: Store build artifacts with metadata
4. **Monitoring**: Monitor build reproducibility metrics

### Security

1. **Supply Chain Security**: Verify all dependencies
2. **Signature Verification**: Sign all build artifacts
3. **Audit Trail**: Maintain complete build logs
4. **Transparency**: Make build process public

## Troubleshooting

### Non-Reproducible Builds

**Common Causes**:
1. Timestamps embedded in binary
   - Fix: Use `SOURCE_DATE_EPOCH`
2. Different compiler versions
   - Fix: Pin toolchain versions
3. Build order differences
   - Fix: Use deterministic build order
4. Filesystem differences
   - Fix: Use containerized builds

**Debugging Steps**:
```bash
# Enable build logging
make V=1 > build.log

# Compare with reference build
diffoscope build1/ build2/

# Check for embedded timestamps
strings binary | grep -i date

# Verify environment
env | sort
```

## Roadmap & Milestones

### Phase 1 (Months 0-3)
- Build environment isolation
- Remove local paths, hostnames, and timestamps
- Basic deterministic compilation
- Container-based builds

### Phase 2 (Months 3-6)
- SPDX-compliant SBOM generator
- Integration with sigpkg-build
- GPG signing of SBOMs
- SBOM database

### Phase 3 (Months 6-9)
- Re-builder farms
- Cross-verification infrastructure
- Automated reproducibility testing
- Build farm orchestration

### Phase 4 (Months 9-12)
- System-wide verification policies
- Prevent installation of unsigned packages
- Advanced threat detection
- Community verification program

## References

- [Reproducible Builds Project](https://reproducible-builds.org/)
- [SPDX Specification](https://spdx.github.io/spdx-spec/)
- [Diffoscope](https://diffoscope.org/)
- [NixOS Reproducible Builds](https://nixos.org/manual/nix/stable/)
- [Supply Chain Security](https://www.cisa.gov/supply-chain-risk)
