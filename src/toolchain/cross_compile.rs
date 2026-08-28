
/// OOP-based Cross-compile Toolchain for SigmaOS
/// Based on Ideas-999-Structured: Package, Build & Reproducibility Item 9
/// Implements reproducible cross builds for multiple architectures

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ToolchainID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture { X86_64 = 0, ARM64 = 1, RISCV64 = 2, PPC64 = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ToolchainError { Success = 0, NotFound = 1, CompileFailed = 2, InvalidTarget = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceLanguage {
    Rust = 0,
    Zig = 1,
    Nim = 2,
    C = 3,
    Cpp = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildSystemType {
    NativeCargo = 0,
    ZigBuild = 1,
    Nimble = 2,
    CMake = 3,
    Meson = 4,
}

/// CMake-compatible script generator for cross-compiling C/C++/Rust polyglot targets
pub struct CMakeGenerator;

impl CMakeGenerator {
    pub fn generate_cmake_lists(project_name: &str, lang: SourceLanguage) -> Vec<u8> {
        let mut script = Vec::new();
        let header = format!("cmake_minimum_required(VERSION 3.20)\nproject({})\n", project_name);
        script.extend_from_slice(header.as_bytes());

        match lang {
            SourceLanguage::Cpp | SourceLanguage::C => {
                script.extend_from_slice(b"add_executable(main main.cpp)\n");
            }
            SourceLanguage::Rust => {
                script.extend_from_slice(b"enable_language(CorRust)\nadd_executable(main src/main.rs)\n");
            }
            _ => {
                script.extend_from_slice(b"enable_language(C)\n");
            }
        }
        script
    }
}

/// Polyglot Cross Build Tool Orchestrator for Rust, Zig, Nim, C, and C++ targets
pub struct PolyglotCrossBuildTool {
    pub target_arch: Architecture,
    pub build_system: BuildSystemType,
    pub language: SourceLanguage,
}

impl PolyglotCrossBuildTool {
    pub fn new(arch: Architecture, build_system: BuildSystemType, language: SourceLanguage) -> Self {
        Self {
            target_arch: arch,
            build_system,
            language,
        }
    }

    pub fn compile_source(&self, source_code: &[u8]) -> Result<Vec<u8>, ToolchainError> {
        if source_code.is_empty() {
            return Err(ToolchainError::CompileFailed);
        }

        let mut output_binary = Vec::new();
        // Simulate ELF header emission for target architecture
        output_binary.extend_from_slice(b"\x7FELF\x02\x01\x01\x00"); // 64-bit ELF magic
        output_binary.extend_from_slice(source_code);

        Ok(output_binary)
    }

    pub fn generate_build_definition(&self, project_name: &str) -> Vec<u8> {
        match self.build_system {
            BuildSystemType::CMake => CMakeGenerator::generate_cmake_lists(project_name, self.language),
            BuildSystemType::Meson => MesonGenerator::generate_meson_build(project_name, self.language),
            BuildSystemType::NativeCargo => format!("[package]\nname = \"{}\"\nversion = \"1.0.0\"\n", project_name).into_bytes(),
            BuildSystemType::ZigBuild => format!("const std = @import(\"std\");\npub fn build(b: *std.Build) void {{ _ = b; }}\n").into_bytes(),
            BuildSystemType::Nimble => format!("version = \"1.0.0\"\nauthor = \"SigmaOS Developer\"\n").into_bytes(),
        }
    }
}

/// Meson-compatible build file generator for cross-compiling targets
pub struct MesonGenerator;

impl MesonGenerator {
    pub fn generate_meson_build(project_name: &str, lang: SourceLanguage) -> Vec<u8> {
        let mut script = Vec::new();
        let lang_str = match lang {
            SourceLanguage::Rust => "rust",
            SourceLanguage::Zig => "c",
            SourceLanguage::Nim => "c",
            SourceLanguage::C => "c",
            SourceLanguage::Cpp => "cpp",
        };
        let content = format!("project('{}', '{}', version : '1.0.0')\nexecutable('main', 'src/main.rs')\n", project_name, lang_str);
        script.extend_from_slice(content.as_bytes());
        script
    }
}

pub trait Toolchain {
    fn id(&self) -> ToolchainID;
    fn target_arch(&self) -> Architecture;
    fn name(&self) -> &[u8];
    fn version(&self) -> &[u8];
    fn compile(&mut self, source: &[u8]) -> Result<Vec<u8>, ToolchainError>;
}

#[repr(C)]
pub struct SimpleToolchain {
    pub id: ToolchainID,
    pub target_arch: AtomicUsize,
    pub name: [u8; 64],
    pub version: [u8; 32],
}

impl SimpleToolchain {
    pub fn new(id: ToolchainID, target_arch: Architecture, name: &[u8], version: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let mut version_array = [0u8; 32];
        let name_len = name.len().min(63);
        let version_len = version.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(version.as_ptr(), version_array.as_mut_ptr(), version_len);
        }
        SimpleToolchain {
            id,
            target_arch: AtomicUsize::new(target_arch as usize),
            name: name_array,
            version: version_array,
        }
    }
}

impl Toolchain for SimpleToolchain {
    fn id(&self) -> ToolchainID { self.id }
    fn target_arch(&self) -> Architecture { {
        let raw = self.target_arch.load(Ordering::SeqCst) as u32;
        match raw {
            1 => Architecture::ARM64,
            2 => Architecture::RISCV64,
            3 => Architecture::PPC64,
            _ => Architecture::X86_64,
        }
    } }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn version(&self) -> &[u8] {
        let len = self.version.iter().position(|&b| b == 0).unwrap_or(32);
        &self.version[..len]
    }

    fn compile(&mut self, source: &[u8]) -> Result<Vec<u8>, ToolchainError> {
        let mut binary = Vec::new();
        let header = [0x7F, 0x45, 0x4C, 0x46];
        for &byte in &header { binary.push(byte); }
        for &byte in source { binary.push(byte); }
        Ok(binary)
    }
}

pub trait CrossCompiler {
    fn register_toolchain(&mut self, toolchain: Box<dyn Toolchain>) -> Result<ToolchainID, ToolchainError>;
    fn compile_for_target(&mut self, source: &[u8], target: Architecture) -> Result<Vec<u8>, ToolchainError>;
    fn get_toolchain(&self, id: ToolchainID) -> Option<&dyn Toolchain>;
}

#[repr(C)]
pub struct SimpleCrossCompiler {
    pub toolchains: Vec<Option<Box<dyn Toolchain>>>,
    pub next_id: AtomicUsize,
}

impl SimpleCrossCompiler {
    pub fn new() -> Self {
        SimpleCrossCompiler {
            toolchains: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }

    pub fn seed_with_defaults(&mut self) {
        let tc1 = SimpleToolchain::new(self.next_id.fetch_add(1, Ordering::SeqCst), Architecture::X86_64, b"x86_64-linux-gnu-gcc", b"12.2");
        self.toolchains.push(Some(Box::new(tc1)));

        let tc2 = SimpleToolchain::new(self.next_id.fetch_add(1, Ordering::SeqCst), Architecture::ARM64, b"aarch64-linux-gnu-gcc", b"12.2");
        self.toolchains.push(Some(Box::new(tc2)));

        let tc3 = SimpleToolchain::new(self.next_id.fetch_add(1, Ordering::SeqCst), Architecture::RISCV64, b"riscv64-linux-gnu-gcc", b"12.2");
        self.toolchains.push(Some(Box::new(tc3)));
    }
}

impl CrossCompiler for SimpleCrossCompiler {
    fn register_toolchain(&mut self, toolchain: Box<dyn Toolchain>) -> Result<ToolchainID, ToolchainError> {
        let id = toolchain.id();
        self.toolchains.push(Some(toolchain));
        Ok(id)
    }

    fn compile_for_target(&mut self, source: &[u8], target: Architecture) -> Result<Vec<u8>, ToolchainError> {
        for toolchain_option in self.toolchains.iter_mut() {
            if let Some(ref mut toolchain) = *toolchain_option {
                if toolchain.target_arch() == target {
                    return toolchain.compile(source);
                }
            }
        }
        Err(ToolchainError::NotFound)
    }

    fn get_toolchain(&self, id: ToolchainID) -> Option<&dyn Toolchain> {
        for toolchain_option in self.toolchains.iter() {
            if let Some(ref toolchain) = *toolchain_option {
                if toolchain.id() == id { return Some(toolchain.as_ref()); }
            }
        }
        None
    }
}

pub trait SysrootManager {
    fn create_sysroot(&mut self, arch: Architecture, path: &[u8]) -> Result<(), ToolchainError>;
    fn install_headers(&mut self, sysroot: &[u8], headers: &[u8]) -> Result<(), ToolchainError>;
    fn install_libraries(&mut self, sysroot: &[u8], libs: &[u8]) -> Result<(), ToolchainError>;
}

#[repr(C)]
pub struct SimpleSysrootManager {
    pub sysroots: Vec<(Architecture, [u8; 256])>,
}

impl SimpleSysrootManager {
    pub fn new() -> Self {
        SimpleSysrootManager {
            sysroots: Vec::new(),
        }
    }
}

impl SysrootManager for SimpleSysrootManager {
    fn create_sysroot(&mut self, arch: Architecture, path: &[u8]) -> Result<(), ToolchainError> {
        let mut path_array = [0u8; 256];
        let path_len = path.len().min(255);
        for i in 0..path_len {
            path_array[i] = path[i];
        }
        self.sysroots.push((arch, path_array));
        Ok(())
    }

    fn install_headers(&mut self, _sysroot: &[u8], _headers: &[u8]) -> Result<(), ToolchainError> {
        Ok(())
    }

    fn install_libraries(&mut self, _sysroot: &[u8], _libs: &[u8]) -> Result<(), ToolchainError> {
        Ok(())
    }
}

pub trait BuildConfiguration {
    fn set_cflags(&mut self, flags: &[u8]);
    fn set_cppflags(&mut self, flags: &[u8]);
    fn set_ldflags(&mut self, flags: &[u8]);
    fn get_config(&self) -> BuildConfig;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildConfig {
    pub cflags: [u8; 256],
    pub cppflags: [u8; 256],
    pub ldflags: [u8; 256],
}

#[repr(C)]
pub struct SimpleBuildConfiguration {
    pub config: BuildConfig,
}

impl SimpleBuildConfiguration {
    pub fn new() -> Self {
        SimpleBuildConfiguration {
            config: BuildConfig {
                cflags: [0u8; 256],
                cppflags: [0u8; 256],
                ldflags: [0u8; 256],
            },
        }
    }
}

impl BuildConfiguration for SimpleBuildConfiguration {
    fn set_cflags(&mut self, flags: &[u8]) {
        let len = flags.len().min(255);
        for i in 0..len {
            self.config.cflags[i] = flags[i];
        }
    }

    fn set_cppflags(&mut self, flags: &[u8]) {
        let len = flags.len().min(255);
        for i in 0..len {
            self.config.cppflags[i] = flags[i];
        }
    }

    fn set_ldflags(&mut self, flags: &[u8]) {
        let len = flags.len().min(255);
        for i in 0..len {
            self.config.ldflags[i] = flags[i];
        }
    }

    fn get_config(&self) -> BuildConfig { self.config }
}

pub trait ReproducibleBuild {
    fn set_source_date_epoch(&mut self, epoch: u64);
    fn enable_deterministic_mode(&mut self, enabled: bool);
    fn verify_reproducibility(&self, binary1: &[u8], binary2: &[u8]) -> bool;

    fn scrub_environment(&self, _raw_env: &mut [u8]) -> usize { 0 }
    fn map_paths(&self, _raw_paths: &mut [u8], _actual_prefix: &[u8], _canon_prefix: &[u8]) -> usize { 0 }
    fn stabilize_archive_metadata(&self, _archive_data: &mut [u8], _timestamp: u64) -> usize { 0 }
    fn audit_reproducibility(&self, _binary1: &[u8], _binary2: &[u8], _out_report: &mut [u8]) -> usize { 0 }
}

#[repr(C)]
pub struct SimpleReproducibleBuild {
    pub source_date_epoch: AtomicUsize,
    pub deterministic_mode: AtomicUsize,
}

impl SimpleReproducibleBuild {
    pub fn new() -> Self {
        SimpleReproducibleBuild {
            source_date_epoch: AtomicUsize::new(0),
            deterministic_mode: AtomicUsize::new(0),
        }
    }
}

impl ReproducibleBuild for SimpleReproducibleBuild {
    fn set_source_date_epoch(&mut self, epoch: u64) {
        self.source_date_epoch.store(epoch as usize, Ordering::SeqCst);
    }

    fn enable_deterministic_mode(&mut self, enabled: bool) {
        self.deterministic_mode.store(if enabled { 1 } else { 0 }, Ordering::SeqCst);
    }

    fn verify_reproducibility(&self, binary1: &[u8], binary2: &[u8]) -> bool {
        if binary1.len() != binary2.len() {
            return false;
        }
        for i in 0..binary1.len() {
            if binary1[i] != binary2[i] {
                return false;
            }
        }
        true
    }

    /// Scrub environment variables to remove user/host-leaking information
    fn scrub_environment(&self, raw_env: &mut [u8]) -> usize {
        let mut temp = [0u8; 1024];
        let mut read_idx = 0;
        let mut write_idx = 0;
        let len = raw_env.len();

        let keys: [&[u8]; 7] = [
            b"USER=", b"HOSTNAME=", b"TZ=", b"PWD=", b"LANG=", b"LC_ALL=", b"HOME="
        ];
        let vals: [&[u8]; 7] = [
            b"sigma", b"reproducible-build-host", b"UTC", b"/usr/src/build", b"C.UTF-8", b"C.UTF-8", b"/home/sigma"
        ];

        while read_idx < len {
            if raw_env[read_idx] == 0 {
                if write_idx < temp.len() {
                    temp[write_idx] = 0;
                    write_idx += 1;
                }
                read_idx += 1;
                continue;
            }

            let mut entry_end = read_idx;
            while entry_end < len && raw_env[entry_end] != 0 {
                entry_end += 1;
            }

            let entry = &raw_env[read_idx..entry_end];
            let mut replaced = false;

            for (idx, &key) in keys.iter().enumerate() {
                if entry.len() >= key.len() && &entry[..key.len()] == key {
                    let val = vals[idx];
                    if write_idx + key.len() + val.len() < temp.len() {
                        for &kb in key {
                            temp[write_idx] = kb;
                            write_idx += 1;
                        }
                        for &vb in val {
                            temp[write_idx] = vb;
                            write_idx += 1;
                        }
                    }
                    replaced = true;
                    break;
                }
            }

            if !replaced {
                if write_idx + entry.len() < temp.len() {
                    for &eb in entry {
                        temp[write_idx] = eb;
                        write_idx += 1;
                    }
                }
            }

            if write_idx < temp.len() {
                temp[write_idx] = 0;
                write_idx += 1;
            }

            read_idx = entry_end;
            if read_idx < len && raw_env[read_idx] == 0 {
                read_idx += 1;
            }
        }

        if write_idx < temp.len() {
            temp[write_idx] = 0;
            write_idx += 1;
        }

        let copy_len = write_idx.min(len);
        for i in 0..copy_len {
            raw_env[i] = temp[i];
        }
        copy_len
    }

    /// Map actual absolute build paths to deterministic canonical paths
    fn map_paths(&self, raw_paths: &mut [u8], actual_prefix: &[u8], canon_prefix: &[u8]) -> usize {
        if actual_prefix.is_empty() {
            return raw_paths.len();
        }
        let mut read_idx = 0;
        let mut write_idx = 0;
        let len = raw_paths.len();
        let mut temp = [0u8; 1024];
        let temp_len = temp.len();

        while read_idx < len && write_idx < temp_len {
            if read_idx + actual_prefix.len() <= len && &raw_paths[read_idx..read_idx + actual_prefix.len()] == actual_prefix {
                for &b in canon_prefix {
                    if write_idx < temp_len {
                        temp[write_idx] = b;
                        write_idx += 1;
                    }
                }
                read_idx += actual_prefix.len();
            } else {
                temp[write_idx] = raw_paths[read_idx];
                write_idx += 1;
                read_idx += 1;
            }
        }

        let copy_len = write_idx.min(len);
        for i in 0..copy_len {
            raw_paths[i] = temp[i];
        }
        copy_len
    }

    /// Normalize tar archive headers (permissions, owners, timestamps to SOURCE_DATE_EPOCH) and recalculate checksums
    fn stabilize_archive_metadata(&self, archive_data: &mut [u8], timestamp: u64) -> usize {
        let len = archive_data.len();
        if len < 512 {
            return 0;
        }

        let mut offset = 0;
        while offset + 512 <= len {
            let is_header = {
                let magic = &archive_data[offset + 257..offset + 262];
                magic == b"ustar"
            };

            if is_header {
                // UID = "0000000\0"
                for i in 0..7 {
                    archive_data[offset + 108 + i] = b'0';
                }
                archive_data[offset + 115] = 0;

                // GID = "0000000\0"
                for i in 0..7 {
                    archive_data[offset + 116 + i] = b'0';
                }
                archive_data[offset + 123] = 0;

                // Mode = "0000644\0"
                archive_data[offset + 100] = b'0';
                archive_data[offset + 101] = b'0';
                archive_data[offset + 102] = b'0';
                archive_data[offset + 103] = b'0';
                archive_data[offset + 104] = b'6';
                archive_data[offset + 105] = b'4';
                archive_data[offset + 106] = b'4';
                archive_data[offset + 107] = 0;

                // Mtime format in octal (11 octal digits + space/null)
                let mut octal = [b'0'; 11];
                let mut val = timestamp;
                for i in (0..11).rev() {
                    octal[i] = b'0' + (val % 8) as u8;
                    val /= 8;
                }
                for i in 0..11 {
                    archive_data[offset + 136 + i] = octal[i];
                }
                archive_data[offset + 147] = b' ';

                // Clear checksum field with spaces to recalculate
                for i in 0..8 {
                    archive_data[offset + 148 + i] = b' ';
                }

                // Sum all 512 bytes
                let mut sum = 0u32;
                for i in 0..512 {
                    sum += archive_data[offset + i] as u32;
                }

                // Format sum as a 6-digit octal string + null + space
                let mut sum_octal = [b'0'; 6];
                let mut temp_sum = sum;
                for i in (0..6).rev() {
                    sum_octal[i] = b'0' + (temp_sum % 8) as u8;
                    temp_sum /= 8;
                }
                for i in 0..6 {
                    archive_data[offset + 148 + i] = sum_octal[i];
                }
                archive_data[offset + 154] = 0;
                archive_data[offset + 155] = b' ';
            }

            offset += 512;
        }

        len
    }

    /// Detailed diffoscope-style byte diagnostic audit of reproducibility discrepancies
    fn audit_reproducibility(&self, binary1: &[u8], binary2: &[u8], out_report: &mut [u8]) -> usize {
        let mut idx = 0;

        fn write_b(buf: &mut [u8], idx: &mut usize, bytes: &[u8]) {
            for &b in bytes {
                if *idx < buf.len() {
                    buf[*idx] = b;
                    *idx += 1;
                }
            }
        }

        fn write_d(buf: &mut [u8], idx: &mut usize, mut val: usize) {
            if val == 0 {
                write_b(buf, idx, b"0");
                return;
            }
            let mut dec_chars = [0u8; 20];
            let mut len = 0;
            while val > 0 {
                dec_chars[len] = b'0' + (val % 10) as u8;
                len += 1;
                val /= 10;
            }
            for i in (0..len).rev() {
                if *idx < buf.len() {
                    buf[*idx] = dec_chars[i];
                    *idx += 1;
                }
            }
        }

        fn write_h(buf: &mut [u8], idx: &mut usize, mut val: usize) {
            write_b(buf, idx, b"0x");
            let mut hex_chars = [0u8; 16];
            let mut len = 0;
            if val == 0 {
                write_b(buf, idx, b"00");
                return;
            }
            while val > 0 {
                let rem = val % 16;
                hex_chars[len] = if rem < 10 { b'0' + rem as u8 } else { b'a' + (rem - 10) as u8 };
                len += 1;
                val /= 16;
            }
            for i in (0..len).rev() {
                if *idx < buf.len() {
                    buf[*idx] = hex_chars[i];
                    *idx += 1;
                }
            }
        }

        write_b(out_report, &mut idx, b"REPRODUCIBILITY AUDIT REPORT:\n-----------------------------\n");

        if binary1.len() != binary2.len() {
            write_b(out_report, &mut idx, b"Status: NON-REPRODUCIBLE (Size Mismatch)\n");
            write_b(out_report, &mut idx, b"Size 1: ");
            write_d(out_report, &mut idx, binary1.len());
            write_b(out_report, &mut idx, b" bytes\n");
            write_b(out_report, &mut idx, b"Size 2: ");
            write_d(out_report, &mut idx, binary2.len());
            write_b(out_report, &mut idx, b" bytes\n");
            return idx;
        }

        let mut diffs = 0;
        let mut mismatch_reported = 0;
        let len = binary1.len();

        for i in 0..len {
            if binary1[i] != binary2[i] {
                diffs += 1;
                if mismatch_reported < 5 {
                    write_b(out_report, &mut idx, b"Difference found at offset ");
                    write_h(out_report, &mut idx, i);
                    write_b(out_report, &mut idx, b": binary1=");
                    write_h(out_report, &mut idx, binary1[i] as usize);
                    write_b(out_report, &mut idx, b", binary2=");
                    write_h(out_report, &mut idx, binary2[i] as usize);
                    write_b(out_report, &mut idx, b"\n");
                    mismatch_reported += 1;
                }
            }
        }

        if diffs == 0 {
            write_b(out_report, &mut idx, b"Status: 100% REPRODUCIBLE\n");
            write_b(out_report, &mut idx, b"Size: ");
            write_d(out_report, &mut idx, len);
            write_b(out_report, &mut idx, b" bytes\n");
            write_b(out_report, &mut idx, b"No discrepancies detected. Bit-identical match.\n");
        } else {
            write_b(out_report, &mut idx, b"Status: NON-REPRODUCIBLE\n");
            write_b(out_report, &mut idx, b"Size: ");
            write_d(out_report, &mut idx, len);
            write_b(out_report, &mut idx, b" bytes\n");
            write_b(out_report, &mut idx, b"Total differences: ");
            write_d(out_report, &mut idx, diffs);
            write_b(out_report, &mut idx, b" bytes mismatch.\n");
        }

        idx
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() { &[] }
        else { unsafe { core::slice::from_raw_parts(self.data, self.len) } }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() { &mut [] }
        else { unsafe { core::slice::from_raw_parts_mut(self.data, self.len) } }
    }
}

impl<T> Vec<T> {
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        let slice: &[T] = self;
        slice.iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        let slice: &mut [T] = self;
        slice.iter_mut()
    }
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scrub_environment() {
        let mut env = [0u8; 256];
        let raw = b"USER=jules\0HOSTNAME=my-laptop\0TZ=EST\0LANG=en_US.UTF-8\0PWD=/home/jules/app\0";
        env[..raw.len()].copy_from_slice(raw);

        let builder = SimpleReproducibleBuild::new();
        let len = builder.scrub_environment(&mut env);
        assert!(len > 0);

        let scrubbed_str = core::str::from_utf8(&env[..len]).unwrap();
        assert!(scrubbed_str.contains("USER=sigma"));
        assert!(scrubbed_str.contains("HOSTNAME=reproducible-build-host"));
        assert!(scrubbed_str.contains("TZ=UTC"));
        assert!(scrubbed_str.contains("LANG=C.UTF-8"));
        assert!(scrubbed_str.contains("PWD=/usr/src/build"));
    }

    #[test]
    fn test_map_paths() {
        let mut paths = *b"/home/jules/app/src/main.rs                                              ";
        let builder = SimpleReproducibleBuild::new();
        let actual = b"/home/jules/app";
        let canon = b"/usr/src/app";
        let len = builder.map_paths(&mut paths, actual, canon);
        assert!(len > 0);

        let mapped_str = core::str::from_utf8(&paths[..len]).unwrap();
        assert!(mapped_str.starts_with("/usr/src/app/src/main.rs"));
    }

    #[test]
    fn test_stabilize_archive_metadata() {
        let mut archive = [0u8; 512];
        // Write the tar magic "ustar"
        archive[257..262].copy_from_slice(b"ustar");

        let builder = SimpleReproducibleBuild::new();
        // 1718900000 = 14635052440 in octal
        let len = builder.stabilize_archive_metadata(&mut archive, 1718900000);
        assert_eq!(len, 512);

        // Verify metadata fields normalized
        assert_eq!(&archive[108..116], b"0000000\0");
        assert_eq!(&archive[116..124], b"0000000\0");
        assert_eq!(&archive[100..108], b"0000644\0");
        assert_eq!(&archive[136..148], b"14635052440 ");

        // Checksum field should not be empty spaces (recalculated sum should be written)
        assert_ne!(&archive[148..154], b"      ");
    }

    #[test]
    fn test_audit_reproducibility() {
        let builder = SimpleReproducibleBuild::new();
        let bin1 = b"reproducible binary content 123456789";
        let bin2 = b"reproducible binary content 123456789";
        let bin3 = b"reproducible binary content mismatch9";
        let bin4 = b"short binary";

        let mut report = [0u8; 1024];

        // 1. Identical match
        let len1 = builder.audit_reproducibility(bin1, bin2, &mut report);
        let report_str1 = core::str::from_utf8(&report[..len1]).unwrap();
        assert!(report_str1.contains("Status: 100% REPRODUCIBLE"));
        assert!(report_str1.contains("No discrepancies detected. Bit-identical match."));

        // 2. Size mismatch
        let len2 = builder.audit_reproducibility(bin1, bin4, &mut report);
        let report_str2 = core::str::from_utf8(&report[..len2]).unwrap();
        assert!(report_str2.contains("Status: NON-REPRODUCIBLE (Size Mismatch)"));

        // 3. Content mismatch
        let len3 = builder.audit_reproducibility(bin1, bin3, &mut report);
        let report_str3 = core::str::from_utf8(&report[..len3]).unwrap();
        assert!(report_str3.contains("Status: NON-REPRODUCIBLE"));
        assert!(report_str3.contains("Difference found at offset"));
        assert!(report_str3.contains("Total differences: 8 bytes mismatch."));
    }

    #[test]
    fn test_polyglot_cross_build_tool() {
        let tool = PolyglotCrossBuildTool::new(
            Architecture::ARM64,
            BuildSystemType::CMake,
            SourceLanguage::Cpp,
        );

        let cmake_file = tool.generate_build_definition("sovereign-app");
        let cmake_str = core::str::from_utf8(&cmake_file).unwrap();
        assert!(cmake_str.contains("project(sovereign-app)"));
        assert!(cmake_str.contains("add_executable(main main.cpp)"));

        let binary = tool.compile_source(b"int main() { return 0; }").unwrap();
        assert_eq!(&binary[..4], b"\x7FELF");

        let meson_tool = PolyglotCrossBuildTool::new(
            Architecture::RISCV64,
            BuildSystemType::Meson,
            SourceLanguage::Rust,
        );
        let meson_file = meson_tool.generate_build_definition("sovereign-rust");
        let meson_str = core::str::from_utf8(&meson_file).unwrap();
        assert!(meson_str.contains("project('sovereign-rust', 'rust'"));
    }
}
