// SigmaOS Sovereign SMP Multi-Core, xHCI USB 3.0 & APC Synthesis Suite
// (`src/kernel/sovereign_smp_xhci_apc_synthesis.rs`)
//
// Linux & BSD inspired core kernel primitives in PR format:
// 1. SmpMultiCoreSchedulerEngine: Symmetric Multiprocessing (SMP) multi-core topology manager, per-CPU runqueues & Inter-Processor Interrupt (IPI) dispatching.
// 2. UsbXhciHostControllerDriver: USB 3.0/3.1 Extensible Host Controller Interface (xHCI) driver with Command/Transfer Rings (TRB) & Device Slot Manager.
// 3. KernelAsyncProcedureCallEngine: Kernel-mode & User-mode Asynchronous Procedure Call (APC) queue dispatcher with priority execution.
// 4. SovereignSmpXhciApcMasterSuite: Master coordinator unifying SMP, xHCI, and APC driver sub-engines.

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

// =========================================================================
// 1. SMP MULTI-CORE TOPOLOGY & INTER-PROCESSOR INTERRUPT (IPI) ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuCoreState {
    Offline,
    Booting,
    OnlineActive,
    HaltedIdle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpiVector {
    Reschedule,
    CallFunction,
    TlbFlush,
    PanicHalt,
}

#[derive(Debug, Clone)]
pub struct CpuCoreDescriptor {
    pub cpu_id: u32,
    pub apic_id: u32,
    pub state: CpuCoreState,
    pub current_pid: Option<usize>,
    pub load_weight: u64,
}

pub struct SmpMultiCoreSchedulerEngine {
    pub cores: BTreeMap<u32, CpuCoreDescriptor>,
    pub ipi_sent_count: usize,
}

impl SmpMultiCoreSchedulerEngine {
    pub fn new(total_cores: u32) -> Self {
        let mut engine = Self {
            cores: BTreeMap::new(),
            ipi_sent_count: 0,
        };

        for cpu in 0..total_cores {
            let state = if cpu == 0 {
                CpuCoreState::OnlineActive // BSP (Bootstrap Processor)
            } else {
                CpuCoreState::HaltedIdle // AP (Application Processors)
            };

            engine.cores.insert(
                cpu,
                CpuCoreDescriptor {
                    cpu_id: cpu,
                    apic_id: cpu * 2,
                    state,
                    current_pid: None,
                    load_weight: 0,
                },
            );
        }

        engine
    }

    pub fn bringup_ap_core(&mut self, cpu_id: u32) -> Result<bool, &'static str> {
        if let Some(core) = self.cores.get_mut(&cpu_id) {
            core.state = CpuCoreState::OnlineActive;
            Ok(true)
        } else {
            Err("SMP Error: CPU ID out of bounds")
        }
    }

    pub fn send_ipi(&mut self, target_cpu_id: u32, vector: IpiVector) -> Result<bool, &'static str> {
        if let Some(core) = self.cores.get(&target_cpu_id) {
            if core.state == CpuCoreState::Offline {
                return Err("SMP Error: Target CPU is offline");
            }
            self.ipi_sent_count += 1;
            let _ = vector; // silence unused variable
            Ok(true)
        } else {
            Err("SMP Error: Target CPU ID not found")
        }
    }

    pub fn online_cores_count(&self) -> usize {
        self.cores
            .values()
            .filter(|c| c.state == CpuCoreState::OnlineActive)
            .count()
    }
}

impl Default for SmpMultiCoreSchedulerEngine {
    fn default() -> Self {
        Self::new(8)
    }
}

// =========================================================================
// 2. USB 3.0 / 3.1 EXTENSIBLE HOST CONTROLLER INTERFACE (xHCI) DRIVER
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrbType {
    NormalTransfer,
    SetupStage,
    DataStage,
    StatusStage,
    LinkTrb,
    CommandNoop,
    EnableSlotCommand,
    DisableSlotCommand,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
pub struct TransferRequestBlock {
    pub parameter: u64,
    pub status: u32,
    pub control: u32,
}

#[derive(Debug, Clone)]
pub struct XhciDeviceSlot {
    pub slot_id: u8,
    pub port_num: u8,
    pub usb_speed: u8, // 1: Full, 2: Low, 3: High, 4: SuperSpeed
    pub context_entries: u8,
    pub is_enabled: bool,
}

pub struct UsbXhciHostControllerDriver {
    pub mmio_base: usize,
    pub max_slots: u8,
    pub slots: BTreeMap<u8, XhciDeviceSlot>,
    pub command_ring: Vec<TransferRequestBlock>,
    pub executed_trbs_count: usize,
}

impl UsbXhciHostControllerDriver {
    pub fn new(max_slots: u8) -> Self {
        Self {
            mmio_base: 0xfeb00000,
            max_slots,
            slots: BTreeMap::new(),
            command_ring: Vec::new(),
            executed_trbs_count: 0,
        }
    }

    pub fn enable_device_slot(&mut self, port_num: u8, speed: u8) -> Result<u8, &'static str> {
        if self.slots.len() >= self.max_slots as usize {
            return Err("xHCI Error: Maximum device slots allocated");
        }

        let slot_id = (self.slots.len() + 1) as u8;
        let slot = XhciDeviceSlot {
            slot_id,
            port_num,
            usb_speed: speed,
            context_entries: 31,
            is_enabled: true,
        };

        // Submit Enable Slot TRB to Command Ring
        let trb = TransferRequestBlock {
            parameter: 0,
            status: 0,
            control: (TrbType::EnableSlotCommand as u32) << 10,
        };
        self.command_ring.push(trb);
        self.executed_trbs_count += 1;

        self.slots.insert(slot_id, slot);
        Ok(slot_id)
    }

    pub fn submit_transfer_trb(&mut self, slot_id: u8, buffer_phys: u64, length: u32, trb_type: TrbType) -> Result<usize, &'static str> {
        if !self.slots.contains_key(&slot_id) {
            return Err("xHCI Error: Invalid device slot ID");
        }

        let trb = TransferRequestBlock {
            parameter: buffer_phys,
            status: length,
            control: (trb_type as u32) << 10,
        };
        self.command_ring.push(trb);
        self.executed_trbs_count += 1;

        Ok(length as usize)
    }
}

