#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Sovereign Link Engine (Hard Links & Variant Symlinks)
// Inspired by Linux (link/unlink/linkat/symlinkat, atomic symlink swaps, ELOOP cycle protection)
// and DragonFly BSD / OpenBSD (Variant Symlinks - varsyms: $SYS, $ARCH, $USER, $ZONE expansion).

use std::string::{String, ToString};
use std::vec::Vec;
use std::vec;
use std::format;
use std::collections::{BTreeMap, BTreeSet};

pub const AT_FDCWD: i32 = -100;
pub const MAX_SYMLINK_DEPTH: usize = 12;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkType {
    HardLink { target_inode: u64 },
    SymLink { target_path: String },
    VariantSymLink { template_path: String }, // DragonFly BSD varsyms: /usr/obj/$ARCH
}

#[derive(Debug, Clone)]
pub struct InodeRecord {
    pub ino: u64,
    pub hard_link_count: u32,
    pub data_bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct DirectoryEntry {
    pub name: String,
    pub inode: u64,
    pub link_type: LinkType,
}

pub struct SovereignLinkEngine {
    pub inodes: BTreeMap<u64, InodeRecord>,
    pub vfs_entries: BTreeMap<String, DirectoryEntry>, // path -> dentry
    pub dirfds: BTreeMap<i32, String>,                 // dirfd -> dir_path
    pub varsym_vars: BTreeMap<String, String>,        // $SYS -> "Linux", $ARCH -> "x86_64"
    next_ino: u64,
}

impl SovereignLinkEngine {
    pub fn new() -> Self {
        let mut vars = BTreeMap::new();
        vars.insert(String::from("SYS"), String::from("SigmaOS"));
        vars.insert(String::from("ARCH"), String::from("x86_64"));
        vars.insert(String::from("USER"), String::from("root"));
        vars.insert(String::from("ZONE"), String::from("global"));

        let mut dirfds = BTreeMap::new();
        dirfds.insert(AT_FDCWD, String::from("/"));

        Self {
            inodes: BTreeMap::new(),
            vfs_entries: BTreeMap::new(),
            dirfds,
            varsym_vars: vars,
            next_ino: 1000,
        }
    }

    pub fn set_varsym(&mut self, key: &str, val: &str) {
        self.varsym_vars.insert(key.to_string(), val.to_string());
    }

    /// Creates a regular file inode and initial directory entry
    pub fn create_file(&mut self, path: &str, content: &[u8]) -> u64 {
        let ino = self.next_ino;
        self.next_ino += 1;

        let inode = InodeRecord {
            ino,
            hard_link_count: 1,
            data_bytes: content.to_vec(),
        };
        self.inodes.insert(ino, inode);

        let dentry = DirectoryEntry {
            name: path.to_string(),
            inode: ino,
            link_type: LinkType::HardLink { target_inode: ino },
        };
        self.vfs_entries.insert(path.to_string(), dentry);

        ino
    }

    /// Hard Link creation (link / linkat parity)
    pub fn create_hard_link(&mut self, old_path: &str, new_path: &str) -> Result<(), String> {
        let old_dentry = self.vfs_entries.get(old_path).ok_or_else(|| format!("ENOENT: Old path {} not found", old_path))?;
        let target_ino = old_dentry.inode;

        let inode = self.inodes.get_mut(&target_ino).ok_or_else(|| format!("ENOENT: Inode {} not found", target_ino))?;
        inode.hard_link_count += 1;

        let new_dentry = DirectoryEntry {
            name: new_path.to_string(),
            inode: target_ino,
            link_type: LinkType::HardLink { target_inode: target_ino },
        };
        self.vfs_entries.insert(new_path.to_string(), new_dentry);

        Ok(())
    }

    /// Unlink (hard link deletion & inode cleanup)
    pub fn unlink(&mut self, path: &str) -> Result<(), String> {
        let dentry = self.vfs_entries.remove(path).ok_or_else(|| format!("ENOENT: Path {} not found", path))?;

        if let Some(inode) = self.inodes.get_mut(&dentry.inode) {
            if inode.hard_link_count > 0 {
                inode.hard_link_count -= 1;
            }
            if inode.hard_link_count == 0 {
                self.inodes.remove(&dentry.inode); // Free inode resources
            }
        }

        Ok(())
    }

    /// Symbolic Link creation (symlink / symlinkat parity)
    pub fn create_symlink(&mut self, target_path: &str, link_path: &str) -> Result<(), String> {
        let ino = self.next_ino;
        self.next_ino += 1;

        let dentry = DirectoryEntry {
            name: link_path.to_string(),
            inode: ino,
            link_type: LinkType::SymLink {
                target_path: target_path.to_string(),
            },
        };

        self.vfs_entries.insert(link_path.to_string(), dentry);
        Ok(())
    }

