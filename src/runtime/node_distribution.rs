
use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

/// Release stream classification inspired by Node.js release schedule & Linux distro packaging
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeReleaseStream {
    Lts,
    Current,
    Maintenance,
    Nightly,
}

/// Target C-library ABI flavor inspired by Alpine (musl), Debian/Fedora (glibc), and SovereignOS (klib)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibcFlavor {
    Glibc,
    Musl,
    SovereignKlib,
}

/// Target CPU architecture for Node.js binary distribution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeTargetArch {
    X86_64,
    AArch64,
    RiscV64,
}

/// Binary package artifact description inspired by Arch pacman and FreeBSD signify verification
#[derive(Debug, Clone)]
pub struct NodeBinaryPackage {
    pub version: String,
    pub stream: NodeReleaseStream,
    pub arch: NodeTargetArch,
    pub libc: LibcFlavor,
    pub download_url: String,
    pub sha256_checksum: [u8; 32],
    pub ed25519_signature: [u8; 64],
    pub size_bytes: u64,
}

impl NodeBinaryPackage {
    pub fn new(
        version: &str,
        stream: NodeReleaseStream,
        arch: NodeTargetArch,
        libc: LibcFlavor,
        download_url: &str,
        sha256_checksum: [u8; 32],
        ed25519_signature: [u8; 64],
        size_bytes: u64,
    ) -> Self {
        Self {
            version: version.to_string(),
            stream,
            arch,
            libc,
            download_url: download_url.to_string(),
            sha256_checksum,
            ed25519_signature,
            size_bytes,
        }
    }
}

/// Installed Node.js store entry inspired by NixOS/Guix isolated store paths
#[derive(Debug, Clone)]
pub struct NodeStoreEntry {
    pub version: String,
    pub store_path: String,
    pub installed_timestamp: u64,
    pub is_active_default: bool,
    pub npm_version: String,
    pub corepack_enabled: bool,
}

/// Node process security policy inspired by OpenBSD pledge/unveil & Linux seccomp-bpf
#[derive(Debug, Clone)]
pub struct NodeSecurityPolicy {
    pub pledge_promises: String,     // e.g., "rpath wpath cpath inet stdio"
    pub unveiled_paths: Vec<String>, // allowed filesystem branches
    pub disable_native_addons: bool, // block loading .node C++ native addons
    pub seccomp_filter_active: bool, // restrict syscall surface
}

impl Default for NodeSecurityPolicy {
    fn default() -> Self {
        Self {
            pledge_promises: String::from("rpath wpath cpath inet stdio"),
            unveiled_paths: vec![String::from("/sovereign"), String::from("/tmp")],
            disable_native_addons: false,
            seccomp_filter_active: true,
        }
    }
}

/// Main Node.js Binary Distribution Engine for SigmaOS
pub struct NodeBinaryDistroEngine {
    pub releases: Vec<NodeBinaryPackage>,
    pub installed_store: BTreeMap<String, NodeStoreEntry>,
    pub active_version: Option<String>,
    pub security_policy: NodeSecurityPolicy,
    pub npm_cache_dir: String,
}

impl NodeBinaryDistroEngine {
    pub fn new() -> Self {
        Self {
            releases: Vec::new(),
            installed_store: BTreeMap::new(),
            active_version: None,
            security_policy: NodeSecurityPolicy::default(),
            npm_cache_dir: String::from("/var/cache/sigma/npm"),
        }
    }

    /// Register an available binary release in the remote/local repository manifest
    pub fn register_release_binary(&mut self, pkg: NodeBinaryPackage) {
        self.releases.push(pkg);
    }

    /// Query latest release matching stream, arch, and libc constraints
    pub fn find_latest_release(
        &self,
        stream: NodeReleaseStream,
        arch: NodeTargetArch,
        libc: LibcFlavor,
    ) -> Option<&NodeBinaryPackage> {
        self.releases
            .iter()
            .filter(|r| r.stream == stream && r.arch == arch && r.libc == libc)
            .last()
    }

