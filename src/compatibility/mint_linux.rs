#![no_std]
#![no_main]

use core::mem;
/// Linux Mint-inspired User Experience Compatibility Suite for SigmaOS
/// Provides MintUpdate-style package ranking, MintBackup-style directory archiving,
/// MintSoftware-style community ratings, and Zenith dynamic compositor window arrangements.
use core::sync::atomic::{AtomicUsize, Ordering};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MintError {
    Success = 0,
    BackupFailed = 1,
    UpdateError = 2,
    LayoutFailed = 3,
}

/// MintUpdate-style Package Update Classifications
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MintUpdateLevel {
    Safe = 1,         // Tested, fully certified
    Recommended = 2,  // Safe, updates core submodules
    Normal = 3,       // Minor features, typical security
    Experimental = 4, // Needs user evaluation
    Dangerous = 5,    // Hard system kernel updates
}

#[derive(Debug, Clone, Copy)]
pub struct MintUpdateItem {
    pub package_name: [u8; 32],
    pub version: [u8; 16],
    pub level: MintUpdateLevel,
}

impl MintUpdateItem {
    pub fn new(package_name: &[u8], version: &[u8], level: MintUpdateLevel) -> Self {
        let mut pkg_array = [0u8; 32];
        let mut ver_array = [0u8; 16];
        let pkg_len = package_name.len().min(31);
        let ver_len = version.len().min(15);

        unsafe {
            core::ptr::copy_nonoverlapping(package_name.as_ptr(), pkg_array.as_mut_ptr(), pkg_len);
            core::ptr::copy_nonoverlapping(version.as_ptr(), ver_array.as_mut_ptr(), ver_len);
        }

        MintUpdateItem {
            package_name: pkg_array,
            version: ver_array,
            level,
        }
    }
}

/// MintUpdate-inspired Latency and Classification Manager
pub struct MintUpdateManager {
    pub updates: Vec<Option<MintUpdateItem>>,
    pub fastest_mirror_id: AtomicUsize,
}

impl MintUpdateManager {
    pub fn new() -> Self {
        MintUpdateManager {
            updates: Vec::new(),
            fastest_mirror_id: AtomicUsize::new(0),
        }
    }

    pub fn register_update(&mut self, item: MintUpdateItem) {
        self.updates.push(Some(item));
    }

    /// Simulate mirror ping latency sweep to select the fastest mirror
    pub fn select_fastest_mirror(&self, latencies_ms: &[usize]) -> usize {
        let mut min_latency = usize::MAX;
        let mut best_id = 0;

        for i in 0..latencies_ms.len() {
            if latencies_ms[i] < min_latency {
                min_latency = latencies_ms[i];
                best_id = i;
            }
        }

        self.fastest_mirror_id.store(best_id, Ordering::SeqCst);
        best_id
    }
}

/// MintBackup-inspired User Directory Archiver
pub struct MintBackupTool {
    pub backup_count: AtomicUsize,
}

impl MintBackupTool {
    pub fn new() -> Self {
        MintBackupTool {
            backup_count: AtomicUsize::new(0),
        }
    }

    /// Create archive snapshot of user directory bytes with basic CRC-32 checksum integrity
    pub fn archive_directory(
        &self,
        src_data: &[u8],
        archive: &mut [u8],
    ) -> Result<(usize, u32), MintError> {
        if src_data.len() > archive.len() {
            return Err(MintError::BackupFailed);
        }

        // Compute a mock checksum of directory bytes
        let mut crc = 0xFFFFFFFFu32;
        for i in 0..src_data.len() {
            crc ^= src_data[i] as u32;
            for _ in 0..8 {
                if (crc & 1) != 0 {
                    crc = (crc >> 1) ^ 0xEDB88320;
                } else {
                    crc >>= 1;
                }
            }
            archive[i] = src_data[i];
        }

        self.backup_count.fetch_add(1, Ordering::SeqCst);
        Ok((src_data.len(), !crc))
    }
}

/// MintSoftware-inspired package catalog rating reviews database
pub struct SoftwareMeta {
    pub name: [u8; 32],
    pub rating_stars: u32,
    pub reviews_count: u32,
}

pub struct MintSoftwareManager {
    pub software_list: Vec<Option<SoftwareMeta>>,
}

impl MintSoftwareManager {
    pub fn new() -> Self {
        MintSoftwareManager {
            software_list: Vec::new(),
        }
    }

