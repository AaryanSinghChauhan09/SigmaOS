# SigmaOS Input Method Framework Specification (`Zenith Input Engine`)

## 1. Overview

The Zenith Input Engine provides comprehensive support for complex non-Latin writing systems—including Chinese (Pinyin, Cangjie), Japanese (Anthology, Mozc/Romaji-to-Kana), Korean (Hangul), Indic scripts, Arabic, and custom symbol layouts. It functions as a lightweight, sandboxed service analogous to IBus and Fcitx5.

```
+-------------------------------------------------------------+
|                     Focused GUI Application                 |
|  (Receives committed UTF-8 text and composition pre-edit)   |
+------------------------------^------------------------------+
                               | Zenith Input Method Protocol
+------------------------------v------------------------------+
|                     Zenith Input Daemon                     |
|  +--------------------+  +--------------------------------+ |
|  | Engine Router      |  | Candidate UI Overlay Window    | |
|  +---------+----------+  +----------------+---------------+ |
+------------|------------------------------|-----------------+
             |                              |
+------------v------------------------------v-----------------+
|                    Active Input Engine Plugin               |
|  (Pinyin Engine / Mozc Engine / Hangul Engine / Symbol Table)|
+-------------------------------------------------------------+
```

## 2. Key Architecture Concepts

1. **Pre-Edit Composition Buffer**: Visual underline/highlight area showing active raw key strokes prior to confirmation (e.g. `nihao` -> `你好`).
2. **Candidate Selection Window**: Floating overlay UI that displays matching candidates, handles pagination, and accepts numeric key choices (`1-9`).
3. **Surrounding Text Context**: Input method engines query surrounding text context (e.g. 10 characters before and after cursor) to improve predictive candidate sorting.
4. **Wayland/Zenith Protocol Binding**: Uses `zenith_text_input_v1` and `zenith_input_method_v1` protocols to coordinate focus states, cursor geometry, and popup placement.

## 3. Supported Engines & Algorithms

- **Pinyin (Simplified & Traditional Chinese)**: N-gram language model for sentence-level candidate prediction.
- **Mozc (Japanese)**: Kana-Kanji conversion dictionary engine.
- **Hangul (Korean)**: Instant character block composition (Jamo combination logic).
- **Table Engines**: Generic table loader supporting Rikai, Wubi, Cangjie, and custom `.cin` table formats.

## 4. Configuration & Hotkeys

- **Default Trigger**: `Super + Space` or `Ctrl + Space` toggles input method engines.
- **Config Storage**: User preferences defined in `~/.config/zenith/im.toml`.

```toml
[input_method]
default_engine = "pinyin"
candidate_page_size = 7
font_family = "Noto Sans CJK SC"
font_size = 14

[[engines]]
name = "pinyin"
layout = "us"

[[engines]]
name = "mozc"
layout = "jp"
```
