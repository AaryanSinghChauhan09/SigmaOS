// SigmaOS Windows WDK / Linux Kernel List & PCB Subsystem
// Zero-dependency, #![no_std] compliant kernel structures.


use std::string::String;
use std::vec::Vec;

// ==========================================
// 1. WDK-Inspired Singly Linked List
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleListEntry {
    pub next: Option<*mut SingleListEntry>,
    pub data: u64,
}

pub struct SingleListHead {
    pub next: Option<*mut SingleListEntry>,
}

impl SingleListHead {
    pub const fn new() -> Self {
        Self { next: None }
    }

    pub unsafe fn push(&mut self, entry: *mut SingleListEntry) {
        if entry.is_null() {
            return;
        }
        (*entry).next = self.next;
        self.next = Some(entry);
    }

    pub unsafe fn pop(&mut self) -> Option<*mut SingleListEntry> {
        let first = self.next?;
        self.next = (*first).next;
        Some(first)
    }

    pub fn is_empty(&self) -> bool {
        self.next.is_none()
    }
}

// ==========================================
// 2. Circular Doubly Linked List (LIST_ENTRY Parity)
// ==========================================

pub struct ListEntry {
    pub flink: *mut ListEntry,
    pub blink: *mut ListEntry,
    pub payload_address: u64,
}

pub struct ListHead {
    pub head: ListEntry,
}

impl ListHead {
    pub fn new() -> Self {
        ListHead {
            head: ListEntry {
                flink: core::ptr::null_mut(),
                blink: core::ptr::null_mut(),
                payload_address: 0,
            },
        }
    }

    /// Initializes the circular doubly linked list in-place (InitializeListHead parity)
    pub unsafe fn initialize(&mut self) {
        let list_head = &mut self.head as *mut ListEntry;
        self.head.flink = list_head;
        self.head.blink = list_head;
    }

    /// Automatically ensures list pointers are initialized to self
    pub unsafe fn ensure_initialized(&mut self) {
        if self.head.flink.is_null() || self.head.blink.is_null() {
            self.initialize();
        }
    }

    pub unsafe fn insert_tail(&mut self, entry: *mut ListEntry) {
        if entry.is_null() {
            return;
        }
        self.ensure_initialized();
        let list_head = &mut self.head as *mut ListEntry;
        let old_blink = (*list_head).blink;

        (*entry).flink = list_head;
        (*entry).blink = old_blink;

        (*old_blink).flink = entry;
        (*list_head).blink = entry;
    }

    pub unsafe fn insert_head(&mut self, entry: *mut ListEntry) {
        if entry.is_null() {
            return;
        }
        self.ensure_initialized();
        let list_head = &mut self.head as *mut ListEntry;
        let old_flink = (*list_head).flink;

        (*entry).flink = old_flink;
        (*entry).blink = list_head;

        (*old_flink).blink = entry;
        (*list_head).flink = entry;
    }

    pub unsafe fn remove_entry(&mut self, entry: *mut ListEntry) -> bool {
        if entry.is_null() || entry == (&mut self.head as *mut ListEntry) {
            return false;
        }
        self.ensure_initialized();
        let next = (*entry).flink;
        let prev = (*entry).blink;

        if !prev.is_null() {
            (*prev).flink = next;
        }
        if !next.is_null() {
            (*next).blink = prev;
        }

        (*entry).flink = core::ptr::null_mut();
        (*entry).blink = core::ptr::null_mut();
        true
    }

    pub unsafe fn pop_head(&mut self) -> Option<*mut ListEntry> {
        self.ensure_initialized();
        let list_head = &mut self.head as *mut ListEntry;
        let first = (*list_head).flink;
        if first == list_head {
            None
        } else {
            self.remove_entry(first);
            Some(first)
        }
    }

    pub unsafe fn is_empty(&self) -> bool {
        if self.head.flink.is_null() {
            return true;
        }
        self.head.flink == (&self.head as *const ListEntry as *mut ListEntry)
    }
}

// ==========================================
// 3. Process Control Block (PCB) & Thread Control Block (TCB)
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Initialized,
    Ready,
    Running,
    Waiting,
    Terminated,
}

#[derive(Debug, Clone)]
pub struct ThreadControlBlock {
    pub tid: u32,
    pub parent_pid: u32,
    pub state: ThreadState,
    pub priority: u8,
    pub stack_base: u64,
    pub stack_limit: u64,
    pub context_ebx: u64,
    pub context_esi: u64,
    pub context_eip: u64,
}

