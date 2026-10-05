# Desktop Input Method Manager

## Overview

The Desktop Input Method Manager provides comprehensive input method management inspired by Linux Mint's input method framework and Omarchy's IME utilities. It supports multiple input method frameworks, language layouts, and keyboard configuration.

## Features

- **Input Method Types**: IBus, Fcitx, XIM, None
- **Input Method Engines**: Multiple engines with language and layout configuration
- **Engine Management**: Add, remove, and switch between engines
- **Language Support**: Multiple language engines (English, German, French, Japanese, etc.)
- **Layout Configuration**: Keyboard layout per engine
- **Variant Support**: Layout variants (e.g., kana for Japanese)
- **Current Engine**: Track and switch current input method engine
- **Engine Filtering**: Filter engines by language
- **Default Engines**: Pre-configured engines for common languages
- **Statistics**: Track total engines and current engine status

## Components

### InputMethodType

```rust
pub enum InputMethodType {
    IBus,   // IBus input method framework
    Fcitx,  // Fcitx input method framework
    Xim,    // XIM input method
    None,   // No input method
}
```

### InputMethodEngine

Input method engine with:
- Engine ID and name
- Language code
- Keyboard layout
- Layout variant (optional)

### DesktopInputMethodManager

Main management interface with:
- Input method type selection
- Engine management (add, remove, switch)
- Language and layout configuration
- Variant support
- Engine filtering by language
- Default engines for common languages
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopInputMethodManager;

let mut manager = DesktopInputMethodManager::new();

// Get configuration
println!("Input method type: {}", manager.get_input_method_type().as_str());
println!("Total engines: {}", manager.get_engines().len());
```

### Input Method Type Selection

```rust
// Set input method type
manager.set_input_method_type(InputMethodType::IBus);
manager.set_input_method_type(InputMethodType::Fcitx);
manager.set_input_method_type(InputMethodType::None);
```

### Engine Management

```rust
// Add engine
let id = manager.add_engine(
    "Spanish".to_string(),
    "es".to_string(),
    "es".to_string(),
);

// Remove engine
manager.remove_engine(&id);

// Switch current engine
manager.set_current_engine(&id);
```

### Engine Filtering

```rust
// Get all engines
let engines = manager.get_engines();

// Get engines by language
let english = manager.get_engines_by_language("en");
println!("English engines: {}", english.len());
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total engines: {}", stats.total_engines);
println!("Current engine set: {}", stats.current_engine);
println!("Input method type: {}", stats.input_method_type.as_str());
```

## Default Engines

The manager includes default engines for common languages:

- **English (US)**: us layout
- **English (UK)**: gb layout
- **German**: de layout
- **French**: fr layout
- **Japanese**: jp layout with kana variant

## Default Configuration

The Input Method Manager includes default configuration:

- **Input Method Type**: IBus
- **Current Engine**: English (US)
- **Default Engines**: 5 pre-configured engines

## AI Agent Maintenance Instructions

When maintaining the Input Method Manager:

1. **IME Integration**: Integrate with actual IME backends (IBus, Fcitx, etc.)
2. **Layout Detection**: Detect available keyboard layouts from system
3. **Hotkey Switching**: Implement hotkey-based engine switching
4. **Per-Application Settings**: Add per-application input method configuration
5. **Candidate Window**: Implement candidate window integration
6. **Composition**: Implement text composition handling
7. **Dictionary Support**: Add dictionary management for IME
8. **Auto-switch**: Implement auto-switch based on application focus
9. **Engine Reload**: Support dynamic engine reloading
10. **Configuration Import/Export**: Add configuration import/export

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::input_method_manager
```

## Future Enhancements

- Integration with actual IME backends
- Keyboard layout detection
- Hotkey-based engine switching
- Per-application input method configuration
- Candidate window integration
- Text composition handling
- Dictionary management
- Auto-switch based on application focus
- Dynamic engine reloading
- Configuration import/export