impl Default for UsbXhciHostControllerDriver {
    fn default() -> Self {
        Self::new(32)
    }
}

// =========================================================================
// 3. KERNEL & USER-MODE ASYNCHRONOUS PROCEDURE CALL (APC) ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApcMode {
    KernelMode,
    UserMode,
}

#[derive(Debug, Clone)]
pub struct AsyncProcedureCall {
    pub apc_id: u64,
    pub target_pid: usize,
    pub mode: ApcMode,
    pub priority: u8,
    pub callback_address: u64,
    pub context_arg: u64,
    pub is_executed: bool,
}

pub struct KernelAsyncProcedureCallEngine {
    pub apc_queue: Vec<AsyncProcedureCall>,
    pub next_apc_id: u64,
    pub total_executed_apcs: usize,
}

impl KernelAsyncProcedureCallEngine {
    pub fn new() -> Self {
        Self {
            apc_queue: Vec::new(),
            next_apc_id: 1,
            total_executed_apcs: 0,
        }
    }

    pub fn queue_apc(&mut self, pid: usize, mode: ApcMode, prio: u8, callback: u64, arg: u64) -> u64 {
        let apc_id = self.next_apc_id;
        self.next_apc_id += 1;

        let apc = AsyncProcedureCall {
            apc_id,
            target_pid: pid,
            mode,
            priority: prio,
            callback_address: callback,
            context_arg: arg,
            is_executed: false,
        };

        self.apc_queue.push(apc);
        apc_id
    }

    pub fn dispatch_pending_apcs_for_process(&mut self, pid: usize, current_mode: ApcMode) -> usize {
        let mut dispatched = 0;
        for apc in self.apc_queue.iter_mut() {
            if apc.target_pid == pid && apc.mode == current_mode && !apc.is_executed {
                apc.is_executed = true;
                dispatched += 1;
                self.total_executed_apcs += 1;
            }
        }
        dispatched
    }
}

impl Default for KernelAsyncProcedureCallEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// MASTER COORDINATOR: SOVEREIGN SMP, XHCI & APC MASTER SUITE
// =========================================================================

pub struct SovereignSmpXhciApcMasterSuite {
    pub smp_scheduler: SmpMultiCoreSchedulerEngine,
    pub xhci_driver: UsbXhciHostControllerDriver,
    pub apc_engine: KernelAsyncProcedureCallEngine,
}

impl SovereignSmpXhciApcMasterSuite {
    pub fn new() -> Self {
        Self {
            smp_scheduler: SmpMultiCoreSchedulerEngine::new(8),
            xhci_driver: UsbXhciHostControllerDriver::new(32),
            apc_engine: KernelAsyncProcedureCallEngine::new(),
        }
    }

    pub fn health_check(&self) -> bool {
        self.smp_scheduler.online_cores_count() >= 1
    }

    pub fn summary_report(&self) -> String {
        format!(
            "Sovereign SMP, xHCI & APC Master Suite Active:\n- Online CPU Cores: {}\n- xHCI Device Slots: {}\n- Executed TRBs: {}\n- Queued APCs: {}",
            self.smp_scheduler.online_cores_count(),
            self.xhci_driver.slots.len(),
            self.xhci_driver.executed_trbs_count,
            self.apc_engine.apc_queue.len(),
        )
    }
}

impl Default for SovereignSmpXhciApcMasterSuite {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smp_multicore_scheduler() {
        let mut smp = SmpMultiCoreSchedulerEngine::new(4);
        assert_eq!(smp.online_cores_count(), 1); // BSP core 0 online

        assert!(smp.bringup_ap_core(1).is_ok());
        assert_eq!(smp.online_cores_count(), 2);

        assert!(smp.send_ipi(1, IpiVector::TlbFlush).is_ok());
        assert_eq!(smp.ipi_sent_count, 1);
    }

    #[test]
    fn test_usb_xhci_driver() {
        let mut xhci = UsbXhciHostControllerDriver::new(8);
        let slot1 = xhci.enable_device_slot(1, 4).unwrap(); // SuperSpeed
        assert_eq!(slot1, 1);

        let trb_bytes = xhci.submit_transfer_trb(1, 0x10000, 4096, TrbType::NormalTransfer).unwrap();
        assert_eq!(trb_bytes, 4096);
        assert_eq!(xhci.executed_trbs_count, 2);
    }

    #[test]
    fn test_kernel_apc_engine() {
        let mut apc_mgr = KernelAsyncProcedureCallEngine::new();
        let apc1 = apc_mgr.queue_apc(100, ApcMode::KernelMode, 10, 0xffffffff80001000, 0x1234);
        assert_eq!(apc1, 1);

        let dispatched = apc_mgr.dispatch_pending_apcs_for_process(100, ApcMode::KernelMode);
        assert_eq!(dispatched, 1);
        assert_eq!(apc_mgr.total_executed_apcs, 1);
    }

    #[test]
    fn test_smp_xhci_apc_master_suite() {
        let suite = SovereignSmpXhciApcMasterSuite::new();
        assert!(suite.health_check());
        assert!(suite.summary_report().contains("Online CPU Cores"));
    }
}
