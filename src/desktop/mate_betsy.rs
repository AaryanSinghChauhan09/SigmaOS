//! Linux Mint Debian Edition (LMDE 2 "Betsy") MATE Desktop Suite Implementation
//! Replicates MATE Desktop Environment components: Marco (WM), Caja (File Manager),
//! Pluma (Text Editor), Atril (Document Viewer), and Eye of MATE (EOM Image Viewer).


use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

/// MATE Marco Window Manager
pub struct MarcoWindowManager {
    pub theme_name: String,
    pub compositing_manager_active: bool,
    pub active_windows: Vec<u32>, // window_ids
    pub window_title_bars: Vec<(u32, String)>,
}

impl MarcoWindowManager {
    pub fn new() -> Self {
        MarcoWindowManager {
            theme_name: String::from("Menta"),
            compositing_manager_active: true,
            active_windows: Vec::new(),
            window_title_bars: Vec::new(),
        }
    }

    pub fn map_window(&mut self, window_id: u32, title: &str) {
        if !self.active_windows.contains(&window_id) {
            self.active_windows.push(window_id);
            self.window_title_bars.push((window_id, title.to_string()));
        }
    }

    pub fn unmap_window(&mut self, window_id: u32) {
        self.active_windows.retain(|&w| w != window_id);
        self.window_title_bars.retain(|(w, _)| *w != window_id);
    }
}

/// MATE Caja File Manager
pub struct CajaFileManager {
    pub current_directory: String,
    pub dual_pane_active: bool,
    pub selected_emblem: String,
    pub extension_scripts: Vec<String>,
}

impl CajaFileManager {
    pub fn new() -> Self {
        CajaFileManager {
            current_directory: String::from("/home/user"),
            dual_pane_active: false,
            selected_emblem: String::from("emblem-favorite"),
            extension_scripts: vec![
                "open-in-terminal.sh".to_string(),
                "caja-image-converter.py".to_string(),
            ],
        }
    }

    pub fn navigate_to(&mut self, dir_path: &str) {
        self.current_directory = dir_path.to_string();
    }

    pub fn toggle_dual_pane(&mut self) -> bool {
        self.dual_pane_active = !self.dual_pane_active;
        self.dual_pane_active
    }
}

/// MATE Pluma Text Editor
pub struct PlumaTextEditor {
    pub document_title: String,
    pub content_lines: Vec<String>,
    pub syntax_highlighting_language: String,
    pub show_line_numbers: bool,
}

impl PlumaTextEditor {
    pub fn new(title: &str) -> Self {
        PlumaTextEditor {
            document_title: title.to_string(),
            content_lines: Vec::new(),
            syntax_highlighting_language: String::from("rust"),
            show_line_numbers: true,
        }
    }

    pub fn append_line(&mut self, line: &str) {
        self.content_lines.push(line.to_string());
    }

    pub fn get_line_count(&self) -> usize {
        self.content_lines.len()
    }
}

/// MATE Atril Document Viewer
pub struct AtrilDocumentViewer {
    pub file_path: String,
    pub current_page: u32,
    pub total_pages: u32,
    pub sidebar_visible: bool,
    pub zoom_level_pct: u32,
}

impl AtrilDocumentViewer {
    pub fn open_pdf(path: &str, pages: u32) -> Self {
        AtrilDocumentViewer {
            file_path: path.to_string(),
            current_page: 1,
            total_pages: pages,
            sidebar_visible: true,
            zoom_level_pct: 100,
        }
    }

    pub fn goto_page(&mut self, page: u32) -> Result<u32, &'static str> {
        if page == 0 || page > self.total_pages {
            Err("Atril: Page number out of range")
        } else {
            self.current_page = page;
            Ok(self.current_page)
        }
    }
}

/// MATE Eye of MATE (EOM) Image Viewer
pub struct EyeOfMateImageViewer {
    pub image_path: String,
    pub width: u32,
    pub height: u32,
    pub rotation_degrees: u32,
    pub exif_camera_model: Option<String>,
}

impl EyeOfMateImageViewer {
    pub fn open_image(path: &str, width: u32, height: u32) -> Self {
        EyeOfMateImageViewer {
            image_path: path.to_string(),
            width,
            height,
            rotation_degrees: 0,
            exif_camera_model: Some("SigmaOS Virtual Camera".to_string()),
        }
    }

    pub fn rotate_90_degrees_clockwise(&mut self) {
        self.rotation_degrees = (self.rotation_degrees + 90) % 360;
    }
}

/// LMDE 2 Betsy MATE Master Desktop Coordinator
pub struct MateBetsyDesktopEnvironment {
    pub marco: MarcoWindowManager,
    pub caja: CajaFileManager,
    pub active_editor: Option<PlumaTextEditor>,
    pub active_atril: Option<AtrilDocumentViewer>,
    pub active_eom: Option<EyeOfMateImageViewer>,
    pub mate_panel_bottom_active: bool,
}

impl MateBetsyDesktopEnvironment {
    pub fn new() -> Self {
        MateBetsyDesktopEnvironment {
            marco: MarcoWindowManager::new(),
            caja: CajaFileManager::new(),
            active_editor: None,
            active_atril: None,
            active_eom: None,
            mate_panel_bottom_active: true,
        }
    }

    pub fn launch_pluma_editor(&mut self, filename: &str) {
        self.active_editor = Some(PlumaTextEditor::new(filename));
        self.marco.map_window(101, &format!("Pluma - {}", filename));
    }

    pub fn launch_atril_pdf(&mut self, pdf_path: &str, total_pages: u32) {
        self.active_atril = Some(AtrilDocumentViewer::open_pdf(pdf_path, total_pages));
        self.marco.map_window(102, &format!("Atril - {}", pdf_path));
    }

    pub fn launch_eye_of_mate(&mut self, image_path: &str, w: u32, h: u32) {
        self.active_eom = Some(EyeOfMateImageViewer::open_image(image_path, w, h));
        self.marco
            .map_window(103, &format!("Eye of MATE - {}", image_path));
    }
}

impl Default for MateBetsyDesktopEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mate_betsy_desktop_environment() {
        let mut mate = MateBetsyDesktopEnvironment::new();
        assert_eq!(mate.marco.theme_name, "Menta");
        assert!(mate.mate_panel_bottom_active);

        mate.caja.navigate_to("/home/user/Documents");
        assert_eq!(mate.caja.current_directory, "/home/user/Documents");
        assert!(mate.caja.toggle_dual_pane());

        mate.launch_pluma_editor("main.rs");
        assert!(mate.active_editor.is_some());
        assert_eq!(mate.marco.active_windows.len(), 1);

        let editor = mate.active_editor.as_mut().unwrap();
        editor.append_line("fn main() {}");
        assert_eq!(editor.get_line_count(), 1);

        mate.launch_atril_pdf("doc.pdf", 10);
        let pdf = mate.active_atril.as_mut().unwrap();
        assert_eq!(pdf.goto_page(5).unwrap(), 5);

        mate.launch_eye_of_mate("photo.jpg", 1920, 1080);
        let eom = mate.active_eom.as_mut().unwrap();
        eom.rotate_90_degrees_clockwise();
        assert_eq!(eom.rotation_degrees, 90);
    }
}
