// SPDX-License-Identifier: MIT
// SigmaOS - Weather Application
// Inspired by GNOME Weather and Weather Underground
// Provides weather information and forecasts

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Weather condition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeatherCondition {
    Clear,
    Cloudy,
    PartlyCloudy,
    Rain,
    Thunderstorm,
    Snow,
    Mist,
    Fog,
    Haze,
    Windy,
}

/// Temperature unit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

/// Wind direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

/// Current weather
#[derive(Debug, Clone)]
pub struct CurrentWeather {
    pub location: String,
    pub temperature: f32,
    pub feels_like: f32,
    pub humidity: u32,
    pub wind_speed: f32,
    wind_direction: WindDirection,
    pub condition: WeatherCondition,
    pub visibility: f32,
    pub pressure: f32,
    pub uv_index: u32,
}

/// Forecast day
#[derive(Debug, Clone)]
pub struct ForecastDay {
    pub date: String,
    pub high_temp: f32,
    pub low_temp: f32,
    pub condition: WeatherCondition,
    pub humidity: u32,
    pub wind_speed: f32,
    pub precipitation_chance: u32,
}

/// Weather location
#[derive(Debug, Clone)]
pub struct WeatherLocation {
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
    pub country: String,
    pub timezone: String,
}

/// Weather app
pub struct WeatherApp {
    pub locations: Vec<WeatherLocation>,
    pub current_weather: Option<CurrentWeather>,
    pub forecast: Vec<ForecastDay>,
    pub temperature_unit: TemperatureUnit,
    pub auto_refresh: bool,
    pub refresh_interval: u32,
}

impl WeatherApp {
    pub fn new() -> Self {
        Self {
            locations: Vec::new(),
            current_weather: None,
            forecast: Vec::new(),
            temperature_unit: TemperatureUnit::Celsius,
            auto_refresh: false,
            refresh_interval: 3600, // 1 hour
        }
    }

    /// Add location
    pub fn add_location(
        &mut self,
        name: String,
        latitude: f32,
        longitude: f32,
        country: String,
        timezone: String,
    ) {
        let location = WeatherLocation {
            name,
            latitude,
            longitude,
            country,
            timezone,
        };
        self.locations.push(location);
    }

    /// Remove location
    pub fn remove_location(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.locations.len() {
            return Err("Location index out of bounds");
        }
        self.locations.remove(index);
        Ok(())
    }

    /// Set active location
    pub fn set_active_location(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.locations.len() {
            return Err("Location index out of bounds");
        }
        // Simplified - in real implementation would fetch weather for this location
        Ok(())
    }

    /// Refresh weather
    pub fn refresh(&mut self) {
        // Simplified - in real implementation would fetch from weather API
        if let Some(location) = self.locations.first() {
            self.current_weather = Some(CurrentWeather {
                location: location.name.clone(),
                temperature: 22.0,
                feels_like: 21.0,
                humidity: 65,
                wind_speed: 12.0,
                wind_direction: WindDirection::North,
                condition: WeatherCondition::PartlyCloudy,
                visibility: 10.0,
                pressure: 1013.0,
                uv_index: 5,
            });
        }
    }

    /// Get current weather
    pub fn get_current_weather(&self) -> Option<&CurrentWeather> {
        self.current_weather.as_ref()
    }

    /// Get forecast
    pub fn get_forecast(&self) -> &[ForecastDay] {
        &self.forecast
    }

    /// Set temperature unit
    pub fn set_temperature_unit(&mut self, unit: TemperatureUnit) {
        self.temperature_unit = unit;
    }

    /// Convert temperature
    pub fn convert_temperature(
        &self,
        temp: f32,
        from: TemperatureUnit,
        to: TemperatureUnit,
    ) -> f32 {
        match (from, to) {
            (TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit) => temp * 9.0 / 5.0 + 32.0,
            (TemperatureUnit::Celsius, TemperatureUnit::Kelvin) => temp + 273.15,
            (TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius) => (temp - 32.0) * 5.0 / 9.0,
            (TemperatureUnit::Fahrenheit, TemperatureUnit::Kelvin) => {
                (temp - 32.0) * 5.0 / 9.0 + 273.15
            }
            (TemperatureUnit::Kelvin, TemperatureUnit::Celsius) => temp - 273.15,
            (TemperatureUnit::Kelvin, TemperatureUnit::Fahrenheit) => {
                (temp - 273.15) * 9.0 / 5.0 + 32.0
            }
            _ => temp,
        }
    }

    /// Set auto refresh
    pub fn set_auto_refresh(&mut self, enabled: bool) {
        self.auto_refresh = enabled;
    }

    /// Set refresh interval
    pub fn set_refresh_interval(&mut self, interval: u32) {
        self.refresh_interval = interval.max(60); // Minimum 1 minute
    }

    /// Get location count
    pub fn location_count(&self) -> usize {
        self.locations.len()
    }

    /// Forecast days count
    pub fn forecast_days_count(&self) -> usize {
        self.forecast.len()
    }
}

/// Global weather app
static mut WEATHER_APP: Option<WeatherApp> = None;

/// Initialize weather app
pub fn init_weather_app() {
    unsafe {
        WEATHER_APP = Some(WeatherApp::new());
    }
}

/// Get weather app
pub fn get_weather_app() -> &'static mut WeatherApp {
    unsafe { WEATHER_APP.as_mut().expect("Weather app not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_app_creation() {
        let app = WeatherApp::new();
        assert_eq!(app.location_count(), 0);
    }

    #[test]
    fn test_add_location() {
        let mut app = WeatherApp::new();
        app.add_location(
            "New York".to_string(),
            40.7128,
            -74.0060,
            "US".to_string(),
            "America/New_York".to_string(),
        );
        assert_eq!(app.location_count(), 1);
    }

    #[test]
    fn test_remove_location() {
        let mut app = WeatherApp::new();
        app.add_location(
            "Test".to_string(),
            0.0,
            0.0,
            "US".to_string(),
            "UTC".to_string(),
        );
        app.remove_location(0).unwrap();
        assert_eq!(app.location_count(), 0);
    }

    #[test]
    fn test_convert_temperature() {
        let app = WeatherApp::new();
        let celsius = 20.0;
        let fahrenheit = app.convert_temperature(
            celsius,
            TemperatureUnit::Celsius,
            TemperatureUnit::Fahrenheit,
        );
        assert!((fahrenheit - 68.0).abs() < 0.1);
    }

    #[test]
    fn test_refresh() {
        let mut app = WeatherApp::new();
        app.add_location(
            "Test".to_string(),
            0.0,
            0.0,
            "US".to_string(),
            "UTC".to_string(),
        );
        app.refresh();
        assert!(app.get_current_weather().is_some());
    }
}
