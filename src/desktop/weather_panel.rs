#![no_std]
extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Weather panel component for desktop
/// Provides text-based weather information display
/// inspired by Omarchy Linux weather panel

#[derive(Debug, Clone, PartialEq)]
pub enum WeatherCondition {
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
    Stormy,
    Foggy,
    Windy,
    PartlyCloudy,
}

#[derive(Debug, Clone)]
pub struct WeatherData {
    pub location: String,
    pub temperature_celsius: f32,
    pub temperature_fahrenheit: f32,
    pub humidity_percent: u8,
    pub wind_speed_kmh: f32,
    pub condition: WeatherCondition,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct WeatherForecast {
    pub date: String,
    pub high_temp_celsius: f32,
    pub low_temp_celsius: f32,
    pub condition: WeatherCondition,
}

/// Weather panel component
pub struct WeatherPanel {
    pub current_weather: Option<WeatherData>,
    pub forecast: Vec<WeatherForecast>,
    pub location: String,
    pub units_metric: bool,
}

impl WeatherPanel {
    pub fn new() -> Self {
        Self {
            current_weather: None,
            forecast: Vec::new(),
            location: String::from("Unknown"),
            units_metric: true,
        }
    }

    /// Set location
    pub fn set_location(&mut self, location: &str) {
        self.location = location.to_string();
    }

    /// Update current weather
    pub fn update_current_weather(&mut self, weather: WeatherData) {
        self.current_weather = Some(weather);
    }

    /// Add forecast entry
    pub fn add_forecast(&mut self, forecast: WeatherForecast) {
        self.forecast.push(forecast);
    }

    /// Clear forecast
    pub fn clear_forecast(&mut self) {
        self.forecast.clear();
    }

    /// Set units to metric
    pub fn set_metric_units(&mut self) {
        self.units_metric = true;
    }

    /// Set units to imperial
    pub fn set_imperial_units(&mut self) {
        self.units_metric = false;
    }

    /// Get current temperature
    pub fn get_current_temperature(&self) -> Option<f32> {
        self.current_weather.as_ref().map(|w| {
            if self.units_metric {
                w.temperature_celsius
            } else {
                w.temperature_fahrenheit
            }
        })
    }

    /// Get current condition
    pub fn get_current_condition(&self) -> Option<&WeatherCondition> {
        self.current_weather.as_ref().map(|w| &w.condition)
    }

    /// Display current weather in text-based format
    pub fn display_current_weather(&self) -> String {
        let mut output = String::from("=== Weather Panel ===\n\n");
        
        match &self.current_weather {
            Some(weather) => {
                output.push_str(&format!("Location: {}\n", weather.location));
                output.push_str(&format!("Condition: {}\n", self.condition_to_string(&weather.condition)));
                
                if self.units_metric {
                    output.push_str(&format!("Temperature: {:.1}°C\n", weather.temperature_celsius));
                } else {
                    output.push_str(&format!("Temperature: {:.1}°F\n", weather.temperature_fahrenheit));
                }
                
                output.push_str(&format!("Humidity: {}%\n", weather.humidity_percent));
                output.push_str(&format!("Wind Speed: {:.1} km/h\n", weather.wind_speed_kmh));
                output.push_str(&format!("Updated: {}\n", self.format_timestamp(weather.timestamp)));
            }
            None => {
                output.push_str("No weather data available.\n");
            }
        }
        
        output
    }

    /// Display forecast in text-based format
    pub fn display_forecast(&self) -> String {
        let mut output = String::from("=== Weather Forecast ===\n\n");
        
        if self.forecast.is_empty() {
            output.push_str("No forecast data available.\n");
            return output;
        }
        
        for (i, forecast) in self.forecast.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", i + 1, forecast.date));
            
            if self.units_metric {
                output.push_str(&format!("   High: {:.1}°C\n", forecast.high_temp_celsius));
                output.push_str(&format!("   Low: {:.1}°C\n", forecast.low_temp_celsius));
            } else {
                output.push_str(&format!("   High: {:.1}°F\n", self.celsius_to_fahrenheit(forecast.high_temp_celsius)));
                output.push_str(&format!("   Low: {:.1}°F\n", self.celsius_to_fahrenheit(forecast.low_temp_celsius)));
            }
            
            output.push_str(&format!("   Condition: {}\n", self.condition_to_string(&forecast.condition)));
            output.push_str("\n");
        }
        
        output.push_str(&format!("Total: {} days\n", self.forecast.len()));
        output
    }

    /// Display full weather panel
    pub fn display(&self) -> String {
        let mut output = self.display_current_weather();
        output.push_str("\n");
        output.push_str(&self.display_forecast());
        output
    }

    /// Convert Celsius to Fahrenheit
    fn celsius_to_fahrenheit(&self, celsius: f32) -> f32 {
        celsius * 9.0 / 5.0 + 32.0
    }

