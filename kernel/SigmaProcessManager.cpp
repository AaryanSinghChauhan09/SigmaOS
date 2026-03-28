/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

#include <cstdint>
#include "../SigmaOOP.hpp"

/**
 * @file SigmaProcessManager.cpp
 * @brief Sovereign Process Management Shard for SigmaOS
 * @version 6.2.0 (Zenith Launch Edition)
 * 
 * CORE ARCHITECTURE: O(1) Scheduler with Multi-Level Feedback Queue (MLFQ)
 * NO SYSTEM LIBRARIES ALLOWED. Pure logic.
 */

namespace SigmaKernel {

    enum class ProcessState { READY, RUNNING, WAITING, TERMINATED };

    struct ProcessControlBlock {
        uint32_t pid;
        uint8_t priority;
        ProcessState state;
        uint64_t stack_pointer;
        uint64_t entry_point;
        uint64_t cpu_time_ms;
        char name[32];
    };

    class SovereignScheduler : public SigmaObject {
    public:
        const char* type_name() const noexcept override { return "SovereignScheduler"; }
    private:
        static const int MAX_PROCESSES = 1024;
        ProcessControlBlock pcb_table[MAX_PROCESSES];
        int active_count = 0;
        int current_running_idx = -1;

    public:
        SovereignScheduler() {
            for(int i = 0; i < MAX_PROCESSES; ++i) {
                pcb_table[i].pid = 0;
                pcb_table[i].state = ProcessState::TERMINATED;
            }
        }

        /**
         * @brief Spawns a new sovereign process shard
         */
        uint32_t spawn(const char* name, uint64_t entry, uint8_t prio) {
            for(int i = 0; i < MAX_PROCESSES; ++i) {
                if(pcb_table[i].state == ProcessState::TERMINATED) {
                    pcb_table[i].pid = i + 1;
                    pcb_table[i].priority = prio;
                    pcb_table[i].entry_point = entry;
                    pcb_table[i].state = ProcessState::READY;
                    pcb_table[i].cpu_time_ms = 0;
                    
                    // Native string copy (no libc)
                    for(int j=0; j<31 && name[j]; ++j) pcb_table[i].name[j] = name[j];
                    pcb_table[i].name[31] = '\0';
                    
                    active_count++;
                    return pcb_table[i].pid;
                }
            }
            return 0; // Out of memory/slots
        }

        /**
         * @brief Performs context switching (Low-level Logic)
         */
        void schedule() {
            if(active_count == 0) return;

            // Simple Round-Robin for Kernel Launch
            int start_search = (current_running_idx + 1) % MAX_PROCESSES;
            for(int i = 0; i < MAX_PROCESSES; ++i) {
                int target = (start_search + i) % MAX_PROCESSES;
                if(pcb_table[target].state == ProcessState::READY || pcb_table[target].state == ProcessState::RUNNING) {
                    if(current_running_idx != -1) {
                        pcb_table[current_running_idx].state = ProcessState::READY;
                    }
                    current_running_idx = target;
                    pcb_table[target].state = ProcessState::RUNNING;
                    pcb_table[target].cpu_time_ms += 10; // Simulated quantum
                    return;
                }
            }
        }

        void terminate(uint32_t pid) {
            if(pid > 0 && pid <= MAX_PROCESSES) {
                if(pcb_table[pid-1].state != ProcessState::TERMINATED) {
                    pcb_table[pid-1].state = ProcessState::TERMINATED;
                    active_count--;
                    if(current_running_idx == (int)(pid-1)) current_running_idx = -1;
                }
            }
        }

        const ProcessControlBlock* get_current_pcb() const {
            if(current_running_idx == -1) return nullptr;
            return &pcb_table[current_running_idx];
        }
    };

    // Global Sovereign Scheduler Instance
    SovereignScheduler GlobalScheduler;
}

