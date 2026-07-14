# SigmaOS High-Priority Absorption Implementation Plans

## Overview

This document provides detailed implementation plans for the highest-priority open-source projects to absorb into SigmaOS. These projects are selected based on: (1) permissive licensing, (2) high impact on roadmap goals, and (3) feasibility of integration in 1-3 sprints.

## Priority 1: WASM Runtime (Critical for Phase 2)

### Target: Wasmtime + Wasmer

**License**: Apache-2.0 (Wasmtime), MIT (Wasmer)
**Impact**: Enables WASM-first app ecosystem (Phase 2 goal)
**Feasibility**: Very High - permissive licenses
**Timeline**: 2 sprints (6 weeks)

#### Implementation Plan

### Sprint 1: Integration & Basic Runtime

- [ ] Add Wasmtime as dependency in Cargo.toml

- [ ] Create sigmad-sandbox/wasm-runtime module

- [ ] Implement basic WASM execution wrapper

- [ ] Add capability-based syscall filtering

- [ ] Create WASI syscalls mapping to SigmaOS syscalls

- [ ] Add basic resource limits (memory, CPU)

### Sprint 2: Advanced Features

- [ ] Implement WASI filesystem access with pledge/unveil

- [ ] Add network capability filtering

- [ ] Implement WASM module caching

- [ ] Add performance monitoring and profiling

- [ ] Create WASM package format for sigpkg

- [ ] Write integration tests

**Repository Mapping**: sigmad-sandbox/wasm-runtime
**Branch**: sigmad-sandbox
**Effort**: 2 engineers × 6 weeks

#### Code Structure

```rust
// sigmad-sandbox/wasm-runtime/src/lib.rs
pub struct WasmRuntime {
    engine: wasmtime::Engine,
    linker: wasmtime::Linker<WasmContext>,
    capabilities: CapabilitySet,
}

impl WasmRuntime {
    pub fn new(capabilities: CapabilitySet) -> Self {
        let engine = wasmtime::Engine::new(
            &wasmtime::Config::new()
                .wasm_simd(true)
                .wasm_bulk_memory(true)
                .max_memory_pages(1024)
        ).unwrap();

        let mut linker = wasmtime::Linker::new(&engine);

        // Add WASI support
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)
            .unwrap();

        WasmRuntime {
            engine,
            linker,
            capabilities,
        }
    }

    pub fn execute(&self, module: &[u8]) -> Result<WasmOutput> {
        let module = wasmtime::Module::from_binary(&self.engine, module)?;
        let mut store = wasmtime::Store::new(&self.engine, WasmContext::new());

        // Apply capability checks
        self.check_capabilities(&mut store)?;

        let instance = self.linker.instantiate(&mut store, &module)?;
        let run = instance.get_typed_func::<(), ()>(&mut store, "run")?;

        run.call(&mut store, ())?;

        Ok(WasmOutput::from_store(store))
    }
}
```

## Priority 2: Network Stack (Critical for Phase 1)

### Target: smoltcp

**License**: MIT
**Impact**: Provides Rust TCP/IP stack for embedded/networking
**Feasibility**: Very High - MIT license
**Timeline**: 1 sprint (3 weeks)

#### Implementation Plan

### Week 1: Basic Integration

- [ ] Add smoltcp as dependency

- [ ] Create net/smoltcp module

- [ ] Implement basic Ethernet driver interface

- [ ] Add ARP support

- [ ] Implement basic IPv4 stack

### Week 2: Advanced Features

- [ ] Add TCP/UDP support

- [ ] Implement DNS client

- [ ] Add DHCP client

- [ ] Create network configuration API

- [ ] Add packet filtering support

### Week 3: Integration & Testing

- [ ] Integrate with VirtIO network driver

- [ ] Add network performance tests

- [ ] Create network monitoring tools

- [ ] Write documentation

**Repository Mapping**: net/smoltcp
**Branch**: drivers-dev
**Effort**: 1 engineer × 3 weeks

#### Code Structure

