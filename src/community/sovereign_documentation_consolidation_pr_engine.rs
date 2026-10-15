// SPDX-License-Identifier: MIT
// SigmaOS Sovereign OS Documentation & GitHub Wiki Consolidation Engine
// Takes inspiration from Linux & BSD distributions (Arch Wiki, FreeBSD Handbook, Gentoo Docs, OpenBSD Man Pages, GNU Info)
// to consolidate Man pages, Info pages, /usr/share/doc, and Wiki/Handbooks into GitHub Wiki PR format.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Section category for traditional Unix/BSD/Linux man pages
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ManSection {
    UserCommands = 1,       // Executable programs / shell commands
    SystemCalls = 2,        // Kernel system calls
    LibraryFunctions = 3,   // C/POSIX library functions
    SpecialFiles = 4,       // Special files / devices (/dev/*)
    FileFormats = 5,        // File formats and conventions (/etc/*)
    Games = 6,              // Games and screensavers
    Miscellaneous = 7,      // Overview, macro packages, conventions
    SystemAdmin = 8,        // System administration commands and daemons
    KernelRoutines = 9,     // Non-standard kernel routines / BSD sysctl
}

impl ManSection {
    pub fn description(&self) -> &'static str {
        match self {
            ManSection::UserCommands => "1 - General User Commands",
            ManSection::SystemCalls => "2 - Kernel System Calls",
            ManSection::LibraryFunctions => "3 - C/POSIX Library Functions",
            ManSection::SpecialFiles => "4 - Special Files and Drivers",
            ManSection::FileFormats => "5 - File Formats and Configurations",
            ManSection::Games => "6 - Games and Demos",
            ManSection::Miscellaneous => "7 - Miscellaneous and Conventions",
            ManSection::SystemAdmin => "8 - System Administration and Daemons",
            ManSection::KernelRoutines => "9 - Kernel Subsystems and BSD Internals",
        }
    }
}

/// Structured Man Page entry parsed from groff/troff/mdoc source
#[derive(Debug, Clone)]
pub struct SovereignManPage {
    pub title: String,
    pub section: ManSection,
    pub synopsis: String,
    pub description: String,
    pub options: BTreeMap<String, String>,
    pub see_also: Vec<String>,
    pub bsd_linux_source: String,
}

impl SovereignManPage {
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str(&format!("# {}({})\n\n", self.title, self.section as u8));
        md.push_str(&format!("**Section:** {}\n", self.section.description()));
        md.push_str(&format!("**Origin/Inspiration:** {}\n\n", self.bsd_linux_source));
        md.push_str("## SYNOPSIS\n```text\n");
        md.push_str(&self.synopsis);
        md.push_str("\n```\n\n");
        md.push_str("## DESCRIPTION\n");
        md.push_str(&self.description);
        md.push_str("\n\n");

        if !self.options.is_empty() {
            md.push_str("## OPTIONS\n");
            for (opt, desc) in &self.options {
                md.push_str(&format!("* `{}`: {}\n", opt, desc));
            }
            md.push('\n');
        }

        if !self.see_also.is_empty() {
            md.push_str("## SEE ALSO\n");
            md.push_str(&self.see_also.join(", "));
            md.push('\n');
        }

        md
    }
}

/// Sovereign Man Pages Consolidation Engine
#[derive(Debug, Default, Clone)]
pub struct SovereignManPagesConsolidationEngine {
    pub pages: BTreeMap<String, SovereignManPage>,
}

impl SovereignManPagesConsolidationEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            pages: BTreeMap::new(),
        };
        engine.populate_defaults();
        engine
    }

    fn populate_defaults(&mut self) {
        // Section 1 - User Commands
        self.register_page(SovereignManPage {
            title: "sigma-sh".to_string(),
            section: ManSection::UserCommands,
            synopsis: "sigma-sh [-c COMMAND] [FILE]".to_string(),
            description: "Sovereign interactive POSIX-compliant command interpreter with eBPF history tracking.".to_string(),
            options: {
                let mut opts = BTreeMap::new();
                opts.insert("-c".to_string(), "Execute string command".to_string());
                opts.insert("-l".to_string(), "Start as login shell".to_string());
                opts
            },
            see_also: vec!["bash(1)".to_string(), "zsh(1)".to_string(), "tcsh(1)".to_string()],
            bsd_linux_source: "OpenBSD ksh(1) & GNU bash(1)".to_string(),
        });

        self.register_page(SovereignManPage {
            title: "sigma-ctl".to_string(),
            section: ManSection::UserCommands,
            synopsis: "sigma-ctl [COMMAND] [TARGET]".to_string(),
            description: "System control utility for service supervision, target state changes, and unit inspection.".to_string(),
            options: {
                let mut opts = BTreeMap::new();
                opts.insert("status".to_string(), "Query runtime status of service unit".to_string());
                opts.insert("restart".to_string(), "Restart specified system daemon".to_string());
                opts
            },
            see_also: vec!["systemctl(1)".to_string(), "rcctl(8)".to_string()],
            bsd_linux_source: "systemd systemctl & OpenBSD rcctl(8)".to_string(),
        });

        self.register_page(SovereignManPage {
            title: "sigma-top".to_string(),
            section: ManSection::UserCommands,
            synopsis: "sigma-top [-d DELAY] [-p PID]".to_string(),
            description: "Interactive real-time process monitor with Cgroup v2 PSI metrics and per-CPU breakdown.".to_string(),
            options: {
                let mut opts = BTreeMap::new();
                opts.insert("-d".to_string(), "Update delay interval in seconds".to_string());
                opts.insert("-p".to_string(), "Filter inspection to specific process ID".to_string());
                opts
            },
            see_also: vec!["htop(1)".to_string(), "top(1)".to_string()],
            bsd_linux_source: "Linux htop(1) & FreeBSD top(1)".to_string(),
        });

        self.register_page(SovereignManPage {
            title: "sigma-fetch".to_string(),
            section: ManSection::UserCommands,
            synopsis: "sigma-fetch [--json]".to_string(),
            description: "Fast system information generator detailing kernel version, uptime, microarchitecture, and memory usage.".to_string(),
            options: {
                let mut opts = BTreeMap::new();
                opts.insert("--json".to_string(), "Output system telemetry in JSON format".to_string());
                opts
            },
            see_also: vec!["fastfetch(1)".to_string(), "neofetch(1)".to_string()],
            bsd_linux_source: "Omarchy Fastfetch & Arch Linux custom tools".to_string(),
        });

        // Section 2 - System Calls
        self.register_page(SovereignManPage {
            title: "pledge".to_string(),
            section: ManSection::SystemCalls,
            synopsis: "int pledge(const char *promises, const char *execpromises);".to_string(),
            description: "OpenBSD-inspired system call restricting process privileges and kernel capability access.".to_string(),
            options: BTreeMap::new(),
            see_also: vec!["unveil(2)".to_string(), "seccomp(2)".to_string(), "landlock(7)".to_string()],
            bsd_linux_source: "OpenBSD pledge(2)".to_string(),
        });

        self.register_page(SovereignManPage {
            title: "unveil".to_string(),
            section: ManSection::SystemCalls,
            synopsis: "int unveil(const char *path, const char *permissions);".to_string(),
            description: "Restricts filesystem visibility for the calling process to explicit path subsets.".to_string(),
            options: BTreeMap::new(),
            see_also: vec!["pledge(2)".to_string(), "landlock(7)".to_string()],
            bsd_linux_source: "OpenBSD unveil(2)".to_string(),
        });

        self.register_page(SovereignManPage {
            title: "seccomp".to_string(),
            section: ManSection::SystemCalls,
            synopsis: "int seccomp(unsigned int operation, unsigned int flags, void *args);".to_string(),
            description: "Linux Seccomp-BPF system call filtering mechanism for sandboxing isolated processes.".to_string(),
            options: BTreeMap::new(),
            see_also: vec!["pledge(2)".to_string(), "bpf(4)".to_string()],
            bsd_linux_source: "Linux seccomp(2)".to_string(),
        });

        // Section 3 - Library Functions
        self.register_page(SovereignManPage {
            title: "cap_rights_limit".to_string(),
            section: ManSection::LibraryFunctions,
            synopsis: "int cap_rights_limit(int fd, const cap_rights_t *rights);".to_string(),
            description: "FreeBSD Capsicum capability rights limitation for file descriptors.".to_string(),
            options: BTreeMap::new(),
            see_also: vec!["pledge(2)".to_string(), "rights(4)".to_string()],
            bsd_linux_source: "FreeBSD Capsicum cap_rights_limit(3)".to_string(),
        });

        self.register_page(SovereignManPage {
            title: "gettext".to_string(),
            section: ManSection::LibraryFunctions,
            synopsis: "char *gettext(const char *msgid);".to_string(),
            description: "Looks up localized message strings in standard message catalogs.".to_string(),
            options: BTreeMap::new(),
            see_also: vec!["catgets(3)".to_string(), "ngettext(3)".to_string()],
            bsd_linux_source: "GNU gettext(3) & FreeBSD catgets(3)".to_string(),
        });

        // Section 4 - Special Files
        self.register_page(SovereignManPage {
            title: "bpf".to_string(),
            section: ManSection::SpecialFiles,
            synopsis: "/dev/bpf".to_string(),
            description: "Berkeley Packet Filter device node for raw link-layer packet capturing.".to_string(),
            options: BTreeMap::new(),
            see_also: vec!["pcap(3)".to_string(), "netintro(4)".to_string()],
            bsd_linux_source: "FreeBSD & OpenBSD bpf(4)".to_string(),
        });

        self.register_page(SovereignManPage {
            title: "zfs".to_string(),
            section: ManSection::SpecialFiles,
            synopsis: "/dev/zfs".to_string(),
            description: "OpenZFS control device providing pool management, datasets, and snapshot transactions.".to_string(),
            options: BTreeMap::new(),
            see_also: vec!["zpool(8)".to_string(), "bectl(8)".to_string()],
            bsd_linux_source: "OpenZFS / FreeBSD zfs(4)".to_string(),
        });

        // Section 5 - File Formats
        self.register_page(SovereignManPage {
            title: "fstab".to_string(),
            section: ManSection::FileFormats,
            synopsis: "/etc/fstab".to_string(),
            description: "Static filesystem information table specifying mount points and options.".to_string(),
            options: BTreeMap::new(),
            see_also: vec!["mount(8)".to_string(), "swapon(8)".to_string()],
            bsd_linux_source: "POSIX & NetBSD fstab(5)".to_string(),
        });

        self.register_page(SovereignManPage {
            title: "pacman.conf".to_string(),
            section: ManSection::FileFormats,
            synopsis: "/etc/pacman.conf".to_string(),
            description: "Package manager configuration file specifying repositories, mirrors, and signature verification options.".to_string(),
            options: BTreeMap::new(),
            see_also: vec!["sigma-pkg(8)".to_string(), "pkg.conf(5)".to_string()],
            bsd_linux_source: "Arch Linux pacman.conf(5)".to_string(),
        });

        // Section 6 - Games
        self.register_page(SovereignManPage {
            title: "sigma-quest".to_string(),
            section: ManSection::Games,
            synopsis: "sigma-quest".to_string(),
            description: "Ncurses terminal adventure game demonstrating kernel syscall and audio subsystem features.".to_string(),
            options: BTreeMap::new(),
            see_also: vec!["ncurses(3)".to_string()],
            bsd_linux_source: "BSD Games tradition (rogue/hack)".to_string(),
        });

        // Section 7 - Miscellaneous
        self.register_page(SovereignManPage {
            title: "cgroups".to_string(),
            section: ManSection::Miscellaneous,
            synopsis: "cgroup v2 hierarchy".to_string(),
            description: "Linux control groups v2 resource management and container isolation.".to_string(),
            options: BTreeMap::new(),
            see_also: vec!["systemd(1)".to_string(), "landlock(7)".to_string()],
            bsd_linux_source: "Linux kernel cgroups(7)".to_string(),
        });

        self.register_page(SovereignManPage {
            title: "pqc_signatures".to_string(),
            section: ManSection::Miscellaneous,
            synopsis: "Post-Quantum Cryptographic Attestation Specification".to_string(),
            description: "Dilithium5 and Signify post-quantum digital signature standards used for package and kernel module attestation.".to_string(),
            options: BTreeMap::new(),
            see_also: vec!["sigma-pkg(8)".to_string(), "signify(1)".to_string()],
            bsd_linux_source: "OpenBSD signify(1) & NIST PQC Standard".to_string(),
        });

        // Section 8 - System Admin
        self.register_page(SovereignManPage {
            title: "sigma-pkg".to_string(),
            section: ManSection::SystemAdmin,
            synopsis: "sigma-pkg [OPTIONS] COMMAND [ARGS]...".to_string(),
            description: "Universal package manager for SigmaOS combining FreeBSD pkg, Arch pacman, Debian apt, and Nix CAS store.".to_string(),
            options: {
                let mut opts = BTreeMap::new();
                opts.insert("-i, --install".to_string(), "Install package with PQC attestation".to_string());
                opts.insert("-S, --sync".to_string(), "Synchronize repository databases".to_string());
                opts.insert("-u, --update".to_string(), "Perform transactional rootfs update".to_string());
                opts
            },
            see_also: vec!["pacman(8)".to_string(), "pkg(8)".to_string(), "apt(8)".to_string()],
            bsd_linux_source: "FreeBSD pkg(8) & Arch Linux pacman(8) hybrid".to_string(),
        });

        self.register_page(SovereignManPage {
            title: "bectl".to_string(),
            section: ManSection::SystemAdmin,
            synopsis: "bectl [subcommand] [args]".to_string(),
            description: "ZFS boot environment management utility for creating, mounting, and rolling back boot snapshots.".to_string(),
            options: {
                let mut opts = BTreeMap::new();
                opts.insert("create".to_string(), "Create a new boot environment snapshot".to_string());
                opts.insert("activate".to_string(), "Set specified boot environment as default for next boot".to_string());
                opts
            },
            see_also: vec!["zfs(8)".to_string(), "snapper(8)".to_string()],
            bsd_linux_source: "FreeBSD bectl(8) & Illumos beadm(1)".to_string(),
        });

        // Section 9 - Kernel Routines
        self.register_page(SovereignManPage {
            title: "vfs".to_string(),
            section: ManSection::KernelRoutines,
            synopsis: "int vfs_mount(struct mount *mp, ...);".to_string(),
            description: "Virtual Filesystem interface and inode/vnode dispatch layer.".to_string(),
            options: BTreeMap::new(),
            see_also: vec!["vnode(9)".to_string(), "mount(8)".to_string()],
            bsd_linux_source: "FreeBSD & DragonFly BSD vfs(9)".to_string(),
        });

        self.register_page(SovereignManPage {
            title: "buddy_allocator".to_string(),
            section: ManSection::KernelRoutines,
            synopsis: "void *buddy_alloc(size_t order);".to_string(),
            description: "Physical buddy memory allocator with anti-fragmentation page compaction passes.".to_string(),
            options: BTreeMap::new(),
            see_also: vec!["vfs(9)".to_string()],
            bsd_linux_source: "Linux mm buddy allocator & FreeBSD vm_page".to_string(),
        });
    }

    pub fn register_page(&mut self, page: SovereignManPage) {
        let key = format!("{}({})", page.title, page.section as u8);
        self.pages.insert(key, page);
    }

    pub fn search(&self, query: &str) -> Vec<&SovereignManPage> {
        self.pages
            .values()
            .filter(|p| p.title.contains(query) || p.description.contains(query))
            .collect()
    }

    pub fn generate_catalog_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("# Consolidated Man Pages Catalog\n\n");
        md.push_str("This catalog consolidates Unix/Linux/BSD manual pages across all 9 standard POSIX/BSD sections for SigmaOS.\n\n");

        md.push_str("## Registered Manual Pages\n\n");
        md.push_str("| Title | Section | Origin / Inspiration | Synopsis |\n");
        md.push_str("| :--- | :--- | :--- | :--- |\n");

        for page in self.pages.values() {
            md.push_str(&format!(
                "| **`{}`** | Section {} | {} | `{}` |\n",
                page.title, page.section as u8, page.bsd_linux_source, page.synopsis
            ));
        }

        md.push_str("\n---\n\n");
        md.push_str("## Detailed Manual Page Specifications\n\n");

        for page in self.pages.values() {
            md.push_str(&page.to_markdown());
            md.push_str("\n---\n\n");
        }

        md
    }
}

