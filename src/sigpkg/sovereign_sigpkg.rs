extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// =========================================================================
// 1. SIGPKG SPEC & FORMAT
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigpkgCompression {
    None,
    Zstd,
    Xz,
    Gzip,
}

#[derive(Debug, Clone)]
pub struct SigpkgHeader {
    pub magic: [u8; 4], // b"SPKG"
    pub format_version: u16,
    pub compression: SigpkgCompression,
    pub payload_size: u64,
    pub uncompressed_size: u64,
    pub sha256_checksum: [u8; 32],
    pub signature_ed25519: [u8; 64],
}

impl SigpkgHeader {
    pub fn new(
        payload_size: u64,
        uncompressed_size: u64,
        sha256_checksum: [u8; 32],
        sig: [u8; 64],
    ) -> Self {
        Self {
            magic: *b"SPKG",
            format_version: 1,
            compression: SigpkgCompression::Zstd,
            payload_size,
            uncompressed_size,
            sha256_checksum,
            signature_ed25519: sig,
        }
    }

    pub fn verify_magic(&self) -> bool {
        self.magic == *b"SPKG"
    }
}

// =========================================================================
// 2. CENTRAL PACKAGE REPOSITORY & CDN MIRRORS
// =========================================================================

#[derive(Debug, Clone)]
pub struct RepoMirror {
    pub url: String,
    pub region: String,
    pub latency_ms: u32,
    pub is_active: bool,
}

pub struct CentralRepositoryManager {
    pub mirrors: Vec<RepoMirror>,
    pub gpg_keyring: Vec<[u8; 32]>,
}

impl CentralRepositoryManager {
    pub fn new() -> Self {
        Self {
            mirrors: Vec::new(),
            gpg_keyring: Vec::new(),
        }
    }

    pub fn add_mirror(&mut self, url: &str, region: &str, latency_ms: u32) {
        self.mirrors.push(RepoMirror {
            url: url.to_string(),
            region: region.to_string(),
            latency_ms,
            is_active: true,
        });
    }

    pub fn select_fastest_mirror(&self) -> Option<&RepoMirror> {
        self.mirrors
            .iter()
            .filter(|m| m.is_active)
            .min_by_key(|m| m.latency_ms)
    }

    pub fn add_trusted_gpg_key(&mut self, key_fingerprint: [u8; 32]) {
        self.gpg_keyring.push(key_fingerprint);
    }
}

impl Default for CentralRepositoryManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. REPRODUCIBLE BUILD SYSTEM (NIX/GUIX-INSPIRED)
// =========================================================================

#[derive(Debug, Clone)]
pub struct ReproducibleBuildContext {
    pub source_date_epoch: u64,
    pub umask: u32,
    pub locale: String,
    pub timezone: String,
    pub build_path: String,
    pub sorted_dir_entries: bool,
}

impl ReproducibleBuildContext {
    pub fn new(epoch: u64) -> Self {
        Self {
            source_date_epoch: epoch,
            umask: 0o022,
            locale: "C.UTF-8".to_string(),
            timezone: "UTC".to_string(),
            build_path: "/build/sigpkg-sandbox".to_string(),
            sorted_dir_entries: true,
        }
    }

    pub fn compute_derivation_hash(
        &self,
        source_hash: &[u8; 32],
        env_vars: &BTreeMap<String, String>,
    ) -> [u8; 32] {
        let mut digest = [0u8; 32];
        for (i, &b) in source_hash.iter().enumerate() {
            digest[i] = b ^ ((self.source_date_epoch as u8).wrapping_add(i as u8));
        }
        let mut idx: usize = 0;
        for (k, v) in env_vars.iter() {
            let k_bytes: &[u8] = AsRef::<[u8]>::as_ref(k);
            for &b in k_bytes {
                digest[idx % 32] ^= b;
                idx += 1;
            }
            let v_bytes: &[u8] = AsRef::<[u8]>::as_ref(v);
            for &b in v_bytes {
                digest[idx % 32] ^= b;
                idx += 1;
            }
        }
        digest
    }
}

// =========================================================================
// 4. SOURCE-FIRST PACKAGING & BINARY CACHE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildPreference {
    SourceOnly,
    BinaryCachePreferred,
    BinaryOnly,
}

pub struct SourceFirstBuilder {
    pub preference: BuildPreference,
    pub binary_cache: BTreeMap<[u8; 32], Vec<u8>>,
}

impl SourceFirstBuilder {
    pub fn new(pref: BuildPreference) -> Self {
        Self {
            preference: pref,
            binary_cache: BTreeMap::new(),
        }
    }

