// SPDX-License-Identifier: MIT
// SigmaOS - Weather Map
// Inspired by GNOME Weather and OpenWeatherMap
// Provides weather map visualization

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Weather map layer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapLayer {
    Temperature,
    Precipitation,
    Wind,
    Clouds,
    Pressure,
}

/// Map zoom level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapZoom {
    World,
    Continent,
    Country,
    Region,
    City,
}

/// Weather map location
#[derive(Debug, Clone)]
pub struct MapLocation {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub temperature: f32,
    pub condition: String,
}

/// Weather map
pub struct WeatherMap {
    pub center_lat: f64,
    pub center_lon: f64,
    pub zoom: MapZoom,
    pub active_layer: MapLayer,
    pub locations: Vec<MapLocation>,
    pub animation_enabled: bool,
    pub show_labels: bool,
}

impl WeatherMap {
    pub fn new() -> Self {
        Self {
            center_lat: 0.0,
            center_lon: 0.0,
            zoom: MapZoom::World,
            active_layer: MapLayer::Temperature,
            locations: Vec::new(),
            animation_enabled: false,
            show_labels: true,
        }
    }

    /// Set center
    pub fn set_center(&mut self, lat: f64, lon: f64) {
        self.center_lat = lat;
        self.center_lon = lon;
    }

    /// Set zoom
    pub fn set_zoom(&mut self, zoom: MapZoom) {
        self.zoom = zoom;
    }

    /// Set active layer
    pub fn set_active_layer(&mut self, layer: MapLayer) {
        self.active_layer = layer;
    }

    /// Add location
    pub fn add_location(&mut self, location: MapLocation) {
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

    /// Toggle animation
    pub fn toggle_animation(&mut self) {
        self.animation_enabled = !self.animation_enabled;
    }

    /// Toggle labels
    pub fn toggle_labels(&mut self) {
        self.show_labels = !self.show_labels;
    }

    /// Zoom in
    pub fn zoom_in(&mut self) {
        match self.zoom {
            MapZoom::World => self.zoom = MapZoom::Continent,
            MapZoom::Continent => self.zoom = MapZoom::Country,
            MapZoom::Country => self.zoom = MapZoom::Region,
            MapZoom::Region => self.zoom = MapZoom::City,
            MapZoom::City => {} // Already at max zoom
        }
    }

    /// Zoom out
    pub fn zoom_out(&mut self) {
        match self.zoom {
            MapZoom::City => self.zoom = MapZoom::Region,
            MapZoom::Region => self.zoom = MapZoom::Country,
            MapZoom::Country => self.zoom = MapZoom::Continent,
            MapZoom::Continent => self.zoom = MapZoom::World,
            MapZoom::World => {} // Already at min zoom
        }
    }

    /// Get locations
    pub fn get_locations(&self) -> &[MapLocation] {
        &self.locations
    }

    /// Location count
    pub fn location_count(&self) -> usize {
        self.locations.len()
    }
}

/// Global weather map
static mut WEATHER_MAP: Option<WeatherMap> = None;

/// Initialize weather map
pub fn init_weather_map() {
    unsafe {
        WEATHER_MAP = Some(WeatherMap::new());
    }
}

/// Get weather map
pub fn get_weather_map() -> &'static mut WeatherMap {
    unsafe { WEATHER_MAP.as_mut().expect("Weather map not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_map_creation() {
        let wm = WeatherMap::new();
        assert_eq!(wm.location_count(), 0);
    }

    #[test]
    fn test_add_location() {
        let mut wm = WeatherMap::new();
        let location = MapLocation {
            name: "New York".to_string(),
            latitude: 40.7128,
            longitude: -74.0060,
            temperature: 20.0,
            condition: "Sunny".to_string(),
        };
        wm.add_location(location);
        assert_eq!(wm.location_count(), 1);
    }

    #[test]
    fn test_zoom() {
        let mut wm = WeatherMap::new();
        wm.zoom_in();
        assert_eq!(wm.zoom, MapZoom::Continent);
        wm.zoom_out();
        assert_eq!(wm.zoom, MapZoom::World);
    }
}
