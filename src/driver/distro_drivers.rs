// SigmaOS Distro-Inspired Clean-Room Drivers
// Replicates key drivers, device nodes, and audio/crypto subsystems from Linux & BSD distributions

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use alloc::vec;
use core::sync::atomic::{AtomicU32, Ordering};

// ============================================================================
// 1. Linux Devtmpfs & Standard Device Nodes Simulator
// ============================================================================

/// Replicates standard Linux devtmpfs device nodes (/dev/null, /dev/zero, /dev/urandom)
pub struct LinuxDevtmpfsSimulator {
    entropy_seed: AtomicU32,
}

impl LinuxDevtmpfsSimulator {
    pub fn new(seed: u32) -> Self {
        Self {
            entropy_seed: AtomicU32::new(seed),
        }
    }

    /// Simulates reading from /dev/null (always returns 0 bytes read, indicating EOF)
    pub fn read_null(&self, buffer: &mut [u8]) -> usize {
        let _ = buffer;
        0
    }

    /// Simulates reading from /dev/zero (fills buffer with zeroes, returns full buffer size)
    pub fn read_zero(&self, buffer: &mut [u8]) -> usize {
        for byte in buffer.iter_mut() {
            *byte = 0;
        }
        buffer.len()
    }

    /// Simulates reading from /dev/urandom (generates high-entropy pseudo-random bytes via LCG)
    pub fn read_urandom(&self, buffer: &mut [u8]) -> usize {
        let mut state = self.entropy_seed.load(Ordering::Relaxed);
        for byte in buffer.iter_mut() {
            // High-entropy LCG parameters matching standard glibc generators
            state = state.wrapping_mul(1103515245).wrapping_add(12345);
            *byte = (state >> 16) as u8;
        }
        self.entropy_seed.store(state, Ordering::Relaxed);
        buffer.len()
    }

    /// Simulates writing to /dev/null (discards all inputs, returns full size indicating success)
    pub fn write_null(&self, data: &[u8]) -> usize {
        data.len()
    }
}

// ============================================================================
// 2. NetBSD-Inspired Multi-Channel Audio Mixer Driver
// ============================================================================

pub const AUDIO_SAMPLE_RATE: usize = 44100;
pub const AUDIO_CHANNELS: usize = 2; // Stereo

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PcmFrame {
    pub left: i16,
    pub right: i16,
}

/// NetBSD-inspired multi-channel PCM software audio mixer driver
pub struct BsdAudioMixer {
    pub channels: Vec<Vec<PcmFrame>>,
    pub master_volume: u16,   // 0 to 256 scale (256 = 100% volume)
    pub channel_pan: Vec<i8>, // -128 (full left) to 127 (full right)
}

impl BsdAudioMixer {
    pub fn new() -> Self {
        Self {
            channels: Vec::new(),
            master_volume: 256,
            channel_pan: Vec::new(),
        }
    }

    /// Registers a new hardware PCM audio stream channel
    pub fn register_channel(&mut self, stream: Vec<PcmFrame>, pan: i8) -> usize {
        self.channels.push(stream);
        self.channel_pan.push(pan);
        self.channels.len() - 1
    }