    /// Convert Fahrenheit to Celsius
    fn fahrenheit_to_celsius(&self, fahrenheit: f32) -> f32 {
        (fahrenheit - 32.0) * 5.0 / 9.0
    }

    /// Format timestamp
    fn format_timestamp(&self, timestamp: u64) -> String {
        format!("Timestamp: {}", timestamp)
    }

    /// Convert condition to string
    fn condition_to_string(&self, condition: &WeatherCondition) -> String {
        match condition {
            WeatherCondition::Sunny => String::from("Sunny"),
            WeatherCondition::Cloudy => String::from("Cloudy"),
            WeatherCondition::Rainy => String::from("Rainy"),
            WeatherCondition::Snowy => String::from("Snowy"),
            WeatherCondition::Stormy => String::from("Stormy"),
            WeatherCondition::Foggy => String::from("Foggy"),
            WeatherCondition::Windy => String::from("Windy"),
            WeatherCondition::PartlyCloudy => String::from("Partly Cloudy"),
        }
    }
}

impl Default for WeatherPanel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_panel_creation() {
        let panel = WeatherPanel::new();
        assert!(panel.current_weather.is_none());
        assert!(panel.forecast.is_empty());
        assert!(panel.units_metric);
    }

    #[test]
    fn test_location_set() {
        let mut panel = WeatherPanel::new();
        panel.set_location("San Francisco");
        assert_eq!(panel.location, "San Francisco");
    }

    #[test]
    fn test_current_weather_update() {
        let mut panel = WeatherPanel::new();
        let weather = WeatherData {
            location: String::from("San Francisco"),
            temperature_celsius: 20.0,
            temperature_fahrenheit: 68.0,
            humidity_percent: 65,
            wind_speed_kmh: 10.0,
            condition: WeatherCondition::Sunny,
            timestamp: 0,
        };
        
        panel.update_current_weather(weather);
        assert!(panel.current_weather.is_some());
    }

    #[test]
    fn test_forecast_management() {
        let mut panel = WeatherPanel::new();
        
        let forecast = WeatherForecast {
            date: String::from("2024-01-01"),
            high_temp_celsius: 25.0,
            low_temp_celsius: 15.0,
            condition: WeatherCondition::Sunny,
        };
        
        panel.add_forecast(forecast);
        assert_eq!(panel.forecast.len(), 1);
        
        panel.clear_forecast();
        assert!(panel.forecast.is_empty());
    }

    #[test]
    fn test_units_toggle() {
        let mut panel = WeatherPanel::new();
        
        assert!(panel.units_metric);
        panel.set_imperial_units();
        assert!(!panel.units_metric);
        panel.set_metric_units();
        assert!(panel.units_metric);
    }

    #[test]
    fn test_temperature_conversion() {
        let panel = WeatherPanel::new();
        
        let celsius = panel.fahrenheit_to_celsius(68.0);
        assert!((celsius - 20.0).abs() < 0.1);
        
        let fahrenheit = panel.celsius_to_fahrenheit(20.0);
        assert!((fahrenheit - 68.0).abs() < 0.1);
    }

    #[test]
    fn test_get_current_temperature() {
        let mut panel = WeatherPanel::new();
        let weather = WeatherData {
            location: String::from("San Francisco"),
            temperature_celsius: 20.0,
            temperature_fahrenheit: 68.0,
            humidity_percent: 65,
            wind_speed_kmh: 10.0,
            condition: WeatherCondition::Sunny,
            timestamp: 0,
        };
        
        panel.update_current_weather(weather);
        
        panel.set_metric_units();
        let temp_c = panel.get_current_temperature();
        assert_eq!(temp_c, Some(20.0));
        
        panel.set_imperial_units();
        let temp_f = panel.get_current_temperature();
        assert_eq!(temp_f, Some(68.0));
    }

    #[test]
    fn test_display_output() {
        let mut panel = WeatherPanel::new();
        panel.set_location("San Francisco");
        
        let weather = WeatherData {
            location: String::from("San Francisco"),
            temperature_celsius: 20.0,
            temperature_fahrenheit: 68.0,
            humidity_percent: 65,
            wind_speed_kmh: 10.0,
            condition: WeatherCondition::Sunny,
            timestamp: 0,
        };
        
        panel.update_current_weather(weather);
        
        let output = panel.display_current_weather();
        assert!(output.contains("San Francisco"));
        assert!(output.contains("Sunny"));
        assert!(output.contains("20.0°C"));
    }

    #[test]
    fn test_forecast_display() {
        let mut panel = WeatherPanel::new();
        
        let forecast = WeatherForecast {
            date: String::from("2024-01-01"),
            high_temp_celsius: 25.0,
            low_temp_celsius: 15.0,
            condition: WeatherCondition::Sunny,
        };
        
        panel.add_forecast(forecast);
        
        let output = panel.display_forecast();
        assert!(output.contains("2024-01-01"));
        assert!(output.contains("25.0°C"));
    }
}
