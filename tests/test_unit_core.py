"""
SigmaOS Core Unit Test Suite
Validates individual system modules: File I/O, Process Scheduling, and Memory Management.
"""

import time
import math


# --- Mock / Simulation Classes for File I/O ---
class SimulatedVFS:
    def __init__(self):
        self.files = {}

    def create_file(self, filename: str) -> bool:
        if filename in self.files:
            return False
        self.files[filename] = bytearray()
        return True

    def write_file(self, filename: str, content: bytes) -> int:
        if filename not in self.files:
            raise FileNotFoundError(f"File {filename} does not exist.")
        self.files[filename].extend(content)
        return len(content)

    def read_file(self, filename: str) -> bytes:
        if filename not in self.files:
            raise FileNotFoundError(f"File {filename} does not exist.")
        return bytes(self.files[filename])

    def delete_file(self, filename: str) -> bool:
        if filename in self.files:
            del self.files[filename]
            return True
        return False


# --- Mock / Simulation Classes for Process Scheduler ---
class Process:
    def __init__(self, pid: int, priority: int = 1):
        self.pid = pid
        self.priority = priority  # Higher = higher priority
        self.cpu_time_allocated = 0
        self.executed_cycles = 0

class RoundRobinScheduler:
    def __init__(self, time_slice_ms: int = 10):
        self.time_slice_ms = time_slice_ms
        self.queue = []

    def add_process(self, proc: Process):
        self.queue.append(proc)

    def schedule_cycles(self, total_cycles: int):
        if not self.queue:
            return
        for _ in range(total_cycles):
            for proc in self.queue:
                cycles = proc.priority * self.time_slice_ms
                proc.cpu_time_allocated += cycles
                proc.executed_cycles += 1


# --- Mock / Simulation Classes for Memory Manager ---
class MemoryBlock:
    def __init__(self, address: int, size: int):
        self.address = address
        self.size = size
        self.is_free = True

class HeapManager:
    def __init__(self, total_size: int = 1024 * 1024):
        self.total_size = total_size
        self.blocks = [MemoryBlock(0, total_size)]
        self.allocated_map = {}

    def allocate(self, size: int) -> int:
        for block in self.blocks:
            if block.is_free and block.size >= size:
                block.is_free = False
                if block.size > size:
                    remaining_size = block.size - size
                    new_addr = block.address + size
                    block.size = size
                    new_block = MemoryBlock(new_addr, remaining_size)
                    idx = self.blocks.index(block)
                    self.blocks.insert(idx + 1, new_block)
                self.allocated_map[block.address] = size
                return block.address
        raise MemoryError("Out of memory or heap fragmentation failure")

    def free(self, address: int) -> bool:
        if address not in self.allocated_map:
            return False
        for block in self.blocks:
            if block.address == address and not block.is_free:
                block.is_free = True
                del self.allocated_map[address]
                self._coalesce()
                return True
        return False

    def _coalesce(self):
        i = 0
        while i < len(self.blocks) - 1:
            current = self.blocks[i]
            next_block = self.blocks[i + 1]
            if current.is_free and next_block.is_free:
                current.size += next_block.size
                self.blocks.pop(i + 1)
            else:
                i += 1

    def get_fragmentation_ratio(self) -> float:
        free_blocks = [b.size for b in self.blocks if b.is_free]
        if not free_blocks:
            return 0.0
        max_free = max(free_blocks)
        total_free = sum(free_blocks)
        if total_free == 0:
            return 0.0
        return 1.0 - (max_free / total_free)


# --- Unit Test Cases ---

def test_file_io_operations():
    vfs = SimulatedVFS()
    filename = "/etc/sigma_config.json"
    content = b'{"hostname": "sigma-node-1", "mode": "sovereign"}'

    assert vfs.create_file(filename) is True
    assert vfs.create_file(filename) is False  # Already exists

    written = vfs.write_file(filename, content)
    assert written == len(content)

    read_data = vfs.read_file(filename)
    assert read_data == content

    assert vfs.delete_file(filename) is True
    assert vfs.delete_file(filename) is False
    try:
        vfs.read_file(filename)
        assert False, "Expected FileNotFoundError"
    except FileNotFoundError:
        pass


def test_process_scheduling_fairness():
    scheduler = RoundRobinScheduler(time_slice_ms=5)
    p1 = Process(pid=101, priority=1)
    p2 = Process(pid=102, priority=1)
    p3 = Process(pid=103, priority=2)

    scheduler.add_process(p1)
    scheduler.add_process(p2)
    scheduler.add_process(p3)

    scheduler.schedule_cycles(100)

    # Ensure no process suffered starvation
    assert p1.executed_cycles == 100
    assert p2.executed_cycles == 100
    assert p3.executed_cycles == 100

    # Higher priority process received proportional CPU time allocation
    assert p3.cpu_time_allocated == 2 * p1.cpu_time_allocated
    assert p1.cpu_time_allocated == p2.cpu_time_allocated


def test_memory_management_alloc_free_leak():
    heap = HeapManager(total_size=4096)
    allocations = []

    # Allocate multiple chunks repeatedly
    for i in range(10):
        addr = heap.allocate(128)
        allocations.append(addr)

    assert len(heap.allocated_map) == 10

    # Free half of allocations
    for addr in allocations[::2]:
        assert heap.free(addr) is True

    assert len(heap.allocated_map) == 5

    # Free remaining allocations
    for addr in allocations[1::2]:
        assert heap.free(addr) is True

    assert len(heap.allocated_map) == 0  # No memory leaks
    assert len(heap.blocks) == 1  # Fully coalesced into single free block
    assert heap.blocks[0].size == 4096
    assert heap.get_fragmentation_ratio() == 0.0