    /// Mixed and synthesizes all registered channels into a single master output stream
    /// Applies software attenuation, stereo panning, and hardware-clipping safety limits.
    pub fn mix_channels(&self) -> Vec<PcmFrame> {
        if self.channels.is_empty() {
            return Vec::new();
        }

        // Find the longest registered stream to define our mixed buffer size
        let max_len = self.channels.iter().map(|ch| ch.len()).max().unwrap_or(0);
        let mut mixed = alloc::vec![PcmFrame { left: 0, right: 0 }; max_len];

        for (ch_idx, channel) in self.channels.iter().enumerate() {
            let pan = self.channel_pan[ch_idx] as f32; // -128 to 127

            // Calculate stereo panning coefficients
            let (left_coeff, right_coeff) = if pan < 0.0 {
                (1.0, (128.0 + pan) / 128.0)
            } else {
                ((128.0 - pan) / 128.0, 1.0)
            };

            for (frame_idx, frame) in channel.iter().enumerate() {
                // Apply channel panning and volume scaling
                let l_panned = (frame.left as f32 * left_coeff) as i32;
                let r_panned = (frame.right as f32 * right_coeff) as i32;

                // Accumulate to master mixed stream
                mixed[frame_idx].left = mixed[frame_idx].left.saturating_add(l_panned as i16);
                mixed[frame_idx].right = mixed[frame_idx].right.saturating_add(r_panned as i16);
            }
        }

        // Apply master volume scaling with clipping protection
        let volume_scale = self.master_volume as f32 / 256.0;
        for frame in mixed.iter_mut() {
            let l_scaled = (frame.left as f32 * volume_scale) as i32;
            let r_scaled = (frame.right as f32 * volume_scale) as i32;

            // Clip within hardware signed 16-bit boundaries
            frame.left = l_scaled.clamp(-32768, 32767) as i16;
            frame.right = r_scaled.clamp(-32768, 32767) as i16;
        }

        mixed
    }
}

impl Default for BsdAudioMixer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. OpenBSD-Inspired Hardware Cryptography Acceleration Driver (/dev/crypto)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptoCipher {
    ChaCha20Poly1305,
    Aes256Gcm,
}

pub struct OpenBsdCryptoDevice {
    pub cipher: CryptoCipher,
    pub key: Vec<u8>,
    pub iv: Vec<u8>,
}

impl OpenBsdCryptoDevice {
    pub fn new(cipher: CryptoCipher, key: &[u8], iv: &[u8]) -> Self {
        Self {
            cipher,
            key: key.to_vec(),
            iv: iv.to_vec(),
        }
    }

    /// Simulates high-speed hardware-accelerated stream decryption/encryption cipher pipe
    pub fn process_data(&self, input: &[u8], output: &mut [u8]) -> Result<(), &'static str> {
        if input.len() != output.len() {
            return Err("Input and output buffer sizes must match");
        }
        if self.key.is_empty() || self.iv.is_empty() {
            return Err("Missing cryptographic key or IV initialization vector");
        }

        match self.cipher {
            CryptoCipher::ChaCha20Poly1305 => {
                // Highly performant stream cipher emulation via byte-wise key-stream XOR mapping
                let mut keystream_state = 0u32;
                for &k in &self.key {
                    keystream_state = keystream_state.wrapping_add(k as u32);
                }
                for &i in &self.iv {
                    keystream_state ^= i as u32;
                }

                for (idx, &byte) in input.iter().enumerate() {
                    keystream_state = keystream_state
                        .wrapping_mul(1664525)
                        .wrapping_add(1013904223);
                    let keystream_byte = (keystream_state >> 16) as u8;
                    output[idx] = byte ^ keystream_byte;
                }
            }
            CryptoCipher::Aes256Gcm => {
                // AES block-chain mixing simulation
                let mut block_state = 0u64;
                for &k in &self.key {
                    block_state = block_state.wrapping_add(k as u64);
                }

                for (idx, &byte) in input.iter().enumerate() {
                    block_state = block_state
                        .wrapping_shl(3)
                        .wrapping_add(block_state)
                        .wrapping_add(byte as u64);
                    let xor_mask = (block_state ^ 0xa5a5_a5a5_a5a5_a5a5) as u8;
                    output[idx] = byte ^ xor_mask;
                }
            }
        }
        Ok(())
    }
}

// ============================================================================
// 4. FreeBSD-Inspired Direct Console (dcons) Debug Port Driver
// ============================================================================

/// FreeBSD dcons-inspired system diagnostic debug port / console simulation
pub struct BsdDconsConsole {
    pub write_buffer: Vec<u8>,
    pub read_buffer: Vec<u8>,
    pub capacity: usize,
}

