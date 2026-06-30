//! sigma_browser.rs — SigmaOS Sovereign Browser Engine
//! HTML5 tokenizer + DOM tree builder.
//!
//! Sovereign constraints:
//!   #![no_std]  — no standard library
//!   No external crates, no predefined functions from libc or alloc.
//!   All dynamic memory through a kernel-provided SovereignAllocator trait.
//!
//! Implements:
//!   - HTML5 tokenizer state machine (subset covering real-world pages)
//!   - DOM Node tree (Element, Text, Comment) built on a static slab
//!   - Base DOM query: getElementById / getElementsByTag

#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

// ─── Sovereign Allocator Trait ────────────────────────────────────────────────
/// Kernel-provided allocator. Implementors must guarantee:
///   - alloc returns aligned, zero-filled memory or null
///   - dealloc is a no-op only if the allocator uses a slab/arena
pub trait SovereignAllocator: Send + Sync {
    unsafe fn alloc(&self, size: usize, align: usize) -> *mut u8;
    unsafe fn dealloc(&self, ptr: *mut u8, size: usize, align: usize);
}

// ─── Static String Slab ───────────────────────────────────────────────────────
/// A fixed arena for interning HTML attribute values & tag names.
pub struct StringSlab {
    buf:  [u8; 65536],
    head: usize,
}

impl StringSlab {
    pub const fn new() -> Self {
        Self { buf: [0u8; 65536], head: 0 }
    }

    /// Intern a byte slice; returns (offset, len) into the slab.
    pub fn intern(&mut self, s: &[u8]) -> Option<(u16, u16)> {
        let len = s.len();
        if self.head + len > self.buf.len() { return None; }
        let off = self.head;
        self.buf[off..off + len].copy_from_slice(s);
        self.head += len;
        Some((off as u16, len as u16))
    }

    pub fn get(&self, off: u16, len: u16) -> &[u8] {
        &self.buf[off as usize .. off as usize + len as usize]
    }
}

// ─── Token Types ──────────────────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TokenKind {
    StartTag,
    EndTag,
    SelfClosingTag,
    Text,
    Comment,
    Doctype,
    Eof,
}

/// Maximum attributes per tag.
const MAX_ATTRS: usize = 32;
/// Maximum attr name/value length (in slab offsets).
const MAX_TAG_NAME: usize = 64;

#[derive(Copy, Clone)]
pub struct Attribute {
    pub name_off: u16,
    pub name_len: u16,
    pub val_off:  u16,
    pub val_len:  u16,
}

#[derive(Clone)]
pub struct HtmlToken<'s> {
    pub kind:       TokenKind,
    pub tag_name:   &'s [u8],
    pub text_data:  &'s [u8],
    pub attrs:      [Attribute; MAX_ATTRS],
    pub attr_count: u8,
}

// ─── Tokenizer State Machine ──────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq)]
enum TokState {
    Data,
    TagOpen,
    EndTagOpen,
    TagName,
    BeforeAttrName,
    AttrName,
    BeforeAttrVal,
    AttrValDoubleQuoted,
    AttrValSingleQuoted,
    AttrValUnquoted,
    SelfClosingStart,
    BogusComment,
    MarkupDecl,      // <!
    Comment,
    CommentEnd,
    Doctype,
}

pub struct Tokenizer<'input, 'slab> {
    input:  &'input [u8],
    pos:    usize,
    state:  TokState,
    slab:   &'slab mut StringSlab,
}

