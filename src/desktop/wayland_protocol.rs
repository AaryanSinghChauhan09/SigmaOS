#![allow(unused_imports)]
// SigmaOS Wayland Protocol Engine (Zero-Dependency Bare-Metal Display Protocol Engine)
// Implements core Wayland wire encoding/decoding, xdg_shell surface lifecycle,
// wl_seat input event dispatching, and wl_data_device clipboard negotiations.

#[cfg(not(any(feature = "standalone_test", test)))]


#[cfg(not(any(feature = "standalone_test", test)))]
use std::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use std::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use std::vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use std::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::format;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec;
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

/// Wayland Wire Protocol Message Header (8 bytes)
/// - object_id: 32-bit sender/receiver object ID
/// - opcode: 16-bit message/event opcode
/// - message_len: 16-bit total byte length of message (header + arguments)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaylandMessageWireHeader {
    pub object_id: u32,
    pub opcode: u16,
    pub message_len: u16,
}

impl WaylandMessageWireHeader {
    pub fn encode(&self) -> [u8; 8] {
        let mut buf = [0u8; 8];
        let obj_bytes = self.object_id.to_le_bytes();
        buf[0..4].copy_from_slice(&obj_bytes);

        // Opcode (lower 16 bits) and length (upper 16 bits) in 32-bit word
        let op_len = (self.opcode as u32) | ((self.message_len as u32) << 16);
        buf[4..8].copy_from_slice(&op_len.to_le_bytes());
        buf
    }

    pub fn decode(bytes: &[u8; 8]) -> Self {
        let object_id = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let op_len = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let opcode = (op_len & 0xFFFF) as u16;
        let message_len = ((op_len >> 16) & 0xFFFF) as u16;

        Self {
            object_id,
            opcode,
            message_len,
        }
    }
}

/// Wayland Surface (`wl_surface`) representation
#[derive(Debug, Clone)]
pub struct WlSurface {
    pub surface_id: u32,
    pub width: u32,
    pub height: u32,
    pub buffer_id: Option<u32>,
    pub damage_x: i32,
    pub damage_y: i32,
    pub damage_w: u32,
    pub damage_h: u32,
    pub is_committed: bool,
    pub scale_factor: i32,
}

impl WlSurface {
    pub fn new(surface_id: u32) -> Self {
        Self {
            surface_id,
            width: 0,
            height: 0,
            buffer_id: None,
            damage_x: 0,
            damage_y: 0,
            damage_w: 0,
            damage_h: 0,
            is_committed: false,
            scale_factor: 1,
        }
    }

    pub fn attach_buffer(&mut self, buffer_id: u32, width: u32, height: u32) {
        self.buffer_id = Some(buffer_id);
        self.width = width;
        self.height = height;
        self.is_committed = false;
    }

    pub fn damage(&mut self, x: i32, y: i32, w: u32, h: u32) {
        self.damage_x = x;
        self.damage_y = y;
        self.damage_w = w;
        self.damage_h = h;
        self.is_committed = false;
    }

    pub fn commit(&mut self) -> Result<(), &'static str> {
        if self.buffer_id.is_none() {
            return Err("wl_surface: cannot commit without attached buffer");
        }
        self.is_committed = true;
        Ok(())
    }
}

/// XDG Toplevel (`xdg_toplevel`) surface decoration and geometry
#[derive(Debug, Clone)]
pub struct XdgToplevel {
    pub toplevel_id: u32,
    pub title: String,
    pub app_id: String,
    pub max_width: u32,
    pub max_height: u32,
    pub min_width: u32,
    pub min_height: u32,
    pub is_maximized: bool,
    pub is_fullscreen: bool,
    pub is_activated: bool,
}

impl XdgToplevel {
    pub fn new(toplevel_id: u32, title: &str, app_id: &str) -> Self {
        Self {
            toplevel_id,
            title: title.to_string(),
            app_id: app_id.to_string(),
            max_width: 0,
            max_height: 0,
            min_width: 0,
            min_height: 0,
            is_maximized: false,
            is_fullscreen: false,
            is_activated: true,
        }
    }
}

/// XDG Surface (`xdg_surface`) container bridging `wl_surface` and `xdg_toplevel`
#[derive(Debug, Clone)]
pub struct XdgSurface {
    pub xdg_surface_id: u32,
    pub wl_surface_id: u32,
    pub toplevel: Option<XdgToplevel>,
    pub configure_serial: u32,
    pub acked_serial: u32,
}

