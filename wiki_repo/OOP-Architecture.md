# SigmaOS OOP Architecture

> Full guide: [docs/OOP_Architecture.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/OOP_Architecture.md)

---

## OOP by Language

| Language | OOP Mechanism | Example |
|---|---|---|
| **Rust** | Traits = interfaces, Structs = classes | `SdfDriver`, `FileSystem`, `Widget` |
| **Zig** | Struct + methods, comptime interfaces | `VesaFb`, `Apic`, `PciBus` |
| **Nim** | `ref object` + `method` for virtual dispatch | `Widget → Button → TextInput` |
| **SPARK/Ada** | Tagged types + design-by-contract | `Kem_Algorithm → Kyber1024` |

---

## Pattern Inventory (20+ used)

| Pattern | Where | Language |
|---|---|---|
| Trait/Interface | SdfDriver, FileSystem, NicDevice, Widget | Rust |
| Builder | Animation::with_*, AppEntry::with_* | Rust |
| State Machine | TcpSocket, DhcpClient, WpaManager, NtpClient | Rust |
| Observer | NotificationManager, InputManager | Rust |
| Strategy | Layout (BSP/Grid/MasterStack), EasingFn | Rust |
| Command | DrawCmd, VaultRequest, PkgAction | Rust |
| Chain of Responsibility | syscall → pledge → unveil → seccomp → MAC | Rust |
| Composite | Panel → PanelItem hierarchy | Rust |
| State/Struct methods | VesaFb, Vmm, PciBus | Zig |
| Virtual dispatch | Widget.handle() → Button.handle() | Nim |
| Design by Contract | Kyber1024.KeyGen with Pre/Post | SPARK |

---

## Core OOP Rules

**Rust**: traits > inheritance, composition > extension, generics for hot paths, dyn only for runtime polymorphism  
**Zig**: every struct has `init()`, no global mutable state outside AtomicXxx  
**Nim**: `method` for virtual, `proc` for non-virtual, no implicit globals  
**SPARK**: Pre+Post on all public operations, gnatprove level-2, no side effects on Pure

---

## Upcoming Improvements

1. **Typed error hierarchy** — `SigmaError { Kernel | Fs | Net | Security | Crypto }`
2. **Capability-typed API** — phantom types prevent privilege escalation at compile time
3. **Plugin system** — `Plugin` trait + dynamic loading via sigpkg
4. **Reactive state (MVI)** — `Store<S,A>` with reducer + subscribers for all app state
5. **Async/await** — `sigma-async` cooperative runtime, no_std compatible

---

*See: [Contributing Guide](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CONTRIBUTING.md)*
