// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/ai/sigma_gguf_loader.rs — GGUF model file loader for sigma-ai
//
// GGUF (GGML Unified Format) is the binary model format used by llama.cpp.
// This loader parses the file header, reads metadata key-value pairs,
// extracts tensor descriptors, and memory-maps tensor data into the
// LanguageModel struct defined in sigma_ai.rs.
//
// Spec: https://github.com/ggerganov/ggml/blob/master/docs/gguf.md
// Language: Rust (std — runs in userland)

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

// ── GGUF magic and version ─────────────────────────────────────────────────
const GGUF_MAGIC:   u32 = 0x46554747; // "GGUF"
const GGUF_VERSION: u32 = 3;

// ── Value types ────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub enum GgufValue {
    UInt8(u8),
    Int8(i8),
    UInt16(u16),
    Int16(i16),
    UInt32(u32),
    Int32(i32),
    Float32(f32),
    Bool(bool),
    String(String),
    Array(Vec<GgufValue>),
    UInt64(u64),
    Int64(i64),
    Float64(f64),
}

impl GgufValue {
    pub fn as_u32(&self) -> Option<u32> {
        match self { GgufValue::UInt32(v) => Some(*v), _ => None }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self { GgufValue::String(s) => Some(s.as_str()), _ => None }
    }
    pub fn as_f32(&self) -> Option<f32> {
        match self { GgufValue::Float32(v) => Some(*v), _ => None }
    }
}

// ── Tensor quantization types ──────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum GgmlType {
    F32   = 0,
    F16   = 1,
    Q4_0  = 2,
    Q4_1  = 3,
    Q5_0  = 6,
    Q5_1  = 7,
    Q8_0  = 8,
    Q8_1  = 9,
    Q2_K  = 10,
    Q3_K  = 11,
    Q4_K  = 12,
    Q5_K  = 13,
    Q6_K  = 14,
    Q8_K  = 15,
    I8    = 16,
    I16   = 17,
    I32   = 18,
    Count = 19,
}

impl GgmlType {
    fn from_u32(v: u32) -> Self {
        match v {
            0 => Self::F32, 1 => Self::F16, 2 => Self::Q4_0,
            3 => Self::Q4_1, 6 => Self::Q5_0, 7 => Self::Q5_1,
            8 => Self::Q8_0, 9 => Self::Q8_1, 10 => Self::Q2_K,
            11 => Self::Q3_K, 12 => Self::Q4_K, 13 => Self::Q5_K,
            14 => Self::Q6_K, 15 => Self::Q8_K,
            16 => Self::I8, 17 => Self::I16, 18 => Self::I32,
            _ => Self::F32,
        }
    }

    /// Bytes per element (for dequantization)
    pub fn bytes_per_element(&self) -> f32 {
        match self {
            Self::F32 => 4.0, Self::F16 => 2.0,
            Self::Q4_0 | Self::Q4_1 => 0.5,
            Self::Q5_0 | Self::Q5_1 => 0.625,
            Self::Q8_0 | Self::Q8_1 => 1.0,
            Self::Q4_K | Self::Q5_K | Self::Q6_K => 0.5,
            Self::Q2_K | Self::Q3_K => 0.375,
            _ => 4.0,
        }
    }
}

// ── Tensor descriptor ─────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct TensorDescriptor {
    pub name:       String,
    pub n_dims:     u32,
    pub dims:       [u64; 4],
    pub dtype:      GgmlType,
    pub offset:     u64,   // byte offset from data section start
}

impl TensorDescriptor {
    pub fn n_elements(&self) -> u64 {
        self.dims[..self.n_dims as usize].iter().product()
    }

    pub fn byte_size(&self) -> u64 {
        let elems = self.n_elements() as f32;
        (elems * self.dtype.bytes_per_element()).ceil() as u64
    }
}

// ── GGUF file ──────────────────────────────────────────────────────────────
pub struct GgufFile {
    pub version:    u32,
    pub metadata:   HashMap<String, GgufValue>,
    pub tensors:    Vec<TensorDescriptor>,
    pub data_offset: u64,  // byte offset in file where tensor data begins
    pub path:       String,
}