```rust
// net/smoltcp/src/lib.rs
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr};
use smoltcp::iface::{EthernetInterface, Interface};
use smoltcp::socket::{SocketSet, TcpSocket, UdpSocket};

pub struct SigmaNetworkStack {
    interface: Interface,
    sockets: SocketSet<'static>,
    ethernet_addr: EthernetAddress,
    ip_addr: IpCidr,
}

impl SigmaNetworkStack {
    pub fn new(ethernet_addr: EthernetAddress, ip_addr: IpCidr) -> Self {
        let mut interface = Interface::new(
            Config::new(ethernet_addr),
            &mut device,
        );

        interface.update_ip_addrs(|addrs| {
            addrs.push(ip_addr).unwrap();
        });

        SigmaNetworkStack {
            interface,
            sockets: SocketSet::new(vec![]),
            ethernet_addr,
            ip_addr,
        }
    }

    pub fn poll(&mut self, timestamp: Instant) {
        self.interface.poll(timestamp).unwrap();
    }

    pub fn tcp_connect(&mut self, remote: IpAddress, port: u16) -> Result<TcpSocketHandle> {
        let rx_buffer = TcpSocket::new_rx_buffer(1024);
        let tx_buffer = TcpSocket::new_tx_buffer(1024);
        let socket = TcpSocket::new(rx_buffer, tx_buffer);

        let handle = self.sockets.add(socket);
        self.sockets.get_mut::<TcpSocket>(handle)
            .unwrap()
            .connect((remote, port), 49152)?;

        Ok(handle)
    }
}
```

## Priority 3: Crypto Primitives (Critical for Security)

### Target: libsodium

**License**: ISC
**Impact**: Provides modern crypto primitives for post-quantum transition
**Feasibility**: Very High - ISC license
**Timeline**: 1 sprint (3 weeks)

#### Implementation Plan

### Week 1: Integration

- [ ] Add libsodium-sys as dependency

- [ ] Create crypto/libsodium wrapper

- [ ] Implement key generation functions

- [ ] Add encryption/decryption wrappers

- [ ] Implement signature functions

### Week 2: SigmaOS Integration

- [ ] Integrate with capability system

- [ ] Add hardware acceleration support

- [ ] Implement secure key storage

- [ ] Add key derivation functions

- [ ] Create crypto performance benchmarks

### Week 3: Testing & Documentation

- [ ] Add comprehensive tests

- [ ] Write security documentation

- [ ] Create crypto API reference

- [ ] Add integration with TPM

**Repository Mapping**: crypto/libsodium
**Branch**: main
**Effort**: 1 engineer × 3 weeks

#### Code Structure

```rust
// crypto/libsodium/src/lib.rs
use libsodium_sys as sys;

pub struct SigmaCrypto {
    context: *mut sys::crypto_aead_xchacha20poly1305_ietf_state,
}

impl SigmaCrypto {
    pub fn new() -> Self {
        // Initialize libsodium
        unsafe {
            sys::sodium_init();
        }

        SigmaCrypto {
            context: std::ptr::null_mut(),
        }
    }

    pub fn encrypt(&self, plaintext: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        let mut ciphertext = vec![0u8; plaintext.len() + 16];

        unsafe {
            let result = sys::crypto_aead_xchacha20poly1305_ietf_encrypt(
                ciphertext.as_mut_ptr(),
                std::ptr::null_mut(),
                plaintext.as_ptr(),
                plaintext.len() as u64,
                std::ptr::null(),
                0,
                nonce.as_ptr(),
                key.as_ptr(),
            );

            if result != 0 {
                return Err(Error::EncryptionFailed);
            }
        }

        Ok(ciphertext)
    }

    pub fn decrypt(&self, ciphertext: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        let mut plaintext = vec![0u8; ciphertext.len() - 16];

        unsafe {
            let result = sys::crypto_aead_xchacha20poly1305_ietf_decrypt(
                plaintext.as_mut_ptr(),
                std::ptr::null_mut(),
                std::ptr::null(),
                ciphertext.as_ptr(),
                ciphertext.len() as u64,
                std::ptr::null(),
                0,
                nonce.as_ptr(),
                key.as_ptr(),
            );

            if result != 0 {
                return Err(Error::DecryptionFailed);
            }
        }

        Ok(plaintext)
    }
}
```