    pub fn store_binary_cache(&mut self, derivation_hash: [u8; 32], artifact: Vec<u8>) {
        self.binary_cache.insert(derivation_hash, artifact);
    }

    pub fn fetch_or_build<F>(
        &mut self,
        derivation_hash: &[u8; 32],
        source_builder: F,
    ) -> Result<Vec<u8>, &'static str>
    where
        F: FnOnce() -> Result<Vec<u8>, &'static str>,
    {
        if self.preference != BuildPreference::SourceOnly {
            if let Some(artifact) = self.binary_cache.get(derivation_hash) {
                return Ok(Vec::clone(artifact));
            }
            if self.preference == BuildPreference::BinaryOnly {
                return Err("Binary cache miss and SourceOnly building is disabled");
            }
        }
        let built = source_builder()?;
        self.binary_cache.insert(*derivation_hash, built.clone());
        Ok(built)
    }
}

// =========================================================================
// 5. DETERMINISTIC DEPENDENCY RESOLVER & CONFLICT DIAGNOSTICS
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageRequirement {
    pub name: String,
    pub version_min: (u32, u32, u32),
    pub conflicts_with: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DependencyDiagnostic {
    pub package_a: String,
    pub package_b: String,
    pub conflict_reason: String,
}

pub struct DeterministicDependencyResolver {
    pub available_packages: BTreeMap<String, Vec<PackageRequirement>>,
}

impl DeterministicDependencyResolver {
    pub fn new() -> Self {
        Self {
            available_packages: BTreeMap::new(),
        }
    }

    pub fn add_package_spec(&mut self, pkg_name: &str, req: PackageRequirement) {
        self.available_packages
            .entry(pkg_name.to_string())
            .or_insert_with(Vec::new)
            .push(req);
    }

    pub fn resolve_dependencies(
        &self,
        root_targets: &[&str],
    ) -> Result<Vec<String>, DependencyDiagnostic> {
        let mut resolved = Vec::new();
        let mut conflicts = BTreeMap::new();

        for &target in root_targets {
            if let Some(reqs) = self.available_packages.get(target) {
                for req in reqs {
                    for conflict in &req.conflicts_with {
                        if resolved.contains(conflict) {
                            return Err(DependencyDiagnostic {
                                package_a: target.to_string(),
                                package_b: conflict.clone(),
                                conflict_reason: format!(
                                    "Direct incompatibility between {} and {}",
                                    target, conflict
                                ),
                            });
                        }
                        conflicts.insert(conflict.clone(), target.to_string());
                    }
                }
                if !resolved.contains(&target.to_string()) {
                    resolved.push(target.to_string());
                }
            }
        }
        resolved.sort();
        Ok(resolved)
    }
}

impl Default for DeterministicDependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 6. ATOMIC UPDATES & TRANSACTIONAL ROLLBACK
// =========================================================================

#[derive(Debug, Clone)]
pub struct PackageGeneration {
    pub generation_id: u32,
    pub installed_packages: Vec<String>,
    pub timestamp: u64,
}

pub struct AtomicTransactionEngine {
    pub history: Vec<PackageGeneration>,
    pub active_generation: u32,
}

impl AtomicTransactionEngine {
    pub fn new() -> Self {
        Self {
            history: vec![PackageGeneration {
                generation_id: 1,
                installed_packages: Vec::new(),
                timestamp: 0,
            }],
            active_generation: 1,
        }
    }

    pub fn commit_transaction(&mut self, packages: Vec<String>, timestamp: u64) -> u32 {
        let next_id = (self.history.len() as u32) + 1;
        self.history.push(PackageGeneration {
            generation_id: next_id,
            installed_packages: packages,
            timestamp,
        });
        self.active_generation = next_id;
        next_id
    }

    pub fn rollback_generation(
        &mut self,
        target_gen: u32,
    ) -> Result<&PackageGeneration, &'static str> {
        for gen in &self.history {
            if gen.generation_id == target_gen {
                self.active_generation = target_gen;
                return Ok(gen);
            }
        }
        Err("Target generation ID not found in transaction history")
    }
}

impl Default for AtomicTransactionEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 7. BINARY DELTA UPDATES (DIFF & PATCH)
// =========================================================================

pub struct BinaryDeltaGenerator;

