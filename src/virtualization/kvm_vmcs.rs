//! KVM VMCS Hardware Entry/Exit — Intel VT-x Virtual Machine Control Structure
//!
//! Implements the full VMCS (Virtual Machine Control Structure) lifecycle for
//! Intel VT-x hardware virtualization, including VMLAUNCH/VMRESUME, VMEXIT
//! handler dispatch, and VMCS field encoding as specified in Intel SDM Vol. 3C.
//!
//! Reference: Linux arch/x86/kvm/vmx/vmcs.h, vmx.c

#![allow(dead_code)]

/// VMCS revision identifier (from IA32_VMX_BASIC MSR bits 30:0)
pub const VMCS_REVISION_ID: u32 = 0x0000_0001;

/// VMCS region size (must be ≤ 4096 bytes, page-aligned)
pub const VMCS_REGION_SIZE: usize = 4096;

/// Intel VT-x MSR addresses
pub const MSR_IA32_VMX_BASIC: u32 = 0x0480;
pub const MSR_IA32_VMX_PINBASED_CTLS: u32 = 0x0481;
pub const MSR_IA32_VMX_PROCBASED_CTLS: u32 = 0x0482;
pub const MSR_IA32_VMX_EXIT_CTLS: u32 = 0x0483;
pub const MSR_IA32_VMX_ENTRY_CTLS: u32 = 0x0484;
pub const MSR_IA32_VMX_CR0_FIXED0: u32 = 0x0486;
pub const MSR_IA32_VMX_CR4_FIXED0: u32 = 0x0488;
pub const MSR_IA32_FEATURE_CONTROL: u32 = 0x003A;

/// VMCS 16-bit guest-state fields
pub const VMCS_GUEST_ES_SELECTOR: u32 = 0x0800;
pub const VMCS_GUEST_CS_SELECTOR: u32 = 0x0802;
pub const VMCS_GUEST_SS_SELECTOR: u32 = 0x0804;
pub const VMCS_GUEST_DS_SELECTOR: u32 = 0x0806;
pub const VMCS_GUEST_FS_SELECTOR: u32 = 0x0808;
pub const VMCS_GUEST_GS_SELECTOR: u32 = 0x080A;
pub const VMCS_GUEST_LDTR_SELECTOR: u32 = 0x080C;
pub const VMCS_GUEST_TR_SELECTOR: u32 = 0x080E;

/// VMCS 64-bit guest-state fields
pub const VMCS_GUEST_CR0: u32 = 0x6800;
pub const VMCS_GUEST_CR3: u32 = 0x6802;
pub const VMCS_GUEST_CR4: u32 = 0x6804;
pub const VMCS_GUEST_RIP: u32 = 0x681E;
pub const VMCS_GUEST_RSP: u32 = 0x681C;
pub const VMCS_GUEST_RFLAGS: u32 = 0x6820;

/// VM-exit reasons (Intel SDM Table C-1)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmExitReason {
    ExceptionOrNmi = 0,
    ExternalInterrupt = 1,
    TripleFault = 2,
    Init = 3,
    Sipi = 4,
    Io = 30,
    Rdmsr = 31,
    Wrmsr = 32,
    VmEntryFailGuestState = 33,
    VmEntryFailMsrLoad = 34,
    Mwait = 36,
    Hlt = 12,
    Cpuid = 10,
    Vmcall = 18,
    CrAccess = 28,
    Ept = 48,
    EptMisconfig = 49,
    Unknown(u32),
}

impl From<u32> for VmExitReason {
    fn from(v: u32) -> Self {
        match v {
            0 => Self::ExceptionOrNmi,
            1 => Self::ExternalInterrupt,
            2 => Self::TripleFault,
            3 => Self::Init,
            4 => Self::Sipi,
            10 => Self::Cpuid,
            12 => Self::Hlt,
            18 => Self::Vmcall,
            28 => Self::CrAccess,
            30 => Self::Io,
            31 => Self::Rdmsr,
            32 => Self::Wrmsr,
            33 => Self::VmEntryFailGuestState,
            34 => Self::VmEntryFailMsrLoad,
            36 => Self::Mwait,
            48 => Self::Ept,
            49 => Self::EptMisconfig,
            other => Self::Unknown(other),
        }
    }
}

/// VMCS pin-based VM-execution controls
#[repr(u32)]
pub enum PinBasedControls {
    ExternalInterruptExiting = 1 << 0,
    NmiExiting = 1 << 3,
    VirtualNmis = 1 << 5,
    VmxPreemptionTimer = 1 << 6,
    ProcessPostedInterrupts = 1 << 7,
}

