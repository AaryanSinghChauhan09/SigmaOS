//! # SMB2/CIFS Network Filesystem Client
//!
//! Implements a client-side driver for the **SMB2** (Server Message Block version 2)
//! and **SMB3** protocols, enabling SigmaOS to mount Windows/Samba network shares.
//!
//! ## Protocol Highlights
//! - **Negotiate**: capability/dialect exchange, signing algorithm selection
//! - **Session Setup**: NTLM / Kerberos / anonymous authentication
//! - **Tree Connect**: map a UNC path (e.g. `\\server\share`) to a `TreeId`
//! - **Create / Read / Write / Close**: POSIX-like file I/O
//! - **Compound Requests**: multiple SMB2 commands in a single network round-trip
//!
//! Reference: [MS-SMB2] Open Specification, Microsoft Corporation.

#![allow(dead_code)]
#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// SMB2 protocol identifier in the packet header (4 bytes).
pub const SMB2_MAGIC: u32 = 0xFE534D42; // "\xFESMB"
/// Default SMB2 port.
pub const SMB2_PORT: u16 = 445;
/// Maximum SMB2 message size (negotiated; here a safe default).
pub const SMB2_MAX_TRANS_SIZE: u32 = 65536;

// ---------------------------------------------------------------------------
// SMB2 Header
// ---------------------------------------------------------------------------

/// SMB2 packet header — present on every SMB2 request and response.
///
/// Reference: §2.2.1 of [MS-SMB2].
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Smb2Header {
    /// Protocol identifier `0xFE534D42`.
    pub protocol_id: u32,
    /// Structure size: always 64.
    pub structure_size: u16,
    /// Credit charge (flow control units consumed by this request).
    pub credit_charge: u16,
    /// Status code (in responses) / channel sequence (in requests).
    pub status: u32,
    /// Command code (see [`Smb2Command`]).
    pub command: u16,
    /// Credits granted (responses) / credits requested (requests).
    pub credit_request_response: u16,
    /// Flags (see `SMB2_FLAGS_*`).
    pub flags: u32,
    /// Offset in bytes to the next command in a compound chain, or 0.
    pub next_command: u32,
    /// Message ID — a monotonically increasing 64-bit identifier.
    pub message_id: u64,
    /// (SMB3.1.1 multi-channel) Process ID.
    pub process_id: u32,
    /// Tree ID identifying the connected share (0 for session-level messages).
    pub tree_id: u32,
    /// Session ID assigned by the server.
    pub session_id: u64,
    /// Message authentication code (16 bytes of HMAC-SHA256 / AES-CMAC).
    pub signature: [u8; 16],
}

impl Smb2Header {
    /// Create an outgoing request header with default fields.
    pub fn new_request(command: Smb2Command, message_id: u64, session_id: u64, tree_id: u32) -> Self {
        Smb2Header {
            protocol_id: SMB2_MAGIC,
            structure_size: 64,
            credit_charge: 1,
            status: 0,
            command: command as u16,
            credit_request_response: 64,
            flags: 0,
            next_command: 0,
            message_id,
            process_id: 0xFEFF,
            tree_id,
            session_id,
            signature: [0u8; 16],
        }
    }

    /// Validate the protocol identifier of an incoming packet.
    pub fn is_valid(&self) -> bool {
        self.protocol_id == SMB2_MAGIC
    }
}

/// SMB2 command codes (§2.2.1.2 of [MS-SMB2]).
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Smb2Command {
    Negotiate = 0x0000,
    SessionSetup = 0x0001,
    Logoff = 0x0002,
    TreeConnect = 0x0003,
    TreeDisconnect = 0x0004,
    Create = 0x0005,
    Close = 0x0006,
    Flush = 0x0007,
    Read = 0x0008,
    Write = 0x0009,
    Lock = 0x000A,
    Ioctl = 0x000B,
    Cancel = 0x000C,
    KeepAlive = 0x000D,
    QueryDirectory = 0x000E,
    ChangeNotify = 0x000F,
    QueryInfo = 0x0010,
    SetInfo = 0x0011,
    OplockBreak = 0x0012,
}

// ---------------------------------------------------------------------------
// SMB2 NEGOTIATE
// ---------------------------------------------------------------------------

/// SMB2 dialect codes.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Smb2Dialect {
    Smb202 = 0x0202,
    Smb210 = 0x0210,
    Smb300 = 0x0300,
    Smb302 = 0x0302,
    Smb311 = 0x0311,
}

/// SMB2 NEGOTIATE request body (§2.2.3).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Smb2NegotiateRequest {
    /// Structure size: 36.
    pub structure_size: u16,
    /// Number of dialect codes in the `dialects` array.
    pub dialect_count: u16,
    /// Security mode flags.
    pub security_mode: u16,
    /// Reserved.
    pub reserved: u16,
    /// Client capabilities bitmask.
    pub capabilities: u32,
    /// Client GUID (16 bytes).
    pub client_guid: [u8; 16],
    /// Negotiate context list offset (SMB 3.1.1 only).
    pub negotiate_context_offset: u32,
    /// Negotiate context count.
    pub negotiate_context_count: u16,
    /// Reserved.
    pub reserved2: u16,
    // Followed by `dialect_count` u16 dialect codes.
}

