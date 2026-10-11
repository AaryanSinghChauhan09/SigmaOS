// tests/hardware/driver_evidence_test.rs
// Standalone unit test verifying driver registration and evidence reporting invariants

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DriverEvidenceLevel {
    Verified,
    Partial,
    Experimental,
    Untested,
}

#[derive(Debug, Clone)]
pub struct DeviceDriverDescriptor {
    pub name: String,
    pub class_name: String,
    pub evidence_level: DriverEvidenceLevel,
    pub has_unit_tests: bool,
    pub has_runtime_evidence: bool,
}

pub struct DriverRegistry {
    pub drivers: Vec<DeviceDriverDescriptor>,
}

impl DriverRegistry {
    pub fn new() -> Self {
        Self { drivers: Vec::new() }
    }

    pub fn register(&mut self, driver: DeviceDriverDescriptor) {
        self.drivers.push(driver);
    }

    pub fn validate_evidence_integrity(&self) -> Result<(), String> {
        for drv in &self.drivers {
            if drv.evidence_level == DriverEvidenceLevel::Verified && !drv.has_runtime_evidence {
                return Err(format!(
                    "DRIVER HONESTY VIOLATION: Driver '{}' marked Verified without runtime evidence!",
                    drv.name
                ));
            }
        }
        Ok(())
    }
}

#[test]
fn test_driver_evidence_reporting_invariants() {
    let mut registry = DriverRegistry::new();

    registry.register(DeviceDriverDescriptor {
        name: "16550_uart".to_string(),
        class_name: "serial".to_string(),
        evidence_level: DriverEvidenceLevel::Verified,
        has_unit_tests: true,
        has_runtime_evidence: true,
    });

    registry.register(DeviceDriverDescriptor {
        name: "virtio_gpu".to_string(),
        class_name: "display".to_string(),
        evidence_level: DriverEvidenceLevel::Verified,
        has_unit_tests: true,
        has_runtime_evidence: true,
    });

    registry.register(DeviceDriverDescriptor {
        name: "wifi7_mlo".to_string(),
        class_name: "network".to_string(),
        evidence_level: DriverEvidenceLevel::Experimental,
        has_unit_tests: true,
        has_runtime_evidence: false,
    });

    let res = registry.validate_evidence_integrity();
    assert!(res.is_ok(), "Driver registry evidence integrity validation failed: {:?}", res);
    println!("Driver evidence integrity invariants verified successfully.");
}