/// GNU Info Texinfo Node Structure
#[derive(Debug, Clone)]
pub struct SovereignInfoNode {
    pub title: String,
    pub node_name: String,
    pub up_node: String,
    pub next_node: String,
    pub prev_node: String,
    pub content: String,
    pub menu_items: Vec<(String, String)>,
}

impl SovereignInfoNode {
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str(&format!("# Info Node: {}\n\n", self.title));
        md.push_str(&format!(
            "**Node Navigation:** Up: `[{}]` | Next: `[{}]` | Prev: `[{}]`  \n\n",
            self.up_node, self.next_node, self.prev_node
        ));
        md.push_str(&self.content);
        md.push_str("\n\n");

        if !self.menu_items.is_empty() {
            md.push_str("## MENU\n");
            for (item, desc) in &self.menu_items {
                md.push_str(&format!("* * {}: {}\n", item, desc));
            }
            md.push('\n');
        }

        md
    }
}

/// Sovereign GNU Info Pages Consolidation Engine
#[derive(Debug, Default, Clone)]
pub struct SovereignInfoPagesConsolidationEngine {
    pub nodes: BTreeMap<String, SovereignInfoNode>,
}

impl SovereignInfoPagesConsolidationEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            nodes: BTreeMap::new(),
        };
        engine.populate_defaults();
        engine
    }

    fn populate_defaults(&mut self) {
        self.register_node(SovereignInfoNode {
            title: "SigmaOS Core Utilities".to_string(),
            node_name: "Top".to_string(),
            up_node: "(dir)".to_string(),
            next_node: "Kernel Architecture".to_string(),
            prev_node: "None".to_string(),
            content: "Welcome to the GNU Info Texinfo manual tree for SigmaOS.".to_string(),
            menu_items: vec![
                ("Kernel Architecture".to_string(), "SMP, eBPF, Buddy Allocator".to_string()),
                ("Packaging System".to_string(), "Universal package format and SAT solver".to_string()),
                ("Sandboxing Security".to_string(), "Pledge, Unveil, Capsicum, Landlock".to_string()),
                ("Storage and Filesystems".to_string(), "OpenZFS, Btrfs, and HAMMER2".to_string()),
            ],
        });

        self.register_node(SovereignInfoNode {
            title: "Kernel Architecture Details".to_string(),
            node_name: "Kernel Architecture".to_string(),
            up_node: "Top".to_string(),
            next_node: "Packaging System".to_string(),
            prev_node: "Top".to_string(),
            content: "Detailed documentation on SMP multi-core dispatch, eBPF JIT, and lockless memory queues.".to_string(),
            menu_items: vec![],
        });

        self.register_node(SovereignInfoNode {
            title: "Packaging System Manual".to_string(),
            node_name: "Packaging System".to_string(),
            up_node: "Top".to_string(),
            next_node: "Sandboxing Security".to_string(),
            prev_node: "Kernel Architecture".to_string(),
            content: "Comprehensive Texinfo documentation covering universal package ingestion across 100+ package formats and DPLL SAT dependency resolution.".to_string(),
            menu_items: vec![],
        });

        self.register_node(SovereignInfoNode {
            title: "Sandboxing Security Architecture".to_string(),
            node_name: "Sandboxing Security".to_string(),
            up_node: "Top".to_string(),
            next_node: "Storage and Filesystems".to_string(),
            prev_node: "Packaging System".to_string(),
            content: "Multi-layered sandboxing governor synthesizing OpenBSD pledge/unveil, FreeBSD Capsicum, and Linux Landlock/Seccomp-BPF.".to_string(),
            menu_items: vec![],
        });

        self.register_node(SovereignInfoNode {
            title: "Storage and Filesystems Handbook".to_string(),
            node_name: "Storage and Filesystems".to_string(),
            up_node: "Top".to_string(),
            next_node: "Top".to_string(),
            prev_node: "Sandboxing Security".to_string(),
            content: "High-reliability transactional storage management with OpenZFS pools, Btrfs subvolume Snapper snapshots, and DragonFly HAMMER2 PFS.".to_string(),
            menu_items: vec![],
        });
    }

    pub fn register_node(&mut self, node: SovereignInfoNode) {
        self.nodes.insert(node.node_name.clone(), node);
    }

    pub fn generate_tree_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("# GNU Info Pages Texinfo Navigation Tree\n\n");
        md.push_str("This document presents the consolidated GNU Info Texinfo documentation tree for SigmaOS, providing structured hierarchical navigation.\n\n");

        md.push_str("## Node Navigation Index\n\n");
        md.push_str("| Node Name | Title | Up Node | Next Node | Prev Node |\n");
        md.push_str("| :--- | :--- | :--- | :--- | :--- |\n");

        for node in self.nodes.values() {
            md.push_str(&format!(
                "| **`{}`** | {} | `{}` | `{}` | `{}` |\n",
                node.node_name, node.title, node.up_node, node.next_node, node.prev_node
            ));
        }

        md.push_str("\n---\n\n");
        md.push_str("## Detailed Texinfo Nodes\n\n");

        for node in self.nodes.values() {
            md.push_str(&node.to_markdown());
            md.push_str("\n---\n\n");
        }

        md
    }
}

