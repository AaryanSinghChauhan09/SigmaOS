# API Reference

The Sovereign Lattice provides a set of hardware-native APIs for shard interaction.

## Kernel API (`sigma_api.h`)


- `void* sigma_alloc(uint64_t size)`: O(1) Slab allocation.
- `int sigma_spawn(const char* shard_name)`: Spawn an isolated shard.
- `void sigma_yield()`: Cooperative multitasking yield.

## Networking API (`socket.h`)


- `int socket_create(int protocol)`: Create a sandboxed socket.
- `int socket_send(int fd, void* buf, int len)`: Secure DMA send.

## UI API (Morphic)


- `void morphic_draw_rect(int x, int y, int w, int h, uint32_t color)`: GPU-accelerated drawing.
- `void morphic_set_opacity(int shard_id, float alpha)`: Glassmorphism control.


