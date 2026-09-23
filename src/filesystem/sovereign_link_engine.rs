// SigmaOS Sovereign Link Engine (Hard Links & Variant Symlinks)
// Inspired by Linux (link/unlink/linkat/symlinkat, atomic symlink swaps, ELOOP cycle protection, fs.protected_hardlinks, CoW reflinks)
// and DragonFly BSD / OpenBSD / FreeBSD (Variant Symlinks - varsyms: $SYS, $ARCH, $USER, $ZONE, directory firmlinks, quota accounting).

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
    Reflink { shared_extent_id: u64 },        // Btrfs/XFS Copy-On-Write reflink
    DirectoryFirmlink { target_dir_inode: u64 }, // macOS/APFS style virtualized directory hard link
}

#[derive(Debug, Clone)]
pub struct InodeRecord {
    pub ino: u64,
    pub uid: u32,
    pub gid: u32,
    pub mode: u32, // POSIX permissions mode
    pub is_dir: bool,
    pub hard_link_count: u32,
    pub data_bytes: Vec<u8>,
    pub extent_id: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct DirectoryEntry {
    pub name: String,
    pub inode: u64,
    pub link_type: LinkType,
}

#[derive(Debug, Clone, Default)]
pub struct QuotaTracker {
    pub used_bytes: u64,
    pub used_inodes: u64,
    pub max_bytes: u64,
    pub max_inodes: u64,
}

/// Directed Acyclic Graph (DAG) Directory Link Engine
#[derive(Debug, Clone)]
pub struct AcyclicDirectoryGraphEngine {
    pub parent_child_map: BTreeMap<u64, BTreeSet<u64>>, // Parent Inode -> Set of Child Inodes
    pub node_depth_map: BTreeMap<u64, usize>,           // Inode -> Depth in DAG
}

impl AcyclicDirectoryGraphEngine {
    pub fn new() -> Self {
        Self {
            parent_child_map: BTreeMap::new(),
            node_depth_map: BTreeMap::new(),
        }
    }

    /// Check if target_ino is an ancestor of candidate_ino (cycle detection)
    pub fn is_ancestor(&self, candidate_ino: u64, target_ino: u64) -> bool {
        if candidate_ino == target_ino {
            return true;
        }

        let mut stack = vec![candidate_ino];
        let mut visited = BTreeSet::new();

        while let Some(current) = stack.pop() {
            if current == target_ino {
                return true;
            }

            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            if let Some(children) = self.parent_child_map.get(&current) {
                for &child in children {
                    if !visited.contains(&child) {
                        stack.push(child);
                    }
                }
            }
        }

        false
    }

    /// Add a directed edge from parent_ino to child_ino if it does not introduce a cycle
    pub fn add_directory_edge(&mut self, parent_ino: u64, child_ino: u64) -> Result<(), String> {
        if self.is_ancestor(child_ino, parent_ino) {
            return Err("ELOOP: Directory graph edge would create a cycle in DAG".to_string());
        }

        self.parent_child_map
            .entry(parent_ino)
            .or_default()
            .insert(child_ino);

        let parent_depth = *self.node_depth_map.get(&parent_ino).unwrap_or(&0);
        let current_child_depth = *self.node_depth_map.get(&child_ino).unwrap_or(&0);
        if parent_depth + 1 > current_child_depth {
            self.node_depth_map.insert(child_ino, parent_depth + 1);
        }

        Ok(())
    }

    /// Remove a directed edge from parent_ino to child_ino
    pub fn remove_directory_edge(&mut self, parent_ino: u64, child_ino: u64) -> bool {
        if let Some(children) = self.parent_child_map.get_mut(&parent_ino) {
            let removed = children.remove(&child_ino);
            if children.is_empty() {
                self.parent_child_map.remove(&parent_ino);
            }
            return removed;
        }
        false
    }