## Priority 4: Wayland Compositor (Critical for Desktop)

### Target: wlroots

**License**: MIT
**Impact**: Provides Wayland compositor foundation for Zenith desktop
**Feasibility**: Very High - MIT license
**Timeline**: 2 sprints (6 weeks)

#### Implementation Plan

### Sprint 1: Basic Compositor

- [ ] Add wlroots as dependency

- [ ] Create desktop/graphics module

- [ ] Implement basic compositor loop

- [ ] Add output management

- [ ] Implement input handling

- [ ] Add basic window management

### Sprint 2: Advanced Features

- [ ] Implement layer shell protocol

- [ ] Add XDG shell support

- [ ] Implement drag-and-drop

- [ ] Add screenshot support

- [ ] Integrate with SigmaOS graphics drivers

- [ ] Add performance optimizations

**Repository Mapping**: desktop/graphics
**Branch**: release/standalone
**Effort**: 2 engineers × 6 weeks

#### Code Structure

```rust
// desktop/graphics/src/compositor.rs
use wlroots::compositor::{Compositor, CompositorHandler};
use wlroots::output::Output;
use wlroots::input::InputDevice;

pub struct SigmaCompositor {
    compositor: Compositor,
    outputs: Vec<Output>,
    input_devices: Vec<InputDevice>,
}

impl SigmaCompositor {
    pub fn new() -> Self {
        let compositor = Compositor::new();

        SigmaCompositor {
            compositor,
            outputs: vec![],
            input_devices: vec![],
        }
    }

    pub fn run(&mut self) {
        // Main compositor loop
        loop {
            self.compositor.dispatch().unwrap();

            // Render outputs
            for output in &self.outputs {
                self.render_output(output);
            }

            // Handle input
            for device in &self.input_devices {
                self.handle_input(device);
            }
        }
    }
}

impl CompositorHandler for SigmaCompositor {
    fn output_frame(&mut self, output: &mut Output) {
        // Render frame for output
        self.render_output(output);
    }

    fn input_device_added(&mut self, device: InputDevice) {
        self.input_devices.push(device);
    }
}
```

## Priority 5: Async Runtime (Critical for Userland)

### Target: Tokio

**License**: MIT
**Impact**: Provides async runtime for userland services
**Feasibility**: Very High - MIT license
**Timeline**: 1 sprint (3 weeks)

#### Implementation Plan

### Week 1: Integration

- [ ] Add tokio as dependency

- [ ] Create userland/libs/rust-async module

- [ ] Implement async syscall wrappers

- [ ] Add async I/O operations

- [ ] Create async task scheduler

### Week 2: SigmaOS Integration

- [ ] Integrate with capability system

- [ ] Add async network stack integration

- [ ] Implement async filesystem operations

- [ ] Add async process management

- [ ] Create async IPC mechanisms

### Week 3: Testing & Documentation

- [ ] Add async performance tests

- [ ] Write async programming guide

- [ ] Create async examples

- [ ] Add async debugging tools

**Repository Mapping**: userland/libs/rust-async
**Branch**: main
**Effort**: 1 engineer × 3 weeks

#### Code Structure

```rust
// userland/libs/rust-async/src/lib.rs
use tokio::runtime::Runtime;
use tokio::task;

pub struct SigmaAsyncRuntime {
    runtime: Runtime,
}

impl SigmaAsyncRuntime {
    pub fn new() -> Self {
        let runtime = Runtime::new().unwrap();

        SigmaAsyncRuntime { runtime }
    }

    pub fn spawn<F>(&self, future: F) -> task::JoinHandle<F::Output>
    where
        F: futures::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.runtime.spawn(future)
    }

    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: futures::Future,
    {
        self.runtime.block_on(future)
    }
}

// Async syscall wrappers
pub async fn async_read(fd: i32, buf: &mut [u8]) -> Result<usize> {
    task::spawn_blocking(move || {
        sys_file_read(fd, buf)
    }).await?
}
```

