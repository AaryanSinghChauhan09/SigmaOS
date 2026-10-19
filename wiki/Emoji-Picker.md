# SigmaOS Emoji Picker Components

## Overview
SigmaOS emoji picker providing native emoji selection with search and categories, competitive with Omarchy's emoji picker (SUPER+CTRL+E).

## Features
- **9 emoji categories**: Smileys, Animals, Food, Activities, Travel, Objects, Symbols, Flags, Custom
- **Keyword search** across names and keywords
- **Recent emojis** (up to 20)
- **Shortcodes** (e.g., :heart:, :smile:)
- **Default emojis** pre-populated
- **Insert** emoji into active field

## Structures

| Structure | Description |
|-----------|-------------|
| **EmojiPicker** | Emoji picker with emoji registry |
| **Emoji** | Individual emoji (character, name, category, keywords, shortcode) |
| **EmojiCategory** | Smileys, Animals, Food, Activities, Travel, Objects, Symbols, Flags, Custom |

## Usage Examples

```rust
// Create emoji picker
let mut picker = EmojiPicker::new();

// Search emojis
let results = picker.search("dog");

// Insert emoji
picker.insert("😀");

// Get recent emojis
let recent = picker.list_recent();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Desktop & UX (P1)

## Related Files
- `src/emoji/picker.rs` - Emoji picker implementation
- `wiki/Emoji-Picker.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, USER_FRIENDLY, SIMPLE_USER_INTERFACE,
CLEAR_ABSTRACTION, COMPOSITION_OVER_INHERITANCE