// SPDX-License-Identifier: MIT
// SigmaOS - Template Manager
// Template management inspired by Omarchy themed templates and Linux template systems

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Template
#[derive(Debug, Clone)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub content: String,
    pub variables: Vec<String>,
}

/// Template manager
pub struct SigmaTemplateManager {
    templates: Vec<Template>,
}

impl SigmaTemplateManager {
    pub fn new() -> Self {
        Self {
            templates: Vec::new(),
        }
    }

    pub fn add_template(&mut self, template: Template) {
        self.templates.push(template);
    }

    pub fn get_templates(&self) -> &[Template] {
        &self.templates
    }

    pub fn get_template(&self, id: &str) -> Option<&Template> {
        self.templates.iter().find(|t| t.id == id)
    }

    pub fn render_template(&self, id: &str, variables: &[(String, String)]) -> Result<String, String> {
        if let Some(template) = self.get_template(id) {
            let mut content = template.content.clone();
            for (key, value) in variables {
                content = content.replace(&alloc::format!("{{{{{}}}}}", key), value);
            }
            Ok(content)
        } else {
            Err(format!("Template {} not found", id))
        }
    }
}

impl Default for SigmaTemplateManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_manager_creation() {
        let manager = SigmaTemplateManager::new();
        assert_eq!(manager.get_templates().len(), 0);
    }

    #[test]
    fn test_add_template() {
        let mut manager = SigmaTemplateManager::new();
        let template = Template {
            id: "config".to_string(),
            name: "Config Template".to_string(),
            content: "{{color}}".to_string(),
            variables: vec!["color".to_string()],
        };
        manager.add_template(template);
        assert_eq!(manager.get_templates().len(), 1);
    }

    #[test]
    fn test_render_template() {
        let mut manager = SigmaTemplateManager::new();
        let template = Template {
            id: "config".to_string(),
            name: "Config Template".to_string(),
            content: "color: {{color}}".to_string(),
            variables: vec!["color".to_string()],
        };
        manager.add_template(template);
        let rendered = manager.render_template("config", &[("color".to_string(), "red".to_string())]).unwrap();
        assert_eq!(rendered, "color: red");
    }
}
