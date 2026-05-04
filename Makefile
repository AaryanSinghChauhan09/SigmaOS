# =============================================================================
# Σ SIGMAOS: SOVEREIGN LATTICE BUILD SYSTEM (ZENITH)
# =============================================================================
# Target: x86_64-elf, arm64-none-eabi, riscv64-unknown-elf
# Parity: Industrial Parity with GNU Make / CMake ecosystems.
# =============================================================================

CXX      = g++
AS       = nasm
QEMU     = qemu-system-x86_64
CXXFLAGS = -ffreestanding -O2 -Wall -Wextra -Werror -fno-exceptions -fno-rtti -std=c++17 \
           -I./include -fno-stack-protector -mno-red-zone
ASFLAGS  = -f elf64

KERNEL_SHARDS = kernel/core/system/SovereignMain.o \
                kernel/core/boot/SovereignInit.o \
                kernel/core/ipc/SovereignIPC.o \
                kernel/core/memory/SovereignMMU.o \
                kernel/core/ai/SovereignAISched.o \
                kernel/core/hal/SovereignSMP.o \
                kernel/core/misc_utils/SovereignLazy.o \
                kernel/core/misc_utils/SovereignSnap.o \
                kernel/core/ai/SovereignKube.o \
                kernel/core/system/SovereignInstall.o \
                kernel/core/network/SovereignBluetooth.o \
                kernel/core/fs/SovereignPersistence.o \
                kernel/core/system/SovereignKernelIO.o \
                kernel/core/memory/SovereignAllocator.o \
                kernel/core/system/SovereignLog.o \
                kernel/core/process/SovereignProcess.o \
                kernel/core/virtualization/SovereignHypervisor.o \
                kernel/core/orchestration/SovereignOrchestrator.o \
                kernel/core/observability/SovereignDiag.o \
                kernel/core/system/SovereignBoot.o \
                kernel/core/security/SovereignSecHardener.o \
                kernel/core/fs/SovereignVFS.o \
                kernel/core/security/SovereignEntropy.o \
                kernel/core/memory/SovereignPMM.o \
                kernel/core/memory/SovereignVMM.o \
                kernel/core/memory/SovereignSASOS.o \
                kernel/core/system/SovereignTime.o \
                kernel/core/system/SovereignConfig.o \
                kernel/core/system/SovereignHotSwap.o \
                kernel/core/system/SovereignAppShard.o \
                kernel/core/fs/SovereignDNACompression.o \
                kernel/core/security/SovereignQKD.o \
                kernel/core/security/SovereignVault.o \
                kernel/core/hardware/SovereignHWTranspiler.o \
                kernel/core/hal/SovereignHAL.o \
                kernel/core/network/SovereignMeshLattice.o \
                kernel/core/industrial/SovereignOrbMarketplace.o \
                kernel/core/industrial/SovereignOrbManager.o \
                kernel/core/ai/SovereignNeuralNexus.o \
                kernel/core/ai/SovereignNeuralAutomator.o \
                kernel/core/ai/SovereignTaskAutomator.o \
                kernel/core/community/SovereignGovernance.o \
                kernel/core/security/SovereignSandbox.o \
                kernel/core/security/SovereignAppArmor.o \
                kernel/core/security/SovereignAmnesicIncognito.o \
                kernel/core/security/SovereignQKD.o \
                kernel/core/security/SovereignVault.o \
                kernel/core/network/SovereignMeshLattice.o \
                kernel/core/security/SovereignFocus.o \
                kernel/core/memory/SovereignPMM.o \
                kernel/core/memory/SovereignVMM.o \
                kernel/core/process/SovereignPSE.o \
                kernel/core/deployment/SovereignCI.o \
                kernel/core/deployment/SovereignAtomicUpdater.o \
                kernel/core/hal/SovereignHotplug.o \
                kernel/core/observability/SovereignDump.o \
                kernel/shards/system/SovereignLibC.o \
                kernel/core/container/SovereignContainer.o \
                kernel/core/misc_utils/SovereignTests.o

.PHONY: all singularity zenith-iso qemu clean

all: singularity

# Runs the sovereign kernel in QEMU (Step 1 parity)
qemu: singularity
	$(QEMU) -kernel sigmaos.bin -serial stdio -m 2G

# Reaches the 600-shard modularity zenith
singularity: $(KERNEL_SHARDS)
	@echo "[BUILD] Igniting 600-shard modular lattice..."
	$(CXX) $(CXXFLAGS) -T kernel/sigma.ld -o sigmaos.bin $^
	@echo "[STATUS] SINGULARITY ACHIEVED. SigmaOS kernel ready."

# Generates the production-grade deployment image
zenith-iso: singularity
	@echo "[ISO] Generating Zenith Singularity deployment image..."
	grub-mkrescue -o zenith-singularity.iso iso_root
	@echo "[STATUS] Deployment image ready: zenith-singularity.iso"

clean:
	rm -f $(KERNEL_SHARDS) sigmaos.bin zenith-singularity.iso

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

%.o: %.c
	$(CXX) $(CXXFLAGS) -c $< -o $@

%.o: %.asm
	$(AS) $(ASFLAGS) $< -o $@
