# SigmaOS Weather Components

## Overview
SigmaOS weather manager providing weather status and location, competitive with Omarchy weather (omancy-weather-status, omancy-weather-location, omancy-weather-icon).

## Features
- **Weather conditions**: Clear, PartlyCloudy, Cloudy, Rainy, Snowy, Stormy, Foggy, Windy, Unknown
- **Location management** - Set and get location
- **Units** - Celsius and Fahrenheit
- **Auto-update** - Configurable auto-update
- **Update interval** - Configurable update interval
- **Weather icons** - Emoji icons per condition

## Structures

| Structure | Description |
|-----------|-------------|
| **WeatherManager** | Weather manager |
| **WeatherData** | Individual weather data |
| **WeatherCondition** | Clear, PartlyCloudy, Cloudy, etc. |
| **WeatherUnits** | Celsius, Fahrenheit |

## Usage Examples

```rust
// Create weather manager
let mut mgr = WeatherManager::new();

// Set location
mgr.set_location("New York".to_string());

// Update weather
let data = WeatherData { ... };
mgr.update(data);

// Get data
let w = mgr.get_data().unwrap();

// Format temperature
let temp_str = mgr.format_temperature(25.0);

// Get icon
let icon = WeatherManager::get_icon_for_condition(&WeatherCondition::Rainy);
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Desktop & UX (P1)

## Related Files
- `src/weather/manager.rs` - Weather manager implementation
- `wiki/Weather.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, SECURE_DEFAULTS,
OPEN_STANDARD_AND_INTEROPERABILITY, DISCOVERABILITY