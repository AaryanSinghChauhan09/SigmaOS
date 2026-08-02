// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::vec::Vec;

/// FFmpeg Parity Core
/// Media demuxing and transcoding architecture.

pub enum StreamType {
    Video,
    Audio,
    Subtitle,
}

pub struct MediaPacket {
    pub stream_index: usize,
    pub timestamp_pts: u64,
    pub payload: Vec<u8>,
}

pub struct FormatContext {
    pub streams: Vec<StreamType>,
    pub packets: Vec<MediaPacket>,
}

impl FormatContext {
    pub fn new() -> Self {
        Self {
            streams: Vec::new(),
            packets: Vec::new(),
        }
    }

    pub fn add_stream(&mut self, st: StreamType) -> usize {
        self.streams.push(st);
        self.streams.len() - 1
    }

    pub fn demux_packet(&mut self, packet: MediaPacket) {
        self.packets.push(packet);
    }
}

pub struct Transcoder {
    pub hardware_accel_enabled: bool,
}

impl Transcoder {
    pub fn transcode_packet(&self, packet: &MediaPacket) -> MediaPacket {
        // Simulated Transcode: If HW accel is enabled, we pretend to process faster
        // by returning a modified payload representing the transcoded frame.
        let mut new_payload = packet.payload.clone();
        if self.hardware_accel_enabled {
            for byte in new_payload.iter_mut() {
                *byte ^= 0xFF; // Mock transcoded bytes
            }
        }
        MediaPacket {
            stream_index: packet.stream_index,
            timestamp_pts: packet.timestamp_pts,
            payload: new_payload,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffmpeg_demuxing() {
        let mut ctx = FormatContext::new();
        let vid_idx = ctx.add_stream(StreamType::Video);

        ctx.demux_packet(MediaPacket {
            stream_index: vid_idx,
            timestamp_pts: 100,
            payload: alloc::vec![1, 2, 3],
        });

        assert_eq!(ctx.packets.len(), 1);

        let tx = Transcoder {
            hardware_accel_enabled: true,
        };
        let out = tx.transcode_packet(&ctx.packets[0]);
        assert_ne!(out.payload, alloc::vec![1, 2, 3]); // Verify simulated transcode applied
    }
}
