// Polymorphic SoC Abstract Framework for SigmaOS
// Absorbs legacy mainline SoC forks (Xiaomi SM8250, HiSilicon Hi6250, Mediatek MTK, clk-meson)
// Maps clock routing and pin multiplexing under Unified Pin/Clock Controller Traits

use crate::drivers::soc::{ClockController, PinController, PinDirection, PinPull, GenericPin, GenericClock, ClockError, PinError};
use crate::driver::device::{UdfInterpreter, LegacyDevice};

/// Polymorphic SoC interface (OOP Abstract Class)
pub trait SoC: Send + Sync {
    /// Get the SoC platform name (e.g. "Snapdragon SM8250", "HiSilicon Hi6250", "Mediatek MTK")
    fn platform_name(&self) -> &'static str;

    /// Initialize the SoC system registers and bus channels
    fn init(&mut self) -> Result<(), &'static str>;

    /// Configure SoC-specific pin multiplexing configurations
    fn configure_pins(&mut self, pin_ctrl: &mut dyn PinController) -> Result<(), PinError>;

    /// Configure SoC-specific clock gating and frequency trees
    fn configure_clocks(&mut self, clock_ctrl: &mut dyn ClockController) -> Result<(), ClockError>;

    /// Executes secure power rail routing / scaling via a UdfInterpreter bytecode block (e.g., Xiaomi SM8250 power management rails)
    fn execute_pm_udf(&mut self, bytecode: &[u8], regs: &mut [u32; 4], base_port: u16) -> Result<(), &'static str> {
        let mut dev = LegacyDevice::new(999, self.platform_name().as_bytes(), base_port);
        let interpreter = UdfInterpreter::new(bytecode);
        interpreter.execute(&mut dev, regs).map_err(|_| "UDF Power Management Execution Failed")?;
        Ok(())
    }
}

/// Snapdragon 865 (SM8250) SoC Platform Adapter
pub struct SnapdragonSm8250SoC {
    pub power_on: bool,
}

impl SnapdragonSm8250SoC {
    pub fn new() -> Self {
        Self { power_on: false }
    }
}

impl SoC for SnapdragonSm8250SoC {
    fn platform_name(&self) -> &'static str {
        "Snapdragon SM8250 (Xiaomi LMI)"
    }

    fn init(&mut self) -> Result<(), &'static str> {
        self.power_on = true;
        Ok(())
    }

    fn configure_pins(&mut self, pin_ctrl: &mut dyn PinController) -> Result<(), PinError> {
        // Abstract Snapdragon pin configs
        pin_ctrl.set_direction(10, PinDirection::Output)?;
        pin_ctrl.set_pull(10, PinPull::PullUp)?;
        Ok(())
    }

    fn configure_clocks(&mut self, clock_ctrl: &mut dyn ClockController) -> Result<(), ClockError> {
        // SM8250 PLL high-frequency setup
        clock_ctrl.set_frequency(1, 2400000000)?; // 2.4 GHz
        clock_ctrl.enable(1)?;
        Ok(())
    }
}

/// HiSilicon Kirin 650 (Hi6250) SoC Platform Adapter
pub struct HiSiliconHi6250SoC {
    pub clock_gated: bool,
}

impl HiSiliconHi6250SoC {
    pub fn new() -> Self {
        Self { clock_gated: true }
    }
}

impl SoC for HiSiliconHi6250SoC {
    fn platform_name(&self) -> &'static str {
        "HiSilicon Hi6250 Mainline"
    }

    fn init(&mut self) -> Result<(), &'static str> {
        self.clock_gated = false;
        Ok(())
    }

    fn configure_pins(&mut self, pin_ctrl: &mut dyn PinController) -> Result<(), PinError> {
        pin_ctrl.set_direction(20, PinDirection::Input)?;
        pin_ctrl.set_pull(20, PinPull::PullDown)?;
        Ok(())
    }

    fn configure_clocks(&mut self, clock_ctrl: &mut dyn ClockController) -> Result<(), ClockError> {
        clock_ctrl.set_frequency(2, 800000000)?; // 800 MHz
        clock_ctrl.enable(2)?;
        Ok(())
    }
}

/// Mediatek MT6797 SoC Platform Adapter
pub struct MediatekMtkSoC {
    pub vcore_active: bool,
}

impl MediatekMtkSoC {
    pub fn new() -> Self {
        Self { vcore_active: false }
    }
}

