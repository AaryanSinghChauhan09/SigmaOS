# Performance Optimization Guidelines

## Overview
Based on performance learnings from .jules/bolt.md, this document provides guidelines for optimizing SigmaOS components for maximum efficiency and resource utilization.

## SIMD String Operations

### Learning: SIMD String to_uppercase Bitwise Optimization

**Issue:** A bitwise SIMD conversion function for ASCII string case switching can cause silent logical bugs if bitwise AND is improperly masked over non-lowercase ranges, leading to zeroing out all non-lowercase characters.

### Solution

**DO:**
```rust
use std::arch::x86_64::*;

#[cfg(target_arch = "x86_64")]
pub unsafe fn simd_to_uppercase(input: &[u8]) -> Vec<u8> {
    let mut output = input.to_vec();
    
    for chunk in output.chunks_mut(16) {
        let vec = _mm_loadu_si128(chunk.as_ptr() as *const __m128i);
        
        // Use bitwise inverse logical AND instead of direct AND
        let mask = _mm_set1_epi8(0xDF);  // 11011111 in binary
        let lower_mask = _mm_set1_epi8(0x20);  // 00100000 in binary
        
        // Check if character is lowercase (bit 5 set)
        let is_lower = _mm_and_si128(vec, lower_mask);
        let is_lower_cmp = _mm_cmpeq_epi8(is_lower, lower_mask);
        
        // Only apply transformation to lowercase characters
        let transformed = _mm_andnot_si128(is_lower_cmp, _mm_and_si128(vec, mask));
        let result = _mm_or_si128(transformed, _mm_and_si128(is_lower_cmp, _mm_and_si128(vec, mask)));
        
        _mm_storeu_si128(chunk.as_mut_ptr() as *mut __m128i, result);
    }
    
    output
}
```

**DON'T:**
```rust
// Unsafe: Direct AND masks all characters
pub unsafe fn simd_to_uppercase_bad(input: &[u8]) -> Vec<u8> {
    let mut output = input.to_vec();
    
    for chunk in output.chunks_mut(16) {
        let vec = _mm_loadu_si128(chunk.as_ptr() as *const __m128i);
        let mask = _mm_set1_epi8(0xDF);
        
        // This zeros out non-lowercase characters!
        let result = _mm_and_si128(vec, mask);
        
        _mm_storeu_si128(chunk.as_mut_ptr() as *mut __m128i, result);
    }
    
    output
}
```

## Rust Dynamic Trait Compatibility

### Learning: Generic parameters on trait methods prevent dyn compatibility in no_std environments.

### Solution

**DO:**
```rust
// Use trait objects for dynamic dispatch
pub trait LlmBackend {
    fn stream_response(&mut self, prompt: &str, callback: &mut dyn FnMut(&str)) -> Result<()>;
}

// Implementation can use trait objects
pub struct LocalLlmBackend {
    // ... fields
}

impl LlmBackend for LocalLlmBackend {
    fn stream_response(&mut self, prompt: &str, callback: &mut dyn FnMut(&str)) -> Result<()> {
        // Implementation
        Ok(())
    }
}
```

**DON'T:**
```rust
// Generic parameters prevent dyn compatibility
pub trait LlmBackend {
    fn stream_response<F: FnMut(&str)>(&mut self, prompt: &str, callback: F) -> Result<()>;
}

// This cannot be used as &mut dyn LlmBackend
```

## Low-Level Pixel Loop Optimization

### Learning: Hoisting atomic checks, Option matches, bounds checks, and address arithmetic outside of inner pixel loops dramatically improves performance.

### Solution

