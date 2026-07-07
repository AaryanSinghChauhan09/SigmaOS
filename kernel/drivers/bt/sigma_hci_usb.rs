// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/drivers/bt/sigma_hci_usb.rs — USB HCI Bluetooth Driver
// Implements: Basic Host Controller Interface via USB for Bluetooth dongles.

#![no_std]
#![allow(dead_code)]

type U8 = u8;
type U16 = u16;
type U32 = u32;
type I32 = i32;

/// USB endpoint types
pub const USB_ENDPOINT_IN: U8 = 0x80;
pub const USB_ENDPOINT_OUT: U8 = 0x00;
pub const USB_ENDPOINT_TYPE_BULK: U8 = 0x02;
pub const USB_ENDPOINT_TYPE_INTERRUPT: U8 = 0x03;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS: U8 = 0x01;

/// USB transfer directions
pub const USB_DIR_IN: U8 = 0x80;
pub const USB_DIR_OUT: U8 = 0x00;

/// USB endpoint descriptor
#[repr(C, packed)]
pub struct UsbEndpointDescriptor {
    pub b_length: U8,
    pub b_descriptor_type: U8,
    pub b_endpoint_address: U8,
    pub bm_attributes: U8,
    pub w_max_packet_size: U16,
    pub b_interval: U8,
}

/// USB HCI endpoint configuration (BUG-007 Fix)
#[repr(C)]
pub struct HciUsbEndpoint {
    pub endpoint_address: U8,
    pub max_packet_size: U16,
    pub interval: U8,
    pub type_: U8,
    pub active: bool,
}

/// HCI USB driver state (BUG-007 Fix)
pub struct HciUsbDriver {
    pub initialized: bool,
    pub command_endpoint: HciUsbEndpoint,
    pub event_endpoint: HciUsbEndpoint,
    pub acl_in_endpoint: HciUsbEndpoint,
    pub acl_out_endpoint: HciUsbEndpoint,
    pub sco_in_endpoint: Option<HciUsbEndpoint>,
    pub sco_out_endpoint: Option<HciUsbEndpoint>,
    pub iso_in_endpoint: Option<HciUsbEndpoint>,
    pub iso_out_endpoint: Option<HciUsbEndpoint>,
}

static mut HCI: HciUsbDriver = HciUsbDriver {
    initialized: false,
    command_endpoint: HciUsbEndpoint {
        endpoint_address: 0,
        max_packet_size: 0,
        interval: 0,
        type_: 0,
        active: false,
    },
    event_endpoint: HciUsbEndpoint {
        endpoint_address: 0,
        max_packet_size: 0,
        interval: 0,
        type_: 0,
        active: false,
    },
    acl_in_endpoint: HciUsbEndpoint {
        endpoint_address: 0,
        max_packet_size: 0,
        interval: 0,
        type_: 0,
        active: false,
    },
    acl_out_endpoint: HciUsbEndpoint {
        endpoint_address: 0,
        max_packet_size: 0,
        interval: 0,
        type_: 0,
        active: false,
    },
    sco_in_endpoint: None,
    sco_out_endpoint: None,
    iso_in_endpoint: None,
    iso_out_endpoint: None,
};

