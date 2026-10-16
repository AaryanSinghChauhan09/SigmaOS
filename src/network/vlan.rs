#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//! # IEEE 802.1Q VLAN Sub-Interface Driver
//!
//! Implements 802.1Q VLAN tagging for SigmaOS network interfaces, providing:
//!
//! - **Tag insertion** (egress on trunk ports): prepend a 4-byte 802.1Q tag
//!   (TPID 0x8100, PCP 3 bits, DEI 1 bit, VID 12 bits) between the Ethernet
//!   source MAC and the EtherType field.
//! - **Tag stripping** (ingress on trunk ports): remove the 802.1Q tag and
//!   expose the inner EtherType and payload to upper-layer protocols.
//! - **Access-port mode**: frames are sent/received without any tag; the PVID
//!   is implicitly applied in software.
//! - **Trunk-port mode**: multiple VLAN IDs may be carried; frames are tagged
//!   on egress and classified by VID on ingress.
//! - **VLAN ID range**: 1–4094 (VID 0 is reserved for PCP-only, 4095 reserved).
//!
//! ## 802.1Q Tag Format (4 bytes)
//! ```text
//! Octet 0-1: TPID = 0x8100
//! Octet 2:   PCP[2:0] | DEI[0] | VID[11:8]
//! Octet 3:   VID[7:0]
//! ```

use std::collections::BTreeSet;
use std::string::{String, ToString};
use std::vec::Vec;

// ─── Constants ────────────────────────────────────────────────────────────────

/// IEEE 802.1Q Tag Protocol Identifier.
pub const TPID_8021Q: u16 = 0x8100;

/// IEEE 802.1ad (QinQ) outer tag TPID.
pub const TPID_8021AD: u16 = 0x88A8;

/// Minimum valid VLAN ID.
pub const VLAN_ID_MIN: u16 = 1;

/// Maximum valid VLAN ID (12-bit, 0xFFF reserved).
pub const VLAN_ID_MAX: u16 = 4094;

/// Default PVID for access ports (untagged native VLAN).
pub const VLAN_DEFAULT_PVID: u16 = 1;

// ─── Enumerations ─────────────────────────────────────────────────────────────

/// Switch-port operating mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortMode {
    /// Only a single VLAN; frames untagged on wire.
    Access,
    /// Multiple VLANs; frames tagged on wire for all non-native VLANs.
    Trunk,
    /// Hybrid: accepts both tagged and untagged frames.
    Hybrid,
}

/// Priority Code Point (PCP) values (3 bits, IEEE 802.1p).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Pcp {
    BestEffort = 0,
    Background = 1,
    ExcellentEffort = 2,
    CriticalApps = 3,
    Video = 4,
    Voice = 5,
    InternetworkControl = 6,
    NetworkControl = 7,
}

/// Errors from VLAN operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VlanError {
    /// VLAN ID out of range [1, 4094].
    InvalidVid,
    /// Frame too short to extract/insert a tag.
    FrameTooShort,
    /// TPID in the frame did not match 0x8100.
    NotTagged,
    /// VLAN not in the port's allowed set.
    VlanNotAllowed,
    /// Attempted to add a VLAN that already exists.
    DuplicateVlan,
}

// ─── 802.1Q Tag ───────────────────────────────────────────────────────────────

/// A decoded 802.1Q VLAN tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VlanTag {
    /// Priority Code Point (QoS).
    pub pcp: u8,
    /// Drop Eligible Indicator.
    pub dei: bool,
    /// VLAN Identifier (1–4094).
    pub vid: u16,
}

impl VlanTag {
    /// Construct a VLAN tag.
    pub fn new(vid: u16, pcp: u8, dei: bool) -> Result<Self, VlanError> {
        if vid < VLAN_ID_MIN || vid > VLAN_ID_MAX {
            return Err(VlanError::InvalidVid);
        }
        Ok(VlanTag {
            pcp: pcp & 0x07,
            dei,
            vid,
        })
    }

