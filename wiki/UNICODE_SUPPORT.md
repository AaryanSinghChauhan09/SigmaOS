# SigmaOS Unicode Support & Text Processing Architecture Specification

## 1. Executive Summary

SigmaOS mandates UTF-8 string encoding across all operating system boundaries, including the kernel, virtual file systems, IPC channels, shell environments, and userland applications. This document outlines the technical architecture for Unicode standard compliance, text segmentation, normalization, bidirectional text rendering, and font rasterization.

## 2. UTF-8 Standard Subsystem Architecture

### 2.1 UTF-8 Universal Encoding
- **Kernel Standard**: All kernel paths, process titles, console output streams, and system logs are strict, valid UTF-8 sequences. Invalid byte sequences are replaced with the standard replacement character (`U+FFFD` - ``).
- **File System VFS Layer**: Filenames are stored as raw UTF-8 byte sequences. Filename comparison supports case-insensitive matching using Unicode default casemapping algorithms.

### 2.2 Text Processing Primitives (`src/klib/unicode.rs` & `src/klib/string_ops.rs`)
SigmaOS includes zero-dependency Rust Unicode utilities supporting:
- **Codepoint Parsing & Conversion**: Zero-allocation UTF-8 iterator converting byte slices to 32-bit scalar Unicode scalar values (`char`).
- **Unicode Normalization Forms**:
  - **NFC (Canonical Decomposition, followed by Canonical Composition)**: Standard format for file system paths and database keys.
  - **NFD (Canonical Decomposition)**: Decomposes characters into base characters + combining diacritics.
- **Grapheme Cluster Segmentation**: Implements UAX #29 (Unicode Text Segmentation) to correctly measure visible character count, preventing string truncation through surrogate pairs or combining emoji sequences (e.g. family emoji sequences like `👨‍👩‍👧‍👦`).

## 3. Bidirectional (Bidi) Text Engine & Complex Text Layout

Supports right-to-left (RTL) writing systems such as Arabic, Hebrew, and Persian according to UAX #9 (Unicode Bidirectional Algorithm):
1. **Paragraph Direction Detection**: Determines base directionality based on the first strong directional character.
2. **Explicit & Implicit Embedding**: Reorders visual text runs while preserving logical character indexing.
3. **Complex Script Shaping (HarfBuzz Equivalent)**: Handles glyph substitution (GSUB) and glyph positioning (GPOS) for ligatures, contextual character forms (Initial, Medial, Final, Isolated Arabic shapes), and stack diacritics.

## 4. Font Rendering & Glyph Management

```
+-------------------------------------------------------------+
|                      Font & Text Stack                      |
|  +--------------------+  +--------------------------------+ |
|  | Fontconfig Config  |  | FreeType / Rust Glyph Rasterizer| |
|  +---------+----------+  +---------------+----------------+ |
+------------|-----------------------------|------------------+
             |                             |
+------------v-----------------------------v------------------+
|                 Sovereign Text Renderer                     |
|  +--------------------+  +--------------------------------+ |
|  | Atlas Glyph Cache  |  | Subpixel Antialiasing (LCD/Sub)| |
|  +--------------------+  +--------------------------------+ |
+-------------------------------------------------------------+
```

- **Font Matching Engine**: Parses system font configuration files (`/etc/fonts/fonts.conf`) to resolve generic family names (`sans-serif`, `serif`, `monospace`) to preferred installed TrueType (`.ttf`) or OpenType (`.otf`) fonts.
- **Glyph Atlas Cache**: Caches rendered glyph textures in GPU VRAM or shared memory to avoid re-rasterizing identical text characters during UI rendering passes.
