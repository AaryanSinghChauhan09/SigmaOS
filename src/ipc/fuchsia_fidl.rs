// SigmaOS Fuchsia FIDL (IPC Interface Definition Language) Wire Protocol & Compiler
// Reference: Fuchsia SDK, FIDL Wire Format Specification (fuchsia.dev)

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

pub const FIDL_WIRE_FORMAT_V2_MAGIC: u8 = 0x01;

/// Standard 16-byte FIDL Message Header
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FidlMsgHeader {
    pub txid: u32,
    pub at_rest_flags: [u8; 2],
    pub dynamic_flags: u8,
    pub magic_number: u8,
    pub ordinal: u64, // Method ordinal (SHA-256 hash of interface.method name)
}

impl FidlMsgHeader {
    pub fn new(txid: u32, ordinal: u64) -> Self {
        Self {
            txid,
            at_rest_flags: [0, 0],
            dynamic_flags: 0,
            magic_number: FIDL_WIRE_FORMAT_V2_MAGIC,
            ordinal,
        }
    }

    pub fn to_bytes(&self) -> [u8; 16] {
        let mut buf = [0u8; 16];
        buf[0..4].copy_from_slice(&self.txid.to_le_bytes());
        buf[4..6].copy_from_slice(&self.at_rest_flags);
        buf[6] = self.dynamic_flags;
        buf[7] = self.magic_number;
        buf[8..16].copy_from_slice(&self.ordinal.to_le_bytes());
        buf
    }

    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < 16 {
            return Err("FIDL buffer too short for header");
        }
        if bytes[7] != FIDL_WIRE_FORMAT_V2_MAGIC {
            return Err("Invalid FIDL magic byte");
        }
        Ok(Self {
            txid: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            at_rest_flags: [bytes[4], bytes[5]],
            dynamic_flags: bytes[6],
            magic_number: bytes[7],
            ordinal: u64::from_le_bytes(bytes[8..16].try_into().unwrap()),
        })
    }
}

/// FIDL Type Representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FidlType {
    Bool,
    Int32,
    Int64,
    Uint32,
    Uint64,
    Float32,
    Float64,
    StringVal,
    Handle,
}

/// FIDL Method Definition
#[derive(Debug, Clone)]
pub struct FidlMethod {
    pub name: String,
    pub ordinal: u64,
    pub request_types: Vec<FidlType>,
    pub response_types: Vec<FidlType>,
}

/// FIDL Protocol / Interface Definition
#[derive(Debug, Clone)]
pub struct FidlProtocol {
    pub name: String,
    pub methods: BTreeMap<u64, FidlMethod>,
}

impl FidlProtocol {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            methods: BTreeMap::new(),
        }
    }

    pub fn add_method(&mut self, method: FidlMethod) {
        self.methods.insert(method.ordinal, method);
    }

    pub fn find_method(&self, ordinal: u64) -> Option<&FidlMethod> {
        self.methods.get(&ordinal)
    }
}

/// FIDL Dispatcher
pub struct FidlDispatcher {
    protocols: BTreeMap<String, FidlProtocol>,
}

impl FidlDispatcher {
    pub fn new() -> Self {
        Self {
            protocols: BTreeMap::new(),
        }
    }

    pub fn register_protocol(&mut self, protocol: FidlProtocol) {
        self.protocols.insert(protocol.name.clone(), protocol);
    }

    /// Dispatch incoming FIDL frame
    pub fn dispatch(&self, frame: &[u8]) -> Result<u64, &'static str> {
        let hdr = FidlMsgHeader::parse(frame)?;
        for proto in self.protocols.values() {
            if let Some(method) = proto.find_method(hdr.ordinal) {
                return Ok(method.ordinal);
            }
        }
        Err("No matching FIDL protocol method found for ordinal")
    }
}
