//! Sovereign Microsoft PowerToys replication suite for SigmaOS
//! Implements a rich suite of system and workspace utilities:
//! - ColorPicker (HEX / RGB screen color inspector)
//! - FancyZones (Coordinates, tiling window layout grid arranger)
//! - PowerRename (Bulk filename batch regular-expression renamer)
//! - FileLocksmith (Tracks active process IDs holding file descriptor locks)
//! - HostsEditor (IP lookup hosts custom DNS routing rule editor)

use crate::klib::{Vec, HashMap};

pub struct ColorPicker;

impl ColorPicker {
    pub fn new() -> Self {
        Self
    }

    /// Converts standard RGB color values into hex code strings
    pub fn rgb_to_hex(&self, r: u8, g: u8, b: u8) -> [u8; 7] {
        let mut hex = [0u8; 7];
        hex[0] = b'#';

        const CHARS: &[u8] = b"0123456789ABCDEF";
        hex[1] = CHARS[(r as usize >> 4) & 0x0F];
        hex[2] = CHARS[r as usize & 0x0F];

        hex[3] = CHARS[(g as usize >> 4) & 0x0F];
        hex[4] = CHARS[g as usize & 0x0F];

        hex[5] = CHARS[(b as usize >> 4) & 0x0F];
        hex[6] = CHARS[b as usize & 0x0F];

        hex
    }
}

pub struct ScreenZone {
    pub id: usize,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub struct FancyZones {
    pub zones: Vec<ScreenZone>,
}

impl FancyZones {
    pub fn new() -> Self {
        Self { zones: Vec::new() }
    }

    /// Configures standard coordinates grid splits
    pub fn create_split_layout(&mut self, screen_width: u32, screen_height: u32) {
        self.zones = Vec::new();
        let half_w = screen_width / 2;

        // Zone 0: Left side
        self.zones.push(ScreenZone {
            id: 0,
            x: 0,
            y: 0,
            width: half_w,
            height: screen_height,
        });

        // Zone 1: Right side
        self.zones.push(ScreenZone {
            id: 1,
            x: half_w,
            y: 0,
            width: half_w,
            height: screen_height,
        });
    }

    /// Returns the matched zone ID for a window position
    pub fn snap_window(&self, x: u32, y: u32) -> Option<usize> {
        for i in 0..self.zones.len() {
            let zone = &self.zones[i];
            if x >= zone.x && x < zone.x + zone.width && y >= zone.y && y < zone.y + zone.height {
                return Some(zone.id);
            }
        }
        None
    }
}

pub struct PowerRename;

impl PowerRename {
    pub fn new() -> Self {
        Self
    }

    /// Performs bulk rename substitutions on matching name patterns
    pub fn rename_files(&self, filenames: &mut Vec<[u8; 32]>, search_pattern: &[u8], replace_pattern: &[u8]) {
        for i in 0..filenames.len() {
            let mut name = filenames[i];
            // Simple byte substring replacement (standard Linux/Windows PowerRename match logic)
            if let Some(pos) = find_substring(&name, search_pattern) {
                let mut new_name = [0u8; 32];
                // copy before search_pattern
                new_name[..pos].copy_from_slice(&name[..pos]);
                // copy replace_pattern
                let r_len = replace_pattern.len().min(32 - pos);
                new_name[pos..pos + r_len].copy_from_slice(&replace_pattern[..r_len]);
                // copy after search_pattern
                let after_pos = pos + search_pattern.len();
                if after_pos < 32 {
                    let rem_len = (32 - (pos + r_len)).min(32 - after_pos);
                    new_name[pos + r_len..pos + r_len + rem_len].copy_from_slice(&name[after_pos..after_pos + rem_len]);
                }
                filenames[i] = new_name;
            }
        }
    }
}

fn find_substring(data: &[u8; 32], pattern: &[u8]) -> Option<usize> {
    if pattern.is_empty() || pattern.len() > 32 {
        return None;
    }
    for i in 0..=(32 - pattern.len()) {
        let mut found = true;
        for j in 0..pattern.len() {
            if data[i + j] != pattern[j] {
                found = false;
                break;
            }
        }
        if found {
            return Some(i);
        }
    }
    None
}

pub struct LocksmithRecord {
    pub filepath_hash: u64,
    pub locking_pids: Vec<usize>,
}

pub struct FileLocksmith {
    pub locks: Vec<LocksmithRecord>,
}

impl FileLocksmith {
    pub fn new() -> Self {
        Self { locks: Vec::new() }
    }

