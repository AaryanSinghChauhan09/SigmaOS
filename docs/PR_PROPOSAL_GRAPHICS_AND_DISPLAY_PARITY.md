# Pull Request Proposals: Linux & BSD Graphics and Display Parity Suite

## Overview
This specification details the comprehensive, clean-room Pull Request proposals for graphics and display subsystem parity in SigmaOS, drawing architecture, driver model, and window compositor inspiration from flagship Linux distributions (Ubuntu, Fedora, Arch Linux, Gentoo, openSUSE, Alpine, SteamOS) and BSD systems (FreeBSD, OpenBSD, NetBSD, DragonFly BSD).

The proposals cover the complete graphics and display stack: core protocols, hardware acceleration drivers, window compositors, input handling, virtual terminals, and automated CI/PR gating mechanisms.

---

## Graphics & Display Subsystem PR Proposals Summary

| PR ID | Category | Subsystem / Component | Distro Inspirations | Description & Key Capabilities |
| :--- | :--- | :--- | :--- | :--- |
| **PR #101** | X11 Protocol | X11 (Xorg) Server Protocol Engine | Arch Linux, Debian, FreeBSD | Core X11 wire protocol request/reply dispatcher, atom table registry, window tree hierarchy, properties, and Xauth security. |
| **PR #102** | Wayland Core | Wayland Core Protocol & `xdg_shell` | Fedora, Ubuntu, Arch Linux | `wl_surface` state machine, `wl_shm` shared memory buffers, damage tracking, and `xdg_shell` top-level/popup surface management. |
| **PR #103** | Mesa 3D | Mesa EGL, DRI3 & Gallium3D Pipeline | Mesa3D, Fedora, CachyOS | DRI3 buffer handle exchange, EGL context initialization, AFBC/modifier support, and Gallium3D state-tracker interface. |
| **PR #104** | DRM/KMS | DRM/KMS Kernel Mode Setting & Atomic State | Linux Kernel DRM, FreeBSD DRM | Dynamic CRTC/connector/plane state machine, GEM buffer objects, PRIME dmabuf handles, and atomic commit validator. |
| **PR #105** | Userland DRM | libdrm Userland Ioctl Wrapper | libdrm, Debian, Alpine | Capability querying (`DRM_CAP_DUMB_BUFFER`, `DRM_CAP_PRIME`), dumb buffer allocations, and ioctl dispatching. |
| **PR #106** | Vulkan API | Cross-Platform Vulkan 3D & Compute API | Vulkan SDK, SteamOS, CachyOS | `VkInstance`, physical device enumeration, command buffer submission, graphics/compute queue routing, and swapchain presentation. |
| **PR #107** | OpenGL API | OpenGL / GLSL State Machine & FBO Rasterizer | Mesa OpenGL, Gentoo, Fedora | VAO/VBO/FBO state bindings, viewport transformations, depth testing, and GLSL shader program compilation. |
| **PR #108** | Weston Shell | Weston Reference Compositor Output Shell | Wayland/Weston, openSUSE | Desktop-shell protocol, multi-output layout rendering, Pixman vs. GL backends, and view damage repainting. |
| **PR #109** | Sway Tiling | Sway Tiling Window Manager Container Tree | Sway, Arch Linux, Alpine | Dynamic container tree, horizontal/vertical splits, tabbed/stacked containers, workspace management, and focus navigation. |
| **PR #110** | wlroots Backend | wlroots Modular Compositor & `wlr_scene` | SwayWM/wlroots, Void Linux | `wlr_backend` abstractions (DRM, Wayland, Headless), `wlr_scene` graph tree, output layout management, and input routing. |
| **PR #111** | Mutter Scene | Mutter / Clutter Scene Graph & Fractional Scale | GNOME, Fedora, Ubuntu | Clutter actor tree, Cogl rendering pipeline, `wp_fractional_scale_v1` buffer resolution scaling, and monitor layout alignment. |
| **PR #112** | KWin Effects | KWin Plasma Desktop Compositor & Effects | KDE Plasma, openSUSE, Fedora | KWin window decorations, blur and contrast GLSL shaders, drop shadow geometry calculators, and Plasma Wayland protocol extensions. |
| **PR #113** | Compiz 3D | Compiz 3D Desktop Cube & Physics Engine | Ubuntu MATE, Gentoo, Arch | 3D desktop cube projection matrix, wobbly window mass-spring physics simulation, and plugin hook architecture. |
| **PR #114** | Picom Compositor | Picom X11 Vsync, Dual-Kawase Blur & Shadows | Arch Linux, Void Linux | GLX/XRender vsync backends, multi-pass dual-kawase blur shaders, window opacity transitions, and corner rounding. |
| **PR #115** | Xwayland Bridge | Xwayland Translation, Glamor 2D & Clipboard | Xwayland, Fedora, Arch | Rootless X11 window mapping, Glamor 2D-over-3D Vulkan/OpenGL acceleration, and bi-directional X11/Wayland selection bridge. |
| **PR #116** | evdev Input | evdev Linux Input Binary Event Parser | Linux Kernel, Arch, Void | `/dev/input/event*` binary packet parsing (`EV_KEY`, `EV_REL`, `EV_ABS`), multitouch slot tracking (`ABS_MT_SLOT`), and EV_SYN syncing. |
| **PR #117** | libinput | libinput Gesture Recognition & Accel Curves | libinput, Fedora, Arch | Pointer acceleration curves (adaptive/flat), palm rejection boundary detection, and touch pad gesture recognition (pinch/swipe). |
| **PR #118** | Console FB | Console Framebuffer / fbdev Double-Buffered FB | Linux fbdev, FreeBSD wscons | Double-buffered ARGB pixel back-buffer, PSF font glyph rendering, RGB primitives, and hardware VSYNC buffer swaps. |
| **PR #119** | Virtual Terminal | vt / syscons / wscons Virtual Terminal Matrix | FreeBSD vt(4), OpenBSD wscons | Virtual terminal grid matrix (`ttyv0`-`ttyv12`), VT switching ioctls (`VT_ACTIVATE`, `VT_WAITACTIVE`), keymap translations, and ANSI escape sequences. |
| **PR #120** | Wayland Ext | Presentation Time v1 & dmabuf Modifiers | Fedora, Ubuntu, Arch | `wp_presentation_time` vsync timestamp feedback & `zwp_linux_dmabuf_v1` multi-planar modifier negotiation. |
| **PR #121** | DRM Planes | DRM/KMS VRR & Multi-Plane Composition | Linux Kernel DRM, SteamOS | DRM CRTC VRR adaptive sync timing, multi-plane zpos blending, pixel blend modes, and alpha rotation. |
| **PR #122** | Mesa Passes | Mesa NIR IR Optimization & EGLSync Fence | Mesa3D, CachyOS, Gentoo | Mesa NIR IR optimization passes, TGSI-to-NIR shader translation, and EGLSync fences. |
| **PR #123** | Vulkan 1.3 | Vulkan Mesh Shading & Timeline Semaphores | SteamOS Gamescope, CachyOS | Vulkan 1.3 mesh shading pipeline execution (`vkCmdDrawMeshTasksEXT`), timeline semaphores, and descriptor set layouts. |
| **PR #124** | wlroots Layout | wlroots Multi-Monitor Output Layout & Seat | SwayWM, Void Linux, Alpine | `wlr_output_layout` multi-monitor alignment, `wlr_seat` pointer/keyboard focus, and damage ring accumulation. |
| **PR #125** | BSD vt(4) | BSD vt(4) Sysmouse & wscons VT Matrix | FreeBSD, OpenBSD | FreeBSD `vt_sysmouse` cursor overlay, OpenBSD `wscons` VT keymap translation, and ANSI VT100 escape sequence parsing. |
| **PR #126** | SteamOS Gamescope | SteamOS Gamescope HDR Color & FSR Upscaling | SteamOS, Gamescope | HDR PQ-ST2084 EOTF tonemapping, FSR upscaling pipeline, and dynamic refresh rate switching. |
| **PR #127** | Hyprland Bezier | Hyprland v2 Bezier Animations & Hyprcursor | Hyprland, Arch Linux | Cubic bezier curve animation evaluation, Hyprcursor SVG/PNG themes, blur passes, and window rules. |
| **PR #128** | Nvidia PRIME | Nvidia PRIME Offload & NVK Open Driver Bridge | Arch, Fedora, Mesa3D | `NV_PRIME_RENDER_OFFLOAD` GPU selection, NVK open Vulkan driver integration, and CUDA DMA-BUF sharing. |
| **PR #129** | Wayland Ext | Extended Protocols (Session Lock, Tearing, Color) | Wayland, Fedora, Ubuntu | `ext-session-lock-v1` security, `tearing-control-v1` async pageflips, and `wp_color_management_v1` ICC transformations. |
| **PR #130** | COSMIC Smithay | System76 COSMIC / Smithay Rust Compositor | System76, Pop!_OS | Smithay Rust window management primitives, Iced GUI shell widget overlays, and dock autohiding. |
| **PR #131** | BSD DRM Isolation | BSD DRM Kernel Driver Isolation & Rump | FreeBSD, OpenBSD, NetBSD | FreeBSD `kld` DRM kernel modules, OpenBSD `inteldrm`/`radeondrm` KMS ports, and NetBSD Rump driver isolation. |

