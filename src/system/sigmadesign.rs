//! SigmaDesign - Design System and UI Components
//!
//! Provides comprehensive design system and UI component management inspired by Design Craft.
//! Supports design tokens, component library, and theming.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Design token type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Color,
    Spacing,
    Typography,
    Shadow,
    BorderRadius,
    Transition,
}

/// Design token value
#[derive(Debug, Clone)]
pub enum TokenValue {
    Color(String),
    Spacing(u32),
    Typography(String),
    Shadow(String),
    BorderRadius(u32),
    Transition(u32),
}

/// Design token
#[derive(Debug, Clone)]
pub struct DesignToken {
    pub id: String,
    pub name: String,
    pub token_type: TokenType,
    pub value: TokenValue,
}

impl DesignToken {
    pub fn new(id: String, name: String, token_type: TokenType, value: TokenValue) -> Self {
        Self {
            id,
            name,
            token_type,
            value,
        }
    }
}

/// Component variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentVariant {
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Info,
}

/// Component size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

/// UI component
#[derive(Debug, Clone)]
pub struct UIComponent {
    pub id: String,
    pub name: String,
    pub component_type: String,
    pub variant: ComponentVariant,
    pub size: ComponentSize,
    pub properties: BTreeMap<String, String>,
}

impl UIComponent {
    pub fn new(id: String, name: String, component_type: String) -> Self {
        Self {
            id,
            name,
            component_type,
            variant: ComponentVariant::Primary,
            size: ComponentSize::Medium,
            properties: BTreeMap::new(),
        }
    }

    pub fn set_variant(&mut self, variant: ComponentVariant) {
        self.variant = variant;
    }

    pub fn set_size(&mut self, size: ComponentSize) {
        self.size = size;
    }

    pub fn add_property(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }

    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }
}

/// Design system
#[derive(Debug, Clone)]
pub struct DesignSystem {
    pub id: String,
    pub name: String,
    pub tokens: BTreeMap<String, DesignToken>,
    pub components: BTreeMap<String, UIComponent>,
}

impl DesignSystem {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            tokens: BTreeMap::new(),
            components: BTreeMap::new(),
        }
    }

    pub fn add_token(&mut self, token: DesignToken) {
        self.tokens.insert(token.id.clone(), token);
    }

    pub fn remove_token(&mut self, id: &str) -> bool {
        self.tokens.remove(id).is_some()
    }

    pub fn get_token(&self, id: &str) -> Option<&DesignToken> {
        self.tokens.get(id)
    }

    pub fn get_token_mut(&mut self, id: &str) -> Option<&mut DesignToken> {
        self.tokens.get_mut(id)
    }

    pub fn add_component(&mut self, component: UIComponent) {
        self.components.insert(component.id.clone(), component);
    }

    pub fn remove_component(&mut self, id: &str) -> bool {
        self.components.remove(id).is_some()
    }

    pub fn get_component(&self, id: &str) -> Option<&UIComponent> {
        self.components.get(id)
    }

    pub fn get_component_mut(&mut self, id: &str) -> Option<&mut UIComponent> {
        self.components.get_mut(id)
    }

    pub fn list_tokens(&self) -> Vec<&DesignToken> {
        self.tokens.values().collect()
    }

    pub fn list_components(&self) -> Vec<&UIComponent> {
        self.components.values().collect()
    }

    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }

    pub fn component_count(&self) -> usize {
        self.components.len()
    }
}

/// SigmaDesign - Design system manager
#[derive(Debug)]
pub struct SigmaDesign {
    design_systems: BTreeMap<String, DesignSystem>,
    current_system: Option<String>,
}

impl SigmaDesign {
    pub fn new() -> Self {
        Self {
            design_systems: BTreeMap::new(),
            current_system: None,
        }
    }

    pub fn add_design_system(&mut self, system: DesignSystem) {
        self.design_systems.insert(system.id.clone(), system);
    }

    pub fn remove_design_system(&mut self, id: &str) -> bool {
        if self.current_system.as_ref() == Some(&id.to_string()) {
            self.current_system = None;
        }
        self.design_systems.remove(id).is_some()
    }

    pub fn get_design_system(&self, id: &str) -> Option<&DesignSystem> {
        self.design_systems.get(id)
    }

