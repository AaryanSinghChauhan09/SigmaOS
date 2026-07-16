// SigmaOS Customization Engine
// Samsung Modes & Routines-style automation and theming

use std::collections::HashMap;

/// Automation trigger type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerType {
    Time,
    Location,
    Device,
    Application,
    SystemEvent,
    Manual,
}

/// Automation condition
#[derive(Debug, Clone)]
pub struct Condition {
    pub trigger_type: TriggerType,
    pub value: String,
    pub operator: String, // "equals", "contains", "greater_than", etc.
}

impl Condition {
    pub fn new(trigger_type: TriggerType, value: String) -> Self {
        Self {
            trigger_type,
            value,
            operator: "equals".to_string(),
        }
    }

    pub fn with_operator(mut self, operator: String) -> Self {
        self.operator = operator;
        self
    }

    pub fn evaluate(&self, current_value: &str) -> bool {
        match self.operator.as_str() {
            "equals" => current_value == self.value,
            "contains" => current_value.contains(&self.value),
            "greater_than" => {
                if let (Ok(curr), Ok(val)) =
                    (current_value.parse::<f64>(), self.value.parse::<f64>())
                {
                    curr > val
                } else {
                    false
                }
            }
            "less_than" => {
                if let (Ok(curr), Ok(val)) =
                    (current_value.parse::<f64>(), self.value.parse::<f64>())
                {
                    curr < val
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

/// Automation action
#[derive(Debug, Clone)]
pub enum Action {
    SetTheme { theme: String },
    LaunchApp { app: String },
    SetVolume { level: u8 },
    EnableFeature { feature: String },
    DisableFeature { feature: String },
    RunScript { script: String },
    SendNotification { message: String },
}

/// Automation routine
#[derive(Debug, Clone)]
pub struct Routine {
    pub id: String,
    pub name: String,
    pub conditions: Vec<Condition>,
    pub actions: Vec<Action>,
    pub enabled: bool,
    pub priority: u32,
}

impl Routine {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            conditions: Vec::new(),
            actions: Vec::new(),
            enabled: true,
            priority: 0,
        }
    }

    pub fn with_condition(mut self, condition: Condition) -> Self {
        self.conditions.push(condition);
        self
    }

    pub fn with_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }

    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn should_trigger(&self, context: &HashMap<String, String>) -> bool {
        if !self.enabled {
            return false;
        }

        self.conditions.iter().all(|condition| {
            let current_value = context.get(&condition.value).unwrap_or(&String::new());
            condition.evaluate(current_value)
        })
    }
}

/// Theme configuration
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub colors: HashMap<String, String>,
    pub fonts: HashMap<String, String>,
    pub icon_set: String,
    pub window_decorations: bool,
    pub animations_enabled: bool,
}

impl Theme {
    pub fn new(name: String) -> Self {
        Self {
            name,
            colors: HashMap::new(),
            fonts: HashMap::new(),
            icon_set: "default".to_string(),
            window_decorations: true,
            animations_enabled: true,
        }
    }

    pub fn with_color(mut self, key: String, value: String) -> Self {
        self.colors.insert(key, value);
        self
    }

    pub fn with_font(mut self, key: String, value: String) -> Self {
        self.fonts.insert(key, value);
        self
    }

    pub fn with_icon_set(mut self, icon_set: String) -> Self {
        self.icon_set = icon_set;
        self
    }

