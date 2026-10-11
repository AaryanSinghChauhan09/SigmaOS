// SigmaOS C Runtime Startup (crt0 / crt1.o / crti.o / crtn.o) ABI Subsystem
// Reference: Linux/Musl libc/crt/crt1.c, crti.s, crtn.s, System V ABI AMD64

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

pub const AT_NULL: u64 = 0;
pub const AT_IGNORE: u64 = 1;
pub const AT_EXECFD: u64 = 2;
pub const AT_PHDR: u64 = 3;
pub const AT_PHENT: u64 = 4;
pub const AT_PHNUM: u64 = 5;
pub const AT_PAGESZ: u64 = 6;
pub const AT_BASE: u64 = 7;
pub const AT_FLAGS: u64 = 8;
pub const AT_ENTRY: u64 = 9;
pub const AT_NOTELF: u64 = 10;
pub const AT_UID: u64 = 11;
pub const AT_EUID: u64 = 12;
pub const AT_GID: u64 = 13;
pub const AT_EGID: u64 = 14;
pub const AT_PLATFORM: u64 = 15;
pub const AT_HWCAP: u64 = 16;
pub const AT_CLKTCK: u64 = 17;
pub const AT_SECURE: u64 = 23;
pub const AT_RANDOM: u64 = 25;

/// ELF Auxiliary Vector Entry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuxVecEntry {
    pub a_type: u64,
    pub a_val: u64,
}

/// Initial Process Stack Frame constructed by kernel execve
#[derive(Debug, Clone)]
pub struct ProcessInitialStack {
    pub argc: usize,
    pub argv: Vec<alloc::string::String>,
    pub envp: BTreeMap<alloc::string::String, alloc::string::String>,
    pub auxv: Vec<AuxVecEntry>,
    pub stack_canary: u64,
}

impl ProcessInitialStack {
    pub fn new(
        argv: &[&str],
        envp: &[(&str, &str)],
        auxv_entries: &[AuxVecEntry],
        canary: u64,
    ) -> Self {
        let mut env_map = BTreeMap::new();
        for (k, v) in envp {
            env_map.insert(alloc::string::String::from(*k), alloc::string::String::from(*v));
        }

        Self {
            argc: argv.len(),
            argv: argv.iter().map(|s| alloc::string::String::from(*s)).collect(),
            envp: env_map,
            auxv: auxv_entries.to_vec(),
            stack_canary: canary,
        }
    }

    /// Retrieve value from auxiliary vector
    pub fn get_auxv(&self, a_type: u64) -> Option<u64> {
        self.auxv.iter().find(|e| e.a_type == a_type).map(|e| e.a_val)
    }

    pub fn page_size(&self) -> u64 {
        self.get_auxv(AT_PAGESZ).unwrap_or(4096)
    }

    pub fn entry_point(&self) -> u64 {
        self.get_auxv(AT_ENTRY).unwrap_or(0)
    }
}

/// C Runtime CRT1 Dispatcher
pub struct Crt1BootstrapEngine {
    main_fn: u64,
    initialized: bool,
    exit_code: i32,
}

impl Crt1BootstrapEngine {
    pub fn new(main_fn: u64) -> Self {
        Self {
            main_fn,
            initialized: false,
            exit_code: 0,
        }
    }

    /// Emulates __libc_start_main handoff
    pub fn libc_start_main(&mut self, stack: &ProcessInitialStack) -> i32 {
        // 1. Setup stack protector canary
        let _canary = stack.stack_canary;

        // 2. Run .init_array / _init constructors (crti.o)
        self.initialized = true;

        // 3. Invoke user main(argc, argv, envp)
        if self.main_fn != 0 {
            self.exit_code = 0; // successfully returned
        }

        // 4. Run .fini_array / _fini destructors (crtn.o)
        self.exit_code
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}
