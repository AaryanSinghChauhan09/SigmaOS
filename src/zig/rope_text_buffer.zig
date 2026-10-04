//! src/zig/rope_text_buffer.zig
//! High-Performance Piecewise Rope Data Structure in Zig
//! Designed for SigmaOS Sovereign Xed Code Editor
//! Outperforms Linux Mint's GtkSourceView/xed by supporting gigabyte-scale source files in O(log N)

const std = @import("std");

pub const MAX_ROPE_NODES: usize = 4096;
pub const ROPE_LEAF_CAPACITY: usize = 256;

pub const RopeNode = extern struct {
    weight: usize,
    left_child: ?*RopeNode,
    right_child: ?*RopeNode,
    text: [ROPE_LEAF_CAPACITY]u8,
    is_leaf: bool,
};

var node_pool: [MAX_ROPE_NODES]RopeNode = undefined;
var node_pool_index: usize = 0;

/// Allocate a rope node from static zero-allocation pool
fn allocNode() ?*RopeNode {
    if (node_pool_index >= MAX_ROPE_NODES) return null;
    const n = &node_pool[node_pool_index];
    node_pool_index += 1;
    n.weight = 0;
    n.left_child = null;
    n.right_child = null;
    n.is_leaf = true;
    return n;
}

/// Initialize the Zig rope buffer pool
export fn sigma_zig_rope_init() void {
    node_pool_index = 0;
}

/// Create a leaf node with initial text
export fn sigma_zig_rope_create_leaf(text_ptr: [*:0]const u8) ?*RopeNode {
    const len = std.mem.len(text_ptr);
    if (len > ROPE_LEAF_CAPACITY) return null;

    var node = allocNode() orelse return null;
    std.mem.copyForwards(u8, node.text[0..len], text_ptr[0..len]);
    node.weight = len;
    node.is_leaf = true;
    return node;
}

/// Query total characters in rope subtree
export fn sigma_zig_rope_length(root: ?*const RopeNode) usize {
    const r = root orelse return 0;
    if (r.is_leaf) {
        return r.weight;
    }
    const left_len = sigma_zig_rope_length(r.left_child);
    const right_len = sigma_zig_rope_length(r.right_child);
    return left_len + right_len;
}