## Priority 6: Embedded Database (Critical for sigpkg)

### Target: SQLite

**License**: Public Domain
**Impact**: Provides embedded database for package metadata
**Feasibility**: Very High - public domain
**Timeline**: 1 sprint (3 weeks)

#### Implementation Plan

### Week 1: Integration

- [ ] Add rusqlite as dependency

- [ ] Create userland/lib/sqlite module

- [ ] Implement database schema for sigpkg

- [ ] Add migration system

- [ ] Create query builders

### Week 2: SigmaOS Integration

- [ ] Integrate with sigpkg registry

- [ ] Add transaction support

- [ ] Implement caching layer

- [ ] Add backup/restore

- [ ] Create performance optimizations

### Week 3: Testing & Documentation

- [ ] Add database tests

- [ ] Write database documentation

- [ ] Create migration scripts

- [ ] Add database monitoring

**Repository Mapping**: userland/lib/sqlite
**Branch**: sigma-pkg
**Effort**: 1 engineer × 3 weeks

#### Code Structure

```rust
// userland/lib/sqlite/src/lib.rs
use rusqlite::{Connection, Result};

pub struct SigmaDatabase {
    conn: Connection,
}

impl SigmaDatabase {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;

        // Enable WAL mode for better concurrency
        conn.execute("PRAGMA journal_mode=WAL", [])?;

        Ok(SigmaDatabase { conn })
    }

    pub fn init_schema(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS packages (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                version TEXT NOT NULL,
                signature BLOB NOT NULL,
                metadata BLOB,
                created_at INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    pub fn insert_package(&self, name: &str, version: &str, signature: &[u8]) -> Result<()> {
        self.conn.execute(
            "INSERT INTO packages (name, version, signature, created_at) VALUES (?1, ?2, ?3, ?4)",
            [name, version, signature, &SystemTime::now()],
        )?;

        Ok(())
    }
}
```

## Priority 7: Metrics Collection (Critical for Observability)

### Target: Prometheus

**License**: Apache-2.0
**Impact**: Provides metrics collection for observability
**Feasibility**: Very High - Apache-2.0 license
**Timeline**: 1 sprint (3 weeks)

#### Implementation Plan

### Week 1: Integration

- [ ] Add prometheus-client as dependency

- [ ] Create userland/observability module

- [ ] Implement metric registry

- [ ] Add counter/gauge/histogram metrics

- [ ] Create metric exporters

### Week 2: SigmaOS Integration

- [ ] Add kernel metrics collection

- [ ] Implement process metrics

- [ ] Add network metrics

- [ ] Create metrics API

- [ ] Add metrics aggregation

### Week 3: Testing & Documentation

- [ ] Add metrics tests

- [ ] Write metrics documentation

- [ ] Create metrics dashboard

- [ ] Add alerting integration

**Repository Mapping**: userland/observability
**Branch**: main
**Effort**: 1 engineer × 3 weeks

#### Code Structure

```rust
// userland/observability/src/lib.rs
use prometheus::{Counter, Gauge, Histogram, Registry};

pub struct SigmaMetrics {
    registry: Registry,
    boot_time: Histogram,
    memory_usage: Gauge,
    cpu_usage: Gauge,
    network_bytes: Counter,
}

impl SigmaMetrics {
    pub fn new() -> Self {
        let registry = Registry::new();

        let boot_time = Histogram::with_opts(
            HistogramOpts::new("sigma_boot_time_seconds", "Boot time in seconds")
                .buckets(vec![0.5, 1.0, 2.0, 5.0, 10.0])
        ).unwrap();

        let memory_usage = Gauge::new("sigma_memory_usage_bytes", "Memory usage in bytes").unwrap();
        let cpu_usage = Gauge::new("sigma_cpu_usage_percent", "CPU usage percentage").unwrap();
        let network_bytes = Counter::new("sigma_network_bytes_total", "Total network bytes").unwrap();

        registry.register(Box::new(boot_time.clone())).unwrap();
        registry.register(Box::new(memory_usage.clone())).unwrap();
        registry.register(Box::new(cpu_usage.clone())).unwrap();
        registry.register(Box::new(network_bytes.clone())).unwrap();

        SigmaMetrics {
            registry,
            boot_time,
            memory_usage,
            cpu_usage,
            network_bytes,
        }
    }

    pub fn record_boot_time(&self, duration: Duration) {
        self.boot_time.observe(duration.as_secs_f64());
    }

    pub fn export(&self) -> String {
        prometheus::TextEncoder::new()
            .encode_to_string(&self.registry)
            .unwrap()
    }
}
```

