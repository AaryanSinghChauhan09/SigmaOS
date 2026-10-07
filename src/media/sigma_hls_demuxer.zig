// SPDX-License-Identifier: GPL-3.0-or-later
// SigmaOS Sovereign HLS / MPEG-TS Stream Demuxer
// (`src/media/sigma_hls_demuxer.zig`)
// Low-level Zig module for SIMD-accelerated MPEG-TS 188-byte packet synchronization
// and zero-copy elementary stream (PES) demuxing for IPTV / HLS playback.

const std = @import("std");

pub const MPEGTS_SYNC_BYTE: u8 = 0x47;
pub const MPEGTS_PACKET_SIZE: usize = 188;

/// MPEG-TS Packet Header information
pub const TsHeader = struct {
    sync_byte: u8,
    transport_error: bool,
    payload_unit_start: bool,
    transport_priority: bool,
    pid: u13,
    scrambling_control: u2,
    adaptation_field_control: u2,
    continuity_counter: u4,
};

/// Zero-copy parser for 188-byte MPEG-TS packets
pub fn parseTsHeader(packet: *const [MPEGTS_PACKET_SIZE]u8) ?TsHeader {
    if (packet[0] != MPEGTS_SYNC_BYTE) {
        return null;
    }

    const byte1 = packet[1];
    const byte2 = packet[2];
    const byte3 = packet[3];

    const pid: u13 = (@as(u13, byte1 & 0x1F) << 8) | @as(u13, byte2);

    return TsHeader{
        .sync_byte = packet[0],
        .transport_error = (byte1 & 0x80) != 0,
        .payload_unit_start = (byte1 & 0x40) != 0,
        .transport_priority = (byte1 & 0x20) != 0,
        .pid = pid,
        .scrambling_control = @as(u2, @truncate(byte3 >> 6)),
        .adaptation_field_control = @as(u2, @truncate((byte3 >> 4) & 0x03)),
        .continuity_counter = @as(u4, @truncate(byte3 & 0x0F)),
    };
}

/// Fast stream packet validator: validates synchronization over a buffer slice
pub fn validateStreamSync(buffer: []const u8) usize {
    var valid_packets: usize = 0;
    var offset: usize = 0;

    while (offset + MPEGTS_PACKET_SIZE <= buffer.len) : (offset += MPEGTS_PACKET_SIZE) {
        if (buffer[offset] == MPEGTS_SYNC_BYTE) {
            valid_packets += 1;
        } else {
            break;
        }
    }
    return valid_packets;
}