/// /usr/share/doc Package Document Record
#[derive(Debug, Clone)]
pub struct SovereignPackageDocRecord {
    pub package_name: String,
    pub version: String,
    pub doc_path: String, // e.g. "/usr/share/doc/sigma-core/README.md"
    pub changelog: String,
    pub copyright: String,
    pub readme: String,
}

impl SovereignPackageDocRecord {
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str(&format!("# Package Documentation: {} (v{})\n\n", self.package_name, self.version));
        md.push_str(&format!("**Installed Path:** `{}`\n\n", self.doc_path));
        md.push_str("## README\n");
        md.push_str(&self.readme);
        md.push_str("\n\n## CHANGELOG\n");
        md.push_str(&self.changelog);
        md.push_str("\n\n## COPYRIGHT & LICENSE\n");
        md.push_str(&self.copyright);
        md.push('\n');
        md
    }
}

/// Sovereign /usr/share/doc Consolidation Engine
#[derive(Debug, Default, Clone)]
pub struct SovereignUsrShareDocConsolidationEngine {
    pub docs: BTreeMap<String, SovereignPackageDocRecord>,
}

impl SovereignUsrShareDocConsolidationEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            docs: BTreeMap::new(),
        };
        engine.populate_defaults();
        engine
    }

    fn populate_defaults(&mut self) {
        self.docs.insert(
            "sigma-core".to_string(),
            SovereignPackageDocRecord {
                package_name: "sigma-core".to_string(),
                version: "1.0.0".to_string(),
                doc_path: "/usr/share/doc/sigma-core".to_string(),
                changelog: "* 1.0.0: Initial release of sovereign kernel and tools.".to_string(),
                copyright: "Copyright (c) 2026 SigmaOS Project. MIT License.".to_string(),
                readme: "SigmaOS core utilities and system runtime documentation.".to_string(),
            },
        );

        self.docs.insert(
            "sigma-pkg".to_string(),
            SovereignPackageDocRecord {
                package_name: "sigma-pkg".to_string(),
                version: "1.0.0".to_string(),
                doc_path: "/usr/share/doc/sigma-pkg".to_string(),
                changelog: "* 1.0.0: Multi-format package ingestion and DPLL SAT solver.".to_string(),
                copyright: "Copyright (c) 2026 SigmaOS Project. MIT License.".to_string(),
                readme: "Universal package manager documentation and schema definition.".to_string(),
            },
        );

        self.docs.insert(
            "sigma-desktop".to_string(),
            SovereignPackageDocRecord {
                package_name: "sigma-desktop".to_string(),
                version: "1.0.0".to_string(),
                doc_path: "/usr/share/doc/sigma-desktop".to_string(),
                changelog: "* 1.0.0: Hyprland tiling compositor with Quickshell and Omakase themes.".to_string(),
                copyright: "Copyright (c) 2026 SigmaOS Project. MIT License.".to_string(),
                readme: "Zero-download sovereign desktop environment configuration guide.".to_string(),
            },
        );

        self.docs.insert(
            "sigma-security".to_string(),
            SovereignPackageDocRecord {
                package_name: "sigma-security".to_string(),
                version: "1.0.0".to_string(),
                doc_path: "/usr/share/doc/sigma-security".to_string(),
                changelog: "* 1.0.0: PQC Dilithium signatures & multi-sandbox policy governor.".to_string(),
                copyright: "Copyright (c) 2026 SigmaOS Project. MIT License.".to_string(),
                readme: "Security hardening guidelines, PQC attestation, and capability sandbox policies.".to_string(),
            },
        );
    }

    pub fn generate_registry_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("# /usr/share/doc Offline Package Docs Registry\n\n");
        md.push_str("This registry indexes offline package documentation records stored in `/usr/share/doc/` across installed SigmaOS system packages.\n\n");

        md.push_str("## Package Documentation Summary\n\n");
        md.push_str("| Package Name | Version | Doc Path | Summary |\n");
        md.push_str("| :--- | :--- | :--- | :--- |\n");

        for doc in self.docs.values() {
            md.push_str(&format!(
                "| **`{}`** | v{} | `{}` | {} |\n",
                doc.package_name, doc.version, doc.doc_path, doc.readme
            ));
        }

        md.push_str("\n---\n\n");
        md.push_str("## Individual Package Specifications\n\n");

        for doc in self.docs.values() {
            md.push_str(&doc.to_markdown());
            md.push_str("\n---\n\n");
        }

        md
    }
}