    pub fn register_software(&mut self, name: &[u8], stars: u32, reviews: u32) {
        let mut name_array = [0u8; 32];
        let len = name.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), len);
        }
        self.software_list.push(Some(SoftwareMeta {
            name: name_array,
            rating_stars: stars,
            reviews_count: reviews,
        }));
    }
}

/// Zenith Dynamic Display Coordinator Layout representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowCoordinates {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub struct ZenithDisplayCompositor;

impl ZenithDisplayCompositor {
    /// Arrange windows using a Master-and-Stack Binary Tiling Layout algorithm (Linux Mint Window Manager equivalent)
    pub fn arrange_tiling(
        screen_w: usize,
        screen_h: usize,
        num_windows: usize,
        coords: &mut [WindowCoordinates],
    ) -> Result<(), MintError> {
        if num_windows == 0 {
            return Ok(());
        }

        if coords.len() < num_windows {
            return Err(MintError::LayoutFailed);
        }

        if num_windows == 1 {
            coords[0] = WindowCoordinates {
                x: 0,
                y: 0,
                width: screen_w,
                height: screen_h,
            };
            return Ok(());
        }

        // Split screen: Master window on left (50%), Stack windows stacked on right (50%)
        let master_w = screen_w / 2;
        coords[0] = WindowCoordinates {
            x: 0,
            y: 0,
            width: master_w,
            height: screen_h,
        };

        let stack_count = num_windows - 1;
        let stack_h = screen_h / stack_count;

        for i in 1..num_windows {
            coords[i] = WindowCoordinates {
                x: master_w,
                y: (i - 1) * stack_h,
                width: screen_w - master_w,
                height: stack_h,
            };
        }

        Ok(())
    }

    /// Arrange windows using Stacking layout (Cascaded coordinations)
    pub fn arrange_stacking(
        num_windows: usize,
        coords: &mut [WindowCoordinates],
    ) -> Result<(), MintError> {
        for i in 0..num_windows {
            if i >= coords.len() {
                return Err(MintError::LayoutFailed);
            }
            coords[i] = WindowCoordinates {
                x: i * 30,
                y: i * 30,
                width: 800,
                height: 600,
            };
        }
        Ok(())
    }
}

// ==========================================
// Cinnamon Desktop Theme Engine
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CinnamonDesklet {
    pub id: u32,
    pub x: usize,
    pub y: usize,
}

pub struct CinnamonThemeEngine {
    pub active_gtk_theme: [u8; 32],
    pub desklets: Vec<Option<CinnamonDesklet>>,
    pub is_panel_enabled: bool,
}

impl CinnamonThemeEngine {
    pub fn new() -> Self {
        let mut theme = [0u8; 32];
        let default_name = b"Mint-Y-Dark";
        unsafe {
            core::ptr::copy_nonoverlapping(default_name.as_ptr(), theme.as_mut_ptr(), default_name.len());
        }
        Self {
            active_gtk_theme: theme,
            desklets: Vec::new(),
            is_panel_enabled: true,
        }
    }

    pub fn set_gtk_theme(&mut self, theme_name: &[u8]) {
        let mut theme = [0u8; 32];
        let len = theme_name.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(theme_name.as_ptr(), theme.as_mut_ptr(), len);
        }
        self.active_gtk_theme = theme;
    }

    pub fn add_desklet(&mut self, id: u32, x: usize, y: usize) {
        self.desklets.push(Some(CinnamonDesklet { id, x, y }));
    }
}

impl Default for CinnamonThemeEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// Timeshift-style System Restorer
// ==========================================

#[derive(Debug, Clone, Copy)]
pub struct SystemRestorePoint {
    pub id: u32,
    pub is_rsync: bool,
    pub timestamp_ms: u64,
}

pub struct TimeshiftSystemRestorer {
    pub restore_points: Vec<Option<SystemRestorePoint>>,
    pub active_restore_point_id: u32,
}

impl TimeshiftSystemRestorer {
    pub fn new() -> Self {
        Self {
            restore_points: Vec::new(),
            active_restore_point_id: 0,
        }
    }

    pub fn create_restore_point(&mut self, id: u32, is_rsync: bool) {
        self.restore_points.push(Some(SystemRestorePoint {
            id,
            is_rsync,
            timestamp_ms: 0,
        }));
    }