/// SMB2 NEGOTIATE response body (§2.2.4).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Smb2NegotiateResponse {
    pub structure_size: u16,
    pub security_mode: u16,
    pub dialect_revision: u16,
    pub negotiate_context_count: u16,
    pub server_guid: [u8; 16],
    pub capabilities: u32,
    pub max_transact_size: u32,
    pub max_read_size: u32,
    pub max_write_size: u32,
    pub system_time: u64,
    pub server_start_time: u64,
    pub security_buffer_offset: u16,
    pub security_buffer_length: u16,
    pub negotiate_context_offset: u32,
}

// ---------------------------------------------------------------------------
// SMB2 SESSION SETUP
// ---------------------------------------------------------------------------

/// SMB2 SESSION_SETUP request body (§2.2.5).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Smb2SessionSetupRequest {
    pub structure_size: u16,
    pub flags: u8,
    pub security_mode: u8,
    pub capabilities: u32,
    pub channel: u32,
    pub security_buffer_offset: u16,
    pub security_buffer_length: u16,
    pub previous_session_id: u64,
}

/// SMB2 SESSION_SETUP response body (§2.2.6).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Smb2SessionSetupResponse {
    pub structure_size: u16,
    pub session_flags: u16,
    pub security_buffer_offset: u16,
    pub security_buffer_length: u16,
}

// ---------------------------------------------------------------------------
// SMB2 TREE CONNECT / DISCONNECT
// ---------------------------------------------------------------------------

/// SMB2 TREE_CONNECT request body (§2.2.9).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Smb2TreeConnectRequest {
    pub structure_size: u16,
    pub flags: u16,
    /// Byte offset of the UNC path string from the start of the SMB2 header.
    pub path_offset: u16,
    /// Length of the UNC path in bytes (Unicode).
    pub path_length: u16,
}

/// SMB2 TREE_CONNECT response body (§2.2.10).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Smb2TreeConnectResponse {
    pub structure_size: u16,
    /// Share type: 1 = disk, 2 = pipe, 3 = print.
    pub share_type: u8,
    pub reserved: u8,
    pub share_flags: u32,
    pub capabilities: u32,
    pub maximal_access: u32,
}

// ---------------------------------------------------------------------------
// SMB2 CREATE / CLOSE
// ---------------------------------------------------------------------------

/// SMB2 CREATE request body (§2.2.13).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Smb2CreateRequest {
    pub structure_size: u16,
    pub security_flags: u8,
    pub requested_oplock_level: u8,
    pub impersonation_level: u32,
    pub smb_create_flags: u64,
    pub reserved: u64,
    pub desired_access: u32,
    pub file_attributes: u32,
    pub share_access: u32,
    pub create_disposition: u32,
    pub create_options: u32,
    pub name_offset: u16,
    pub name_length: u16,
    pub create_contexts_offset: u32,
    pub create_contexts_length: u32,
}

/// File handle returned in a CREATE response.
#[derive(Debug, Clone, Copy)]
pub struct Smb2FileId {
    /// Persistent file ID component.
    pub persistent: u64,
    /// Volatile file ID component.
    pub volatile: u64,
}

// ---------------------------------------------------------------------------
// SMB2 READ / WRITE
// ---------------------------------------------------------------------------

/// SMB2 READ request body (§2.2.19).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Smb2ReadRequest {
    pub structure_size: u16,
    pub padding: u8,
    pub flags: u8,
    /// Number of bytes to read.
    pub length: u32,
    /// File offset to start reading from.
    pub offset: u64,
    /// Target file ID.
    pub file_id: [u8; 16],
    /// Minimum bytes the server must return.
    pub minimum_count: u32,
    pub channel: u32,
    pub remaining_bytes: u32,
    pub channel_info_offset: u16,
    pub channel_info_length: u16,
}

/// SMB2 WRITE request body (§2.2.21).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Smb2WriteRequest {
    pub structure_size: u16,
    pub data_offset: u16,
    pub length: u32,
    pub offset: u64,
    pub file_id: [u8; 16],
    pub channel: u32,
    pub remaining_bytes: u32,
    pub channel_info_offset: u16,
    pub channel_info_length: u16,
    pub flags: u32,
}

// ---------------------------------------------------------------------------
// Compound Request Builder
// ---------------------------------------------------------------------------

/// A builder for composing multiple SMB2 commands into a single compound request.
///
/// Compound requests reduce round-trips by piggybacking multiple operations
/// (e.g. CREATE + READ + CLOSE) in one TCP send.
pub struct CompoundRequestBuilder {
    /// Accumulated raw bytes of the compound packet.
    buffer: Vec<u8>,
    /// Byte offset of the most recently added command header.
    last_command_offset: usize,
    /// Monotonically increasing message ID for each command.
    message_id: u64,
}