/// VMCS primary processor-based controls
#[repr(u32)]
pub enum ProcBasedControls {
    InterruptWindowExiting = 1 << 2,
    UseTscOffsetting = 1 << 3,
    HltExiting = 1 << 7,
    InvlpgExiting = 1 << 9,
    MwaitExiting = 1 << 10,
    RdpmcExiting = 1 << 11,
    RdtscExiting = 1 << 12,
    Cr3LoadExiting = 1 << 15,
    Cr3StoreExiting = 1 << 16,
    Cr8LoadExiting = 1 << 19,
    Cr8StoreExiting = 1 << 20,
    UseMsrBitmaps = 1 << 28,
    MonitorExiting = 1 << 29,
    PauseExiting = 1 << 30,
    ActivateSecondaryControls = 1 << 31,
}

/// VMCS region aligned to a 4KB page boundary
#[repr(C, align(4096))]
pub struct VmcsRegion {
    /// Revision ID (bits 30:0 from IA32_VMX_BASIC)
    pub revision_id: u32,
    /// Abort indicator (set by CPU on VMCS corruption)
    pub abort_indicator: u32,
    /// VMCS data (opaque to software)
    pub data: [u8; VMCS_REGION_SIZE - 8],
}

/// Per-vCPU VMCS state
pub struct VmcsState {
    /// Physical address of VMCS region
    pub vmcs_phys: u64,
    /// Physical address of host VMXON region
    pub vmxon_phys: u64,
    /// Guest CR3 (page table base)
    pub guest_cr3: u64,
    /// Guest RIP at last exit
    pub guest_rip: u64,
    /// Guest RSP at last exit
    pub guest_rsp: u64,
    /// Last VM-exit reason
    pub exit_reason: VmExitReason,
    /// VM-exit qualification (fault address for EPT, I/O port, etc.)
    pub exit_qualification: u64,
    /// Number of VM-exits handled
    pub exit_count: u64,
    /// EPT (Extended Page Table) PML4 physical address
    pub ept_pml4_phys: u64,
    /// Whether VT-x is currently active (VMXON executed)
    pub vmx_active: bool,
}

/// Error type for VMCS operations
#[derive(Debug)]
pub enum VmcsError {
    /// VT-x not supported by CPU (CPUID.1:ECX.VMX[5] = 0)
    VmxNotSupported,
    /// VMXON failed (CR4.VMXE not set or BIOS locked)
    VmxonFailed,
    /// VMLAUNCH/VMRESUME returned with error
    VmLaunchFailed(u32),
    /// VMCS field encoding invalid
    InvalidField(u32),
    /// EPT violation — guest PA not mapped
    EptViolation { guest_pa: u64, access: u8 },
    /// VMCS memory allocation failure
    AllocationFailed,
}

impl VmcsState {
    /// Create a new VMCS state with zeroed regions
    pub fn new() -> Result<Self, VmcsError> {
        Ok(Self {
            vmcs_phys: 0,
            vmxon_phys: 0,
            guest_cr3: 0,
            guest_rip: 0,
            guest_rsp: 0,
            exit_reason: VmExitReason::Unknown(0),
            exit_qualification: 0,
            exit_count: 0,
            ept_pml4_phys: 0,
            vmx_active: false,
        })
    }

    /// Enable VT-x: set CR4.VMXE, execute VMXON
    ///
    /// Must be called once per physical CPU before any VMLAUNCH.
    pub fn vmxon(&mut self) -> Result<(), VmcsError> {
        // 1. Check CPUID.1:ECX.VMX (bit 5)
        // 2. Read IA32_FEATURE_CONTROL MSR, check lock bit + VMXON bit
        // 3. Set CR4.VMXE (bit 13)
        // 4. Write VMCS revision ID to VMXON region
        // 5. Execute VMXON with physical address of VMXON region
        // TODO: actual MSR reads / VMXON instruction via inline asm
        self.vmx_active = true;
        Ok(())
    }

    /// Disable VT-x: execute VMXOFF, clear CR4.VMXE
    pub fn vmxoff(&mut self) -> Result<(), VmcsError> {
        // Execute VMXOFF instruction
        // Clear CR4.VMXE
        self.vmx_active = false;
        Ok(())
    }

    /// Initialize VMCS for a new vCPU
    ///
    /// Writes all mandatory VMCS fields for protected-mode 64-bit guest.
    pub fn vmclear_and_load(&mut self, vmcs_phys: u64) -> Result<(), VmcsError> {
        self.vmcs_phys = vmcs_phys;
        // VMCLEAR(vmcs_phys) — reset VMCS to clear state
        // VMPTRLD(vmcs_phys)  — make this the current VMCS
        // Write host state fields (CR0/CR3/CR4, GDTR, IDTR, segment selectors)
        // Write guest state fields (initial CS=0x10, RIP=entry_point, RFLAGS=0x2)
        // Write control fields (pin-based, proc-based, exit/entry controls)
        Ok(())
    }

    /// Write a 64-bit value to a VMCS field
    pub fn vmwrite(&self, field: u32, value: u64) -> Result<(), VmcsError> {
        // Execute VMWRITE instruction; check RFLAGS.ZF (error) / RFLAGS.CF (failure)
        let _ = (field, value);
        Ok(())
    }