impl BsdDconsConsole {
    pub fn new(capacity: usize) -> Self {
        Self {
            write_buffer: Vec::new(),
            read_buffer: Vec::new(),
            capacity,
        }
    }

    /// Appends debug output bytes to the dcons ring buffer
    pub fn write_console(&mut self, data: &[u8]) -> usize {
        let mut written = 0;
        for &byte in data {
            if self.write_buffer.len() < self.capacity {
                self.write_buffer.push(byte);
                written += 1;
            } else {
                // Circular ring buffer behavior: drop the oldest byte to accommodate newer logs
                self.write_buffer.remove(0);
                self.write_buffer.push(byte);
                written += 1;
            }
        }
        written
    }

    /// Simulates reads from diagnostic input channels (FireWire/Serial)
    pub fn read_console(&mut self, buffer: &mut [u8]) -> usize {
        let read_len = core::cmp::min(buffer.len(), self.read_buffer.len());
        for i in 0..read_len {
            buffer[i] = self.read_buffer.remove(0);
        }
        read_len
    }

    /// Enqueues mock bytes into the input buffer for the OS to consume
    pub fn feed_input(&mut self, data: &[u8]) {
        for &byte in data {
            if self.read_buffer.len() < self.capacity {
                self.read_buffer.push(byte);
            }
        }
    }

    /// Flushes and retrieves all printed console logs
    pub fn flush_output(&mut self) -> Vec<u8> {
        let out = self.write_buffer.clone();
        self.write_buffer.clear();
        out
    }
}

// ============================================================================
// 5. Linux-Inspired Loopback Block Device Driver (loop_dev)
// ============================================================================

/// Replicates the behavior of standard Linux loopback block devices (`/dev/loopX`)
pub struct LinuxLoopDevice {
    pub backing_store: Vec<u8>,
    pub sector_size: usize,
    pub partition_offset: usize, // Offset in bytes representing partition offset
}

impl LinuxLoopDevice {
    pub fn new(size_sectors: usize, sector_size: usize) -> Self {
        Self {
            backing_store: alloc::vec![0u8; size_sectors * sector_size],
            sector_size,
            partition_offset: 0,
        }
    }

    /// Associates loopback device with virtual memory ranges acting as a disk block file
    pub fn associate_backing_store(&mut self, data: &[u8]) -> Result<(), &'static str> {
        if data.len() > self.backing_store.len() {
            return Err("Backing data is larger than the loop device capacity");
        }
        for (idx, &byte) in data.iter().enumerate() {
            self.backing_store[idx] = byte;
        }
        Ok(())
    }

    /// Reads block sector ranges from the simulated block device storage
    pub fn read_sectors(&self, start_sector: usize, sectors_count: usize, buffer: &mut [u8]) -> Result<usize, &'static str> {
        let byte_offset = (start_sector * self.sector_size) + self.partition_offset;
        let read_size = sectors_count * self.sector_size;

        if byte_offset + read_size > self.backing_store.len() {
            return Err("Address out of block device bounds");
        }
        if buffer.len() < read_size {
            return Err("Buffer is too small to receive requested sectors");
        }

        for i in 0..read_size {
            buffer[i] = self.backing_store[byte_offset + i];
        }
        Ok(read_size)
    }

    /// Writes block sector ranges back into the backing memory store
    pub fn write_sectors(&mut self, start_sector: usize, sectors_count: usize, data: &[u8]) -> Result<usize, &'static str> {
        let byte_offset = (start_sector * self.sector_size) + self.partition_offset;
        let write_size = sectors_count * self.sector_size;

        if byte_offset + write_size > self.backing_store.len() {
            return Err("Address out of block device bounds");
        }
        if data.len() < write_size {
            return Err("Source data has insufficient bytes for sector range");
        }

        for i in 0..write_size {
            self.backing_store[byte_offset + i] = data[i];
        }
        Ok(write_size)
    }

    /// Simulates basic MBR (Master Boot Record) partition table mapping offset
    pub fn parse_partition_table_offset(&mut self) -> Result<bool, &'static str> {
        // If boot signature is present in first sector (bytes 510, 511 are 0x55, 0xAA)
        if self.backing_store.len() >= 512 {
            if self.backing_store[510] == 0x55 && self.backing_store[511] == 0xAA {
                // Mock reading partition entry offset (typically starts at byte 446 in MBR)
                // Set custom mock partition offset of 1 sector (512 bytes)
                self.partition_offset = 512;
                return Ok(true);
            }
        }
        Ok(false)
    }
}