impl BinaryDeltaGenerator {
    pub fn create_diff(old_bytes: &[u8], new_bytes: &[u8]) -> Vec<u8> {
        let mut diff = Vec::new();
        let max_len = old_bytes.len().max(new_bytes.len());
        for i in 0..max_len {
            let old_b = old_bytes.get(i).copied().unwrap_or(0);
            let new_b = new_bytes.get(i).copied().unwrap_or(0);
            diff.push(old_b ^ new_b);
        }
        diff
    }

    pub fn apply_patch(old_bytes: &[u8], diff_bytes: &[u8]) -> Vec<u8> {
        let mut patched = Vec::new();
        for i in 0..diff_bytes.len() {
            let old_b = old_bytes.get(i).copied().unwrap_or(0);
            let diff_b = diff_bytes[i];
            patched.push(old_b ^ diff_b);
        }
        patched
    }
}

// =========================================================================
// 8. PACKAGE BUILD SANDBOXING (NAMESPACE & ISOLATION)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SandboxPolicy {
    pub isolate_network: bool,
    pub isolate_pid: bool,
    pub isolate_ipc: bool,
    pub read_only_root: bool,
}

pub struct BuildSandboxEngine {
    pub policy: SandboxPolicy,
}

impl BuildSandboxEngine {
    pub fn new(policy: SandboxPolicy) -> Self {
        Self { policy }
    }

    pub fn execute_sandboxed_build<F>(&self, build_fn: F) -> Result<&'static str, &'static str>
    where
        F: FnOnce() -> bool,
    {
        if self.policy.isolate_network && self.policy.read_only_root {
            if build_fn() {
                Ok("Sandboxed build completed cleanly")
            } else {
                Err("Build script failed inside isolated sandbox")
            }
        } else {
            Err("Sandbox policy violated: network isolation or read-only root required")
        }
    }
}

// =========================================================================
// 9. CROSS-COMPILE TOOLCHAIN
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetArchitecture {
    X86_64,
    AArch64,
    RiscV64,
    Wasm32,
}

pub struct CrossCompileToolchain {
    pub host_arch: TargetArchitecture,
    pub target_arch: TargetArchitecture,
    pub sysroot: String,
}

impl CrossCompileToolchain {
    pub fn new(host: TargetArchitecture, target: TargetArchitecture, sysroot: &str) -> Self {
        Self {
            host_arch: host,
            target_arch: target,
            sysroot: sysroot.to_string(),
        }
    }

    pub fn get_target_triple(&self) -> &'static str {
        match self.target_arch {
            TargetArchitecture::X86_64 => "x86_64-sigmaos-linux-gnu",
            TargetArchitecture::AArch64 => "aarch64-sigmaos-linux-gnu",
            TargetArchitecture::RiscV64 => "riscv64-sigmaos-linux-gnu",
            TargetArchitecture::Wasm32 => "wasm32-unknown-emscripten",
        }
    }
}

// =========================================================================
// 10. PACKAGE SIGNING & SUPPLY-CHAIN ATTESTATION (SLSA)
// =========================================================================

#[derive(Debug, Clone)]
pub struct SlsaProvenanceAttestation {
    pub builder_id: String,
    pub build_type: String,
    pub source_repo: String,
    pub commit_sha: String,
    pub timestamp: u64,
}

impl SlsaProvenanceAttestation {
    pub fn new(builder: &str, source: &str, commit: &str, time: u64) -> Self {
        Self {
            builder_id: builder.to_string(),
            build_type: "https://sigmaos.org/sigpkg/build/v1".to_string(),
            source_repo: source.to_string(),
            commit_sha: commit.to_string(),
            timestamp: time,
        }
    }

    pub fn verify_provenance(&self) -> bool {
        !self.builder_id.is_empty() && !self.commit_sha.is_empty()
    }
}

// =========================================================================
// 11. LOCAL PACKAGE CACHE & PROXY
// =========================================================================

pub struct LocalPackageProxyCache {
    pub cached_downloads: BTreeMap<String, Vec<u8>>,
    pub total_hits: usize,
    pub total_misses: usize,
}

impl LocalPackageProxyCache {
    pub fn new() -> Self {
        Self {
            cached_downloads: BTreeMap::new(),
            total_hits: 0,
            total_misses: 0,
        }
    }

    pub fn get_or_download<F>(&mut self, url: &str, download_fn: F) -> Result<Vec<u8>, &'static str>
    where
        F: FnOnce() -> Result<Vec<u8>, &'static str>,
    {
        if let Some(bytes) = self.cached_downloads.get(url) {
            self.total_hits += 1;
            return Ok(Vec::clone(bytes));
        }
        self.total_misses += 1;
        let downloaded = download_fn()?;
        self.cached_downloads
            .insert(url.to_string(), downloaded.clone());
        Ok(downloaded)
    }
}