    /// Get current depth of node in DAG
    pub fn get_depth(&self, ino: u64) -> usize {
        *self.node_depth_map.get(&ino).unwrap_or(&0)
    }
}

impl Default for AcyclicDirectoryGraphEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SovereignLinkEngine {
    pub inodes: BTreeMap<u64, InodeRecord>,
    pub vfs_entries: BTreeMap<String, DirectoryEntry>, // path -> dentry
    pub dirfds: BTreeMap<i32, String>,                 // dirfd -> dir_path
    pub varsym_vars: BTreeMap<String, String>,        // $SYS -> "Linux", $ARCH -> "x86_64"
    pub user_quotas: BTreeMap<u32, QuotaTracker>,     // UID -> QuotaTracker
    pub protected_hardlinks_enabled: bool,           // Linux sysctl fs.protected_hardlinks
    next_ino: u64,
    next_extent: u64,
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
            user_quotas: BTreeMap::new(),
            protected_hardlinks_enabled: true,
            next_ino: 1000,
            next_extent: 5000,
        }
    }

    pub fn set_varsym(&mut self, key: &str, val: &str) {
        self.varsym_vars.insert(key.to_string(), val.to_string());
    }

    /// Sets user quota limits
    pub fn set_user_quota(&mut self, uid: u32, max_bytes: u64, max_inodes: u64) {
        let entry = self.user_quotas.entry(uid).or_default();
        entry.max_bytes = max_bytes;
        entry.max_inodes = max_inodes;
    }

    /// Creates a regular file inode and initial directory entry
    pub fn create_file(&mut self, path: &str, content: &[u8], uid: u32, gid: u32) -> Result<u64, String> {
        // Check Quota
        if let Some(quota) = self.user_quotas.get(&uid) {
            if quota.max_inodes > 0 && quota.used_inodes + 1 > quota.max_inodes {
                return Err("EDQUOT: Inode quota exceeded".to_string());
            }
            if quota.max_bytes > 0 && quota.used_bytes + content.len() as u64 > quota.max_bytes {
                return Err("EDQUOT: Disk space quota exceeded".to_string());
            }
        }

        let ino = self.next_ino;
        self.next_ino += 1;

        let extent_id = self.next_extent;
        self.next_extent += 1;

        let inode = InodeRecord {
            ino,
            uid,
            gid,
            mode: 0o644,
            is_dir: false,
            hard_link_count: 1,
            data_bytes: content.to_vec(),
            extent_id: Some(extent_id),
        };
        self.inodes.insert(ino, inode);

        let dentry = DirectoryEntry {
            name: path.to_string(),
            inode: ino,
            link_type: LinkType::HardLink { target_inode: ino },
        };
        self.vfs_entries.insert(path.to_string(), dentry);

        // Update Quota
        let quota = self.user_quotas.entry(uid).or_default();
        quota.used_inodes += 1;
        quota.used_bytes += content.len() as u64;

        Ok(ino)
    }

    /// Creates a directory entry
    pub fn create_directory(&mut self, path: &str, uid: u32, gid: u32) -> Result<u64, String> {
        let ino = self.next_ino;
        self.next_ino += 1;

        let inode = InodeRecord {
            ino,
            uid,
            gid,
            mode: 0o755,
            is_dir: true,
            hard_link_count: 2, // '.' and parent reference
            data_bytes: Vec::new(),
            extent_id: None,
        };
        self.inodes.insert(ino, inode);

        let dentry = DirectoryEntry {
            name: path.to_string(),
            inode: ino,
            link_type: LinkType::HardLink { target_inode: ino },
        };
        self.vfs_entries.insert(path.to_string(), dentry);

        Ok(ino)
    }

    /// Hard Link creation with Linux `fs.protected_hardlinks` security policy enforcement
    pub fn create_hard_link(&mut self, old_path: &str, new_path: &str, caller_uid: u32) -> Result<(), String> {
        let old_dentry = self.vfs_entries.get(old_path).ok_or_else(|| format!("ENOENT: Old path {} not found", old_path))?;
        let target_ino = old_dentry.inode;

        let inode = self.inodes.get(&target_ino).ok_or_else(|| format!("ENOENT: Inode {} not found", target_ino))?;

        if inode.is_dir {
            return Err("EPERM: Standard directory hard links forbidden to prevent cyclic filesystem graphs".to_string());
        }

        // Linux fs.protected_hardlinks check:
        // Caller must own the target file, OR target file must be regular/readable/writable/executable depending on S_ISUID/S_ISGID/unreadable
        if self.protected_hardlinks_enabled && caller_uid != 0 && caller_uid != inode.uid {
            if (inode.mode & 0o400 == 0) || (inode.mode & 0o6000 != 0) {
                return Err("EACCES: Hard link creation denied by fs.protected_hardlinks policy".to_string());
            }
        }

        let inode_mut = self.inodes.get_mut(&target_ino).unwrap();
        inode_mut.hard_link_count += 1;

        let new_dentry = DirectoryEntry {
            name: new_path.to_string(),
            inode: target_ino,
            link_type: LinkType::HardLink { target_inode: target_ino },
        };
        self.vfs_entries.insert(new_path.to_string(), new_dentry);

        Ok(())
    }

    /// Virtualized Directory Firmlink (APFS/macOS/BSD style safe directory linking)
    pub fn create_directory_firmlink(&mut self, target_dir_path: &str, link_path: &str) -> Result<(), String> {
        let target_dentry = self.vfs_entries.get(target_dir_path).ok_or_else(|| format!("ENOENT: Path {} not found", target_dir_path))?;
        let target_ino = target_dentry.inode;

        let target_inode = self.inodes.get(&target_ino).ok_or_else(|| format!("ENOENT: Inode {} not found", target_ino))?;
        if !target_inode.is_dir {
            return Err("ENOTDIR: Target path is not a directory for firmlink".to_string());
        }

        // Prevent direct recursive loop (link path cannot be prefix of target or vice versa)
        if target_dir_path.starts_with(link_path) || link_path.starts_with(target_dir_path) {
            return Err("ELOOP: Firmlink would create directory graph cycle".to_string());
        }

        let link_ino = self.next_ino;
        self.next_ino += 1;

        let dentry = DirectoryEntry {
            name: link_path.to_string(),
            inode: link_ino,
            link_type: LinkType::DirectoryFirmlink { target_dir_inode: target_ino },
        };

        self.vfs_entries.insert(link_path.to_string(), dentry);
        Ok(())
    }

    /// Btrfs/XFS style Copy-On-Write Reflink creation
    pub fn create_reflink(&mut self, src_path: &str, dest_path: &str, caller_uid: u32) -> Result<(), String> {
        let src_dentry = self.vfs_entries.get(src_path).ok_or_else(|| format!("ENOENT: Source path {} not found", src_path))?;
        let src_inode = self.inodes.get(&src_dentry.inode).ok_or_else(|| format!("ENOENT: Source inode not found"))?;

        if src_inode.is_dir {
            return Err("EISDIR: Reflinks cannot be created directly on directory inodes".to_string());
        }

        let extent_id = src_inode.extent_id.ok_or_else(|| "EINVAL: Source inode has no backing extent".to_string())?;

        let dest_ino = self.next_ino;
        self.next_ino += 1;

        let new_inode = InodeRecord {
            ino: dest_ino,
            uid: caller_uid,
            gid: src_inode.gid,
            mode: src_inode.mode,
            is_dir: false,
            hard_link_count: 1,
            data_bytes: src_inode.data_bytes.clone(), // Logical CoW clone
            extent_id: Some(extent_id),
        };

        self.inodes.insert(dest_ino, new_inode);

        let dentry = DirectoryEntry {
            name: dest_path.to_string(),
            inode: dest_ino,
            link_type: LinkType::Reflink { shared_extent_id: extent_id },
        };

        self.vfs_entries.insert(dest_path.to_string(), dentry);
        Ok(())
    }

    /// Write to file with CoW extent detachment if shared reflink
    pub fn write_file_cow(&mut self, path: &str, new_data: &[u8]) -> Result<(), String> {
        let dentry = self.vfs_entries.get(path).ok_or_else(|| format!("ENOENT: Path {} not found", path))?;
        let ino = dentry.inode;

        let inode = self.inodes.get_mut(&ino).ok_or_else(|| format!("ENOENT: Inode {} not found", ino))?;

        // Detach extent if reflink CoW write occurs
        if let Some(old_extent) = inode.extent_id {
            let mut count = 0;
            for in_rec in self.inodes.values() {
                if in_rec.extent_id == Some(old_extent) {
                    count += 1;
                }
            }
            if count > 1 {
                // Perform CoW allocation
                let new_extent = self.next_extent;
                self.next_extent += 1;
                inode.extent_id = Some(new_extent);
            }
        }

        let old_len = inode.data_bytes.len() as u64;
        let new_len = new_data.len() as u64;
        let uid = inode.uid;

        inode.data_bytes = new_data.to_vec();

        // Adjust disk space quota
        if let Some(quota) = self.user_quotas.get_mut(&uid) {
            if new_len > old_len {
                quota.used_bytes += new_len - old_len;
            } else {
                quota.used_bytes = quota.used_bytes.saturating_sub(old_len - new_len);
            }
        }

        Ok(())
    }

    /// Unlink (hard link deletion & inode/quota cleanup)
    pub fn unlink(&mut self, path: &str) -> Result<(), String> {
        let dentry = self.vfs_entries.remove(path).ok_or_else(|| format!("ENOENT: Path {} not found", path))?;

        if let Some(inode) = self.inodes.get_mut(&dentry.inode) {
            if inode.hard_link_count > 0 {
                inode.hard_link_count -= 1;
            }
            if inode.hard_link_count == 0 {
                let bytes = inode.data_bytes.len() as u64;
                let uid = inode.uid;
                self.inodes.remove(&dentry.inode); // Free inode resources

                // Update Quota
                if let Some(quota) = self.user_quotas.get_mut(&uid) {
                    quota.used_inodes = quota.used_inodes.saturating_sub(1);
                    quota.used_bytes = quota.used_bytes.saturating_sub(bytes);
                }
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

    /// Symlink path resolution with ELOOP cycle detection and firmlink expansion
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
                    LinkType::HardLink { .. } | LinkType::Reflink { .. } => return Ok(current_path),
                    LinkType::DirectoryFirmlink { target_dir_inode } => {
                        let target_entry = self.vfs_entries.values().find(|e| e.inode == *target_dir_inode);
                        if let Some(target) = target_entry {
                            current_path = target.name.clone();
                            depth += 1;
                        } else {
                            return Ok(current_path);
                        }
                    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hard_link_creation_and_unlinking() {
        let mut engine = SovereignLinkEngine::new();
        let ino = engine.create_file("/var/log/syslog", b"log_data", 0, 0).unwrap();
        assert_eq!(engine.inodes.get(&ino).unwrap().hard_link_count, 1);

        engine.create_hard_link("/var/log/syslog", "/var/log/syslog.hard", 0).unwrap();
        assert_eq!(engine.inodes.get(&ino).unwrap().hard_link_count, 2);

        engine.unlink("/var/log/syslog").unwrap();
        assert_eq!(engine.inodes.get(&ino).unwrap().hard_link_count, 1);
        assert!(engine.inodes.contains_key(&ino));

        engine.unlink("/var/log/syslog.hard").unwrap();
        assert!(!engine.inodes.contains_key(&ino)); // Freed on 0 ref count
    }

    #[test]
    fn test_protected_hardlinks_policy() {
        let mut engine = SovereignLinkEngine::new();
        let _ino = engine.create_file("/etc/shadow", b"secret", 0, 0).unwrap();

        // Set mode to S_ISUID (0o4600)
        engine.inodes.get_mut(&_ino).unwrap().mode = 0o4600;

        // Non-root caller attempting hard link to root setuid file
        let res = engine.create_hard_link("/etc/shadow", "/tmp/shadow_link", 1001);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("EACCES"));
    }

    #[test]
    fn test_reflink_cow_detachment() {
        let mut engine = SovereignLinkEngine::new();
        let ino1 = engine.create_file("/source.img", b"original data", 1000, 1000).unwrap();
        engine.create_reflink("/source.img", "/clone.img", 1000).unwrap();

        let clone_dentry = engine.vfs_entries.get("/clone.img").unwrap();
        let ino2 = clone_dentry.inode;

        let ext1 = engine.inodes.get(&ino1).unwrap().extent_id;
        let ext2 = engine.inodes.get(&ino2).unwrap().extent_id;
        assert_eq!(ext1, ext2); // Shared extent initially

        // Write to clone triggers CoW detachment
        engine.write_file_cow("/clone.img", b"modified clone data").unwrap();

        let ext1_after = engine.inodes.get(&ino1).unwrap().extent_id;
        let ext2_after = engine.inodes.get(&ino2).unwrap().extent_id;
        assert_ne!(ext1_after, ext2_after); // Detached extent after write
    }

    #[test]
    fn test_user_quota_accounting() {
        let mut engine = SovereignLinkEngine::new();
        engine.set_user_quota(1002, 100, 2); // Max 100 bytes, 2 inodes

        engine.create_file("/file1", b"hello", 1002, 1002).unwrap();
        engine.create_file("/file2", b"world", 1002, 1002).unwrap();

        // Exceed inode quota
        let res = engine.create_file("/file3", b"extra", 1002, 1002);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("EDQUOT"));
    }

    #[test]
    fn test_variant_symlinks_varsyms() {
        let mut engine = SovereignLinkEngine::new();
        engine.create_file("/lib/x86_64/libc.so", b"elf_data", 0, 0).unwrap();
        engine.create_variant_symlink("/lib/$ARCH/libc.so", "/lib/libc.so").unwrap();

        let resolved = engine.resolve_path("/lib/libc.so").unwrap();
        assert_eq!(resolved, "/lib/x86_64/libc.so");
    }

    #[test]
    fn test_directory_firmlink() {
        let mut engine = SovereignLinkEngine::new();
        engine.create_directory("/Users/Shared", 0, 0).unwrap();
        engine.create_directory_firmlink("/Users/Shared", "/System/Volumes/Data/Shared").unwrap();

        let res = engine.resolve_path("/System/Volumes/Data/Shared").unwrap();
        assert_eq!(res, "/Users/Shared");
    }

    #[test]
    fn test_acyclic_directory_graph_engine() {
        let mut graph = AcyclicDirectoryGraphEngine::new();

        // 1. Add valid parent -> child edges (root -> dir1 -> dir2)
        assert!(graph.add_directory_edge(1, 10).is_ok());
        assert!(graph.add_directory_edge(10, 20).is_ok());

        assert_eq!(graph.get_depth(10), 1);
        assert_eq!(graph.get_depth(20), 2);

        // 2. Cycle detection: Attempting to link dir2 -> root should fail
        assert!(graph.add_directory_edge(20, 1).is_err());

        // 3. Remove edge
        assert!(graph.remove_directory_edge(10, 20));
        assert!(!graph.is_ancestor(10, 20));
    }
}
