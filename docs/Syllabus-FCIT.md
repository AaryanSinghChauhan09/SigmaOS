# Fundamentals of Computer & IT → SigmaOS Implementation

> Maps the FCIT academic syllabus to concrete SigmaOS kernel features, HAL drivers, and userland apps.

---

## Unit I: Basic of Computers

### 🔢 Number Systems & Base Conversion

**Syllabus:**Binary, Octal, Decimal, Hexadecimal; conversions in C, C++, HTML, JS, PHP, Python.**SigmaOS Implementation:** `SovereignCodec` — a zero-dependency encoding/conversion module in the kernel.

```cpp
// kernel/core/sigma_codec.h
namespace Sigma::Codec {
    uint64_t to_binary(uint64_t decimal);
    uint64_t to_octal(uint64_t decimal);
    uint64_t to_hex(uint64_t decimal);
    uint64_t from_binary(const char* bin_str);
    uint64_t from_hex(const char* hex_str);

    // Multi-language output formatters
    void print_c_style(uint64_t val);       // 0xFF format
    void print_js_style(uint64_t val);      // 0xFF / 0b1111 format
    void print_python_style(uint64_t val);  // hex() / bin() / oct()
    void print_html_style(uint64_t val);    // &#xHH; entity format
    void print_php_style(uint64_t val);     // hexdec() / decoct()
}

```text

**Integration Layer:** Kernel Core → `sigma-cli` shell (run `sigma convert 255 --to hex`)

---

### 💻 Basic Binary Arithmetic

**Syllabus:**Addition, subtraction, multiplication, 1s/2s complement, overflow detection.**SigmaOS Implementation:** Native ALU primitives exposed via HAL.

```cpp
// kernel/hal/sigma_alu.h
namespace Sigma::ALU {
    uint64_t add_binary(uint64_t a, uint64_t b);
    uint64_t sub_twos_complement(uint64_t a, uint64_t b);
    bool detect_overflow(uint64_t a, uint64_t b, uint64_t result);
    uint64_t ones_complement(uint64_t val, int bits);
    uint64_t twos_complement(uint64_t val, int bits);
}

```text

---

### 📝 Computer Codes (ASCII, BCD, EBCDIC, UNICODE)

**Syllabus:**ASCII table, BCD encoding, EBCDIC legacy, Unicode UTF-8/16/32.**SigmaOS Implementation:** `SovereignCharset` multi-encoding layer.

```cpp
// kernel/core/SovereignCharset.cpp
namespace Sigma::Charset {
    // ASCII: 7-bit encoding, 128 characters
    char ascii_lookup[128];

    // BCD: 4-bit per decimal digit
    uint8_t to_bcd(uint8_t decimal);
    uint8_t from_bcd(uint8_t bcd);

    // EBCDIC: IBM legacy 8-bit encoding
    uint8_t ascii_to_ebcdic(uint8_t ascii_char);
    uint8_t ebcdic_to_ascii(uint8_t ebcdic_char);

    // Unicode: UTF-8/16/32 encode/decode
    uint32_t utf8_codepoint(const char* utf8_seq);
    void codepoint_to_utf8(uint32_t cp, char* out);
    void codepoint_to_utf16(uint32_t cp, uint16_t* out);
}

```text

---

### 🖥️ History & Generations of Computers

**Syllabus:**5 generations (vacuum tubes → AI chips), characteristics of each.**SigmaOS Context:**SigmaOS is designed for the**5th Generation** — AI-native, silicon-direct, post-quantum secure.

| Generation | Technology | SigmaOS Parallel |
| --- | --- | --- |
| 1st | Vacuum Tubes (1940s) | — |
| 2nd | Transistors (1950s) | — |
| 3rd | ICs (1960s) | — |
| 4th | VLSI Microprocessors (1970s-present) | x86/ARM HAL support |
| 5th | AI/Quantum (2020s+) | **SigmaOS Zenith** — AI-native kernel |

---

### 🏗️ Basic Organization of Computer

**Syllabus:** CPU, ALU, CU, Memory, I/O — von Neumann architecture.

### SigmaOS Mapping

| Component | SigmaOS Module |
| --- | --- |
| CPU (ALU + CU) | `SovereignScheduler` + HAL ALU |
| Primary Memory | `SovereignAllocator` (RAM manager) |
| Secondary Storage | `SovereignZFSPool` (S-ZFS) |
| Input Devices | HAL input drivers (`/drivers/input/`) |
| Output Devices | HAL output drivers (`/drivers/output/`) |
| System Bus | IPC message bus (`SovereignIPC`) |

---

### 🖱️ Command Prompt

**Syllabus:**Basic CMD commands, file navigation, scripting.**SigmaOS Implementation:** `sigma-cli` — the Sovereign Shell.

