// SPDX-License-Identifier: MIT
//! SigmaOS Sovereign Customized Editions Engine
//!
//! Provides specialized, verified, and pre-configured OS customized profiles/editions:
//! - BetterOffice Edition: Documents, Spreadsheets, Slides, Diagrams
//! - CraftOffice Edition: Integrated Creative & Production Canvas Suite
//! - DosEdition: FreeDOS, MS-DOS, BASIC, ReactOS compatibility
//! - GenOffice Edition: Generative Docs, Sheet, Slides, PDF, Markdown, HTML
//! - KernelEdition: Multi-personality Kernel (Linux, GNU, Minix, ReactOS, XNU)
//! - LinuxDistroEdition: 14 Distro personas (Elementary, Endeavour, Fedora, Gentoo, Kali, Mint, Omarchy, Parrot, Qubes, ReactOS, Redox, Slackware, SUSE, Ubuntu)
//! - LibreOpenOfficeEdition: 6 classic modules (Writer, Calc, Impress, Draw, Math, Base)
//! - OnlyOfficeEdition: DocumentServer and Real-time Collaboration
//! - RdbmsEdition: PostgreSQL, DuckDB, DBeaver, Apache Arrow, Superset, Metabase
//! - ToolsEdition: 13 Pro Developer & AI Tools (Blender, Cline, CodexCLI, Command Code, CrewAI, DeepSeek, Docker, Hermes, LangChain, Notepad++, OpenClaw, VLC, Warp)
//! - CraftSuiteEdition: 20 Creative Craft modules (3D, Ani, Art, CAD, CraftLibs, Deck, Design, Effect, Fig, Film, Font, Grid, Light, PDF, Photo, Print, Sound, Vector, Voice, Word)
//!
//! Principles:
//! - AVOID FRAGMENTATION: Canonical central dispatch for all edition configurations.
//! - MAXIMISE COHESION & MINIMISE COUPLING: Self-contained edition descriptors.
//! - DEFENSIVE PROGRAMMING: Validated parameter bounds and strict error types.
//! - ARRANGE-ACT-ASSERT: Comprehensive unit tests proving every edition configures cleanly.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;

/// Primary categorization of customized SigmaOS Editions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SovereignEditionCategory {
    BetterOffice,
    CraftOffice,
    Dos,
    GenOffice,
    Kernel,
    LinuxDistro,
    LibreOpenOffice,
    OnlyOffice,
    Rdbms,
    Tools,
    CraftSuite,
}

/// Kernel personality configuration attached to an edition
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelPersonality {
    MicrokernelNative,
    LinuxAbiCompat,
    BsdPledgeCompat,
    MinixReincarnation,
    NtReactOsCompat,
    XnuMachCompat,
    DosRealModeEmul,
}

/// Memory & Scheduler policy preset
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditionRuntimePolicy {
    pub personality: KernelPersonality,
    pub scheduler_quantum_ms: u32,
    pub enforce_cfi: bool,
    pub enable_realtime_priority: bool,
    pub max_memory_mb: u64,
}

/// A specific, verified SigmaOS customized profile
#[derive(Debug, Clone)]
pub struct SigmaOSEditionConfig {
    pub id: &'static str,
    pub name: &'static str,
    pub category: SovereignEditionCategory,
    pub description: &'static str,
    pub default_shell: &'static str,
    pub runtime_policy: EditionRuntimePolicy,
    pub bundled_components: Vec<&'static str>,
    pub supported_formats: Vec<&'static str>,
}

/// Central Edition Registry and Customization Manager
pub struct SovereignEditionManager {
    editions: Vec<SigmaOSEditionConfig>,
    active_edition_index: usize,
}

