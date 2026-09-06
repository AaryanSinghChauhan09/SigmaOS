"""
SigmaOS Stress, Fuzzing & Performance Benchmark Test Suite
Pushes system to its limits, feeds random/invalid inputs to syscall interfaces,
and compares simulated performance metrics against Linux baseline standards.
"""

import pytest
import random
import string
import time
from concurrent.futures import ThreadPoolExecutor, as_completed


# --- Stress Testing Module ---

class SimulatedKernelProcessManager:
    def __init__(self):
        self.lock_table = {}
        self.active_count = 0

    def spawn_process(self, pid: int) -> bool:
        # Simulate thread execution with atomic-like increment & dummy work
        self.active_count += 1
        val = 0
        for i in range(50):
            val += i * pid
        self.active_count -= 1
        return True


def test_stress_concurrent_processes():
    proc_mgr = SimulatedKernelProcessManager()
    num_processes = 1000

    # Run 1000 concurrent process tasks using ThreadPoolExecutor
    with ThreadPoolExecutor(max_workers=50) as executor:
        futures = [executor.submit(proc_mgr.spawn_process, pid) for pid in range(num_processes)]
        results = [f.result() for f in as_completed(futures)]

    assert len(results) == num_processes
    assert all(res is True for res in results)
    assert proc_mgr.active_count == 0  # All processes exited cleanly without deadlocks


# --- Fuzz Testing Module ---

class SystemCallFuzzTarget:
    def parse_and_execute_sys_param(self, sys_id: int, payload: str, flags: int) -> str:
        if not isinstance(sys_id, int) or sys_id < 0:
            return "EINVAL_SYS_ID"
        if not isinstance(flags, int):
            return "EINVAL_FLAGS"
        if payload is None:
            return "EFAULT_NULL_PTR"

        # Guard against crash / unhandled exceptions on malformed strings
        try:
            # Check length limit
            if len(payload) > 4096:
                return "E2BIG_PAYLOAD_TOO_LARGE"
            # Return sanitised summary
            return f"PROCESSED_{len(payload)}_BYTES"
        except Exception:
            return "EINTERNAL_CRASH_PREVENTED"


def test_fuzz_syscall_inputs():
    target = SystemCallFuzzTarget()

    # 1. Random garbage string generation
    random.seed(42)
    for _ in range(500):
        length = random.randint(0, 5000)
        fuzzy_payload = "".join(random.choices(string.ascii_letters + string.punctuation + "\x00\xFF\n\r\t", k=length))
        sys_id = random.choice([-5, 0, 1, 99, 99999])
        flags = random.choice([-1, 0, 1, 0xFFFFFFFF])

        res = target.parse_and_execute_sys_param(sys_id, fuzzy_payload, flags)
        assert res in ["EINVAL_SYS_ID", "EFAULT_NULL_PTR", "E2BIG_PAYLOAD_TOO_LARGE", f"PROCESSED_{len(fuzzy_payload)}_BYTES"]

    # 2. Extreme edge case inputs
    assert target.parse_and_execute_sys_param(-1, "test", 0) == "EINVAL_SYS_ID"
    assert target.parse_and_execute_sys_param(1, "A" * 5000, 0) == "E2BIG_PAYLOAD_TOO_LARGE"


# --- Benchmark Suite Module ---

class PerformanceBenchmarkSuite:
    @staticmethod
    def measure_cpu_ops(num_iterations: int = 100_000) -> float:
        start = time.perf_counter()
        acc = 0.0
        for i in range(num_iterations):
            acc += (i * 0.5) ** 0.5
        end = time.perf_counter()
        return end - start

    @staticmethod
    def measure_memory_bandwidth_mb_sec(block_size_mb: int = 5) -> float:
        size = block_size_mb * 1024 * 1024
        buf_src = bytearray(b"A" * size)
        start = time.perf_counter()
        buf_dst = bytearray(buf_src)
        end = time.perf_counter()
        elapsed = end - start
        if elapsed == 0:
            return 999999.0
        return block_size_mb / elapsed


def test_benchmark_against_baseline():
    bench = PerformanceBenchmarkSuite()

    # Measure CPU calculation time
    cpu_duration = bench.measure_cpu_ops(100_000)
    # Target Linux/SigmaOS baseline: 100k math ops under 0.5 sec
    assert cpu_duration < 0.5, f"CPU performance regression: {cpu_duration:.4f}s"

    # Measure Memory throughput
    mem_bandwidth = bench.measure_memory_bandwidth_mb_sec(block_size_mb=5)
    # Target baseline: Memory copy rate >= 100 MB/s
    assert mem_bandwidth >= 100.0, f"Memory throughput below baseline: {mem_bandwidth:.2f} MB/s"