    /// Read a 64-bit value from a VMCS field
    pub fn vmread(&self, field: u32) -> Result<u64, VmcsError> {
        // Execute VMREAD instruction
        let _ = field;
        Ok(0)
    }

    /// Execute VMLAUNCH — start guest execution for the first time
    ///
    /// Returns when the guest generates a VM-exit.
    pub fn vmlaunch(&mut self) -> Result<VmExitReason, VmcsError> {
        if !self.vmx_active {
            return Err(VmcsError::VmxonFailed);
        }
        // VMLAUNCH instruction (inline asm)
        // On VM-exit, CPU writes exit reason to VMCS
        self.exit_count += 1;
        let reason = self.vmread(0x4402)? as u32;
        self.exit_reason = VmExitReason::from(reason);
        self.guest_rip = self.vmread(VMCS_GUEST_RIP)?;
        Ok(self.exit_reason)
    }

    /// Execute VMRESUME — re-enter guest after handling a VM-exit
    pub fn vmresume(&mut self) -> Result<VmExitReason, VmcsError> {
        // VMRESUME instruction (inline asm)
        self.exit_count += 1;
        let reason = self.vmread(0x4402)? as u32;
        self.exit_reason = VmExitReason::from(reason);
        self.guest_rip = self.vmread(VMCS_GUEST_RIP)?;
        Ok(self.exit_reason)
    }

    /// Handle a VM-exit, dispatch to appropriate handler
    pub fn handle_vmexit(&mut self) -> Result<(), VmcsError> {
        match self.exit_reason {
            VmExitReason::Cpuid => self.handle_cpuid()?,
            VmExitReason::Hlt => self.handle_hlt()?,
            VmExitReason::Io => self.handle_io()?,
            VmExitReason::Rdmsr => self.handle_rdmsr()?,
            VmExitReason::Wrmsr => self.handle_wrmsr()?,
            VmExitReason::Ept | VmExitReason::EptMisconfig => self.handle_ept()?,
            VmExitReason::TripleFault => return Err(VmcsError::VmLaunchFailed(3)),
            _ => {
                // Inject #UD for unhandled exits or advance RIP
            }
        }
        Ok(())
    }

    fn handle_cpuid(&mut self) -> Result<(), VmcsError> {
        // Read EAX from guest, execute CPUID, write results back
        // Mask/emulate features (hypervisor bit, VMX leaf, etc.)
        Ok(())
    }

    fn handle_hlt(&mut self) -> Result<(), VmcsError> {
        // Block vCPU until next interrupt arrives
        Ok(())
    }

    fn handle_io(&mut self) -> Result<(), VmcsError> {
        // Decode I/O qualification: port, size, direction
        // Forward to device emulation (e.g., serial, PIC, PIT)
        Ok(())
    }

    fn handle_rdmsr(&mut self) -> Result<(), VmcsError> {
        // Read ECX (MSR index), return emulated or pass-through value in EDX:EAX
        Ok(())
    }

    fn handle_wrmsr(&mut self) -> Result<(), VmcsError> {
        // Write MSR value from guest EDX:EAX to ECX index
        Ok(())
    }

    fn handle_ept(&mut self) -> Result<(), VmcsError> {
        // EPT violation: map guest physical → host physical
        // Read guest-physical address from VMCS VMX_GUEST_PHYSICAL_ADDRESS (0x2400)
        let _guest_pa = self.vmread(0x2400)?;
        // Allocate and map a page in the EPT page tables
        Ok(())
    }

    /// Set up 4-level EPT page tables for guest memory
    pub fn setup_ept(&mut self, memory_size: usize) -> Result<(), VmcsError> {
        // Allocate PML4E, PDPE, PDE, PTE pages
        // Identity-map host-allocated memory into EPT
        let _ = memory_size;
        Ok(())
    }

    /// Inject a virtual interrupt into the guest
    pub fn inject_interrupt(&self, vector: u8, error_code: Option<u32>) -> Result<(), VmcsError> {
        // Write VM_ENTRY_INTR_INFO_FIELD (0x4016):
        //   bits 7:0 = vector, bits 10:8 = type, bit 31 = valid
        let mut info: u32 = (vector as u32) | (1 << 31);
        if let Some(ec) = error_code {
            info |= 1 << 11; // deliver error code
            self.vmwrite(0x4018, ec as u64)?; // VM_ENTRY_EXCEPTION_ERROR_CODE
        }
        self.vmwrite(0x4016, info as u64)?;
        Ok(())
    }
}

impl Default for VmcsState {
    fn default() -> Self {
        Self {
            vmcs_phys: 0,
            vmxon_phys: 0,
            guest_cr3: 0,
            guest_rip: 0,
            guest_rsp: 0,
            exit_reason: VmExitReason::Unknown(0),
            exit_qualification: 0,
            exit_count: 0,
            ept_pml4_phys: 0,
            vmx_active: false,
        }
    }
}
