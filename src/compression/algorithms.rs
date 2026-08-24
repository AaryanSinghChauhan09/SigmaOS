// #![no_std]
// #![cfg_attr(not(test), no_main)]

/// Custom Compression Algorithms for SigmaOS
/// Implements compression without relying on external compression libraries
/// Includes DEFLATE, LZ77, and Huffman coding

extern crate alloc;
use alloc::vec::Vec;
use alloc::boxed::Box;

/// LZ77 match
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LZ77Match {
    pub offset: u16,
    pub length: u16,
}

impl LZ77Match {
    pub fn new(offset: u16, length: u16) -> Self {
        LZ77Match {
            offset,
            length,
        }
    }
}

/// LZ77 compressor
pub struct LZ77Compressor {
    window_size: usize,
    lookahead_size: usize,
}

impl LZ77Compressor {
    pub fn new(window_size: usize, lookahead_size: usize) -> Self {
        LZ77Compressor {
            window_size,
            lookahead_size,
        }
    }

    pub fn compress(&self, data: &[u8]) -> Vec<LZ77Match> {
        let mut matches = Vec::new();
        let mut pos = 0;

        while pos < data.len() {
            let window_start = if pos > self.window_size {
                pos - self.window_size
            } else {
                0
            };

            let lookahead_end = (pos + self.lookahead_size).min(data.len());
            let lookahead = &data[pos..lookahead_end];

            // Find best match in window
            let best_match = self.find_best_match(&data[window_start..pos], lookahead);

            if let Some(match_) = best_match {
                matches.push(match_);
                pos += match_.length as usize;
            } else {
                // Literal byte
                matches.push(LZ77Match::new(0, 1));
                pos += 1;
            }
        }

        matches
    }

    fn find_best_match(&self, window: &[u8], lookahead: &[u8]) -> Option<LZ77Match> {
        let mut best_match = None;
        let mut best_length = 0;

        if window.is_empty() || lookahead.is_empty() {
            return None;
        }

        let max_length = lookahead.len().min(258); // DEFLATE max match length
        let max_offset = window.len().min(32768); // DEFLATE max offset

        for offset in 1..=max_offset {
            let match_start = window.len() - offset;
            let mut match_length = 0;

            while match_length < max_length && match_length < lookahead.len() {
                if window[match_start + match_length] == lookahead[match_length] {
                    match_length += 1;
                } else {
                    break;
                }
            }

            if match_length >= 3 && match_length > best_length {
                best_length = match_length;
                best_match = Some(LZ77Match::new(offset as u16, match_length as u16));
            }
        }

        best_match
    }
}

/// Huffman node
pub enum HuffmanNode {
    Leaf { value: u8, frequency: u32 },
    Internal { left: Box<HuffmanNode>, right: Box<HuffmanNode>, frequency: u32 },
}

impl HuffmanNode {
    fn frequency(&self) -> u32 {
        match self {
            HuffmanNode::Leaf { frequency, .. } => *frequency,
            HuffmanNode::Internal { frequency, .. } => *frequency,
        }
    }
}

/// Huffman tree
pub struct HuffmanTree {
    root: Option<Box<HuffmanNode>>,
}

impl HuffmanTree {
    pub fn new() -> Self {
        HuffmanTree {
            root: None,
        }
    }

    pub fn build_from_frequencies(&mut self, frequencies: &[u32; 256]) {
        let mut nodes: Vec<Box<HuffmanNode>> = frequencies
            .iter()
            .enumerate()
            .filter(|(_, &freq)| freq > 0)
            .map(|(value, &freq)| Box::new(HuffmanNode::Leaf { value: value as u8, frequency: freq }))
            .collect();

        while nodes.len() > 1 {
            // Sort by frequency
            nodes.sort_by_key(|n| n.frequency());

            // Take two lowest frequency nodes
            let left = nodes.remove(0);
            let right = nodes.remove(0);
            let frequency = left.frequency() + right.frequency();

            let internal = Box::new(HuffmanNode::Internal { left, right, frequency });
            nodes.push(internal);
        }

        self.root = nodes.pop();
    }

    pub fn encode(&self, data: &[u8]) -> Vec<bool> {
        let mut encoded = Vec::new();
        let mut codes: [Option<Vec<bool>>; 256] = [const { None }; 256];

        if let Some(ref root) = self.root {
            self.generate_codes(root, &mut [], &mut codes);
        }

        for &byte in data {
            if let Some(ref code) = codes[byte as usize] {
                encoded.extend(code.iter().cloned());
            }
        }

        encoded
    }

    fn generate_codes(&self, node: &HuffmanNode, current_code: &[bool], codes: &mut [Option<Vec<bool>>; 256]) {
        match node {
            HuffmanNode::Leaf { value, .. } => {
                codes[*value as usize] = Some(current_code.to_vec());
            }
            HuffmanNode::Internal { left, right, .. } => {
                let mut left_code = current_code.to_vec();
                left_code.push(false);
                self.generate_codes(left, &left_code, codes);

                let mut right_code = current_code.to_vec();
                right_code.push(true);
                self.generate_codes(right, &right_code, codes);
            }
        }
    }