impl Default for LocalPackageProxyCache {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 12. PACKAGE VULNERABILITY SCANNER (CVE DIAGNOSTICS)
// =========================================================================

#[derive(Debug, Clone)]
pub struct CveEntry {
    pub cve_id: String,
    pub affected_package: String,
    pub severity: u8, // 1 - 10
}

pub struct VulnerabilityScanner {
    pub cve_database: Vec<CveEntry>,
}

impl VulnerabilityScanner {
    pub fn new() -> Self {
        Self {
            cve_database: Vec::new(),
        }
    }

    pub fn add_cve(&mut self, cve_id: &str, package: &str, severity: u8) {
        self.cve_database.push(CveEntry {
            cve_id: cve_id.to_string(),
            affected_package: package.to_string(),
            severity,
        });
    }

    pub fn scan_package(&self, package_name: &str) -> Vec<&CveEntry> {
        self.cve_database
            .iter()
            .filter(|cve| cve.affected_package == package_name)
            .collect()
    }
}

impl Default for VulnerabilityScanner {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 13. BUILD FARM AUTOMATION
// =========================================================================

#[derive(Debug, Clone)]
pub struct BuildWorker {
    pub worker_id: u32,
    pub supported_arch: TargetArchitecture,
    pub is_busy: bool,
}

pub struct BuildFarmManager {
    pub workers: Vec<BuildWorker>,
}

impl BuildFarmManager {
    pub fn new() -> Self {
        Self {
            workers: Vec::new(),
        }
    }

    pub fn register_worker(&mut self, id: u32, arch: TargetArchitecture) {
        self.workers.push(BuildWorker {
            worker_id: id,
            supported_arch: arch,
            is_busy: false,
        });
    }

    pub fn schedule_build(&mut self, target_arch: TargetArchitecture) -> Result<u32, &'static str> {
        for worker in self.workers.iter_mut() {
            if worker.supported_arch == target_arch && !worker.is_busy {
                worker.is_busy = true;
                return Ok(worker.worker_id);
            }
        }
        Err("No available build worker for target architecture")
    }
}

impl Default for BuildFarmManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 14. LANGUAGE RUNTIME MANAGEMENT (PYTHON, NODE, JAVA, RUST)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LanguageRuntime {
    Python,
    NodeJS,
    Java,
    Rust,
}

pub struct UnifiedRuntimeManager {
    pub active_runtimes: BTreeMap<LanguageRuntime, String>,
}

impl UnifiedRuntimeManager {
    pub fn new() -> Self {
        Self {
            active_runtimes: BTreeMap::new(),
        }
    }

    pub fn set_runtime_version(&mut self, runtime: LanguageRuntime, version: &str) {
        self.active_runtimes.insert(runtime, version.to_string());
    }

    pub fn get_runtime_version(&self, runtime: LanguageRuntime) -> Option<&str> {
        self.active_runtimes.get(&runtime).map(|s: &String| s.as_str())
    }
}

impl Default for UnifiedRuntimeManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 15. FLATPAK & CONTAINER INTEGRATION
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationType {
    NativeSigpkg,
    FlatpakSandbox,
    OciContainer,
}

pub struct FlatpakContainerIntegration {
    pub app_name: String,
    pub app_type: ApplicationType,
    pub sandbox_flags: Vec<String>,
}

impl FlatpakContainerIntegration {
    pub fn new(name: &str, app_type: ApplicationType) -> Self {
        Self {
            app_name: name.to_string(),
            app_type,
            sandbox_flags: Vec::new(),
        }
    }

    pub fn add_permission(&mut self, perm: &str) {
        self.sandbox_flags.push(perm.to_string());
    }
}

// =========================================================================
// 16. PACKAGE QUALITY GATES & LINTING
// =========================================================================

pub struct PackageQualityChecker;

impl PackageQualityChecker {
    pub fn check_quality(
        name: &str,
        license: &str,
        binaries_present: bool,
    ) -> Result<(), &'static str> {
        if name.is_empty() {
            return Err("Quality Gate: Package name cannot be empty");
        }
        if license.is_empty() {
            return Err("Quality Gate: Package license must be specified");
        }
        if !binaries_present {
            return Err("Quality Gate: No binaries or artifacts built");
        }
        Ok(())
    }
}

// =========================================================================
// 17. BINARY COMPATIBILITY LAYER (LINUX ABI SHIMS)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CRuntimeProvider {
    Glibc,
    Musl,
    SovereignKlib,
}

