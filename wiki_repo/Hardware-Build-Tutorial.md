# 🛠️ Tutorials: Building & Deploying SigmaOS on Different Hardware

> This guide provides step-by-step instructions for cross-compiling and deploying the zero-dependency SigmaOS kernel across various computer architectures—ranging from x86_64 emulator and bare-metal environments to ARM64 and Raspberry Pi embedded systems.

---

## 💻 1. x86_64 Architecture (Emulators & Bare-Metal PCs)

The primary target for development and validation is the x86_64 platform. The build pipeline produces a multi-boot compliant ISO image bootable via GRUB.

### Prerequisites (Ubuntu/Debian Host)
```bash
sudo apt-get update
sudo apt-get install -y build-essential nasm xorriso grub-pc-bin qemu-system-x86_64
```

### Building the Toolchain & Source
If you are compiling directly, you must use an ELF cross-compiler to prevent linking host-system standard libraries:
```bash
# 1. Download x86_64-elf-gcc and x86_64-elf-binutils, or compile them from source
# 2. Add the toolchain bin/ to your system PATH
export PATH="$PATH:/opt/toolchains/x86_64-elf/bin"

# 3. Compile the standard kernel & build an ISO
make clean
make all
```

### Running inside QEMU Emulator
To test the generated ISO under a safe hardware emulation layer:
```bash
qemu-system-x86_64 \
  -cdrom build/sigmaos.iso \
  -serial stdio \
  -m 2G \
  -net nic,model=e1000 \
  -cpu host
```

---

## 📱 2. Generic ARM64 (aarch64) Architecture

For highly efficient, power-saving mobile and cloud profiles, SigmaOS can be compiled for aarch64.

### Prerequisites (Cross-Compiler)
```bash
# Install the standard ARM64 ELF bare-metal toolchain
sudo apt-get install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu
```

### Cross-Compilation Strategy
The standard kernel build system routes builds to ARM64 targets by injecting the architecture macro:
```bash
# Clean previous builds
make clean

# Compile with ARM64 environment flags
make ARCH=aarch64 CROSS_COMPILE=aarch64-linux-gnu- all
```

The output kernel image is generated as `build/sigmaos_aarch64.bin` (a flat binary format without GRUB headers, designed to be loaded directly by U-Boot or equivalent firmware).

---

## 🍓 3. Raspberry Pi 4 & 5 (ARM64 / RPi-Distro Mode)

Deploying SigmaOS directly on physical Raspberry Pi boards utilizes the specialized `arm64-rpi` build target.

### Prerequisites
* A Raspberry Pi 4 Model B or Raspberry Pi 5.
* An SD Card (minimum 4GB capacity).
* SD Card formatter and disk partition tools (`fdisk`/`parted`).

### Step 1: Compilation
Ensure your cross-compiler is available, then build with the dedicated Raspberry Pi target:
```bash
make clean
make TARGET=arm64-rpi all
```
This compilation outputs:
* `kernel8.img`: The standard 64-bit arm64 kernel binary required by the Raspberry Pi bootloader.

### Step 2: Preparing the SD Card
The Raspberry Pi bootloader expects a FAT32 boot partition containing specialized GPU firmware files.

1. **Partition the SD Card**:
   Create a single primary partition formatted as **FAT32** (LBA), and mark it as active.
   ```bash
   # Identify your SD card (e.g., /dev/sdX or /dev/mmcblk0)
   sudo fdisk /dev/sdX
   # Create a new primary partition, type FAT32 (0x0C), write and exit.
   sudo mkfs.vfat -F 32 /dev/sdX1
   ```

2. **Mount the partition**:
   ```bash
   sudo mkdir -p /mnt/rpi_boot
   sudo mount /dev/sdX1 /mnt/rpi_boot
   ```

### Step 3: Coping Firmware & Kernel
Copy the compiled kernel along with the standard Raspberry Pi start-up files (which can be fetched from the official Raspberry Pi firmware repository):
```bash
# Copy firmware binaries
cp bootcode.bin start4.elf fixup4.dat /mnt/rpi_boot/

# Copy device tree blobs (DTBs) for hardware mapping
cp bcm2711-rpi-4-b.dtb bcm2712-rpi-5.dtb /mnt/rpi_boot/

# Copy the SigmaOS ARM64 kernel
cp build/kernel8.img /mnt/rpi_boot/
```

### Step 4: Configuring Boot Options
Create a new file named `config.txt` inside `/mnt/rpi_boot/` to instruct the Pi on how to load the SigmaOS kernel:
```ini
# --- config.txt ---
# Enforce 64-bit kernel execution
arm_64bit=1

# Disable default GUI loading and allocate memory for drivers
gpu_mem=64

# Point directly to the SigmaOS kernel binary
kernel=kernel8.img

# Enable UART debugging interface
enable_uart=1
uart_2ndstage=1

# Map the Device Tree
device_tree=bcm2711-rpi-4-b.dtb
```

Create a `cmdline.txt` file next to it for bootloader console routing:
```text
console=serial0,115200 console=tty1 root=/dev/mmcblk0p1 rootwait
```

### Step 5: Unmount and Boot
Unmount the SD card, insert it into your Raspberry Pi, and connect the serial cable to view the diagnostic boot output:
```bash
sudo umount /mnt/rpi_boot
```
* **Booting**: Connect an FTDI serial-to-USB cable to the GPIO UART pins (TX on Pin 8, RX on Pin 10, GND on Pin 6) to read the startup diagnostics via `screen` or `putty` at `115200` baud rate!
