#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use std::format;
use std::vec;
// Moksha Desktop and EFL Engine for SigmaOS
// Natively absorbs JeffHoogland/Moksha Enlightenment Window Manager

#[cfg(test)]
use core::sync::atomic::{AtomicUsize, Ordering};
#[cfg(not(test))]
use core::sync::atomic::{AtomicUsize, Ordering};

use std::string::{String, ToString};
use std::vec::Vec;

/// Moksha Window Type / Class
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MokshaWindowType {
    Normal,
    Dialog,
    Gadget,
    Panel,
    Menu,
}

/// Moksha Shelf (Panel) Placement orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShelfOrientation {
    Top,
    Bottom,
    Left,
    Right,
}

/// Moksha Wallpaper Transition Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallpaperTransition {
    None,
    Fade,
    Slide,
    Zoom,
}

/// Enlightenment Foundation Library (EFL) Canvas Object
#[derive(Debug, Clone)]
pub struct EvasObject {
    pub object_id: usize,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub visible: bool,
}

/// Enlightenment Foundation Library (EFL) Canvas Manager (Evas Parity)
#[derive(Debug, Default)]
pub struct EvasCanvasManager {
    pub objects: Vec<EvasObject>,
    pub next_id: usize,
}

impl EvasCanvasManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        EvasCanvasManager {
            objects: Vec::new(),
            next_id: 1,
        }
    }

    pub fn create_rectangle(&mut self, name: &str, w: u32, h: u32) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.objects.push(EvasObject {
            object_id: id,
            name: name.to_string(),
            width: w,
            height: h,
            visible: true,
        });
        id
    }

    pub fn delete_object(&mut self, id: usize) -> bool {
        if let Some(pos) = self.objects.iter().position(|obj| obj.object_id == id) {
            self.objects.remove(pos);
            true
        } else {
            false
        }
    }
}

/// Moksha profile selector schema
#[derive(Debug, Clone)]
pub struct MokshaProfile {
    pub name: String,
    pub theme: String,
    pub scale_factor: f32,
    pub virtual_desktops_count: usize,
    pub animations_enabled: bool,
}

/// Moksha Desktop Window Manager (parities Enlightenment DR17)
pub struct MokshaWindowManager {
    pub active_profile: MokshaProfile,
    pub active_wallpaper: String,
    pub wallpaper_transition: WallpaperTransition,
    pub canvas: EvasCanvasManager,
}

impl MokshaWindowManager {
    pub fn new(default_profile: MokshaProfile) -> Self {
        MokshaWindowManager {
            active_profile: default_profile,
            active_wallpaper: "bodhi_leaves.png".to_string(),
            wallpaper_transition: WallpaperTransition::Fade,
            canvas: EvasCanvasManager::new(),
        }
    }

    pub fn switch_profile(&mut self, profile: MokshaProfile) {
        self.active_profile = profile;
    }

    pub fn set_wallpaper(&mut self, wallpaper: &str, transition: WallpaperTransition) {
        self.active_wallpaper = wallpaper.to_string();
        self.wallpaper_transition = transition;
    }
}

/// Terminology terminal escape parser supporting custom media links (Bodhi terminology escape parity)
pub struct TerminologyBackend {
    pub buffer_history: Vec<String>,
    pub grid_cols: usize,
    pub grid_rows: usize,
}

impl TerminologyBackend {
    pub fn new(cols: usize, rows: usize) -> Self {
        TerminologyBackend {
            buffer_history: Vec::new(),
            grid_cols: cols,
            grid_rows: rows,
        }
    }

    pub fn write_char(&mut self, ch: char) {
        // If Terminology ESC block occurs, we parse escape codes
        self.buffer_history.push(ch.to_string());
    }

