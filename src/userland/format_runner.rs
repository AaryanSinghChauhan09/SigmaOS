// Universal Userland Executable & Binary Format Runner
// Location: src/userland/format_runner.rs
//
// Inspired by Linux (glibc, musl, Bionic, AppImage, Flatpak, WASI)
// and BSD (FreeBSD, OpenBSD, NetBSD ELF ABI headers and pledge/unveil notes),
// macOS (Mach-O), Windows (PE/COFF), and Android (APEX/APK).

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

/// Supported userland executable binary and package runtime formats across Linux, BSD, and major OS ecosystems.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserlandExecutableFormat {
    LinuxElf64Glibc,
    LinuxElf64Musl,
    FreeBsdElf64,
    OpenBsdElf64,
    NetBsdElf64,
    ShebangScript,
    AppImageSquashFs,
    FlatpakOstree,
    SnapSquashFs,
    WasmWasi,
    MacOsMachO,
    PeCoffWindows,
    AndroidApexApk,
}

impl UserlandExecutableFormat {
    pub fn name(&self) -> &'static str {
        match self {
            Self::LinuxElf64Glibc => "Linux ELF64 (glibc)",
            Self::LinuxElf64Musl => "Linux ELF64 (musl)",
            Self::FreeBsdElf64 => "FreeBSD ELF64",
            Self::OpenBsdElf64 => "OpenBSD ELF64",
            Self::NetBsdElf64 => "NetBSD ELF64",
            Self::ShebangScript => "Shebang Interpreter Script",
            Self::AppImageSquashFs => "AppImage Portable SquashFS Bundle",
            Self::FlatpakOstree => "Flatpak OSTree Sandbox Application",
            Self::SnapSquashFs => "Canonical Snap Container Package",
            Self::WasmWasi => "WebAssembly WASI Module",
            Self::MacOsMachO => "macOS Mach-O Universal Binary",
            Self::PeCoffWindows => "Windows PE/COFF Executable",
            Self::AndroidApexApk => "Android Bionic APEX/APK Binary",
        }
    }

    pub fn default_interpreter(&self) -> &'static str {
        match self {
            Self::LinuxElf64Glibc => "/lib64/ld-linux-x86-64.so.2",
            Self::LinuxElf64Musl => "/lib/ld-musl-x86_64.so.1",
            Self::FreeBsdElf64 => "/libexec/ld-elf.so.1",
            Self::OpenBsdElf64 => "/usr/libexec/ld.so",
            Self::NetBsdElf64 => "/libexec/ld.elf_so",
            Self::ShebangScript => "/bin/sh",
            Self::AppImageSquashFs => "/usr/bin/apprun",
            Self::FlatpakOstree => "/usr/bin/flatpak-run",
            Self::SnapSquashFs => "/usr/bin/snap-confine",
            Self::WasmWasi => "/usr/bin/wasmtime-runner",
            Self::MacOsMachO => "/usr/bin/macho-compat-runner",
            Self::PeCoffWindows => "/usr/bin/wine-runner",
            Self::AndroidApexApk => "/system/bin/linker64",
        }
    }
}

/// Dynamic userland execution context detailing binary metadata and environment initialization requirements.
#[derive(Debug, Clone)]
pub struct UserlandExecutionContext {
    pub format: UserlandExecutableFormat,
    pub binary_path: String,
    pub entry_point: u64,
    pub interpreter: String,
    pub library_paths: Vec<String>,
    pub exported_symbols: BTreeMap<String, u64>,
    pub environment_vars: BTreeMap<String, String>,
    pub pledge_mask: Option<String>,
}

pub struct UserlandFormatRunner;

impl UserlandFormatRunner {
    pub fn new() -> Self {
        Self
    }

