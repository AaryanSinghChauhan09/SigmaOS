# 🧠 OOP Architecture in SigmaOS (C11 Struct-Based Polymorphism)

SigmaOS implements Object-Oriented Programming **without any C++ runtime**, using pure C11 structs, function pointers, and macros defined in `libc/SigmaOOP.h`.

---

## The Problem with C++ in a Sovereign Kernel

Using C++ in a zero-dependency kernel would introduce:
- RTTI (Run Time Type Information) overhead
- `std::` namespace dependencies
- Implicit vtable management by the compiler
- ABI fragility

SigmaOS eliminates all of this by implementing OOP as **explicit C11 patterns**.

---

## Core Mechanism: `SigmaOOP.h`

```c
// CLASS_DECLARE creates a typedef'd struct automatically
#define CLASS_DECLARE(name) typedef struct name name##_t; struct name

// VIRTUAL defines a function pointer as a "virtual method"
#define VIRTUAL(ret, name, ...) ret (*name)(__VA_ARGS__)
```

---

## Base Class: `SigmaObject`

Every class in SigmaOS inherits from `SigmaObject_t`:

```c
CLASS_DECLARE(SigmaObject) {
    const char* class_name;   // Object's type name string
    sigma_u32   object_id;    // Unique runtime ID
    VIRTUAL(void, destroy, struct SigmaObject* self);  // Destructor
};
```

Initialization acts as a constructor call:
```c
sigma_object_init(&obj.core, "MyClass", 101);
```

---

## Example: `AIModel` Class

```c
CLASS_DECLARE(AIModel) {
    SigmaObject_t core;             // Inherits base
    const char*   name;
    const char*   internal_socket;
    sigma_u32     priority_weight;

    VIRTUAL(void, dispatch,     struct AIModel* self, const char* prompt);
    VIRTUAL(void, print_status, struct AIModel* self);
};
```

### Implementation (User-Defined Methods)

```c
static void sigma_local_model_dispatch(AIModel_t* self, const char* prompt) {
    sigma_printf("[AI] -> Routing to: ");
    sigma_printf(self->name);
    sigma_printf("\n");
}

static AIModel_t create_ai_model(const char* name, const char* sock, sigma_u32 prio) {
    AIModel_t m;
    sigma_object_init(&m.core, "AIModel", 102); // Super call
    m.name = name;
    m.internal_socket = sock;
    m.priority_weight = prio;
    m.dispatch = sigma_local_model_dispatch;    // Bind virtual method
    return m;
}
```

### Polymorphic Call
```c
AIModel_t llm = create_ai_model("Sigma_QWen_local", "/var/ipc/llm.sock", 100);
llm.dispatch(&llm, "Analyze kernel memory.");   // Runtime dispatch
```

---

## All OOP Classes in SigmaOS

| Class | File | Inherits | Virtual Methods |
|-------|------|----------|----------------|
| `AIModel_t` | `sigma_ai_distribute.c` | `SigmaObject_t` | `dispatch`, `print_status` |
| `NodeResource_t` | `sigma_auto_optimizer.c` | `SigmaObject_t` | `balance`, `scale_up`, `evict` |
| `MemoryScrubber_t` | `system_cleaner.c` | `SigmaObject_t` | `scrub`, `report` |

---

## Type System

All types are defined in `libc/sigma_types.h` — no external headers:

```c
typedef unsigned char      sigma_u8;
typedef unsigned short     sigma_u16;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;
typedef signed int         sigma_i32;
typedef signed long long   sigma_i64;
```

---

## Design Principles

- **No vtable magic**: All virtual dispatch is explicit via struct function pointers
- **Inheritance by embedding**: Child structs embed parent as first member
- **Constructor pattern**: Factory functions initialize + bind methods
- **Zero overhead**: No RTTI, no exceptions, no ABI hidden calls


---
## ADDED FROM Architecture-OOPS.md
# Σ SIGMAOS: ARCHITECTURE & OOPS FINALITY
[![OOPS](https://img.shields.io/badge/Architecture-OOPS-blue?style=for-the-badge)]()

**Σ SIGMAOS** is built on a strictly sharded, Object-Oriented architecture to ensure **Absolute Technical Encapsulation**.

## 🧱 THE SIGMASHARD BASE CLASS
- All professional tools (AI, DS, DSA) inherit from the **`SigmaShard`** base class.
- **Polymorphism**: Tools override the `execute()` and `render()` methods to provide domain-specific silicon logic.
- **Encapsulation**: Shard-state is contained within the class context, preventing unauthorized state-leaks between system modules.

## ⚙️ HLL-REDUCTION & SMU
- **Sovereign Math Unit (SMU)**: Replaces high-level `Math.*` dependencies with User-Defined Functions (UDFs).
- **Silicon Parity**: Browser-based shards use raw indexing and loops to mirror the Low-Level C Kernels.

## 📦 SHARDED BOUNDARIES
- **SigmaVFS**: Virtual File System shard.
- **SigmaWM**: Low-latency Window Manager.
- **SigmaShell**: C11-parity Command Interface.

---
**Σ SIGMAOS: ENCAPSULATED POWER. POLYMORPHIC FINALITY. 🧱🧠⚙️**
