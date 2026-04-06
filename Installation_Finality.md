# Σ SIGMAOS: INSTALLATION FINALITY

[![Mode](https://img.shields.io/badge/Mode-MULTI--PLATFORM-blue?style=for-the-badge)]()

**Σ SIGMAOS** supports **Every Professional Environment** through sharded deployment logic.

## 💻 PC (DUAL BOOT / MULTI BOOT)

1. Use the **`scripts/SigmaSovereignBootBuilder.sh`** tool to generate a bootable ISO.
2. Compile the **Low-Level C/ASM Kernels** (`kernel/SigmaProfessionalKernels.c`) for your specific silica.
3. Flash the ISO to a professional storage unit.

## 🐳 CONTAINERIZED (DOCKER / CLUSTER)

1. Build the industrial container: `docker build -t sigmaos .`
2. Orchestrate clustered compute: `bash scripts/SigmaClusterShard.sh`

## 🐧 VIRTUALBOX / EMULATORS

1. Start the **`SigmaOS_Zenith.iso`** in a VirtualBox/QEMU environment.
2. Ensure hardware virtualization (VT-x/AMD-V) is ACTIVE for pure performance.

## ☁️ CLOUD / WEB-BASED

1. Deploy the **Responsive ESM Shards** to any industrial remote node.
2. Access the **Cloud Zenith** mode (minimalist HLL-Reduced UI).

---

## Prerequisites

| Tool | Version | Purpose |
| --- | --- | --- |
| `gcc` | ≥ 10.0 | C11 kernel compilation |
| `nasm` | ≥ 2.15 | x86-64 Assembly assembly |
| `ld` | ≥ 2.35 | Linker for ELF output |
| `qemu-system-x86_64` | Optional | Boot kernel in VM for testing |

All tools must be available in your system `PATH`.

---

## Build Steps

```powershell

# Clone the repository

git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Run the sovereign build system

.\build.ps1
```

### Build Script Internals (`build.ps1`)

```powershell

# Assemble boot sector

nasm -f bin kernel/boot.asm -o build/boot.bin

# Compile kernel (zero-dependency)

gcc -nostdlib -ffreestanding -std=c11 `
    -fno-builtin -fno-stack-protector `
    -I libc/ -I kernel/ `
    -T kernel/sigma.ld `
    -o build/sigma.elf `
    kernel/main.c kernel/vfs.c kernel/pmm.c kernel/vmm.c `
    kernel/process.c kernel/ipc.c kernel/syscall.c `
    libc/sigma_libc.c libc/sigma_std.c

# Link final binary

ld -T kernel/sigma.ld -o build/sigma.img build/sigma.elf
```

---

## Running in Browser

The SigmaOS UI layer runs directly in any modern browser:

```bash

# Open index.html directly

start index.html

# Or serve via any static HTTP server

python -m http.server 8080

# Open: http://localhost:8080

```

---

## Sovereignty Audit (Pre-Build Check)

Always run the build master before compiling:

```powershell
gcc -o SovereignBuildMaster sovereign_tools/SovereignBuildMaster.c
.\SovereignBuildMaster
```

**Expected output** (clean build):

```
[SIGMA-AUDIT] Scanning kernel/... OK
[SIGMA-AUDIT] Scanning libc/... OK
[SIGMA-AUDIT] Scanning sovereign_tools/... OK
[SIGMA-AUDIT] All sovereignty checks PASSED. Proceed to build.
```

---

## Testing Individual Shards

```powershell

# Build and run AI Distributor

gcc -nostdlib -ffreestanding -std=c11 -I libc/ `
    sovereign_tools/sigma_ai_distribute.c libc/sigma_libc.c `
    -o build/sigma_ai_distribute
.\build\sigma_ai_distribute

# Build and run Auto Optimizer

gcc -nostdlib -ffreestanding -std=c11 -I libc/ `
    sovereign_tools/sigma_auto_optimizer.c libc/sigma_libc.c `
    -o build/sigma_auto_optimizer
.\build\sigma_auto_optimizer

# Build and run System Cleaner

gcc -nostdlib -ffreestanding -std=c11 -I libc/ `
    sovereign_tools/system_cleaner.c libc/sigma_libc.c `
    -o build/system_cleaner
.\build\system_cleaner
```

---

## Directory Structure

```
SigmaOS/
├── kernel/           # C11 + ASM kernel source (120+ files)
├── libc/             # Zero-dependency custom LibC
│   ├── SovereignLibC.h
│   ├── SigmaOOP.h    # C11 OOP abstraction layer
│   ├── sigma_libc.c
│   ├── sigma_std.c
│   └── sigma_types.h
├── sovereign_tools/  # Native CLI tools
├── scripts/          # UI JavaScript modules
├── index.html        # SigmaOS browser UI entry point
├── index.js          # SigmaSystem OOP orchestrator
├── restore_features.js  # Legacy feature restoration layer
├── build.ps1         # Sovereign build system
└── COMPETITIVE_ANALYSIS.md  # Linux distro battle plan
```

---

## GitHub Sync

```powershell

# Sync all changes to GitHub

git add .
git commit -m "Σ SigmaOS: [describe changes]"
git push origin main
```

**Σ SIGMAOS: ANY HARDWARE. ANY SCALE. NATIVE FINALITY. 🌍📱💻☁️**
