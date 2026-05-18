# Gaming & GPU Optimization in SigmaOS

SigmaOS achieves **SteamOS parity** through silicon-direct GPU scheduling and gaming-first resource allocation.

## 🎮 The Sovereign Game Mode

SigmaOS provides a native `sigma-game-mode` shard that dynamically reconfigures the lattice for maximum FPS and sub-1ms latency.

### Key Algorithms

- **Dynamic GPU Scheduler (`SovereignGPUSched`)**: Prioritizes the rendering shard's access to silicon compute units.

- **Haptic Feedback Sync**: Sub-ms synchronization between game audio and controller haptics.

- **Vulkan-Direct Sharding**: Bypasses traditional driver overhead for zero-copy vertex throughput.

## 🕹️ Peripheral Support

- **Controller Manager**: Native support for Xbox, PlayStation, and Nintendo controllers via the `SovereignInput` shard.

- **VR/AR Shard**: Specialized profile for low-persistence rendering and spatial tracking.

## 🚀 How to Enable

Run the following in the Sovereign CLI:

```bash
sigma-cli install sigma-game-mode
sigma-cli profile set gaming


```