    /// Helper to find subslice within raw binary header bytes
    fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
        if needle.is_empty() || haystack.len() < needle.len() {
            return false;
        }
        haystack.windows(needle.len()).any(|w| w == needle)
    }

    /// Auto-detects the userland executable format from magic header bytes, ELF EI_OSABI/notes, or shebang prefix.
    pub fn detect_format(raw_header: &[u8], filename_hint: Option<&str>) -> UserlandExecutableFormat {
        if raw_header.len() >= 4 {
            // ELF binary detection: 0x7F 'E' 'L' 'F'
            if raw_header[0] == 0x7F && raw_header[1] == b'E' && raw_header[2] == b'L' && raw_header[3] == b'F' {
                // Check ELF EI_OSABI byte (index 7 in e_ident)
                if raw_header.len() > 7 {
                    match raw_header[7] {
                        0x09 => return UserlandExecutableFormat::FreeBsdElf64,
                        0x0C => return UserlandExecutableFormat::NetBsdElf64,
                        _ => {}
                    }
                }

                // Check ELF ABI note sections by inspecting byte signatures
                if Self::contains_bytes(raw_header, b"FreeBSD") {
                    return UserlandExecutableFormat::FreeBsdElf64;
                }
                if Self::contains_bytes(raw_header, b"OpenBSD") {
                    return UserlandExecutableFormat::OpenBsdElf64;
                }
                if Self::contains_bytes(raw_header, b"NetBSD") {
                    return UserlandExecutableFormat::NetBsdElf64;
                }
                if Self::contains_bytes(raw_header, b"musl") {
                    return UserlandExecutableFormat::LinuxElf64Musl;
                }

                return UserlandExecutableFormat::LinuxElf64Glibc;
            }

            // WebAssembly WASI: '\0' 'a' 's' 'm'
            if raw_header[0] == 0x00 && raw_header[1] == b'a' && raw_header[2] == b's' && raw_header[3] == b'm' {
                return UserlandExecutableFormat::WasmWasi;
            }

            // macOS Mach-O Magic (MH_MAGIC_64 / MH_CIGAM_64 / FAT)
            if (raw_header[0] == 0xCF && raw_header[1] == 0xFA && raw_header[2] == 0xED && raw_header[3] == 0xFE)
                || (raw_header[0] == 0xCA && raw_header[1] == 0xFE && raw_header[2] == 0xBA && raw_header[3] == 0xBE)
            {
                return UserlandExecutableFormat::MacOsMachO;
            }

            // PE/COFF Windows Executable: 'M' 'Z'
            if raw_header[0] == b'M' && raw_header[1] == b'Z' {
                return UserlandExecutableFormat::PeCoffWindows;
            }

            // SquashFS AppImage / Snap Container header: 'h' 's' 'q' 's'
            if raw_header[0] == b'h' && raw_header[1] == b's' && raw_header[2] == b'q' && raw_header[3] == b's' {
                if let Some(hint) = filename_hint {
                    if hint.ends_with(".snap") {
                        return UserlandExecutableFormat::SnapSquashFs;
                    }
                }
                return UserlandExecutableFormat::AppImageSquashFs;
            }

            // Shebang Script Prefix: '#!'
            if raw_header[0] == b'#' && raw_header[1] == b'!' {
                return UserlandExecutableFormat::ShebangScript;
            }
        }

        // Fallback file extension hints
        if let Some(hint) = filename_hint {
            let lower = hint.to_lowercase();
            if lower.ends_with(".wasm") || lower.ends_with(".wasi") {
                return UserlandExecutableFormat::WasmWasi;
            }
            if lower.ends_with(".appimage") {
                return UserlandExecutableFormat::AppImageSquashFs;
            }
            if lower.ends_with(".flatpak") {
                return UserlandExecutableFormat::FlatpakOstree;
            }
            if lower.ends_with(".snap") {
                return UserlandExecutableFormat::SnapSquashFs;
            }
            if lower.ends_with(".exe") || lower.ends_with(".dll") {
                return UserlandExecutableFormat::PeCoffWindows;
            }
            if lower.ends_with(".apex") || lower.ends_with(".apk") {
                return UserlandExecutableFormat::AndroidApexApk;
            }
            if lower.ends_with(".sh") || lower.ends_with(".py") || lower.ends_with(".pl") {
                return UserlandExecutableFormat::ShebangScript;
            }
        }

        UserlandExecutableFormat::LinuxElf64Glibc
    }

    /// Prepares execution context for any userland binary payload.
    pub fn prepare_execution_context(
        &self,
        raw_payload: &[u8],
        path: &str,
    ) -> Result<UserlandExecutionContext, &'static str> {
        if raw_payload.is_empty() {
            return Err("Cannot execute empty binary payload");
        }

        let format = Self::detect_format(raw_payload, Some(path));
        let interpreter = format.default_interpreter().to_string();

        let mut library_paths = Vec::new();
        match format {
            UserlandExecutableFormat::LinuxElf64Glibc => {
                library_paths.push("/lib64".to_string());
                library_paths.push("/usr/lib64".to_string());
            }
            UserlandExecutableFormat::LinuxElf64Musl => {
                library_paths.push("/lib".to_string());
                library_paths.push("/usr/lib/musl".to_string());
            }
            UserlandExecutableFormat::FreeBsdElf64 => {
                library_paths.push("/freebsd/lib".to_string());
                library_paths.push("/freebsd/usr/lib".to_string());
            }
            UserlandExecutableFormat::OpenBsdElf64 | UserlandExecutableFormat::NetBsdElf64 => {
                library_paths.push("/bsd/lib".to_string());
                library_paths.push("/bsd/usr/lib".to_string());
            }
            UserlandExecutableFormat::AndroidApexApk => {
                library_paths.push("/system/lib64".to_string());
            }
            _ => {
                library_paths.push("/usr/lib".to_string());
            }
        }

        let mut environment_vars = BTreeMap::new();
        environment_vars.insert("SIGMAOS_USERLAND_FORMAT".to_string(), format.name().to_string());
        environment_vars.insert("LD_LIBRARY_PATH".to_string(), library_paths.join(":"));

        let mut exported_symbols = BTreeMap::new();
        exported_symbols.insert("main".to_string(), 0x401000);
        exported_symbols.insert("_start".to_string(), 0x401050);

        Ok(UserlandExecutionContext {
            format,
            binary_path: path.to_string(),
            entry_point: 0x401050,
            interpreter,
            library_paths,
            exported_symbols,
            environment_vars,
            pledge_mask: if format == UserlandExecutableFormat::OpenBsdElf64 {
                Some("stdio rpath wpath cpath proc exec".to_string())
            } else {
                None
            },
        })
    }

    /// Simulates execution of the userland context in the SigmaOS microkernel runtime.
    pub fn execute(&self, ctx: &UserlandExecutionContext) -> Result<i32, &'static str> {
        if ctx.binary_path.is_empty() {
            return Err("Invalid binary execution path");
        }
        Ok(0)
    }
}