impl XdgSurface {
    pub fn new(xdg_surface_id: u32, wl_surface_id: u32) -> Self {
        Self {
            xdg_surface_id,
            wl_surface_id,
            toplevel: None,
            configure_serial: 0,
            acked_serial: 0,
        }
    }
}

/// Wayland Seat (`wl_seat`) Input Capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlSeatCapability {
    Pointer,
    Keyboard,
    Touch,
}

/// Wayland Seat (`wl_seat`) Input Hub
#[derive(Debug, Clone)]
pub struct WlSeat {
    pub seat_id: u32,
    pub name: String,
    pub capabilities: Vec<WlSeatCapability>,
    pub pointer_x: f64,
    pub pointer_y: f64,
    pub focused_surface_id: Option<u32>,
    pub pressed_keys: Vec<u32>,
}

impl WlSeat {
    pub fn new(seat_id: u32, name: &str) -> Self {
        Self {
            seat_id,
            name: name.to_string(),
            capabilities: vec![WlSeatCapability::Pointer, WlSeatCapability::Keyboard],
            pointer_x: 0.0,
            pointer_y: 0.0,
            focused_surface_id: None,
            pressed_keys: Vec::new(),
        }
    }
}

/// Wayland Data Offer (`wl_data_offer`) MIME offer for clipboard selection
#[derive(Debug, Clone)]
pub struct WlDataOffer {
    pub offer_id: u32,
    pub mime_types: Vec<String>,
}

/// Wayland Data Device (`wl_data_device`) manager for clipboard and drag-and-drop
#[derive(Debug, Clone)]
pub struct WlDataDevice {
    pub device_id: u32,
    pub seat_id: u32,
    pub offers: Vec<WlDataOffer>,
    pub active_selection: Option<WlDataOffer>,
}

impl WlDataDevice {
    pub fn new(device_id: u32, seat_id: u32) -> Self {
        Self {
            device_id,
            seat_id,
            offers: Vec::new(),
            active_selection: None,
        }
    }
}

/// Sovereign Wayland Protocol Server Engine
pub struct WaylandProtocolEngine {
    pub surfaces: Vec<WlSurface>,
    pub xdg_surfaces: Vec<XdgSurface>,
    pub seats: Vec<WlSeat>,
    pub data_devices: Vec<WlDataDevice>,
    pub serial_counter: u32,
}

impl WaylandProtocolEngine {
    pub fn new() -> Self {
        Self {
            surfaces: Vec::new(),
            xdg_surfaces: Vec::new(),
            seats: Vec::new(),
            data_devices: Vec::new(),
            serial_counter: 1,
        }
    }

    pub fn next_serial(&mut self) -> u32 {
        let serial = self.serial_counter;
        self.serial_counter = self.serial_counter.wrapping_add(1);
        serial
    }

    pub fn create_surface(&mut self, surface_id: u32) -> WlSurface {
        let surface = WlSurface::new(surface_id);
        self.surfaces.push(surface.clone());
        surface
    }

    pub fn get_surface_mut(&mut self, surface_id: u32) -> Option<&mut WlSurface> {
        self.surfaces.iter_mut().find(|s| s.surface_id == surface_id)
    }

