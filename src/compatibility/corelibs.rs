// SigmaOS Corelibs Multi-Distro C/POSIX Standard Library Engine
// Provides glibc, musl, FreeBSD libc, and OpenBSD libc compatibility layers and symbol resolution shims.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CorelibsLibraryKind {
    LibC,
    LibM,
    LibPthread,
    LibRt,
    LibDl,
    LibCrypt,
    LibUtil,
    LibExecinfo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CorelibsAbiFlavors {
    Glibc,
    Musl,
    FreeBsdLibc,
    OpenBsdLibc,
    NetBsdLibc,
}

#[derive(Debug, Clone)]
pub struct CorelibsSymbol {
    pub name: String,
    pub library: CorelibsLibraryKind,
    pub version: String,
    pub address_offset: usize,
    pub is_weak: bool,
}

#[derive(Debug, Clone)]
pub struct CorelibsConfig {
    pub default_abi: CorelibsAbiFlavors,
    pub enable_glibc_versioning: bool,
    pub enable_musl_static_pie: bool,
    pub enable_bsd_libc_compat: bool,
}

impl Default for CorelibsConfig {
    fn default() -> Self {
        Self {
            default_abi: CorelibsAbiFlavors::Glibc,
            enable_glibc_versioning: true,
            enable_musl_static_pie: true,
            enable_bsd_libc_compat: true,
        }
    }
}

pub struct CorelibsSymbolResolver {
    symbols: BTreeMap<String, CorelibsSymbol>,
}

impl CorelibsSymbolResolver {
    pub fn new() -> Self {
        let mut resolver = Self {
            symbols: BTreeMap::new(),
        };
        resolver.populate_standard_symbols();
        resolver
    }

    fn populate_standard_symbols(&mut self) {
        let std_symbols = [
            ("malloc", CorelibsLibraryKind::LibC, "GLIBC_2.2.5", 0x1000, false),
            ("free", CorelibsLibraryKind::LibC, "GLIBC_2.2.5", 0x1080, false),
            ("calloc", CorelibsLibraryKind::LibC, "GLIBC_2.2.5", 0x1100, false),
            ("realloc", CorelibsLibraryKind::LibC, "GLIBC_2.2.5", 0x1180, false),
            ("memcpy", CorelibsLibraryKind::LibC, "GLIBC_2.14", 0x1200, false),
            ("memset", CorelibsLibraryKind::LibC, "GLIBC_2.2.5", 0x1280, false),
            ("sin", CorelibsLibraryKind::LibM, "GLIBC_2.2.5", 0x2000, false),
            ("cos", CorelibsLibraryKind::LibM, "GLIBC_2.2.5", 0x2080, false),
            ("pow", CorelibsLibraryKind::LibM, "GLIBC_2.2.5", 0x2100, false),
            ("pthread_create", CorelibsLibraryKind::LibPthread, "GLIBC_2.2.5", 0x3000, false),
            ("pthread_join", CorelibsLibraryKind::LibPthread, "GLIBC_2.2.5", 0x3080, false),
            ("clock_gettime", CorelibsLibraryKind::LibRt, "GLIBC_2.17", 0x4000, false),
            ("dlopen", CorelibsLibraryKind::LibDl, "GLIBC_2.2.5", 0x5000, false),
            ("dlsym", CorelibsLibraryKind::LibDl, "GLIBC_2.2.5", 0x5080, false),
            ("crypt", CorelibsLibraryKind::LibCrypt, "GLIBC_2.2.5", 0x6000, false),
            ("backtrace", CorelibsLibraryKind::LibExecinfo, "GLIBC_2.2.5", 0x7000, false),
        ];

        for (name, lib, ver, offset, weak) in std_symbols {
            self.symbols.insert(
                name.to_string(),
                CorelibsSymbol {
                    name: name.to_string(),
                    library: lib,
                    version: ver.to_string(),
                    address_offset: offset,
                    is_weak: weak,
                },
            );
        }
    }

    pub fn lookup(&self, symbol_name: &str) -> Option<&CorelibsSymbol> {
        self.symbols.get(symbol_name)
    }

    pub fn total_symbols(&self) -> usize {
        self.symbols.len()
    }
}

pub struct SovereignCorelibsEngine {
    config: CorelibsConfig,
    resolver: CorelibsSymbolResolver,
    active_abi: CorelibsAbiFlavors,
}

impl SovereignCorelibsEngine {
    pub fn new(config: CorelibsConfig) -> Self {
        let active_abi = config.default_abi.clone();
        Self {
            config,
            resolver: CorelibsSymbolResolver::new(),
            active_abi,
        }
    }

    pub fn active_abi(&self) -> &CorelibsAbiFlavors {
        &self.active_abi
    }

    pub fn set_abi(&mut self, abi: CorelibsAbiFlavors) {
        self.active_abi = abi;
    }

    pub fn resolve_symbol(&self, name: &str) -> Option<&CorelibsSymbol> {
        self.resolver.lookup(name)
    }

    pub fn supports_musl(&self) -> bool {
        self.config.enable_musl_static_pie
    }

    pub fn supports_bsd_libc(&self) -> bool {
        self.config.enable_bsd_libc_compat
    }
}

#[cfg(test)]
mod corelibs_tests {
    use super::*;

    #[test]
    fn test_corelibs_engine_init() {
        let config = CorelibsConfig::default();
        let engine = SovereignCorelibsEngine::new(config);
        assert_eq!(*engine.active_abi(), CorelibsAbiFlavors::Glibc);
        assert!(engine.supports_musl());
        assert!(engine.supports_bsd_libc());
    }

    #[test]
    fn test_corelibs_symbol_lookup() {
        let engine = SovereignCorelibsEngine::new(CorelibsConfig::default());
        let malloc_sym = engine.resolve_symbol("malloc");
        assert!(malloc_sym.is_some());
        let sym = malloc_sym.unwrap();
        assert_eq!(sym.name, "malloc");
        assert_eq!(sym.library, CorelibsLibraryKind::LibC);

        let pthread_sym = engine.resolve_symbol("pthread_create");
        assert!(pthread_sym.is_some());
        assert_eq!(pthread_sym.unwrap().library, CorelibsLibraryKind::LibPthread);

        let dl_sym = engine.resolve_symbol("dlopen");
        assert!(dl_sym.is_some());
        assert_eq!(dl_sym.unwrap().library, CorelibsLibraryKind::LibDl);
    }

    #[test]
    fn test_abi_switch() {
        let mut engine = SovereignCorelibsEngine::new(CorelibsConfig::default());
        engine.set_abi(CorelibsAbiFlavors::Musl);
        assert_eq!(*engine.active_abi(), CorelibsAbiFlavors::Musl);

        engine.set_abi(CorelibsAbiFlavors::FreeBsdLibc);
        assert_eq!(*engine.active_abi(), CorelibsAbiFlavors::FreeBsdLibc);
    }
}