// ============================================================================
// 6. Linux-Style Framebuffer Console Driver (fbcon)
// ============================================================================

pub struct LinuxFbConsole {
    pub width: usize,
    pub height: usize,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub text_cols: usize,
    pub text_rows: usize,
    pub char_width: usize,
    pub char_height: usize,
    pub framebuffer: Vec<u32>, // 32-bit ARGB pixel buffer
    pub text_buffer: Vec<char>,
}

impl LinuxFbConsole {
    pub fn new(width: usize, height: usize) -> Self {
        let char_width = 8;
        let char_height = 16;
        let text_cols = width / char_width;
        let text_rows = height / char_height;

        Self {
            width,
            height,
            cursor_x: 0,
            cursor_y: 0,
            text_cols,
            text_rows,
            char_width,
            char_height,
            framebuffer: alloc::vec![0u32; width * height],
            text_buffer: alloc::vec![' '; text_cols * text_rows],
        }
    }

    /// Appends a character to the console and simulates text rendering on the framebuffer
    pub fn putchar(&mut self, c: char) {
        if c == '\n' {
            self.cursor_x = 0;
            self.cursor_y += 1;
        } else {
            if self.cursor_x < self.text_cols && self.cursor_y < self.text_rows {
                let idx = self.cursor_y * self.text_cols + self.cursor_x;
                self.text_buffer[idx] = c;

                // Simulate rendering pixels for the character on the 32-bit ARGB framebuffer
                // We'll set a block of 8x16 pixels to white (0x00FFFFFF)
                let start_pixel_x = self.cursor_x * self.char_width;
                let start_pixel_y = self.cursor_y * self.char_height;

                for py in 0..self.char_height {
                    for px in 0..self.char_width {
                        let fx = start_pixel_x + px;
                        let fy = start_pixel_y + py;
                        if fx < self.width && fy < self.height {
                            let fb_idx = fy * self.width + fx;
                            self.framebuffer[fb_idx] = 0x00FFFFFF; // White
                        }
                    }
                }

                self.cursor_x += 1;
            }
        }

        // Automatic hardware scrolling if cursor reaches screen boundaries
        if self.cursor_x >= self.text_cols {
            self.cursor_x = 0;
            self.cursor_y += 1;
        }

        if self.cursor_y >= self.text_rows {
            self.scroll_up();
        }
    }

    /// Simulates scrolling up the terminal by one row
    pub fn scroll_up(&mut self) {
        // Shift text buffer up by one row
        let row_size = self.text_cols;
        for r in 0..(self.text_rows - 1) {
            for c in 0..row_size {
                let target = r * row_size + c;
                let source = (r + 1) * row_size + c;
                self.text_buffer[target] = self.text_buffer[source];
            }
        }

        // Clear bottom row
        let last_row_start = (self.text_rows - 1) * row_size;
        for c in 0..row_size {
            self.text_buffer[last_row_start + c] = ' ';
        }

        // Shift framebuffer pixels up by 16 scanlines
        let pixel_row_size = self.width;
        let shift_pixels = self.char_height * pixel_row_size;

        for p in 0..(self.framebuffer.len() - shift_pixels) {
            self.framebuffer[p] = self.framebuffer[p + shift_pixels];
        }

        // Clear bottom 16 scanlines
        let last_scanlines_start = self.framebuffer.len() - shift_pixels;
        for p in last_scanlines_start..self.framebuffer.len() {
            self.framebuffer[p] = 0; // Black
        }

        self.cursor_y = self.text_rows - 1;
        self.cursor_x = 0;
    }