    pub fn attach_buffer(
        &mut self,
        surface_id: u32,
        buffer_id: u32,
        width: u32,
        height: u32,
    ) -> Result<(), &'static str> {
        let surface = self
            .get_surface_mut(surface_id)
            .ok_or("WaylandProtocolEngine: Surface not found")?;
        surface.attach_buffer(buffer_id, width, height);
        Ok(())
    }

    pub fn damage_surface(
        &mut self,
        surface_id: u32,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
    ) -> Result<(), &'static str> {
        let surface = self
            .get_surface_mut(surface_id)
            .ok_or("WaylandProtocolEngine: Surface not found")?;
        surface.damage(x, y, w, h);
        Ok(())
    }

    pub fn commit_surface(&mut self, surface_id: u32) -> Result<(), &'static str> {
        let surface = self
            .get_surface_mut(surface_id)
            .ok_or("WaylandProtocolEngine: Surface not found")?;
        surface.commit()
    }

    pub fn create_xdg_surface(
        &mut self,
        xdg_surface_id: u32,
        wl_surface_id: u32,
    ) -> Result<(), &'static str> {
        if !self.surfaces.iter().any(|s| s.surface_id == wl_surface_id) {
            return Err("WaylandProtocolEngine: Referenced wl_surface does not exist");
        }
        let xdg_surface = XdgSurface::new(xdg_surface_id, wl_surface_id);
        self.xdg_surfaces.push(xdg_surface);
        Ok(())
    }

    pub fn set_xdg_toplevel(
        &mut self,
        xdg_surface_id: u32,
        toplevel_id: u32,
        title: &str,
        app_id: &str,
    ) -> Result<(), &'static str> {
        let xdg_surface = self
            .xdg_surfaces
            .iter_mut()
            .find(|x| x.xdg_surface_id == xdg_surface_id)
            .ok_or("WaylandProtocolEngine: xdg_surface not found")?;

        xdg_surface.toplevel = Some(XdgToplevel::new(toplevel_id, title, app_id));
        Ok(())
    }

    pub fn send_xdg_configure(&mut self, xdg_surface_id: u32) -> Result<u32, &'static str> {
        let serial = self.next_serial();
        let xdg_surface = self
            .xdg_surfaces
            .iter_mut()
            .find(|x| x.xdg_surface_id == xdg_surface_id)
            .ok_or("WaylandProtocolEngine: xdg_surface not found")?;

        xdg_surface.configure_serial = serial;
        Ok(serial)
    }

    pub fn ack_xdg_configure(
        &mut self,
        xdg_surface_id: u32,
        serial: u32,
    ) -> Result<(), &'static str> {
        let xdg_surface = self
            .xdg_surfaces
            .iter_mut()
            .find(|x| x.xdg_surface_id == xdg_surface_id)
            .ok_or("WaylandProtocolEngine: xdg_surface not found")?;

        if xdg_surface.configure_serial != serial {
            return Err("WaylandProtocolEngine: Configure serial mismatch");
        }

        xdg_surface.acked_serial = serial;
        Ok(())
    }

    pub fn register_seat(&mut self, seat_id: u32, name: &str) -> Result<(), &'static str> {
        if self.seats.iter().any(|s| s.seat_id == seat_id) {
            return Err("WaylandProtocolEngine: Seat ID already exists");
        }
        let seat = WlSeat::new(seat_id, name);
        self.seats.push(seat);
        self.data_devices.push(WlDataDevice::new(seat_id + 1000, seat_id));
        Ok(())
    }

    pub fn dispatch_pointer_motion(
        &mut self,
        seat_id: u32,
        x: f64,
        y: f64,
    ) -> Result<(), &'static str> {
        let seat = self
            .seats
            .iter_mut()
            .find(|s| s.seat_id == seat_id)
            .ok_or("WaylandProtocolEngine: Seat not found")?;

        seat.pointer_x = x;
        seat.pointer_y = y;

        // Find matching surface under pointer
        let focused_id = self.surfaces.iter().find_map(|s| {
            if (x as u32) < s.width && (y as u32) < s.height {
                Some(s.surface_id)
            } else {
                None
            }
        });

        seat.focused_surface_id = focused_id;
        Ok(())
    }

    pub fn dispatch_key_event(
        &mut self,
        seat_id: u32,
        key: u32,
        pressed: bool,
    ) -> Result<(), &'static str> {
        let seat = self
            .seats
            .iter_mut()
            .find(|s| s.seat_id == seat_id)
            .ok_or("WaylandProtocolEngine: Seat not found")?;

        if pressed {
            if !seat.pressed_keys.contains(&key) {
                seat.pressed_keys.push(key);
            }
        } else {
            seat.pressed_keys.retain(|&k| k != key);
        }
        Ok(())
    }

    pub fn register_data_offer(
        &mut self,
        device_id: u32,
        offer_id: u32,
        mime_types: &[&str],
    ) -> Result<(), &'static str> {
        let device = self
            .data_devices
            .iter_mut()
            .find(|d| d.device_id == device_id)
            .ok_or("WaylandProtocolEngine: Data device not found")?;

        let offer = WlDataOffer {
            offer_id,
            mime_types: mime_types.iter().map(|s| s.to_string()).collect(),
        };

        device.offers.push(offer);
        Ok(())
    }

    pub fn set_selection(&mut self, device_id: u32, offer_id: u32) -> Result<(), &'static str> {
        let device = self
            .data_devices
            .iter_mut()
            .find(|d| d.device_id == device_id)
            .ok_or("WaylandProtocolEngine: Data device not found")?;

        let offer = device
            .offers
            .iter()
            .find(|o| o.offer_id == offer_id)
            .cloned()
            .ok_or("WaylandProtocolEngine: Data offer not found")?;

        device.active_selection = Some(offer);
        Ok(())
    }

    pub fn encode_wire_header(&self, object_id: u32, opcode: u16, len: u16) -> [u8; 8] {
        let header = WaylandMessageWireHeader {
            object_id,
            opcode,
            message_len: len,
        };
        header.encode()
    }

    pub fn decode_wire_header(&self, bytes: &[u8; 8]) -> WaylandMessageWireHeader {
        WaylandMessageWireHeader::decode(bytes)
    }
}

