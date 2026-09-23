// SigmaOS SCSI Sense Code & Descriptor Engine
// Inspired by Linux SCSI subsystem (`drivers/scsi/scsi_error.c`, `include/scsi/scsi_status.h`)
// and FreeBSD CAM (`sys/cam/scsi/scsi_all.h`, `sys/cam/cam_ccb.h`)
// Provides SCSI sense keys, ASC (Additional Sense Code), and ASCQ (Additional Sense Code Qualifier) decoding.

use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Standard SCSI Sense Keys (Fixed format byte 2 & Descriptor format byte 1)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScsiSenseKey {
    NoSense = 0x0,
    RecoveredError = 0x1,
    NotReady = 0x2,
    MediumError = 0x3,
    HardwareError = 0x4,
    IllegalRequest = 0x5,
    UnitAttention = 0x6,
    DataProtect = 0x7,
    BlankCheck = 0x8,
    VendorSpecific = 0x9,
    CopyAborted = 0xA,
    AbortedCommand = 0xB,
    VolumeOverflow = 0xD,
    Miscompare = 0xE,
    Completed = 0xF,
}

impl ScsiSenseKey {
    pub fn from_u8(val: u8) -> Self {
        match val & 0x0F {
            0x0 => ScsiSenseKey::NoSense,
            0x1 => ScsiSenseKey::RecoveredError,
            0x2 => ScsiSenseKey::NotReady,
            0x3 => ScsiSenseKey::MediumError,
            0x4 => ScsiSenseKey::HardwareError,
            0x5 => ScsiSenseKey::IllegalRequest,
            0x6 => ScsiSenseKey::UnitAttention,
            0x7 => ScsiSenseKey::DataProtect,
            0x8 => ScsiSenseKey::BlankCheck,
            0x9 => ScsiSenseKey::VendorSpecific,
            0xA => ScsiSenseKey::CopyAborted,
            0xB => ScsiSenseKey::AbortedCommand,
            0xD => ScsiSenseKey::VolumeOverflow,
            0xE => ScsiSenseKey::Miscompare,
            0xF => ScsiSenseKey::Completed,
            _ => ScsiSenseKey::NoSense,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ScsiSenseKey::NoSense => "NO SENSE",
            ScsiSenseKey::RecoveredError => "RECOVERED ERROR",
            ScsiSenseKey::NotReady => "NOT READY",
            ScsiSenseKey::MediumError => "MEDIUM ERROR",
            ScsiSenseKey::HardwareError => "HARDWARE ERROR",
            ScsiSenseKey::IllegalRequest => "ILLEGAL REQUEST",
            ScsiSenseKey::UnitAttention => "UNIT ATTENTION",
            ScsiSenseKey::DataProtect => "DATA PROTECT",
            ScsiSenseKey::BlankCheck => "BLANK CHECK",
            ScsiSenseKey::VendorSpecific => "VENDOR SPECIFIC",
            ScsiSenseKey::CopyAborted => "COPY ABORTED",
            ScsiSenseKey::AbortedCommand => "ABORTED COMMAND",
            ScsiSenseKey::VolumeOverflow => "VOLUME OVERFLOW",
            ScsiSenseKey::Miscompare => "MISCOMPARE",
            ScsiSenseKey::Completed => "COMPLETED",
        }
    }
}

/// Decodes SCSI ASC (Additional Sense Code) and ASCQ (Additional Sense Code Qualifier)
pub struct ScsiAscAscqDecoder;

impl ScsiAscAscqDecoder {
    pub fn decode(asc: u8, ascq: u8) -> String {
        match (asc, ascq) {
            (0x00, 0x00) => "No additional sense information".to_string(),
            (0x00, 0x01) => "Filemark detected".to_string(),
            (0x00, 0x02) => "End-of-partition/medium detected".to_string(),
            (0x04, 0x00) => "Logical unit not ready, cause not reportable".to_string(),
            (0x04, 0x01) => "Logical unit in process of becoming ready".to_string(),
            (0x04, 0x02) => "Logical unit not ready, initializing cmd. required".to_string(),
            (0x04, 0x03) => "Logical unit not ready, manual intervention required".to_string(),
            (0x08, 0x00) => "Logical unit communication failure".to_string(),
            (0x08, 0x01) => "Logical unit communication timeout".to_string(),
            (0x0C, 0x00) => "Write error".to_string(),
            (0x11, 0x00) => "Unrecovered read error".to_string(),
            (0x20, 0x00) => "Invalid command operation code".to_string(),
            (0x21, 0x00) => "Logical block address out of range".to_string(),
            (0x24, 0x00) => "Invalid field in CDB".to_string(),
            (0x27, 0x00) => "Write protected".to_string(),
            (0x28, 0x00) => "Not ready to ready change, medium may have changed".to_string(),
            (0x29, 0x00) => "Power on, reset, or bus device reset occurred".to_string(),
            (0x3A, 0x00) => "Medium not present".to_string(),
            _ => format!("ASC: {:#04X}, ASCQ: {:#04X}", asc, ascq),
        }
    }
}

