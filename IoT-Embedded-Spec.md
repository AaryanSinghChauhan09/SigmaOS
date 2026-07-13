# SigmaOS IoT & Embedded — Architecture Spec

> **Status**: Planning | **Target**: v0.4-embedded | **Codename**: Sigma Atom

---

## Vision

**Sigma Atom** is the embedded/IoT variant of SigmaOS. It targets resource-constrained devices with a footprint as small as 512 KB flash and 64 KB RAM. The core kernel is shared with SigmaOS Desktop, with an ultra-minimal profile that strips all non-essential subsystems.

---

## Target Hardware

| Category | Examples | RAM | Flash | I/O |
|----------|---------|-----|-------|-----|
| Microcontrollers | RP2040, STM32H7 | 64 KB–1 MB | 256 KB–4 MB | GPIO, SPI, I2C, UART |
| SBCs (Small) | Raspberry Pi Zero 2W | 512 MB | microSD | USB, WiFi, GPIO |
| Industrial | BeagleBone Black, SAME70 | 512 MB–2 GB | eMMC | CAN, EtherCAT, RS-485 |
| Automotive | NXP S32G, Renesas R-Car | 2–8 GB | eMMC | AUTOSAR, CAN FD |
| Edge Compute | Jetson Nano, Coral Dev Board | 4–16 GB | eMMC/NVMe | GPU, NPU, PCIe |

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│              Sigma Atom Application Layer               │
│  MQTT Client │ OPC-UA │ REST Agent │ Modbus │ Profinet  │
├─────────────────────────────────────────────────────────┤
│              Sigma Atom Middleware                      │
│  OTA Update │ Device Shadow │ Diagnostics │ Time Sync   │
├─────────────────────────────────────────────────────────┤
│              SigmaOS Kernel (Embedded Profile)          │
│  RT Scheduler │ Interrupt Handler │ Minimal VFS │ IPC   │
├─────────────────────────────────────────────────────────┤
│              Embedded HAL                               │
│  GPIO │ SPI │ I2C │ UART │ CAN │ ADC │ PWM │ Ethernet  │
├─────────────────────────────────────────────────────────┤
│              Hardware                                   │
└─────────────────────────────────────────────────────────┘
```

---

## Build Profiles

### `sigma-atom-nano` (MCU target)
- RAM: ≥ 64 KB required
- No MMU (uses MPU instead)
- No dynamic allocation (static pools only)
- No VFS (register-based I/O)
- RTOS scheduler with 64 priority levels
- Footprint: ~150 KB flash

### `sigma-atom-micro` (SBC target)
- RAM: ≥ 128 MB
- MMU enabled
- Minimal VFS (read-only squashfs root)
- Basic networking (TCP/IP, MQTT)
- Footprint: ~32 MB
- OTA update support

### `sigma-atom-industrial` (Industrial target)
- RAM: ≥ 512 MB
- Full kernel with RT patches
- Deterministic latency: < 50 µs worst-case interrupt response
- EtherCAT, PROFINET, CAN FD support
- Functional Safety hooks (IEC 61508 SIL-2 ready)

---

## Real-Time Scheduler (Embedded)

For embedded profiles, SigmaOS uses a **Fixed-Priority Preemptive Scheduler**:

```rust
pub struct EmbeddedTask {
    pub id: u8,
    pub priority: u8,         // 0 = highest
    pub period_us: u32,       // Task period (0 = aperiodic)
    pub deadline_us: u32,     // Relative deadline
    pub wcet_us: u32,         // Worst-case execution time
    pub stack: &'static mut [u8],
    pub entry: fn(),
}

pub struct RtScheduler {
    tasks: heapless::Vec<EmbeddedTask, 64>,
    current: Option<u8>,
    tick_us: u32,
}
```

Deadline analysis:
- Utilization bound check: `Σ(WCET/Period) ≤ n(2^(1/n) - 1)`
- Response time analysis for each task
- Assert all deadlines met at compile time (optional `#[sigma_rt_verify]`)

---

## Peripheral HAL