    /// Encode as two TCI bytes (after TPID).
    pub fn to_tci_bytes(&self) -> [u8; 2] {
        let hi = (self.pcp << 5) | ((self.dei as u8) << 4) | ((self.vid >> 8) as u8 & 0x0F);
        let lo = (self.vid & 0xFF) as u8;
        [hi, lo]
    }

    /// Decode from two TCI bytes.
    pub fn from_tci_bytes(bytes: [u8; 2]) -> Self {
        let pcp = (bytes[0] >> 5) & 0x07;
        let dei = (bytes[0] >> 4) & 0x01 != 0;
        let vid = (((bytes[0] & 0x0F) as u16) << 8) | bytes[1] as u16;
        VlanTag { pcp, dei, vid }
    }
}

// ─── Frame Tagging ────────────────────────────────────────────────────────────

/// Insert an 802.1Q tag into an Ethernet frame.
///
/// The tag is inserted **after** the 12-byte DA+SA fields:
/// `[DA(6)] [SA(6)] [TPID(2)] [TCI(2)] [EtherType(2)] [payload...]`
pub fn tag_insert(frame: &[u8], tag: VlanTag) -> Result<Vec<u8>, VlanError> {
    if frame.len() < 14 {
        return Err(VlanError::FrameTooShort);
    }
    let tci = tag.to_tci_bytes();
    let tpid = TPID_8021Q.to_be_bytes();
    let mut out = Vec::with_capacity(frame.len() + 4);
    out.extend_from_slice(&frame[..12]); // DA + SA
    out.extend_from_slice(&tpid); // TPID 0x8100
    out.extend_from_slice(&tci); // PCP | DEI | VID
    out.extend_from_slice(&frame[12..]); // EtherType + payload
    Ok(out)
}

/// Strip an 802.1Q tag from a tagged Ethernet frame.
///
/// Returns `(inner_frame, tag)`.
pub fn tag_strip(frame: &[u8]) -> Result<(Vec<u8>, VlanTag), VlanError> {
    if frame.len() < 18 {
        return Err(VlanError::FrameTooShort);
    }
    let tpid = u16::from_be_bytes([frame[12], frame[13]]);
    if tpid != TPID_8021Q && tpid != TPID_8021AD {
        return Err(VlanError::NotTagged);
    }
    let tag = VlanTag::from_tci_bytes([frame[14], frame[15]]);
    let mut out = Vec::with_capacity(frame.len() - 4);
    out.extend_from_slice(&frame[..12]); // DA + SA
    out.extend_from_slice(&frame[16..]); // EtherType + payload (tag removed)
    Ok((out, tag))
}

/// Classify an ingress frame: if tagged return VID, else return port's PVID.
pub fn classify_ingress(frame: &[u8], pvid: u16) -> u16 {
    if frame.len() >= 14 {
        let tpid = u16::from_be_bytes([frame[12], frame[13]]);
        if tpid == TPID_8021Q || tpid == TPID_8021AD {
            let vid_hi = (frame[14] & 0x0F) as u16;
            let vid_lo = frame[15] as u16;
            let vid = (vid_hi << 8) | vid_lo;
            if vid >= VLAN_ID_MIN && vid <= VLAN_ID_MAX {
                return vid;
            }
        }
    }
    pvid
}

// ─── VLAN Port ────────────────────────────────────────────────────────────────

/// Represents a VLAN-aware switch port.
#[derive(Debug)]
pub struct VlanPort {
    /// Port name (e.g. "eth0").
    pub name: String,
    /// Port operating mode.
    pub mode: PortMode,
    /// Port VLAN ID (PVID) — native untagged VLAN.
    pub pvid: u16,
    /// Set of allowed VLANs (trunk mode).
    pub allowed_vlans: BTreeSet<u16>,
    /// Statistics.
    pub rx_tagged: u64,
    pub rx_untagged: u64,
    pub tx_tagged: u64,
    pub tx_untagged: u64,
    pub rx_dropped: u64,
}

