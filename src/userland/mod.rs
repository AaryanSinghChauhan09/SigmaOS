pub mod coreutils;
pub mod format_runner;
pub mod init;
pub mod libc;
pub mod pkg;
pub mod security_sandbox;
pub mod shell;
pub mod stratum;

pub use format_runner::*;
pub use security_sandbox::*;
pub use shell::*;
pub use stratum::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_userland_format_runner_all_formats() {
        let runner = UserlandFormatRunner::new();

        // 1. Linux ELF glibc
        let elf_hdr = [0x7F, b'E', b'L', b'F', 0, 0, 0, 0];
        let ctx = runner.prepare_execution_context(&elf_hdr, "/bin/ls").unwrap();
        assert_eq!(ctx.format, UserlandExecutableFormat::LinuxElf64Glibc);
        assert_eq!(ctx.interpreter, "/lib64/ld-linux-x86-64.so.2");

        // 2. WebAssembly WASI
        let wasm_hdr = [0x00, b'a', b's', b'm', 1, 0, 0, 0];
        let ctx = runner.prepare_execution_context(&wasm_hdr, "/bin/app.wasm").unwrap();
        assert_eq!(ctx.format, UserlandExecutableFormat::WasmWasi);

        // 3. Shebang script
        let shebang_hdr = [b'#', b'!', b'/', b'b', b'i', b'n', b'/', b's', b'h'];
        let ctx = runner.prepare_execution_context(&shebang_hdr, "/bin/script.sh").unwrap();
        assert_eq!(ctx.format, UserlandExecutableFormat::ShebangScript);

        // 4. Windows PE/COFF
        let pe_hdr = [b'M', b'Z', 0, 0];
        let ctx = runner.prepare_execution_context(&pe_hdr, "/bin/app.exe").unwrap();
        assert_eq!(ctx.format, UserlandExecutableFormat::PeCoffWindows);
    }

    #[test]
    fn test_stratum_manager_cross_distro_interop() {
        let mgr = StratumManager::new();

        // Cross-stratum command path resolution
        let path = mgr.resolve_command_path("dpkg").unwrap();
        assert!(path.contains("dpkg"));

        // Unified library path construction across Linux & BSD strata
        let lib_paths = mgr.construct_unified_library_path();
        assert!(lib_paths.contains("/lib"));
        assert!(lib_paths.contains("/strata/arch/lib"));

        // Cross-stratum execution path translation
        let exec_path = mgr.cross_stratum_exec("emerge", StratumKind::Gentoo).unwrap();
        assert_eq!(exec_path, "/strata/gentoo/usr/bin/emerge");
    }

    #[test]
    fn test_openbsd_hardenedbsd_security_sandbox() {
        let mut sandbox = UserlandSecuritySandbox::new(42);

        // Pledge capability gating
        assert!(sandbox.pledge("stdio rpath cpath").is_ok());
        assert!(sandbox.check_pledge(PledgeCapability::StdIo));
        assert!(!sandbox.check_pledge(PledgeCapability::INet));

        // Unveil path restriction rules
        assert!(sandbox.unveil("/var/log", "r").is_ok());
        assert!(sandbox.check_unveil("/var/log/messages", "r"));
        assert!(!sandbox.check_unveil("/var/log/messages", "w"));

        // HardenedBSD W^X memory security enforcement
        assert!(sandbox.check_memory_protection(true, false));  // Write-only
        assert!(sandbox.check_memory_protection(false, true));  // Exec-only
        assert!(!sandbox.check_memory_protection(true, true));  // W^X violation!
    }

    #[test]
    fn test_userland_shell_integration() {
        let mut shell = Shell::new();

        let status = shell.execute_line("echo 'SigmaOS Userland Multi-Format System' > /tmp/out.txt").unwrap();
        assert_eq!(status, 0);

        let log = &shell.redirection_engine.redirection_log;
        assert!(log.iter().any(|l| l.contains("/tmp/out.txt")));
    }
}