    pub fn get_color(&self, key: &str) -> Option<&String> {
        self.colors.get(key)
    }
}

/// Customization engine
pub struct CustomizationEngine {
    pub routines: HashMap<String, Routine>,
    pub themes: HashMap<String, Theme>,
    pub active_theme: Option<String>,
    pub context: HashMap<String, String>,
}

impl CustomizationEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            routines: HashMap::new(),
            themes: HashMap::new(),
            active_theme: None,
            context: HashMap::new(),
        };

        engine.add_default_themes();
        engine.add_default_routines();
        engine
    }

    fn add_default_themes(&mut self) {
        let dark_theme = Theme::new("Dark".to_string())
            .with_color("background".to_string(), "#1e1e1e".to_string())
            .with_color("foreground".to_string(), "#ffffff".to_string())
            .with_color("accent".to_string(), "#0078d4".to_string())
            .with_font("system".to_string(), "Segoe UI".to_string());

        let light_theme = Theme::new("Light".to_string())
            .with_color("background".to_string(), "#ffffff".to_string())
            .with_color("foreground".to_string(), "#000000".to_string())
            .with_color("accent".to_string(), "#0078d4".to_string())
            .with_font("system".to_string(), "Segoe UI".to_string());

        self.themes.insert(dark_theme.name.clone(), dark_theme);
        self.themes.insert(light_theme.name.clone(), light_theme);
    }

    fn add_default_routines(&mut self) {
        let work_mode = Routine::new("work_mode".to_string(), "Work Mode".to_string())
            .with_condition(
                Condition::new(TriggerType::Time, "09:00".to_string())
                    .with_operator("equals".to_string()),
            )
            .with_action(Action::SetTheme {
                theme: "Light".to_string(),
            })
            .with_action(Action::LaunchApp {
                app: "email".to_string(),
            })
            .with_priority(10);

        let night_mode = Routine::new("night_mode".to_string(), "Night Mode".to_string())
            .with_condition(
                Condition::new(TriggerType::Time, "20:00".to_string())
                    .with_operator("equals".to_string()),
            )
            .with_action(Action::SetTheme {
                theme: "Dark".to_string(),
            })
            .with_action(Action::EnableFeature {
                feature: "blue_light_filter".to_string(),
            })
            .with_priority(10);

        self.routines.insert(work_mode.id.clone(), work_mode);
        self.routines.insert(night_mode.id.clone(), night_mode);
    }

    pub fn add_routine(&mut self, routine: Routine) {
        self.routines.insert(routine.id.clone(), routine);
    }

    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.insert(theme.name.clone(), theme);
    }

    pub fn set_active_theme(&mut self, name: &str) -> Result<(), CustomizationError> {
        if !self.themes.contains_key(name) {
            return Err(CustomizationError::ThemeNotFound);
        }
        self.active_theme = Some(name.to_string());
        Ok(())
    }

    pub fn get_active_theme(&self) -> Option<&Theme> {
        self.active_theme
            .as_ref()
            .and_then(|name| self.themes.get(name))
    }

    pub fn update_context(&mut self, key: String, value: String) {
        self.context.insert(key, value);
    }

    pub fn evaluate_routines(&mut self) -> Vec<Action> {
        let mut triggered_actions = Vec::new();

        for routine in self.routines.values() {
            if routine.should_trigger(&self.context) {
                triggered_actions.extend(routine.actions.clone());
            }
        }

        triggered_actions
    }

    pub fn execute_action(&mut self, action: Action) -> Result<(), CustomizationError> {
        match action {
            Action::SetTheme { theme } => {
                self.set_active_theme(&theme)?;
            }
            Action::LaunchApp { app } => {
                // Simulate app launch
                println!("Launching app: {}", app);
            }
            Action::SetVolume { level } => {
                println!("Setting volume to: {}", level);
            }
            Action::EnableFeature { feature } => {
                println!("Enabling feature: {}", feature);
            }
            Action::DisableFeature { feature } => {
                println!("Disabling feature: {}", feature);
            }
            Action::RunScript { script } => {
                println!("Running script: {}", script);
            }
            Action::SendNotification { message } => {
                println!("Notification: {}", message);
            }
        }
        Ok(())
    }

    pub fn list_routines(&self) -> Vec<&Routine> {
        self.routines.values().collect()
    }

    pub fn list_themes(&self) -> Vec<&Theme> {
        self.themes.values().collect()
    }
}

impl Default for CustomizationEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Customization errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CustomizationError {
    ThemeNotFound,
    RoutineNotFound,
    InvalidAction,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = CustomizationEngine::new();
        assert_eq!(engine.themes.len(), 2);
        assert_eq!(engine.routines.len(), 2);
    }

    #[test]
    fn test_theme_switching() {
        let mut engine = CustomizationEngine::new();
        assert!(engine.set_active_theme("Dark").is_ok());
        assert_eq!(engine.active_theme, Some("Dark".to_string()));
    }

    #[test]
    fn test_invalid_theme() {
        let mut engine = CustomizationEngine::new();
        assert!(engine.set_active_theme("Nonexistent").is_err());
    }

    #[test]
    fn test_condition_evaluation() {
        let condition = Condition::new(TriggerType::Time, "09:00".to_string());
        assert!(condition.evaluate("09:00"));
        assert!(!condition.evaluate("10:00"));
    }

    #[test]
    fn test_routine_creation() {
        let routine = Routine::new("test".to_string(), "Test".to_string())
            .with_condition(Condition::new(TriggerType::Time, "09:00".to_string()))
            .with_action(Action::SetTheme {
                theme: "Dark".to_string(),
            });
        assert_eq!(routine.conditions.len(), 1);
        assert_eq!(routine.actions.len(), 1);
    }
}