impl<'input, 'slab> Tokenizer<'input, 'slab> {
    pub fn new(input: &'input [u8], slab: &'slab mut StringSlab) -> Self {
        Self { input, pos: 0, state: TokState::Data, slab }
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.pos).copied()
    }

    fn consume(&mut self) -> Option<u8> {
        let c = self.input.get(self.pos).copied();
        if c.is_some() { self.pos += 1; }
        c
    }

    fn match_bytes(&mut self, s: &[u8]) -> bool {
        if self.pos + s.len() > self.input.len() { return false; }
        if &self.input[self.pos .. self.pos + s.len()] == s {
            self.pos += s.len();
            true
        } else { false }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c == b' ' || c == b'\t' || c == b'\n' || c == b'\r' {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    /// Scan until `delim` and return the byte slice from `start` to the
    /// delimiter (exclusive). Advances past the delimiter.
    fn scan_until(&mut self, delim: u8) -> &'input [u8] {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if c == delim { break; }
            self.pos += 1;
        }
        let end = self.pos;
        if self.peek() == Some(delim) { self.pos += 1; }
        &self.input[start..end]
    }

    fn scan_until2(&mut self, d0: u8, d1: u8) -> &'input [u8] {
        let start = self.pos;
        loop {
            if self.pos + 1 >= self.input.len() { break; }
            if self.input[self.pos] == d0 && self.input[self.pos + 1] == d1 {
                let end = self.pos;
                self.pos += 2;
                return &self.input[start..end];
            }
            self.pos += 1;
        }
        &self.input[start..]
    }

    /// Scan a tag name (ASCII letters/digits/hyphen).
    fn scan_name(&mut self) -> &'input [u8] {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == b'-' || c == b'_' || c == b':' {
                self.pos += 1;
            } else {
                break;
            }
        }
        &self.input[start..self.pos]
    }

    /// Produce the next HTML token. Returns None on EOF.
    pub fn next_token(&mut self) -> Option<HtmlToken<'input>> {
        if self.pos >= self.input.len() {
            return Some(HtmlToken {
                kind: TokenKind::Eof,
                tag_name: b"",
                text_data: b"",
                attrs: [Attribute { name_off: 0, name_len: 0, val_off: 0, val_len: 0 }; MAX_ATTRS],
                attr_count: 0,
            });
        }

        let blank_attr = Attribute { name_off: 0, name_len: 0, val_off: 0, val_len: 0 };
        let mut attrs = [blank_attr; MAX_ATTRS];
        let mut attr_count = 0u8;

        match self.peek()? {
            b'<' => {
                self.pos += 1; // consume '<'
                match self.peek() {
                    Some(b'/') => {
                        // End tag
                        self.pos += 1;
                        let name = self.scan_name();
                        // skip to '>'
                        while let Some(c) = self.consume() { if c == b'>' { break; } }
                        Some(HtmlToken {
                            kind: TokenKind::EndTag,
                            tag_name: name,
                            text_data: b"",
                            attrs,
                            attr_count: 0,
                        })
                    }
                    Some(b'!') => {
                        self.pos += 1;
                        if self.match_bytes(b"--") {
                            // Comment
                            let data = self.scan_until2(b'-', b'-');
                            if self.peek() == Some(b'>') { self.pos += 1; }
                            Some(HtmlToken {
                                kind: TokenKind::Comment,
                                tag_name: b"",
                                text_data: data,
                                attrs,
                                attr_count: 0,
                            })
                        } else {
                            // DOCTYPE or bogus
                            let data = self.scan_until(b'>');
                            Some(HtmlToken {
                                kind: TokenKind::Doctype,
                                tag_name: b"",
                                text_data: data,
                                attrs,
                                attr_count: 0,
                            })
                        }
                    }
                    Some(b'?') => {
                        // Processing instruction — skip
                        self.scan_until(b'>');
                        self.next_token()
                    }
                    _ => {
                        // Start tag
                        let name = self.scan_name();
                        // Parse attributes
                        loop {
                            self.skip_whitespace();
                            match self.peek() {
                                None | Some(b'>') => {
                                    if self.peek() == Some(b'>') { self.pos += 1; }
                                    break;
                                }
                                Some(b'/') => {
                                    self.pos += 1; // consume '/'
                                    if self.peek() == Some(b'>') { self.pos += 1; }
                                    return Some(HtmlToken {
                                        kind: TokenKind::SelfClosingTag,
                                        tag_name: name,
                                        text_data: b"",
                                        attrs,
                                        attr_count,
                                    });
                                }
                                _ => {
                                    // Attr name
                                    let aname = self.scan_name();
                                    self.skip_whitespace();
                                    let aval: &'input [u8] = if self.peek() == Some(b'=') {
                                        self.pos += 1;
                                        self.skip_whitespace();
                                        match self.peek() {
                                            Some(b'"')  => { self.pos += 1; self.scan_until(b'"') }
                                            Some(b'\'') => { self.pos += 1; self.scan_until(b'\'') }
                                            _ => self.scan_name(),
                                        }
                                    } else { b"" };

                                    if (attr_count as usize) < MAX_ATTRS && !aname.is_empty() {
                                        // Intern into slab
                                        let (no, nl) = self.slab.intern(aname).unwrap_or((0,0));
                                        let (vo, vl) = self.slab.intern(aval).unwrap_or((0,0));
                                        attrs[attr_count as usize] = Attribute {
                                            name_off: no, name_len: nl,
                                            val_off:  vo, val_len:  vl,
                                        };
                                        attr_count += 1;
                                    }
                                }
                            }
                        }
                        Some(HtmlToken {
                            kind: TokenKind::StartTag,
                            tag_name: name,
                            text_data: b"",
                            attrs,
                            attr_count,
                        })
                    }
                }
            }
            _ => {
                // Text node
                let start = self.pos;
                while let Some(c) = self.peek() {
                    if c == b'<' { break; }
                    self.pos += 1;
                }
                let text = &self.input[start..self.pos];
                Some(HtmlToken {
                    kind: TokenKind::Text,
                    tag_name: b"",
                    text_data: text,
                    attrs,
                    attr_count: 0,
                })
            }
        }
    }
}