**DO:**
```rust
pub fn fill_rect(buffer: &mut [u8], x: usize, y: usize, width: usize, height: usize, color: [u8; 4]) {
    let stride = width * 4;
    let total_bytes = height * stride;
    
    // Hoist bounds check
    if x + width > buffer.len() || y + height > buffer.len() {
        return;
    }
    
    // Pre-calculate pointers
    let base_ptr = buffer.as_mut_ptr();
    let start_offset = (y * stride) + (x * 4);
    let mut ptr = base_ptr.add(start_offset);
    
    // Bulk row copy using pointer arithmetic
    for _ in 0..height {
        // Fill entire row at once
        for col in 0..width {
            let pixel_ptr = ptr.add(col * 4);
            std::ptr::copy_nonoverlapping(color.as_ptr(), pixel_ptr, 4);
        }
        ptr = ptr.add(stride);
    }
}
```

**DON'T:**
```rust
// Slow: Per-pixel helper function calls
pub fn fill_rect_slow(buffer: &mut [u8], x: usize, y: usize, width: usize, height: usize, color: [u8; 4]) {
    for py in y..(y + height) {
        for px in x..(x + width) {
            // Bounds check on every pixel
            if px < buffer.len() && py < buffer.len() {
                putpixel(buffer, px, py, color);
            }
        }
    }
}

fn putpixel(buffer: &mut [u8], x: usize, y: usize, color: [u8; 4]) {
    let offset = (y * 4) + (x * 4);
    buffer[offset..offset+4].copy_from_slice(&color);
}
```

## Allocation-Free SemVer Comparison

### Learning: Heap-allocating string parsing in SemVer constraint checking causes fragmentation and slows dependency resolution.

### Solution

**DO:**
```rust
pub fn compare_semver(v1: &str, v2: &str) -> std::cmp::Ordering {
    // Use lazy iterator mapping instead of Vec allocation
    let mut parts1 = v1.split('.').map(|s| s.parse::<u32>().unwrap_or(0));
    let mut parts2 = v2.split('.').map(|s| s.parse::<u32>().unwrap_or(0));
    
    loop {
        match (parts1.next(), parts2.next()) {
            (Some(a), Some(b)) => {
                match a.cmp(&b) {
                    std::cmp::Ordering::Equal => continue,
                    ord => return ord,
                }
            }
            (Some(_), None) => return std::cmp::Ordering::Greater,
            (None, Some(_)) => return std::cmp::Ordering::Less,
            (None, None) => return std::cmp::Ordering::Equal,
        }
    }
}
```

**DON'T:**
```rust
// Slow: Allocates Vec for every comparison
pub fn compare_semver_slow(v1: &str, v2: &str) -> std::cmp::Ordering {
    let parts1: Vec<u32> = v1.split('.').map(|s| s.parse().unwrap()).collect();
    let parts2: Vec<u32> = v2.split('.').map(|s| s.parse().unwrap()).collect();
    
    // Compare vectors
    parts1.cmp(&parts2)
}
```

## DAG Topological Sort for Dependency Resolution

### Learning: Deep dependency trees can cause lock inversions or circular wait conditions if not sorted topologically.

### Solution

**DO:**
```rust
use std::collections::{HashMap, HashSet, VecDeque};

pub fn topological_sort<T: Clone + Eq + std::hash::Hash>(
    nodes: Vec<T>,
    edges: &HashMap<T, Vec<T>>,
) -> Result<Vec<T>, String> {
    let mut in_degree: HashMap<T, usize> = HashMap::new();
    let mut adj_list: HashMap<T, Vec<T>> = HashMap::new();
    
    // Build adjacency list and calculate in-degrees
    for node in &nodes {
        in_degree.entry(node.clone()).or_insert(0);
        adj_list.entry(node.clone()).or_insert_with(Vec::new);
    }
    
    for (from, to_list) in edges {
        for to in to_list {
            adj_list.entry(from.clone()).or_insert_with(Vec::new).push(to.clone());
            *in_degree.entry(to.clone()).or_insert(0) += 1;
        }
    }
    
    // Kahn's algorithm
    let mut queue: VecDeque<T> = in_degree
        .iter()
        .filter(|(_, &deg)| deg == 0)
        .map(|(node, _)| node.clone())
        .collect();
    
    let mut result = Vec::new();
    
    while let Some(node) = queue.pop_front() {
        result.push(node.clone());
        
        if let Some(neighbors) = adj_list.get(&node) {
            for neighbor in neighbors {
                let deg = in_degree.get_mut(neighbor).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }
    
    // Check for cycles
    if result.len() != nodes.len() {
        return Err("Cycle detected in dependency graph".to_string());
    }
    
    Ok(result)
}
```