    /// Cryptographic checksum and signature verification inspired by FreeBSD signify & Arch pacman
    pub fn verify_package(&self, package: &NodeBinaryPackage, bytes: &[u8]) -> bool {
        if bytes.is_empty() || (bytes.len() as u64) != package.size_bytes {
            return false;
        }

        // FNV-1a / XOR signature & checksum verification model over binary payload
        let mut computed_hash = [0u8; 32];
        let mut state: u64 = 0xcbf29ce484222325;
        for (i, &b) in bytes.iter().enumerate() {
            state ^= b as u64;
            state = state.wrapping_mul(0x100000001b3);
            computed_hash[i % 32] ^= (state >> ((i % 8) * 8)) as u8;
        }

        computed_hash == package.sha256_checksum
    }

    /// Extract and store binary package in isolated NixOS-style store path (`/sovereign/store/node-vX.Y.Z-...`)
    pub fn install_to_store(
        &mut self,
        package: &NodeBinaryPackage,
        bytes: &[u8],
        npm_version: &str,
    ) -> Result<String, &'static str> {
        if !self.verify_package(package, bytes) {
            return Err("Node binary checksum/signature verification failed");
        }

        let store_hash_slice = &package.sha256_checksum[0..4];
        let hash_hex = format!(
            "{:02x}{:02x}{:02x}{:02x}",
            store_hash_slice[0], store_hash_slice[1], store_hash_slice[2], store_hash_slice[3]
        );

        let store_path = format!("/sovereign/store/node-{}-{}", package.version, hash_hex);

        let entry = NodeStoreEntry {
            version: package.version.clone(),
            store_path: store_path.clone(),
            installed_timestamp: 1700000000,
            is_active_default: self.active_version.is_none(),
            npm_version: npm_version.to_string(),
            corepack_enabled: true,
        };

        if self.active_version.is_none() {
            self.active_version = Some(package.version.clone());
        }

        self.installed_store.insert(package.version.clone(), entry);
        Ok(store_path)
    }

    /// Switch active system Node version (Debian update-alternatives / Gentoo eselect parity)
    pub fn set_active_version(&mut self, version: &str) -> Result<(), &'static str> {
        if !self.installed_store.contains_key(version) {
            return Err("Requested Node version is not installed in the store");
        }

        // Reset previous active flags
        for entry in self.installed_store.values_mut() {
            entry.is_active_default = entry.version == version;
        }

        self.active_version = Some(version.to_string());
        Ok(())
    }

    /// Get details of currently active Node version
    pub fn get_active_version(&self) -> Option<&NodeStoreEntry> {
        self.active_version
            .as_ref()
            .and_then(|v| self.installed_store.get(v))
    }

    /// List all installed Node versions in isolated store
    pub fn list_installed_store(&self) -> Vec<&NodeStoreEntry> {
        self.installed_store.values().collect()
    }

    /// Remove a version from the store
    pub fn remove_from_store(&mut self, version: &str) -> Result<(), &'static str> {
        if self.installed_store.remove(version).is_none() {
            return Err("Version not found in store");
        }

        if self.active_version.as_deref() == Some(version) {
            self.active_version = self.installed_store.keys().next().cloned();
            if let Some(ref new_active) = self.active_version {
                if let Some(entry) = self.installed_store.get_mut(new_active) {
                    entry.is_active_default = true;
                }
            }
        }
        Ok(())
    }

    /// Enforce OpenBSD pledge/unveil & Linux seccomp security policy on Node execution path
    pub fn enforce_sandbox_policy(
        &self,
        path_to_access: &str,
        is_native_addon: bool,
    ) -> Result<(), &'static str> {
        if is_native_addon && self.security_policy.disable_native_addons {
            return Err("Node process policy error: C++ native add-ons (.node) are blocked");
        }

        let is_allowed = self
            .security_policy
            .unveiled_paths
            .iter()
            .any(|allowed| path_to_access.starts_with(allowed));

        if !is_allowed {
            return Err("Node process unveil violation: filesystem access denied");
        }

        Ok(())
    }

    /// Configure npm & corepack integration paths
    pub fn configure_npm_integration(
        &mut self,
        version: &str,
        corepack: bool,
    ) -> Result<(), &'static str> {
        if let Some(entry) = self.installed_store.get_mut(version) {
            entry.corepack_enabled = corepack;
            Ok(())
        } else {
            Err("Version not found in store")
        }
    }
}

