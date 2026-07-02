# LLVM / Clang Toolchain Integration

## Overview

SigmaOS uses **LLVM/Clang** as its primary C/C++ toolchain for components that require C (firmware blobs, Mesa, Wine dependencies). LLVM enables LTO, ThinLTO, PGO, BOLT post-link optimization, and cross-compilation for all target architectures (x86_64, ARM64, RISC-V).

---

## Use Cases

| Optimization | Tool | Benefit |
|---|---|---|
| Link-Time Optimization | Clang LTO / ThinLTO | Cross-TU inlining, dead code elimination |
| Profile-Guided Optimization | Clang PGO (`-fprofile-generate`) | Branch layout from real workloads |
| BOLT post-link | `llvm-bolt` | Function reordering from perf data |
| Cross-compilation | `clang --target=` | Build for ARM64/RISC-V from x86_64 CI |
| Size optimization | `-Os -flto` | Kernel binary reduction |

---

## sigma-sdk Default Compiler

The sigma-sdk sets Clang as the default C compiler:

```bash
export CC=clang
export CXX=clang++
export AR=llvm-ar
export NM=llvm-nm
export RANLIB=llvm-ranlib
export LDFLAGS="-fuse-ld=lld"
```

---

## build/toolchain/sigma_clang.cmake

```cmake
# build/toolchain/sigma_clang.cmake
# CMake toolchain file for SigmaOS + Clang + LTO

set(CMAKE_C_COMPILER   clang)
set(CMAKE_CXX_COMPILER clang++)
set(CMAKE_AR           llvm-ar)
set(CMAKE_RANLIB       llvm-ranlib)
set(CMAKE_LINKER       lld)

# Size-optimized build with LTO
set(CMAKE_C_FLAGS_RELEASE
    "-Os -flto=thin -ffunction-sections -fdata-sections \
     -fstack-protector-strong -D_FORTIFY_SOURCE=2")
set(CMAKE_EXE_LINKER_FLAGS_RELEASE
    "-fuse-ld=lld -flto=thin -Wl,--gc-sections -Wl,-z,relro -Wl,-z,now")

# Cross-compilation targets
if(SIGMA_ARCH STREQUAL "arm64")
    set(CMAKE_C_COMPILER_TARGET aarch64-linux-musl)
    set(CMAKE_SYSROOT /usr/local/sigma-sdk/sysroots/aarch64-musl)
elseif(SIGMA_ARCH STREQUAL "riscv64")
    set(CMAKE_C_COMPILER_TARGET riscv64-linux-musl)
    set(CMAKE_SYSROOT /usr/local/sigma-sdk/sysroots/riscv64-musl)
endif()
```

---

## Kernel Build with LTO

```makefile
# kernel/Makefile (LTO target)
CLANG    := clang
LLD      := ld.lld
CFLAGS   := -Os -flto=thin -target x86_64-elf -fno-stack-protector \
             -ffunction-sections -fdata-sections -mno-red-zone

sigma-kernel.elf: $(OBJ_FILES)
	$(CLANG) $(CFLAGS) -fuse-ld=$(LLD) \
		-Wl,--gc-sections \
		-T kernel/kernel.ld \
		-o $@ $^

.PHONY: lto-report
lto-report:
	llvm-lto -summary-only sigma-kernel.elf > lto-report.txt
```

---

## PGO Workflow

```bash
# Step 1: Instrument build
make SIGMA_PGO=generate sigma-kernel.elf

# Step 2: Run workload to gather profile
qemu-system-x86_64 -kernel sigma-kernel.elf -append "sigma.pgo=1" ...

# Step 3: Merge profiles
llvm-profdata merge -output=sigma.profdata *.profraw

# Step 4: Optimised build with profile
make SIGMA_PGO=use SIGMA_PROFDATA=sigma.profdata sigma-kernel.elf
```

---

## BOLT Post-Link Optimization

```bash
# Instrument binary
llvm-bolt sigma-kernel.elf -instrument -o sigma-kernel.bolt-inst

# Run and collect perf data
perf record -e cycles:u -j any,u sigma-kernel.bolt-inst ...
perf2bolt -p perf.data -o sigma.fdata sigma-kernel.bolt-inst

# Optimize with BOLT
llvm-bolt sigma-kernel.elf \
  -data sigma.fdata \
  -reorder-blocks=ext-tsp \
  -reorder-functions=hfsort \
  -split-functions -split-all-cold \
  -dyno-stats \
  -o sigma-kernel.elf.bolt
```

---

## Exit Criteria

- `make sigma-kernel.elf CFLAGS="-Os -flto=thin"` succeeds; binary is ≥10% smaller than non-LTO build.
- `clang --target=aarch64-linux-musl -Os kernel/main.c` cross-compiles without errors.
- BOLT-optimized kernel boots in QEMU and shows reduced cold-start time in benchmarks.