/// Arch Wiki / FreeBSD Handbook Style Article
#[derive(Debug, Clone)]
pub struct SovereignHandbookArticle {
    pub title: String,
    pub category: String,
    pub distro_inspiration: String,
    pub content: String,
    pub troubleshooting_steps: Vec<String>,
}

impl SovereignHandbookArticle {
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str(&format!("# {}\n\n", self.title));
        md.push_str(&format!("**Category:** {} | **Inspiration:** {}\n\n", self.category, self.distro_inspiration));
        md.push_str(&self.content);
        md.push_str("\n\n");

        if !self.troubleshooting_steps.is_empty() {
            md.push_str("## Troubleshooting & FAQ\n");
            for step in &self.troubleshooting_steps {
                md.push_str(&format!("1. {}\n", step));
            }
            md.push('\n');
        }

        md
    }
}

/// Sovereign Wiki & Handbook Consolidation Engine
#[derive(Debug, Default, Clone)]
pub struct SovereignWikiHandbookConsolidationEngine {
    pub articles: BTreeMap<String, SovereignHandbookArticle>,
}

impl SovereignWikiHandbookConsolidationEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            articles: BTreeMap::new(),
        };
        engine.populate_defaults();
        engine
    }

    fn populate_defaults(&mut self) {
        self.articles.insert(
            "Installation-Guide".to_string(),
            SovereignHandbookArticle {
                title: "SigmaOS Installation & Deployment Guide".to_string(),
                category: "System Administration".to_string(),
                distro_inspiration: "Arch Linux Wiki & FreeBSD Handbook".to_string(),
                content: "Step-by-step installation guide for bare-metal, QEMU/KVM, and cloud instances.".to_string(),
                troubleshooting_steps: vec![
                    "Check UEFI Secure Boot status if bootloader fails to handoff.".to_string(),
                    "Verify ZFS/Btrfs subvolume mounting flags in /etc/fstab.".to_string(),
                ],
            },
        );

        self.articles.insert(
            "Package-Management".to_string(),
            SovereignHandbookArticle {
                title: "Universal Package Management Handbook".to_string(),
                category: "Software Management".to_string(),
                distro_inspiration: "Arch Linux pacman Wiki & FreeBSD pkg Handbook".to_string(),
                content: "Comprehensive guide to managing 100+ package formats via sigma-pkg and DPLL constraint resolution.".to_string(),
                troubleshooting_steps: vec![
                    "Run `sigma-pkg --verify-signatures` if PQC attestation fails.".to_string(),
                    "Use `sigma-pkg --rollback` to revert to previous boot environment snapshot.".to_string(),
                ],
            },
        );

        self.articles.insert(
            "Security-Hardening-Guide".to_string(),
            SovereignHandbookArticle {
                title: "Security Hardening & Multi-Sandbox Policy Guide".to_string(),
                category: "Security & Auditing".to_string(),
                distro_inspiration: "OpenBSD Security Handbook & HardenedBSD".to_string(),
                content: "Complete guide to process isolation, pledge/unveil restrictions, FreeBSD Capsicum capabilities, and Linux Landlock v5 policies.".to_string(),
                troubleshooting_steps: vec![
                    "Inspect audit log via `sigma-ctl status security-audit` for pledge violation alerts.".to_string(),
                    "Verify memory protection status with `sigma-ctl status kernel-hardening`.".to_string(),
                ],
            },
        );

        self.articles.insert(
            "Kernel-Compilation-Handbook".to_string(),
            SovereignHandbookArticle {
                title: "Kernel Compilation & Custom Module Build Handbook".to_string(),
                category: "Kernel Development".to_string(),
                distro_inspiration: "Gentoo Kernel Handbook & Debian Kernel Guide".to_string(),
                content: "Guide to compiling sovereign kernel images, configuring microarchitecture optimization flags (x86-64 v1..v4/ARM), and building out-of-tree drivers.".to_string(),
                troubleshooting_steps: vec![
                    "Ensure target ISA flags match hardware capability (`sigma-fetch --json`).".to_string(),
                    "Check initramfs microcode splicing if early boot panics occur.".to_string(),
                ],
            },
        );
    }

    pub fn generate_handbook_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("# Consolidated OS Documentation & Handbook System\n\n");
        md.push_str("Inspired by the **Arch Linux Wiki** and **FreeBSD Handbook**, this single-source-of-truth handbook provides comprehensive guides for operating and administering SigmaOS.\n\n");

        md.push_str("## Handbook Articles Index\n\n");
        md.push_str("| Article Title | Category | Distro Inspiration |\n");
        md.push_str("| :--- | :--- | :--- |\n");

        for article in self.articles.values() {
            md.push_str(&format!(
                "| **{}** | {} | {} |\n",
                article.title, article.category, article.distro_inspiration
            ));
        }

        md.push_str("\n---\n\n");
        md.push_str("## Detailed Handbook Articles\n\n");

        for article in self.articles.values() {
            md.push_str(&article.to_markdown());
            md.push_str("\n---\n\n");
        }

        md
    }
}