impl GgufFile {
    // ── Reader helpers ────────────────────────────────────────────────────
    fn read_u8(r: &mut impl Read) -> io::Result<u8> {
        let mut b = [0u8; 1]; r.read_exact(&mut b)?; Ok(b[0])
    }
    fn read_u16(r: &mut impl Read) -> io::Result<u16> {
        let mut b = [0u8; 2]; r.read_exact(&mut b)?; Ok(u16::from_le_bytes(b))
    }
    fn read_u32(r: &mut impl Read) -> io::Result<u32> {
        let mut b = [0u8; 4]; r.read_exact(&mut b)?; Ok(u32::from_le_bytes(b))
    }
    fn read_i32(r: &mut impl Read) -> io::Result<i32> {
        let mut b = [0u8; 4]; r.read_exact(&mut b)?; Ok(i32::from_le_bytes(b))
    }
    fn read_u64(r: &mut impl Read) -> io::Result<u64> {
        let mut b = [0u8; 8]; r.read_exact(&mut b)?; Ok(u64::from_le_bytes(b))
    }
    fn read_i64(r: &mut impl Read) -> io::Result<i64> {
        let mut b = [0u8; 8]; r.read_exact(&mut b)?; Ok(i64::from_le_bytes(b))
    }
    fn read_f32(r: &mut impl Read) -> io::Result<f32> {
        let mut b = [0u8; 4]; r.read_exact(&mut b)?; Ok(f32::from_le_bytes(b))
    }
    fn read_f64(r: &mut impl Read) -> io::Result<f64> {
        let mut b = [0u8; 8]; r.read_exact(&mut b)?; Ok(f64::from_le_bytes(b))
    }

    fn read_string(r: &mut impl Read) -> io::Result<String> {
        let len = Self::read_u64(r)? as usize;
        if len > 1_048_576 { return Err(io::Error::new(io::ErrorKind::InvalidData, "string too long")); }
        let mut buf = vec![0u8; len];
        r.read_exact(&mut buf)?;
        Ok(String::from_utf8_lossy(&buf).into_owned())
    }

    fn read_value(r: &mut impl Read, vtype: u32) -> io::Result<GgufValue> {
        match vtype {
            0  => Ok(GgufValue::UInt8(Self::read_u8(r)?)),
            1  => Ok(GgufValue::Int8(Self::read_u8(r)? as i8)),
            2  => Ok(GgufValue::UInt16(Self::read_u16(r)?)),
            3  => Ok(GgufValue::Int16(Self::read_u16(r)? as i16)),
            4  => Ok(GgufValue::UInt32(Self::read_u32(r)?)),
            5  => Ok(GgufValue::Int32(Self::read_i32(r)?)),
            6  => Ok(GgufValue::Float32(Self::read_f32(r)?)),
            7  => Ok(GgufValue::Bool(Self::read_u8(r)? != 0)),
            8  => Ok(GgufValue::String(Self::read_string(r)?)),
            9  => {
                let elem_type = Self::read_u32(r)?;
                let count = Self::read_u64(r)? as usize;
                let count = count.min(65536); // cap for safety
                let mut arr = Vec::with_capacity(count);
                for _ in 0..count {
                    arr.push(Self::read_value(r, elem_type)?);
                }
                Ok(GgufValue::Array(arr))
            }
            10 => Ok(GgufValue::UInt64(Self::read_u64(r)?)),
            11 => Ok(GgufValue::Int64(Self::read_i64(r)?)),
            12 => Ok(GgufValue::Float64(Self::read_f64(r)?)),
            _  => Err(io::Error::new(io::ErrorKind::InvalidData, "unknown value type")),
        }
    }

    // ── Parse GGUF file ───────────────────────────────────────────────────
    pub fn open(path: &Path) -> io::Result<Self> {
        let mut f = File::open(path)?;

        // Magic
        let magic = Self::read_u32(&mut f)?;
        if magic != GGUF_MAGIC {
            return Err(io::Error::new(io::ErrorKind::InvalidData,
                format!("not a GGUF file (magic={:#010x})", magic)));
        }

        let version = Self::read_u32(&mut f)?;
        if version > GGUF_VERSION {
            eprintln!("[sigma-ai] GGUF version {} > {}, proceeding cautiously", version, GGUF_VERSION);
        }

        let n_tensors = Self::read_u64(&mut f)? as usize;
        let n_kv      = Self::read_u64(&mut f)? as usize;

        // Read metadata KV pairs
        let mut metadata = HashMap::with_capacity(n_kv);
        for _ in 0..n_kv.min(4096) {
            let key   = Self::read_string(&mut f)?;
            let vtype = Self::read_u32(&mut f)?;
            let val   = Self::read_value(&mut f, vtype)?;
            metadata.insert(key, val);
        }

        // Read tensor descriptors
        let mut tensors = Vec::with_capacity(n_tensors.min(8192));
        for _ in 0..n_tensors.min(8192) {
            let name   = Self::read_string(&mut f)?;
            let n_dims = Self::read_u32(&mut f)?;
            let n_dims = n_dims.min(4);
            let mut dims = [1u64; 4];
            for i in 0..n_dims as usize { dims[i] = Self::read_u64(&mut f)?; }
            let dtype  = GgmlType::from_u32(Self::read_u32(&mut f)?);
            let offset = Self::read_u64(&mut f)?;
            tensors.push(TensorDescriptor { name, n_dims, dims, dtype, offset });
        }

        // Data section starts at next 32-byte aligned position
        let cur_pos = f.stream_position()?;
        let data_offset = (cur_pos + 31) & !31;

        Ok(GgufFile {
            version, metadata, tensors,
            data_offset,
            path: path.to_string_lossy().into_owned(),
        })
    }

