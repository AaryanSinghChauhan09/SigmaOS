// SoC Hardware Absorption
// Unified Pin/GPIO and Clock Controller Framework
// Absorbs clk-meson, MTK, Snapdragon, and other SoC-specific drivers

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinDirection {
    Input,
    Output,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinPull {
    None,
    PullUp,
    PullDown,
    HighZ,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockError {
    ClockLocked,
    FrequencyOutOfBounds,
    NoSuchClockLine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinError {
    PinLocked,
    InvalidPin,
    DirectionLocked,
}

/// Unified Pin Controller Trait
pub trait PinController {
    fn set_direction(&mut self, pin: u32, direction: PinDirection) -> Result<(), PinError>;
    fn set_pull(&mut self, pin: u32, pull: PinPull) -> Result<(), PinError>;
    fn get_direction(&self, pin: u32) -> Option<PinDirection>;
    fn read(&self, pin: u32) -> Option<bool>;
    fn write(&mut self, pin: u32, value: bool) -> Result<(), PinError>;
}

/// Unified Clock Controller Trait
pub trait ClockController {
    fn set_frequency(&mut self, clock_id: u32, freq_hz: u32) -> Result<(), ClockError>;
    fn get_frequency(&self, clock_id: u32) -> Option<u32>;
    fn enable(&mut self, clock_id: u32) -> Result<(), ClockError>;
    fn disable(&mut self, clock_id: u32) -> Result<(), ClockError>;
    fn is_enabled(&self, clock_id: u32) -> bool;
}

/// Generic Pin Implementation
pub struct GenericPin {
    pub id: u32,
    pub direction: PinDirection,
    pub pull: PinPull,
    pub state: bool,
    pub locked: bool,
}

impl GenericPin {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            direction: PinDirection::Input,
            pull: PinPull::None,
            state: false,
            locked: false,
        }
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn unlock(&mut self) {
        self.locked = false;
    }
}

/// Generic Clock Implementation
pub struct GenericClock {
    pub id: u32,
    pub frequency: u32,
    pub enabled: bool,
    pub locked: bool,
    pub min_freq: u32,
    pub max_freq: u32,
}

impl GenericClock {
    pub fn new(id: u32, min_freq: u32, max_freq: u32) -> Self {
        Self {
            id,
            frequency: 0,
            enabled: false,
            locked: false,
            min_freq,
            max_freq,
        }
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn unlock(&mut self) {
        self.locked = false;
    }
}

/// SoC Pin Controller
pub struct SocPinController {
    pins: BTreeMap<u32, GenericPin>,
}

impl SocPinController {
    pub fn new() -> Self {
        Self {
            pins: BTreeMap::new(),
        }
    }

    pub fn add_pin(&mut self, pin: GenericPin) {
        self.pins.insert(pin.id, pin);
    }

    pub fn get_pin(&self, pin_id: u32) -> Option<&GenericPin> {
        self.pins.get(&pin_id)
    }

    pub fn pin_count(&self) -> usize {
        self.pins.len()
    }
}

impl PinController for SocPinController {
    fn set_direction(&mut self, pin: u32, direction: PinDirection) -> Result<(), PinError> {
        let pin_obj = self.pins.get_mut(&pin).ok_or(PinError::InvalidPin)?;

        if pin_obj.locked {
            return Err(PinError::DirectionLocked);
        }

        pin_obj.direction = direction;
        Ok(())
    }

    fn set_pull(&mut self, pin: u32, pull: PinPull) -> Result<(), PinError> {
        let pin_obj = self.pins.get_mut(&pin).ok_or(PinError::InvalidPin)?;

        if pin_obj.locked {
            return Err(PinError::PinLocked);
        }

        pin_obj.pull = pull;
        Ok(())
    }

    fn get_direction(&self, pin: u32) -> Option<PinDirection> {
        self.pins.get(&pin).map(|p| p.direction)
    }

    fn read(&self, pin: u32) -> Option<bool> {
        self.pins.get(&pin).map(|p| p.state)
    }

    fn write(&mut self, pin: u32, value: bool) -> Result<(), PinError> {
        let pin_obj = self.pins.get_mut(&pin).ok_or(PinError::InvalidPin)?;

        if pin_obj.locked {
            return Err(PinError::PinLocked);
        }

        if pin_obj.direction != PinDirection::Output {
            return Err(PinError::DirectionLocked);
        }

        pin_obj.state = value;
        Ok(())
    }
}

impl Default for SocPinController {
    fn default() -> Self {
        Self::new()
    }
}

/// SoC Clock Controller
pub struct SocClockController {
    clocks: BTreeMap<u32, GenericClock>,
}

impl SocClockController {
    pub fn new() -> Self {
        Self {
            clocks: BTreeMap::new(),
        }
    }

    pub fn add_clock(&mut self, clock: GenericClock) {
        self.clocks.insert(clock.id, clock);
    }

    pub fn get_clock(&self, clock_id: u32) -> Option<&GenericClock> {
        self.clocks.get(&clock_id)
    }

    pub fn clock_count(&self) -> usize {
        self.clocks.len()
    }
}

impl ClockController for SocClockController {
    fn set_frequency(&mut self, clock_id: u32, freq_hz: u32) -> Result<(), ClockError> {
        let clock = self.clocks.get_mut(&clock_id)
            .ok_or(ClockError::NoSuchClockLine)?;

        if clock.locked {
            return Err(ClockError::ClockLocked);
        }

        if freq_hz < clock.min_freq || freq_hz > clock.max_freq {
            return Err(ClockError::FrequencyOutOfBounds);
        }

        clock.frequency = freq_hz;
        Ok(())
    }

    fn get_frequency(&self, clock_id: u32) -> Option<u32> {
        self.clocks.get(&clock_id).map(|c| c.frequency)
    }

    fn enable(&mut self, clock_id: u32) -> Result<(), ClockError> {
        let clock = self.clocks.get_mut(&clock_id)
            .ok_or(ClockError::NoSuchClockLine)?;

        if clock.locked {
            return Err(ClockError::ClockLocked);
        }

        clock.enabled = true;
        Ok(())
    }

    fn disable(&mut self, clock_id: u32) -> Result<(), ClockError> {
        let clock = self.clocks.get_mut(&clock_id)
            .ok_or(ClockError::NoSuchClockLine)?;

        if clock.locked {
            return Err(ClockError::ClockLocked);
        }

        clock.enabled = false;
        Ok(())
    }

    fn is_enabled(&self, clock_id: u32) -> bool {
        self.clocks.get(&clock_id).map(|c| c.enabled).unwrap_or(false)
    }
}

impl Default for SocClockController {
    fn default() -> Self {
        Self::new()
    }
}

/// Unified SoC Controller
pub struct UnifiedSocController {
    pub pin_controller: SocPinController,
    pub clock_controller: SocClockController,
}

impl UnifiedSocController {
    pub fn new() -> Self {
        Self {
            pin_controller: SocPinController::new(),
            clock_controller: SocClockController::new(),
        }
    }

    pub fn add_pin(&mut self, pin: GenericPin) {
        self.pin_controller.add_pin(pin);
    }

    pub fn add_clock(&mut self, clock: GenericClock) {
        self.clock_controller.add_clock(clock);
    }

    pub fn pin_controller(&self) -> &SocPinController {
        &self.pin_controller
    }

    pub fn clock_controller(&self) -> &SocClockController {
        &self.clock_controller
    }

    pub fn pin_controller_mut(&mut self) -> &mut SocPinController {
        &mut self.pin_controller
    }

    pub fn clock_controller_mut(&mut self) -> &mut SocClockController {
        &mut self.clock_controller
    }
}

impl Default for UnifiedSocController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pin_creation() {
        let pin = GenericPin::new(1);
        assert_eq!(pin.id, 1);
        assert_eq!(pin.direction, PinDirection::Input);
    }

    #[test]
    fn test_pin_direction() {
        let mut controller = SocPinController::new();
        controller.add_pin(GenericPin::new(1));

        controller.set_direction(1, PinDirection::Output).unwrap();
        assert_eq!(controller.get_direction(1), Some(PinDirection::Output));
    }

    #[test]
    fn test_pin_locked() {
        let mut controller = SocPinController::new();
        let mut pin = GenericPin::new(1);
        pin.lock();
        controller.add_pin(pin);

        assert!(controller.set_direction(1, PinDirection::Output).is_err());
    }

    #[test]
    fn test_pin_write() {
        let mut controller = SocPinController::new();
        let mut pin = GenericPin::new(1);
        pin.direction = PinDirection::Output;
        controller.add_pin(pin);

        controller.write(1, true).unwrap();
        assert_eq!(controller.read(1), Some(true));
    }

    #[test]
    fn test_pin_pull() {
        let mut controller = SocPinController::new();
        controller.add_pin(GenericPin::new(1));

        controller.set_pull(1, PinPull::PullUp).unwrap();
        let pin = controller.get_pin(1).unwrap();
        assert_eq!(pin.pull, PinPull::PullUp);
    }

    #[test]
    fn test_clock_creation() {
        let clock = GenericClock::new(1, 1000, 100000);
        assert_eq!(clock.id, 1);
        assert_eq!(clock.min_freq, 1000);
        assert_eq!(clock.max_freq, 100000);
    }

    #[test]
    fn test_clock_frequency() {
        let mut controller = SocClockController::new();
        controller.add_clock(GenericClock::new( 1, 1000, 100000));

        controller.set_frequency(1, 50000).unwrap();
        assert_eq!(controller.get_frequency(1), Some(50000));
    }

    #[test]
    fn test_clock_frequency_bounds() {
        let mut controller = SocClockController::new();
        controller.add_clock(GenericClock::new(1, 1000, 100000));

        assert!(controller.set_frequency(1, 500).is_err());
        assert!(controller.set_frequency(1, 200000).is_err());
    }

    #[test]
    fn test_clock_enable() {
        let mut controller = SocClockController::new();
        controller.add_clock(GenericClock::new(1, 1000, 100000));

        controller.enable(1).unwrap();
        assert!(controller.is_enabled(1));
    }

    #[test]
    fn test_clock_locked() {
        let mut controller = SocClockController::new();
        let mut clock = GenericClock::new(1, 1000, 100000);
        clock.lock();
        controller.add_clock(clock);

        assert!(controller.enable(1).is_err());
    }

    #[test]
    fn test_unified_soc_controller() {
        let mut soc = UnifiedSocController::new();

        soc.add_pin(GenericPin::new(1));
        soc.add_clock(GenericClock::new(1, 1000, 100000));

        assert_eq!(soc.pin_controller().pin_count(), 1);
        assert_eq!(soc.clock_controller().clock_count(), 1);
    }

    #[test]
    fn test_pin_write_input_direction() {
        let mut controller = SocPinController::new();
        controller.add_pin(GenericPin::new(1));

        assert!(controller.write(1, true).is_err());
    }
}