/// Master Gateway for Documentation & GitHub Wiki Consolidation
#[derive(Debug, Default, Clone)]
pub struct SovereignDocumentationConsolidationPrGateway {
    pub man_engine: SovereignManPagesConsolidationEngine,
    pub info_engine: SovereignInfoPagesConsolidationEngine,
    pub doc_engine: SovereignUsrShareDocConsolidationEngine,
    pub wiki_engine: SovereignWikiHandbookConsolidationEngine,
}

impl SovereignDocumentationConsolidationPrGateway {
    pub fn new() -> Self {
        Self {
            man_engine: SovereignManPagesConsolidationEngine::new(),
            info_engine: SovereignInfoPagesConsolidationEngine::new(),
            doc_engine: SovereignUsrShareDocConsolidationEngine::new(),
            wiki_engine: SovereignWikiHandbookConsolidationEngine::new(),
        }
    }

    pub fn generate_consolidated_pr_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("# Pull Request Proposal: Consolidated Operating System Documentation & GitHub Wiki Engine\n\n");
        md.push_str("## Summary & Objective\n");
        md.push_str("This Pull Request Proposal consolidates all disparate operating system documentation sources—including Unix/Linux/BSD **man pages**, **GNU Info pages**, **/usr/share/doc** offline package guides, and **Wiki/Handbooks** (Arch Wiki & FreeBSD Handbook)—into a unified, searchable, single-source-of-truth GitHub Wiki architecture for SigmaOS.\n\n");