    pub fn get_design_system_mut(&mut self, id: &str) -> Option<&mut DesignSystem> {
        self.design_systems.get_mut(id)
    }

    pub fn set_current_system(&mut self, id: &str) -> bool {
        if self.design_systems.contains_key(id) {
            self.current_system = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_system(&self) -> Option<&DesignSystem> {
        self.current_system.as_ref()
            .and_then(|id| self.design_systems.get(id))
    }

    pub fn get_current_system_mut(&mut self) -> Option<&mut DesignSystem> {
        self.current_system.as_ref()
            .and_then(|id| self.design_systems.get_mut(id.as_str()))
    }

    pub fn list_design_systems(&self) -> Vec<&DesignSystem> {
        self.design_systems.values().collect()
    }

    pub fn design_system_count(&self) -> usize {
        self.design_systems.len()
    }
}

impl Default for SigmaDesign {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaDesign instance
static mut SIGMADESIGN_INSTANCE: Option<SigmaDesign> = None;

/// Initialize the global SigmaDesign instance
pub fn init_sigmadesign() {
    unsafe {
        SIGMADESIGN_INSTANCE = Some(SigmaDesign::new());
    }
}

/// Get the global SigmaDesign instance
pub fn get_sigmadesign() -> Option<&'static SigmaDesign> {
    unsafe {
        SIGMADESIGN_INSTANCE.as_ref()
    }
}

/// Get the global SigmaDesign instance (mutable)
pub fn get_sigmadesign_mut() -> Option<&'static mut SigmaDesign> {
    unsafe {
        SIGMADESIGN_INSTANCE.as_mut()
    }
}

/// Convenience function to add a design system
pub fn add_design_system(system: DesignSystem) -> bool {
    if let Some(design) = get_sigmadesign_mut() {
        design.add_design_system(system);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_design_token_creation() {
        let token = DesignToken::new(
            "token1".to_string(),
            "Primary Color".to_string(),
            TokenType::Color,
            TokenValue::Color("#FF0000".to_string()),
        );
        assert_eq!(token.id, "token1");
        assert_eq!(token.name, "Primary Color");
    }

    #[test]
    fn test_ui_component_creation() {
        let component = UIComponent::new("btn1".to_string(), "Button".to_string(), "button".to_string());
        assert_eq!(component.id, "btn1");
        assert_eq!(component.component_type, "button");
        assert_eq!(component.variant, ComponentVariant::Primary);
    }

    #[test]
    fn test_ui_component_modification() {
        let mut component = UIComponent::new("btn1".to_string(), "Button".to_string(), "button".to_string());
        component.set_variant(ComponentVariant::Secondary);
        component.set_size(ComponentSize::Large);
        component.add_property("radius".to_string(), "8px".to_string());
        assert_eq!(component.variant, ComponentVariant::Secondary);
        assert_eq!(component.size, ComponentSize::Large);
        assert!(component.get_property("radius").is_some());
    }

    #[test]
    fn test_design_system_creation() {
        let system = DesignSystem::new("system1".to_string(), "Material Design".to_string());
        assert_eq!(system.id, "system1");
        assert_eq!(system.name, "Material Design");
    }

    #[test]
    fn test_design_system_token_management() {
        let mut system = DesignSystem::new("system1".to_string(), "Test".to_string());
        let token = DesignToken::new(
            "token1".to_string(),
            "Color".to_string(),
            TokenType::Color,
            TokenValue::Color("#FF0000".to_string()),
        );
        system.add_token(token);
        assert_eq!(system.token_count(), 1);
        assert!(system.remove_token("token1"));
        assert_eq!(system.token_count(), 0);
    }

    #[test]
    fn test_design_system_component_management() {
        let mut system = DesignSystem::new("system1".to_string(), "Test".to_string());
        let component = UIComponent::new("btn1".to_string(), "Button".to_string(), "button".to_string());
        system.add_component(component);
        assert_eq!(system.component_count(), 1);
    }

    #[test]
    fn test_sigmadesign_creation() {
        let design = SigmaDesign::new();
        assert_eq!(design.design_system_count(), 0);
    }

    #[test]
    fn test_sigmadesign_system_management() {
        let mut design = SigmaDesign::new();
        let system = DesignSystem::new("system1".to_string(), "Test".to_string());
        design.add_design_system(system);
        assert_eq!(design.design_system_count(), 1);
    }
}