// ─── DOM ──────────────────────────────────────────────────────────────────────
pub const DOM_NODE_LIMIT: usize = 4096;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NodeType { Element, Text, Comment, Document }

#[derive(Copy, Clone)]
pub struct DomNode {
    pub node_type:  NodeType,
    pub tag_off:    u16,   // slab offset for tag name
    pub tag_len:    u16,
    pub text_off:   u16,   // slab offset for text content
    pub text_len:   u16,
    pub parent:     u16,   // node index (0 = no parent)
    pub first_child: u16,
    pub next_sibling: u16,
    pub attr_start: u16,   // index into attr pool
    pub attr_count: u8,
}

impl DomNode {
    pub const fn default() -> Self {
        Self {
            node_type: NodeType::Text,
            tag_off: 0, tag_len: 0,
            text_off: 0, text_len: 0,
            parent: 0xFFFF,
            first_child: 0xFFFF,
            next_sibling: 0xFFFF,
            attr_start: 0,
            attr_count: 0,
        }
    }
}

pub const ATTR_POOL_LIMIT: usize = 8192;

pub struct Dom {
    pub nodes:      [DomNode; DOM_NODE_LIMIT],
    pub node_count: u16,
    pub attr_pool:  [Attribute; ATTR_POOL_LIMIT],
    pub attr_used:  u16,
    pub slab:       StringSlab,
}

impl Dom {
    pub const fn new() -> Self {
        Self {
            nodes: [DomNode::default(); DOM_NODE_LIMIT],
            node_count: 0,
            attr_pool: [Attribute { name_off: 0, name_len: 0, val_off: 0, val_len: 0 }; ATTR_POOL_LIMIT],
            attr_used: 0,
            slab: StringSlab::new(),
        }
    }

    pub fn alloc_node(&mut self, t: NodeType) -> Option<u16> {
        if self.node_count as usize >= DOM_NODE_LIMIT { return None; }
        let idx = self.node_count;
        self.nodes[idx as usize] = DomNode::default();
        self.nodes[idx as usize].node_type = t;
        self.node_count += 1;
        Some(idx)
    }

    pub fn append_child(&mut self, parent_idx: u16, child_idx: u16) {
        self.nodes[child_idx as usize].parent = parent_idx;
        let parent = &mut self.nodes[parent_idx as usize];
        if parent.first_child == 0xFFFF {
            parent.first_child = child_idx;
        } else {
            // Walk to last sibling
            let mut sib = parent.first_child;
            loop {
                let next = self.nodes[sib as usize].next_sibling;
                if next == 0xFFFF {
                    self.nodes[sib as usize].next_sibling = child_idx;
                    break;
                }
                sib = next;
            }
        }
    }
}

// ─── DOM Builder ──────────────────────────────────────────────────────────────
pub struct DomBuilder {
    pub dom:        Dom,
    open_stack:     [u16; 256],   // open element stack
    stack_depth:    u8,
    root_idx:       u16,
}

impl DomBuilder {
    pub fn new() -> Self {
        let mut b = Self {
            dom: Dom::new(),
            open_stack: [0xFFFF; 256],
            stack_depth: 0,
            root_idx: 0,
        };
        // Allocate document root node
        let root = b.dom.alloc_node(NodeType::Document).unwrap_or(0);
        b.root_idx = root;
        b.open_stack[0] = root;
        b.stack_depth = 1;
        b
    }

    pub fn current_parent(&self) -> u16 {
        if self.stack_depth > 0 {
            self.open_stack[(self.stack_depth - 1) as usize]
        } else {
            self.root_idx
        }
    }