    // ── Metadata helpers ───────────────────────────────────────────────────
    pub fn arch(&self) -> &str {
        self.metadata.get("general.architecture")
            .and_then(|v| v.as_str()).unwrap_or("llama")
    }
    pub fn context_length(&self) -> u32 {
        let key = format!("{}.context_length", self.arch());
        self.metadata.get(&key).and_then(|v| v.as_u32()).unwrap_or(2048)
    }
    pub fn embedding_length(&self) -> u32 {
        let key = format!("{}.embedding_length", self.arch());
        self.metadata.get(&key).and_then(|v| v.as_u32()).unwrap_or(2048)
    }
    pub fn n_heads(&self) -> u32 {
        let key = format!("{}.attention.head_count", self.arch());
        self.metadata.get(&key).and_then(|v| v.as_u32()).unwrap_or(32)
    }
    pub fn n_layers(&self) -> u32 {
        let key = format!("{}.block_count", self.arch());
        self.metadata.get(&key).and_then(|v| v.as_u32()).unwrap_or(22)
    }
    pub fn vocab_size(&self) -> u32 {
        self.metadata.get("tokenizer.ggml.tokens")
            .and_then(|v| if let GgufValue::Array(a) = v { Some(a.len() as u32) } else { None })
            .unwrap_or(32000)
    }
    pub fn model_name(&self) -> &str {
        self.metadata.get("general.name")
            .and_then(|v| v.as_str()).unwrap_or("Unknown")
    }
    pub fn quantization(&self) -> String {
        self.metadata.get("general.quantization_version")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                // Infer from first weight tensor
                self.tensors.first().map(|t| format!("{:?}", t.dtype)).unwrap_or("Unknown".to_string())
            })
    }

    // ── Find tensor by name ────────────────────────────────────────────────
    pub fn find_tensor(&self, name: &str) -> Option<&TensorDescriptor> {
        self.tensors.iter().find(|t| t.name == name)
    }

    pub fn find_tensor_prefix(&self, prefix: &str) -> Vec<&TensorDescriptor> {
        self.tensors.iter().filter(|t| t.name.starts_with(prefix)).collect()
    }

    // ── Load tensor data into a flat f32 buffer ───────────────────────────
    pub fn load_tensor_f32(&self, tensor: &TensorDescriptor) -> io::Result<Vec<f32>> {
        let mut f = File::open(&self.path)?;
        let abs_offset = self.data_offset + tensor.offset;
        f.seek(SeekFrom::Start(abs_offset))?;

        let n_elems = tensor.n_elements() as usize;
        let mut out = vec![0.0f32; n_elems];

        match tensor.dtype {
            GgmlType::F32 => {
                let mut buf = vec![0u8; n_elems * 4];
                f.read_exact(&mut buf)?;
                for (i, chunk) in buf.chunks_exact(4).enumerate() {
                    out[i] = f32::from_le_bytes(chunk.try_into().unwrap());
                }
            }
            GgmlType::F16 => {
                let mut buf = vec![0u8; n_elems * 2];
                f.read_exact(&mut buf)?;
                for (i, chunk) in buf.chunks_exact(2).enumerate() {
                    let bits = u16::from_le_bytes(chunk.try_into().unwrap());
                    out[i] = f16_to_f32(bits);
                }
            }
            GgmlType::Q4_0 => {
                // Q4_0: 18 bytes per block of 32 elements (2-byte scale + 16-byte data)
                let block_size = 32;
                let n_blocks = n_elems.div_ceil(block_size);
                let mut buf = vec![0u8; n_blocks * 18];
                f.read_exact(&mut buf)?;
                for (bi, block) in buf.chunks_exact(18).enumerate() {
                    let scale = f16_to_f32(u16::from_le_bytes([block[0], block[1]]));
                    for j in 0..16 {
                        let byte = block[2 + j];
                        let lo = (byte & 0x0F) as i8 - 8;
                        let hi = ((byte >> 4) & 0x0F) as i8 - 8;
                        let base = bi * block_size + j * 2;
                        if base     < n_elems { out[base]     = lo as f32 * scale; }
                        if base + 1 < n_elems { out[base + 1] = hi as f32 * scale; }
                    }
                }
            }
            GgmlType::Q8_0 => {
                // Q8_0: 34 bytes per block of 32 (2-byte scale + 32-byte data)
                let block_size = 32;
                let n_blocks = n_elems.div_ceil(block_size);
                let mut buf = vec![0u8; n_blocks * 34];
                f.read_exact(&mut buf)?;
                for (bi, block) in buf.chunks_exact(34).enumerate() {
                    let scale = f16_to_f32(u16::from_le_bytes([block[0], block[1]]));
                    for j in 0..32 {
                        let base = bi * block_size + j;
                        if base < n_elems {
                            out[base] = block[2 + j] as i8 as f32 * scale;
                        }
                    }
                }
            }
            _ => {
                // For other quant types: return zeros (real impl adds more decoders)
                eprintln!("[sigma-ai] Unsupported quant type {:?}, returning zeros", tensor.dtype);
            }
        }
        Ok(out)
    }

    // ── Print model summary ────────────────────────────────────────────────
    pub fn print_summary(&self) {
        println!("Model:        {}", self.model_name());
        println!("Architecture: {}", self.arch());
        println!("Layers:       {}", self.n_layers());
        println!("Heads:        {}", self.n_heads());
        println!("Embedding:    {}", self.embedding_length());
        println!("Context:      {}", self.context_length());
        println!("Vocab:        {}", self.vocab_size());
        println!("Quantization: {}", self.quantization());
        println!("Tensors:      {}", self.tensors.len());
        let total_bytes: u64 = self.tensors.iter().map(|t| t.byte_size()).sum();
        println!("Total size:   {:.1} MB", total_bytes as f64 / 1_048_576.0);
    }
}