        md.push_str("## Architectural Matrix of Consolidated Documentation\n\n");
        md.push_str("| Source Documentation Type | Distro Inspiration | Target GitHub Wiki Format | Consolidated Feature |\n");
        md.push_str("| :--- | :--- | :--- | :--- |\n");
        md.push_str("| **Man Pages (Sections 1..9)** | FreeBSD / OpenBSD / Arch / Debian | `wiki/Man-Pages-Catalog.md` | Standardized troff/mdoc markdown rendering |\n");
        md.push_str("| **GNU Info Pages** | GNU / Debian Texinfo | `wiki/Info-Pages-Tree.md` | Hierarchical node navigation & menu linking |\n");
        md.push_str("| **/usr/share/doc** | Debian / Fedora / Gentoo | `wiki/Package-Docs-Registry.md` | Offline package READMEs, copyright, changelogs |\n");
        md.push_str("| **Arch Wiki & FreeBSD Handbook** | Arch Linux & FreeBSD | `wiki/Consolidated-OS-Documentation-System.md` | Comprehensive system administration & guides |\n\n");

        md.push_str("## Summary Statistics\n");
        md.push_str(&format!("* **Man Pages Registered:** {}\n", self.man_engine.pages.len()));
        md.push_str(&format!("* **Info Nodes Registered:** {}\n", self.info_engine.nodes.len()));
        md.push_str(&format!("* **Package Docs Tracked:** {}\n", self.doc_engine.docs.len()));
        md.push_str(&format!("* **Handbook Articles:** {}\n\n", self.wiki_engine.articles.len()));