## Priority 8: Tracing (Critical for Debugging)

### Target: OpenTelemetry

**License**: Apache-2.0
**Impact**: Provides distributed tracing for debugging
**Feasibility**: Very High - Apache-2.0 license
**Timeline**: 1 sprint (3 weeks)

#### Implementation Plan

### Week 1: Integration

- [ ] Add opentelemetry-rust as dependency

- [ ] Create kernel/tracing module

- [ ] Implement trace context propagation

- [ ] Add span creation

- [ ] Create trace exporters

### Week 2: SigmaOS Integration

- [ ] Add kernel tracing points

- [ ] Implement syscall tracing

- [ ] Add driver tracing

- [ ] Create trace aggregation

- [ ] Add trace visualization

### Week 3: Testing & Documentation

- [ ] Add tracing tests

- [ ] Write tracing documentation

- [ ] Create tracing examples

- [ ] Add trace analysis tools

**Repository Mapping**: kernel/tracing
**Branch**: main
**Effort**: 1 engineer × 3 weeks

#### Code Structure

```rust
// kernel/tracing/src/lib.rs
use opentelemetry::trace::{Tracer, TracerProvider};
use opentelemetry::global;

pub struct SigmaTracer {
    tracer: Box<dyn Tracer>,
}

impl SigmaTracer {
    pub fn new() -> Self {
        let provider = opentelemetry_jaeger::new_pipeline()
            .with_service_name("sigmaos")
            .install_simple();

        let tracer = provider.tracer("sigmaos");

        SigmaTracer { tracer }
    }

    pub fn trace_syscall(&self, syscall: &str, args: &[u64]) {
        let span = self.tracer.start(syscall);

        // Record syscall arguments
        for (i, arg) in args.iter().enumerate() {
            span.set_attribute(format!("arg_{}", i), arg.to_string());
        }

        span.end();
    }
}
```

## Priority 9: Signing & Provenance (Critical for Supply Chain)

### Target: Sigstore/Cosign

**License**: Apache-2.0
**Impact**: Provides artifact signing and provenance
**Feasibility**: Very High - Apache-2.0 license
**Timeline**: 1 sprint (3 weeks)

#### Implementation Plan

### Week 1: Integration

- [ ] Add cosign-rs as dependency

- [ ] Create release/signing module

- [ ] Implement signing with Dilithium-5

- [ ] Add signature verification

- [ ] Create provenance generation

### Week 2: SigmaOS Integration

- [ ] Integrate with build pipeline

- [ ] Add package signing

- [ ] Implement signature verification in sigpkg

- [ ] Add provenance publishing

- [ ] Create signature verification UI

### Week 3: Testing & Documentation

- [ ] Add signing tests

- [ ] Write security documentation

- [ ] Create signing examples

- [ ] Add key management

**Repository Mapping**: release/signing
**Branch**: main
**Effort**: 1 engineer × 3 weeks

#### Code Structure

```rust
// release/signing/src/lib.rs
use sigstore::cosign::SignatureScheme;
use sigstore::crypto::SigningScheme;

pub struct SigmaSigner {
    scheme: SigningScheme,
    key_pair: KeyPair,
}

impl SigmaSigner {
    pub fn new() -> Result<Self> {
        let key_pair = KeyPair::generate_dilithium5()?;
        let scheme = SigningScheme::Dilithium5;

        Ok(SigmaSigner { scheme, key_pair })
    }

    pub fn sign(&self, artifact: &[u8]) -> Result<Signature> {
        let signature = self.scheme.sign(artifact, &self.key_pair)?;

        Ok(signature)
    }

    pub fn verify(&self, artifact: &[u8], signature: &Signature) -> Result<bool> {
        let verified = self.scheme.verify(artifact, signature, &self.key_pair.public_key)?;

        Ok(verified)
    }

    pub fn generate_provenance(&self, artifact: &[u8]) -> Result<Provenance> {
        Ok(Provenance {
            builder: "SigmaOS",
            signature: self.sign(artifact)?,
            timestamp: SystemTime::now(),
            build_id: env!("GIT_COMMIT"),
        })
    }
}
```

