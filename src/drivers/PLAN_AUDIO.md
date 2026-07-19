# 🎵 SigmaOS OOP Audio Subsystem Development Plan

This document outlines the blueprint for developing the **SigmaOS Audio Subsystem**. Taking architectural inspiration from **ALSA (Advanced Linux Sound Architecture)** and **OSS (Open Sound System)** found in modular Linux distributions such as **Slackware** and **Debian**, this subsystem guarantees ultra-low-latency sound processing on modern computers while seamlessly integrating retro-hardware using OOP principles.

---

## 🏗️ 1. Subsystem Architecture

The audio subsystem is divided into three abstraction tiers to separate raw physical I/O from high-level sound synthesis and PCM playback:

```
        +-------------------------------------------------+
        |           SoundServer & Mixer Subsystem         | (Unified API)
        +-------------------------------------------------+
                                |
            +-------------------+-------------------+
            | (Dynamic Polymorphic dispatch)        |
            v                                       v
+-----------------------+               +-----------------------+
|  Analog/Synthesizer   |               |   PCM Stream Device   | (OOP Traits)
+-----------------------+               +-----------------------+
| - AdLib FM Synth      |               | - SoundBlaster 16     |
| - Custom MIDI Synth   |               | - Intel HD Audio      |
+-----------------------+               +-----------------------+
```

### 1.1 The Core Trait (`AudioPeripheral`)
Every audio controller implements this `#![no_std]` interface:

```rust
pub trait AudioPeripheral: PeripheralDevice {
    /// Configures the digital-to-analog converter (DAC) sample rate and channel layout
    fn configure_format(&mut self, sample_rate: u32, channels: u16) -> Result<(), &'static str>;

    /// Starts audio playback / synthesis
    fn start_stream(&mut self) -> Result<(), &'static str>;

    /// Stops audio playback / synthesis
    fn stop_stream(&mut self) -> Result<(), &'static str>;
}
```

### 1.2 PCM Streaming Trait (`PcmDevice`)
Extends `AudioPeripheral` to handle DMA sound transfers:

```rust
pub trait PcmDevice: AudioPeripheral {
    /// Submits a block of raw PCM samples directly into the device ring-buffer
    fn queue_pcm_samples(&mut self, buffer: &[i16]) -> Result<usize, &'static str>;

    /// Returns current play-head position in bytes
    fn playback_position(&self) -> usize;
}
```

---

## 📻 2. Supported Generations & Compatibility

To preserve our commitment to dual-generation hardware compatibility, we implement two primary drivers:

### 2.1 Legacy: AdLibSynthDriver (Ancient Generation)
- **Interface**: Isa Port I/O (ports `0x388` and `0x389`).
- **Synthesis Model**: 9-channel dual-operator FM synthesis chip (YM3812 / OPL2).
- **Execution Model**: The driver receives high-level synthesizer commands, converts them using a register-mapping User-Defined Function (UDF), and writes directly to raw registers.

### 2.2 Modern: IntelHdAudioDriver (Modern Generation)
- **Interface**: Memory-Mapped I/O (MMIO), PCIe bus communication, and Direct Memory Access (DMA) page tables.
- **Synthesis Model**: High-definition digital audio streams supporting up to 192kHz/32-bit surround sound.
- **Execution Model**: Dual DMA ring buffers (cyclic page-tables) that stream memory with zero CPU overhead.

---

## ⚡ 3. UDF MIDI Interpreter Integration

For synthesized instruments and retro gaming tracks, the system implements a **UDF-based MIDI Instrument Mapper**:
- Instead of compiling physical sound banks into the kernel, the user supplies a **UDF bytecode mapping** representing register modifications for custom synthesizers.
- The UDF engine safely intercepts high-level sound events (e.g. `Note On`, `Pitch Bend`) and translates them to immediate register states in under 1 microsecond.

---

## 📈 4. Milestones & Implementation Order

1. **Phase 1: Interface Definition**
   - Create `AudioPeripheral` and `PcmDevice` interfaces inside `src/drivers/audio.rs`.
2. **Phase 2: AdLib ISA Driver**
   - Implement low-level port writes to port `0x388` to verify simple tone synthesis under x86 environments.
3. **Phase 3: SoundBlaster 16 ISA DMA Driver**
   - Implement DSP control commands and custom DMA buffer setups on ISA channel 5.
4. **Phase 4: Intel HD Audio PCIe Controller**
   - Setup PCI config space parsing, register DMA buffer ring, and output stereo sinus signals.
5. **Phase 5: Subsystem Mixer**
   - Implement software-mixing to combine multiple sound streams into a single outgoing hardware DAC buffer without using external memory allocation libraries.