impl VlanPort {
    /// Create a new access port with the given PVID.
    pub fn new_access(name: &str, pvid: u16) -> Result<Self, VlanError> {
        if pvid < VLAN_ID_MIN || pvid > VLAN_ID_MAX {
            return Err(VlanError::InvalidVid);
        }
        let mut allowed = BTreeSet::new();
        allowed.insert(pvid);
        Ok(VlanPort {
            name: name.to_string(),
            mode: PortMode::Access,
            pvid,
            allowed_vlans: allowed,
            rx_tagged: 0,
            rx_untagged: 0,
            tx_tagged: 0,
            tx_untagged: 0,
            rx_dropped: 0,
        })
    }

    /// Create a new trunk port with a native PVID and an allowed VLAN list.
    pub fn new_trunk(name: &str, pvid: u16, vlans: &[u16]) -> Result<Self, VlanError> {
        if pvid < VLAN_ID_MIN || pvid > VLAN_ID_MAX {
            return Err(VlanError::InvalidVid);
        }
        let mut allowed = BTreeSet::new();
        for &v in vlans {
            if v < VLAN_ID_MIN || v > VLAN_ID_MAX {
                return Err(VlanError::InvalidVid);
            }
            allowed.insert(v);
        }
        allowed.insert(pvid);
        Ok(VlanPort {
            name: name.to_string(),
            mode: PortMode::Trunk,
            pvid,
            allowed_vlans: allowed,
            rx_tagged: 0,
            rx_untagged: 0,
            tx_tagged: 0,
            tx_untagged: 0,
            rx_dropped: 0,
        })
    }

    /// Add a VLAN to the trunk allowed list.
    pub fn allow_vlan(&mut self, vid: u16) -> Result<(), VlanError> {
        if vid < VLAN_ID_MIN || vid > VLAN_ID_MAX {
            return Err(VlanError::InvalidVid);
        }
        self.allowed_vlans.insert(vid);
        Ok(())
    }

    /// Remove a VLAN from the allowed list.
    pub fn deny_vlan(&mut self, vid: u16) {
        self.allowed_vlans.remove(&vid);
    }

    /// Process an ingress frame: classify, validate, strip tag if present.
    /// Returns the inner frame and the resolved VID.
    pub fn ingress(&mut self, frame: &[u8]) -> Result<(Vec<u8>, u16), VlanError> {
        if frame.len() < 14 {
            self.rx_dropped += 1;
            return Err(VlanError::FrameTooShort);
        }
        let tpid = u16::from_be_bytes([frame[12], frame[13]]);
        let is_tagged = tpid == TPID_8021Q || tpid == TPID_8021AD;

        let (inner, vid) = if is_tagged {
            self.rx_tagged += 1;
            let (inner, tag) = tag_strip(frame)?;
            (inner, tag.vid)
        } else {
            self.rx_untagged += 1;
            (frame.to_vec(), self.pvid)
        };

        if !self.allowed_vlans.contains(&vid) {
            self.rx_dropped += 1;
            return Err(VlanError::VlanNotAllowed);
        }
        Ok((inner, vid))
    }

    /// Process an egress frame for the given VID.
    /// Access ports always send untagged; trunk ports tag all non-native VLANs.
    pub fn egress(&mut self, frame: &[u8], vid: u16) -> Result<Vec<u8>, VlanError> {
        if !self.allowed_vlans.contains(&vid) {
            return Err(VlanError::VlanNotAllowed);
        }
        match self.mode {
            PortMode::Access => {
                self.tx_untagged += 1;
                Ok(frame.to_vec())
            }
            PortMode::Trunk | PortMode::Hybrid => {
                if vid == self.pvid {
                    self.tx_untagged += 1;
                    Ok(frame.to_vec())
                } else {
                    self.tx_tagged += 1;
                    tag_insert(frame, VlanTag::new(vid, 0, false)?)
                }
            }
        }
    }
}