---

## Detailed Component Specifications

### 1. X11 (Xorg) Server Protocol & Atom Table (`SovereignX11ServerEngine`)
- **Architecture**: Complete request/reply protocol parser supportingCreateWindow, MapWindow, UnmapWindow, SetProperty, GetProperty, InternAtom, ChangeProperty, and ConfigureWindow.
- **Atom Table**: Dynamic bidirectionally-indexed map (`BTreeMap<String, u32>`) pre-populated with standard ICCCM and EWMH atoms (`PRIMARY`, `CLIPBOARD`, `UTF8_STRING`, `_NET_WM_NAME`).
- **Inspirations**: X.Org Foundation xserver, Arch Linux xorg-server packaging.

### 2. Wayland Core Wire Protocol & xdg_shell (`SovereignWaylandCoreEngine`)
- **Architecture**: Stateful Wayland display server maintaining surface hierarchies, attached `wl_shm` or `wl_drm` buffer descriptors, rectangular damage tracking, and `xdg_toplevel` focus state.
- **Inspirations**: Wayland core protocol, Fedora/Ubuntu Wayland transition specs.

### 3. Mesa EGL, DRI3 & Gallium3D Pipeline (`SovereignMesaGallium3dEngine`)
- **Architecture**: Direct Rendering Infrastructure 3 (DRI3) fd buffer handle allocator with DRM format modifiers (`DRM_FORMAT_MOD_LINEAR`), EGL context state binding, and Gallium3D rasterizer state evaluation.
- **Inspirations**: Mesa3D Gallium architecture, CachyOS graphics stack optimizations.