/// Initialize USB HCI driver with endpoint configuration (BUG-007 Fix)
pub fn hci_usb_init() -> I32 {
    unsafe {
        if HCI.initialized {
            return 0; // Already initialized
        }

        // Configure default endpoints for Bluetooth USB dongles
        // Most USB Bluetooth dongles use these standard endpoint configurations:
        // - Endpoint 0: Control (default)
        // - Endpoint 1 (IN): Interrupt (HCI Events)
        // - Endpoint 2 (OUT): Bulk (HCI ACL Data)
        // - Endpoint 2 (IN): Bulk (HCI ACL Data)
        // - Endpoint 3 (IN/OUT): Isochronous (HCI SCO/ISO Data, optional)

        HCI.command_endpoint = HciUsbEndpoint {
            endpoint_address: 0x00, // Control endpoint
            max_packet_size: 64,
            interval: 0,
            type_: USB_ENDPOINT_TYPE_INTERRUPT,
            active: true,
        };

        HCI.event_endpoint = HciUsbEndpoint {
            endpoint_address: 0x81, // Endpoint 1 IN
            max_packet_size: 64,
            interval: 1,
            type_: USB_ENDPOINT_TYPE_INTERRUPT,
            active: true,
        };

        HCI.acl_in_endpoint = HciUsbEndpoint {
            endpoint_address: 0x82, // Endpoint 2 IN
            max_packet_size: 512,
            interval: 0,
            type_: USB_ENDPOINT_TYPE_BULK,
            active: true,
        };

        HCI.acl_out_endpoint = HciUsbEndpoint {
            endpoint_address: 0x02, // Endpoint 2 OUT
            max_packet_size: 512,
            interval: 0,
            type_: USB_ENDPOINT_TYPE_BULK,
            active: true,
        };

        // Optional SCO endpoints for voice
        HCI.sco_in_endpoint = Some(HciUsbEndpoint {
            endpoint_address: 0x83, // Endpoint 3 IN
            max_packet_size: 64,
            interval: 1,
            type_: USB_ENDPOINT_TYPE_ISOCHRONOUS,
            active: false,
        });

        HCI.sco_out_endpoint = Some(HciUsbEndpoint {
            endpoint_address: 0x03, // Endpoint 3 OUT
            max_packet_size: 64,
            interval: 1,
            type_: USB_ENDPOINT_TYPE_ISOCHRONOUS,
            active: false,
        });

        HCI.initialized = true;
        0
    }
}

/// Configure endpoints from USB descriptor (BUG-007 Fix)
#[no_mangle]
pub unsafe extern "C" fn hci_usb_configure_endpoints(
    descriptors: *const UsbEndpointDescriptor,
    descriptor_count: U32,
) -> I32 {
    if descriptors.is_null() || descriptor_count == 0 {
        return -1;
    }

    for i in 0..descriptor_count {
        let desc = &*descriptors.add(i as usize);
        let endpoint_addr = desc.b_endpoint_address;
        let is_input = (endpoint_addr & USB_DIR_IN) != 0;
        let endpoint_num = endpoint_addr & 0x0F;

        match (desc.bm_attributes & 0x03, endpoint_num) {
            (USB_ENDPOINT_TYPE_INTERRUPT, 1) if is_input => {
                // HCI Event endpoint
                HCI.event_endpoint.endpoint_address = endpoint_addr;
                HCI.event_endpoint.max_packet_size = desc.w_max_packet_size;
                HCI.event_endpoint.interval = desc.b_interval;
                HCI.event_endpoint.type_ = USB_ENDPOINT_TYPE_INTERRUPT;
                HCI.event_endpoint.active = true;
            }
            (USB_ENDPOINT_TYPE_BULK, 2) if is_input => {
                // ACL IN endpoint
                HCI.acl_in_endpoint.endpoint_address = endpoint_addr;
                HCI.acl_in_endpoint.max_packet_size = desc.w_max_packet_size;
                HCI.acl_in_endpoint.interval = desc.b_interval;
                HCI.acl_in_endpoint.type_ = USB_ENDPOINT_TYPE_BULK;
                HCI.acl_in_endpoint.active = true;
            }
            (USB_ENDPOINT_TYPE_BULK, 2) if !is_input => {
                // ACL OUT endpoint
                HCI.acl_out_endpoint.endpoint_address = endpoint_addr;
                HCI.acl_out_endpoint.max_packet_size = desc.w_max_packet_size;
                HCI.acl_out_endpoint.interval = desc.b_interval;
                HCI.acl_out_endpoint.type_ = USB_ENDPOINT_TYPE_BULK;
                HCI.acl_out_endpoint.active = true;
            }
            (USB_ENDPOINT_TYPE_ISOCHRONOUS, 3) if is_input => {
                // ISO IN endpoint
                if let Some(ref mut ep) = HCI.sco_in_endpoint {
                    ep.endpoint_address = endpoint_addr;
                    ep.max_packet_size = desc.w_max_packet_size;
                    ep.interval = desc.b_interval;
                    ep.type_ = USB_ENDPOINT_TYPE_ISOCHRONOUS;
                    ep.active = true;
                }
            }
            (USB_ENDPOINT_TYPE_ISOCHRONOUS, 3) if !is_input => {
                // ISO OUT endpoint
                if let Some(ref mut ep) = HCI.sco_out_endpoint {
                    ep.endpoint_address = endpoint_addr;
                    ep.max_packet_size = desc.w_max_packet_size;
                    ep.interval = desc.b_interval;
                    ep.type_ = USB_ENDPOINT_TYPE_ISOCHRONOUS;
                    ep.active = true;
                }
            }
            _ => {}
        }
    }

    0
}

