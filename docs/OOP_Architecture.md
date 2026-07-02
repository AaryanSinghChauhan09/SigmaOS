# SigmaOS OOP Architecture Guide

> How object-oriented principles are applied across Rust, Zig, Nim, and SPARK/Ada.

---

## OOP in SigmaOS: Language-by-Language

### Rust — Traits as Interfaces

```rust
// ── CORE OOP PATTERN: Trait = interface, Struct = concrete class ──

// 1. Define the interface (trait)
pub trait SdfDriver: Send + Sync {
    fn probe(dev: &DeviceId) -> bool where Self: Sized;
    fn init(&mut self)     -> SdfResult<()>;
    fn shutdown(&mut self);
    fn name(&self)         -> &'static str;
}

// 2. Concrete implementation (struct + impl)
pub struct E1000Driver { mmio_base: usize, rx_ring: RxRing }

impl SdfDriver for E1000Driver {
    fn probe(dev: &DeviceId) -> bool { dev.vendor == 0x8086 }
    fn init(&mut self)       -> SdfResult<()> { self.reset(); self.setup_rings() }
    fn shutdown(&mut self)   { self.flush(); }
    fn name(&self)           -> &'static str { "sigma-e1000" }
}

// 3. Polymorphism via trait objects
fn register_driver(driver: Box<dyn SdfDriver>) { ... }
```

#### Inheritance via Composition

```rust
// SigmaOS uses composition, not inheritance
pub struct TlsSocket {
    tcp:    TcpSocket,          // has-a TCP socket
    crypto: TlsSession,         // has-a TLS session
}

impl TlsSocket {
    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let encrypted = self.tcp.recv(buf);
        self.crypto.decrypt(buf, encrypted)
    }
}
```

#### Builder Pattern (OOP construction)

```rust
pub struct FilterBuilder { filter: LogFilter }
impl FilterBuilder {
    pub fn new()              -> Self { Self { filter: LogFilter::default() } }
    pub fn level(mut self, l: Level) -> Self { self.filter.min_level = Some(l); self }
    pub fn source(mut self, s: &str) -> Self { self.filter.source = Some(s.to_owned()); self }
    pub fn search(mut self, q: &str) -> Self { self.filter.search = Some(q.to_owned()); self }
    pub fn build(self)          -> LogFilter { self.filter }
}

// Usage — fluent API
let filter = FilterBuilder::new()
    .level(Level::Warn)
    .source("kernel")
    .build();
```

#### Observer Pattern (event system)

```rust
pub trait Observer<E>: Send {
    fn on_event(&mut self, event: &E);
}

pub struct EventBus<E> {
    observers: Vec<Box<dyn Observer<E>>>,
}

impl<E: Clone> EventBus<E> {
    pub fn subscribe(&mut self, obs: Box<dyn Observer<E>>) { self.observers.push(obs); }
    pub fn emit(&mut self, event: E) {
        for obs in &mut self.observers { obs.on_event(&event); }
    }
}
```

#### State Machine Pattern

```rust
// Used in TcpSocket, DhcpClient, WpaManager, etc.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TcpState { Closed, Listen, SynSent, SynReceived, Established, ... }

pub struct TcpSocket { state: TcpState, ... }

impl TcpSocket {
    pub fn transition(&mut self, event: TcpEvent) -> Result<(), TcpError> {
        self.state = match (self.state, event) {
            (TcpState::Closed,       TcpEvent::Listen)   => TcpState::Listen,
            (TcpState::Listen,       TcpEvent::SynRcvd)  => TcpState::SynReceived,
            (TcpState::Established,  TcpEvent::FinSent)  => TcpState::FinWait1,
            _ => return Err(TcpError::InvalidTransition),
        };
        Ok(())
    }
}
```

---

### Zig — Structs with Methods