### 4. DRM/KMS Atomic Modesetting & GEM (`SovereignDrmKmsEngine`)
- **Architecture**: Kernel Mode Setting atomic commit validator ensuring atomic state coherence across CRTC, Connector, Plane, and Framebuffer IDs. Manages GEM buffer allocation and PRIME dmabuf exporting.
- **Inspirations**: Linux DRM subsystem (`drivers/gpu/drm`), FreeBSD DRM/KMS porting layer.

### 5. libdrm Userland Bridge (`SovereignLibdrmBridgeEngine`)
- **Architecture**: Userland capability matrix solver wrapping `drmIoctl` queries (`DRM_CAP_DUMB_BUFFER`, `DRM_CAP_PRIME`, `DRM_CAP_ASYNC_PAGE_FLIP`).
- **Inspirations**: `libdrm` library specification across Alpine & Debian.

### 6. Vulkan Cross-Platform 3D & Compute API (`SovereignVulkanApiEngine`)
- **Architecture**: Vulkan 1.3 pipeline manager integrating `VulkanInstance`, physical device queue family selection, swapchain image acquisition (`VK_PRESENT_MODE_MAILBOX_KHR`), and command buffer submission queues.
- **Inspirations**: Khronos Vulkan specification, SteamOS Gamescope Vulkan compositor.