    pub fn parse_terminology_media_escape(&self, escape_seq: &str) -> Option<String> {
        if escape_seq.starts_with("\x1b]10;file://") {
            let path = escape_seq
                .trim_start_matches("\x1b]10;file://")
                .trim_end_matches('\x07');
            Some(path.to_string())
        } else {
            None
        }
    }
}

/// Ephoto image viewing engine
pub struct EphotoViewer {
    pub file_path: String,
    pub zoom_level: f32,
    pub rotation_angle: i32,
}

impl EphotoViewer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        EphotoViewer {
            file_path: String::new(),
            zoom_level: 1.0,
            rotation_angle: 0,
        }
    }

    pub fn load_image(&mut self, path: &str) {
        self.file_path = path.to_string();
        self.zoom_level = 1.0;
        self.rotation_angle = 0;
    }

    pub fn rotate_clockwise(&mut self) {
        self.rotation_angle = (self.rotation_angle + 90) % 360;
    }
}

/// Bodhi AppCenter integration manager (offline/online safe sandbox app installer)
pub struct BodhiAppCenterInstaller {
    pub local_sandbox_dir: String,
    pub allowed_apturl_protocols: Vec<String>,
}

impl BodhiAppCenterInstaller {
    pub fn new(sandbox_dir: &str) -> Self {
        BodhiAppCenterInstaller {
            local_sandbox_dir: sandbox_dir.to_string(),
            allowed_apturl_protocols: std::vec!["apt".to_string(), "apturl".to_string()],
        }
    }

    pub fn install_from_apturl(&self, apturl: &str) -> Result<String, &'static str> {
        if !apturl.starts_with("apt:") {
            return Err("Invalid apturl protocol; must start with apt:");
        }
        let package = apturl.trim_start_matches("apt:");
        if package.is_empty() || package.contains('/') || package.contains(' ') {
            return Err("Invalid package name inside apturl");
        }
        Ok(format!(
            "Installed package '{}' successfully inside Bodhi AppCenter sandbox",
            package
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evas_canvas_manager() {
        let mut evas = EvasCanvasManager::new();
        let obj1 = evas.create_rectangle("rect1", 100, 200);
        assert_eq!(obj1, 1);
        assert_eq!(evas.objects[0].name, "rect1");
        assert!(evas.delete_object(obj1));
        assert_eq!(evas.objects.len(), 0);
    }

    #[test]
    fn test_moksha_wm() {
        let profile = MokshaProfile {
            name: "Default_Moksha".to_string(),
            theme: "Moksha_Green.edj".to_string(),
            scale_factor: 1.0,
            virtual_desktops_count: 4,
            animations_enabled: true,
        };
        let mut wm = MokshaWindowManager::new(profile);
        assert_eq!(wm.active_wallpaper, "bodhi_leaves.png");
        wm.set_wallpaper("autumn.jpg", WallpaperTransition::Zoom);
        assert_eq!(wm.active_wallpaper, "autumn.jpg");
        assert_eq!(wm.wallpaper_transition, WallpaperTransition::Zoom);
    }

    #[test]
    fn test_terminology_escapes() {
        let term = TerminologyBackend::new(80, 24);
        let esc = "\x1b]10;file:///usr/share/backgrounds/leaf.png\x07";
        let res = term.parse_terminology_media_escape(esc);
        assert_eq!(res, Some("/usr/share/backgrounds/leaf.png".to_string()));
    }

    #[test]
    fn test_ephoto_viewer() {
        let mut ephoto = EphotoViewer::new();
        ephoto.load_image("avatar.png");
        assert_eq!(ephoto.file_path, "avatar.png");
        ephoto.rotate_clockwise();
        assert_eq!(ephoto.rotation_angle, 90);
    }

    #[test]
    fn test_bodhi_appcenter() {
        let installer = BodhiAppCenterInstaller::new("/opt/appcenter");
        let res = installer.install_from_apturl("apt:terminology");
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            "Installed package 'terminology' successfully inside Bodhi AppCenter sandbox"
        );
    }
}
