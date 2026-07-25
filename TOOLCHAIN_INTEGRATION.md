# GCC/LLVM Toolchain Integration for SigmaOS
# Compiler toolchain integration for C/C++/Rust compilation
# Part of GNU/Linux ecosystem standards

## Overview

SigmaOS supports both GCC and LLVM toolchains for compilation, providing flexibility and optimization options for different use cases.

## GCC Integration

### Configuration

```cmake
# Find GCC
find_program(GCC gcc)
find_program(GXX g++)

# Set compiler
set(CMAKE_C_COMPILER ${GCC})
set(CMAKE_CXX_COMPILER ${GXX})
```

### GCC Versions

SigmaOS supports multiple GCC versions:

- **GCC 13**: Default for stable releases
- **GCC 14**: Available in rolling releases
- **GCC 12**: Legacy support for older systems

### Security Features

```bash
# Enable stack protection
gcc -fstack-protector-strong -o app app.c

# Enable fortify source
gcc -D_FORTIFY_SOURCE=2 -o app app.c

# Enable position-independent code
gcc -fPIE -pie -o app app.c

# Enable relro
gcc -Wl,-z,relro,-z,now -o app app.c
```

### Optimization Levels

```bash
# No optimization (debug builds)
gcc -O0 -g -o app app.c

# Moderate optimization (default)
gcc -O2 -o app app.c

# Aggressive optimization (performance builds)
gcc -O3 -march=native -o app app.c

# Size optimization (minimal footprint)
gcc -Os -o app app.c
```

## LLVM Integration

### Configuration

```cmake
# Find LLVM/Clang
find_program(CLANG clang)
find_program(CLANGXX clang++)

# Set compiler
set(CMAKE_C_COMPILER ${CLANG})
set(CMAKE_CXX_COMPILER ${CLANGXX})
```

### LLVM Versions

SigmaOS supports multiple LLVM versions:

- **LLVM 17**: Default for stable releases
- **LLVM 18**: Available in rolling releases
- **LLVM 16**: Legacy support for older systems

### Clang Features

```bash
# Enable static analysis
clang --analyze -Xanalyzer -analyzer-checker=core app.c

# Enable sanitizers
clang -fsanitize=address -fsanitize=undefined -o app app.c

# Enable fuzzing
clang -fsanitize=fuzzer -o app app.c

# Enable coverage
clang -fprofile-instr-generate -fcoverage-mapping -o app app.c
```

### LLVM Tools

- **clang**: C/C++ compiler
- **clang++**: C++ compiler
- **llvm-ar**: Archive utility
- **llvm-nm**: Symbol listing
- **llvm-objdump**: Object file dumping
- **llvm-objcopy**: Object file copying
- **llvm-strip**: Symbol stripping
- **llvm-readelf**: ELF file reading

## Cross-Compilation

### GCC Cross-Compilers

```bash
# ARM64
aarch64-linux-gnu-gcc -o app app.c
aarch64-linux-gnu-g++ -o app app.cpp

# x86_64
x86_64-linux-gnu-gcc -o app app.c
x86_64-linux-gnu-g++ -o app app.cpp

# RISC-V
riscv64-linux-gnu-gcc -o app app.c
riscv64-linux-gnu-g++ -o app app.cpp
```

### Clang Cross-Compilation

```bash
# ARM64
clang --target=aarch64-linux-gnu -o app app.c

# x86_64
clang --target=x86_64-linux-gnu -o app app.c

# RISC-V
clang --target=riscv64-linux-gnu -o app app.c
```

### Toolchain Files

```cmake
# GCC ARM64 toolchain
set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR aarch64)
set(CMAKE_C_COMPILER aarch64-linux-gnu-gcc)
set(CMAKE_CXX_COMPILER aarch64-linux-gnu-g++)
set(CMAKE_AR aarch64-linux-gnu-ar)
set(CMAKE_STRIP aarch64-linux-gnu-strip)

# Clang ARM64 toolchain
set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR aarch64)
set(CMAKE_C_COMPILER clang)
set(CMAKE_CXX_COMPILER clang++)
set(CMAKE_C_FLAGS "--target=aarch64-linux-gnu")
set(CMAKE_CXX_FLAGS "--target=aarch64-linux-gnu")
```

