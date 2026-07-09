//! SigmaOS Common Types
//! Shared type definitions used across SigmaOS drivers and subsystems
//! Reduces code duplication and ensures type consistency

#![no_std]

/// SigmaOS unsigned 8-bit integer
pub type SigmaU8 = u8;

/// SigmaOS unsigned 16-bit integer
pub type SigmaU16 = u16;

/// SigmaOS unsigned 32-bit integer
pub type SigmaU32 = u32;

/// SigmaOS unsigned 64-bit integer
pub type SigmaU64 = u64;

/// SigmaOS signed 32-bit integer
pub type SigmaI32 = i32;

/// SigmaOS signed 64-bit integer
pub type SigmaI64 = i64;

/// SigmaOS 32-bit floating point
pub type SigmaF32 = f32;

/// SigmaOS 64-bit floating point
pub type SigmaF64 = f64;

/// SigmaOS boolean
pub type SigmaBool = bool;

/// SigmaOS usize
pub type SigmaUsize = usize;

/// SigmaOS isize
pub type SigmaIsize = isize;
