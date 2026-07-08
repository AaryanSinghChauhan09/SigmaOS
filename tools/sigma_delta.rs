// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// tools/sigma_delta.rs — Binary Delta Update Algorithm
//
// Implements binary delta updates for SigmaOS packages to reduce download sizes.
// Inspired by: Fedora OSTree, bsdiff, rsync algorithm
// Language: Rust (std available for userland tools)

use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

// ── Types ─────────────────────────────────────────────────────────────────────
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    DeltaError(String),
    PatchError(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

// ── Constants ─────────────────────────────────────────────────────────────────
/// Block size for delta computation (64KB).
const BLOCK_SIZE: usize = 64 * 1024;
/// Maximum match length for delta.
const MAX_MATCH_LEN: usize = 64 * 1024;

// ── Delta Header ─────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct DeltaHeader {
    pub magic: [u8; 8],
   pub old_size: u64,
    pub new_size: u64,
    pub block_size: u32,
    pub num_blocks: u32,
}

impl DeltaHeader {
    pub const MAGIC: [u8; 8] = *b"SIGDELTA";
    
    pub fn new(old_size: u64, new_size: u64, block_size: u32, num_blocks: u32) -> Self {
        Self {
            magic: Self::MAGIC,
            old_size,
            new_size,
            block_size,
            num_blocks,
        }
    }
}

// ── Delta Block ────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct DeltaBlock {
    pub offset: u64,
    pub length: u32,
    pub new_data: Vec<u8>,
}

// ── Delta File ───────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct DeltaFile {
    pub header: DeltaHeader,
    pub blocks: Vec<DeltaBlock>,
}

// ── Delta Generator ─────────────────────────────────────────────────────────
pub struct DeltaGenerator {
    pub block_size: usize,
}

impl DeltaGenerator {
    pub fn new(block_size: usize) -> Self {
        Self {
            block_size: block_size.max(4096),
        }
    }

    // ── Public API ────────────────────────────────────────────────────────────

    /// Generate delta between old and new files
    pub fn generate_delta(&self, old_path: &Path, new_path: &Path) -> Result<DeltaFile> {
        let old_data = fs::read(old_path)?;
        let new_data = fs::read(new_path)?;

        let mut blocks = Vec::new();
        let mut old_idx = 0;

        // Build hash table of old file blocks
        let old_blocks = self.build_block_hash(&old_data);

        // Process new file in blocks
        while old_idx < new_data.len() {
            let remaining = new_data.len() - old_idx;
            let chunk_size = remaining.min(self.block_size);
            let chunk = &new_data[old_idx..old_idx + chunk_size];

            // Try to find matching block in old file
            if let Some((old_offset, match_len)) = self.find_match(chunk, &old_data, &old_blocks) {
                blocks.push(DeltaBlock {
                    offset: old_offset as u64,
                    length: match_len as u32,
                    new_data: Vec::new(),
                });
                old_idx += match_len;
            } else {
                // No match, include as new data
                blocks.push(DeltaBlock {
                    offset: 0,
                    length: chunk_size as u32,
                    new_data: chunk.to_vec(),
                });
                old_idx += chunk_size;
            }
        }

        let header = DeltaHeader::new(
            old_data.len() as u64,
            new_data.len() as u64,
            self.block_size as u32,
            blocks.len() as u32,
        );

        Ok(DeltaFile { header, blocks })
    }

    /// Apply delta to old file to create new file
    pub fn apply_delta(&self, old_path: &Path, delta: &DeltaFile, output_path: &Path) -> Result<()> {
        let old_data = fs::read(old_path)?;
        let mut output = Vec::with_capacity(delta.header.new_size as usize);

        for block in &delta.blocks {
            if block.new_data.is_empty() {
                // Copy from old file
                let offset = block.offset as usize;
                let length = block.length as usize;
                if offset + length <= old_data.len() {
                    output.extend_from_slice(&old_data[offset..offset + length]);
                } else {
                    return Err(Error::PatchError("Invalid delta offset".to_string()));
                }
            } else {
                // Use new data
                output.extend_from_slice(&block.new_data);
            }
        }

        fs::write(output_path, output)?;
        Ok(())
    }

    // ── Helper Functions ───────────────────────────────────────────────────────

    fn build_block_hash(&self, data: &[u8]) -> Vec<(u64, usize)> {
        let mut hash_table = Vec::new();
        let mut idx = 0;

        while idx + self.block_size <= data.len() {
            let block = &data[idx..idx + self.block_size];
            let hash = self.compute_hash(block);
            hash_table.push((hash, idx));
            idx += self.block_size;
        }

        hash_table
    }

    fn find_match(&self, chunk: &[u8], old_data: &[u8], hash_table: &[(u64, usize)]) -> Option<(usize, usize)> {
        let chunk_hash = self.compute_hash(chunk);

        // Find potential matches in hash table
        for &(hash, offset) in hash_table {
            if hash == chunk_hash {
                // Verify actual match
                let max_len = (old_data.len() - offset).min(chunk.len()).min(MAX_MATCH_LEN);
                let mut match_len = 0;

                while match_len < max_len && chunk[match_len] == old_data[offset + match_len] {
                    match_len += 1;
                }

                if match_len >= 16 {
                    return Some((offset, match_len));
                }
            }
        }

        None
    }

    fn compute_hash(&self, data: &[u8]) -> u64 {
        // Simple rolling hash (similar to rsync)
        let mut hash: u64 = 5381;
        for &byte in data {
            hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
        }
        hash
    }

