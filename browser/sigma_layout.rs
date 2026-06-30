//! sigma_layout.rs — SigmaOS Sovereign CSS Box Layout Engine
//! Implements the CSS Visual Formatting Model (block/inline layout) without
//! any external crates or std. Produces a render tree from the DOM slab.
//!
//! Sovereign: #![no_std], no third-party dependencies.

#![no_std]
#![allow(dead_code)]

use crate::sigma_browser::{Dom, DomNode, NodeType};

// ─── CSS Display ─────────────────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Display { Block, Inline, InlineBlock, None, Flex }

// ─── Box Model dimensions ─────────────────────────────────────────────────────
#[derive(Copy, Clone, Default)]
pub struct Edges {
    pub top:    i32,
    pub right:  i32,
    pub bottom: i32,
    pub left:   i32,
}

#[derive(Copy, Clone, Default)]
pub struct BoxDims {
    pub x:       i32,
    pub y:       i32,
    pub width:   u32,
    pub height:  u32,
    pub margin:  Edges,
    pub padding: Edges,
    pub border:  Edges,
}

// ─── Computed Style (simplified) ──────────────────────────────────────────────
#[derive(Copy, Clone)]
pub struct ComputedStyle {
    pub display:     Display,
    pub color:       u32,   // ARGB
    pub bg_color:    u32,   // ARGB (0 = transparent)
    pub font_size:   u16,   // 1/64 pt
    pub font_weight: u8,    // 100–900 / 10
    pub text_align:  u8,    // 0=left 1=center 2=right 3=justify
    pub margin:      Edges,
    pub padding:     Edges,
    pub border_w:    u16,
    pub border_color: u32,
}

impl Default for ComputedStyle {
    fn default() -> Self {
        Self {
            display:      Display::Block,
            color:        0xFF_EE_EE_EE,
            bg_color:     0,
            font_size:    768,   // 12pt
            font_weight:  4,     // 400
            text_align:   0,
            margin:       Edges::default(),
            padding:      Edges::default(),
            border_w:     0,
            border_color: 0,
        }
    }
}

/// Map HTML tag name to default computed style.
pub fn default_style_for_tag(tag: &[u8]) -> ComputedStyle {
    let mut s = ComputedStyle::default();
    match tag {
        b"h1" => { s.font_size = 2048; s.font_weight = 7; s.margin.bottom = 16; }
        b"h2" => { s.font_size = 1536; s.font_weight = 7; s.margin.bottom = 12; }
        b"h3" => { s.font_size = 1280; s.font_weight = 7; s.margin.bottom = 10; }
        b"p"  => { s.margin.bottom = 14; }
        b"a"  => { s.display = Display::Inline; s.color = 0xFF_4A_90_E2; }
        b"span" | b"em" | b"strong" | b"code" | b"b" | b"i" => {
            s.display = Display::Inline;
            if tag == b"b" || tag == b"strong" { s.font_weight = 7; }
            if tag == b"code" { s.bg_color = 0xFF_2A_2A_2A; }
        }
        b"div" | b"section" | b"article" | b"main" | b"header" | b"footer" | b"nav" => {
            s.display = Display::Block;
        }
        b"ul" | b"ol" => { s.margin.left = 24; }
        b"li" => { s.display = Display::Block; s.margin.bottom = 4; }
        b"body" => { s.margin = Edges { top: 16, right: 16, bottom: 16, left: 16 }; }
        b"table" => { s.display = Display::Block; s.border_w = 1; s.border_color = 0xFF_55_55_55; }
        b"button" | b"input" => {
            s.display = Display::InlineBlock;
            s.bg_color = 0xFF_33_33_66;
            s.border_w = 1;
            s.padding  = Edges { top: 6, right: 12, bottom: 6, left: 12 };
        }
        b"script" | b"style" | b"head" | b"meta" | b"link" => {
            s.display = Display::None;
        }
        _ => {}
    }
    s
}

// ─── Render Box ───────────────────────────────────────────────────────────────
pub const MAX_RENDER_BOXES: usize = 4096;

#[derive(Copy, Clone)]
pub struct RenderBox {
    pub dom_node:   u16,    // index into Dom.nodes
    pub style:      ComputedStyle,
    pub dims:       BoxDims,
    pub first_child: u16,   // index into render_boxes (0xFFFF = none)
    pub next_sib:    u16,
}

impl RenderBox {
    pub const fn default() -> Self {
        Self {
            dom_node:    0xFFFF,
            style:       ComputedStyle {
                display: Display::Block, color: 0xFFEEEEEE, bg_color: 0,
                font_size: 768, font_weight: 4, text_align: 0,
                margin: Edges { top: 0, right: 0, bottom: 0, left: 0 },
                padding: Edges { top: 0, right: 0, bottom: 0, left: 0 },
                border_w: 0, border_color: 0,
            },
            dims:        BoxDims { x: 0, y: 0, width: 0, height: 0,
                                   margin: Edges { top: 0, right: 0, bottom: 0, left: 0 },
                                   padding: Edges { top: 0, right: 0, bottom: 0, left: 0 },
                                   border: Edges { top: 0, right: 0, bottom: 0, left: 0 } },
            first_child: 0xFFFF,
            next_sib:    0xFFFF,
        }
    }
}

