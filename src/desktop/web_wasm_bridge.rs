// SPDX-License-Identifier: MIT
// SigmaOS Native WebAssembly Desktop Bridge Module
// Replaces JavaScript UI event routing and DOM manipulation with native Rust/WASM abstractions

use std::collections::HashMap;
use std::string::String;

/// Native WebAssembly Desktop Engine
/// Bypasses JavaScript engine overheads for desktop event handling and ARIA accessibility
#[derive(Debug, Clone, Default)]
pub struct NativeWasmDesktopEngine {
    pub active_focus_id: Option<String>,
    pub aria_attributes: HashMap<String, String>,
    pub text_node_contents: HashMap<String, String>,
    pub key_events_count: u64,
}

impl NativeWasmDesktopEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dispatch_key_event(&mut self, element_id: &str, _role: &str, key: &str) -> bool {
        self.key_events_count += 1;
        let is_activation = key == "Enter" || key == " ";
        if is_activation {
            self.active_focus_id = Some(element_id.to_string());
        }
        is_activation
    }

    pub fn set_aria_label(&mut self, element_id: &str, label: &str) {
        self.aria_attributes.insert(element_id.to_string(), label.to_string());
    }

    pub fn set_secure_text(&mut self, element_id: &str, text: &str) {
        self.text_node_contents.insert(element_id.to_string(), text.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_wasm_desktop_engine() {
        let mut engine = NativeWasmDesktopEngine::new();
        engine.set_aria_label("dock-launcher", "Application Launcher");
        assert_eq!(
            engine.aria_attributes.get("dock-launcher").map(|s| s.as_str()),
            Some("Application Launcher")
        );

        let activated = engine.dispatch_key_event("dock-launcher", "button", "Enter");
        assert!(activated);
        assert_eq!(engine.active_focus_id.as_deref(), Some("dock-launcher"));

        engine.set_secure_text("app-title", "SigmaOS Zenith Desktop");
        assert_eq!(
            engine.text_node_contents.get("app-title").map(|s| s.as_str()),
            Some("SigmaOS Zenith Desktop")
        );
    }
}