impl CompoundRequestBuilder {
    /// Create a new compound builder starting at the given `message_id`.
    pub fn new(message_id: u64) -> Self {
        CompoundRequestBuilder {
            buffer: Vec::new(),
            last_command_offset: 0,
            message_id,
        }
    }

    /// Append a raw SMB2 command body to the compound chain.
    ///
    /// The builder patches the `NextCommand` field of the previous header to
    /// point to this new command.
    pub fn add_command(&mut self, session_id: u64, tree_id: u32, command: Smb2Command, body: &[u8]) {
        let header = Smb2Header::new_request(command, self.message_id, session_id, tree_id);
        self.message_id += 1;

        // Align to 8-byte boundary.
        let padding = (8 - (self.buffer.len() % 8)) % 8;
        self.buffer.extend(core::iter::repeat(0u8).take(padding));

        // Patch the previous command's NextCommand field.
        if !self.buffer.is_empty() {
            let next_cmd_offset = self.last_command_offset + 20; // offset of `next_command` in header
            let next_offset = (self.buffer.len() - self.last_command_offset) as u32;
            self.buffer[next_cmd_offset..next_cmd_offset + 4]
                .copy_from_slice(&next_offset.to_le_bytes());
        }

        self.last_command_offset = self.buffer.len();
        let hdr_bytes = unsafe {
            core::slice::from_raw_parts(
                &header as *const Smb2Header as *const u8,
                core::mem::size_of::<Smb2Header>(),
            )
        };
        self.buffer.extend_from_slice(hdr_bytes);
        self.buffer.extend_from_slice(body);
    }

    /// Finalise and return the compound packet bytes.
    pub fn finish(self) -> Vec<u8> {
        self.buffer
    }
}

// ---------------------------------------------------------------------------
// Session State
// ---------------------------------------------------------------------------

/// Authentication mechanism used for a session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmbAuthMethod {
    /// NT LAN Manager (NTLM) authentication.
    Ntlm,
    /// Kerberos via SPNEGO/GSS-API.
    Kerberos,
    /// Anonymous access (no authentication).
    Anonymous,
}

/// State of an SMB2 client session.
#[derive(Debug)]
pub struct Smb2Session {
    /// Server hostname or IP address.
    pub server: String,
    /// Negotiated dialect.
    pub dialect: Smb2Dialect,
    /// Session ID assigned by the server.
    pub session_id: u64,
    /// Authentication method in use.
    pub auth_method: SmbAuthMethod,
    /// Signing key (32 bytes for HMAC-SHA-256).
    pub signing_key: [u8; 32],
    /// Whether packet signing is required.
    pub signing_required: bool,
    /// Monotonically increasing message ID counter.
    pub next_message_id: u64,
}

impl Smb2Session {
    /// Begin a new unauthenticated session to `server`.
    pub fn new(server: String) -> Self {
        Smb2Session {
            server,
            dialect: Smb2Dialect::Smb311,
            session_id: 0,
            auth_method: SmbAuthMethod::Anonymous,
            signing_key: [0u8; 32],
            signing_required: false,
            next_message_id: 1,
        }
    }

    /// Allocate the next message ID.
    pub fn next_id(&mut self) -> u64 {
        let id = self.next_message_id;
        self.next_message_id += 1;
        id
    }
}

/// A connected tree (mounted share) within an SMB2 session.
#[derive(Debug)]
pub struct Smb2Tree {
    /// Back-reference to the session.
    pub session_id: u64,
    /// Tree ID assigned by the server.
    pub tree_id: u32,
    /// UNC path of the share (e.g. `\\server\share`).
    pub path: String,
    /// Share capabilities bitmask.
    pub capabilities: u32,
}

// ---------------------------------------------------------------------------
// Error Type
// ---------------------------------------------------------------------------

/// Errors produced by the SMB2 client.
#[derive(Debug, Clone)]
pub enum SmbError {
    /// The remote peer returned an NT status error code.
    NtStatus(u32),
    /// A packet had an unexpected or malformed header.
    InvalidPacket,
    /// The server and client could not agree on a common dialect.
    NegotiationFailed,
    /// Authentication failed.
    AuthenticationFailed,
    /// An I/O error occurred on the underlying transport.
    IoError,
    /// The operation timed out.
    Timeout,
}

impl fmt::Display for SmbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmbError::NtStatus(s) => write!(f, "NT status 0x{:08X}", s),
            SmbError::InvalidPacket => write!(f, "invalid SMB2 packet"),
            SmbError::NegotiationFailed => write!(f, "dialect negotiation failed"),
            SmbError::AuthenticationFailed => write!(f, "authentication failed"),
            SmbError::IoError => write!(f, "transport I/O error"),
            SmbError::Timeout => write!(f, "request timed out"),
        }
    }
}