        md.push_str("## Verification & Quality Assurance\n");
        md.push_str("1. Execute `./run_sigma_tests.sh` to run tests for documentation engines.\n");
        md.push_str("2. Execute `scripts/sync_wiki.sh` to synchronize GitHub Wiki markdown pages across `WIKI/`, `wiki/`, and `wiki_repo/`.\n");

        md
    }

    /// Exports consolidated documentation Markdown files to the target directory (`wiki/`).
    pub fn export_all_wiki_pages<P: AsRef<Path>>(&self, target_dir: P) -> Result<Vec<String>, std::io::Error> {
        let dir = target_dir.as_ref();
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }

        let mut generated_files = Vec::new();

        // 1. Man Pages Catalog
        let man_path = dir.join("Man-Pages-Catalog.md");
        fs::write(&man_path, self.man_engine.generate_catalog_markdown())?;
        generated_files.push(man_path.to_string_lossy().into_owned());

        // 2. Info Pages Tree
        let info_path = dir.join("Info-Pages-Tree.md");
        fs::write(&info_path, self.info_engine.generate_tree_markdown())?;
        generated_files.push(info_path.to_string_lossy().into_owned());

        // 3. Package Docs Registry
        let doc_path = dir.join("Package-Docs-Registry.md");
        fs::write(&doc_path, self.doc_engine.generate_registry_markdown())?;
        generated_files.push(doc_path.to_string_lossy().into_owned());

        // 4. Consolidated OS Documentation System
        let wiki_path = dir.join("Consolidated-OS-Documentation-System.md");
        fs::write(&wiki_path, self.wiki_engine.generate_handbook_markdown())?;
        generated_files.push(wiki_path.to_string_lossy().into_owned());

        Ok(generated_files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_man_pages_consolidation() {
        let engine = SovereignManPagesConsolidationEngine::new();
        let pages = engine.search("sigma-pkg");
        assert!(!pages.is_empty());
        assert_eq!(pages[0].section, ManSection::SystemAdmin);
        let md = pages[0].to_markdown();
        assert!(md.contains("SYNOPSIS"));

        let catalog = engine.generate_catalog_markdown();
        assert!(catalog.contains("Consolidated Man Pages Catalog"));
        assert!(catalog.contains("sigma-sh"));
    }

    #[test]
    fn test_info_pages_consolidation() {
        let engine = SovereignInfoPagesConsolidationEngine::new();
        assert!(engine.nodes.contains_key("Top"));
        let node = &engine.nodes["Top"];
        let md = node.to_markdown();
        assert!(md.contains("Info Node:"));

        let tree = engine.generate_tree_markdown();
        assert!(tree.contains("GNU Info Pages Texinfo Navigation Tree"));
    }

    #[test]
    fn test_usr_share_doc_consolidation() {
        let engine = SovereignUsrShareDocConsolidationEngine::new();
        assert!(engine.docs.contains_key("sigma-core"));
        let rec = &engine.docs["sigma-core"];
        let md = rec.to_markdown();
        assert!(md.contains("sigma-core"));

        let registry = engine.generate_registry_markdown();
        assert!(registry.contains("/usr/share/doc Offline Package Docs Registry"));
    }

    #[test]
    fn test_wiki_handbook_consolidation() {
        let engine = SovereignWikiHandbookConsolidationEngine::new();
        assert!(engine.articles.contains_key("Installation-Guide"));
        let article = &engine.articles["Installation-Guide"];
        let md = article.to_markdown();
        assert!(md.contains("Installation & Deployment Guide"));

        let handbook = engine.generate_handbook_markdown();
        assert!(handbook.contains("Consolidated OS Documentation & Handbook System"));
    }

    #[test]
    fn test_pr_gateway_generation_and_export() {
        let gateway = SovereignDocumentationConsolidationPrGateway::new();
        let md = gateway.generate_consolidated_pr_markdown();
        assert!(md.contains("Pull Request Proposal"));
        assert!(md.contains("Architectural Matrix"));

        let temp_dir = std::env::temp_dir().join("sigma_doc_test_export");
        let exported = gateway.export_all_wiki_pages(&temp_dir).expect("Export failed");
        assert_eq!(exported.len(), 4);
        assert!(temp_dir.join("Man-Pages-Catalog.md").exists());
        assert!(temp_dir.join("Info-Pages-Tree.md").exists());
        assert!(temp_dir.join("Package-Docs-Registry.md").exists());
        assert!(temp_dir.join("Consolidated-OS-Documentation-System.md").exists());

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