```zig
// Zig: no inheritance, but comptime interfaces via anytype
pub const VesaFb = struct {
    base:   usize,
    width:  u32,
    height: u32,

    // Constructor
    pub fn init(info: *const BootInfo) VesaFb {
        return VesaFb{ .base=info.framebuffer, .width=info.fb_width, .height=info.fb_height };
    }

    // Methods
    pub fn put_pixel(self: *const VesaFb, x: u32, y: u32, color: Color) void { ... }
    pub fn fill_rect(self: *const VesaFb, rect: Rect, color: Color) void { ... }
    pub fn clear(self: *const VesaFb, color: Color) void { ... }
};

// Comptime interface (duck typing)
pub fn render_to(fb: anytype, rect: Rect, color: Color) void {
    // Works with any type that has fill_rect
    fb.fill_rect(rect, color);
}
```

#### Tagged Union (Sum Types / ADT)

```zig
pub const DriverResult = union(enum) {
    Ok:    void,
    Error: DriverError,
    Retry: u32,  // retry after N ticks
};

pub fn probe_device(pci: *PciDevice) DriverResult {
    if (pci.vendor == 0x8086 and pci.device == 0x100E) return .Ok;
    if (pci.vendor == 0xFFFF) return .{ .Retry = 100 };
    return .{ .Error = DriverError.UnknownDevice };
}
```

---

### Nim — Object Variants + Method Dispatch

```nim
# OOP via object + methods + inheritance (ref object)
type
  Widget = ref object of RootObj   # base class
    id:      WidgetId
    bounds:  Rect
    visible: bool

  Button = ref object of Widget    # derived
    label:   string
    pressed: bool

  TextInput = ref object of Widget
    text:    string
    focused: bool

# Virtual methods via method keyword
method handle(w: Widget, ev: InputEvent): Option[WidgetEvent] {.base.} =
  discard

method handle(b: Button, ev: InputEvent): Option[WidgetEvent] =
  if ev of MouseDownEvent:
    b.pressed = true
    return some(WidgetEvent(kind: Clicked, id: b.id))

# Polymorphic dispatch
proc dispatch_all(widgets: seq[Widget], ev: InputEvent) =
  for w in widgets: discard w.handle(ev)   # virtual dispatch
```

---

### SPARK/Ada — Formal Contracts as OOP

```ada
-- Ada tagged types = classes
-- SPARK Pre/Post = design-by-contract OOP

package Sigma.Crypto.Kyber
  with SPARK_Mode => On
is
   -- Abstract type (interface)
   type Kem_Algorithm is abstract tagged null record;

   -- Abstract operations (virtual methods with contracts)
   procedure KeyGen
     (Algorithm  : in out Kem_Algorithm;
      Public_Key : out    Bytes_512;
      Secret_Key : out    Bytes_1024)
   is abstract
   with Post => Public_Key'Length = 512;

   -- Concrete implementation
   type Kyber1024 is new Kem_Algorithm with private;

   overriding procedure KeyGen
     (Algorithm  : in out Kyber1024;
      Public_Key : out    Bytes_512;
      Secret_Key : out    Bytes_1024)
   with
     SPARK_Mode => On,
     Post => (for all I in Public_Key'Range => Public_Key(I) /= 0);

private
   type Kyber1024 is new Kem_Algorithm with record
      Ntt_Cache : Poly_Array;
   end record;
end Sigma.Crypto.Kyber;
```

---

## OOP Patterns Used Across SigmaOS

### Pattern Inventory