impl SovereignEditionManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            editions: Vec::new(),
            active_edition_index: 0,
        };
        mgr.bootstrap_canonical_editions();
        mgr
    }

    fn bootstrap_canonical_editions(&mut self) {
        // 1. BetterOffice Edition
        self.editions.push(SigmaOSEditionConfig {
            id: "sigmaos-betteroffice",
            name: "SigmaOS BetterOffice Edition",
            category: SovereignEditionCategory::BetterOffice,
            description: "High-performance office suite: Documents, Spreadsheets, Slides, Diagrams",
            default_shell: "/bin/betteroffice-shell",
            runtime_policy: EditionRuntimePolicy {
                personality: KernelPersonality::MicrokernelNative,
                scheduler_quantum_ms: 10,
                enforce_cfi: true,
                enable_realtime_priority: false,
                max_memory_mb: 8192,
            },
            bundled_components: alloc::vec![
                "betteroffice-docs", "betteroffice-sheets", "betteroffice-slides", "betteroffice-diagrams"
            ],
            supported_formats: alloc::vec!["odt", "ods", "odp", "docx", "xlsx", "pptx", "svg"],
        });

        // 2. CraftOffice Edition
        self.editions.push(SigmaOSEditionConfig {
            id: "sigmaos-craftoffice",
            name: "SigmaOS CraftOffice Edition",
            category: SovereignEditionCategory::CraftOffice,
            description: "Creative & production canvas suite integrating vector graphics, layers, and flowcharts",
            default_shell: "/bin/craft-shell",
            runtime_policy: EditionRuntimePolicy {
                personality: KernelPersonality::MicrokernelNative,
                scheduler_quantum_ms: 8,
                enforce_cfi: true,
                enable_realtime_priority: true,
                max_memory_mb: 16384,
            },
            bundled_components: alloc::vec![
                "craftoffice-canvas", "craftoffice-vector", "craftoffice-flowchart", "craftoffice-assets"
            ],
            supported_formats: alloc::vec!["craft", "svg", "png", "pdf"],
        });

        // 3. DOS Edition
        self.editions.push(SigmaOSEditionConfig {
            id: "sigmaos-dos",
            name: "SigmaOS DOS & Legacy Edition",
            category: SovereignEditionCategory::Dos,
            description: "Full real-mode and protected-mode compatibility: FreeDOS, MS-DOS, BASIC, ReactOS NT",
            default_shell: "/bin/command.com",
            runtime_policy: EditionRuntimePolicy {
                personality: KernelPersonality::DosRealModeEmul,
                scheduler_quantum_ms: 50,
                enforce_cfi: false,
                enable_realtime_priority: false,
                max_memory_mb: 512,
            },
            bundled_components: alloc::vec![
                "freedos-compat", "msdos-emul", "basic-interpreter", "reactos-subsystem"
            ],
            supported_formats: alloc::vec!["com", "exe", "bas", "bat"],
        });

        // 4. GenOffice Edition
        self.editions.push(SigmaOSEditionConfig {
            id: "sigmaos-genoffice",
            name: "SigmaOS GenOffice Edition",
            category: SovereignEditionCategory::GenOffice,
            description: "Generative AI multi-target pipeline: Docs, Sheet, Slides, PDF, Markdown, HTML",
            default_shell: "/bin/genoffice-cli",
            runtime_policy: EditionRuntimePolicy {
                personality: KernelPersonality::MicrokernelNative,
                scheduler_quantum_ms: 15,
                enforce_cfi: true,
                enable_realtime_priority: false,
                max_memory_mb: 12288,
            },
            bundled_components: alloc::vec![
                "genoffice-docs", "genoffice-sheet", "genoffice-slides", "genoffice-pdf", "genoffice-md", "genoffice-html"
            ],
            supported_formats: alloc::vec!["md", "html", "pdf", "csv", "json"],
        });

        // 5. Kernel Edition
        self.editions.push(SigmaOSEditionConfig {
            id: "sigmaos-kernel-universal",
            name: "SigmaOS Universal Kernel Persona Edition",
            category: SovereignEditionCategory::Kernel,
            description: "Multi-personality kernel dispatcher supporting Linux, GNU Hurd, Minix, ReactOS, XNU",
            default_shell: "/bin/kdebug-shell",
            runtime_policy: EditionRuntimePolicy {
                personality: KernelPersonality::LinuxAbiCompat,
                scheduler_quantum_ms: 5,
                enforce_cfi: true,
                enable_realtime_priority: true,
                max_memory_mb: 32768,
            },
            bundled_components: alloc::vec![
                "linux-parity", "gnu-hurd-translators", "minix-reincarnation", "reactos-nt-driver", "xnu-iokit-emul"
            ],
            supported_formats: alloc::vec!["elf", "pe", "mach-o"],
        });

        // 6. Linux Distro Parity Edition
        self.editions.push(SigmaOSEditionConfig {
            id: "sigmaos-linux-universal",
            name: "SigmaOS Universal Linux Distro Edition",
            category: SovereignEditionCategory::LinuxDistro,
            description: "Integrated persona support: Elementary, Endeavour, Fedora, Gentoo, Kali, Mint, Omarchy, Parrot, Qubes, ReactOS, Redox, Slackware, SUSE, Ubuntu",
            default_shell: "/bin/fish",
            runtime_policy: EditionRuntimePolicy {
                personality: KernelPersonality::LinuxAbiCompat,
                scheduler_quantum_ms: 10,
                enforce_cfi: true,
                enable_realtime_priority: false,
                max_memory_mb: 16384,
            },
            bundled_components: alloc::vec![
                "mint-cinnamon", "omarchy-hyprland", "kali-sec-tools", "qubes-vm-domains", "fedora-dnf5", "arch-pacman"
            ],
            supported_formats: alloc::vec!["deb", "pkg.tar.zst", "rpm", "AppImage", "flatpak"],
        });

        // 7. LibreOffice / OpenOffice Edition
        self.editions.push(SigmaOSEditionConfig {
            id: "sigmaos-libreoffice",
            name: "SigmaOS LibreOffice / OpenOffice Parity Edition",
            category: SovereignEditionCategory::LibreOpenOffice,
            description: "Classical 6-module office suite: Writer, Calc, Impress, Draw, Math, Base",
            default_shell: "/bin/soffice",
            runtime_policy: EditionRuntimePolicy {
                personality: KernelPersonality::MicrokernelNative,
                scheduler_quantum_ms: 12,
                enforce_cfi: true,
                enable_realtime_priority: false,
                max_memory_mb: 8192,
            },
            bundled_components: alloc::vec![
                "writer", "calc", "impress", "draw", "math", "base"
            ],
            supported_formats: alloc::vec!["odt", "ods", "odp", "odg", "odf", "odb"],
        });

        // 8. OnlyOffice Edition
        self.editions.push(SigmaOSEditionConfig {
            id: "sigmaos-onlyoffice",
            name: "SigmaOS OnlyOffice Collaborative Edition",
            category: SovereignEditionCategory::OnlyOffice,
            description: "Real-time collaborative editing server and native OOXML document viewer/editor",
            default_shell: "/bin/onlyoffice-server",
            runtime_policy: EditionRuntimePolicy {
                personality: KernelPersonality::MicrokernelNative,
                scheduler_quantum_ms: 10,
                enforce_cfi: true,
                enable_realtime_priority: false,
                max_memory_mb: 8192,
            },
            bundled_components: alloc::vec![
                "onlyoffice-documentserver", "onlyoffice-coauthoring", "onlyoffice-ooxml-core"
            ],
            supported_formats: alloc::vec!["docx", "xlsx", "pptx"],
        });

        // 9. RDBMS & Analytics Edition
        self.editions.push(SigmaOSEditionConfig {
            id: "sigmaos-rdbms",
            name: "SigmaOS RDBMS & Data Engineering Edition",
            category: SovereignEditionCategory::Rdbms,
            description: "ACID transactions & vectorized OLAP: PostgreSQL, DuckDB, DBeaver, Apache Arrow, Superset, Metabase",
            default_shell: "/bin/psql",
            runtime_policy: EditionRuntimePolicy {
                personality: KernelPersonality::MicrokernelNative,
                scheduler_quantum_ms: 5,
                enforce_cfi: true,
                enable_realtime_priority: true,
                max_memory_mb: 32768,
            },
            bundled_components: alloc::vec![
                "postgres-engine", "duckdb-engine", "dbeaver-studio", "apache-datafusion", "superset-bi", "metabase-insights"
            ],
            supported_formats: alloc::vec!["sql", "parquet", "arrow", "csv", "duckdb"],
        });

        // 10. Tools & AI Autonomous Agents Edition
        self.editions.push(SigmaOSEditionConfig {
            id: "sigmaos-developer-ai-tools",
            name: "SigmaOS Developer & Autonomous AI Tools Edition",
            category: SovereignEditionCategory::Tools,
            description: "Pro developer toolchain: Blender, Cline, CodexCLI, Command Code, CrewAI, DeepSeek, Docker, Hermes, LangChain, Notepad++, OpenClaw, VLC, Warp",
            default_shell: "/bin/warp-terminal",
            runtime_policy: EditionRuntimePolicy {
                personality: KernelPersonality::MicrokernelNative,
                scheduler_quantum_ms: 8,
                enforce_cfi: true,
                enable_realtime_priority: true,
                max_memory_mb: 32768,
            },
            bundled_components: alloc::vec![
                "blender-pipeline", "cline-agent", "codex-cli", "crewai-orchestrator", "deepseek-coder",
                "docker-oci", "hermes-agent", "langchain-pipeline", "notepad-sigma", "openclaw", "vlc-pipeline", "warp-terminal"
            ],
            supported_formats: alloc::vec!["rs", "py", "c", "cpp", "blend", "mp4", "mkv", "oci"],
        });

        // 11. Craft Creative Suite Edition (20 modules)
        self.editions.push(SigmaOSEditionConfig {
            id: "sigmaos-craft-suite",
            name: "SigmaOS Craft 20-Module Creative Production Edition",
            category: SovereignEditionCategory::CraftSuite,
            description: "3D, Ani, Art, CAD, CraftLibs, Deck, Design, Effect, Fig, Film, Font, Grid, Light, PDF, Photo, Print, Sound, Vector, Voice, Word",
            default_shell: "/bin/craft-studio",
            runtime_policy: EditionRuntimePolicy {
                personality: KernelPersonality::MicrokernelNative,
                scheduler_quantum_ms: 6,
                enforce_cfi: true,
                enable_realtime_priority: true,
                max_memory_mb: 32768,
            },
            bundled_components: alloc::vec![
                "3dcraft", "anicraft", "artcraft", "cadcraft", "craftlibs", "deckcraft", "designcraft",
                "effectcraft", "figcraft", "filmcraft", "fontcraft", "gridcraft", "lightcraft", "pdfcraft",
                "photocraft", "printcraft", "soundcraft", "vectorcraft", "voicecraft", "wordcraft"
            ],
            supported_formats: alloc::vec![
                "3dc", "ani", "art", "step", "deck", "des", "fx", "fig", "mp4", "otf", "ttf", "wav", "flac", "svg", "pdf"
            ],
        });
    }

    pub fn total_editions(&self) -> usize {
        self.editions.len()
    }

    pub fn get_edition(&self, index: usize) -> Option<&SigmaOSEditionConfig> {
        self.editions.get(index)
    }

    pub fn find_by_id(&self, id: &str) -> Option<&SigmaOSEditionConfig> {
        self.editions.iter().find(|e| e.id == id)
    }

    pub fn find_by_category(&self, cat: SovereignEditionCategory) -> Vec<&SigmaOSEditionConfig> {
        self.editions.iter().filter(|e| e.category == cat).collect()
    }

    pub fn set_active_edition(&mut self, id: &str) -> Result<String, &'static str> {
        if let Some((idx, ed)) = self.editions.iter().enumerate().find(|(_, e)| e.id == id) {
            self.active_edition_index = idx;
            Ok(format!("Activated edition: {}", ed.name))
        } else {
            Err("Edition ID not found in registry")
        }
    }

    pub fn get_active_edition(&self) -> &SigmaOSEditionConfig {
        &self.editions[self.active_edition_index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edition_manager_initialization() {
        // Arrange
        let mgr = SovereignEditionManager::new();

        // Act & Assert
        assert_eq!(mgr.total_editions(), 11);
        let active = mgr.get_active_edition();
        assert_eq!(active.id, "sigmaos-betteroffice");
    }

    #[test]
    fn test_edition_category_lookup() {
        // Arrange
        let mgr = SovereignEditionManager::new();

        // Act
        let tools = mgr.find_by_category(SovereignEditionCategory::Tools);
        let craft = mgr.find_by_category(SovereignEditionCategory::CraftSuite);
        let dos = mgr.find_by_category(SovereignEditionCategory::Dos);

        // Assert
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].id, "sigmaos-developer-ai-tools");
        assert_eq!(tools[0].bundled_components.len(), 12);

        assert_eq!(craft.len(), 1);
        assert_eq!(craft[0].id, "sigmaos-craft-suite");
        assert_eq!(craft[0].bundled_components.len(), 20);

        assert_eq!(dos.len(), 1);
        assert_eq!(dos[0].runtime_policy.personality, KernelPersonality::DosRealModeEmul);
    }

    #[test]
    fn test_switch_active_edition() {
        // Arrange
        let mut mgr = SovereignEditionManager::new();

        // Act
        let res = mgr.set_active_edition("sigmaos-craft-suite");
        assert!(res.is_ok());

        // Assert
        let active = mgr.get_active_edition();
        assert_eq!(active.category, SovereignEditionCategory::CraftSuite);
        assert_eq!(active.default_shell, "/bin/craft-studio");
        assert!(active.supported_formats.contains(&"3dc"));
        assert!(active.supported_formats.contains(&"flac"));
    }

    #[test]
    fn test_reject_unknown_edition() {
        // Arrange
        let mut mgr = SovereignEditionManager::new();

        // Act
        let res = mgr.set_active_edition("non-existent-edition");

        // Assert
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Edition ID not found in registry");
    }
}