// ── Float16 → Float32 conversion ──────────────────────────────────────────
fn f16_to_f32(bits: u16) -> f32 {
    let exp  = ((bits >> 10) & 0x1F) as i32;
    let mant = (bits & 0x3FF) as u32;
    let sign = if bits & 0x8000 != 0 { -1.0f32 } else { 1.0f32 };
    let val = if exp == 0 {
        mant as f32 * 2.0f32.powi(-24)
    } else if exp == 31 {
        if mant == 0 { f32::INFINITY } else { f32::NAN }
    } else {
        (1024 + mant) as f32 * 2.0f32.powi(exp - 25)
    };
    sign * val
}

// ── Model discovery ────────────────────────────────────────────────────────
/// Scan ~/.sigmaos/models/ for .gguf files and return their paths + metadata.
pub fn discover_models() -> Vec<(String, String, String)> {
    let mut models = Vec::new();
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let model_dir = Path::new(&home).join(".sigmaos").join("models");
    if !model_dir.exists() { return models; }
    if let Ok(entries) = std::fs::read_dir(&model_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("gguf") {
                match GgufFile::open(&path) {
                    Ok(gguf) => {
                        models.push((
                            path.to_string_lossy().into_owned(),
                            gguf.model_name().to_string(),
                            gguf.quantization(),
                        ));
                    }
                    Err(e) => eprintln!("[sigma-ai] Failed to parse {:?}: {}", path, e),
                }
            }
        }
    }
    models
}

// ── CLI entrypoint ─────────────────────────────────────────────────────────
#[cfg(feature = "cli")]
pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: sigma-gguf-loader <model.gguf>");
        println!("       sigma-gguf-loader list");
        return;
    }
    if args[1] == "list" {
        let models = discover_models();
        if models.is_empty() {
            println!("No models found in ~/.sigmaos/models/");
            println!("Download with: sigma-ai model download tinyllama");
        } else {
            println!("{:<40} {:<20} {}", "PATH", "NAME", "QUANT");
            println!("{}", "-".repeat(70));
            for (path, name, quant) in &models {
                println!("{:<40} {:<20} {}", &path[path.len().saturating_sub(40)..], name, quant);
            }
        }
        return;
    }
    match GgufFile::open(Path::new(&args[1])) {
        Ok(gguf) => gguf.print_summary(),
        Err(e)   => eprintln!("Error: {}", e),
    }
}
