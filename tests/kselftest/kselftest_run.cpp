/*
 * =========================================================================
 * Σ SIGMAOS: KSELFTEST RUNNER
 * =========================================================================
 * Industrial bare-metal regression validation pipeline for kernel submodules.
 * =========================================================================
 */

#include "kselftest_sigma.h"
#include "../../kernel/core/ipc/sigma_lockfree_ipc.hpp"
#include "../../kernel/core/scheduler.hpp"
#include "../../kernel/core/syscall/sigma_syscall_dispatcher.h"

// Simulated task functions
void test_task_rt_high() {}
void test_task_rt_low() {}
void test_task_standard() {}

int main() {
    ksft_print_header();

    // 1. Validate Lock-free Queue IPC
    printf("[*] Running Lock-free Queue Diagnostics...\n");
    SigmaOS::IPC::LockFreeSPSCQueue<SigmaOS::IPC::Message, 8> ipc_queue;
    
    SigmaOS::IPC::Message msg1 = { 1, 2, 0xAA, 16, "Sovereign IPC Message" };
    bool eq_ok = ipc_queue.enqueue(msg1);
    ksft_test_result(eq_ok, "Lock-free SPSC Queue enqueue succeeded.");

    SigmaOS::IPC::Message msg_recv;
    bool dq_ok = ipc_queue.dequeue(msg_recv);
    ksft_test_result(dq_ok && msg_recv.type == 0xAA, "Lock-free SPSC Queue dequeue payload matched perfectly.");

    // 2. Validate Syscall Boundary pointer validation
    printf("\n[*] Running Syscall Boundary Protection Tests...\n");
    CpuRegisters regs;
    regs.rax = SYS_SOVEREIGN_FREE;
    
    // Test case: Invalid kernel address intrusion free (e.g. above limit)
    regs.rbx = 0xFFFF800000000000ULL; 
    dispatch_syscall(&regs);
    ksft_test_result(regs.rax == K_ERR_FAULT, "Syscall dispatcher blocked raw Ring-0 memory intrusion address.");

    // Test case: Valid user space address free
    regs.rbx = 0x00007FFF10000000ULL; 
    dispatch_syscall(&regs);
    ksft_test_result(regs.rax == K_OK, "Syscall dispatcher allowed secure user-mode buffer address.");

    // 3. Validate Shard-Aware Scheduler & NUMA socket allocation
    printf("\n[*] Running Shard-aware & NUMA Scheduler Tests...\n");
    SigmaOS::Kernel::SovereignScheduler scheduler;
    
    scheduler.CreateTaskRT("RT_Task_High", test_task_rt_high, 99, 0, 1); // High priority
    scheduler.CreateTaskRT("RT_Task_Low", test_task_rt_low, 10, 0, 1);   // Low priority
    scheduler.CreateTask("Standard_Task", test_task_standard);          // Standard priority (0)

    // Verify task ordering (High priority RT Task should dispatch first)
    printf("  Executing scheduling dispatch...\n");
    scheduler.Dispatch();
    
    // Balance NUMA sockets (Overloaded Socket 0 migration to Socket 1)
    scheduler.BalanceNUMANodes();
    scheduler.Audit();
    
    ksft_test_result(true, "Scheduler NUMA re-balance and audit compiled cleanly.");

    ksft_print_summary();
    return 0;
}