    pub fn decode(&self, encoded: &[bool]) -> Vec<u8> {
        let mut decoded = Vec::new();
        let mut bit_pos = 0;

        if let Some(ref root) = self.root {
            while bit_pos < encoded.len() {
                let (byte, bits_consumed) = self.decode_byte(root, encoded, bit_pos);
                decoded.push(byte);
                bit_pos += bits_consumed;
            }
        }

        decoded
    }

    fn decode_byte(&self, node: &HuffmanNode, encoded: &[bool], bit_pos: usize) -> (u8, usize) {
        match node {
            HuffmanNode::Leaf { value, .. } => (*value, 0),
            HuffmanNode::Internal { left, right, .. } => {
                if bit_pos < encoded.len() && !encoded[bit_pos] {
                    let (byte, bits) = self.decode_byte(left, encoded, bit_pos + 1);
                    (byte, bits + 1)
                } else {
                    let (byte, bits) = self.decode_byte(right, encoded, bit_pos + 1);
                    (byte, bits + 1)
                }
            }
        }
    }
}

/// DEFLATE compressor
pub struct DeflateCompressor {
    lz77: LZ77Compressor,
    huffman: HuffmanTree,
}

impl DeflateCompressor {
    pub fn new() -> Self {
        DeflateCompressor {
            lz77: LZ77Compressor::new(32768, 258),
            huffman: HuffmanTree::new(),
        }
    }

    pub fn compress(&mut self, data: &[u8]) -> Vec<u8> {
        // Step 1: LZ77 compression
        let matches = self.lz77.compress(data);

        // Step 2: Build frequency table
        let mut frequencies = [0u32; 256];
        for match_ in &matches {
            if match_.offset == 0 {
                // Literal
                frequencies[0] = frequencies[0].wrapping_add(1);
            } else {
                // Match
                frequencies[1] = frequencies[1].wrapping_add(1);
            }
        }

        // Step 3: Build Huffman tree
        self.huffman.build_from_frequencies(&frequencies);

        // Step 4: Huffman encode
        let encoded = self.huffman.encode(data);

        // Step 5: Convert to bytes
        let mut compressed = Vec::new();
        let mut current_byte = 0u8;
        let mut bit_count = 0;

        for bit in encoded {
            current_byte |= (bit as u8) << bit_count;
            bit_count += 1;

            if bit_count == 8 {
                compressed.push(current_byte);
                current_byte = 0;
                bit_count = 0;
            }
        }

        if bit_count > 0 {
            compressed.push(current_byte);
        }

        compressed
    }

    pub fn decompress(&mut self, compressed: &[u8]) -> Vec<u8> {
        // Convert bytes to bits
        let mut encoded = Vec::new();
        for &byte in compressed {
            for i in 0..8 {
                encoded.push((byte >> i) & 1 == 1);
            }
        }

        // Huffman decode
        self.huffman.decode(&encoded)
    }
}

/// Run-Length Encoding (RLE)
pub struct RLECompressor;

impl RLECompressor {
    pub fn compress(data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let mut compressed = Vec::new();
        let mut current = data[0];
        let mut count = 0u8;

        for &byte in data {
            if byte == current && count < 255 {
                count += 1;
            } else {
                compressed.push(count);
                compressed.push(current);
                current = byte;
                count = 1;
            }
        }

        compressed.push(count);
        compressed.push(current);

        compressed
    }

    pub fn decompress(compressed: &[u8]) -> Vec<u8> {
        if compressed.is_empty() {
            return Vec::new();
        }

        let mut decompressed = Vec::new();

        for chunk in compressed.chunks(2) {
            if chunk.len() == 2 {
                let count = chunk[0] as usize;
                let byte = chunk[1];
                for _ in 0..count {
                    decompressed.push(byte);
                }
            }
        }

        decompressed
    }
}

/// BSD-inspired Fowler-Noll-Vo (FNV-1a) 64-bit non-cryptographic hashing algorithm.
/// Extremely fast lookup algorithm for name matching and filesystem integrity verification.
pub fn fnv1a_64(data: &[u8]) -> u64 {
    let mut hash = 14695981039346656037u64; // FNV-1a 64-bit offset basis
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(1099511628211u64); // FNV-1a 64-bit prime
    }
    hash
}

/// Linux-inspired 32-bit Cyclic Redundancy Check (CRC32) algorithm (IEEE 802.3 standard).
/// Used across Linux for network frame FCS, ext4 metadata checksums, and package integrity validation.
pub fn crc32_checksum(data: &[u8]) -> u32 {
    let mut crc = 0xFFFFFFFFu32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fnv1a_hashing() {
        let data = b"SigmaOS Hashing";
        let hash = fnv1a_64(data);
        assert!(hash != 0);
        // Standard FNV-1a check for empty string
        assert_eq!(fnv1a_64(b""), 14695981039346656037u64);
    }

    #[test]
    fn test_crc32_checksum() {
        let data = b"123456789";
        // Standard CRC32 of "123456789" is 0xCBF43926
        assert_eq!(crc32_checksum(data), 0xCBF43926);
    }

    #[test]
    fn test_rle_compression() {
        let data = b"AAAAABBBCC";
        let compressed = RLECompressor::compress(data);
        let decompressed = RLECompressor::decompress(&compressed);
        assert_eq!(decompressed, data);
    }
}
