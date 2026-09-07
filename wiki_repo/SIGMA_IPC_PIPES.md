# SigmaOS IPC Pipes

## Overview

SigmaOS implements its own sovereign IPC pipe subsystem (`src/ipc/sigma_pipe.rs`), providing Unix-style anonymous and named pipes with ring-buffer semantics, no standard-library dependencies beyond `alloc`.

---

## Design

### Ring Buffer

The core data structure is `PipeBuffer`, a fixed-capacity circular (ring) buffer:

```
write_pos ──►  [ b ][ b ][ b ][   ][   ]
               ▲                         ▲
           read_pos                   capacity
```

- `capacity`: fixed at creation time (e.g. 4096 bytes)
- `read_pos`: next byte to consume
- `write_pos`: next byte to produce
- `len`: current fill level

Both positions wrap modulo `capacity`, giving O(1) reads and writes.

### Pipe Ends

A pipe is split into two RAII halves:

| Type | Role |
|------|------|
| `PipeWriter` | Produces data; holds a reference-counted `PipeInner` |
| `PipeReader` | Consumes data; holds a reference-counted `PipeInner` |

Both ends share the same `PipeInner` via `Rc<RefCell<PipeInner>>`.

Dropping a `PipeWriter` automatically calls `close_write()`, signalling EOF to the reader. Dropping a `PipeReader` calls `close_read()`, causing subsequent writes to fail with `PipeError::ReadEndClosed`.

---

## API Reference

### `SigmaPipe::new(capacity) → (PipeWriter, PipeReader)`

Creates an anonymous pipe with the given buffer capacity.

```rust
let (mut writer, mut reader) = SigmaPipe::new(4096);
writer.write(b"hello")?;
let mut buf = [0u8; 64];
let n = reader.read(&mut buf)?;
```

### `PipeWriter::write(data: &[u8]) → Result<usize, PipeError>`

Writes `data` into the ring buffer. Returns bytes written (may be less than `data.len()` if the buffer is nearly full).

| Error | Meaning |
|-------|---------|
| `WouldBlock` | Buffer completely full |
| `ReadEndClosed` | Reader already dropped |

### `PipeReader::read(buf: &mut [u8]) → Result<usize, PipeError>`

Reads up to `buf.len()` bytes. Returns `Ok(0)` on EOF (write end closed and buffer drained).

| Error | Meaning |
|-------|---------|
| `Empty` | Buffer empty, write end still open |
| `ZeroLengthBuffer` | `buf` has zero length |

### `PipeWriter::close_write()` / `PipeReader::close_read()`

Half-close operations. Also called automatically on drop.

---

## Named Pipes — `SigmaFifo`

`SigmaFifo` wraps an anonymous `SigmaPipe` and associates it with a filesystem path:

```rust
let mut fifo = SigmaFifo::create("/tmp/sigma.fifo".to_string(), 8192);
fifo.writer().write(b"data")?;
```

In the full kernel integration, `SigmaFifo::create` registers the path in the VFS via `SigmaVfs::mkfifo`, and `SigmaFifo::unlink` removes it.

---

## Comparison with Unix Pipes and Plan 9 Channels

| Feature | Unix `pipe(2)` | Plan 9 Channels | SigmaOS `SigmaPipe` |
|---------|---------------|-----------------|---------------------|
| Anonymous | ✅ | ✅ | ✅ |
| Named (FIFO) | ✅ (`mkfifo`) | ✅ (filesystem) | ✅ (`SigmaFifo`) |
| Typed messages | ❌ (byte stream) | ✅ (typed) | ❌ (byte stream) |
| Priority | ❌ | ❌ | Use `SigmaMessageQueue` |
| Kernel integration | Full | Full | VFS via `SigmaFifo` |
| Blocking model | Full blocking | Full blocking | `WouldBlock` / `Empty` error |
| no_std | ❌ | ❌ | ✅ (`alloc` only) |
| Backpressure | Kernel sleep | Rendezvous | Caller must retry |

### Key differences from Unix `pipe(2)`

- SigmaOS uses `WouldBlock`/`Empty` errors rather than sleeping the calling process — the scheduler integration handles the sleep at a higher level.
- The ring buffer is allocated from the kernel heap (`alloc::vec`), not a fixed kernel pool.
- Both half-close directions are explicit and symmetric.

### Key differences from Plan 9 channels

- SigmaOS pipes are untyped byte streams like Unix; typed message passing is delegated to `SigmaMessageQueue`.
- Plan 9 channels use rendezvous (synchronous handoff); SigmaOS pipes are buffered.

---

## Kernel Integration Points

| Subsystem | Integration |
|-----------|-------------|
| VFS | `SigmaFifo` path registered via `VfsContext::mount` |
| Scheduler | Blocked processes re-queued on `WouldBlock` |
| Process descriptor table | `PipeReader`/`PipeWriter` tracked as file descriptors |
| `sigma_sh` | Shell pipeline (`cmd1 \| cmd2`) via pipe pairs |

---

## See Also

- [`SIGMA_MESSAGE_QUEUE.md`](SIGMA_MESSAGE_QUEUE.md) — priority-based IPC
- [`SIGMA_VFS_LAYER.md`](SIGMA_VFS_LAYER.md) — VFS integration for named pipes
- [`SIGMA_CONCURRENCY_PRIMITIVES.md`](SIGMA_CONCURRENCY_PRIMITIVES.md) — synchronisation
