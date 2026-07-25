# GNU Binutils Integration for SigmaOS
# Binary utilities integration for object file manipulation and linking
# Part of GNU/Linux ecosystem standards

## Overview

SigmaOS integrates GNU Binutils for object file manipulation, linking, and binary analysis. This provides standard tooling for working with executables, libraries, and object files.

## Components

### Included Binutils Tools

- **as**: GNU assembler
- **ld**: GNU linker
- **ar**: Archive utility for creating static libraries
- **nm**: Symbol listing utility
- **objdump**: Object file information display
- **objcopy**: Object file copying and conversion
- **strip**: Strip symbols from object files
- **readelf**: ELF file information display
- **size**: Section size display
- **strings**: Print printable strings in files
- **addr2line**: Convert addresses to file names and line numbers

## Configuration

### Build System Integration

```cmake
# Find Binutils
find_program(BINUTILS_AR ar)
find_program(BINUTILS_AS as)
find_program(BINUTILS_LD ld)
find_program(BINUTILS_NM nm)
find_program(BINUTILS_OBJDUMP objdump)
find_program(BINUTILS_OBJCOPY objcopy)
find_program(BINUTILS_STRIP strip)
find_program(BINUTILS_READELF readelf)

# Use Binutils tools
set(CMAKE_AR ${BINUTILS_AR})
set(CMAKE_RANLIB ${BINUTILS_RANLIB})
```

### Cross-Compilation Toolchains

```cmake
# ARM64 Binutils
set(CMAKE_AR aarch64-linux-gnu-ar)
set(CMAKE_STRIP aarch64-linux-gnu-strip)
set(CMAKE_OBJCOPY aarch64-linux-gnu-objcopy)

# x86_64 Binutils
set(CMAKE_AR x86_64-linux-gnu-ar)
set(CMAKE_STRIP x86_64-linux-gnu-strip)
set(CMAKE_OBJCOPY x86_64-linux-gnu-objcopy)
```

## Usage Examples

### Creating Static Libraries

```bash
# Create static library from object files
ar rcs libsigmaos.a file1.o file2.o file3.o

# List archive contents
ar t libsigmaos.a

# Extract from archive
ar x libsigmaos.a file1.o
```

### Linking with Custom Linker Scripts

```bash
# Use custom linker script
ld -T sigmaos.lds -o sigmaos.elf file1.o file2.o

# Generate map file
ld -T sigmaos.lds -Map=sigmaos.map -o sigmaos.elf file1.o file2.o
```

### Symbol Analysis

```bash
# List symbols in object file
nm -C sigmaos.elf

# List only undefined symbols
nm -u sigmaos.elf

# List only defined symbols
nm --defined-only sigmaos.elf
```

### Object File Analysis

```bash
# Display object file header
readelf -h sigmaos.elf

# Display section headers
readelf -S sigmaos.elf

# Display program headers
readelf -l sigmaos.elf

# Display symbol table
readelf -s sigmaos.elf
```

### Binary Conversion

```bash
# Convert ELF to binary
objcopy -O binary sigmaos.elf sigmaos.bin

# Extract specific sections
objcopy -j .text -j .rodata -O binary sigmaos.elf text.bin

# Add debug information
objcopy --add-gnu-debuglink=sigmaos.debug sigmaos.elf
```

### Stripping Symbols

```bash
# Strip all symbols
strip sigmaos.elf

# Strip debug symbols only
strip --strip-debug sigmaos.elf

# Strip unneeded symbols
strip --strip-unneeded sigmaos.elf
```

## Security Features

### Address Space Layout Randomization (ASLR)

```bash
# Create position-independent executable
ld -pie -fPIE -o sigmaos.elf file1.o file2.o
```

### RELRO (Relocation Read-Only)

```bash
# Enable RELRO
ld -z relro -z now -o sigmaos.elf file1.o file2.o
```

### Stack Protection

```bash
# Enable stack canaries (requires compiler support)
gcc -fstack-protector-strong -o sigmaos file1.c file2.c
```

## Integration with SigmaOS Build System

### Kernel Build

```makefile
# Use SigmaOS-specific linker script
KERNEL_LDFLAGS = -T kernel/sigmaos.lds -nostdlib

# Link kernel
$(LD) $(KERNEL_LDFLAGS) -o kernel.elf $(KERNEL_OBJS)

# Extract raw binary
$(OBJCOPY) -O binary kernel.elf kernel.bin
```

### Userland Build

```makefile
# Create static libraries
$(AR) rcs libsigmaos.a $(LIB_OBJS)

# Link userland applications
$(CC) -o app app.o -L. -lsigmaos
```

## Musl libc Integration

### Building with Musl

```bash
# Use musl-gcc wrapper
musl-gcc -o app app.c -static

# Use musl-specific binutils
musl-ar rcs libapp.a app.o
musl-ld -o app app.o -lc
```

### Cross-Compilation

```bash
# ARM64 with musl
aarch64-linux-musl-gcc -o app app.c
aarch64-linux-musl-ld -o app app.o -lc

# x86_64 with musl
x86_64-linux-musl-gcc -o app app.c
x86_64-linux-musl-ld -o app app.o -lc
```

## Implementation Status

| Component | Status | Notes |
|-----------|--------|-------|
| as (assembler) | ✅ Complete | Integrated in build system |
| ld (linker) | ✅ Complete | Custom linker scripts |
| ar (archive) | ✅ Complete | Static library creation |
| nm (symbols) | ✅ Complete | Symbol analysis tools |
| objdump | ✅ Complete | Object file inspection |
| objcopy | ✅ Complete | Binary conversion |
| strip | ✅ Complete | Symbol stripping |
| readelf | ✅ Complete | ELF file analysis |
| Cross-compilation | ✅ Complete | Multi-architecture support |
| Musl integration | ✅ Complete | musl-gcc wrappers |

## Best Practices

1. **Use position-independent code**: Enable PIE for security
2. **Strip debug symbols**: Reduce binary size for production
3. **Generate map files**: Useful for debugging and analysis
4. **Use custom linker scripts**: Control memory layout
5. **Enable security features**: RELRO, ASLR, stack protection
6. **Static linking for minimal systems**: Reduce dependencies

## Troubleshooting

### Missing Symbols

```bash
# Check for undefined symbols
nm -u app.elf

# Check library dependencies
ldd app.elf
```

### Linker Errors

```bash
# Use verbose linking
ld -verbose -o app.elf file1.o file2.o

# Check library search paths
ld --verbose | grep SEARCH_DIR
```

### Architecture Mismatches

```bash
# Check file architecture
readelf -h app.elf | grep Machine

# Convert architectures if needed
objcopy -I binary -O elf64-x86-64 input.bin output.o
```

## References

- GNU Binutils Documentation: https://sourceware.org/binutils/docs/
- ELF Format: https://refspecs.linuxfoundation.org/elf/elf.pdf
- Linker Scripts: https://sourceware.org/binutils/docs/ld/Scripts.html
