//! SigmaOS Hardware Abstraction Layer (HAL)
//! Unified driver interface abstraction and platform-specific shims (x86_64, ARM64, RISC-V).
//! Inspired by professional x86/x64 GDT, ARM64 Exception Levels, and Windows NT Kernel architectures (IRQL, KPCR, DPC, APC).

// #![no_std]

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Unified driver interface trait
pub trait SovereignDriver {
    fn probe(&self) -> Result<(), DriverError>;
    fn init(&mut self) -> Result<(), DriverError>;
    fn handle_irq(&mut self) -> IRQHandlerResult;
    fn shutdown(&mut self);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverError {
    ProbeFailed,
    InitFailed,
    IRQError,
    UnsupportedDevice,
    ResourceConflict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IRQHandlerResult {
    Handled,
    NotHandled,
    NeedsRetry,
}

/// Platform-specific HAL trait
pub trait PlatformHAL {
    fn init(&mut self) -> Result<(), DriverError>;
    fn enumerate_pci(&self) -> Result<(), DriverError>;
    fn parse_acpi(&self) -> Result<(), DriverError>;
    fn configure_apic(&self) -> Result<(), DriverError>;
}

/// x86_64 HAL implementation
pub struct X86_64HAL {
    pci_devices: AtomicUsize,
    acpi_tables: AtomicUsize,
    apic_enabled: AtomicUsize,
}

impl X86_64HAL {
    pub const fn new() -> Self {
        X86_64HAL {
            pci_devices: AtomicUsize::new(0),
            acpi_tables: AtomicUsize::new(0),
            apic_enabled: AtomicUsize::new(0),
        }
    }

    /// PCI enumeration for x86_64
    pub fn enumerate_pci(&self) -> Result<(), DriverError> {
        // Scan PCI configuration space
        for bus in 0..=255u8 {
            for device in 0..32 {
                for function in 0..8 {
                    let vendor_id = self.read_pci_config(bus, device, function, 0x00);
                    if vendor_id != 0xFFFF {
                        self.pci_devices.fetch_add(1, Ordering::SeqCst);
                    }
                }
            }
        }
        Ok(())
    }

    /// Read PCI configuration register
    fn read_pci_config(&self, bus: u8, device: u8, function: u8, offset: u16) -> u16 {
        // In real implementation, would use PCI configuration I/O ports
        // 0xCF8 for address, 0xCFC for data
        let _address = (1u32 << 31) | ((bus as u32) << 16) | ((device as u32) << 11) | ((function as u32) << 8) | ((offset as u32) & 0xFC);
        // Write to 0xCF8, read from 0xCFC
        0xFFFF // Stub
    }

    /// ACPI parsing for x86_64
    pub fn parse_acpi(&self) -> Result<(), DriverError> {
        // Find RSDP (Root System Description Pointer)
        // Parse RSDT/XSDT
        // Enumerate ACPI tables (MADT, FADT, DSDT, etc.)
        self.acpi_tables.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    /// APIC/MSI-X configuration for x86_64
    pub fn configure_apic(&self) -> Result<(), DriverError> {
        // Enable local APIC
        // Configure I/O APIC
        // Setup MSI-X for PCI devices
        self.apic_enabled.store(1, Ordering::SeqCst);
        Ok(())
    }
}

impl PlatformHAL for X86_64HAL {
    fn init(&mut self) -> Result<(), DriverError> {
        self.enumerate_pci()?;
        self.parse_acpi()?;
        self.configure_apic()?;
        Ok(())
    }

    fn enumerate_pci(&self) -> Result<(), DriverError> {
        self.enumerate_pci()
    }

    fn parse_acpi(&self) -> Result<(), DriverError> {
        self.parse_acpi()
    }

    fn configure_apic(&self) -> Result<(), DriverError> {
        self.configure_apic()
    }
}

/// ARM64 HAL implementation
pub struct ARM64HAL {
    device_tree: AtomicUsize,
    gic_enabled: AtomicUsize,
    smmu_enabled: AtomicUsize,
}

impl ARM64HAL {
    pub const fn new() -> Self {
        ARM64HAL {
            device_tree: AtomicUsize::new(0),
            gic_enabled: AtomicUsize::new(0),
            smmu_enabled: AtomicUsize::new(0),
        }
    }

    /// Device tree parsing for ARM64
    pub fn parse_device_tree(&self) -> Result<(), DriverError> {
        // Parse flattened device tree (FDT)
        // Extract device nodes and properties
        self.device_tree.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    /// GIC (Generic Interrupt Controller) for ARM64
    pub fn configure_gic(&self) -> Result<(), DriverError> {
        // Initialize GIC distributor
        // Configure CPU interfaces
        // Setup interrupt routing
        self.gic_enabled.store(1, Ordering::SeqCst);
        Ok(())
    }

    /// SMMU (System MMU) for ARM64
    pub fn configure_smmu(&self) -> Result<(), DriverError> {
        // Initialize SMMU for IOMMU functionality
        self.smmu_enabled.store(1, Ordering::SeqCst);
        Ok(())
    }
}

impl PlatformHAL for ARM64HAL {
    fn init(&mut self) -> Result<(), DriverError> {
        self.parse_device_tree()?;
        self.configure_gic()?;
        self.configure_smmu()?;
        Ok(())
    }

    fn enumerate_pci(&self) -> Result<(), DriverError> {
        // ARM64 may use PCIe or custom interconnects
        Ok(())
    }

    fn parse_acpi(&self) -> Result<(), DriverError> {
        // ARM64 may use ACPI or device tree
        Ok(())
    }

    fn configure_apic(&self) -> Result<(), DriverError> {
        // ARM64 uses GIC instead of APIC
        self.configure_gic()
    }
}

/// RISC-V HAL implementation
pub struct RISCV64HAL {
    plic_enabled: AtomicUsize,
    iommu_enabled: AtomicUsize,
    clint_enabled: AtomicUsize,
}

impl RISCV64HAL {
    pub const fn new() -> Self {
        RISCV64HAL {
            plic_enabled: AtomicUsize::new(0),
            iommu_enabled: AtomicUsize::new(0),
            clint_enabled: AtomicUsize::new(0),
        }
    }

    /// PLIC (Platform-Level Interrupt Controller) for RISC-V
    pub fn configure_plic(&self) -> Result<(), DriverError> {
        // Initialize PLIC
        // Set priority thresholds
        // Enable interrupt contexts
        self.plic_enabled.store(1, Ordering::SeqCst);
        Ok(())
    }

    /// IOMMU for RISC-V
    pub fn configure_iommu(&self) -> Result<(), DriverError> {
        // Initialize IOMMU for DMA protection
        self.iommu_enabled.store(1, Ordering::SeqCst);
        Ok(())
    }

    /// CLINT (Core-Local Interruptor) for RISC-V
    pub fn configure_clint(&self) -> Result<(), DriverError> {
        // Initialize CLINT for timer and software interrupts
        self.clint_enabled.store(1, Ordering::SeqCst);
        Ok(())
    }
}

impl PlatformHAL for RISCV64HAL {
    fn init(&mut self) -> Result<(), DriverError> {
        self.configure_plic()?;
        self.configure_iommu()?;
        self.configure_clint()?;
        Ok(())
    }

    fn enumerate_pci(&self) -> Result<(), DriverError> {
        // RISC-V may use PCIe or custom interconnects
        Ok(())
    }

    fn parse_acpi(&self) -> Result<(), DriverError> {
        // RISC-V typically uses device tree
        Ok(())
    }

    fn configure_apic(&self) -> Result<(), DriverError> {
        // RISC-V uses PLIC instead of APIC
        self.configure_plic()
    }
}

/// HAL factory for creating platform-specific instances
pub enum Architecture {
    X86_64,
    ARM64,
    RISCV64,
}

pub struct HALFactory;

impl HALFactory {
    pub fn create(arch: Architecture) -> Box<dyn PlatformHAL> {
        match arch {
            Architecture::X86_64 => Box::new(X86_64HAL::new()),
            Architecture::ARM64 => Box::new(ARM64HAL::new()),
            Architecture::RISCV64 => Box::new(RISCV64HAL::new()),
        }
    }
}

/// Example driver implementation using SovereignDriver trait
pub struct ExampleDriver {
    initialized: AtomicUsize,
    irq_count: AtomicUsize,
}

impl ExampleDriver {
    pub const fn new() -> Self {
        ExampleDriver {
            initialized: AtomicUsize::new(0),
            irq_count: AtomicUsize::new(0),
        }
    }
}

impl SovereignDriver for ExampleDriver {
    fn probe(&self) -> Result<(), DriverError> {
        // Check if device is present
        Ok(())
    }

    fn init(&mut self) -> Result<(), DriverError> {
        self.initialized.store(1, Ordering::SeqCst);
        Ok(())
    }

    fn handle_irq(&mut self) -> IRQHandlerResult {
        self.irq_count.fetch_add(1, Ordering::SeqCst);
        IRQHandlerResult::Handled
    }

    fn shutdown(&mut self) {
        self.initialized.store(0, Ordering::SeqCst);
    }
}

// ==============================================================================
// Windows NT-style IRQL (Interrupt Request Levels)
// ==============================================================================
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeIrql {
    PassiveLevel = 0,     // User mode and normal thread execution
    ApcLevel = 1,         // Asynchronous Procedure Calls execution
    DispatchLevel = 2,    // Thread scheduler & deferred execution (DPCs)
    Dirql = 3,            // Device Interrupt Request Level (hardware drivers)
    HighLevel = 31,       // All interrupts masked/disabled (panic, IPIs)
}

#[repr(C)]
pub struct KeIrqlManager {
    pub current_irql: AtomicUsize,
}

impl KeIrqlManager {
    pub const fn new() -> Self {
        Self { current_irql: AtomicUsize::new(KeIrql::PassiveLevel as usize) }
    }

    pub fn get_irql(&self) -> KeIrql {
        match self.current_irql.load(Ordering::SeqCst) {
            0 => KeIrql::PassiveLevel,
            1 => KeIrql::ApcLevel,
            2 => KeIrql::DispatchLevel,
            3 => KeIrql::Dirql,
            _ => KeIrql::HighLevel,
        }
    }

    pub fn raise_irql(&self, new_irql: KeIrql) -> Result<KeIrql, &'static str> {
        let old_irql = self.get_irql();
        if new_irql < old_irql {
            return Err("KeRaiseIrql: Cannot raise to a lower IRQL level!");
        }
        self.current_irql.store(new_irql as usize, Ordering::SeqCst);
        Ok(old_irql)
    }

    pub fn lower_irql(&self, new_irql: KeIrql) -> Result<KeIrql, &'static str> {
        let old_irql = self.get_irql();
        if new_irql > old_irql {
            return Err("KeLowerIrql: Cannot lower to a higher IRQL level!");
        }
        self.current_irql.store(new_irql as usize, Ordering::SeqCst);
        Ok(old_irql)
    }
}

impl Default for KeIrqlManager {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// Windows NT-style KPCR (Processor Control Region) and KPRCB (Processor Control Block)
// ==============================================================================
#[repr(C)]
pub struct Kpcr {
    pub cpu_id: u32,
    pub current_thread_id: u64,
    pub irql_manager: KeIrqlManager,
    pub dpc_queue: Vec<Dpc>,
    pub apc_queue: Vec<Apc>,
}

impl Kpcr {
    pub fn new(id: u32) -> Self {
        Self {
            cpu_id: id,
            current_thread_id: 0,
            irql_manager: KeIrqlManager::new(),
            dpc_queue: Vec::new(),
            apc_queue: Vec::new(),
        }
    }

    pub fn queue_dpc(&mut self, dpc: Dpc) -> bool {
        self.dpc_queue.push(dpc);
        true
    }

    pub fn queue_apc(&mut self, apc: Apc) -> bool {
        self.apc_queue.push(apc);
        true
    }

    pub fn dispatch_pending_dpcs(&mut self) -> usize {
        let old_irql = self.irql_manager.raise_irql(KeIrql::DispatchLevel).unwrap();
        let mut executed = 0;
        // Executing deferred callbacks at DISPATCH_LEVEL (resembling Linux bottom-halves/softirqs)
        while let Some(dpc) = self.dpc_queue.pop() {
            if let Some(ref routine) = dpc.deferred_routine {
                routine();
                executed += 1;
            }
        }
        let _ = self.irql_manager.lower_irql(old_irql);
        executed
    }

    pub fn dispatch_pending_apcs(&mut self) -> usize {
        let old_irql = self.irql_manager.raise_irql(KeIrql::ApcLevel).unwrap();
        let mut executed = 0;
        // Executing async callbacks in target thread context at APC_LEVEL
        while let Some(apc) = self.apc_queue.pop() {
            if let Some(ref routine) = apc.kernel_routine {
                routine();
                executed += 1;
            }
        }
        let _ = self.irql_manager.lower_irql(old_irql);
        executed
    }
}

// ==============================================================================
// Windows NT-style DPC (Deferred Procedure Call)
// ==============================================================================
pub struct Dpc {
    pub id: u32,
    pub deferred_routine: Option<Box<dyn Fn()>>,
    pub importance: u32, // 0 = Low, 1 = Medium, 2 = High
}

impl Dpc {
    pub fn new(id: u32, routine: Option<Box<dyn Fn()>>) -> Self {
        Self {
            id,
            deferred_routine: routine,
            importance: 1, // Medium priority
        }
    }
}

// ==============================================================================
// Windows NT-style APC (Asynchronous Procedure Call)
// ==============================================================================
pub struct Apc {
    pub id: u32,
    pub target_thread_id: u64,
    pub kernel_routine: Option<Box<dyn Fn()>>,
}

impl Apc {
    pub fn new(id: u32, thread: u64, routine: Option<Box<dyn Fn()>>) -> Self {
        Self {
            id,
            target_thread_id: thread,
            kernel_routine: routine,
        }
    }
}
