# Void Linux Parity Features for SigmaOS

## Overview

This document outlines Void Linux-specific features and their implementation in SigmaOS to provide parity with Void's focus on simplicity, performance, and native binary packages.

## XBPS Package Manager

### Native Package Management

```rust
pub struct SigmaXBPS {
    pub repositories: Vec<XbpsRepository>,
    pub virtual_packages: HashMap<String, VirtualPackage>,
    pub alternatives: AlternativesSystem,
}

pub struct XbpsRepository {
    pub name: String,
    pub url: String,
    pub architecture: String,
    pub enabled: bool,
}

pub struct VirtualPackage {
    pub name: String,
    pub providers: Vec<String>,
}

pub struct AlternativesSystem {
    pub groups: HashMap<String, AlternativeGroup>,
}

pub struct AlternativeGroup {
    pub name: String,
    pub current: String,
    pub alternatives: Vec<Alternative>,
}

pub struct Alternative {
    pub path: String,
    pub priority: i32,
}

impl SigmaXBPS {
    pub fn install(&mut self, packages: Vec<String>) -> Result<(), XbpsError> {
        for package in packages {
            if let Some(virtual_pkg) = self.virtual_packages.get(&package) {
                let provider = self.select_best_provider(virtual_pkg)?;
                self.install_package(&provider)?;
            } else {
                self.install_package(&package)?;
            }
        }
        Ok(())
    }

    pub fn remove(&mut self, packages: Vec<String>) -> Result<(), XbpsError> {
        for package in packages {
            self.remove_package(&package)?;
        }
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), XbpsError> {
        for repo in &mut self.repositories {
            if repo.enabled {
                self.update_repository_metadata(repo)?;
            }
        }

        let updates = self.get_package_updates()?;
        for update in updates {
            self.update_package(&update)?;
        }

        Ok(())
    }

    pub fn set_alternative(&mut self, group: &str, path: &str, priority: i32) -> Result<(), XbpsError> {
        let alt_group = self.alternatives.groups.get_mut(group)
            .ok_or(XbpsError::AlternativeGroupNotFound)?;

        let alternative = Alternative {
            path: path.to_string(),
            priority,
        };

        alt_group.alternatives.push(alternative);
        alt_group.current = path.to_string();

        self.update_alternative_symlinks(group)?;
        Ok(())
    }
}
```

## Runit Init System

### Simple Service Supervision

```rust
pub struct SigmaRunit {
    pub services: HashMap<String, RunitService>,
    pub svdir: PathBuf,
}

pub struct RunitService {
    pub name: String,
    pub enabled: bool,
    pub status: ServiceStatus,
    pub script: RunitScript,
}

pub struct RunitScript {
    pub run: String,
    pub finish: Option<String>,
    pub control: Option<String>,
}

pub enum ServiceStatus {
    Up,
    Down,
    Unknown,
}

impl SigmaRunit {
    pub fn enable_service(&mut self, service_name: &str) -> Result<(), RunitError> {
        let service = self.services.get_mut(service_name)
            .ok_or(RunitError::ServiceNotFound)?;

        service.enabled = true;
        self.create_symlink(service_name)?;

        Ok(())
    }

    pub fn disable_service(&mut self, service_name: &str) -> Result<(), RunitError> {
        let service = self.services.get_mut(service_name)
            .ok_or(RunitError::ServiceNotFound)?;

        service.enabled = false;
        self.remove_symlink(service_name)?;

        Ok(())
    }

    pub fn sv(&mut self, service_name: &str, command: SvCommand) -> Result<(), RunitError> {
        let service = self.services.get_mut(service_name)
            .ok_or(RunitError::ServiceNotFound)?;

        match command {
            SvCommand::Up => {
                self.start_service(service)?;
                service.status = ServiceStatus::Up;
            }
            SvCommand::Down => {
                self.stop_service(service)?;
                service.status = ServiceStatus::Down;
            }
            SvCommand::Restart => {
                self.restart_service(service)?;
            }
            SvCommand::Status => {
                service.status = self.check_service_status(service_name)?;
            }
        }

        Ok(())
    }
}
```

## LibreSSL Integration

### Modern Cryptography Library

```rust
pub struct SigmaLibreSSL {
    pub version: String,
    pub features: LibreSSLFeatures,
}

pub struct LibreSSLFeatures {
    pub tls_1_3: bool,
    pub post_quantum: bool,
    pub secure_source: bool,
}

impl SigmaLibreSSL {
    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn enable_feature(&mut self, feature: LibreSSLFeature) -> Result<(), LibreSSLError> {
        match feature {
            LibreSSLFeature::TLS13 => self.features.tls_1_3 = true,
            LibreSSLFeature::PostQuantum => self.features.post_quantum = true,
            LibreSSLFeature::SecureSource => self.features.secure_source = true,
        }
        Ok(())
    }
}
```

## Package Signatures

### Ed25519 Digital Signatures

```rust
pub struct SigmaPackageSignatures {
    pub keys: HashMap<String, Ed25519Key>,
    pub verification_policy: VerificationPolicy,
}

pub struct Ed25519Key {
    pub key_id: String,
    pub public_key: Vec<u8>,
    pub private_key: Option<Vec<u8>>,
}

pub enum VerificationPolicy {
    Strict,
    Permissive,
    Disabled,
}

impl SigmaPackageSignatures {
    pub fn verify_package(&self, package: &Package, signature: &[u8]) -> Result<bool, SignatureError> {
        match self.verification_policy {
            VerificationPolicy::Strict => {
                let key = self.get_key_for_package(package)?;
                self.verify_with_key(package, signature, key)
            }
            VerificationPolicy::Permissive => {
                if let Ok(result) = self.verify_strict(package, signature) {
                    Ok(result)
                } else {
                    Ok(true)
                }
            }
            VerificationPolicy::Disabled => Ok(true),
        }
    }

    pub fn sign_package(&self, package: &Package, key_id: &str) -> Result<Vec<u8>, SignatureError> {
        let key = self.keys.get(key_id)
            .ok_or(SignatureError::KeyNotFound)?;

        let private_key = key.private_key.as_ref()
            .ok_or(SignatureError::NoPrivateKey)?;

        self.sign_with_key(package, private_key)
    }
}
```

## Implementation Verification

All Void Linux parity components are verified through the automated test runner:

```bash
./run_sigma_tests.sh
```

Specific tests include:
- `test_xbps_package_manager`: Verifies XBPS package operations
- `test_runit_service_supervision`: Verifies Runit service management
- `test_libressl_integration`: Verifies LibreSSL features
- `test_package_signatures`: Verifies Ed25519 signature verification

## Best Practices

1. **Native Packages**: Use native binary packages instead of source compilation
2. **Simple Init**: Use Runit for simple and reliable service supervision
3. **Modern Crypto**: Use LibreSSL for modern cryptography
4. **Security**: Implement Ed25519 package signatures
5. **Minimal Dependencies**: Keep dependencies to a minimum

## References

* [Void Linux Documentation](https://voidlinux.org/)
* [XBPS Package Manager](https://github.com/void-linux/xbps)
* [Runit Documentation](http://smarden.org/runit/)
* [LibreSSL Project](https://www.libressl.org/)
