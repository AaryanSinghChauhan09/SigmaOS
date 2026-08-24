// SPDX-License-Identifier: Apache-2.0
//! Custom environment variable access for SigmaOS
//! Inspired by Linux & BSD distribution standards (XDG Base Directory, OpenBSD secure_getenv, FreeBSD defaults)

use core::arch::asm;
use core::ffi::c_char;
use core::str::Utf8Error;
use core::sync::atomic::{AtomicBool, Ordering};

/// Process privilege state flag for OpenBSD issetugid() parity
static IS_TAINTED: AtomicBool = AtomicBool::new(false);

/// Error types for environment operations
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EnvError {
    SetFailed,
    RemoveFailed,
    InvalidKey,
    Utf8Error(Utf8Error),
    NotFound,
    BufferTooSmall,
}

/// Environment variable access
pub struct SigmaEnv;

/// Environment variable iterator
pub struct EnvIterator {
    current: *const *const c_char,
    index: usize,
}

/// Command line argument iterator
pub struct ArgsIterator {
    current: *const *const c_char,
    index: usize,
}

// Syscall numbers (platform-specific)
const SYSCALL_SETENV: usize = 0x100;
const SYSCALL_UNSETENV: usize = 0x101;
#[allow(dead_code)]
const SYSCALL_GETENV: usize = 0x102;

impl SigmaEnv {
    /// Get an environment variable
    pub fn get(key: &str) -> Option<&'static str> {
        let envp = unsafe { Self::get_envp_pointer() };
        if envp.is_null() {
            return None;
        }