// ─── Layout Engine ────────────────────────────────────────────────────────────
pub struct LayoutEngine {
    pub boxes:      [RenderBox; MAX_RENDER_BOXES],
    pub box_count:  u16,
    viewport_w:     u32,
    viewport_h:     u32,
}

impl LayoutEngine {
    pub fn new(vp_w: u32, vp_h: u32) -> Self {
        Self {
            boxes:      [RenderBox::default(); MAX_RENDER_BOXES],
            box_count:  0,
            viewport_w: vp_w,
            viewport_h: vp_h,
        }
    }

    fn alloc_box(&mut self) -> Option<u16> {
        if self.box_count as usize >= MAX_RENDER_BOXES { return None; }
        let idx = self.box_count;
        self.boxes[idx as usize] = RenderBox::default();
        self.box_count += 1;
        Some(idx)
    }

    /// Build render tree from DOM. Returns root render box index.
    pub fn build_render_tree(&mut self, dom: &Dom) -> u16 {
        self.box_count = 0;
        self.build_node(dom, dom.nodes[0].first_child, 0xFFFF)
    }

    fn build_node(&mut self, dom: &Dom, node_idx: u16, _parent_box: u16) -> u16 {
        if node_idx == 0xFFFF { return 0xFFFF; }
        let node = &dom.nodes[node_idx as usize];
        let tag = dom.slab.get(node.tag_off, node.tag_len);
        let style = default_style_for_tag(tag);

        if style.display == Display::None {
            return self.build_node(dom, node.next_sibling, _parent_box);
        }

        let box_idx = match self.alloc_box() {
            Some(i) => i,
            None    => return 0xFFFF,
        };
        self.boxes[box_idx as usize].dom_node = node_idx;
        self.boxes[box_idx as usize].style    = style;

        // Recurse into children
        let first_child_box = self.build_node(dom, node.first_child, box_idx);
        self.boxes[box_idx as usize].first_child = first_child_box;

        // Siblings
        let sib_box = self.build_node(dom, node.next_sibling, _parent_box);
        self.boxes[box_idx as usize].next_sib = sib_box;

        box_idx
    }

    /// Perform block layout on the render tree.
    pub fn layout(&mut self, root: u16) {
        self.layout_block(root, 0, 0, self.viewport_w, self.viewport_h);
    }

    fn layout_block(&mut self, box_idx: u16, x: i32, y: i32, avail_w: u32, _avail_h: u32) {
        if box_idx == 0xFFFF { return; }

        let style  = self.boxes[box_idx as usize].style;
        let ml     = style.margin.left;
        let mr     = style.margin.right;
        let pl     = style.padding.left;
        let pr     = style.padding.right;
        let bw     = style.border_w as i32;

        let content_x = x + ml + pl + bw;
        let content_w = (avail_w as i32 - ml - mr - pl - pr - bw * 2).max(0) as u32;

        self.boxes[box_idx as usize].dims.x     = content_x;
        self.boxes[box_idx as usize].dims.y     = y + style.margin.top;
        self.boxes[box_idx as usize].dims.width = content_w;

        let mut child_y: i32 = y + style.margin.top + style.padding.top + bw;
        let mut total_h: u32 = 0;
        let mut child = self.boxes[box_idx as usize].first_child;

        while child != 0xFFFF {
            let child_style = self.boxes[child as usize].style;
            match child_style.display {
                Display::Block | Display::Flex => {
                    self.layout_block(child, content_x, child_y, content_w, 0);
                    let ch = self.boxes[child as usize].dims.height
                           + child_style.margin.top as u32
                           + child_style.margin.bottom as u32;
                    child_y  += ch as i32;
                    total_h  += ch;
                }
                Display::Inline | Display::InlineBlock => {
                    // Simplified: treat inline as block for layout
                    self.layout_block(child, content_x, child_y, content_w, 0);
                    let ch = self.boxes[child as usize].dims.height.max(
                        (child_style.font_size / 64) as u32 + 4
                    );
                    child_y += ch as i32;
                    total_h += ch;
                }
                Display::None => {}
            }
            child = self.boxes[child as usize].next_sib;
        }

        // Estimate text height: font_size / 64 * approx_lines
        let line_h = (style.font_size / 64) as u32 + 4;
        self.boxes[box_idx as usize].dims.height =
            (total_h + style.padding.top as u32 + style.padding.bottom as u32
             + bw as u32 * 2).max(line_h);
    }
}