```bash

# sigma-cli commands (SigmaOS equivalent of CMD/bash)

sigma ls /sigma/home           # list directory

sigma cd /sigma/apps           # change directory

sigma run SigmaDocs            # launch app

sigma convert 255 --to binary  # number conversion

sigma pkg install sigma-python # package manager

sigma sysinfo                  # system information

sigma netstat                  # network status

sigma kill <pid>               # process management

```text

**File:** `tools/sigma_cli.cpp`

---

## Unit II: I/O Devices, Memory, Storage & Processor

### 🖨️ Types of Input & Output Devices

**Input Devices:**Keyboard, Mouse, Scanner, Microphone, Webcam, Touchscreen, Joystick.**Output Devices:** Monitor, Printer, Speaker, Projector, Headphones.

### SigmaOS HAL Driver Registry

```cpp
// kernel/core/drivers/SovereignDriverRegistry.cpp
struct HALDevice {
    DeviceType type;          // INPUT / OUTPUT / STORAGE / NETWORK
    const char* name;
    uint32_t vendor_id;
    bool (*probe)(HALDevice*);
    int  (*read)(HALDevice*, void* buf, size_t len);
    int  (*write)(HALDevice*, const void* buf, size_t len);
};

// Registered drivers
HALDevice input_devices[] = {
    { INPUT, "PS/2 Keyboard",  0x0001, kb_probe,    kb_read,  nullptr },
    { INPUT, "USB HID Mouse",  0x0002, ms_probe,    ms_read,  nullptr },
    { INPUT, "USB Webcam",     0x0003, cam_probe,   cam_read, nullptr },
    { OUTPUT,"VGA/VESA Monitor",0x0010, vga_probe,  nullptr,  vga_write },
    { OUTPUT,"ALSA Audio",     0x0011, audio_probe, nullptr,  audio_write },
};

```text

---

### 💾 Types of Memory

| Type | Description | SigmaOS Mapping |
| --- | --- | --- |
| **RAM** | Volatile, fast, primary | `SovereignAllocator` — heap/stack manager |
| **ROM** | Non-volatile, firmware | Boot ROM mapped at `0x0000_0000` |
| **PROM** | Programmable ROM | Secure boot key storage in TPM |
| **EPROM** | Erasable PROM (UV) | Legacy BIOS compatibility layer |
| **EEPROM** | Electrically erasable | UEFI variable storage, NVram |
| **Cache** | L1/L2/L3 CPU cache | `SovereignCacheStore` prefetch hints |

```cpp
// kernel/core/SovereignAllocator.cpp
namespace Sigma::Memory {
    void* kmalloc(size_t size, MemoryTier tier);
    // tier: TIER_RAM | TIER_PERSISTENT | TIER_CACHED

    struct MemoryMap {
        uintptr_t rom_base;       // 0x0000_0000
        uintptr_t ram_base;       // 0x0010_0000 (1MB+)
        uintptr_t kernel_heap;    // Dynamic
        uintptr_t eeprom_region;  // UEFI NVram
    };
}

```text

---

### 💿 Storage Devices

| Device | Capacity | Speed | SigmaOS Driver |
| --- | --- | --- | --- |
| Hard Disk (HDD) | TB range | ~150 MB/s | `SovereignATA.cpp` |
| SSD (NVMe) | TB range | ~7 GB/s | `SovereignNVMe.cpp` |
| Flash Drive (USB) | GB range | ~400 MB/s | `SovereignUSBMass.cpp` |
| Optical Disk (CD/DVD/BD) | 700MB–50GB | ~72 MB/s | `SovereignOptical.cpp` |
| Cloud Storage | Unlimited | Network-speed | `SovereignCloudFS.cpp` |

### S-ZFS Unified Storage API

```cpp
// All storage types unified under S-ZFS
SovereignZFSPool pool;
pool.mount("/sigma/data", DEVICE_NVME, ZFS_RAIDZ);
pool.mount("/sigma/backup", DEVICE_HDD, ZFS_MIRROR);
pool.mount("/sigma/cloud", PROVIDER_S3, ZFS_CLOUD);

```text

---

### ☁️ Cloud Storage

**SigmaOS Implementation:** `SovereignCloudFS` — transparent cloud mount.

```cpp
// kernel/core/SovereignCloudFS.cpp
class SovereignCloudFS {
public:
    void mount_s3(const char* bucket, const char* mount_point);
    void mount_gcs(const char* bucket, const char* mount_point);
    void mount_azure(const char* container, const char* mount_point);
    void mount_sftp(const char* host, const char* mount_point);

    // Files appear locally under /sigma/cloud/
    // Encrypted with PQC Kyber-1024 in transit
};

```text

---

### ⚙️ Processor Types