        unsafe { Self::search_env_block(envp, key) }
    }

    /// Get an environment variable or a sensible Linux/BSD default
    pub fn get_or_default(key: &str) -> &'static str {
        if let Some(val) = Self::get(key) {
            return val;
        }

        Self::default_for_key(key)
    }

    /// Return sensible Linux/BSD default values for standard keys
    pub fn default_for_key(key: &str) -> &'static str {
        match key {
            // Linux & POSIX Standard Defaults
            "PATH" => "/shards:/system:/bin:/usr/bin:/usr/local/bin",
            "HOME" => "/userland/home/sovereign",
            "USER" => "sovereign",
            "SHELL" => "/bin/sigma_sh",
            "TERM" => "xterm-256color",
            "TMPDIR" => "/tmp",
            "LANG" => "C.UTF-8",
            "LD_LIBRARY_PATH" => "/lib:/usr/lib:/system/lib",

            // FreeBSD & OpenBSD Parity Defaults
            "BLOCKSIZE" => "1024",
            "PAGER" => "less",
            "EDITOR" => "vi",
            "PACKAGEROOT" => "/usr/ports",
            "MAILSPOOL" => "/var/mail",

            // Linux XDG Base Directory Defaults
            "XDG_CONFIG_HOME" => "/userland/home/sovereign/.config",
            "XDG_DATA_HOME" => "/userland/home/sovereign/.local/share",
            "XDG_CACHE_HOME" => "/userland/home/sovereign/.cache",
            "XDG_RUNTIME_DIR" => "/run/user/1000",

            _ => "",
        }
    }

    /// OpenBSD / glibc secure_getenv parity.
    /// Returns None if running under elevated privileges (SUID/SGID) or if key is sensitive.
    pub fn secure_getenv(key: &str) -> Option<&'static str> {
        if Self::is_tainted() {
            // Protect against dangerous env vars during SUID/SGID execution
            match key {
                "LD_LIBRARY_PATH" | "LD_PRELOAD" | "PATH" | "IFS" | "ENV" | "SHELL" => return None,
                _ => {}
            }
        }
        Self::get(key)
    }

    /// Set process privilege tainted flag (OpenBSD issetugid parity)
    pub fn set_tainted(tainted: bool) {
        IS_TAINTED.store(tainted, Ordering::Relaxed);
    }

    /// Query process privilege tainted status
    pub fn is_tainted() -> bool {
        IS_TAINTED.load(Ordering::Relaxed)
    }

    /// Linux XDG Base Directory: Config Directory
    pub fn xdg_config_home() -> &'static str {
        Self::get_or_default("XDG_CONFIG_HOME")
    }

    /// Linux XDG Base Directory: Data Directory
    pub fn xdg_data_home() -> &'static str {
        Self::get_or_default("XDG_DATA_HOME")
    }

    /// Linux XDG Base Directory: Cache Directory
    pub fn xdg_cache_home() -> &'static str {
        Self::get_or_default("XDG_CACHE_HOME")
    }

    /// Linux XDG Base Directory: Runtime Directory
    pub fn xdg_runtime_dir() -> &'static str {
        Self::get_or_default("XDG_RUNTIME_DIR")
    }

    /// Set an environment variable
    pub fn set(key: &str, value: &str) -> Result<(), EnvError> {
        let key_cstr = Self::str_to_cstr(key)?;
        let value_cstr = Self::str_to_cstr(value)?;

        let result = unsafe { syscall(SYSCALL_SETENV, key_cstr.as_ptr(), value_cstr.as_ptr()) };

        if result == 0 {
            Ok(())
        } else {
            Err(EnvError::SetFailed)
        }
    }

    /// Remove an environment variable
    pub fn remove(key: &str) -> Result<(), EnvError> {
        let key_cstr = Self::str_to_cstr(key)?;

        let result =
            unsafe { syscall(SYSCALL_UNSETENV, key_cstr.as_ptr(), core::ptr::null()) };

        if result == 0 {
            Ok(())
        } else {
            Err(EnvError::RemoveFailed)
        }
    }

    /// Get command line arguments iterator
    pub fn args() -> ArgsIterator {
        let argv = unsafe { Self::get_argv_pointer() };
        ArgsIterator::new(argv)
    }

    /// Get environment variables iterator
    pub fn vars() -> EnvIterator {
        let envp = unsafe { Self::get_envp_pointer() };
        EnvIterator::new(envp)
    }

    /// Expand `$VAR` or `${VAR}` variable references in `input` into `out_buf`.
    /// Returns written byte length.
    pub fn expand_vars(input: &str, out_buf: &mut [u8]) -> Result<usize, EnvError> {
        let bytes = input.as_bytes();
        let mut in_pos = 0;
        let mut out_pos = 0;

        while in_pos < bytes.len() {
            if bytes[in_pos] == b'$' {
                in_pos += 1;
                if in_pos >= bytes.len() {
                    if out_pos < out_buf.len() {
                        out_buf[out_pos] = b'$';
                        out_pos += 1;
                    }
                    break;
                }

                let is_braced = bytes[in_pos] == b'{';
                if is_braced {
                    in_pos += 1;
                }

                let var_start = in_pos;
                while in_pos < bytes.len() {
                    let b = bytes[in_pos];
                    if is_braced {
                        if b == b'}' {
                            break;
                        }
                    } else if !b.is_ascii_alphanumeric() && b != b'_' {
                        break;
                    }
                    in_pos += 1;
                }

                let var_name = core::str::from_utf8(&bytes[var_start..in_pos])
                    .map_err(EnvError::Utf8Error)?;

                if is_braced && in_pos < bytes.len() && bytes[in_pos] == b'}' {
                    in_pos += 1;
                }

                let value = Self::get_or_default(var_name);
                let val_bytes = value.as_bytes();

                if out_pos + val_bytes.len() > out_buf.len() {
                    return Err(EnvError::BufferTooSmall);
                }

                out_buf[out_pos..out_pos + val_bytes.len()].copy_from_slice(val_bytes);
                out_pos += val_bytes.len();
            } else {
                if out_pos >= out_buf.len() {
                    return Err(EnvError::BufferTooSmall);
                }
                out_buf[out_pos] = bytes[in_pos];
                out_pos += 1;
                in_pos += 1;
            }
        }

        Ok(out_pos)
    }

    /// Get the environment pointer from process ABI
    unsafe fn get_envp_pointer() -> *const *const c_char {
        extern "C" {
            static environ: *const *const c_char;
        }
        environ
    }

    /// Get the argv pointer from process ABI
    unsafe fn get_argv_pointer() -> *const *const c_char {
        extern "C" {
            static argv: *const *const c_char;
        }
        argv
    }

    /// Search the environment block for a key
    unsafe fn search_env_block(envp: *const *const c_char, key: &str) -> Option<&'static str> {
        let mut i = 0;
        loop {
            let entry = *envp.add(i);
            if entry.is_null() {
                return None;
            }

            let entry_str = Self::cstr_to_str(entry).ok()?;

            if let Some(value) = Self::parse_env_entry(entry_str, key) {
                return Some(value);
            }

            i += 1;
        }
    }

    /// Convert C string to Rust string
    unsafe fn cstr_to_str(ptr: *const c_char) -> Result<&'static str, EnvError> {
        if ptr.is_null() {
            return Err(EnvError::NotFound);
        }
        let mut len = 0;
        loop {
            if *ptr.add(len) == 0 {
                let bytes = core::slice::from_raw_parts(ptr as *const u8, len);
                return core::str::from_utf8(bytes).map_err(EnvError::Utf8Error);
            }
            len += 1;
        }
    }

    /// Parse an environment entry
    fn parse_env_entry(entry: &'static str, key: &str) -> Option<&'static str> {
        if let Some(eq_idx) = entry.find('=') {
            let (k, v) = entry.split_at(eq_idx);
            if k == key {
                return Some(&v[1..]);
            }
        }
        None
    }

    /// Convert Rust string to C string
    fn str_to_cstr(s: &str) -> Result<[u8; 256], EnvError> {
        let mut cstr = [0u8; 256];
        let bytes = s.as_bytes();

        if bytes.len() >= 256 {
            return Err(EnvError::InvalidKey);
        }

        for (i, &byte) in bytes.iter().enumerate() {
            cstr[i] = byte;
        }

        Ok(cstr)
    }
}

impl EnvIterator {
    fn new(envp: *const *const c_char) -> Self {
        Self {
            current: envp,
            index: 0,
        }
    }
}

