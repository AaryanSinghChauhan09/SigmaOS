## 2025-10-24 - [Legacy Windowing Protocol Bottlenecks]
**Learning:** Traditional window display architectures (e.g., X11, Wayland) introduce latency overhead due to heavy event marshaling and context switches.
**Action:** Direct frame buffers coupled with a dedicated lightweight Vulkan compositing thread achieve latency parity with dedicated gaming consoles.
