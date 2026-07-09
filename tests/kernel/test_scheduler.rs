// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// tests/kernel/test_scheduler.rs

#![no_std]
#![no_main]

type SigmaU8 = u8;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaBool = bool;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TestResult {
    Pass = 0,
    Fail = 1,
    Skip = 2,
    Error = 3,
}

// Mock thread structure for testing
#[repr(C)]
pub struct MockThread {
    pub id: SigmaU32,
    pub priority: SigmaU32,
    pub state: SigmaU32,
    pub cpu_time: SigmaU64,
}

// Mock scheduler state
static mut THREAD_COUNT: SigmaU32 = 0;
static mut CURRENT_THREAD: SigmaU32 = 0;
static mut RUNQUEUE: [MockThread; 64] = [MockThread {
    id: 0,
    priority: 0,
    state: 0,
    cpu_time: 0,
}; 64];

/// Initialize mock scheduler
unsafe fn init_scheduler() {
    THREAD_COUNT = 0;
    CURRENT_THREAD = 0;
    for i in 0..64 {
        RUNQUEUE[i] = MockThread {
            id: 0,
            priority: 0,
            state: 0,
            cpu_time: 0,
        };
    }
}

/// Add mock thread to runqueue
unsafe fn add_thread(id: SigmaU32, priority: SigmaU32) -> SigmaI32 {
    if THREAD_COUNT >= 64 {
        return -1;
    }

    RUNQUEUE[THREAD_COUNT as usize] = MockThread {
        id,
        priority,
        state: 1, // Ready
        cpu_time: 0,
    };

    THREAD_COUNT += 1;
    0
}

/// Test: Thread creation and addition to runqueue
unsafe fn test_thread_creation() -> TestResult {
    init_scheduler();

    // Add threads with different priorities
    if add_thread(1, 10) != 0 {
        return TestResult::Fail;
    }
    if add_thread(2, 5) != 0 {
        return TestResult::Fail;
    }
    if add_thread(3, 15) != 0 {
        return TestResult::Fail;
    }

    // Verify thread count
    if THREAD_COUNT != 3 {
        return TestResult::Fail;
    }

    // Verify thread IDs
    if RUNQUEUE[0].id != 1 || RUNQUEUE[1].id != 2 || RUNQUEUE[2].id != 3 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: Priority-based scheduling
unsafe fn test_priority_scheduling() -> TestResult {
    init_scheduler();

    // Add threads with different priorities
    add_thread(1, 10);
    add_thread(2, 20); // Higher priority
    add_thread(3, 5);

    // Simulate scheduler tick - should select highest priority thread
    let mut highest_priority = 0;
    let mut selected_index = 0;

    for i in 0..THREAD_COUNT as usize {
        if RUNQUEUE[i].priority > highest_priority && RUNQUEUE[i].state == 1 {
            highest_priority = RUNQUEUE[i].priority;
            selected_index = i;
        }
    }

    if RUNQUEUE[selected_index].id != 2 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: Context switch simulation
unsafe fn test_context_switch() -> TestResult {
    init_scheduler();

    add_thread(1, 10);
    add_thread(2, 10);

    // Simulate context switch from thread 1 to thread 2
    CURRENT_THREAD = 0; // Thread 1
    RUNQUEUE[0].state = 2; // Running
    RUNQUEUE[0].cpu_time = 1000;

    // Switch to thread 2
    CURRENT_THREAD = 1;
    RUNQUEUE[1].state = 2; // Running
    RUNQUEUE[0].state = 1; // Ready

    if CURRENT_THREAD != 1 {
        return TestResult::Fail;
    }

    if RUNQUEUE[0].state != 1 || RUNQUEUE[1].state != 2 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: Thread state transitions
unsafe fn test_thread_states() -> TestResult {
    init_scheduler();

    add_thread(1, 10);

    // Test Ready -> Running
    RUNQUEUE[0].state = 1; // Ready
    if RUNQUEUE[0].state != 1 {
        return TestResult::Fail;
    }

    // Test Running -> Blocked
    RUNQUEUE[0].state = 2; // Running
    RUNQUEUE[0].state = 3; // Blocked
    if RUNQUEUE[0].state != 3 {
        return TestResult::Fail;
    }

    // Test Blocked -> Ready
    RUNQUEUE[0].state = 1; // Ready
    if RUNQUEUE[0].state != 1 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: CPU time accounting
unsafe fn test_cpu_time_accounting() -> TestResult {
    init_scheduler();

    add_thread(1, 10);

    // Simulate CPU time accumulation
    RUNQUEUE[0].cpu_time = 0;
    RUNQUEUE[0].cpu_time += 100;
    RUNQUEUE[0].cpu_time += 200;
    RUNQUEUE[0].cpu_time += 150;

    if RUNQUEUE[0].cpu_time != 450 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: Runqueue overflow protection
unsafe fn test_runqueue_overflow() -> TestResult {
    init_scheduler();

    // Fill runqueue to capacity
    for i in 0..64 {
        if add_thread(i as SigmaU32, 10) != 0 {
            return TestResult::Fail;
        }
    }

    // Attempt to add one more thread - should fail
    if add_thread(65, 10) != -1 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Main test entry point
#[no_mangle]
pub extern "C" fn test_main() -> SigmaI32 {
    let mut passed = 0;
    let mut failed = 0;

    // Run scheduler tests
    unsafe {
        if test_thread_creation() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_priority_scheduling() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_context_switch() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_thread_states() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_cpu_time_accounting() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_runqueue_overflow() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }
    }

    // Return 0 on success, non-zero on failure
    if failed > 0 {
        1
    } else {
        0
    }
}
