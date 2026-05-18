# C++ Programming → SigmaOS Kernel Core (C++17)

> Maps the C++ OOP syllabus to the SigmaOS Zenith microkernel, which is written entirely in modern C++17.

---

## Unit I: OOP Concepts & Basics

### OOP vs Procedural in SigmaOS

| Paradigm | Procedural (C) | OOP (C++) |
|---|---|---|
| Code style | Functions on data | Objects encapsulating data + behavior |
| Kernel use | Drivers, boot code | Core subsystems, HAL, shard modules |
| Reuse | Copy-paste, macros | Inheritance, templates |
| Safety | Manual conventions | Access specifiers, RAII |

### C vs C++ in SigmaOS

| Feature | C | C++ |
|---|---|---|
| `struct` | Data only | Data + methods |
| Memory | `malloc`/`free` | `new`/`delete` + RAII |
| Type safety | Casts are unsafe | `static_cast`, `dynamic_cast` |
| Templates | Macros | Type-safe generics |
| Namespaces | Global only | `Sigma::Core::Memory` |

```cpp
// C++ basics used throughout SigmaOS kernel
#include <cstdint>

namespace Sigma {
    // endl manipulator equivalent (kernel uses \n directly)
    // setw used in SigmaCLI output formatting
    constexpr int KERNEL_ALIGN = 16;

    // Reference variables
    void increment(int& value) { value++; }

    // Data types
    using u8  = uint8_t;
    using u64 = uint64_t;
    using i32 = int32_t;
}
```

---

## Unit II: Functions & Classes

```cpp
// --- Function Overloading ---
namespace Sigma::Log {
    void log(const char* msg);
    void log(const char* fmt, int val);
    void log(const char* fmt, uint64_t val);
    void log(const char* fmt, const char* str);
}

// --- Default Arguments ---
void* kmalloc(size_t size, MemoryFlags flags = MEM_KERNEL | MEM_ZEROED);

// --- Inline Function ---
inline bool is_page_aligned(uintptr_t addr) {
    return (addr & 0xFFF) == 0;
}

// --- Scope Resolution Operator ---
uint64_t Sigma::Memory::SovereignAllocator::total_allocated() {
    return m_allocated;  // :: accesses class member
}

// --- Friend Function ---
class SovereignProcess;
class SovereignScheduler {
    friend void debug_dump_process(SovereignProcess& p);  // access private members
};

// --- Classes & Objects ---
class SovereignProcess {
private:
    uint32_t   m_pid;
    char       m_name[64];
    uint64_t   m_stack_base;

protected:
    ProcessState m_state;

public:
    // Constructor
    SovereignProcess(uint32_t pid, const char* name);

    // Member functions
    uint32_t pid() const { return m_pid; }   // inline getter
    void     set_state(ProcessState s);

    // 'this' pointer
    SovereignProcess* get_self() { return this; }

    // Arrow operator usage: proc_ptr->pid()
};

// Function defined outside class using ::
void SovereignProcess::set_state(ProcessState s) {
    m_state = s;
}

// Array of objects
SovereignProcess proc_table[4096];

// Array within a class — page table
class AddressSpace {
    uint64_t m_pml4[512];  // Page Map Level 4 — 512 entries
};
```

---

## Unit III: Inheritance, Virtual Functions & Polymorphism