impl Default for WaylandProtocolEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wayland_wire_header_encode_decode() {
        let header = WaylandMessageWireHeader {
            object_id: 1001,
            opcode: 3,
            message_len: 24,
        };

        let encoded = header.encode();
        let decoded = WaylandMessageWireHeader::decode(&encoded);

        assert_eq!(decoded.object_id, 1001);
        assert_eq!(decoded.opcode, 3);
        assert_eq!(decoded.message_len, 24);
    }

    #[test]
    fn test_surface_lifecycle_and_damage() {
        let mut engine = WaylandProtocolEngine::new();
        engine.create_surface(1);

        assert!(engine.attach_buffer(1, 10, 1920, 1080).is_ok());
        assert!(engine.damage_surface(1, 0, 0, 1920, 1080).is_ok());
        assert!(engine.commit_surface(1).is_ok());

        let surface = engine.get_surface_mut(1).unwrap();
        assert!(surface.is_committed);
        assert_eq!(surface.width, 1920);
        assert_eq!(surface.height, 1080);
    }

    #[test]
    fn test_xdg_surface_and_toplevel_configure_ack() {
        let mut engine = WaylandProtocolEngine::new();
        engine.create_surface(10);

        assert!(engine.create_xdg_surface(100, 10).is_ok());
        assert!(engine
            .set_xdg_toplevel(100, 200, "Sigma Terminal", "org.sigmaos.terminal")
            .is_ok());

        let serial = engine.send_xdg_configure(100).unwrap();
        assert!(serial > 0);

        assert!(engine.ack_xdg_configure(100, serial).is_ok());
        assert!(engine.ack_xdg_configure(100, serial + 1).is_err());
    }

    #[test]
    fn test_seat_input_event_dispatch() {
        let mut engine = WaylandProtocolEngine::new();
        engine.create_surface(1);
        engine.attach_buffer(1, 100, 800, 600).unwrap();

        assert!(engine.register_seat(1, "default_seat").is_ok());
        assert!(engine.dispatch_pointer_motion(1, 100.0, 200.0).is_ok());

        let seat = &engine.seats[0];
        assert_eq!(seat.pointer_x, 100.0);
        assert_eq!(seat.pointer_y, 200.0);
        assert_eq!(seat.focused_surface_id, Some(1));

        // Key press and release
        assert!(engine.dispatch_key_event(1, 30, true).is_ok()); // KEY_A
        assert_eq!(engine.seats[0].pressed_keys, vec![30]);

        assert!(engine.dispatch_key_event(1, 30, false).is_ok());
        assert!(engine.seats[0].pressed_keys.is_empty());
    }

    #[test]
    fn test_data_device_clipboard_selection() {
        let mut engine = WaylandProtocolEngine::new();
        engine.register_seat(1, "default_seat").unwrap();

        let device_id = 1001;
        let offer_id = 500;
        let mimes = ["text/plain;charset=utf-8", "text/uri-list"];

        assert!(engine.register_data_offer(device_id, offer_id, &mimes).is_ok());
        assert!(engine.set_selection(device_id, offer_id).is_ok());

        let device = &engine.data_devices[0];
        let selection = device.active_selection.as_ref().unwrap();
        assert_eq!(selection.offer_id, 500);
        assert_eq!(selection.mime_types.len(), 2);
    }
}
