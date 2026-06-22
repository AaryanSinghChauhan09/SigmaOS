# C++ Programming → SigmaOS Kernel Core (C++17)

> Maps the C++ OOP syllabus to the SigmaOS Zenith microkernel, which is written entirely in modern, freestanding C++17.

---

## Unit I: OOP Principles, Concepts & Basics

### Core OOP Principles

* **Encapsulation:** Bundling data attributes and member functions into unified class abstractions, shielding internal state via explicit access specifiers (`private`, `protected`, `public`).
* **Inheritance:** Establishing hierarchical relationships between base and derived classes, enabling structural code reuse and interface subtyping E.g., `NVMeDriver` inheriting from `HALDriver`.
* **Polymorphism:** Permitting distinct derived objects to be treated uniformly via base pointers, utilizing dynamic vtable dispatch for late binding or templates for static compile-time resolution.
* **Abstraction:** Exposing simplified, high-level operational interfaces while hiding complex internal implementation mechanics E.g., presenting a clean `read()` method while concealing low-level DMA register manipulation.

**Unique Selling Point (USP):** Unmatched reusability, modularity, and clean architectural boundaries, enabling failure-isolated kernel shards and rapid ecosystem scaling.

### OOP vs Procedural in SigmaOS

| Paradigm | Procedural (C) | OOP (C++) |
| :--- | :--- | :--- |
| **Code style** | Functions operating on external data structures | Objects encapsulating data attributes + behavioral methods |
| **Kernel use** | Low-level assembly traps, early boot loaders | Core microkernel subsystems, HAL, modular utility shards |
| **Reuse** | Copy-paste mechanics, macro expansions | Inheritance hierarchies, template metaprogramming |
| **Safety** | Manual pointer discipline and conventions | Access specifiers, RAII deterministic resource management |

### C vs C++ in SigmaOS

| Feature | C | C++ |
| :--- | :--- | :--- |
| `struct` | Plain Old Data (POD) only | Data attributes + member methods |
| **Memory** | `malloc`/`free` manual management | `new`/`delete` + RAII smart pointers |
| **Type safety**| Implicit C-style casts are unsafe | Explicit `static_cast`, `reinterpret_cast` |
| **Templates** | Preprocessor macros | Type-safe generic programming |
| **Namespaces** | Global symbol space only | Hierarchical `Sigma::Core::Memory` |

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

## Debugging & Problem-Solving in C++ OOP

### Common Issues & Fix Strategies

* **Issue - Memory Leaks & Dangling Pointers:** Manual `new`/`delete` mismanagement leaves orphaned heap allocations or dangling pointer references.
  * *Fix Strategy:* Enforce strict RAII smart pointer wrapping (`SigmaUniquePtr`, `SigmaSharedPtr`) to guarantee deterministic heap deallocation upon scope exit.
* **Issue - Concurrency Deadlocks in Object Methods:** Multiple threads invoking synchronized class methods acquire member mutexes in conflicting orders.
  * *Fix Strategy:* Utilize `std::scoped_lock` (or sovereign equivalent) for deadlock-free multi-lock acquisition, and adhere to strict hierarchical locking protocols across object boundaries.
* **Issue - Virtual Table (vtable) Slicing & Corruption:** Passing derived objects by value rather than reference/pointer slices off derived attributes and corrupts polymorphic vtable dispatch.
  * *Fix Strategy:* Always pass polymorphic objects by reference (`const HALDriver&`) or smart pointer (`SigmaUniquePtr<HALDriver>`), and enforce mandatory `virtual` destructors on all base classes.
* **Issue - Algorithmic Complexity in Container Traversal:** Linear array scanning ($O(n)$) or naive sorting ($O(n^2)$) degrades object container performance.
  * *Fix Strategy:* Migrate from linear vectors to balanced B+ Trees or hash maps (`SovereignHashMap`), reducing search and indexing complexity to $O(\log n)$ or $O(1)$.

---

## SigmaOS C++ Standards & Conventions

| Rule | Detail |
| :--- | :--- |
| **Standard** | C++17 (`-std=c++17`) |
| **RTTI** | Disabled (`-fno-rtti`) — eliminates `dynamic_cast` runtime overhead |
| **Exceptions** | Enabled only in userland; microkernel core utilizes explicit error codes |
| **STL** | Forbidden in Ring-0 kernel memory; sovereign containers only |
| **Namespaces**| All kernel code encapsulated within `Sigma::` hierarchy |
| **Virtual dtors**| Mandatory on all base classes containing virtual methods |

*Last updated: 2026-05-19 | SigmaOS Zenith v15.2*