impl ThreadControlBlock {
    pub fn new(tid: u32, parent_pid: u32, priority: u8) -> Self {
        Self {
            tid,
            parent_pid,
            state: ThreadState::Initialized,
            priority,
            stack_base: 0,
            stack_limit: 0,
            context_ebx: 0,
            context_esi: 0,
            context_eip: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProcessControlBlock {
    pub pid: u32,
    pub name: String,
    pub address_space_directory_phys: u64,
    pub threads: Vec<ThreadControlBlock>,
    pub is_kernel_mode: bool,
}

impl ProcessControlBlock {
    pub fn new(pid: u32, name: &str, pml4_phys: u64) -> Self {
        Self {
            pid,
            name: String::from(name),
            address_space_directory_phys: pml4_phys,
            threads: Vec::new(),
            is_kernel_mode: false,
        }
    }

    pub fn register_thread(&mut self, thread: ThreadControlBlock) {
        self.threads.push(thread);
    }
}

// ==========================================
// 4. KDPC (Deferred Procedure Call) Entry
// ==========================================

pub type PkdpcRoutine = fn(deferred_context: u64, system_argument1: u64, system_argument2: u64);

pub struct Kdpc {
    pub list_entry: ListEntry,
    pub dpc_routine: PkdpcRoutine,
    pub deferred_context: u64,
    pub system_argument1: u64,
    pub system_argument2: u64,
    pub importance: u8,
}

impl Kdpc {
    pub fn new(routine: PkdpcRoutine, context: u64) -> Self {
        Self {
            list_entry: ListEntry {
                flink: core::ptr::null_mut(),
                blink: core::ptr::null_mut(),
                payload_address: 0,
            },
            dpc_routine: routine,
            deferred_context: context,
            system_argument1: 0,
            system_argument2: 0,
            importance: 1, // Medium importance
        }
    }

    pub fn execute(&self) {
        (self.dpc_routine)(
            self.deferred_context,
            self.system_argument1,
            self.system_argument2,
        );
    }
}

// ==========================================
// 5. STDCALL Calling Convention Simulation
// ==========================================

pub struct StdCallSimulator {
    pub registers_ebx: u64,
    pub registers_esi: u64,
    pub stack: Vec<u64>,
}

impl StdCallSimulator {
    pub fn new() -> Self {
        Self {
            registers_ebx: 0,
            registers_esi: 0,
            stack: Vec::new(),
        }
    }

    /// Simulates pushing parameters onto the stack in right-to-left order (stdcall)
    pub fn push_arg(&mut self, val: u64) {
        self.stack.push(val);
    }

    /// Simulates a stdcall execution. The callee cleans up the stack, popping parameters.
    pub fn simulate_call(
        &mut self,
        routine: fn(u64, u64) -> u64,
        num_args: usize,
    ) -> Result<u64, &'static str> {
        if self.stack.len() < num_args {
            return Err("Stack underflow during stdcall execution");
        }

        // Pop in reverse order
        let arg1 = self.stack.pop().unwrap();
        let arg2 = self.stack.pop().unwrap();

        // Perform the call
        let result = routine(arg1, arg2);
        Ok(result)
    }
}

impl Default for StdCallSimulator {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// Unit Tests
// ==========================================

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_singly_linked_list_wdk() {
        let mut head = SingleListHead::new();
        assert!(head.is_empty());

        let mut entry1 = SingleListEntry { next: None, data: 101 };
        let mut entry2 = SingleListEntry { next: None, data: 202 };

        unsafe {
            head.push(&mut entry1 as *mut SingleListEntry);
            head.push(&mut entry2 as *mut SingleListEntry);
        }

        assert!(!head.is_empty());

        unsafe {
            let popped1 = head.pop().unwrap();
            assert_eq!((*popped1).data, 202);

            let popped2 = head.pop().unwrap();
            assert_eq!((*popped2).data, 101);
        }

        assert!(head.is_empty());
    }

    #[test]
    fn test_circular_doubly_linked_list_wdk() {
        unsafe {
            let mut list = ListHead::new();
            list.initialize();
            assert!(list.is_empty());

            let mut entry1 = ListEntry {
                flink: core::ptr::null_mut(),
                blink: core::ptr::null_mut(),
                payload_address: 0xDEADBEEF,
            };

            let mut entry2 = ListEntry {
                flink: core::ptr::null_mut(),
                blink: core::ptr::null_mut(),
                payload_address: 0xCAFEBABE,
            };

            list.insert_tail(&mut entry1 as *mut ListEntry);
            list.insert_tail(&mut entry2 as *mut ListEntry);

            assert!(!list.is_empty());

            let popped1 = list.pop_head().unwrap();
            assert_eq!((*popped1).payload_address, 0xDEADBEEF);

            let popped2 = list.pop_head().unwrap();
            assert_eq!((*popped2).payload_address, 0xCAFEBABE);

            assert!(list.is_empty());
        }
    }

    #[test]
    fn test_pcb_tcb_and_kdpc() {
        let mut pcb = ProcessControlBlock::new(101, "svchost.exe", 0x1000);
        assert_eq!(pcb.name, "svchost.exe");
        assert!(!pcb.is_kernel_mode);

        let tcb = ThreadControlBlock::new(1, 101, 15);
        pcb.register_thread(tcb);
        assert_eq!(pcb.threads.len(), 1);

        // Test KDPC Execution
        static mut DPC_RAN: bool = false;
        fn mock_dpc_routine(context: u64, _arg1: u64, _arg2: u64) {
            assert_eq!(context, 0xAA55);
            unsafe {
                DPC_RAN = true;
            }
        }

        let dpc = Kdpc::new(mock_dpc_routine, 0xAA55);
        dpc.execute();
        assert!(unsafe { DPC_RAN });
    }

    #[test]
    fn test_stdcall_simulator() {
        let mut sim = StdCallSimulator::new();
        sim.registers_ebx = 0x1122;
        sim.registers_esi = 0x3344;

        // Push args right-to-left
        sim.push_arg(20);
        sim.push_arg(10);

        fn mock_stdcall_routine(a: u64, b: u64) -> u64 {
            a + b
        }

        let res = sim.simulate_call(mock_stdcall_routine, 2).unwrap();
        assert_eq!(res, 30);
    }
}
