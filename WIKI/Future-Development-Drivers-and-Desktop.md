# Future Development: Device Drivers and Desktop

**Status:** Proposal. Driver names and display APIs in the repository do not by themselves guarantee hardware support.

## Scope

Develop reliable device discovery, safe DMA, display modesetting, input, and accessible desktop workflows. Current source entry points include [`drm_kms.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/drivers/drm_kms.rs), [`unified_dma.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/drivers/unified_dma.rs), [`modern_wifi.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/drivers/modern_wifi.rs), [`zenith_compositor.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/desktop/zenith_compositor.rs), and [`tiling.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/desktop/tiling.rs).

## Linux and BSD ideas to evaluate

| Project | Design idea | SigmaOS question |
|---|---|---|
| Linux DRM/KMS | Atomic display-state changes and device discovery | Can a failed modeset leave the previous display state intact? |
| FreeBSD | Device lifecycle events and stable driver interfaces | How are hotplug, suspend, reset, and device removal represented? |
| NetBSD | Portable driver interfaces and rump testing | Which drivers can be tested outside a full machine boot? |
| Pop!_OS COSMIC | Keyboard-centered tiling and workspace behavior | Which window-management choices improve usability without reducing control? |
| Arch Linux | Clear hardware-specific package and configuration guidance | Can supported hardware and required firmware be documented precisely? |

## Proposed work sequence

1. **Publish a support matrix.** List device IDs, architectures, tested functions, required firmware, and known limitations.
2. **Harden driver lifecycle.** Define probe, initialization, reset, suspend, resume, and removal states with explicit failure cleanup.
3. **Constrain DMA.** Validate address width, mapping lifetime, alignment, and device ownership before enabling bus mastering.
4. **Build display fallbacks.** Keep a simple framebuffer path available while atomic modesetting and accelerated paths are developed.
5. **Tie desktop actions to real capabilities.** Window-management and settings controls should reflect available displays, input devices, and permissions.
6. **Keep accessibility in the design.** Keyboard navigation, focus handling, text scaling, and screen-reader interfaces need acceptance criteria for each workflow.

## Completion criteria

- Each supported device has documented identification and tested operations; untested devices are marked unsupported.
- Probe or reset failure leaves resources released and the device in a safe state.
- DMA mappings cannot outlive their buffers or be reused by another device without explicit transfer.
- Display mode changes either commit completely or restore the prior mode.
- Core desktop operations remain usable by keyboard and have predictable focus behavior.

## Maintenance

Update support claims only from hardware or emulator results tied to a recorded configuration. Keep planned devices separate from supported devices.