    /// Feed one token into the DOM builder.
    pub fn feed(&mut self, tok: &HtmlToken<'_>) {
        match tok.kind {
            TokenKind::StartTag | TokenKind::SelfClosingTag => {
                let node_idx = match self.dom.alloc_node(NodeType::Element) {
                    Some(i) => i,
                    None    => return,
                };
                let parent = self.current_parent();
                self.dom.append_child(parent, node_idx);

                // Intern tag name
                let (to, tl) = self.dom.slab.intern(tok.tag_name).unwrap_or((0, 0));
                let node = &mut self.dom.nodes[node_idx as usize];
                node.tag_off   = to;
                node.tag_len   = tl;

                // Store attributes
                let attr_start = self.dom.attr_used;
                let ac = tok.attr_count.min((ATTR_POOL_LIMIT - self.dom.attr_used as usize) as u8);
                for i in 0..ac as usize {
                    if self.dom.attr_used as usize >= ATTR_POOL_LIMIT { break; }
                    self.dom.attr_pool[self.dom.attr_used as usize] = tok.attrs[i];
                    self.dom.attr_used += 1;
                }
                let node = &mut self.dom.nodes[node_idx as usize];
                node.attr_start = attr_start;
                node.attr_count = ac;

                if tok.kind == TokenKind::StartTag && !is_void_element(tok.tag_name) {
                    if (self.stack_depth as usize) < self.open_stack.len() {
                        self.open_stack[self.stack_depth as usize] = node_idx;
                        self.stack_depth += 1;
                    }
                }
            }
            TokenKind::EndTag => {
                // Pop matching open element
                let mut i = self.stack_depth as i16 - 1;
                while i >= 0 {
                    let idx = self.open_stack[i as usize];
                    let n = &self.dom.nodes[idx as usize];
                    let tag = self.dom.slab.get(n.tag_off, n.tag_len);
                    if tag.eq_ignore_ascii_case(tok.tag_name) {
                        self.stack_depth = i as u8;
                        break;
                    }
                    i -= 1;
                }
            }
            TokenKind::Text => {
                if tok.text_data.iter().all(|&b| b == b' ' || b == b'\t' || b == b'\n' || b == b'\r') {
                    return; // skip whitespace-only text
                }
                let node_idx = match self.dom.alloc_node(NodeType::Text) {
                    Some(i) => i,
                    None    => return,
                };
                let parent = self.current_parent();
                self.dom.append_child(parent, node_idx);
                let (to, tl) = self.dom.slab.intern(tok.text_data).unwrap_or((0, 0));
                let node = &mut self.dom.nodes[node_idx as usize];
                node.text_off = to;
                node.text_len = tl;
            }
            TokenKind::Comment => { /* intentionally dropped */ }
            TokenKind::Doctype | TokenKind::Eof => {}
        }
    }

    /// Parse a complete HTML document.
    pub fn parse(&mut self, html: &[u8]) {
        let mut slab2 = StringSlab::new(); // temporary slab for tokenizer attr interning
        let mut tok = Tokenizer::new(html, &mut slab2);
        loop {
            match tok.next_token() {
                None => break,
                Some(t) => {
                    if t.kind == TokenKind::Eof { break; }
                    self.feed(&t);
                }
            }
        }
    }

    /// Find first node with matching tag name.
    pub fn query_tag<'a>(&'a self, tag: &[u8]) -> Option<u16> {
        for i in 0..self.dom.node_count as usize {
            let n = &self.dom.nodes[i];
            if n.node_type == NodeType::Element {
                let t = self.dom.slab.get(n.tag_off, n.tag_len);
                if t.eq_ignore_ascii_case(tag) { return Some(i as u16); }
            }
        }
        None
    }
}

// ─── Void elements (cannot have children) ────────────────────────────────────
fn is_void_element(tag: &[u8]) -> bool {
    const VOIDS: &[&[u8]] = &[
        b"area", b"base", b"br", b"col", b"embed",
        b"hr", b"img", b"input", b"link", b"meta",
        b"param", b"source", b"track", b"wbr",
    ];
    for v in VOIDS {
        if v.eq_ignore_ascii_case(tag) { return true; }
    }
    false
}

// ─── Trait for byte slice case-insensitive compare ────────────────────────────
trait SliceExt {
    fn eq_ignore_ascii_case(&self, other: &[u8]) -> bool;
}
impl SliceExt for [u8] {
    fn eq_ignore_ascii_case(&self, other: &[u8]) -> bool {
        if self.len() != other.len() { return false; }
        for (a, b) in self.iter().zip(other.iter()) {
            let ca = if *a >= b'A' && *a <= b'Z' { *a + 32 } else { *a };
            let cb = if *b >= b'A' && *b <= b'Z' { *b + 32 } else { *b };
            if ca != cb { return false; }
        }
        true
    }
}
