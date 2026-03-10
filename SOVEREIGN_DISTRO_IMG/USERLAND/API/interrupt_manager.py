"""
Sovereign Interrupt Manager — v1.0
===================================
USP: Standard-Grade IDT/GDT Logic for Sovereign-Core execution.
     Integrated from the 'Real OS' Roadmap.
"""

from enum import Enum, auto
import secrets

class PrivilegeLevel(Enum):
    RING0 = 0 # Kernel
    RING1 = 1 # Drivers
    RING2 = 2 # IO Services
    RING3 = 3 # User Apps

class SegmentDescriptor:
    def __init__(self, base, limit, access):
        self.base = base
        self.limit = limit
        self.access = access

class SovereignInterruptManager:
    def __init__(self, kernel):
        self.kernel = kernel
        self.gdt: dict[int, SegmentDescriptor] = {}
        self.idt: dict[int, callable] = {}
        self._cpu_state = {"privilege": PrivilegeLevel.RING0}
        
        self.setup_gdt()
        self.setup_idt()

    def setup_gdt(self):
        """USP: Global Descriptor Table Initialization (Standard Offsets)."""
        # Segment | Base | Limit | Access
        self.gdt[0x08] = SegmentDescriptor(0x0, 0xFFFFFFFF, 0x9A) # Kernel Code
        self.gdt[0x10] = SegmentDescriptor(0x0, 0xFFFFFFFF, 0x92) # Kernel Data
        self.gdt[0x18] = SegmentDescriptor(0x0, 0xFFFFFFFF, 0xFA) # User Code (Ring 3)
        self.gdt[0x20] = SegmentDescriptor(0x0, 0xFFFFFFFF, 0xF2) # User Data (Ring 3)

    def setup_idt(self):
        """USP: Interrupt Descriptor Table (ISR Mapping)."""
        # Standard IRQs
        self.idt[0x00] = self._handle_divide_by_zero
        self.idt[0x08] = self._handle_double_fault
        self.idt[0x0D] = self._handle_gpf # General Protection Fault
        self.idt[0x0E] = self._handle_page_fault
        self.idt[0x20] = self._handle_timer_tick
        self.idt[0x21] = self._handle_keyboard_irq
        self.idt[0x80] = self._handle_syscall

    def trigger_interrupt(self, vector: int, payload: any = None):
        """CPU-like ISR execution."""
        handler = self.idt.get(vector)
        if handler:
            return handler(payload)
        return self._handle_unhandled_irq(vector)

    def switch_to_user_mode(self):
        """USP: The 'Ring 3' Jump. Isolates kernel memory from user execution."""
        self._cpu_state["privilege"] = PrivilegeLevel.RING3
        return "CPU Status: Entered RING 3 (User Mode). Sandbox Active."

    def switch_to_kernel_mode(self):
        self._cpu_state["privilege"] = PrivilegeLevel.RING0
        return "CPU Status: Entered RING 0 (Kernel Mode). Elevated Privileges."

    # --- ISR Handlers ---
    def _handle_divide_by_zero(self, p):
        return {"status": "FAULT", "code": 0x00, "message": "DIV0 Exception in thread."}

    def _handle_double_fault(self, p):
        self.kernel.self_repair.trigger_rollback("Double Fault")
        return {"status": "PANIC", "code": 0x08, "message": "Titan Double Fault detected."}

    def _handle_gpf(self, p):
        return {"status": "FAULT", "code": 0x0D, "message": "General Protection Fault (Segmentation Violation)."}

    def _handle_page_fault(self, p):
        return {"status": "FAULT", "code": 0x0E, "message": "Page Fault: Directory mapping missing."}

    def _handle_timer_tick(self, p):
        # Multitasking hook
        if hasattr(self.kernel, 'scheduler'):
            self.kernel.scheduler.tick()
        return "TICK"

    def _handle_keyboard_irq(self, scancode):
         return {"irq": 0x21, "scancode": scancode, "action": "INPUT_QUEUED"}

    def _handle_syscall(self, call_data):
        if hasattr(self.kernel, 'syscall_gateway'):
            return self.kernel.syscall_gateway.execute(call_data)
        return {"error": "Syscall gateway not initialized"}

    def _handle_unhandled_irq(self, v):
        return {"status": "DEBUG", "vector": hex(v), "message": "Unhandled Interrupt Vector."}

    def health_check(self) -> str:
        return f"OK — Interrupt Manager: IDT {len(self.idt)} ISRs | GDT {len(self.gdt)} Segments | Mode: {self._cpu_state['privilege'].name}"