    pub fn lock_file(&mut self, filepath_hash: u64, pid: usize) {
        let mut record_idx = None;
        for i in 0..self.locks.len() {
            if self.locks[i].filepath_hash == filepath_hash {
                record_idx = Some(i);
                break;
            }
        }

        if let Some(idx) = record_idx {
            if !self.locks[idx].locking_pids.contains(&pid) {
                self.locks[idx].locking_pids.push(pid);
            }
        } else {
            let mut pids = Vec::new();
            pids.push(pid);
            self.locks.push(LocksmithRecord { filepath_hash, locking_pids: pids });
        }
    }

    pub fn unlock_file(&mut self, filepath_hash: u64, pid: usize) {
        for i in 0..self.locks.len() {
            if self.locks[i].filepath_hash == filepath_hash {
                self.locks[i].locking_pids.retain(|&p| p != pid);
                break;
            }
        }
    }

    /// Queries which process IDs hold a file lock
    pub fn get_locking_processes(&self, filepath_hash: u64) -> Vec<usize> {
        for i in 0..self.locks.len() {
            if self.locks[i].filepath_hash == filepath_hash {
                return self.locks[i].locking_pids.clone();
            }
        }
        Vec::new()
    }
}

pub struct HostsEditor {
    pub routes: HashMap<[u8; 32], [u8; 4]>, // maps Domain -> IPv4
}

impl HostsEditor {
    pub fn new() -> Self {
        Self { routes: HashMap::new() }
    }

    pub fn add_route(&mut self, domain: &[u8], ip: [u8; 4]) {
        let mut dom_arr = [0u8; 32];
        let len = domain.len().min(31);
        dom_arr[..len].copy_from_slice(&domain[..len]);
        self.routes.insert(dom_arr, ip);
    }

    pub fn resolve_domain(&self, domain: &[u8]) -> Option<[u8; 4]> {
        let mut dom_arr = [0u8; 32];
        let len = domain.len().min(31);
        dom_arr[..len].copy_from_slice(&domain[..len]);
        self.routes.get(&dom_arr).copied()
    }
}

pub struct SovereignPowerToys {
    pub color_picker: ColorPicker,
    pub fancy_zones: FancyZones,
    pub power_rename: PowerRename,
    pub locksmith: FileLocksmith,
    pub hosts_editor: HostsEditor,
}

impl SovereignPowerToys {
    pub fn new() -> Self {
        Self {
            color_picker: ColorPicker::new(),
            fancy_zones: FancyZones::new(),
            power_rename: PowerRename::new(),
            locksmith: FileLocksmith::new(),
            hosts_editor: HostsEditor::new(),
        }
    }
}

impl Default for SovereignPowerToys {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_powertoys_suite() {
        let mut toys = SovereignPowerToys::new();

        // 1. ColorPicker convert RGB to Hex
        let hex = toys.color_picker.rgb_to_hex(255, 10, 42);
        assert_eq!(&hex, b"#FF0A2A");

        // 2. FancyZones splits layout snap checks
        toys.fancy_zones.create_split_layout(1920, 1080);
        let zone1 = toys.fancy_zones.snap_window(100, 100).unwrap();
        assert_eq!(zone1, 0); // snaps to Left side
        let zone2 = toys.fancy_zones.snap_window(1500, 500).unwrap();
        assert_eq!(zone2, 1); // snaps to Right side

        // 3. PowerRename bulk file renamer
        let mut names = Vec::new();
        let mut name1 = [0u8; 32];
        name1[..8].copy_from_slice(b"test_img");
        names.push(name1);

        toys.power_rename.rename_files(&mut names, b"test", b"prod");
        assert_eq!(&names[0][..8], b"prod_img");

        // 4. FileLocksmith open process file locks
        toys.locksmith.lock_file(5555, 100);
        toys.locksmith.lock_file(5555, 101);
        let pids = toys.locksmith.get_locking_processes(5555);
        assert_eq!(pids.len(), 2);
        assert_eq!(pids[0], 100);

        toys.locksmith.unlock_file(5555, 100);
        let pids_after = toys.locksmith.get_locking_processes(5555);
        assert_eq!(pids_after.len(), 1);
        assert_eq!(pids_after[0], 101);

        // 5. HostsEditor lookup mapping
        toys.hosts_editor.add_route(b"www.google.com", [8, 8, 8, 8]);
        let ip = toys.hosts_editor.resolve_domain(b"www.google.com").unwrap();
        assert_eq!(ip, [8, 8, 8, 8]);
        assert!(toys.hosts_editor.resolve_domain(b"www.offline.com").is_none());
    }
}