| Type | Use Case | SigmaOS Scheduler |
| --- | --- | --- |
| CISC (x86-64) | Desktop/Server | Full support, native compilation |
| RISC (ARM64) | Mobile/Embedded | ARM64 HAL, energy-efficient scheduling |
| RISC-V | Open Hardware | Community HAL port |
| GPU (CUDA/OpenCL) | Parallel/AI | `SigmaGPUScheduler` for AI workloads |
| NPU (AI Chips) | Neural inference | `SigmaAI::NPURuntime` |
| DSP | Signal processing | `SigmaDSP` audio/signal stack |

```cpp
// kernel/core/SovereignScheduler.cpp
enum ProcessorTarget {
    CPU_X86_64,
    CPU_ARM64,
    CPU_RISCV,
    GPU_CUDA,
    GPU_OPENCL,
    NPU_GENERIC
};

void schedule_task(Task* t, ProcessorTarget target);

```text

---

## Unit III: Information Technology

### 📊 Data → Information → Knowledge Pipeline

```text
Raw Data (bytes) → Processed Information (context) → Knowledge (insight) → Action
     ↓                        ↓                             ↓
  SovereignFS            SigmaDB                      SigmaAI
  (stores data)      (queries/transforms)          (ML insights)

```text

**SigmaOS Knowledge Graph:** An in-kernel graph database connecting data entities.

```cpp
// kernel/core/SovereignKnowledgeGraph.cpp
class KnowledgeNode {
    std::string entity;
    std::vector<KnowledgeEdge> relations;
};

class SovereignKnowledgeGraph {
public:
    void add_fact(const char* subject, const char* predicate, const char* object);
    std::vector<KnowledgeNode> query(const char* sparql_like_query);
    void infer_rules();  // Forward chaining inference
};

```text

---

### 🛡️ Virus Detection & Prevention

**SigmaOS Implementation:** `SentinelNeural` — real-time integrity checker.

| Protection Layer | Mechanism |
| --- | --- |
| Boot Integrity | PQC-signed bootloader (CRYSTALS-Dilithium) |
| Kernel Integrity | Merkle-tree hash of all kernel modules |
| Process Isolation | Mandatory Access Control (MAC) sandboxing |
| File Integrity | Per-file BLAKE3 checksums in S-ZFS |
| Network | Stateful packet firewall + DPI |
| Behavior Analysis | `SentinelNeural` ML-based anomaly detection |

```cpp
// kernel/security/SentinelNeural.cpp
class SentinelNeural {
public:
    bool scan_process(pid_t pid);
    bool scan_file(const char* path);
    ThreatLevel assess_behavior(ProcessTrace* trace);
    void quarantine(pid_t pid);
    void rollback_to_snapshot(SnapshotID snap);
};

```text

---

## Unit IV: Office Automation

### 📄 SigmaDocs (MS Word Equivalent)

| Word Feature | SigmaDocs Equivalent |
| --- | --- |
| Text Formatting | Rich text engine (RTF/DOCX/ODF) |
| Tables | Native table renderer |
| Mail Merge | Template engine with SigmaDB data source |
| Track Changes | Git-based document versioning |
| Styles & Themes | CSS-like style sheets |

**File:** `userland/apps/SigmaDocs/`

---

### 📊 SigmaSheets (MS Excel Equivalent)

| Excel Feature | SigmaSheets Equivalent |
| --- | --- |
| Data Sorting | Multi-column sort engine |
| Filtering | AutoFilter with regex support |
| Pivot Tables | `SigmaSheets::PivotEngine` |
| Formulas | Full formula parser (500+ functions) |
| Charts | `SigmaViz` chart renderer |
| Macros | SigmaPy scripting integration |

**File:** `userland/apps/SigmaSheets/`

---

### 🎞️ SigmaSlides (MS PowerPoint Equivalent)

| PowerPoint Feature | SigmaSlides Equivalent |
| --- | --- |
| Slides | Vector-based slide canvas |
| Animations | CSS/WebGL transition engine |
| Multimedia | Native video/audio embedding |
| Themes | SVG-based theme system |
| Presenter Mode | Dual-display manager |

**File:** `userland/apps/SigmaSlides/`

---

### 🗄️ SigmaDB GUI (MS Access Equivalent)

| Access Feature | SigmaDB Equivalent |
| --- | --- |
| Table Designer | Visual schema editor |
| Query Builder | SQL visual query builder |
| Forms | Data entry form generator |
| Reports | PDF/HTML report engine |
| Relationships | ER diagram viewer |

**File:** `userland/apps/SigmaDB/`

---

## 🔗 Related Wiki Pages

- [Syllabus Implementation Map](Syllabus-Implementation-Map)

- [HAL Driver Framework](HAL)

- [Sovereign Memory Management](Sovereign-Memory-Management)

- [S-ZFS Storage Pool](Sovereign_ZFS_Pool)

- [SigmaAI Intelligence Layer](Syllabus-AIML)

---

### Last updated: 2026-05-18 | SigmaOS Zenith v15.1
