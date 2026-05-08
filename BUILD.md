# Building SigmaOS Sovereign Lattice

This guide provides instructions for building the SigmaOS Sovereign Lattice from source.

## 🛠️ Prerequisites

* **Compiler**: GCC 12+ or Clang 15+ (Support for C++20 required)
* **Build System**: CMake 3.22+
* **Tools**: GNU Make, `clang-format`, `clang-tidy`
* **Emulator**: QEMU (with KVM/HAXM support)

## 🔨 Build Instructions

### 1. Configure the Build
Create a build directory and configure the project with CMake. You must specify the target architecture.

```bash
mkdir build
cd build
cmake .. -DARCH=x86_64 -DCMAKE_BUILD_TYPE=Release
```

### 2. Compile
Compile the kernel and shards in parallel.

```bash
make -j$(nproc)
```

### 3. Run in QEMU
Use the provided boot script to launch the kernel in an emulator.

```bash
./qemu-boot.sh
```

## 🧪 Testing & Quality Assurance

### Running Tests
SigmaOS uses GTest for host-mode testing of kernel shards.

```bash
cd build
ctest --output-on-failure
```

### Linting & Formatting
Enforce coding standards using `clang-format`.

```bash
make format-check
```

### Static Analysis
Run `clang-tidy` to identify potential bugs and security vulnerabilities (CodeQL integration).

```bash
make lint
```

## 📦 Packaging
To create a release ISO/archive:

```bash
make release
```

---
*For architectural overview, see [Architecture.md](Architecture.md).*
