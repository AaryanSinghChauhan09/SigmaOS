//! SigmaOS Weather Manager
//!
//! Inspired by: Omarchy weather (omancy-weather-status, omancy-weather-location, omancy-weather-icon)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, OPEN_STANDARD_AND_INTEROPERABILITY, DISCOVERABILITY.

/// Weather condition
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatherCondition {
    Clear,
    PartlyCloudy,
    Cloudy,
    Rainy,
    Snowy,
    Stormy,
    Foggy,
    Windy,
    Unknown,
}

/// Weather data
#[derive(Debug, Clone)]
pub struct WeatherData {
    pub location: String,
    pub temperature: f32,
    pub condition: WeatherCondition,
    pub humidity: u32,
    pub wind_speed: f32,
    pub icon: String,
    pub updated_at: u64,
}

/// Weather manager
pub struct WeatherManager {
    pub data: Option<WeatherData>,
    pub location: String,
    pub units: WeatherUnits,
    pub auto_update: bool,
    pub update_interval_seconds: u64,
}

/// Weather units
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatherUnits {
    Celsius,
    Fahrenheit,
}

impl WeatherManager {
    pub fn new() -> Self {
        Self {
            data: None,
            location: "Auto-detect".to_string(),
            units: WeatherUnits::Celsius,
            auto_update: true,
            update_interval_seconds: 1800, // 30 minutes
        }
    }

    /// Set location
    pub fn set_location(&mut self, location: String) {
        self.location = location;
        self.data = None; // Invalidate cached data
    }

    /// Get location
    pub fn get_location(&self) -> &str {
        &self.location
    }

    /// Set units
    pub fn set_units(&mut self, units: WeatherUnits) {
        self.units = units;
    }

    /// Update weather data
    pub fn update(&mut self, data: WeatherData) {
        self.data = Some(data);
    }

    /// Get weather data
    pub fn get_data(&self) -> Option<&WeatherData> {
        self.data.as_ref()
    }

    /// Is auto-update enabled
    pub fn is_auto_update(&self) -> bool {
        self.auto_update
    }

    /// Toggle auto-update
    pub fn toggle_auto_update(&mut self) -> bool {
        self.auto_update = !self.auto_update;
        self.auto_update
    }

    /// Set update interval
    pub fn set_update_interval(&mut self, seconds: u64) -> Result<(), String> {
        if seconds < 60 {
            return Err("Update interval must be at least 60 seconds".to_string());
        }
        self.update_interval_seconds = seconds;
        Ok(())
    }

    /// Get weather icon for condition
    pub fn get_icon_for_condition(condition: &WeatherCondition) -> &str {
        match condition {
            WeatherCondition::Clear => "☀️",
            WeatherCondition::PartlyCloudy => "⛅",
            WeatherCondition::Cloudy => "☁️",
            WeatherCondition::Rainy => "🌧️",
            WeatherCondition::Snowy => "❄️",
            WeatherCondition::Stormy => "⛈️",
            WeatherCondition::Foggy => "🌫️",
            WeatherCondition::Windy => "💨",
            WeatherCondition::Unknown => "❓",
        }
    }

    /// Format temperature
    pub fn format_temperature(&self, temp: f32) -> String {
        match self.units {
            WeatherUnits::Celsius => format!("{:.1}°C", temp),
            WeatherUnits::Fahrenheit => format!("{:.1}°F", temp * 9.0 / 5.0 + 32.0),
        }
    }

    /// Data count
    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }
}

impl Default for WeatherManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_location() {
        let mut mgr = WeatherManager::new();
        mgr.set_location("New York".to_string());
        assert_eq!(mgr.get_location(), "New York");
        assert!(!mgr.has_data());
    }

    #[test]
    fn test_update_weather() {
        let mut mgr = WeatherManager::new();
        let data = WeatherData {
            location: "London".to_string(),
            temperature: 15.5,
            condition: WeatherCondition::Rainy,
            humidity: 80,
            wind_speed: 12.5,
            icon: "🌧️".to_string(),
            updated_at: now_secs(),
        };
        mgr.update(data);
        assert!(mgr.has_data());
        let w = mgr.get_data().unwrap();
        assert_eq!(w.location, "London");
        assert_eq!(w.condition, WeatherCondition::Rainy);
    }

    #[test]
    fn test_set_units() {
        let mut mgr = WeatherManager::new();
        mgr.set_units(WeatherUnits::Fahrenheit);
        assert_eq!(mgr.format_temperature(0.0), "32.0°F");
        assert_eq!(mgr.format_temperature(100.0), "212.0°F");
    }

    #[test]
    fn test_celsius_format() {
        let mgr = WeatherManager::new();
        assert_eq!(mgr.format_temperature(25.0), "25.0°C");
    }

    #[test]
    fn test_toggle_auto_update() {
        let mut mgr = WeatherManager::new();
        assert!(mgr.auto_update);
        assert!(!mgr.toggle_auto_update());
        assert!(!mgr.auto_update);
    }

    #[test]
    fn test_set_update_interval() {
        let mut mgr = WeatherManager::new();
        assert!(mgr.set_update_interval(300).is_ok());
        assert_eq!(mgr.update_interval_seconds, 300);
        assert!(mgr.set_update_interval(30).is_err());
    }

    #[test]
    fn test_weather_icon() {
        assert_eq!(WeatherManager::get_icon_for_condition(&WeatherCondition::Clear), "☀️");
        assert_eq!(WeatherManager::get_icon_for_condition(&WeatherCondition::Stormy), "⛈️");
    }
}