```rust
pub trait GpioPin {
    fn set_high(&mut self);
    fn set_low(&mut self);
    fn is_high(&self) -> bool;
    fn set_mode(&mut self, mode: PinMode);
    fn enable_interrupt(&mut self, trigger: IrqTrigger);
}

pub trait SpiDevice {
    fn transfer(&mut self, tx: &[u8], rx: &mut [u8]) -> Result<(), SpiError>;
    fn set_frequency_hz(&mut self, hz: u32);
    fn set_mode(&mut self, mode: SpiMode);
}

pub trait I2cBus {
    fn write(&mut self, addr: u8, data: &[u8]) -> Result<(), I2cError>;
    fn read(&mut self, addr: u8, buf: &mut [u8]) -> Result<(), I2cError>;
    fn write_read(&mut self, addr: u8, write: &[u8], read: &mut [u8]) -> Result<(), I2cError>;
}

pub trait CanBus {
    fn send(&mut self, frame: &CanFrame) -> Result<(), CanError>;
    fn recv(&mut self) -> Option<CanFrame>;
    fn set_bitrate(&mut self, bitrate: u32, data_bitrate: Option<u32>);
}
```

---

## OTA Update (sigma-atom-ota)

- **A/B partition scheme**: Active + standby system partitions
- **Delta updates**: Binary patches using bsdiff/zstd
- **Signature verification**: Dilithium5 signed update manifests
- **Rollback protection**: Hardware fuse or anti-rollback counter

```toml
# /etc/sigma-atom/ota.toml
[ota]
update_server = "https://updates.sigmaos.org/atom/"
check_interval_s = 3600
signature_key = "/etc/sigma/pqc/ota-public.dilithium5"
partition_a = "/dev/mmcblk0p2"
partition_b = "/dev/mmcblk0p3"
rollback_count_max = 3
```

---

## Industrial Protocols

| Protocol | Standard | Status |
|---------|---------|--------|
| CAN 2.0B | ISO 11898 | Planned |
| CAN FD | ISO 11898-1:2015 | Planned |
| Modbus RTU/TCP | IEC 61158 | Planned |
| PROFINET RT | IEC 61158 Type 10 | Planned |
| EtherCAT | IEC 61158 Type 12 | Planned |
| OPC-UA | IEC 62541 | Planned |
| MQTT 5.0 | ISO/IEC 20922 | Planned |
| BACnet | ANSI/ASHRAE 135 | Research |

---

## Security for IoT

- **Secure Boot**: Hardware root of trust (TPM/eFuse)
- **Encrypted Storage**: AES-256-GCM for all persistent data
- **Network Security**: mTLS 1.3 for all cloud connections
- **Minimal Attack Surface**: Remove all non-required subsystems at compile time
- **Capability-based I/O**: Each task has explicit I/O permissions
- **Tamper Detection**: Enclosure tamper detection GPIO monitoring

---

## Power Management (IoT)

```rust
pub enum SleepMode {
    Active,           // Full operation
    Idle,             // CPU halted, peripherals active
    Stop,             // CPU + RAM retained, peripherals optional
    Standby,          // Only RTC/wake pin active, RAM lost
    Shutdown,         // Complete power off (RTC only)
}

pub trait PowerController {
    fn enter_sleep(&mut self, mode: SleepMode, wakeup: WakeupSource) -> !;
    fn get_power_consumption_ua(&self) -> u32;
    fn get_battery_voltage_mv(&self) -> Option<u32>;
}
```

Target power budgets:

| Mode | Current Draw | Wake Latency |
|------|-------------|-------------|
| Active | 50–200 mA | — |
| Idle | 5–20 mA | < 10 µs |
| Stop | 100–500 µA | < 100 µs |
| Standby | 1–10 µA | < 1 ms |
| Shutdown | < 1 µA | Boot time |

---

## Roadmap

| Milestone | Target | Description |
|-----------|--------|-------------|
| M1 | 2027 Q2 | sigma-atom-micro boots on RPi 5 |
| M2 | 2027 Q3 | GPIO/SPI/I2C HAL complete |
| M3 | 2027 Q4 | MQTT + OTA update support |
| M4 | 2028 Q1 | sigma-atom-industrial on BeagleBone |
| M5 | 2028 Q2 | CAN + Modbus protocols |
| M6 | 2028 Q3 | RTOS profile for MCU targets |
| M7 | 2028 Q4 | IEC 61508 SIL-2 safety analysis |