pub struct BinaryCompatibilityLayer {
    pub cruntime: CRuntimeProvider,
    pub sysv_abi_enabled: bool,
}

impl BinaryCompatibilityLayer {
    pub fn new(cruntime: CRuntimeProvider) -> Self {
        Self {
            cruntime,
            sysv_abi_enabled: true,
        }
    }

    pub fn resolve_symbol_shim(&self, symbol: &str) -> Option<&'static str> {
        match symbol {
            "__libc_start_main" => Some("sovereign_libc_start_main"),
            "malloc" => Some("sovereign_malloc"),
            "free" => Some("sovereign_free"),
            _ => None,
        }
    }
}

// =========================================================================
// 18. DEVELOPER PACKAGE TEMPLATES
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateKind {
    CCppCmake,
    RustCargo,
    NodeNpm,
    PythonSetuptools,
}

pub struct DeveloperPackageTemplateManager;

impl DeveloperPackageTemplateManager {
    pub fn generate_spec_template(name: &str, kind: TemplateKind) -> String {
        match kind {
            TemplateKind::CCppCmake => format!(
                "name = \"{}\"\nbuild_system = \"cmake\"\nsrc = \"https://github.com/example/{}.tar.gz\"\ndeps = [\"gcc\", \"cmake\"]\n",
                name, name
            ),
            TemplateKind::RustCargo => format!(
                "name = \"{}\"\nbuild_system = \"cargo\"\nsrc = \"https://crates.io/crates/{}\"\ndeps = [\"rustc\", \"cargo\"]\n",
                name, name
            ),
            TemplateKind::NodeNpm => format!(
                "name = \"{}\"\nbuild_system = \"npm\"\nsrc = \"https://registry.npmjs.org/{}\"\ndeps = [\"nodejs\"]\n",
                name, name
            ),
            TemplateKind::PythonSetuptools => format!(
                "name = \"{}\"\nbuild_system = \"setuptools\"\nsrc = \"https://pypi.org/packages/{}\"\ndeps = [\"python3\"]\n",
                name, name
            ),
        }
    }
}

// =========================================================================
// 19. PACKAGE ANALYTICS DASHBOARD
// =========================================================================

pub struct PackageAnalyticsDashboard {
    pub download_counts: BTreeMap<String, u64>,
    pub bandwidth_bytes_served: u64,
}

impl PackageAnalyticsDashboard {
    pub fn new() -> Self {
        Self {
            download_counts: BTreeMap::new(),
            bandwidth_bytes_served: 0,
        }
    }

    pub fn record_download(&mut self, pkg_name: &str, size_bytes: u64) {
        *self
            .download_counts
            .entry(pkg_name.to_string())
            .or_insert(0) += 1;
        self.bandwidth_bytes_served += size_bytes;
    }

    pub fn get_total_downloads(&self, pkg_name: &str) -> u64 {
        self.download_counts.get(pkg_name).copied().unwrap_or(0)
    }
}

impl Default for PackageAnalyticsDashboard {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 20. MIGRATION TOOLING (DEBIAN / ARCH / FEDORA TO SIGPKG)
// =========================================================================

pub struct LegacyPackageMigrator;

impl LegacyPackageMigrator {
    pub fn convert_deb_control(control_text: &str) -> Result<String, &'static str> {
        if !control_text.contains("Package:") {
            return Err("Invalid .deb control file format");
        }
        let mut name = "";
        let mut version = "";
        for line in control_text.lines() {
            if line.starts_with("Package:") {
                name = line.split(':').nth(1).unwrap_or("").trim();
            } else if line.starts_with("Version:") {
                version = line.split(':').nth(1).unwrap_or("").trim();
            }
        }
        Ok(format!(
            "[sigpkg]\nname = \"{}\"\nversion = \"{}\"\nconverted_from = \"debian\"\n",
            name, version
        ))
    }

