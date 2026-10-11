// SigmaOS USB / Bluetooth HID Report Descriptor Parser
// Compliant with USB Device Class Definition for Human Interface Devices (HID) v1.11.

#![allow(dead_code)]
extern crate alloc;

use alloc::vec::Vec;

/// Item Types defined by USB HID specification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HidItemType {
    Main,
    Global,
    Local,
    Reserved,
}

/// Main item tags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainItemTag {
    Input = 0b1000,
    Output = 0b1001,
    Collection = 0b1010,
    Feature = 0b1011,
    EndCollection = 0b1100,
}

/// Global item tags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalItemTag {
    UsagePage = 0,
    LogicalMinimum = 1,
    LogicalMaximum = 2,
    PhysicalMinimum = 3,
    PhysicalMaximum = 4,
    UnitExponent = 5,
    Unit = 6,
    ReportSize = 7,
    ReportId = 8,
    ReportCount = 9,
    Push = 10,
    Pop = 11,
}

/// Local item tags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalItemTag {
    Usage = 0,
    UsageMinimum = 1,
    UsageMaximum = 2,
    DesignatorIndex = 3,
    DesignatorMinimum = 4,
    DesignatorMaximum = 5,
    StringIndex = 7,
    StringMinimum = 8,
    StringMaximum = 9,
    Delimiter = 10,
}

/// Standard HID Usage Pages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsagePage {
    Undefined,
    GenericDesktop,
    Simulation,
    VrControls,
    SportControls,
    GameControls,
    GenericDevice,
    Keyboard,
    Led,
    Button,
    Ordinal,
    Telephony,
    Consumer,
    Digitizer,
    Haptics,
    PhysicalInput,
    Unicode,
    VendorDefined(u16),
}

impl UsagePage {
    pub fn from_u16(val: u16) -> Self {
        match val {
            0x00 => UsagePage::Undefined,
            0x01 => UsagePage::GenericDesktop,
            0x02 => UsagePage::Simulation,
            0x03 => UsagePage::VrControls,
            0x04 => UsagePage::SportControls,
            0x05 => UsagePage::GameControls,
            0x06 => UsagePage::GenericDevice,
            0x07 => UsagePage::Keyboard,
            0x08 => UsagePage::Led,
            0x09 => UsagePage::Button,
            0x0A => UsagePage::Ordinal,
            0x0B => UsagePage::Telephony,
            0x0C => UsagePage::Consumer,
            0x0D => UsagePage::Digitizer,
            0x0E => UsagePage::Haptics,
            0x0F => UsagePage::PhysicalInput,
            0x10 => UsagePage::Unicode,
            other => UsagePage::VendorDefined(other),
        }
    }
}

/// Collection type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionType {
    Physical,
    Application,
    Logical,
    Report,
    NamedArray,
    UsageSwitch,
    UsageModifier,
    Vendor(u32),
}

impl CollectionType {
    pub fn from_u32(val: u32) -> Self {
        match val {
            0x00 => CollectionType::Physical,
            0x01 => CollectionType::Application,
            0x02 => CollectionType::Logical,
            0x03 => CollectionType::Report,
            0x04 => CollectionType::NamedArray,
            0x05 => CollectionType::UsageSwitch,
            0x06 => CollectionType::UsageModifier,
            other => CollectionType::Vendor(other),
        }
    }
}

/// Represents an individual field in a parsed HID report
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HidReportField {
    pub report_id: u8,
    pub usage_page: UsagePage,
    pub usages: Vec<u32>,
    pub logical_min: i32,
    pub logical_max: i32,
    pub report_size_bits: u32,
    pub report_count: u32,
    pub is_variable: bool,
    pub is_relative: bool,
    pub is_array: bool,
}

/// State of global items during descriptor parsing
#[derive(Debug, Clone, Default)]
struct GlobalState {
    usage_page: u16,
    logical_min: i32,
    logical_max: i32,
    physical_min: i32,
    physical_max: i32,
    report_size: u32,
    report_id: u8,
    report_count: u32,
}

/// HID Descriptor Parser
pub struct HidParser {
    raw_descriptor: Vec<u8>,
}

impl HidParser {
    pub fn new(descriptor: &[u8]) -> Self {
        Self {
            raw_descriptor: descriptor.to_vec(),
        }
    }