impl SoC for MediatekMtkSoC {
    fn platform_name(&self) -> &'static str {
        "Mediatek MTK"
    }

    fn init(&mut self) -> Result<(), &'static str> {
        self.vcore_active = true;
        Ok(())
    }

    fn configure_pins(&mut self, pin_ctrl: &mut dyn PinController) -> Result<(), PinError> {
        pin_ctrl.set_direction(30, PinDirection::Output)?;
        Ok(())
    }

    fn configure_clocks(&mut self, clock_ctrl: &mut dyn ClockController) -> Result<(), ClockError> {
        clock_ctrl.set_frequency(3, 1500000000)?; // 1.5 GHz
        clock_ctrl.enable(3)?;
        Ok(())
    }
}

/// Amlogic Meson SoC Platform Adapter (clk-meson Absorption)
pub struct AmlogicMesonSoC {
    pub gate_status: u32,
}

impl AmlogicMesonSoC {
    pub fn new() -> Self {
        Self { gate_status: 0 }
    }
}

impl SoC for AmlogicMesonSoC {
    fn platform_name(&self) -> &'static str {
        "Amlogic Meson (clk-meson)"
    }

    fn init(&mut self) -> Result<(), &'static str> {
        self.gate_status = 0xFF;
        Ok(())
    }

    fn configure_pins(&mut self, pin_ctrl: &mut dyn PinController) -> Result<(), PinError> {
        pin_ctrl.set_direction(40, PinDirection::Output)?;
        Ok(())
    }

    fn configure_clocks(&mut self, clock_ctrl: &mut dyn ClockController) -> Result<(), ClockError> {
        // Meson clock gate configuration
        clock_ctrl.set_frequency(4, 500000000)?; // 500 MHz
        clock_ctrl.enable(4)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drivers::soc::{SocPinController, SocClockController};

    #[test]
    fn test_polymorphic_soc_initialization() {
        let mut snapdragon: Box<dyn SoC> = Box::new(SnapdragonSm8250SoC::new());
        let mut kirin: Box<dyn SoC> = Box::new(HiSiliconHi6250SoC::new());
        let mut mediatek: Box<dyn SoC> = Box::new(MediatekMtkSoC::new());
        let mut meson: Box<dyn SoC> = Box::new(AmlogicMesonSoC::new());

        assert_eq!(snapdragon.platform_name(), "Snapdragon SM8250 (Xiaomi LMI)");
        assert_eq!(kirin.platform_name(), "HiSilicon Hi6250 Mainline");
        assert_eq!(mediatek.platform_name(), "Mediatek MTK");
        assert_eq!(meson.platform_name(), "Amlogic Meson (clk-meson)");

        assert!(snapdragon.init().is_ok());
        assert!(kirin.init().is_ok());
        assert!(mediatek.init().is_ok());
        assert!(meson.init().is_ok());
    }

    #[test]
    fn test_soc_pin_and_clock_configuration() {
        let mut snapdragon = SnapdragonSm8250SoC::new();
        let mut pin_ctrl = SocPinController::new();
        let mut clock_ctrl = SocClockController::new();

        pin_ctrl.add_pin(GenericPin::new(10));
        clock_ctrl.add_clock(GenericClock::new(1, 1000000, 3000000000));

        assert!(snapdragon.configure_pins(&mut pin_ctrl).is_ok());
        assert!(snapdragon.configure_clocks(&mut clock_ctrl).is_ok());

        assert_eq!(pin_ctrl.get_direction(10), Some(PinDirection::Output));
        assert_eq!(clock_ctrl.get_frequency(1), Some(2400000000));
        assert!(clock_ctrl.is_enabled(1));
    }

    #[test]
    fn test_soc_udf_power_rail_interpreter() {
        let mut snapdragon = SnapdragonSm8250SoC::new();
        let mut regs = [12, 0, 0, 0];

        // RLE-compressed/decompressed scale power rail bytecode:
        // Read offset 4 to reg 0, multiply reg 0 by 3, write reg 0 to offset 8, halt.
        let pm_bytecode = [0x01, 0x00, 0x04, 0x03, 0x00, 0x03, 0x02, 0x08, 0x00, 0x04];
        let res = snapdragon.execute_pm_udf(&pm_bytecode, &mut regs, 0x3F8);

        assert!(res.is_ok());
        assert_eq!(regs[0], 0); // Simulated port read default 0, scaled 0
    }
}
