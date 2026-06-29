# Building from Source

Supported build host: **Ubuntu 22.04 LTS (x86_64)**. All branches build from the same repository; the profile/toolchain file selects the target.

---

## Prerequisites

```bash
sudo apt update
sudo apt install -y \
  build-essential nasm gcc g++ make cmake ninja-build \
  qemu-system-x86 \
  grub-pc-bin grub-efi-amd64-bin xorriso mtools \
  clang clang-tidy cppcheck \
  golang-go \
  git curl wget \
  nodejs npm \
  gcc-aarch64-linux-gnu g++-aarch64-linux-gnu  # for ARM64 builds

# Go daemons (healthd, apid, etc.)
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest

# Pre-commit hooks (Round 2)
pip install pre-commit && pre-commit install

# Commit conformance (Round 3)
# Download conform from: https://github.com/siderolabs/conform/releases
```

---

## 1. Clone

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
git submodule update --init --recursive
```

---

## 2. Choose a Profile

| Use case | Command |
|---|---|
| Workstation (default) | `cmake -B build -DCMAKE_TOOLCHAIN_FILE=profiles/workstation.cmake` |
| IoT / ARM64 | `cmake -B build -DCMAKE_TOOLCHAIN_FILE=profiles/iot-minimal.cmake` |
| RTOS / hard real-time | `make SIGMA_USE_ZENITH_DE=0 SIGMA_SCHED_REALTIME=1` |
| Cloud immutable | `cmake -B build -DSIGMA_PROFILE=cloud-x86 -DSIGMA_IMMUTABLE_ROOT=ON` |
| Bare microkernel | `make SIGMA_USE_ZENITH_DE=0 SIGMA_USE_AI_ENGINE=0` |

---

## 3. Build

```bash
# Check for stub warnings first (Buildroot BR2_BROKEN pattern)
make check-stubs

# Build kernel + ISO
make clean && make all -j$(nproc)

# Build Go daemons
cd sigmad/healthd && go build -o ../../build/daemons/sigma-healthd .
# Repeat for each daemon in sigmad/
```

**Stub warnings** — the Makefile emits a banner listing unimplemented subsystems on every build. `SIGMA_RELEASE_BUILD=1` turns these into fatal errors.

---

## 4. Generate gRPC stubs (api/sigma.proto)

```bash
protoc --go_out=. --go-grpc_out=. api/sigma.proto
# Generates: api/sigma.pb.go, api/sigma_grpc.pb.go
```

---

## 5. Test in QEMU

```bash
# Standard desktop boot
qemu-system-x86_64 -cdrom build/sigmaos.iso -serial stdio -m 2G -enable-kvm

# RTOS profile (no GUI)
qemu-system-x86_64 -kernel build/sigmaos.bin -serial stdio -m 1G -enable-kvm

# ARM64 (mobile profile)
qemu-system-aarch64 -M virt -cpu cortex-a57 -m 1G \
  -kernel build/sigmaos-aarch64.bin -serial stdio -display none
```

---

## 6. Run the Test Suite

```bash
# Google Test unit tests (host-mode)
cd tests/cpp_host && cmake -B build && cmake --build build
cd build && ctest --output-on-failure

# OpenBSD-style kernel regression tests
make -C tests regress

# libFuzzer TCP harness (30-second budget)
clang++ -fsanitize=fuzzer,address -Iinclude \
  tests/kernel/fuzz_tcp.cpp kernel/net/sigma_tcpip.c -o fuzz_tcp
./fuzz_tcp -max_total_time=30

# pledge violation real test (SIGABRT)
g++ -std=c++17 -Iinclude -Iklib/include -Ikernel/security/jail \
  tests/kernel/pledge/test_pledge_sigabrt.cpp \
  kernel/security/jail/sigma_pledge.cpp \
  tests/kernel/stubs/sigma_stubs.cpp -o test_pledge && ./test_pledge

# Manifest validator
cd pkg/sigma-manifest-validator && go run . ../sigma-manifest.toml.example
```

---

## 7. Stub Health Check

```bash
# Build-time stub report
make check-stubs

# Runtime stub report (requires sigma-healthd running)
sigmactl health
# Shows: ✗ cryptfs FAILED — derive_key() stub
```

---

## 8. Write to USB (bare-metal)

```bash
# Find device
lsblk

# Write (DESTRUCTIVE — replaces entire drive)
sudo dd if=build/sigmaos.iso of=/dev/sdX bs=4M status=progress && sync
```

---

## Build Profiles Reference

```bash
# Explicitly set USE flags
make SIGMA_USE_HYPERVISOR=1 \
     SIGMA_USE_AI_ENGINE=1  \
     SIGMA_USE_ZENITH_DE=1  \
     SIGMA_USE_CRYPTFS=1    \
     SIGMA_USE_BLUETOOTH=0  \
     SIGMA_USE_WIFI=0       \
     SIGMA_IMMUTABLE_ROOT=0
```

---

## Troubleshooting

**`[STUB] sigma-jail: Only prints to console`** — the namespace isolation replacement is in `kernel/security/jail/sigma_namespace.cpp`. Wire `sigma_jail_create()` to call `sigma_jail_enter()`.

**glibc symbols in kernel binary** — run `nm build/sigmaos.bin | grep GLIBC`. Any match means a kernel source file includes `<stdio.h>` / `<string.h>`. Replace with `klib/` equivalents.

**QEMU triple-faults immediately** — IDT not initialised before interrupts enabled. Check `sigma_idt_init()` is called first in `kmain.cpp`.

**`make check-stubs` shows `kernel/core` missing** — the core kernel source files (scheduler, MM, syscall table) haven't been committed yet. See Contributor Roadmap.

---

*See also: [Branch Guide](Branch-Guide) · [Contributor Roadmap](Contributor-Roadmap) · [Architecture Overview](Architecture-Overview)*