## Priority 10: MicroVM Runtime (Critical for Cloud)

### Target: Firecracker

**License**: Apache-2.0
**Impact**: Provides microVM runtime for serverless
**Feasibility**: Very High - Apache-2.0 license
**Timeline**: 2 sprints (6 weeks)

#### Implementation Plan

### Sprint 1: Basic Integration

- [ ] Add Firecracker as dependency

- [ ] Create runtime/vmm module

- [ ] Implement basic VM creation

- [ ] Add VM lifecycle management

- [ ] Implement resource limits

- [ ] Add VM networking

### Sprint 2: Advanced Features

- [ ] Add snapshot/restore

- [ ] Implement VM metrics

- [ ] Add VM security features

- [ ] Integrate with sigpkg

- [ ] Create VM orchestration

- [ ] Add performance optimizations

**Repository Mapping**: runtime/vmm
**Branch**: release/cloud
**Effort**: 2 engineers × 6 weeks

#### Code Structure

```rust
// runtime/vmm/src/lib.rs
use firecracker::Firecracker;

pub struct SigmaMicroVM {
    firecracker: Firecracker,
    id: String,
}

impl SigmaMicroVM {
    pub fn new(id: String) -> Result<Self> {
        let firecracker = Firecracker::new(id.clone())?;

        Ok(SigmaMicroVM { firecracker, id })
    }

    pub async fn start(&mut self) -> Result<()> {
        self.firecracker.start().await?;
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<()> {
        self.firecracker.stop().await?;
        Ok(())
    }

    pub async fn snapshot(&self) -> Result<Snapshot> {
        let snapshot = self.firecracker.snapshot().await?;
        Ok(snapshot)
    }

    pub async fn restore(&mut self, snapshot: Snapshot) -> Result<()> {
        self.firecracker.restore(snapshot).await?;
        Ok(())
    }
}
```

## Implementation Timeline

### Week 1-3 (Sprint 1)

- WASM Runtime (Wasmtime)

- Network Stack (smoltcp)

- Crypto Primitives (libsodium)

### Week 4-6 (Sprint 2)

- Wayland Compositor (wlroots) - continues

- Async Runtime (Tokio)

- Embedded Database (SQLite)

### Week 7-9 (Sprint 3)

- Metrics Collection (Prometheus)

- Tracing (OpenTelemetry)

- Signing & Provenance (Sigstore/Cosign)

### Week 10-12 (Sprint 4)

- MicroVM Runtime (Firecracker) - continues

- Integration testing

- Documentation

## Resource Requirements

### Engineering Resources

- **Total Effort**: 30 engineer-weeks

- **Peak Concurrent**: 4 engineers

- **Timeline**: 12 weeks (3 months)

### Dependencies

- All selected projects have permissive licenses

- No GPL compliance issues

- Can be integrated directly with attribution

## Risk Mitigation

### Technical Risks

- **Integration complexity**: Start with minimal integration, expand gradually

- **Performance impact**: Add performance benchmarks for each integration

- **Security concerns**: Security review before integration

### License Risks

- **All selected projects have permissive licenses**

- **Attribution requirements documented**

- **No copyleft compliance needed**

## Success Criteria

### Technical Success

- All 10 projects integrated successfully

- Performance benchmarks meet targets

- Security review passed

- Documentation complete

### Roadmap Impact

- Phase 1 goals achieved (network stack, drivers)

- Phase 2 goals achieved (WASM runtime, sigpkg)

- Phase 3 goals achieved (observability, tracing)

---

**Last Updated**: 2026-07-05
**Maintained by**: SigmaOS Core Team
