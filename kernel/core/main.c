/*
 * =========================================================================
 * Σ SIGMAOS ZENITH SUPREME: KERNEL ENTRY POINT (MAIN SHARD)
 * =========================================================================
 * Mission: Initialize all Sovereign subsystems and start the Aether.
 * Capability: OS Core, AI Neural Engine, ML Processing, Indian Law DB
 * SOLID Principles: High cohesion through isolated init delegates.
 * =========================================================================
 */

#include "../../libc/SovereignLibC.h"
#include "../SovereignMemoryRAII.h"

// --- OS Core Subsystems ---
extern void sigma_scheduler_init();
extern void sigma_vfs_init();
extern void sigma_slab_init(); 

// --- Advanced Domain Shards (AI/ML/LAW/DS) ---
extern void SovereignAIKernel_Init(void);
extern void SovereignML_Init(void);
extern void SovereignDataScience_Init(void);
extern void SovereignIndianLaw_BNSS_Init(void);
extern void SovereignQuantum_LatticeInit(void);
extern void SovereignGaming_Init(void);
extern void SovereignCyber_Init(void);
extern void SovereignFintech_Init(void);
extern void SovereignBio_Init(void);
extern void SovereignWatchdog_Init(void);
extern void SovereignWatchdog_Pulse(void);

void sigma_kernel_main() {
    sigma_printf("\n==================================================\n");
    sigma_printf("  Σ SIGMAOS ZENITH SUPREME (v160.0) BOOTING...  \n");
    sigma_printf("==================================================\n");

    /* 1. LAYER ONE: Core OS Principles (Memory & FS) */
    sigma_printf("[*] Activating Core OS Abstractions...\n");
    sigma_slab_init(); 
    sigma_vfs_init();
    
    /* Auto-allocate startup matrix using pure C RAII for safety */
    {
        SOVEREIGN_AUTOSHARD(sigma_u8, boot_matrix, 2048, "Kernel_Boot");
        sigma_printf("[+] Sovereign RAII Memory Protection Activated.\n");
    }

    /* 2. LAYER TWO: Security & Synchronization (PQC) */
    SovereignQuantum_LatticeInit();
    sigma_printf("[+] Post-Quantum Cryptography Latice Synchronized.\n");

    /* 3. LAYER THREE: AI & Machine Learning Vectors */
    sigma_printf("\n[*] Initializing Neural and ML Shards...\n");
    SovereignAIKernel_Init();
    SovereignML_Init();

    /* 4. LAYER FOUR: Industrial Sectors (Data Science, Gaming, Cyber, Fintech, Bio) */
    SovereignDataScience_Init();
    SovereignGaming_Init();
    SovereignCyber_Init();
    SovereignFintech_Init();
    SovereignBio_Init();
    
    /* 4.5 LAYER 4.5: Automation & Reliability (Self-Healing) */
    SovereignWatchdog_Init();
    SovereignWatchdog_Pulse();
    
    /* 5. LAYER FIVE: Legal Frameworks (BNS / BNSS compliance) */
    sigma_printf("\n[*] Loading Indian Legal Framework Constants...\n");
    SovereignIndianLaw_BNSS_Init();

    /* 6. FINAL PHASE: Process Orchestration */
    sigma_scheduler_init();
    
    sigma_printf("--------------------------------------------------\n");
    sigma_printf("Σ SYSTEM SOVEREIGNTY ACHIEVED. READY TO LAUNCH.\n");
    sigma_printf("Switching to Ring-3 Omni-Shell Dropin...\n");
    
    /* Execute OmniShell Loop */
    extern void Sovereign_OmniShell_Enter();
    Sovereign_OmniShell_Enter();
}