### 7. OpenGL / GLSL State Machine (`SovereignOpenGlEngine`)
- **Architecture**: Framebuffer Object (FBO) rasterization pipeline, VAO/VBO element array bindings, GLSL vertex/fragment shader program compilation emulator, and viewport clear passes.
- **Inspirations**: Mesa OpenGL driver layer.

### 8. Weston Reference Compositor (`SovereignWestonCompositorEngine`)
- **Architecture**: Output shell repaint manager orchestrating surface views, shell layout geometry, and GL/Pixman rendering backends.
- **Inspirations**: Wayland/Weston reference compositor.

### 9. Sway Tiling Window Manager (`SovereignSwayEngine`)
- **Architecture**: i3-compatible container tree supporting horizontal, vertical, tabbed, and stacked split layouts. Manages active workspaces and container focus trees.
- **Inspirations**: SwayWM tiling compositor.

### 10. wlroots Modular Compositor Library (`SovereignWlrootsEngine`)
- **Architecture**: Modular backend abstractions (`wlr_backend`), `wlr_scene` scene graph node creation, output layout management, and input routing.
- **Inspirations**: wlroots compositor library project.

### 11. Mutter Scene Graph & Fractional Scaling (`SovereignMutterEngine`)
- **Architecture**: Clutter actor transformation pipeline, Cogl rendering abstraction, and `wp_fractional_scale_v1` logical-to-buffer coordinate converter.
- **Inspirations**: GNOME Mutter compositor.

### 12. KWin Plasma Desktop Effects (`SovereignKWinEngine`)
- **Architecture**: Desktop compositing shader manager (dual-pass blur, contrast adjustment), window shadow border radius calculators, and window decoration height geometry evaluators.
- **Inspirations**: KDE KWin Plasma compositor.

### 13. Compiz 3D Desktop Cube & Physics (`SovereignCompizEngine`)
- **Architecture**: 3D desktop cube rotation matrix math, mass-spring-damper physics simulation for wobbly window drag effects (`accel = drag - k * disp`).
- **Inspirations**: Compiz Fusion 3D compositor.

### 14. Picom X11 Vsync & Blur Compositor (`SovereignPicomEngine`)
- **Architecture**: GLX/XRender vsync loop, dual-kawase multi-pass blur downsample/upsample pass generator, and active/inactive window opacity controls.
- **Inspirations**: Picom (formerly Compton) X11 compositor.

### 15. Xwayland Bridge Server (`SovereignXwaylandEngine`)
- **Architecture**: Rootless X11-to-Wayland window translation, Glamor 2D-over-3D acceleration bridge, and bi-directional clipboard text synchronization.
- **Inspirations**: Xwayland architecture in Fedora & Arch Linux.

### 16. evdev Input Binary Event Parser (`SovereignEvdevEngine`)
- **Architecture**: Binary `/dev/input/event*` packet reader parsing `EV_KEY`, `EV_REL`, and `EV_ABS` events, with 10-slot multitouch tracking (`ABS_MT_SLOT`, `ABS_MT_POSITION_X/Y`).
- **Inspirations**: Linux kernel `evdev` interface.

### 17. libinput Gesture & Acceleration Engine (`SovereignLibinputEngine`)
- **Architecture**: Adaptive pointer acceleration curve calculator (`multiplier = 1.0 + velocity * 0.05 * speed`), palm rejection touch pad margin checking, and multi-finger gesture recognition.
- **Inspirations**: `libinput` input stack library.

### 18. Console Framebuffer / fbdev (`SovereignFramebufferEngine`)
- **Architecture**: Direct linear framebuffer memory rendering, double-buffered back-buffer pixel operations, PSF font glyph rasterization, and hardware wscons/fbdev integration.
- **Inspirations**: Linux `/dev/fb0`, FreeBSD `/dev/fb` console framebuffers.

