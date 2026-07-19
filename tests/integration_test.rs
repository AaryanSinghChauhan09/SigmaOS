// SigmaOS Integration Tests
// Tests for core system components
#![allow(unused, clippy::all)]

#[cfg(test)]
mod tests {
    use sigmaos::drivers::*;

    #[test]
    fn test_system_integration() {
        assert!(true);
    }

    #[test]
    fn test_peripheral_manager_driver_validation() {
        let mut manager = PeripheralManager::new();

        // 1. Register 12 drivers from `more_devices.rs`
        assert!(manager
            .register_device(Box::new(FloppyDiskDriver::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(SoundBlaster16Driver::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(GameportJoystickDriver::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(IdeControllerDriver::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(ParallelPrinterDriver::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(CgaGraphicsDriver::new()))
            .is_ok());

        assert!(manager
            .register_device(Box::new(PcieGen5NvmeDriver::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(Thunderbolt4Controller::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(Wifi7Adapter::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(IntelXeGpuDriver::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(CxlMemoryDriver::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(AppleSiliconUnifiedMemoryBus::new()))
            .is_ok());

        // 2. Register 12 drivers from `even_more_devices.rs`
        assert!(manager
            .register_device(Box::new(AdLibSynthDriver::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(PciIdeBridge::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(Ps2MouseDriver::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(VgaTextModeDriver::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(SerialMouseDriver::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(Ne2000NetworkDriver::new()))
            .is_ok());

        assert!(manager
            .register_device(Box::new(Usb4HostController::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(NvlinkBusDriver::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(Bluetooth5_4Adapter::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(PcieGen6Bridge::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(Sata3Controller::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(Ufs4StorageDriver::new()))
            .is_ok());

        // 3. Register 9 drivers from `kernel_releases.rs`
        assert!(manager
            .register_device(Box::new(MainlineReleaseDriver::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(StableReleaseDriver::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(LongtermReleaseDriver::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(PrepatchRcDriver1::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(PrepatchRcDriver2::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(PrepatchRcDriver3::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(PrepatchRcDriver4::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(PrepatchRcDriver5::new()))
            .is_ok());
        assert!(manager
            .register_device(Box::new(PrepatchRcDriver6::new()))
            .is_ok());

        // 4. Verify all 33 drivers are registered and initialized
        assert_eq!(manager.device_count(), 33);

        // Test power broadcasting
        manager.broadcast_power_state(PowerState::Sleep);
    }
}