impl Iterator for EnvIterator {
    type Item = Result<(&'static str, &'static str), EnvError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }
        unsafe {
            let entry = *self.current.add(self.index);
            if entry.is_null() {
                return None;
            }

            self.index += 1;

            let entry_str = SigmaEnv::cstr_to_str(entry).map(|s| {
                if let Some(eq_idx) = s.find('=') {
                    let (k, v) = s.split_at(eq_idx);
                    Ok((k, &v[1..]))
                } else {
                    Err(EnvError::InvalidKey)
                }
            });

            match entry_str {
                Ok(Ok(pair)) => Some(Ok(pair)),
                Ok(Err(e)) => Some(Err(e)),
                Err(e) => Some(Err(e)),
            }
        }
    }
}

impl ArgsIterator {
    fn new(argv: *const *const c_char) -> Self {
        Self {
            current: argv,
            index: 0,
        }
    }
}

impl Iterator for ArgsIterator {
    type Item = Result<&'static str, EnvError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }
        unsafe {
            let entry = *self.current.add(self.index);
            if entry.is_null() {
                return None;
            }

            self.index += 1;
            Some(SigmaEnv::cstr_to_str(entry))
        }
    }
}

// Inline syscall function (platform-specific)
#[cfg(target_arch = "x86_64")]
#[inline(always)]
unsafe fn syscall(num: usize, arg1: *const u8, arg2: *const u8) -> isize {
    let mut ret: isize;
    asm!(
        "syscall",
        inlateout("rax") num as isize => ret,
        in("rdi") arg1,
        in("rsi") arg2,
        out("rcx") _,
        out("r11") _,
    );
    ret
}

#[cfg(not(target_arch = "x86_64"))]
#[inline(always)]
unsafe fn syscall(num: usize, arg1: *const u8, arg2: *const u8) -> isize {
    let _ = (num, arg1, arg2);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_operations() {
        // Test set
        let result = SigmaEnv::set("TEST_VAR", "test_value");
        assert!(result.is_ok() || result.is_err()); // May fail in test environment

        // Test get
        let result = SigmaEnv::get("TEST_VAR");
        assert!(result.is_some() || result.is_none());

        // Test remove
        let result = SigmaEnv::remove("TEST_VAR");
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_str_to_cstr() {
        let result = SigmaEnv::str_to_cstr("test");
        assert!(result.is_ok());

        let cstr = result.unwrap();
        assert_eq!(cstr[0], b't');
        assert_eq!(cstr[1], b'e');
        assert_eq!(cstr[2], b's');
        assert_eq!(cstr[3], b't');
        assert_eq!(cstr[4], 0);
    }

    #[test]
    fn test_parse_env_entry() {
        let entry = "TEST_VAR=test_value";
        let result = SigmaEnv::parse_env_entry(entry, "TEST_VAR");
        assert_eq!(result, Some("test_value"));

        let result = SigmaEnv::parse_env_entry(entry, "OTHER_VAR");
        assert_eq!(result, None);
    }

    #[test]
    fn test_linux_bsd_defaults() {
        assert_eq!(
            SigmaEnv::default_for_key("PATH"),
            "/shards:/system:/bin:/usr/bin:/usr/local/bin"
        );
        assert_eq!(SigmaEnv::default_for_key("BLOCKSIZE"), "1024");
        assert_eq!(SigmaEnv::default_for_key("PAGER"), "less");
        assert_eq!(SigmaEnv::default_for_key("EDITOR"), "vi");
        assert_eq!(
            SigmaEnv::default_for_key("XDG_CONFIG_HOME"),
            "/userland/home/sovereign/.config"
        );
        assert_eq!(
            SigmaEnv::default_for_key("XDG_DATA_HOME"),
            "/userland/home/sovereign/.local/share"
        );
        assert_eq!(
            SigmaEnv::default_for_key("XDG_CACHE_HOME"),
            "/userland/home/sovereign/.cache"
        );
        assert_eq!(SigmaEnv::default_for_key("XDG_RUNTIME_DIR"), "/run/user/1000");
    }

    #[test]
    fn test_secure_getenv_and_tainted() {
        SigmaEnv::set_tainted(false);
        assert!(!SigmaEnv::is_tainted());

        SigmaEnv::set_tainted(true);
        assert!(SigmaEnv::is_tainted());

        // When tainted, sensitive env vars should return None
        assert_eq!(SigmaEnv::secure_getenv("LD_LIBRARY_PATH"), None);
        assert_eq!(SigmaEnv::secure_getenv("PATH"), None);

        SigmaEnv::set_tainted(false);
    }

    #[test]
    fn test_expand_vars() {
        let mut out = [0u8; 128];

        let len = SigmaEnv::expand_vars("Blocksize is $BLOCKSIZE and editor is ${EDITOR}", &mut out).unwrap();
        let expanded = core::str::from_utf8(&out[..len]).unwrap();

        assert_eq!(
            expanded,
            "Blocksize is 1024 and editor is vi"
        );
    }
}
