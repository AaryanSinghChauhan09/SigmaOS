# SigmaOS Architecture

## Overview

SigmaOS is a sovereign operating system built with Rust and Nim, designed for security, performance, and independence from external dependencies. This document describes the high-level architecture and design principles.

## Design Principles

1. **Zero-Dependency Policy**: No external third-party crates in kernel and core components
2. **No-Std First**: Kernel and core modules use `no_std` to avoid standard library dependencies
3. **Sovereign Security**: Capability-based security model with fine-grained access control
4. **Performance First**: Zero-allocation optimizations and efficient data structures
5. **Modular Design**: Clear separation between kernel, userland, and tools

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Userland Applications                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │ Office Tools │  │ System APIs  │  │ Core Utils   │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                      System Call Interface                    │
│                    (Syscall Dispatch Table)                   │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                         Kernel Core                           │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │Scheduler │  │Memory Mgmt│  │Security  │  │IPC       │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                      Hardware Abstraction                     │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │APIC/PIC  │  │Timer     │  │Framebuffer│  │ACPI      │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                      Bootloader (UEFI)                        │
└─────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Bootloader (UEFI)

- **Location**: `bootloader/uefi/sigma_boot.rs`
- **Purpose**: Minimal UEFI bootloader that loads the kernel
- **Features**:
  - UEFI protocol initialization
  - Memory map acquisition
  - Kernel memory allocation
  - Framebuffer setup
  - ACPI RSDP location
  - Jump to kernel entry point

### 2. Memory Management

#### Buddy Allocator
- **Location**: `kernel/mm/buddy_allocator.rs`
- **Purpose**: Physical memory allocation using buddy system
- **Features**:
  - Linked list free lists per order
  - Block splitting and merging
  - Static frame table for tracking
  - No heap allocations

#### Slab Allocator
- **Location**: `kernel/mm/slab_allocator.rs`
- **Purpose**: Kernel object allocation
- **Features**:
  - Per-object-type caches
  - Efficient small object allocation
  - Cache management

#### Page Table Walker
- **Location**: `kernel/mm/page_table_walker.rs`
- **Purpose**: Virtual memory management
- **Features**:
  - Page table traversal
  - Page mapping/unmapping
  - Permission management

### 3. Process Scheduling

#### Round-Robin Scheduler
- **Location**: `kernel/scheduler/round_robin_scheduler.rs`
- **Purpose**: Basic CPU scheduling
- **Features**:
  - Task queue management
  - Time slice quantum
  - Context switching
  - RDTSC-based timestamps

### 4. Hardware Abstraction

#### APIC/PIC
- **Location**: `kernel/core/hal/sigma_pic.rs`
- **Purpose**: Interrupt controller management
- **Features**:
  - PIC initialization
  - APIC setup
  - IRQ routing

#### Timer
- **Location**: `kernel/core/hal/sigma_timer.rs`
- **Purpose**: System timer management
- **Features**:
  - HPET initialization
  - APIC timer setup
  - Timer interrupts

#### Framebuffer
- **Location**: `kernel/gfx/sigma_framebuffer.rs`
- **Purpose**: Graphics output
- **Features**:
  - VESA/VBE support
  - UEFI GOP support
  - Pixel manipulation
  - Console output

### 5. Security

#### Capability System
- **Location**: `kernel/security/sigma_capability.rs`
- **Purpose**: Linux-compatible capability model
- **Features**:
  - Per-process capability sets
  - Capability derivation
  - Revocation support

#### Capability Table
- **Location**: `kernel/core/capability_table.rs`
- **Purpose**: Sovereign Capability Derivation Forest
- **Features**:
  - Sparse derivation tree
  - Revocation-safe design
  - Zero-allocation implementation

### 6. System Services

#### Init System
- **Location**: `init/sigma_init.rs`
- **Purpose**: Process 1 and service management
- **Features**:
  - Service lifecycle management
  - Boot targets
  - Restart policies

#### Init Abstraction
- **Location**: `kernel/init/init_abstraction.rs`
- **Purpose**: Support for multiple init systems
- **Features**:
  - SigmaInit (default)
  - Runit support
  - S6 support
  - Dinit support
  - Sysvinit support
  - OpenRC support

### 7. Standard Library

#### Sigma Libc
- **Location**: `lib/sigma_libc/sigma_libc.rs`
- **Purpose**: Musl-compatible C standard library
- **Features**:
  - Memory allocation
  - String manipulation
  - File I/O
  - Process management

#### Zero-Allocation Optimizations
- **Location**: `kernel/core/sigma_zero_alloc.rs`
- **Purpose**: String and memory operations without heap
- **Features**:
  - Fixed-size strings
  - Zero-allocation sprintf
  - Temporary buffer pool

### 8. Feature Flags

- **Location**: `tools/feature_flags/sigma_features.rs`
- **Purpose**: Runtime feature toggling
- **Features**:
  - Feature registration
  - Dependency resolution
  - Conflict detection
  - Runtime enable/disable

## Userland Components

### Office Tools

#### Word Processor
- **Location**: `applications/wordprocessor/sigma_wordprocessor.rs`
- **Features**: Document editing, formatting, export (DOCX, ODT, PDF, TXT)

#### Spreadsheet
- **Location**: `applications/spreadsheet/sigma_spreadsheet.rs`
- **Features**: Cell management, formulas, charts, export (XLSX, ODS, CSV)

#### Presentation
- **Location**: `applications/presentation/sigma_presentation.rs`
- **Features**: Slide management, animations, transitions, export (PDF, PPTX, ODP)

#### Email Client
- **Location**: `applications/email_advanced/sigma_email_advanced.rs`
- **Features**: Email accounts, contacts, calendar, tasks

#### Database Client
- **Location**: `applications/database/sigma_database.rs`
- **Features**: Multiple database types, query execution, table listing

### System APIs

#### Control Center
- **Location**: `userland/system_api/control_center/`
- **Features**: System monitoring, GPU monitoring, logging

#### AI Integration
- **Location**: `userland/system_api/ai_integration/`
- **Features**: LLM integration, model downloads, inference

#### Dev Studio
- **Location**: `userland/system_api/dev_studio/`
- **Features**: Git integration, API testing, Docker/Kubernetes support

## Tools

### ISO Builder
- **Location**: `tools/sigma_iso_builder.rs`
- **Purpose**: Create bootable ISO images
- **Features**: GPT partition table, EFI System Partition

### Feature Flags
- **Location**: `tools/feature_flags/`
- **Purpose**: Feature flag management

## Build System

SigmaOS uses a multi-language build system:

- **Rust**: Kernel, bootloader, most userland components
- **Nim**: Some userland suites and tools
- **Shell**: Build scripts and ISO generation

## Security Model

SigmaOS uses a capability-based security model:

1. **Capabilities**: Fine-grained permissions (CAP_CHOWN, CAP_NET_ADMIN, etc.)
2. **Derivation Forest**: Tree-based capability derivation with revocation
3. **Audit Logging**: All capability checks are logged
4. **Bounding Sets**: Prevent privilege escalation

## Performance Optimizations

1. **Zero-Allocation**: Core operations avoid heap allocations
2. **Static Data Structures**: Fixed-size arrays instead of dynamic allocations
3. **Efficient Algorithms**: Buddy allocator, round-robin scheduler
4. **Inline Assembly**: Critical paths use inline assembly for performance

## Future Roadmap

See [ROADMAP.md](./ROADMAP.md) for detailed implementation plans.

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for contribution guidelines.

## License

MIT License - See [LICENSE](../LICENSE) for details.
