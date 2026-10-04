# Mint & Omarchy Hardware & Audio Supremacy

The **Mint & Omarchy Hardware & Audio Supremacy Suite** ([`src/distro/mint_omarchy_hardware_and_audio_supremacy.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/mint_omarchy_hardware_and_audio_supremacy.rs)) provides deep hardware fingerprinting, automated out-of-tree module signing, real-time studio audio filtering, mobile handheld/laptop power throttling, and sub-millisecond fuzzy search.

---

## 1. Overview & Comparative Advantages

| Subsystem | Linux Mint | Omarchy (Arch) | **SigmaOS Hardware & Audio Supremacy** |
| :--- | :--- | :--- | :--- |
| **Driver Management** | Python `mintdrivers` GUI | Manual pacman / AUR dkms | **Automated PCI/USB fingerprinting & MOK enrollment** |
| **Studio Audio DSP** | PulseAudio / basic PipeWire | EasyEffects GUI (C++) | **Real-time 1.33ms neural noise suppression & 10-band EQ** |
| **Handheld/Mobile Power** | Generic TLP | `auto-cpufreq` daemon | **Form-factor aware (Steam Deck, ROG Ally, Framework) dynamic TDP** |
| **Application Search** | Cinnamon menu (GMenu) | Walker / Fuzzel / Wofi | **Sub-millisecond fuzzy search + instant calculator** |

---

## 2. Component Specifications

### A. Sovereign Mint Driver Manager (`SovereignMintDriverManager`)
*Source: [`src/distro/mint_omarchy_hardware_and_audio_supremacy.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/mint_omarchy_hardware_and_audio_supremacy.rs#L34-L125)*

* **Automated Device Discovery**: Scans vendor and product IDs across NVIDIA (e.g. `10de:2484`), Intel (`8086:2723`), and AMD (`1002:73bf`).
* **Categorized Driver Tiers**:
  * `InTreeKernelNative`: Directly compiled into the kernel.
  * `OpenSourceOptimized`: E.g., `amdgpu-radv-vulkan` or `nvidia-open-kernel-dkms`.
  * `VendorProprietary`: Closed-source vendor binaries when mandatory.
  * `FirmwareBlobOnly`: Microcode / Wi-Fi firmware packages.
* **Zero-Touch MOK Signer**: Automates Machine Owner Key generation and enrollment for UEFI Secure Boot compliance.
* **Offline Driver Bundle Mounter**: Directly mounts driver ISOs during offline installations.

### B. Sovereign Omarchy Studio Audio Pipeline (`SovereignOmarchyStudioAudioPipeline`)
*Source: [`src/distro/mint_omarchy_hardware_and_audio_supremacy.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/mint_omarchy_hardware_and_audio_supremacy.rs#L127-L200)*

* **Ultra-Low Buffer Latency**:
  * $64\text{ frames} @ 48\text{kHz} = 1.33\text{ms}$ buffer latency ($1333\mu\text{s}$).
  * Prevents buffer overruns (XRUNs) with lock-free atomic tracking.
* **Integrated DSP Chain**:
  * 10-Band Parametric Equalizer.
  * DeepFilterNet-style neural noise suppression.
  * Dynamic range compressor and multiband limiter.
  * Noise gate threshold ($ -42.0\text{ dB}$) and output limiter ($ -0.1\text{ dB}$).

### C. Sovereign Omarchy Handheld Power Optimizer (`SovereignOmarchyHandheldPowerOptimizer`)
*Source: [`src/distro/mint_omarchy_hardware_and_audio_supremacy.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/mint_omarchy_hardware_and_audio_supremacy.rs#L202-L290)*

* **Form-Factor Detection**:
  * `HandheldGamingConsole` (Steam Deck, ASUS ROG Ally, Lenovo Legion Go)
  * `ModularLaptop` (Framework 13 / 16)
  * `StandardUltrabook` (Lenovo ThinkPad, Dell XPS)
  * `DesktopWorkstation`
* **Dynamic TDP Control**: Dynamically scales between 3W and 150W based on power state and workload.
* **Battery Longevity Protection**: Implements configurable charge cutoffs (default 80%) to double lithium battery longevity.
* **AC / Battery Adaptive EPP**: Auto-transitions between `EnergyPreference::Performance` on AC and `EnergyPreference::BalancePower` on battery.

### D. Sovereign Omarchy Fuzzy Launcher (`SovereignOmarchyFuzzyLauncherEngine`)
*Source: [`src/distro/mint_omarchy_hardware_and_audio_supremacy.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/mint_omarchy_hardware_and_audio_supremacy.rs#L292-L375)*

* **Sub-Millisecond Search**: In-memory matching prioritizing exact matches (500 pts), prefixes (300 pts), substrings (200 pts), and category tags (100 pts).
* **Instant Calculator**: Evaluates arithmetic expressions inline and copies results directly to Wayland clipboard.
* **Zero-Flicker Presentation**: Directly interfaces with `SigmaCompositor` layer shell.

---

## 3. Source Code Reference

* Implementation: [`src/distro/mint_omarchy_hardware_and_audio_supremacy.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/mint_omarchy_hardware_and_audio_supremacy.rs)
* Registration: [`src/distro/mod.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/mod.rs#L396)

---

## 4. AI Agent Maintenance Instructions

> **For AI Agents Maintaining This Page:**
> - Source: `src/distro/mint_omarchy_hardware_and_audio_supremacy.rs`
> - When new hardware vendor/product IDs are mapped, update the device scan table.
> - Ensure audio quantum sizes are benchmarked against 48kHz and 96kHz stream configurations.
> - Maintain links with [Audio-and-Graphics](Audio-and-Graphics.md), [Hardware-Drivers](Hardware-Drivers.md), and [Power-Management](Power-Management.md).
