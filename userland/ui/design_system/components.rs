// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS Design System - UI Components

use serde::{Deserialize, Serialize};

/// Button component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Button {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
    pub loading: bool,
    pub label: String,
}

impl Button {
    pub fn new(label: &str) -> Self {
        Self {
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            disabled: false,
            loading: false,
            label: label.to_string(),
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
    Destructive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

/// Input component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub placeholder: String,
    pub value: String,
    pub disabled: bool,
    pub error: Option<String>,
    pub size: InputSize,
}

impl Input {
    pub fn new() -> Self {
        Self {
            placeholder: String::new(),
            value: String::new(),
            disabled: false,
            error: None,
            size: InputSize::Medium,
        }
    }

    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    pub fn value(mut self, value: &str) -> Self {
        self.value = value.to_string();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputSize {
    Small,
    Medium,
    Large,
}

/// Card component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub content: String,
    pub elevation: u32,
}

impl Card {
    pub fn new(content: &str) -> Self {
        Self {
            title: None,
            subtitle: None,
            content: content.to_string(),
            elevation: 1,
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn subtitle(mut self, subtitle: &str) -> Self {
        self.subtitle = Some(subtitle.to_string());
        self
    }
}

/// Modal component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Modal {
    pub title: String,
    pub content: String,
    pub size: ModalSize,
    pub closable: bool,
}

impl Modal {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
            size: ModalSize::Medium,
            closable: true,
        }
    }

    pub fn size(mut self, size: ModalSize) -> Self {
        self.size = size;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModalSize {
    Small,
    Medium,
    Large,
    Fullscreen,
}