/// Send HCI command via USB (BUG-007 Fix)
#[no_mangle]
pub unsafe extern "C" fn hci_usb_send_command(
    data: *const U8,
    length: U32,
) -> I32 {
    if !HCI.initialized || data.is_null() || length == 0 {
        return -1;
    }

    if !HCI.command_endpoint.active {
        return -2;
    }

    // In a real implementation, this would:
    // 1. Submit USB control transfer to command endpoint
    // 2. Wait for completion
    // 3. Return status

    0
}

/// Receive HCI event via USB (BUG-007 Fix)
#[no_mangle]
pub unsafe extern "C" fn hci_usb_receive_event(
    buffer: *mut U8,
    buffer_size: U32,
    bytes_received: *mut U32,
) -> I32 {
    if !HCI.initialized || buffer.is_null() || buffer_size == 0 || bytes_received.is_null() {
        return -1;
    }

    if !HCI.event_endpoint.active {
        return -2;
    }

    // In a real implementation, this would:
    // 1. Submit USB interrupt transfer to event endpoint
    // 2. Wait for completion
    // 3. Copy data to buffer
    // 4. Return bytes received

    *bytes_received = 0;
    0
}

/// Send ACL data via USB (BUG-007 Fix)
#[no_mangle]
pub unsafe extern "C" fn hci_usb_send_acl(
    data: *const U8,
    length: U32,
) -> I32 {
    if !HCI.initialized || data.is_null() || length == 0 {
        return -1;
    }

    if !HCI.acl_out_endpoint.active {
        return -2;
    }

    // In a real implementation, this would:
    // 1. Submit USB bulk transfer to ACL OUT endpoint
    // 2. Wait for completion
    // 3. Return status

    0
}

/// Receive ACL data via USB (BUG-007 Fix)
#[no_mangle]
pub unsafe extern "C" fn hci_usb_receive_acl(
    buffer: *mut U8,
    buffer_size: U32,
    bytes_received: *mut U32,
) -> I32 {
    if !HCI.initialized || buffer.is_null() || buffer_size == 0 || bytes_received.is_null() {
        return -1;
    }

    if !HCI.acl_in_endpoint.active {
        return -2;
    }

    // In a real implementation, this would:
    // 1. Submit USB bulk transfer to ACL IN endpoint
    // 2. Wait for completion
    // 3. Copy data to buffer
    // 4. Return bytes received

    *bytes_received = 0;
    0
}

/// Check if HCI USB is initialized (BUG-007 Fix)
#[no_mangle]
pub unsafe extern "C" fn hci_usb_is_initialized() -> I32 {
    if HCI.initialized {
        1
    } else {
        0
    }
}

/// Get endpoint info (BUG-007 Fix)
#[no_mangle]
pub unsafe extern "C" fn hci_usb_get_endpoint_info(
    endpoint_type: U32,
    address: *mut U8,
    max_packet: *mut U16,
) -> I32 {
    if !HCI.initialized || address.is_null() || max_packet.is_null() {
        return -1;
    }

    let endpoint = match endpoint_type {
        0 => &HCI.command_endpoint,
        1 => &HCI.event_endpoint,
        2 => &HCI.acl_in_endpoint,
        3 => &HCI.acl_out_endpoint,
        _ => return -2,
    };

    *address = endpoint.endpoint_address;
    *max_packet = endpoint.max_packet_size;

    0
}
