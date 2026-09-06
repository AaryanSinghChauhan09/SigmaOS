#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Standard Plymouth bootsplash theme variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlymouthTheme {
    UbuntuSpinner,
    FedoraCharge,
    ArchGlow,
    BgrtLogo,
    GtkMinimalProgress,
    CustomTheme,
}

/// Plymouth Boot Mode (Normal boot vs LUKS password prompt vs Shutdown)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlymouthMode {
    Booting,
    PasswordPrompt,
    Shutdown,
    UpdateProgress,
}

/// GTK Plymouth Bootsplash Engine
pub struct GtkPlymouthBootsplashEngine {
    pub theme: PlymouthTheme,
    pub mode: PlymouthMode,
    pub progress_percent: u32, // 0 to 100
    pub current_spinner_frame: usize,
    pub message_log: Vec<String>,
    pub password_input_buffer: String,
}

impl GtkPlymouthBootsplashEngine {
    pub fn new(theme: PlymouthTheme) -> Self {
        Self {
            theme,
            mode: PlymouthMode::Booting,
            progress_percent: 0,
            current_spinner_frame: 0,
            message_log: Vec::new(),
            password_input_buffer: String::new(),
        }
    }

    pub fn update_progress(&mut self, percent: u32) {
        self.progress_percent = percent.min(100);
        self.current_spinner_frame = (self.current_spinner_frame + 1) % 12;
    }

    pub fn append_status_message(&mut self, msg: &str) {
        self.message_log.push(msg.to_string());
    }

    pub fn prompt_luks_password(&mut self, prompt: &str) {
        self.mode = PlymouthMode::PasswordPrompt;
        self.append_status_message(prompt);
    }

    pub fn input_password_char(&mut self, ch: char) {
        if self.mode == PlymouthMode::PasswordPrompt {
            self.password_input_buffer.push(ch);
        }
    }

    pub fn submit_password(&mut self) -> String {
        let pwd = self.password_input_buffer.clone();
        self.password_input_buffer.clear();
        self.mode = PlymouthMode::Booting;
        pwd
    }

    pub fn render_frame_status(&self) -> String {
        match self.mode {
            PlymouthMode::Booting => format!(
                "[{:?}] Booting... {}% (Frame #{})",
                self.theme, self.progress_percent, self.current_spinner_frame
            ),
            PlymouthMode::PasswordPrompt => format!(
                "[{:?}] DISK ENCRYPTION KEY PROMPT: {}",
                self.theme,
                "*".repeat(self.password_input_buffer.len())
            ),
            PlymouthMode::Shutdown => format!("[{:?}] Shutting down system...", self.theme),
            PlymouthMode::UpdateProgress => format!(
                "[{:?}] Installing System Updates... {}%",
                self.theme, self.progress_percent
            ),
        }
    }
}

impl Default for GtkPlymouthBootsplashEngine {
    fn default() -> Self {
        Self::new(PlymouthTheme::UbuntuSpinner)
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_gtk_plymouth_bootsplash_engine() {
        let mut plymouth = GtkPlymouthBootsplashEngine::new(PlymouthTheme::BgrtLogo);
        assert_eq!(plymouth.progress_percent, 0);

        plymouth.update_progress(50);
        assert_eq!(plymouth.progress_percent, 50);
        assert_eq!(plymouth.current_spinner_frame, 1);

        let status = plymouth.render_frame_status();
        assert!(status.contains("BgrtLogo"));
        assert!(status.contains("50%"));

        // Test LUKS password prompt
        plymouth.prompt_luks_password("Enter LUKS Passphrase:");
        plymouth.input_password_char('s');
        plymouth.input_password_char('e');
        plymouth.input_password_char('c');

        let pwd_status = plymouth.render_frame_status();
        assert!(pwd_status.contains("***"));

        let submitted = plymouth.submit_password();
        assert_eq!(submitted, "sec");
        assert_eq!(plymouth.mode, PlymouthMode::Booting);

        // Test Shutdown and Update modes
        plymouth.mode = PlymouthMode::Shutdown;
        assert!(plymouth.render_frame_status().contains("Shutting down"));

        plymouth.mode = PlymouthMode::UpdateProgress;
        plymouth.update_progress(80);
        assert!(plymouth.render_frame_status().contains("80%"));
    }
}