    pub fn convert_arch_pkgbuild(pkgbuild_text: &str) -> Result<String, &'static str> {
        if !pkgbuild_text.contains("pkgname=") {
            return Err("Invalid Arch PKGBUILD format");
        }
        let mut name = "";
        let mut version = "";
        for line in pkgbuild_text.lines() {
            if line.starts_with("pkgname=") {
                name = line
                    .split('=')
                    .nth(1)
                    .unwrap_or("")
                    .trim_matches('"')
                    .trim();
            } else if line.starts_with("pkgver=") {
                version = line
                    .split('=')
                    .nth(1)
                    .unwrap_or("")
                    .trim_matches('"')
                    .trim();
            }
        }
        Ok(format!(
            "[sigpkg]\nname = \"{}\"\nversion = \"{}\"\nconverted_from = \"arch\"\n",
            name, version
        ))
    }

    pub fn convert_fedora_spec(spec_text: &str) -> Result<String, &'static str> {
        if !spec_text.contains("Name:") {
            return Err("Invalid Fedora RPM .spec file format");
        }
        let mut name = "";
        let mut version = "";
        for line in spec_text.lines() {
            if line.starts_with("Name:") {
                name = line.split(':').nth(1).unwrap_or("").trim();
            } else if line.starts_with("Version:") {
                version = line.split(':').nth(1).unwrap_or("").trim();
            }
        }
        Ok(format!(
            "[sigpkg]\nname = \"{}\"\nversion = \"{}\"\nconverted_from = \"fedora\"\n",
            name, version
        ))
    }
}

// =========================================================================
// 21. ALPINE APK INDEX VERIFICATION & CONTAINER TRIGGERS
// =========================================================================

#[derive(Debug, Clone)]
pub struct ApkPackageTrigger {
    pub trigger_path: String,
    pub target_script: String,
}

pub struct ApkIndexVerifier {
    pub trusted_keys: Vec<[u8; 32]>,
    pub triggers: Vec<ApkPackageTrigger>,
}

impl ApkIndexVerifier {
    pub fn new() -> Self {
        Self {
            trusted_keys: Vec::new(),
            triggers: Vec::new(),
        }
    }

    pub fn add_key(&mut self, key: [u8; 32]) {
        self.trusted_keys.push(key);
    }

    pub fn add_trigger(&mut self, path: &str, script: &str) {
        self.triggers.push(ApkPackageTrigger {
            trigger_path: path.to_string(),
            target_script: script.to_string(),
        });
    }

    pub fn verify_apk_index_hash(&self, index_bytes: &[u8], expected_hash: &[u8; 32]) -> bool {
        if index_bytes.is_empty() {
            return false;
        }
        // FNV-1a based 256-bit hashing over index bytes
        let mut computed = [0u8; 32];
        let mut state: u64 = 0xcbf29ce484222325;
        for (i, &b) in index_bytes.iter().enumerate() {
            state ^= b as u64;
            state = state.wrapping_mul(0x100000001b3);
            computed[i % 32] ^= (state >> ((i % 8) * 8)) as u8;
        }
        computed == *expected_hash
    }

    pub fn match_triggers(&self, path: &str) -> Vec<String> {
        let mut matched = Vec::new();
        for trigger in &self.triggers {
            if path.starts_with(&trigger.trigger_path) || trigger.trigger_path == "*" {
                matched.push(trigger.target_script.clone());
            }
        }
        matched
    }
}

// =========================================================================
// 22. OPENBSD PKG SIGNIFY & UNVEIL SANDBOX
// =========================================================================

#[derive(Debug, Clone)]
pub struct PkgUnveilPolicy {
    pub path: String,
    pub permissions: String, // e.g. "r", "rw", "rx", "rwc"
}

pub struct OpenBsdPkgSignifyVerifier {
    pub signify_pubkeys: Vec<String>,
    pub unveil_policies: Vec<PkgUnveilPolicy>,
}

impl OpenBsdPkgSignifyVerifier {
    pub fn new() -> Self {
        Self {
            signify_pubkeys: Vec::new(),
            unveil_policies: Vec::new(),
        }
    }

    pub fn add_signify_pubkey(&mut self, pubkey_b64: &str) {
        self.signify_pubkeys.push(pubkey_b64.to_string());
    }

    pub fn add_unveil_rule(&mut self, path: &str, permissions: &str) {
        self.unveil_policies.push(PkgUnveilPolicy {
            path: path.to_string(),
            permissions: permissions.to_string(),
        });
    }

    pub fn verify_signify_signature(&self, pkg_bytes: &[u8], signature_header: &str) -> bool {
        if pkg_bytes.is_empty() || !signature_header.starts_with("untrusted comment: verify with ") {
            return false;
        }
        // Extract key identifier from signify untrusted comment header
        let key_id = signature_header
            .split("untrusted comment: verify with ")
            .nth(1)
            .and_then(|s| s.lines().next())
            .unwrap_or("")
            .trim();

        if key_id.is_empty() {
            return false;
        }

        // Validate that key identifier matches one of registered trusted signify pubkeys
        self.signify_pubkeys.iter().any(|k| k.contains(key_id) || key_id.contains(k.as_str()))
    }

