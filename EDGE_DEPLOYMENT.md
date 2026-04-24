# SigmaOS Edge & IoT Deployment Guide

Welcome to the Sovereign Lattice Edge Deployment Guide. This document outlines how to compile and boot SigmaOS on commodity hardware (Raspberry Pi 3) and RISC-V architectures, proving that SigmaOS can scale down to minimalistic footprints without sacrificing its core sovereign features.

---

## 1. Prerequisites & Toolchains

Before compiling, you must have the appropriate cross-compilation toolchains installed on your host machine.

### For Debian / Ubuntu
```bash
# Install ARM64 (AArch64) toolchain
sudo apt-get install gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu

# Install RISC-V 64 toolchain
sudo apt-get install gcc-riscv64-unknown-elf binutils-riscv64-unknown-elf

# Install QEMU for emulation (optional but recommended)
sudo apt-get install qemu-system-arm qemu-system-misc qemu-system-riscv64
```

### For macOS (via Homebrew)
```bash
brew tap riscv-software-src/riscv
brew install riscv-tools
brew install aarch64-elf-gcc
brew install qemu
```

---

## 2. Using the Edge Orchestrator

We have provided a dedicated cross-compilation script (`deploy_edge.sh`) that dynamically strips out GUI and Networking components while retaining the AI Scheduler and Memory Isolation pools.

### Compiling for Raspberry Pi 3 (ARM64)

```bash
./deploy_edge.sh --target rpi3
```

**What this does:**
1. Generates a minimal `sigma_features.json` targeted for `aarch64`.
2. Compiles the kernel using `aarch64-linux-gnu-gcc`.
3. Invokes `objcopy` to convert the ELF binary into a raw `kernel8.img`.

### Compiling for RISC-V (SiFive / QEMU)

```bash
./deploy_edge.sh --target riscv
```

**What this does:**
1. Generates a minimal `sigma_features.json` targeted for `riscv64`.
2. Compiles the kernel using `riscv64-unknown-elf-gcc`.
3. Outputs `build/sigmaos_riscv64.bin` ready for OpenSBI.

---

## 3. Flashing & Booting

### 🍓 Raspberry Pi 3 (Hardware Boot)

Once the `deploy_edge.sh --target rpi3` script finishes, you will have a `build/kernel8.img` file.

1. Format a MicroSD card to **FAT32**.
2. Copy the standard Raspberry Pi firmware files (`bootcode.bin`, `start.elf`, `fixup.dat`, `bcm2710-rpi-3-b.dtb`) to the root of the SD card.
3. Copy your generated `build/kernel8.img` to the root of the SD card.
4. Ensure your `config.txt` contains:
   ```text
   arm_64bit=1
   kernel=kernel8.img
   enable_uart=1
   ```
5. Insert the SD card into the Pi, connect a USB-to-TTL serial cable to GPIO pins 14 (TX) and 15 (RX), and open a serial monitor at `115200` baud.
6. Power on the Pi.

### 🦊 RISC-V (QEMU Emulation)

You can immediately test the RISC-V build using QEMU. The SiFive UART driver will route console output directly to your terminal.

```bash
qemu-system-riscv64 \
    -machine virt \
    -bios default \
    -kernel build/sigmaos_riscv64.bin \
    -nographic
```

To exit QEMU, press `Ctrl+A` then `X`.

---

## 4. Expected Boot Output

Upon a successful boot, you should see the SigmaOS Sovereign Lattice initialize via the serial console:

```text
==================================================
  Σ SIGMAOS KERNEL v12.5 [EDGE DEPLOYMENT]
==================================================
[HAL] BCM2837 Mini UART Initialized.
[MEM] Slab Allocator Bootstrapped (16MB Quota).
[ZTC] Zero-Trust Capability Engine Online.
[SCH] AI-Optimized Scheduler Initialized (CFS/FIFO/AI).
[IPC] Microkernel IPC Isolation Layer Ready.

[SHARD] S01_Genesis ........... OK
[SHARD] S04_HAL ............... OK
[SHARD] S05_Memory ............ OK
[SHARD] S07_Scheduling ........ OK
[SHARD] S08_Security .......... OK
[SHARD] S09_Intelligence ...... OK

Σ Sovereign Lattice Achieved.
root@sigmaos:~#
```

---

## 5. Running the Zenith UI Neural-Net Demo

SigmaOS features a Zenith Web Dashboard that visually demonstrates the OS's capability to intelligently route workloads.

1. Once the kernel is running, access the Zenith Dashboard (in a full environment, this is served via the `S07_Network` shard to a browser, or viewed via the `S02_ZenithUI` local compositor).
2. For local testing without compiling the full kernel, simply open `web_ui/index.html` in any modern web browser.
3. Locate the **Live CNN Inference Demo**.
4. Click **Run Inference**.
5. You will see a simulated 4-stage convolutional neural network. The UI communicates with the `sigma_api_service.js` (which mimics the `S07_Scheduling` dispatch logic).
6. Watch as the OS dynamically decides whether to route the matrix multiplications and convolutions to the **CPU (Fallback)** or the **NPU (Hardware Accelerated)** based on simulated hardware availability, complete with timing metrics.

---

## 6. Next Steps for Contributors

With the kernel now booting on commodity hardware and the visual demo in place, our next major milestone is **Hardware-Native Intelligence**. We are actively looking for contributors to help write hardware-abstraction drivers for specific NPUs (Neural Processing Units) to offload the `S09_Intelligence` tensor operations. 

If you have experience with custom silicon or vendor-specific ML accelerators, check out `suites/S09_Intelligence/tensor_math.c` and join the effort!
