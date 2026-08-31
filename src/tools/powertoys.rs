use alloc::string::{String, ToString};
use alloc::format;
//! Sovereign Microsoft PowerToys replication suite for SigmaOS
//! Implements a rich suite of system and workspace utilities:
//! - ColorPicker (HEX / RGB screen color inspector)
//! - FancyZones (Coordinates, tiling window layout grid arranger)
//! - PowerRename (Bulk filename batch regular-expression renamer)
//! - FileLocksmith (Tracks active process IDs holding file descriptor locks)
//! - HostsEditor (IP lookup hosts custom DNS routing rule editor)

use crate::klib::{Vec, BTreeMap};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseOption {
    None,
    LowerCase,
    UpperCase,
    TitleCase,
}

pub struct PowerRename;

impl PowerRename {
    pub fn new() -> Self {
        Self
    }

    /// Performs bulk rename substitutions on matching name patterns with optional case conversion and numbering
    pub fn rename_files(&self, filenames: &mut Vec<[u8; 32]>, search_pattern: &[u8], replace_pattern: &[u8]) {
        self.rename_files_advanced(filenames, search_pattern, replace_pattern, CaseOption::None, false);
    }

    /// Advanced bulk rename support including case options, counter insertion, and collision safety checks
    pub fn rename_files_advanced(
        &self,
        filenames: &mut Vec<[u8; 32]>,
        search_pattern: &[u8],
        replace_pattern: &[u8],
        case_opt: CaseOption,
        enable_enum_counter: bool,
    ) {
        let mut new_names = Vec::new();

        for i in 0..filenames.len() {
            let name = filenames[i];
            let mut new_name = name;

            if let Some(pos) = find_substring(&name, search_pattern) {
                let mut temp = [0u8; 32];
                // copy before search_pattern
                temp[..pos].copy_from_slice(&name[..pos]);
                // copy replace_pattern
                let r_len = replace_pattern.len().min(32 - pos);
                temp[pos..pos + r_len].copy_from_slice(&replace_pattern[..r_len]);
                // copy after search_pattern
                let after_pos = pos + search_pattern.len();
                if after_pos < 32 {
                    let rem_len = (32 - (pos + r_len)).min(32 - after_pos);
                    temp[pos + r_len..pos + r_len + rem_len].copy_from_slice(&name[after_pos..after_pos + rem_len]);
                }
                new_name = temp;
            }

            // Apply case option
            if case_opt != CaseOption::None {
                let mut capitalize_next = true;
                for b in new_name.iter_mut() {
                    match case_opt {
                        CaseOption::LowerCase => {
                            if *b >= b'A' && *b <= b'Z' {
                                *b += 32;
                            }
                        }
                        CaseOption::UpperCase => {
                            if *b >= b'a' && *b <= b'z' {
                                *b -= 32;
                            }
                        }
                        CaseOption::TitleCase => {
                            if *b == b' ' || *b == b'_' || *b == b'-' || *b == b'.' {
                                capitalize_next = true;
                            } else if capitalize_next {
                                if *b >= b'a' && *b <= b'z' {
                                    *b -= 32;
                                }
                                capitalize_next = false;
                            } else if *b >= b'A' && *b <= b'Z' {
                                *b += 32;
                            }
                        }
                        CaseOption::None => {}
                    }
                }
            }

            // Apply counter enumeration if enabled
            if enable_enum_counter {
                let counter_str = format!("_{:02}", i + 1);
                let counter_bytes = counter_str.as_bytes();
                // append before null byte or end
                let len = new_name.iter().position(|&b| b == 0).unwrap_or(32);
                let space = 32 - len;
                if space >= counter_bytes.len() {
                    new_name[len..len + counter_bytes.len()].copy_from_slice(counter_bytes);
                }
            }

            new_names.push(new_name);
        }

        // Collision safety check: ensure no duplicate filenames created
        let mut has_duplicate = false;
        for i in 0..new_names.len() {
            for j in (i + 1)..new_names.len() {
                if new_names[i] == new_names[j] && new_names[i] != [0u8; 32] {
                    has_duplicate = true;
                    break;
                }
            }
        }

        if !has_duplicate {
            for i in 0..filenames.len() {
                filenames[i] = new_names[i];
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
    pub routes: BTreeMap<[u8; 32], [u8; 4]>, // maps Domain -> IPv4
}

impl HostsEditor {
    pub fn new() -> Self {
        Self { routes: BTreeMap::new() }
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

/// Always On Top window Z-order pinning manager
pub struct AlwaysOnTop {
    pub pinned_pids: Vec<usize>,
}

impl AlwaysOnTop {
    pub fn new() -> Self {
        Self { pinned_pids: Vec::new() }
    }

    pub fn toggle_pin(&mut self, pid: usize) -> bool {
        if let Some(pos) = self.pinned_pids.iter().position(|&p| p == pid) {
            self.pinned_pids.remove(pos);
            false
        } else {
            self.pinned_pids.push(pid);
            true
        }
    }

    pub fn is_pinned(&self, pid: usize) -> bool {
        self.pinned_pids.contains(&pid)
    }
}

/// On-screen Text Extractor & OCR text sanitizer
pub struct TextExtractor;

impl TextExtractor {
    pub fn new() -> Self {
        Self
    }

    /// Strips non-printable control characters from screen text captures
    pub fn sanitize_extracted_text(&self, raw_bytes: &[u8]) -> Vec<u8> {
        let mut clean = Vec::new();
        for &b in raw_bytes {
            if b >= 32 && b <= 126 || b == b'\n' || b == b'\t' {
                clean.push(b);
            }
        }
        clean
    }
}

/// Paste as Plain Text formatting stripper
pub struct PastePlain;

impl PastePlain {
    pub fn new() -> Self {
        Self
    }

    /// Strips HTML/rich tags from clipboard buffer
    pub fn strip_rich_formatting(&self, input: &[u8]) -> Vec<u8> {
        let mut plain = Vec::new();
        let mut in_tag = false;
        for &b in input {
            if b == b'<' {
                in_tag = true;
            } else if b == b'>' {
                in_tag = false;
            } else if !in_tag {
                plain.push(b);
            }
        }
        plain
    }
}

/// Multi-monitor fast cursor teleportation helper
pub struct MouseJump {
    pub current_x: u32,
    pub current_y: u32,
}

impl MouseJump {
    pub fn new() -> Self {
        Self { current_x: 0, current_y: 0 }
    }

    pub fn jump_to_screen_center(&mut self, screen_x: u32, screen_y: u32, width: u32, height: u32) -> (u32, u32) {
        self.current_x = screen_x + width / 2;
        self.current_y = screen_y + height / 2;
        (self.current_x, self.current_y)
    }
}

/// Awake power state sleep/display idle suppression manager
pub struct AwakePowerKeep {
    pub keep_awake: bool,
    pub keep_display_on: bool,
}

impl AwakePowerKeep {
    pub fn new() -> Self {
        Self {
            keep_awake: false,
            keep_display_on: false,
        }
    }

    pub fn set_mode(&mut self, keep_awake: bool, keep_display_on: bool) {
        self.keep_awake = keep_awake;
        self.keep_display_on = keep_display_on;
    }
}

pub struct SovereignPowerToys {
    pub color_picker: ColorPicker,
    pub fancy_zones: FancyZones,
    pub power_rename: PowerRename,
    pub locksmith: FileLocksmith,
    pub hosts_editor: HostsEditor,
    pub always_on_top: AlwaysOnTop,
    pub text_extractor: TextExtractor,
    pub paste_plain: PastePlain,
    pub mouse_jump: MouseJump,
    pub awake_keep: AwakePowerKeep,
}

impl SovereignPowerToys {
    pub fn new() -> Self {
        Self {
            color_picker: ColorPicker::new(),
            fancy_zones: FancyZones::new(),
            power_rename: PowerRename::new(),
            locksmith: FileLocksmith::new(),
            hosts_editor: HostsEditor::new(),
            always_on_top: AlwaysOnTop::new(),
            text_extractor: TextExtractor::new(),
            paste_plain: PastePlain::new(),
            mouse_jump: MouseJump::new(),
            awake_keep: AwakePowerKeep::new(),
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
        name1[..8].copy_from_slice(b"TEST_img");
        names.push(name1);

        toys.power_rename.rename_files_advanced(&mut names, b"TEST", b"prod", CaseOption::LowerCase, true);
        assert_eq!(&names[0][..11], b"prod_img_01");

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

        // 6. AlwaysOnTop window Z-order pinning
        assert!(!toys.always_on_top.is_pinned(404));
        assert!(toys.always_on_top.toggle_pin(404));
        assert!(toys.always_on_top.is_pinned(404));
        assert!(!toys.always_on_top.toggle_pin(404));

        // 7. TextExtractor OCR text sanitizer
        let raw_ocr = b"Line 1\nLine 2\x07\x00";
        let clean_text = toys.text_extractor.sanitize_extracted_text(raw_ocr);
        assert_eq!(&clean_text, b"Line 1\nLine 2");

        // 8. PastePlain HTML tag stripper
        let html_clip = b"<b>Bold Text</b> <i>Italic</i>";
        let plain_clip = toys.paste_plain.strip_rich_formatting(html_clip);
        assert_eq!(&plain_clip, b"Bold Text Italic");

        // 9. MouseJump monitor center teleport
        let (cx, cy) = toys.mouse_jump.jump_to_screen_center(1920, 0, 1920, 1080);
        assert_eq!((cx, cy), (2880, 540));

        // 10. AwakePowerKeep sleep suppression mode
        toys.awake_keep.set_mode(true, true);
        assert!(toys.awake_keep.keep_awake);
        assert!(toys.awake_keep.keep_display_on);
    }
}