```cpp
// --- Base class: HAL Driver ---
class HALDriver {
protected:
    const char* m_name;
    bool        m_initialized;

public:
    explicit HALDriver(const char* name) : m_name(name), m_initialized(false) {}

    // Virtual function — overridden by specific drivers
    virtual int  probe()    { return -1; }
    virtual int  read(void* buf, size_t len)  = 0;  // Pure virtual
    virtual int  write(const void* buf, size_t len) = 0;  // Pure virtual
    virtual void reset() { m_initialized = false; }

    virtual ~HALDriver() {}  // Virtual destructor — essential for polymorphism
};

// --- Derived: NVMe Driver (public inheritance) ---
class NVMeDriver : public HALDriver {
    uint64_t m_bar_base;
public:
    NVMeDriver() : HALDriver("NVMe"), m_bar_base(0) {}
    int  probe()  override;
    int  read(void* buf, size_t len)  override;
    int  write(const void* buf, size_t len) override;
};

// --- Multiple Inheritance ---
class SovereignNetDev : public HALDriver, public NetworkInterface {
    // Virtual base class prevents diamond problem:
    // class A { }; class B : virtual A {}; class C : virtual A {}; class D : B, C {};
};

// --- Abstract Class — cannot be instantiated directly ---
class AbstractShard {
public:
    virtual void init()    = 0;
    virtual void update()  = 0;
    virtual void shutdown()= 0;
    virtual ~AbstractShard() = default;
};

// --- Polymorphism ---
HALDriver* drivers[] = { new NVMeDriver(), new ATADriver(), new USBDriver() };
for (auto* drv : drivers) {
    drv->probe();   // calls correct override at runtime (late binding / vtable)
}

// Early binding: resolved at compile time (non-virtual, templates)
// Late binding: resolved at runtime via vtable (virtual functions)

// --- Operator Overloading ---
class SovereignAddress {
    uint64_t m_addr;
public:
    SovereignAddress operator+(size_t offset) const {
        return SovereignAddress(m_addr + offset);
    }
    SovereignAddress& operator++() {      // Unary prefix ++
        m_addr += 4096; return *this;
    }
    bool operator==(const SovereignAddress& other) const {
        return m_addr == other.m_addr;
    }
};
```

---

## Unit IV: Constructors, Destructors, Files & Exceptions

```cpp
// --- Constructor Types ---
class SovereignBuffer {
    uint8_t* m_data;
    size_t   m_size;

public:
    SovereignBuffer()                        // Default constructor
        : m_data(nullptr), m_size(0) {}

    SovereignBuffer(size_t size)             // Parameterized constructor
        : m_data(new uint8_t[size]), m_size(size) {}

    SovereignBuffer(const SovereignBuffer& other)  // Copy constructor
        : m_data(new uint8_t[other.m_size]), m_size(other.m_size) {
        sigma_memcpy(m_data, other.m_data, m_size);
    }

    ~SovereignBuffer() {                     // Destructor — RAII cleanup
        delete[] m_data;
        m_data = nullptr;
    }
};

// Constructor/Destructor invocation order:
// Base ctor → Member ctors → Derived ctor
// Derived dtor → Member dtors → Base dtor

// --- File Management ---
#include <fstream>

class SovereignFileStream {
    std::fstream m_stream;
public:
    void open_read(const char* path) {
        m_stream.open(path, std::ios::in | std::ios::binary);
    }
    void open_write(const char* path) {
        m_stream.open(path, std::ios::out | std::ios::binary | std::ios::trunc);
    }
    void open_append(const char* path) {
        m_stream.open(path, std::ios::app);
    }
    // File modes: ios::in, ios::out, ios::binary, ios::ate, ios::app, ios::trunc
};

// --- Exception Handling ---
class SigmaKernelException : public std::exception {
    const char* m_msg;
public:
    explicit SigmaKernelException(const char* msg) : m_msg(msg) {}
    const char* what() const noexcept override { return m_msg; }
};

void risky_operation(int fd) {
    try {
        if (fd < 0) throw SigmaKernelException("Invalid file descriptor");
        if (!is_valid_fd(fd)) throw std::runtime_error("FD not open");
        perform_io(fd);
    }
    catch (const SigmaKernelException& e) {
        sigma_klog(LOG_ERROR, "Kernel error: %s\n", e.what());
    }
    catch (const std::exception& e) {
        sigma_klog(LOG_WARN, "std::exception: %s\n", e.what());
    }
    catch (...) {
        sigma_klog(LOG_CRIT, "Unknown exception — triggering panic\n");
        sigma_panic("unhandled exception");
    }
}
```

---

## SigmaOS C++ Standards & Conventions

| Rule | Detail |
|---|---|
| Standard | C++17 (`-std=c++17`) |
| RTTI | Disabled (`-fno-rtti`) — no `dynamic_cast` cost |
| Exceptions | Enabled only in userland; kernel uses error codes |
| STL | Forbidden in kernel; sovereign containers only |
| Namespaces | All kernel code in `Sigma::` hierarchy |
| Virtual dtors | Mandatory on all base classes with inheritance |

*Last updated: 2026-05-18 | SigmaOS Zenith v15.1*