### 19. Virtual Terminal Matrix (`SovereignVirtualTerminalEngine`)
- **Architecture**: 80x25 character grid matrix, VT switching ioctls (`VT_ACTIVATE`), cursor tracking, and FreeBSD `vt(4)` / OpenBSD `wscons` keymap translation.
- **Inspirations**: FreeBSD `vt(4)` and OpenBSD `wscons(4)` drivers.

### 20. SteamOS Gamescope HDR & FSR Upscaling (`SovereignGamescopeHdrFsrEngine`)
- **Architecture**: FSR scale factor resolution computation, HDR PQ-ST2084 EOTF curve tonemapping, dynamic refresh rate (DRR) switching, and Steam overlay layer surface management.
- **Inspirations**: SteamOS Gamescope compositor.

### 21. Hyprland v2 Bezier Animations & Hyprcursor (`SovereignHyprlandBezierHyprcursorEngine`)
- **Architecture**: Cubic bezier animation easing curve evaluation, Hyprcursor SVG/PNG theme rendering, multi-pass blur shaders, and dynamic window class rules.
- **Inspirations**: Hyprland window manager & Hyprcursor project.

### 22. Nvidia PRIME Offload & NVK Open Driver Bridge (`SovereignNvidiaPrimeNvkBridgeEngine`)
- **Architecture**: `NV_PRIME_RENDER_OFFLOAD` GPU target selection, NVK open Vulkan driver feature querying, and CUDA/NVENC DMA-BUF surface handle sharing.
- **Inspirations**: Mesa NVK driver & Nvidia PRIME offloading.

### 23. Wayland Extended Protocols (`SovereignWaylandExtProtocolsEngine`)
- **Architecture**: `ext-session-lock-v1` security state management, `tearing-control-v1` async flip toggling, and `wp_color_management_v1` ICC color space transformations.
- **Inspirations**: Wayland-Protocols extended specifications.

### 24. System76 COSMIC / Smithay Rust Compositor (`SovereignCosmicSmithayCompositorEngine`)
- **Architecture**: Smithay Rust-native window layout evaluation, Iced shell widget layer surface spawning, and dock auto-hide state handling.
- **Inspirations**: System76 COSMIC desktop & Smithay compositor library.

### 25. BSD DRM Driver Isolation & Rump Graphics (`SovereignBsdDrmRumpGraphicsEngine`)
- **Architecture**: FreeBSD `kld` DRM kernel module loading, OpenBSD `inteldrm`/`radeondrm` KMS connector probing, and NetBSD Rump kernel userland DRM driver hypercall isolation.
- **Inspirations**: FreeBSD DRM/KMS, OpenBSD DRM, and NetBSD Rump Kernels.

---

## PR Gateway, PQC Signatures & CI Gating

All 31 PR proposals (#101..#131) are registered in `SovereignGraphicsDisplayPrGatewayEngine` featuring:
1. **Post-Quantum Cryptography (PQC)** signatures (`PQC-SIG-DILITHIUM5-GRAPHICS-PR-*`).
2. **SAT Dependency Constraint Solver** validation.
3. **Automated CI Gating Pipeline**: Checks PQC signature validity and SAT constraints before transitioning PR status from `Proposed` -> `CiValidationPass` -> `Merged`.
4. **Unified Diff Generation**: Computes standard git unified diff patches for each graphics subsystem component.

---

## Standalone Verification

Verified via standalone Rust test runner:
```bash
rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/graphics/sovereign_graphics_display_pr_suite.rs -o build/test_graphics_pr_suite && ./build/test_graphics_pr_suite
```
**Results**: 29 passed; 0 failed. All 29 unit tests and 26 system audit checks across X11, Wayland, Mesa, DRM/KMS, Vulkan, OpenGL, Weston, Sway, wlroots, Mutter, KWin, Compiz, Picom, Xwayland, evdev, libinput, fbdev, Virtual Terminals, Gamescope, Hyprland, Nvidia PRIME/NVK, Wayland Ext Protocols, COSMIC/Smithay, and BSD DRM driver isolation passed cleanly.