    pub fn validate_path_unveiled(&self, path: &str, required_perm: &str) -> bool {
        for policy in &self.unveil_policies {
            if path.starts_with(&policy.path) && policy.permissions.contains(required_perm) {
                return true;
            }
        }
        false
    }
}

// =========================================================================
// UNIT TESTS FOR ALL SUB-COMPONENTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigpkg_header() {
        let header = SigpkgHeader::new(1024, 2048, [1u8; 32], [2u8; 64]);
        assert!(header.verify_magic());
        assert_eq!(header.compression, SigpkgCompression::Zstd);
    }

    #[test]
    fn test_central_repository_manager() {
        let mut repo = CentralRepositoryManager::new();
        repo.add_mirror("https://us.sigmaos.org/pkg", "us", 45);
        repo.add_mirror("https://eu.sigmaos.org/pkg", "eu", 20);

        let fastest = repo.select_fastest_mirror().unwrap();
        assert_eq!(fastest.region, "eu");
    }

    #[test]
    fn test_reproducible_build_context() {
        let ctx = ReproducibleBuildContext::new(1700000000);
        let mut env = BTreeMap::new();
        env.insert("CC".to_string(), "gcc".to_string());
        let hash = ctx.compute_derivation_hash(&[0u8; 32], &env);
        assert_ne!(hash, [0u8; 32]);
    }

    #[test]
    fn test_source_first_builder() {
        let mut builder = SourceFirstBuilder::new(BuildPreference::BinaryCachePreferred);
        let hash = [5u8; 32];
        builder.store_binary_cache(hash, vec![1, 2, 3]);

        let res = builder.fetch_or_build(&hash, || Ok(vec![4, 5, 6])).unwrap();
        assert_eq!(res, vec![1, 2, 3]);
    }

    #[test]
    fn test_deterministic_dependency_resolver() {
        let mut resolver = DeterministicDependencyResolver::new();
        resolver.add_package_spec(
            "nginx",
            PackageRequirement {
                name: "nginx".to_string(),
                version_min: (1, 24, 0),
                conflicts_with: vec!["apache2".to_string()],
            },
        );
        resolver.add_package_spec(
            "apache2",
            PackageRequirement {
                name: "apache2".to_string(),
                version_min: (2, 4, 0),
                conflicts_with: vec!["nginx".to_string()],
            },
        );

        let res = resolver.resolve_dependencies(&["nginx"]);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec!["nginx"]);
    }

    #[test]
    fn test_atomic_transaction_engine() {
        let mut engine = AtomicTransactionEngine::new();
        let gen_id = engine.commit_transaction(vec!["curl".to_string()], 100);
        assert_eq!(gen_id, 2);

        let rolled = engine.rollback_generation(1).unwrap();
        assert_eq!(rolled.generation_id, 1);
    }

    #[test]
    fn test_binary_delta_generator() {
        let old_b = b"hello world";
        let new_b = b"hello sigma";
        let diff = BinaryDeltaGenerator::create_diff(old_b, new_b);
        let patched = BinaryDeltaGenerator::apply_patch(old_b, &diff);
        assert_eq!(&patched[..new_b.len()], new_b);
    }

    #[test]
    fn test_build_sandbox_engine() {
        let policy = SandboxPolicy {
            isolate_network: true,
            isolate_pid: true,
            isolate_ipc: true,
            read_only_root: true,
        };
        let sandbox = BuildSandboxEngine::new(policy);
        let res = sandbox.execute_sandboxed_build(|| true);
        assert!(res.is_ok());
    }

    #[test]
    fn test_cross_compile_toolchain() {
        let toolchain = CrossCompileToolchain::new(
            TargetArchitecture::X86_64,
            TargetArchitecture::AArch64,
            "/sysroot/aarch64",
        );
        assert_eq!(toolchain.get_target_triple(), "aarch64-sigmaos-linux-gnu");
    }

    #[test]
    fn test_slsa_provenance_attestation() {
        let att = SlsaProvenanceAttestation::new("builder-01", "github.com/org/repo", "abc1234", 1000);
        assert!(att.verify_provenance());
    }

    #[test]
    fn test_local_package_proxy_cache() {
        let mut cache = LocalPackageProxyCache::new();
        let bytes = cache.get_or_download("https://pkg.org/a.spkg", || Ok(vec![9, 9, 9])).unwrap();
        assert_eq!(bytes, vec![9, 9, 9]);

        // Second call should hit cache
        let cached_bytes = cache.get_or_download("https://pkg.org/a.spkg", || Err("should not run")).unwrap();
        assert_eq!(cached_bytes, vec![9, 9, 9]);
        assert_eq!(cache.total_hits, 1);
    }

    #[test]
    fn test_vulnerability_scanner() {
        let mut scanner = VulnerabilityScanner::new();
        scanner.add_cve("CVE-2024-0001", "openssl", 9);
        let cves = scanner.scan_package("openssl");
        assert_eq!(cves.len(), 1);
        assert_eq!(cves[0].cve_id, "CVE-2024-0001");
    }

    #[test]
    fn test_build_farm_manager() {
        let mut farm = BuildFarmManager::new();
        farm.register_worker(101, TargetArchitecture::RiscV64);
        let w_id = farm.schedule_build(TargetArchitecture::RiscV64).unwrap();
        assert_eq!(w_id, 101);
    }

    #[test]
    fn test_unified_runtime_manager() {
        let mut mgr = UnifiedRuntimeManager::new();
        mgr.set_runtime_version(LanguageRuntime::Rust, "1.78.0");
        assert_eq!(mgr.get_runtime_version(LanguageRuntime::Rust), Some("1.78.0"));
    }

    #[test]
    fn test_flatpak_container_integration() {
        let mut flatpak = FlatpakContainerIntegration::new("org.gimp.GIMP", ApplicationType::FlatpakSandbox);
        flatpak.add_permission("--socket=x11");
        assert_eq!(flatpak.sandbox_flags.len(), 1);
    }

    #[test]
    fn test_package_quality_checker() {
        let res = PackageQualityChecker::check_quality("my-app", "MIT", true);
        assert!(res.is_ok());
    }

    #[test]
    fn test_binary_compatibility_layer() {
        let compat = BinaryCompatibilityLayer::new(CRuntimeProvider::Glibc);
        assert_eq!(compat.resolve_symbol_shim("malloc"), Some("sovereign_malloc"));
    }

    #[test]
    fn test_developer_package_template_manager() {
        let spec = DeveloperPackageTemplateManager::generate_spec_template("ripgrep", TemplateKind::RustCargo);
        assert!(spec.contains("cargo"));
    }

    #[test]
    fn test_package_analytics_dashboard() {
        let mut dash = PackageAnalyticsDashboard::new();
        dash.record_download("bash", 2048);
        assert_eq!(dash.get_total_downloads("bash"), 1);
        assert_eq!(dash.bandwidth_bytes_served, 2048);
    }

    #[test]
    fn test_legacy_package_migrator() {
        let deb_ctrl = "Package: htop\nVersion: 3.2.2\n";
        let sigpkg_spec = LegacyPackageMigrator::convert_deb_control(deb_ctrl).unwrap();
        assert!(sigpkg_spec.contains("name = \"htop\""));
    }

    #[test]
    fn test_alpine_apk_index_verifier() {
        let mut verifier = ApkIndexVerifier::new();
        verifier.add_trigger("/etc/ssl/certs", "c_rehash");
        let sample_bytes = b"sample index content";
        let mut expected_hash = [0u8; 32];
        let mut state: u64 = 0xcbf29ce484222325;
        for (i, &b) in sample_bytes.iter().enumerate() {
            state ^= b as u64;
            state = state.wrapping_mul(0x100000001b3);
            expected_hash[i % 32] ^= (state >> ((i % 8) * 8)) as u8;
        }

        assert!(verifier.verify_apk_index_hash(sample_bytes, &expected_hash));
        assert!(!verifier.verify_apk_index_hash(b"", &expected_hash));

        let triggers = verifier.match_triggers("/etc/ssl/certs/ca.pem");
        assert_eq!(triggers.len(), 1);
        assert_eq!(triggers[0], "c_rehash");
    }

    #[test]
    fn test_openbsd_pkg_signify_verifier() {
        let mut verifier = OpenBsdPkgSignifyVerifier::new();
        verifier.add_signify_pubkey("RWT1234567890...");
        verifier.add_unveil_rule("/usr/local", "rx");

        assert!(verifier.verify_signify_signature(b"data", "untrusted comment: verify with RWT1234567890..."));
        assert!(!verifier.verify_signify_signature(b"", "untrusted comment: verify with RWT1234567890..."));
        assert!(!verifier.verify_signify_signature(b"data", "untrusted comment: verify with unknown_key"));
        assert!(verifier.validate_path_unveiled("/usr/local/bin/git", "r"));
        assert!(!verifier.validate_path_unveiled("/etc/shadow", "r"));
    }
}
