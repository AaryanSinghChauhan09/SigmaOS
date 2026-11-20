// installer/tests/failure_injection.rs
// Standalone integration test for installer abort, failure injection, and rollback state cleanup

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FailurePoint {
    None,
    DiskPartitioning,
    FilesystemFormatting,
    PackageExtraction,
    BootloaderInstallation,
}

pub struct TransactionalSnapshot {
    pub id: u64,
    pub created_files: Vec<String>,
    pub active_mounts: Vec<String>,
}

pub struct TransactionalInstaller {
    pub failure_point: FailurePoint,
    pub snapshot: TransactionalSnapshot,
    pub is_rolled_back: bool,
}

impl TransactionalInstaller {
    pub fn new(failure_point: FailurePoint) -> Self {
        Self {
            failure_point,
            snapshot: TransactionalSnapshot {
                id: 1,
                created_files: Vec::new(),
                active_mounts: Vec::new(),
            },
            is_rolled_back: false,
        }
    }

    pub fn run_installation(&mut self) -> Result<(), &'static str> {
        // Stage 1: Mount Target
        self.snapshot.active_mounts.push("/mnt/target".to_string());
        if self.failure_point == FailurePoint::DiskPartitioning {
            self.rollback();
            return Err("Injected failure during disk partitioning");
        }

        // Stage 2: Format & Base System
        self.snapshot.created_files.push("/mnt/target/etc/fstab".to_string());
        if self.failure_point == FailurePoint::FilesystemFormatting {
            self.rollback();
            return Err("Injected failure during filesystem formatting");
        }

        // Stage 3: Package Extraction
        self.snapshot.created_files.push("/mnt/target/boot/vmlinuz".to_string());
        if self.failure_point == FailurePoint::PackageExtraction {
            self.rollback();
            return Err("Injected failure during package extraction");
        }

        // Stage 4: Bootloader
        if self.failure_point == FailurePoint::BootloaderInstallation {
            self.rollback();
            return Err("Injected failure during bootloader installation");
        }

        Ok(())
    }

    pub fn rollback(&mut self) {
        println!("Initiating transactional installer rollback...");
        self.snapshot.created_files.clear();
        self.snapshot.active_mounts.clear();
        self.is_rolled_back = true;
    }
}

#[test]
fn test_installer_failure_injection_and_rollback() {
    let failure_points = vec![
        FailurePoint::DiskPartitioning,
        FailurePoint::FilesystemFormatting,
        FailurePoint::PackageExtraction,
        FailurePoint::BootloaderInstallation,
    ];

    for fp in failure_points {
        let mut installer = TransactionalInstaller::new(fp.clone());
        let res = installer.run_installation();

        assert!(res.is_err(), "Installation should fail under failure point {:?}", fp);
        assert!(installer.is_rolled_back, "Installer should set is_rolled_back = true");
        assert!(
            installer.snapshot.created_files.is_empty(),
            "Rollback failed to clean up created files for failure point {:?}", fp
        );
        assert!(
            installer.snapshot.active_mounts.is_empty(),
            "Rollback failed to unmount targets for failure point {:?}", fp
        );
    }

    println!("Installer failure injection and rollback test passed cleanly.");
}