    /// DragonFly BSD / OpenBSD Variant Symlink creation (varsyms)
    pub fn create_variant_symlink(&mut self, template_path: &str, link_path: &str) -> Result<(), String> {
        let ino = self.next_ino;
        self.next_ino += 1;

        let dentry = DirectoryEntry {
            name: link_path.to_string(),
            inode: ino,
            link_type: LinkType::VariantSymLink {
                template_path: template_path.to_string(),
            },
        };

        self.vfs_entries.insert(link_path.to_string(), dentry);
        Ok(())
    }

    /// Atomic Symlink Swap (updates symlink target atomically without broken window)
    pub fn swap_symlink_atomic(&mut self, link_path: &str, new_target: &str) -> Result<(), String> {
        let dentry = self.vfs_entries.get_mut(link_path).ok_or_else(|| format!("ENOENT: Symlink {} not found", link_path))?;
        dentry.link_type = LinkType::SymLink {
            target_path: new_target.to_string(),
        };
        Ok(())
    }

    /// Resolves DragonFly BSD Variant Symlink variables ($SYS, $ARCH, $USER, $ZONE)
    pub fn expand_varsym(&self, template: &str) -> String {
        let mut result = template.to_string();
        for (k, v) in &self.varsym_vars {
            let var_key = format!("${}", k);
            result = result.replace(&var_key, v);
        }
        result
    }

    /// Symlink path resolution with ELOOP cycle detection
    pub fn resolve_path(&self, path: &str) -> Result<String, String> {
        let mut current_path = path.to_string();
        let mut visited = BTreeSet::new();
        let mut depth = 0;

        loop {
            if depth >= MAX_SYMLINK_DEPTH {
                return Err(format!("ELOOP: Excessive symlink recursion level ({})", depth));
            }

            if visited.contains(&current_path) {
                return Err(format!("ELOOP: Symlink loop detected for path: {}", current_path));
            }

            visited.insert(current_path.clone());

            if let Some(dentry) = self.vfs_entries.get(&current_path) {
                match &dentry.link_type {
                    LinkType::HardLink { .. } => return Ok(current_path),
                    LinkType::SymLink { target_path } => {
                        current_path = target_path.clone();
                        depth += 1;
                    }
                    LinkType::VariantSymLink { template_path } => {
                        current_path = self.expand_varsym(template_path);
                        depth += 1;
                    }
                }
            } else {
                return Ok(current_path); // Terminal path
            }
        }
    }
}

impl Default for SovereignLinkEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_hard_link_creation_and_unlinking() {
        let mut engine = SovereignLinkEngine::new();
        let ino = engine.create_file("/var/log/syslog", b"log_data");
        assert_eq!(engine.inodes.get(&ino).unwrap().hard_link_count, 1);

        engine.create_hard_link("/var/log/syslog", "/var/log/syslog.hard").unwrap();
        assert_eq!(engine.inodes.get(&ino).unwrap().hard_link_count, 2);

        engine.unlink("/var/log/syslog").unwrap();
        assert_eq!(engine.inodes.get(&ino).unwrap().hard_link_count, 1);
        assert!(engine.inodes.contains_key(&ino));

        engine.unlink("/var/log/syslog.hard").unwrap();
        assert!(!engine.inodes.contains_key(&ino)); // Freed on 0 ref count
    }

    #[test]
    fn test_variant_symlinks_varsyms() {
        let mut engine = SovereignLinkEngine::new();
        engine.create_file("/lib/x86_64/libc.so", b"elf_data");
        engine.create_variant_symlink("/lib/$ARCH/libc.so", "/lib/libc.so").unwrap();

        let resolved = engine.resolve_path("/lib/libc.so").unwrap();
        assert_eq!(resolved, "/lib/x86_64/libc.so");
    }

    #[test]
    fn test_symlink_loop_detection_eloop() {
        let mut engine = SovereignLinkEngine::new();
        engine.create_symlink("/link_b", "/link_a").unwrap();
        engine.create_symlink("/link_a", "/link_b").unwrap();

        let res = engine.resolve_path("/link_a");
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("ELOOP"));
    }

    #[test]
    fn test_atomic_symlink_swap() {
        let mut engine = SovereignLinkEngine::new();
        engine.create_symlink("/v1/app", "/app/current").unwrap();
        assert_eq!(engine.resolve_path("/app/current").unwrap(), "/v1/app");

        engine.swap_symlink_atomic("/app/current", "/v2/app").unwrap();
        assert_eq!(engine.resolve_path("/app/current").unwrap(), "/v2/app");
    }
}