impl Default for UserlandFormatRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_userland_executable_format_detection() {
        // ELF header magic
        let elf_hdr = [0x7F, b'E', b'L', b'F', 0, 0, 0, 0];
        assert_eq!(
            UserlandFormatRunner::detect_format(&elf_hdr, None),
            UserlandExecutableFormat::LinuxElf64Glibc
        );

        // FreeBSD ELF OSABI
        let mut freebsd_hdr = [0x7F, b'E', b'L', b'F', 0, 0, 0, 0x09];
        assert_eq!(
            UserlandFormatRunner::detect_format(&freebsd_hdr, None),
            UserlandExecutableFormat::FreeBsdElf64
        );

        // FreeBSD note byte sequence
        let mut freebsd_note = vec![0x7F, b'E', b'L', b'F', 0, 0, 0, 0];
        freebsd_note.extend_from_slice(b"FreeBSD");
        assert_eq!(
            UserlandFormatRunner::detect_format(&freebsd_note, None),
            UserlandExecutableFormat::FreeBsdElf64
        );

        // WASM header magic
        let wasm_hdr = [0x00, b'a', b's', b'm', 1, 0, 0, 0];
        assert_eq!(
            UserlandFormatRunner::detect_format(&wasm_hdr, None),
            UserlandExecutableFormat::WasmWasi
        );

        // PE Windows header magic
        let pe_hdr = [b'M', b'Z', 0, 0];
        assert_eq!(
            UserlandFormatRunner::detect_format(&pe_hdr, None),
            UserlandExecutableFormat::PeCoffWindows
        );

        // Shebang header magic
        let shebang_hdr = [b'#', b'!', b'/', b'b', b'i', b'n'];
        assert_eq!(
            UserlandFormatRunner::detect_format(&shebang_hdr, None),
            UserlandExecutableFormat::ShebangScript
        );

        // AppImage extension
        assert_eq!(
            UserlandFormatRunner::detect_format(&[0; 4], Some("app.appimage")),
            UserlandExecutableFormat::AppImageSquashFs
        );
    }

    #[test]
    fn test_prepare_execution_context() {
        let runner = UserlandFormatRunner::new();
        let elf_hdr = [0x7F, b'E', b'L', b'F', 0, 0, 0, 0];
        let ctx = runner.prepare_execution_context(&elf_hdr, "/usr/bin/nginx").unwrap();

        assert_eq!(ctx.format, UserlandExecutableFormat::LinuxElf64Glibc);
        assert_eq!(ctx.interpreter, "/lib64/ld-linux-x86-64.so.2");
        assert!(ctx.library_paths.contains(&"/lib64".to_string()));
        assert_eq!(runner.execute(&ctx).unwrap(), 0);
    }
}
