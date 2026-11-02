//! SigmaOS asdcontrol – ASD Display Control Library
//!
//! This library provides low-level access to ASD (Advanced System Display)
//! hardware registers through the DDC/CI protocol over I²C.  It is the
//! foundation for all display brightness, contrast, and color-temperature
//! management on SigmaOS.
//!
//! # Architecture
//!
//! - [`Display`]: represents a single physical monitor.  Discovered via
//!   `/dev/i2c-*` enumeration and DDC/CI capability queries.
//! - [`DisplayManager`]: enumerates all connected displays and provides
//!   an iterator over [`Display`] instances.
//! - [`VcpFeature`]: strongly-typed enum of the VCP (Virtual Control Panel)
//!   feature codes this library supports (brightness, contrast, color temp,
//!   etc.).
//! - [`AsdError`]: unified error type covering I²C I/O errors, DDC/CI
//!   protocol violations, and out-of-range register values.
//!
//! # Example
//!
//! ```rust,no_run
//! use sigmaos_asdcontrol::{DisplayManager, VcpFeature};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let manager = DisplayManager::new()?;
//!     for mut display in manager.displays()? {
//!         println!("Display: {}", display.name());
//!         let brightness = display.get(VcpFeature::Brightness)?;
//!         println!("  Brightness: {}/{}", brightness.current, brightness.maximum);
//!     }
//!     Ok(())
//! }
//! ```

/// Errors produced by the asdcontrol library.
#[derive(Debug)]
pub enum AsdError {
    /// An I²C I/O error (wraps the underlying OS error message).
    Io(String),
    /// The DDC/CI response did not conform to the protocol specification.
    Protocol(String),
    /// The requested value is outside the display's reported valid range.
    OutOfRange { value: u16, maximum: u16 },
    /// No display was found matching the requested criteria.
    NotFound(String),
}

impl std::fmt::Display for AsdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsdError::Io(msg) => write!(f, "I/O error: {msg}"),
            AsdError::Protocol(msg) => write!(f, "DDC/CI protocol error: {msg}"),
            AsdError::OutOfRange { value, maximum } => {
                write!(f, "Value {value} exceeds maximum {maximum}")
            }
            AsdError::NotFound(msg) => write!(f, "Display not found: {msg}"),
        }
    }
}

impl std::error::Error for AsdError {}

/// A VCP (Virtual Control Panel) feature code supported by this library.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VcpFeature {
    /// Luminance / backlight brightness (VCP 0x10).
    Brightness,
    /// Image contrast (VCP 0x12).
    Contrast,
    /// Color temperature (VCP 0x0B, in 100K units offset from 3000K).
    ColorTemperature,
    /// Input source select (VCP 0x60).
    InputSource,
}

impl VcpFeature {
    /// Returns the DDC/CI VCP feature code byte for this feature.
    pub fn code(self) -> u8 {
        match self {
            VcpFeature::Brightness => 0x10,
            VcpFeature::Contrast => 0x12,
            VcpFeature::ColorTemperature => 0x0B,
            VcpFeature::InputSource => 0x60,
        }
    }
}

/// The current and maximum values of a VCP feature returned by a `GetVCP` query.
#[derive(Debug, Clone, Copy)]
pub struct VcpValue {
    pub current: u16,
    pub maximum: u16,
}

/// A single physical display accessible via DDC/CI over I²C.
#[derive(Debug)]
pub struct Display {
    name: String,
    i2c_bus: u8,
}

impl Display {
    /// Create a new `Display` wrapping the given I²C bus index.
    pub fn new(i2c_bus: u8) -> Self {
        Display {
            name: format!("Display on /dev/i2c-{i2c_bus}"),
            i2c_bus,
        }
    }

    /// Returns the human-readable name for this display.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the I²C bus index used to communicate with this display.
    pub fn i2c_bus(&self) -> u8 {
        self.i2c_bus
    }

    /// Read the current and maximum value of a VCP feature.
    ///
    /// # Errors
    ///
    /// Returns [`AsdError::Io`] if the I²C device cannot be opened, or
    /// [`AsdError::Protocol`] if the DDC/CI response is malformed.
    pub fn get(&self, feature: VcpFeature) -> Result<VcpValue, AsdError> {
        // Placeholder: a real implementation opens /dev/i2c-<bus> and performs
        // a DDC/CI GetVCP transaction (I2C_M_RD at address 0x37).
        let _ = feature.code();
        Ok(VcpValue {
            current: 80,
            maximum: 100,
        })
    }

    /// Write a new value to a VCP feature.
    ///
    /// # Errors
    ///
    /// Returns [`AsdError::OutOfRange`] if `value` exceeds the feature maximum,
    /// or [`AsdError::Io`] on I²C write failure.
    pub fn set(&mut self, feature: VcpFeature, value: u16) -> Result<(), AsdError> {
        let current = self.get(feature)?;
        if value > current.maximum {
            return Err(AsdError::OutOfRange {
                value,
                maximum: current.maximum,
            });
        }
        // Placeholder: real implementation issues a DDC/CI SetVCP command.
        let _ = feature.code();
        Ok(())
    }
}

/// Enumerates all ASD-capable displays connected to the system.
#[derive(Debug, Default)]
pub struct DisplayManager;

impl DisplayManager {
    /// Create a new `DisplayManager`.
    pub fn new() -> Result<Self, AsdError> {
        Ok(DisplayManager)
    }

    /// Return an iterator over all discovered displays.
    ///
    /// Scans `/dev/i2c-*` devices and filters those that respond to DDC/CI
    /// capability queries.
    ///
    /// # Errors
    ///
    /// Returns [`AsdError::Io`] if the `/dev` directory cannot be read.
    pub fn displays(&self) -> Result<Vec<Display>, AsdError> {
        // Placeholder: real implementation reads /dev/i2c-* and probes each.
        Ok(vec![Display::new(0)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vcp_brightness_code() {
        assert_eq!(VcpFeature::Brightness.code(), 0x10);
    }

    #[test]
    fn vcp_contrast_code() {
        assert_eq!(VcpFeature::Contrast.code(), 0x12);
    }

    #[test]
    fn display_get_returns_placeholder_value() {
        let display = Display::new(0);
        let val = display.get(VcpFeature::Brightness).unwrap();
        assert_eq!(val.current, 80);
        assert_eq!(val.maximum, 100);
    }

    #[test]
    fn display_set_out_of_range_errors() {
        let mut display = Display::new(0);
        let result = display.set(VcpFeature::Brightness, 200);
        assert!(matches!(result, Err(AsdError::OutOfRange { value: 200, maximum: 100 })));
    }

    #[test]
    fn display_manager_returns_one_display() {
        let mgr = DisplayManager::new().unwrap();
        let displays = mgr.displays().unwrap();
        assert_eq!(displays.len(), 1);
    }
}
