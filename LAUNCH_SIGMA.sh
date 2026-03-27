#!/bin/bash
# Σ SIGMAOS ZENITH BUILD & LAUNCH AGENT (v6.2.0)
# ============================================================

echo -e "\033[1mΣ SIGMA OS: SOVEREIGN ZENITH BUILD PIPELINE\033[0m"
echo -e "============================================================\n"

# Step 1: Sync
echo -e "\033[34m[1] Synchronizing Silicon Shards...\033[0m"
git pull origin master 2>/dev/null || echo -e "\033[33m[!] WARNING: Offline Mode.\033[0m"

# Step 2: Build
echo -e "\033[34m[2] Compiling Sovereign Zenith Dispatcher (Ring 0)...\033[0m"
g++ -O3 -std=c++23 -I. -o SigmaKernel.bin \
    SigmaFinalIntegration.cpp \
    kernel/sigma_sml.cpp \
    kernel/SovereignVFS.cpp \
    kernel/SovereignNetwork.cpp \
    kernel/SovereignSecurity.cpp \
    kernel/SovereignVirtualizer.cpp \
    kernel/SovereignContainer.cpp \
    kernel/SovereignProcessManager.cpp \
    kernel/SovereignPM.cpp \
    -lpthread >build_log.txt 2>&1

if [ $? -ne 0 ]; then
    echo -e "\033[31m[!] FATAL: Build failed. Check build_log.txt for telemetry.\033[0m"
    cat build_log.txt
    exit 1
fi
echo -e "\033[32m[OK] Zenith Dispatcher Online: SigmaKernel.bin\033[0m"

# Step 3: Boot
echo -e "\n============================================================"
echo -e " \033[1;32mSIGMA OS ZENITH IS READY FOR LAUNCH.\033[0m"
echo -e " ALL SHARDS SYNCED. SYSTEM SOVEREIGNTY SECURED."
echo -e " PRESS ANY KEY TO BOOT INTO ZENITH..."
echo -e "============================================================\n"
read -n 1 -s
clear
echo -e "\033[1;36m[BOOT]: Engaging Sovereign Kernel...\033[0m"
./SigmaKernel.bin