## Musl libc Integration

### GCC with Musl

```bash
# Use musl-gcc wrapper
musl-gcc -o app app.c -static

# Direct musl linking
gcc -o app app.c -static -L/usr/local/musl/lib -I/usr/local/musl/include
```

### Clang with Musl

```bash
# Use musl-clang wrapper
musl-clang -o app app.c -static

# Direct musl linking
clang --target=x86_64-linux-musl -o app app.c -static
```

## Rust Integration

### Rust Toolchain

```toml
# .cargo/config.toml
[build]
target = "x86_64-unknown-linux-gnu"

[target.x86_64-unknown-linux-gnu]
linker = "gcc"
ar = "gcc-ar"
```

### Cross-Compilation

```bash
# ARM64
rustup target add aarch64-unknown-linux-gnu
cargo build --target aarch64-unknown-linux-gnu

# Musl
rustup target add x86_64-unknown-linux-musl
cargo build --target x86_64-unknown-linux-musl
```

## Build System Integration

### CMake Configuration

```cmake
# Toolchain selection
option(USE_CLANG "Use Clang instead of GCC" OFF)

if(USE_CLANG)
    set(CMAKE_C_COMPILER clang)
    set(CMAKE_CXX_COMPILER clang++)
else()
    set(CMAKE_C_COMPILER gcc)
    set(CMAKE_CXX_COMPILER g++)
endif()

# Common flags
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wall -Wextra")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra")

# Release flags
set(CMAKE_C_FLAGS_RELEASE "${CMAKE_C_FLAGS_RELEASE} -O3 -DNDEBUG")
set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -O3 -DNDEBUG")

# Debug flags
set(CMAKE_C_FLAGS_DEBUG "${CMAKE_C_FLAGS_DEBUG} -g -O0")
set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -g -O0")
```

### Profile-Guided Optimization (PGO)

```bash
# GCC PGO
gcc -fprofile-generate -o app app.c
./app
gcc -fprofile-use -o app app.c

# Clang PGO
clang -fprofile-instr-generate -o app app.c
./app
clang -fprofile-instr-use -o app app.c
```

## Implementation Status

| Component | GCC | Clang | Status |
|-----------|-----|-------|--------|
| C compilation | ✅ | ✅ | Complete |
| C++ compilation | ✅ | ✅ | Complete |
| Rust compilation | ✅ | ✅ | Complete |
| Cross-compilation | ✅ | ✅ | Complete |
| Musl integration | ✅ | ✅ | Complete |
| Security features | ✅ | ✅ | Complete |
| PGO | ✅ | ✅ | Complete |
| LTO | ✅ | ✅ | Complete |

## Best Practices

1. **Use consistent toolchains**: Choose GCC or Clang and stick with it
2. **Enable security features**: Stack protection, PIE, RELRO
3. **Use appropriate optimization**: Debug builds with -O0, release with -O2/-O3
4. **Enable warnings**: -Wall -Wextra for catching issues early
5. **Use static analysis**: Clang static analyzer, GCC static analysis
6. **Profile-guided optimization**: Use PGO for performance-critical code

## Troubleshooting

### Compiler Not Found

```bash
# Check GCC installation
gcc --version

# Check Clang installation
clang --version

# Install missing compilers
# On Debian/Ubuntu: apt install gcc clang
# On Fedora: dnf install gcc clang
```

### Cross-Compilation Issues

```bash
# Check cross-compiler installation
aarch64-linux-gnu-gcc --version

# Install cross-compilers
# On Debian/Ubuntu: apt install gcc-aarch64-linux-gnu
# On Fedora: dnf install gcc-aarch64-linux-gnu
```

### Linking Errors

```bash
# Check library paths
ldconfig -p

# Add library paths
export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH

# Check symbol dependencies
nm -D app.elf | grep symbol_name
```

## References

- GCC Documentation: https://gcc.gnu.org/onlinedocs/
- Clang Documentation: https://clang.llvm.org/docs/
- LLVM Documentation: https://llvm.org/docs/
- Rust Documentation: https://doc.rust-lang.org/