    /// Parse report descriptor into input, output, and feature fields
    pub fn parse(&self) -> (Vec<HidReportField>, Vec<HidReportField>, Vec<HidReportField>) {
        let mut input_fields = Vec::new();
        let mut output_fields = Vec::new();
        let mut feature_fields = Vec::new();

        let mut current_global = GlobalState::default();
        let mut global_stack: Vec<GlobalState> = Vec::new();
        let mut local_usages: Vec<u32> = Vec::new();

        let mut i = 0;
        let bytes = &self.raw_descriptor;

        while i < bytes.len() {
            let b0 = bytes[i];
            i += 1;

            if b0 == 0xFE {
                // Long item: b0=0xFE, b1=data size, b2=tag, data...
                if i + 1 < bytes.len() {
                    let data_size = bytes[i] as usize;
                    i += 2 + data_size;
                }
                continue;
            }

            // Short item: bSize = b0 & 0x03, bType = (b0 >> 2) & 0x03, bTag = (b0 >> 4) & 0x0F
            let raw_size = b0 & 0x03;
            let item_size = match raw_size {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 4,
                _ => 0,
            };

            let item_type = (b0 >> 2) & 0x03;
            let item_tag = (b0 >> 4) & 0x0F;

            let mut data: u32 = 0;
            for byte_idx in 0..item_size {
                if i + byte_idx < bytes.len() {
                    data |= (bytes[i + byte_idx] as u32) << (byte_idx * 8);
                }
            }
            i += item_size;

            let signed_data: i32 = match item_size {
                1 => (data as u8) as i8 as i32,
                2 => (data as u16) as i16 as i32,
                4 => data as i32,
                _ => 0,
            };

            match item_type {
                0 => {
                    // Main item
                    match item_tag {
                        0b1000 => {
                            // Input
                            let is_variable = (data & 0x02) != 0;
                            let is_relative = (data & 0x04) != 0;
                            let is_array = (data & 0x02) == 0;
                            input_fields.push(HidReportField {
                                report_id: current_global.report_id,
                                usage_page: UsagePage::from_u16(current_global.usage_page),
                                usages: local_usages.clone(),
                                logical_min: current_global.logical_min,
                                logical_max: current_global.logical_max,
                                report_size_bits: current_global.report_size,
                                report_count: current_global.report_count,
                                is_variable,
                                is_relative,
                                is_array,
                            });
                            local_usages.clear();
                        }
                        0b1001 => {
                            // Output
                            let is_variable = (data & 0x02) != 0;
                            let is_relative = (data & 0x04) != 0;
                            let is_array = (data & 0x02) == 0;
                            output_fields.push(HidReportField {
                                report_id: current_global.report_id,
                                usage_page: UsagePage::from_u16(current_global.usage_page),
                                usages: local_usages.clone(),
                                logical_min: current_global.logical_min,
                                logical_max: current_global.logical_max,
                                report_size_bits: current_global.report_size,
                                report_count: current_global.report_count,
                                is_variable,
                                is_relative,
                                is_array,
                            });
                            local_usages.clear();
                        }
                        0b1011 => {
                            // Feature
                            let is_variable = (data & 0x02) != 0;
                            let is_relative = (data & 0x04) != 0;
                            let is_array = (data & 0x02) == 0;
                            feature_fields.push(HidReportField {
                                report_id: current_global.report_id,
                                usage_page: UsagePage::from_u16(current_global.usage_page),
                                usages: local_usages.clone(),
                                logical_min: current_global.logical_min,
                                logical_max: current_global.logical_max,
                                report_size_bits: current_global.report_size,
                                report_count: current_global.report_count,
                                is_variable,
                                is_relative,
                                is_array,
                            });
                            local_usages.clear();
                        }
                        0b1010 => {
                            // Collection
                            local_usages.clear();
                        }
                        0b1100 => {
                            // End Collection
                        }
                        _ => {}
                    }
                }
                1 => {
                    // Global item
                    match item_tag {
                        0 => current_global.usage_page = data as u16,
                        1 => current_global.logical_min = signed_data,
                        2 => current_global.logical_max = signed_data,
                        3 => current_global.physical_min = signed_data,
                        4 => current_global.physical_max = signed_data,
                        7 => current_global.report_size = data,
                        8 => current_global.report_id = data as u8,
                        9 => current_global.report_count = data,
                        10 => global_stack.push(current_global.clone()),
                        11 => {
                            if let Some(prev) = global_stack.pop() {
                                current_global = prev;
                            }
                        }
                        _ => {}
                    }
                }
                2 => {
                    // Local item
                    match item_tag {
                        0 => local_usages.push(data),
                        1 => {
                            // Usage Minimum (stored in temporary if needed)
                            local_usages.push(data);
                        }
                        2 => {
                            // Usage Maximum
                            local_usages.push(data);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        (input_fields, output_fields, feature_fields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hid_mouse_descriptor_parse() {
        // Standard 3-button USB mouse descriptor snippet
        let mouse_desc = [
            0x05, 0x01, // Usage Page (Generic Desktop)
            0x09, 0x02, // Usage (Mouse)
            0xA1, 0x01, // Collection (Application)
            0x05, 0x09, //   Usage Page (Button)
            0x19, 0x01, //   Usage Minimum (Button 1)
            0x29, 0x03, //   Usage Maximum (Button 3)
            0x15, 0x00, //   Logical Minimum (0)
            0x25, 0x01, //   Logical Maximum (1)
            0x95, 0x03, //   Report Count (3)
            0x75, 0x01, //   Report Size (1)
            0x81, 0x02, //   Input (Data, Var, Abs)
            0xC0,       // End Collection
        ];

        let parser = HidParser::new(&mouse_desc);
        let (inputs, outputs, features) = parser.parse();

        assert_eq!(inputs.len(), 1);
        assert_eq!(outputs.len(), 0);
        assert_eq!(features.len(), 0);

        let button_input = &inputs[0];
        assert_eq!(button_input.usage_page, UsagePage::Button);
        assert_eq!(button_input.report_size_bits, 1);
        assert_eq!(button_input.report_count, 3);
        assert!(button_input.is_variable);
    }
}