| Pattern | Where Used | Language |
|---|---|---|
| **Trait / Interface** | SdfDriver, FileSystem, AudioDevice, NicDevice, Widget, Benchmark | Rust |
| **Builder** | FilterBuilder, Animation::with_*, AppEntry::with_* | Rust |
| **State Machine** | TcpSocket, DhcpClient, WpaManager, NtpClient, QuicConn | Rust |
| **Observer / Event Bus** | NotificationManager, InputManager, AnimationEngine | Rust |
| **Strategy** | Layout (MasterStack/Grid/BSP), EasingFn, Collector | Rust |
| **Command** | DrawCmd (renderer), VaultRequest, PkgAction | Rust |
| **Decorator** | LandlockRuleset wrapping sigma_unveil | Rust |
| **Factory** | SvhStore::issue(), ProcessManager::fork() | Rust |
| **Singleton** | ASLR_STATE (AtomicU64), NEXT_TOKEN (AtomicU64) | Rust |
| **Flyweight** | SlabCache (object caching in kmalloc) | Rust |
| **Chain of Responsibility** | syscall → pledge → unveil → seccomp → MAC → AVC | Rust |
| **Composite** | Widget hierarchy (Panel → items) | Rust |
| **Facade** | VaultDaemon wrapping MetricRegistry + Unix socket | Rust |
| **Template Method** | SettingsPanel trait (apply/load/save hook methods) | Rust |
| **Struct + Methods** | VesaFb, Apic, Vmm, Idt, PciBus | Zig |
| **Comptime Interface** | render_to(fb: anytype) duck typing | Zig |
| **ref object inheritance** | Widget → Button → TextInput | Nim |
| **Virtual dispatch** | method handle() → Nim runtime dispatch | Nim |
| **Design by Contract** | Pre/Post on all SPARK procedures | SPARK |
| **Tagged type inheritance** | Kem_Algorithm → Kyber1024 | Ada |

---

## OOP Quality Rules (enforced by CI)

### Rust
```
✓ No naked unsafe blocks without justification comment
✓ Every public trait has a doc comment with contract
✓ No God structs > 500 lines — split into smaller types
✓ Prefer generics over dyn for hot paths (zero-cost)
✓ Use dyn only when runtime polymorphism is essential
✓ No inheritance chains — use composition
✓ Builder pattern for structs with > 5 fields
✓ State machine enums with transition() method, not ad-hoc if chains
```

### Zig
```
✓ Every struct has init() constructor
✓ No global mutable state outside AtomicXxx
✓ Comptime interfaces preferred over anytype where possible
✓ Error union return type for all fallible operations
```

### Nim
```
✓ Use ref object for heap types, object for value types
✓ method for virtual dispatch, proc for non-virtual
✓ No implicit globals — pass state explicitly
✓ nimcall for all platform callbacks
```

### SPARK
```
✓ All public subprograms have Pre + Post contracts
✓ SPARK_Mode => On for all crypto and memory subsystems
✓ gnatprove --level=2 passes with zero unproved VCs
✓ No side effects on Pure functions
```

---

## Upcoming OOP Improvements

### v1.0 — Type Safety Improvements

```rust
// Replace stringly-typed errors with typed error hierarchies
pub enum SigmaError {
    Kernel(KernelError),
    Fs(FsError),
    Net(NetError),
    Security(SecurityError),
    Crypto(CryptoError),
}

impl From<KernelError> for SigmaError {
    fn from(e: KernelError) -> Self { Self::Kernel(e) }
}
```

### v1.0 — Effect System (capabilities as types)

```rust
// Phantom type tags for capability-gated operations
pub struct Privileged;
pub struct Unprivileged;

pub struct File<C> { fd: u32, _cap: core::marker::PhantomData<C> }

impl File<Privileged>   { pub fn write(&mut self, ...) { ... } }
impl File<Unprivileged> { /* write not available */ }
```

### v1.5 — Async/Await for I/O

```rust
// sigma-async: cooperative async runtime (no_std compatible)
pub trait AsyncDriver: Send {
    async fn recv(&mut self) -> Option<Vec<u8>>;
    async fn send(&mut self, data: &[u8]) -> bool;
}
```

---

*See also: [CONTRIBUTING.md](../CONTRIBUTING.md) · [docs/Language_Implementation_Policy.md](Language_Implementation_Policy.md)*