## On-Device AI Inference Memory Management

### Learning: Running local AI models causes massive memory spikes during weight loading. Memory-mapped files allow kernel paging without OOM events.

### Solution

**DO:**
```rust
use memmap2::Mmap;
use std::fs::File;

pub struct MmappedModel {
    mmap: Mmap,
    _file: File,
}

impl MmappedModel {
    pub fn load(path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        
        Ok(MmappedModel {
            mmap,
            _file: file,
        })
    }
    
    pub fn weights(&self) -> &[u8] {
        &self.mmap
    }
}

// Usage: Weights are paged in/out by kernel as needed
let model = MmappedModel::load("/models/phi3.gguf")?;
let weights = model.weights();
// Process weights - kernel handles paging
```

**DON'T:**
```rust
// Unsafe: Loads entire model into heap RAM
pub struct HeapModel {
    weights: Vec<u8>,
}

impl HeapModel {
    pub fn load(path: &str) -> Result<Self, std::io::Error> {
        let weights = std::fs::read(path)?;  // Allocates entire model
        Ok(HeapModel { weights })
    }
}
// Can cause OOM on systems with limited RAM
```

## Zero-Copy Ring Buffers for Audio

### Learning: Pre-allocated ring buffers with head/tail pointer arithmetic eliminate heap allocation during audio playback.

### Solution

**DO:**
```rust
pub struct AudioRingBuffer {
    buffer: Vec<f32>,
    capacity: usize,
    head: usize,
    tail: usize,
}

impl AudioRingBuffer {
    pub fn new(capacity: usize) -> Self {
        // Pre-allocate to maximum expected size
        AudioRingBuffer {
            buffer: vec![0.0; capacity],
            capacity,
            head: 0,
            tail: 0,
        }
    }
    
    pub fn write(&mut self, samples: &[f32]) -> usize {
        let written = 0;
        
        for &sample in samples {
            let next_tail = (self.tail + 1) % self.capacity;
            
            // Check if buffer is full
            if next_tail == self.head {
                break;
            }
            
            self.buffer[self.tail] = sample;
            self.tail = next_tail;
        }
        
        written
    }
    
    pub fn read(&mut self, output: &mut [f32]) -> usize {
        let mut read = 0;
        
        for out_sample in output {
            if self.head == self.tail {
                break;  // Buffer empty
            }
            
            *out_sample = self.buffer[self.head];
            self.head = (self.head + 1) % self.capacity;
            read += 1;
        }
        
        read
    }
}
```

**DON'T:**
```rust
// Slow: Dynamic resizing causes audio glitches
pub struct DynamicAudioBuffer {
    buffer: Vec<f32>,
}

impl DynamicAudioBuffer {
    pub fn write(&mut self, samples: &[f32]) {
        self.buffer.extend_from_slice(samples);  // May reallocate
    }
    
    pub fn read(&mut self, count: usize) -> Vec<f32> {
        let result = self.buffer.drain(..count).collect();
        result
    }
}
```

## Implementation Checklist

- [ ] Audit SIMD operations for proper masking
- [ ] Replace generic trait parameters with trait objects where needed
- [ ] Optimize pixel rendering loops with hoisted checks
- [ ] Implement allocation-free SemVer comparison
- [ ] Use topological sort for dependency resolution
- [ ] Memory-map AI model weights
- [ ] Use pre-allocated ring buffers for audio
- [ ] Add performance benchmarks for critical paths

## References

- Original learnings from: .jules/bolt.md (2026-07-12 to 2026-07-14)
- SIMD Programming Guide
- Rust Performance Book
- Memory-Mapped Files Documentation