/// Parsed SCSI Sense Data representation (supports both Fixed format & Descriptor format)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScsiSenseData {
    pub is_descriptor_format: bool,
    pub response_code: u8,
    pub sense_key: ScsiSenseKey,
    pub asc: u8,
    pub ascq: u8,
    pub info_valid: bool,
    pub information: u64,
}

impl ScsiSenseData {
    /// Builds a Fixed Format Sense Buffer (18 bytes standard)
    pub fn build_fixed_sense(
        sense_key: ScsiSenseKey,
        asc: u8,
        ascq: u8,
        info: Option<u32>,
    ) -> [u8; 18] {
        let mut buf = [0u8; 18];
        buf[0] = 0x70; // Current error, fixed format
        buf[2] = sense_key as u8;
        if let Some(val) = info {
            buf[0] |= 0x80; // Valid bit
            buf[3] = (val >> 24) as u8;
            buf[4] = (val >> 16) as u8;
            buf[5] = (val >> 8) as u8;
            buf[6] = val as u8;
        }
        buf[7] = 10; // Additional sense length (18 - 8 = 10 bytes)
        buf[12] = asc;
        buf[13] = ascq;
        buf
    }

    /// Parses sense buffer (Fixed format 0x70/0x71 or Descriptor format 0x72/0x73)
    pub fn parse_sense_buffer(buffer: &[u8]) -> Result<Self, &'static str> {
        if buffer.len() < 8 {
            return Err("Sense buffer too short");
        }

        let response_code = buffer[0] & 0x7F;
        match response_code {
            0x70 | 0x71 => {
                // Fixed format sense data
                if buffer.len() < 14 {
                    return Err("Fixed format sense buffer truncated");
                }
                let info_valid = (buffer[0] & 0x80) != 0;
                let sense_key = ScsiSenseKey::from_u8(buffer[2]);
                let information = ((buffer[3] as u64) << 24)
                    | ((buffer[4] as u64) << 16)
                    | ((buffer[5] as u64) << 8)
                    | (buffer[6] as u64);
                let asc = buffer[12];
                let ascq = buffer[13];

                Ok(Self {
                    is_descriptor_format: false,
                    response_code,
                    sense_key,
                    asc,
                    ascq,
                    info_valid,
                    information,
                })
            }
            0x72 | 0x73 => {
                // Descriptor format sense data
                let sense_key = ScsiSenseKey::from_u8(buffer[1]);
                let asc = buffer[2];
                let ascq = buffer[3];

                Ok(Self {
                    is_descriptor_format: true,
                    response_code,
                    sense_key,
                    asc,
                    ascq,
                    info_valid: false,
                    information: 0,
                })
            }
            _ => Err("Unsupported SCSI sense response code"),
        }
    }

    pub fn formatted_string(&self) -> String {
        format!(
            "[{}] Sense Key: {:?} ({}), {}",
            if self.is_descriptor_format {
                "Descriptor Format"
            } else {
                "Fixed Format"
            },
            self.sense_key,
            self.sense_key.description(),
            ScsiAscAscqDecoder::decode(self.asc, self.ascq)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scsi_sense_key_descriptions() {
        assert_eq!(
            ScsiSenseKey::from_u8(0x02).description(),
            "NOT READY"
        );
        assert_eq!(
            ScsiSenseKey::from_u8(0x05).description(),
            "ILLEGAL REQUEST"
        );
        assert_eq!(
            ScsiSenseKey::from_u8(0x06).description(),
            "UNIT ATTENTION"
        );
    }

    #[test]
    fn test_scsi_asc_ascq_decoding() {
        let desc_not_ready = ScsiAscAscqDecoder::decode(0x04, 0x01);
        assert_eq!(desc_not_ready, "Logical unit in process of becoming ready");

        let desc_lba_out_of_range = ScsiAscAscqDecoder::decode(0x21, 0x00);
        assert_eq!(desc_lba_out_of_range, "Logical block address out of range");
    }

    #[test]
    fn test_fixed_sense_building_and_parsing() {
        let fixed_buf = ScsiSenseData::build_fixed_sense(
            ScsiSenseKey::IllegalRequest,
            0x21, // LBA out of range
            0x00,
            Some(0x00001000),
        );

        let parsed = ScsiSenseData::parse_sense_buffer(&fixed_buf).unwrap();
        assert!(!parsed.is_descriptor_format);
        assert_eq!(parsed.sense_key, ScsiSenseKey::IllegalRequest);
        assert_eq!(parsed.asc, 0x21);
        assert_eq!(parsed.ascq, 0x00);
        assert!(parsed.info_valid);
        assert_eq!(parsed.information, 0x00001000);

        let fmt_str = parsed.formatted_string();
        assert!(fmt_str.contains("Fixed Format"));
        assert!(fmt_str.contains("ILLEGAL REQUEST"));
        assert!(fmt_str.contains("Logical block address out of range"));
    }
}
