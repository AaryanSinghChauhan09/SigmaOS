# SigmaOS Display Server Subsystem Specification (`Zenith Display Server`)

## 1. Overview & Architecture

The Zenith Display Server is the core graphical display engine for SigmaOS, providing a modern, asynchronous, zero-copy display protocol inspired by Wayland while optimized for high-throughput, low-latency microkernel IPC.

In traditional X11 architectures, the display server acted as a monolithic rendering broker with heavy network transparency overheads and global synchronous locking. Zenith replaces this model with an IPC-driven surface management protocol where clients render directly into shared memory buffers (shm or DRM/KMS DMA-BUFs) and submit damage regions to the Zenith display server daemon for hardware presentation.

```
+-------------------------------------------------------------+
|                      GUI Application                        |
|   (Zenith UI Toolkit / GTK / Qt Compatibility Bridge)       |
+------------------------------+------------------------------+
                               |
                   Zenith IPC Protocol (Unix Socket / Shared Memory)
                               |
+------------------------------v------------------------------+
|                   Zenith Display Server                     |
|  +--------------------+  +-------------------+  +---------+ |
|  | Surface Compositor |  | Input Dispatcher  |  | KMS/DRM | |
|  +--------------------+  +-------------------+  +---------+ |
+------------------------------+------------------------------+
                               |
                          DRM/KMS / libinput
                               |
+------------------------------v------------------------------+
|                       GPU Driver / Hardware                 |
+-------------------------------------------------------------+
```

## 2. Display Protocol Specification

The Zenith protocol is asynchronous, object-oriented, and based on binary RPC messages passed over local Unix domain sockets or zero-copy shared ring buffers (`sigma_io_uring`).

### 2.1 Core Protocol Interfaces

1. **`zenith_display`**: Core registry interface for discovering global interfaces (compositor, output manager, input seat, shell).
2. **`zenith_compositor`**: Interface for creating surface objects (`zenith_surface`) and region objects (`zenith_region`).
3. **`zenith_surface`**: Represents a rectangular pixel buffer presented on screen. Supports damage reporting, frame callbacks, and transform attributes.
4. **`zenith_buffer`**: Encapsulates backing memory (SHM pool or DMA-BUF file descriptor).
5. **`zenith_output`**: Describes physical monitor properties (resolution, refresh rate, HDR color space, scale factor, subpixel layout).
6. **`zenith_seat`**: Represents a group of input devices (keyboard, pointer/mouse, touch, tablet pen).

### 2.2 Wire Protocol Message Format

All Zenith IPC protocol messages are 32-bit aligned binary structures:

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                          Object ID                            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|         Opcode (16-bit)       |        Message Size (16-bit)  |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                        Arguments ...                          |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

- **Object ID**: 32-bit unsigned identifier representing the target protocol object.
- **Opcode**: 16-bit event or request code.
- **Message Size**: Total payload length in bytes, including header.

## 3. Graphics Buffer Management & DMA-BUF Zero-Copy

To achieve 120Hz+ rendering without CPU memory copies:
1. **Shared Memory (SHM) Pools**: Applications map a shared memory page created via `memfd_create` or `shm_open`. Used primarily for software-rendered applications.
2. **DRM/KMS DMA-BUF**: GPU-accelerated applications allocate buffers directly in VRAM via DRM driver ioctls (`GBM` / `EGL`). The file descriptor is passed to Zenith via socket control messages (`SCM_RIGHTS`).
3. **Multi-Buffer Swapchains**: Standard double or triple buffering (`front_buffer`, `back_buffer`, `present_buffer`) synchronized via sync fences (`sync_file`).

## 4. Input Handling & Event Routing

- **Libinput Integration**: The Zenith display server reads hardware input events directly from `/dev/input/event*` devices or kernel evdev drivers.
- **Focus Rules**: Pointer events (motion, click, scroll) are routed based on surface geometry and region clipping masks. Keyboard focus is assigned via shell focus events.
- **High-Precision Input**: Supports sub-pixel pointer motion, tablet pressure/tilt, and multi-touch gesture events (pinch, swipe, rotate).

## 5. Security & Isolation

- **Client Sandboxing**: Zenith clients cannot access surfaces or screen contents belonging to other applications unless explicit permission interfaces (`zenith_screencast`, `zenith_clipboard`) are authorized.
- **Input Keylogging Protection**: Keyboard grabs and global shortcuts are strictly mediated by the compositor shell.