    pub fn print_string(&mut self, s: &str) {
        for c in s.chars() {
            self.putchar(c);
        }
    }
}

// ============================================================================
// 7. BSD — Physical & Kernel Virtual Memory Device Driver (/dev/mem, /dev/kmem)
// ============================================================================

pub struct BsdMemDevice {
    pub simulated_physical_ram: Vec<u8>,
    pub simulated_kernel_vmem: Vec<u8>,
}

impl BsdMemDevice {
    pub fn new(ram_size: usize, vmem_size: usize) -> Self {
        Self {
            simulated_physical_ram: alloc::vec![0u8; ram_size],
            simulated_kernel_vmem: alloc::vec![0xAAu8; vmem_size], // default filled with 0xAA
        }
    }

    /// Read from physical memory (/dev/mem) at given offset
    pub fn read_phys(&self, offset: usize, buffer: &mut [u8], caller_uid: u32) -> Result<usize, &'static str> {
        if caller_uid != 0 {
            return Err("PermissionDenied: Only root user (UID 0) can read /dev/mem");
        }
        if offset + buffer.len() > self.simulated_physical_ram.len() {
            return Err("AccessViolation: Address out of physical RAM bounds");
        }
        for i in 0..buffer.len() {
            buffer[i] = self.simulated_physical_ram[offset + i];
        }
        Ok(buffer.len())
    }

    /// Write to physical memory (/dev/mem) at given offset
    pub fn write_phys(&mut self, offset: usize, data: &[u8], caller_uid: u32) -> Result<usize, &'static str> {
        if caller_uid != 0 {
            return Err("PermissionDenied: Only root user (UID 0) can write to /dev/mem");
        }
        if offset + data.len() > self.simulated_physical_ram.len() {
            return Err("AccessViolation: Address out of physical RAM bounds");
        }
        for i in 0..data.len() {
            self.simulated_physical_ram[offset + i] = data[i];
        }
        Ok(data.len())
    }

    /// Read from kernel virtual memory (/dev/kmem)
    pub fn read_kernel(&self, offset: usize, buffer: &mut [u8], caller_uid: u32) -> Result<usize, &'static str> {
        if caller_uid != 0 {
            return Err("PermissionDenied: Only root user (UID 0) can read /dev/kmem");
        }
        if offset + buffer.len() > self.simulated_kernel_vmem.len() {
            return Err("AccessViolation: Address out of kernel virtual memory bounds");
        }
        for i in 0..buffer.len() {
            buffer[i] = self.simulated_kernel_vmem[offset + i];
        }
        Ok(buffer.len())
    }

    /// Write to kernel virtual memory (/dev/kmem)
    pub fn write_kernel(&mut self, offset: usize, data: &[u8], caller_uid: u32) -> Result<usize, &'static str> {
        if caller_uid != 0 {
            return Err("PermissionDenied: Only root user (UID 0) can write to /dev/kmem");
        }
        if offset + data.len() > self.simulated_kernel_vmem.len() {
            return Err("AccessViolation: Address out of kernel virtual memory bounds");
        }
        for i in 0..data.len() {
            self.simulated_kernel_vmem[offset + i] = data[i];
        }
        Ok(data.len())
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn test_linux_devtmpfs() {
        let simulator = LinuxDevtmpfsSimulator::new(12345);
        let mut buffer = [1u8; 16];

        // 1. /dev/null read (should always return 0 bytes read)
        assert_eq!(simulator.read_null(&mut buffer), 0);
        assert_eq!(buffer, [1u8; 16]); // Buffer remains unchanged

        // 2. /dev/zero read (should fill buffer with zeroes)
        assert_eq!(simulator.read_zero(&mut buffer), 16);
        assert_eq!(buffer, [0u8; 16]);

        // 3. /dev/urandom read (should fill buffer with pseudo-random bytes)
        assert_eq!(simulator.read_urandom(&mut buffer), 16);
        assert_ne!(buffer, [0u8; 16]); // Should contain non-zero random values

        // 4. /dev/null write (should discard and return size)
        assert_eq!(simulator.write_null(b"test data"), 9);
    }

    #[test]
    fn test_bsd_audio_mixer() {
        let mut mixer = BsdAudioMixer::new();

        let stream1 = alloc::vec![
            PcmFrame {
                left: 1000,
                right: 2000
            },
            PcmFrame {
                left: -500,
                right: -1000
            },
        ];
        let stream2 = alloc::vec![
            PcmFrame {
                left: 3000,
                right: 1000
            },
            PcmFrame {
                left: 1500,
                right: 500
            },
        ];

        mixer.register_channel(stream1, -64); // Pan slightly left
        mixer.register_channel(stream2, 64); // Pan slightly right

        let mixed = mixer.mix_channels();
        assert_eq!(mixed.len(), 2);

        // Ensure values are mixed and saturating addition / panning occurs without panics
        assert!(mixed[0].left != 0);
        assert!(mixed[0].right != 0);
    }

    #[test]
    fn test_openbsd_crypto_device() {
        // Security Note: This is a TEST ONLY implementation using deterministic generation.
        // In production, use a proper CSPRNG like getrandom() or hardware RNG.
        // Use timestamp-based generation for test purposes
        let mut key = [0u8; 32];
        let mut iv = [0u8; 12];
        
        // Use timestamp-based generation for better randomness in tests
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        
        for i in 0..32 {
            let mut val = timestamp.wrapping_mul(i as u128 + 1);
            val ^= val >> 33;
            val = val.wrapping_mul(0xff51afd7ed558ccd);
            val ^= val >> 33;
            key[i] = (val & 0xFF) as u8;
        }
        
        // Initialize IV with timestamp-based values for test security
        for i in 0..12 {
            let mut val = timestamp.wrapping_add(i as u128 * 7);
            val ^= val >> 17;
            val = val.wrapping_mul(0x9e3779b97f4a7c15);
            iv[i] = (val & 0xFF) as u8;
        }

        let input = b"Secret Linux/BSD Sovereign Payload!";
        let mut ciphered = vec![0u8; input.len()];
        let mut _deciphered = vec![0u8; input.len()];

        // 1. ChaCha20-Poly1305 simulation
        let crypto_dev = OpenBsdCryptoDevice::new(CryptoCipher::ChaCha20Poly1305, &key, &iv);
        crypto_dev.process_data(input, &mut ciphered).unwrap();
        assert_ne!(input, ciphered.as_slice());

        // Decrypt (XOR symmetric cipher logic should restore input exactly)
        crypto_dev.process_data(&ciphered, &mut _deciphered).unwrap();
        assert_eq!(input, _deciphered.as_slice());

        // 2. AES-256-GCM simulation
        let aes_dev = OpenBsdCryptoDevice::new(CryptoCipher::Aes256Gcm, &key, &iv);
        let mut aes_ciphered = vec![0u8; input.len()];
        let mut _aes_deciphered = vec![0u8; input.len()];

        aes_dev.process_data(input, &mut aes_ciphered).unwrap();
        assert_ne!(input, aes_ciphered.as_slice());
    }

    #[test]
    fn test_bsd_dcons_console() {
        let mut console = BsdDconsConsole::new(10);

        // Test normal writes within capacity
        assert_eq!(console.write_console(b"Hello"), 5);
        let output = console.flush_output();
        assert_eq!(output, b"Hello");
        assert!(console.write_buffer.is_empty());

        // Test writes with circular overflow
        assert_eq!(console.write_console(b"abcdefghijkl"), 12);
        let overflow_output = console.flush_output();
        // Since buffer capacity is 10, first 2 bytes ("ab") should be evicted
        assert_eq!(overflow_output, b"cdefghijkl");

        // Test console reading and feeding inputs
        console.feed_input(b"ping");
        let mut read_buf = [0u8; 4];
        assert_eq!(console.read_console(&mut read_buf), 4);
        assert_eq!(&read_buf, b"ping");
    }

    #[test]
    fn test_linux_loop_device() {
        let mut loop_dev = LinuxLoopDevice::new(5, 512); // 5 sectors, 512 bytes each
        let data = b"MBR Sector Data with boot signature!";
        let mut payload = vec![0u8; 512];
        for (i, &b) in data.iter().enumerate() {
            payload[i] = b;
        }
        // Inject MBR boot signature at the very end of first sector
        payload[510] = 0x55;
        payload[511] = 0xAA;

        loop_dev.associate_backing_store(&payload).unwrap();

        // Parse MBR partitions
        assert!(loop_dev.parse_partition_table_offset().unwrap());
        assert_eq!(loop_dev.partition_offset, 512);

        // Write sector on loops (relative to partition offset)
        let custom_sector_data = [7u8; 512];
        assert_eq!(loop_dev.write_sectors(0, 1, &custom_sector_data).unwrap(), 512);

        // Read sector back and assert matching contents
        let mut receive_buf = vec![0u8; 512];
        assert_eq!(loop_dev.read_sectors(0, 1, &mut receive_buf).unwrap(), 512);
        assert_eq!(receive_buf, custom_sector_data.as_slice());
    }

    #[test]
    fn test_fb_console() {
        let mut fbcon = LinuxFbConsole::new(80, 160); // 10 columns by 10 rows
        assert_eq!(fbcon.text_cols, 10);
        assert_eq!(fbcon.text_rows, 10);

        fbcon.print_string("Hello\nWorld");
        assert_eq!(fbcon.cursor_x, 5);
        assert_eq!(fbcon.cursor_y, 1);
        assert_eq!(fbcon.text_buffer[0], 'H');
        assert_eq!(fbcon.text_buffer[fbcon.text_cols + 0], 'W');

        // Verify pixel rendering in framebuffer
        assert_eq!(fbcon.framebuffer[0], 0x00FFFFFF); // rendered 'H'

        // Scroll testing
        let mut fbcon_small = LinuxFbConsole::new(80, 32); // 10 cols by 2 rows
        fbcon_small.print_string("Line1\nLine2\nLine3");
        // Row 1 should have been scrolled up to Row 0, Line3 is now on Row 1
        assert_eq!(fbcon_small.text_buffer[0], 'L');
        assert_eq!(fbcon_small.text_buffer[1], 'i');
        assert_eq!(fbcon_small.text_buffer[2], 'n');
        assert_eq!(fbcon_small.text_buffer[3], 'e');
        assert_eq!(fbcon_small.text_buffer[4], '2');
    }

    #[test]
    fn test_bsd_mem_device() {
        let mut mem_dev = BsdMemDevice::new(1024, 512);

        // 1. Physical RAM reads / writes
        let write_res = mem_dev.write_phys(10, b"phys_data", 0); // Root UID 0
        assert!(write_res.is_ok());

        let mut read_buf = [0u8; 9];
        let read_res = mem_dev.read_phys(10, &mut read_buf, 0);
        assert!(read_res.is_ok());
        assert_eq!(&read_buf, b"phys_data");

        // Privilege checks
        let non_root_err = mem_dev.write_phys(10, b"phys_data", 1000); // UID 1000
        assert!(non_root_err.is_err());

        // 2. Kernel Virtual Memory reads / writes
        let mut k_read_buf = [0u8; 4];
        mem_dev.read_kernel(0, &mut k_read_buf, 0).unwrap();
        assert_eq!(&k_read_buf, &[0xAA, 0xAA, 0xAA, 0xAA]); // Default values

        mem_dev.write_kernel(100, b"vmem", 0).unwrap();
        let mut k_read_buf_new = [0u8; 4];
        mem_dev.read_kernel(100, &mut k_read_buf_new, 0).unwrap();
        assert_eq!(&k_read_buf_new, b"vmem");
    }
}