    pub fn rollback_system(&mut self, id: u32) -> Result<(), MintError> {
        let mut found = false;
        for i in 0..self.restore_points.len {
            if let Some(ref rp) = self.restore_points[i] {
                if rp.id == id {
                    found = true;
                    break;
                }
            }
        }
        if found {
            self.active_restore_point_id = id;
            Ok(())
        } else {
            Err(MintError::UpdateError)
        }
    }
}

impl Default for TimeshiftSystemRestorer {
    fn default() -> Self {
        Self::new()
    }
}

struct Vec<T> {
    pub data: *mut T,
    pub len: usize,
    pub capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mint_update_manager() {
        let mut manager = MintUpdateManager::new();
        manager.register_update(MintUpdateItem::new(
            b"kernel",
            b"1.0.1",
            MintUpdateLevel::Dangerous,
        ));
        manager.register_update(MintUpdateItem::new(
            b"firefox",
            b"120.0",
            MintUpdateLevel::Normal,
        ));

        assert_eq!(manager.updates.len, 2);

        let mut update_name = [0u8; 7];
        for i in 0..7 {
            update_name[i] = manager.updates[1].unwrap().package_name[i];
        }
        assert_eq!(&update_name, b"firefox");

        let latencies = [45, 12, 98, 150];
        let best_mirror = manager.select_fastest_mirror(&latencies);
        assert_eq!(best_mirror, 1);
    }

    #[test]
    fn test_mint_backup_checksum() {
        let src = b"Important User Profile Data";
        let mut archive = [0u8; 64];
        let tool = MintBackupTool::new();

        let (bytes, checksum) = tool.archive_directory(src, &mut archive).unwrap();
        assert_eq!(bytes, src.len());
        assert!(checksum > 0);
        assert_eq!(&archive[..bytes], src);
    }

    #[test]
    fn test_zenith_layouts() {
        let mut coords = [WindowCoordinates {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }; 3];
        assert!(ZenithDisplayCompositor::arrange_tiling(1920, 1080, 3, &mut coords).is_ok());

        // Master window takes left 50%
        assert_eq!(
            coords[0],
            WindowCoordinates {
                x: 0,
                y: 0,
                width: 960,
                height: 1080
            }
        );

        // Stack windows split right 50% horizontally
        assert_eq!(
            coords[1],
            WindowCoordinates {
                x: 960,
                y: 0,
                width: 960,
                height: 540
            }
        );
        assert_eq!(
            coords[2],
            WindowCoordinates {
                x: 960,
                y: 540,
                width: 960,
                height: 540
            }
        );

        // Test cascade stacking
        assert!(ZenithDisplayCompositor::arrange_stacking(3, &mut coords).is_ok());
        assert_eq!(
            coords[1],
            WindowCoordinates {
                x: 30,
                y: 30,
                width: 800,
                height: 600
            }
        );
    }

    #[test]
    fn test_cinnamon_theme_engine() {
        let mut engine = CinnamonThemeEngine::new();
        let mut default_theme = [0u8; 11];
        for i in 0..11 {
            default_theme[i] = engine.active_gtk_theme[i];
        }
        assert_eq!(&default_theme, b"Mint-Y-Dark");

        engine.set_gtk_theme(b"Mint-Y-Light");
        let mut new_theme = [0u8; 12];
        for i in 0..12 {
            new_theme[i] = engine.active_gtk_theme[i];
        }
        assert_eq!(&new_theme, b"Mint-Y-Light");

        engine.add_desklet(101, 200, 200);
        assert_eq!(engine.desklets.len, 1);
        assert_eq!(engine.desklets[0].unwrap().id, 101);
    }

    #[test]
    fn test_timeshift_system_restorer() {
        let mut restorer = TimeshiftSystemRestorer::new();
        assert_eq!(restorer.active_restore_point_id, 0);

        restorer.create_restore_point(101, true); // rsync snapshot
        restorer.create_restore_point(102, false); // btrfs snapshot
        assert_eq!(restorer.restore_points.len, 2);

        // Rollback succeeds for existing restore point
        assert!(restorer.rollback_system(102).is_ok());
        assert_eq!(restorer.active_restore_point_id, 102);

        // Rollback fails for nonexistent point
        assert_eq!(restorer.rollback_system(999), Err(MintError::UpdateError));
    }
}
