# FontCraft Typography & SDF Rasterization Component Agent

## Role
The **FontCraft Agent** (`SigmaFontAgent`) coordinates font file parsing, OpenType/TrueType glyph metrics extraction, text advance measurements with kerning, and signed distance field vector rasterization for the Zenith desktop compositor.

## Inspiration
- HarfBuzz text shaping, FreeType font rendering, and GPU Multi-channel Signed Distance Field (MSDF) text rendering.

## Current SigmaOS Status
FontCraft engine implemented in `src/creative/fontcraft.rs` with glyph outline data structures, advance width measurement, kerning pairs, and point-segment distance calculation for vector text rasterization.

## Key Files
- `src/creative/fontcraft.rs`
- `src/creative/mod.rs`
- `src/display/wayland.rs`

## Public API & Invariant Contract
```rust
pub struct FontMetrics {
    pub units_per_em: u16,
    pub ascender: i16,
    pub descender: i16,
    pub line_gap: i16,
}

pub struct SovereignFontEngine {
    pub fn register_glyph(&mut self, codepoint: u32, glyph: GlyphData);
    pub fn set_kerning(&mut self, first: u32, second: u32, adjustment: i16);
    pub fn measure_text_advance(&self, text: &str) -> u32;
    pub fn point_segment_distance(px: f32, py: f32, ax: f32, ay: f32, bx: f32, by: f32) -> f32;
}
```

## Testing Strategy
```bash
rustc --test src/creative/fontcraft.rs --edition 2021 -o /tmp/test_fontcraft && /tmp/test_fontcraft --test-threads=1
```
- Verify glyph registration and correct string advance calculation with negative and positive kerning.
- Test mathematical precision of point-segment distance function.

## Future Enhancements
- GPU compute shader MSDF texture atlas generation.
- Bi-directional text layout (Arabic/Hebrew) and Indic complex script shaping.