    /// Write delta to file
    pub fn write_delta(&self, delta: &DeltaFile, output_path: &Path) -> Result<()> {
        let file = File::create(output_path)?;
        let mut writer = BufWriter::new(file);

        // Write header
        writer.write_all(&delta.header.magic)?;
        writer.write_all(&delta.header.old_size.to_le_bytes())?;
        writer.write_all(&delta.header.new_size.to_le_bytes())?;
        writer.write_all(&delta.header.block_size.to_le_bytes())?;
        writer.write_all(&delta.header.num_blocks.to_le_bytes())?;

        // Write blocks
        for block in &delta.blocks {
            writer.write_all(&block.offset.to_le_bytes())?;
            writer.write_all(&block.length.to_le_bytes())?;
            writer.write_all(&(block.new_data.len() as u32).to_le_bytes())?;
            writer.write_all(&block.new_data)?;
        }

        writer.flush()?;
        Ok(())
    }

    /// Read delta from file
    pub fn read_delta(&self, input_path: &Path) -> Result<DeltaFile> {
        let file = File::open(input_path)?;
        let mut reader = BufReader::new(file);

        let mut magic = [0u8; 8];
        reader.read_exact(&mut magic)?;

        if magic != DeltaHeader::MAGIC {
            return Err(Error::DeltaError("Invalid delta magic".to_string()));
        }

        let mut old_size = [0u8; 8];
        let mut new_size = [0u8; 8];
        let mut block_size = [0u8; 4];
        let mut num_blocks = [0u8; 4];

        reader.read_exact(&mut old_size)?;
        reader.read_exact(&mut new_size)?;
        reader.read_exact(&mut block_size)?;
        reader.read_exact(&mut num_blocks)?;

        let header = DeltaHeader {
            magic,
            old_size: u64::from_le_bytes(old_size),
            new_size: u64::from_le_bytes(new_size),
            block_size: u32::from_le_bytes(block_size),
            num_blocks: u32::from_le_bytes(num_blocks),
        };

        let mut blocks = Vec::new();
        for _ in 0..header.num_blocks {
            let mut offset = [0u8; 8];
            let mut length = [0u8; 4];
            let mut new_data_len = [0u8; 4];

            reader.read_exact(&mut offset)?;
            reader.read_exact(&mut length)?;
            reader.read_exact(&mut new_data_len)?;

            let offset = u64::from_le_bytes(offset);
            let length = u32::from_le_bytes(length);
            let new_data_len = u32::from_le_bytes(new_data_len);

            let mut new_data = vec![0u8; new_data_len as usize];
            if new_data_len > 0 {
                reader.read_exact(&mut new_data)?;
            }

            blocks.push(DeltaBlock {
                offset,
                length,
                new_data,
            });
        }

        Ok(DeltaFile { header, blocks })
    }

    /// Calculate delta compression ratio
    pub fn compression_ratio(&self, delta: &DeltaFile) -> f64 {
        let delta_size = self.estimate_delta_size(delta);
        let new_size = delta.header.new_size as f64;
        if new_size > 0.0 {
            1.0 - (delta_size / new_size)
        } else {
            0.0
        }
    }

    fn estimate_delta_size(&self, delta: &DeltaFile) -> f64 {
        let mut size = 24.0; // header size
        for block in &delta.blocks {
            size += 16.0; // block metadata
            if !block.new_data.is_empty() {
                size += block.new_data.len() as f64;
            }
        }
        size
    }
}

// ── CLI Interface ─────────────────────────────────────────────────────────────
pub fn run_delta_generator(args: Vec<String>) -> Result<()> {
    if args.len() < 4 {
        eprintln!("Usage: sigma-delta <command> [args]");
        eprintln!("Commands: generate, apply, info");
        eprintln!("  generate <old> <new> <output>");
        eprintln!("  apply <old> <delta> <output>");
        eprintln!("  info <delta>");
        std::process::exit(1);
    }

    let generator = DeltaGenerator::new(BLOCK_SIZE);

    match args[1].as_str() {
        "generate" => {
            if args.len() < 5 {
                eprintln!("Usage: sigma-delta generate <old> <new> <output>");
                std::process::exit(1);
            }
            let old_path = Path::new(&args[2]);
            let new_path = Path::new(&args[3]);
            let output_path = Path::new(&args[4]);

            let delta = generator.generate_delta(old_path, new_path)?;
            generator.write_delta(&delta, output_path)?;

            let ratio = generator.compression_ratio(&delta);
            println!("Delta generated: {}", output_path.display());
            println!("Compression ratio: {:.2}%", ratio * 100.0);
        }
        "apply" => {
            if args.len() < 5 {
                eprintln!("Usage: sigma-delta apply <old> <delta> <output>");
                std::process::exit(1);
            }
            let old_path = Path::new(&args[2]);
            let delta_path = Path::new(&args[3]);
            let output_path = Path::new(&args[4]);

            let delta = generator.read_delta(delta_path)?;
            generator.apply_delta(old_path, &delta, output_path)?;

            println!("Delta applied: {}", output_path.display());
        }
        "info" => {
            if args.len() < 3 {
                eprintln!("Usage: sigma-delta info <delta>");
                std::process::exit(1);
            }
            let delta_path = Path::new(&args[2]);
            let delta = generator.read_delta(delta_path)?;

            println!("Delta Information:");
            println!("  Old size: {} bytes", delta.header.old_size);
            println!("  New size: {} bytes", delta.header.new_size);
            println!("  Block size: {} bytes", delta.header.block_size);
            println!("  Num blocks: {}", delta.header.num_blocks);
            println!("  Compression ratio: {:.2}%", generator.compression_ratio(&delta) * 100.0);
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Err(e) = run_delta_generator(args) {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}
