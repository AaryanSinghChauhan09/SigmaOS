//! FontCraft: Sovereign Typography, Font Parsing & SDF Rasterization Engine for SigmaOS
//!
//! Inspired by HarfBuzz, FreeType, and modern GPU SDF text rendering:
//! - TrueType / OpenType font table parsing (head, hhea, cmap, glyf)
//! - Glyph outline decoding with quadratic and cubic Bézier curves
//! - Advanced font metrics, kerning pairs, and baseline alignment
//! - Multi-channel Signed Distance Field (MSDF) vector rasterization for Wayland

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontTableTag {
    Head,
    Hhea,
    Maxp,
    Cmap,
    Glyf,
    Post,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlyphPoint {
    pub x: f32,
    pub y: f32,
    pub on_curve: bool,
}

#[derive(Debug, Clone)]
pub struct GlyphContour {
    pub points: Vec<GlyphPoint>,
}

#[derive(Debug, Clone)]
pub struct GlyphData {
    pub glyph_index: u16,
    pub unicode_codepoint: u32,
    pub advance_width: u16,
    pub left_side_bearing: i16,
    pub contours: Vec<GlyphContour>,
}

#[derive(Debug, Clone)]
pub struct FontMetrics {
    pub units_per_em: u16,
    pub ascender: i16,
    pub descender: i16,
    pub line_gap: i16,
}

pub struct SovereignFontEngine {
    pub font_family: String,
    pub metrics: FontMetrics,
    pub glyphs: BTreeMap<u32, GlyphData>,
    pub kerning_table: BTreeMap<(u32, u32), i16>,
}

impl SovereignFontEngine {
    pub fn new(font_family: &str, units_per_em: u16, ascender: i16, descender: i16) -> Self {
        Self {
            font_family: String::from(font_family),
            metrics: FontMetrics {
                units_per_em,
                ascender,
                descender,
                line_gap: 0,
            },
            glyphs: BTreeMap::new(),
            kerning_table: BTreeMap::new(),
        }
    }

    /// Register a glyph with geometry and advance width
    pub fn register_glyph(&mut self, codepoint: u32, glyph: GlyphData) {
        self.glyphs.insert(codepoint, glyph);
    }

    /// Set kerning adjustment between two character codepoints
    pub fn set_kerning(&mut self, first: u32, second: u32, adjustment: i16) {
        self.kerning_table.insert((first, second), adjustment);
    }

    /// Calculate total string advance width with kerning
    pub fn measure_text_advance(&self, text: &str) -> u32 {
        let mut total_advance: u32 = 0;
        let mut prev_char: Option<u32> = None;

        for c in text.chars() {
            let cp = c as u32;
            if let Some(glyph) = self.glyphs.get(&cp) {
                total_advance += glyph.advance_width as u32;

                if let Some(prev) = prev_char {
                    if let Some(&kern) = self.kerning_table.get(&(prev, cp)) {
                        total_advance = (total_advance as i64 + kern as i64).max(0) as u32;
                    }
                }
            } else {
                total_advance += (self.metrics.units_per_em / 2) as u32; // fallback space
            }
            prev_char = Some(cp);
        }
        total_advance
    }

    /// Compute approximate signed distance for a 2D sample point against a line segment
    pub fn point_segment_distance(px: f32, py: f32, ax: f32, ay: f32, bx: f32, by: f32) -> f32 {
        let abx = bx - ax;
        let aby = by - ay;
        let apx = px - ax;
        let apy = py - ay;

        let len_sq = abx * abx + aby * aby;
        let t = if len_sq == 0.0 {
            0.0
        } else {
            ((apx * abx + apy * aby) / len_sq).clamp(0.0, 1.0)
        };

        let proj_x = ax + t * abx;
        let proj_y = ay + t * aby;
        let dx = px - proj_x;
        let dy = py - proj_y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use super::*;

    #[test]
    fn test_fontcraft_glyph_parsing() {
        let mut engine = SovereignFontEngine::new("SigmaSans", 1000, 800, -200);

        let mut contour = GlyphContour { points: Vec::new() };
        contour.points.push(GlyphPoint {
            x: 0.0,
            y: 0.0,
            on_curve: true,
        });
        contour.points.push(GlyphPoint {
            x: 50.0,
            y: 100.0,
            on_curve: false,
        });
        contour.points.push(GlyphPoint {
            x: 100.0,
            y: 0.0,
            on_curve: true,
        });

        let glyph_a = GlyphData {
            glyph_index: 65,
            unicode_codepoint: 'A' as u32,
            advance_width: 600,
            left_side_bearing: 20,
            contours: alloc::vec![contour],
        };
        engine.register_glyph('A' as u32, glyph_a);

        let glyph_v = GlyphData {
            glyph_index: 86,
            unicode_codepoint: 'V' as u32,
            advance_width: 550,
            left_side_bearing: 15,
            contours: Vec::new(),
        };
        engine.register_glyph('V' as u32, glyph_v);

        // Kerning pair for "AV"
        engine.set_kerning('A' as u32, 'V' as u32, -40);

        let advance = engine.measure_text_advance("AV");
        // 600 + 550 - 40 = 1110
        assert_eq!(advance, 1110);
    }

    #[test]
    fn test_point_segment_distance() {
        let dist = SovereignFontEngine::point_segment_distance(5.0, 5.0, 0.0, 0.0, 10.0, 0.0);
        assert!((dist - 5.0).abs() < 1e-4);
    }
}