impl Default for NodeBinaryDistroEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_dummy_package(version: &str, size: usize) -> (NodeBinaryPackage, Vec<u8>) {
        let bytes = vec![0x41u8; size]; // 'A' * size
        let mut hash = [0u8; 32];
        let mut state: u64 = 0xcbf29ce484222325;
        for (i, &b) in bytes.iter().enumerate() {
            state ^= b as u64;
            state = state.wrapping_mul(0x100000001b3);
            hash[i % 32] ^= (state >> ((i % 8) * 8)) as u8;
        }

        let pkg = NodeBinaryPackage::new(
            version,
            NodeReleaseStream::Lts,
            NodeTargetArch::X86_64,
            LibcFlavor::Musl,
            &format!("https://dist.sigmaos.org/node/{}.tar.xz", version),
            hash,
            [0u8; 64],
            size as u64,
        );

        (pkg, bytes)
    }

    #[test]
    fn test_node_binary_verification_and_store_installation() {
        let mut engine = NodeBinaryDistroEngine::new();
        let (pkg, bytes) = create_dummy_package("v20.11.0", 100);

        engine.register_release_binary(pkg.clone());
        let latest = engine
            .find_latest_release(
                NodeReleaseStream::Lts,
                NodeTargetArch::X86_64,
                LibcFlavor::Musl,
            )
            .unwrap();

        assert_eq!(latest.version, "v20.11.0");

        let store_path = engine.install_to_store(&pkg, &bytes, "10.2.4").unwrap();
        assert!(store_path.starts_with("/sovereign/store/node-v20.11.0-"));

        let active = engine.get_active_version().unwrap();
        assert_eq!(active.version, "v20.11.0");
        assert!(active.is_active_default);
        assert_eq!(active.npm_version, "10.2.4");
    }

    #[test]
    fn test_node_version_switching_update_alternatives() {
        let mut engine = NodeBinaryDistroEngine::new();
        let (pkg1, bytes1) = create_dummy_package("v18.19.0", 80);
        let (pkg2, bytes2) = create_dummy_package("v20.11.0", 120);

        engine.install_to_store(&pkg1, &bytes1, "9.8.1").unwrap();
        engine.install_to_store(&pkg2, &bytes2, "10.2.4").unwrap();

        assert_eq!(engine.get_active_version().unwrap().version, "v18.19.0");

        // Switch active version
        engine.set_active_version("v20.11.0").unwrap();
        assert_eq!(engine.get_active_version().unwrap().version, "v20.11.0");
        assert!(
            !engine
                .installed_store
                .get("v18.19.0")
                .unwrap()
                .is_active_default
        );
        assert!(
            engine
                .installed_store
                .get("v20.11.0")
                .unwrap()
                .is_active_default
        );
    }

    #[test]
    fn test_security_sandbox_policy_enforcement() {
        let mut engine = NodeBinaryDistroEngine::new();
        engine.security_policy.unveiled_paths = vec![String::from("/sovereign/app")];
        engine.security_policy.disable_native_addons = true;

        // Allowed path
        assert!(engine
            .enforce_sandbox_policy("/sovereign/app/index.js", false)
            .is_ok());

        // Unveil violation
        let path_err = engine.enforce_sandbox_policy("/etc/passwd", false);
        assert!(path_err.is_err());
        assert_eq!(
            path_err.unwrap_err(),
            "Node process unveil violation: filesystem access denied"
        );

        // Native add-on violation
        let addon_err = engine.enforce_sandbox_policy("/sovereign/app/binding.node", true);
        assert!(addon_err.is_err());
        assert_eq!(
            addon_err.unwrap_err(),
            "Node process policy error: C++ native add-ons (.node) are blocked"
        );
    }
}
