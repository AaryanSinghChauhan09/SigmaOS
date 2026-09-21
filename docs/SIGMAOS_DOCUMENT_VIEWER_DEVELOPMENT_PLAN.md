# SigmaOS Document Viewer Subsystem (`sigma-view`) - Master Development Plan

## 1. Executive Summary & Vision

`sigma-view` (also known as `Sigma-DocView`) is the native, ultra-lightweight, high-performance document viewer for **SigmaOS**. Designed as a core component of the Zenith Wayland Desktop Environment, `sigma-view` provides seamless reading, searching, annotation, and digital signing capabilities across electronic document formats—including PDF, EPUB, DjVu, PostScript (PS/EPS), XPS, TIFF, and comic book archives (CBZ/CBR).

Drawing direct inspiration from the robust multi-format engine of KDE Okular, the minimalist keyboard-driven efficiency of Zathura on Arch Linux, the modular rendering architecture of GNOME Evince, the blazing speed of MuPDF on Gentoo, and the security sandboxing of OpenBSD's `pledge`/`unveil` applied to untrusted PDF streams, `sigma-view` delivers an uncompromising document viewing experience that is fast, accessible, secure, and resource-efficient.

---

## 2. Inspirations from Linux & BSD Ecosystems

| Ecosystem Origin | Feature & Subsystem Inspired | Target Implementation Subsystem |
| :--- | :--- | :--- |
| **KDE Okular (Fedora / openSUSE)** | Multi-format backend engine, rich PDF annotation layers (highlighters, inline text notes, stamps), form filling, and PKCS#7 X.509 digital signature verification. | `src/media/document_viewer.rs` & `src/productivity/` |
| **Zathura & Arch Linux** | Modal, vim-like keyboard navigation (`h/j/k/l`, `/` search, `gg`/`G`, zoom `+`/`-`, dual-page layout `d`), minimal screen footprint, and customizable keybindings. | `src/desktop/zenith_window_manager.rs` |
| **GNOME Evince (Ubuntu / Debian)** | Automatic document state persistence (page number, zoom level, scroll position), continuous scroll, thumbnail sidebar, and AT-SPI2 screen reader accessibility. | `src/desktop/` & `src/accessibility/` |
| **MuPDF & Gentoo / Void** | High-speed multi-threaded page rasterization, subpixel anti-aliasing, memory-mapped page cache, and low memory footprint (< 35MB RAM). | `src/media/rasterizer.rs` |
| **OpenBSD & FreeBSD Ports** | OpenBSD `pledge("stdio rpath prot_exec")` and `unveil` sandboxing for untrusted PDF parser streams, isolating complex C/C++ font and stream decoders. | `src/security/pledge.rs` & `src/security/landlock.rs` |
| **Alpine Linux & Void Linux** | Zero-dependency static compilation, instant startup (< 10ms cold boot), and low dependency overhead. | `src/distro/linux_bsd_inspirations.rs` |

---

## 3. 5-Layer Document Viewer Architecture

```
┌────────────────────────────────────────────────────────────────────────┐
│ Layer 5: Wayland UI & Zenith Desktop Integration (AT-SPI2, Dark Mode)  │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 4: Sandboxed Document Parsing Engine (Landlock, Pledge & Unveil) │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 3: Interactive Annotations, Form Filling & Digital Signatures    │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 2: Vector Layout & GPU Accelerated Rendering Engine             │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 1: Multiformat Parsing Kernel & Rasterizer (PDF, DjVu, EPUB, PS) │
└────────────────────────────────────────────────────────────────────────┘
```

### Layer 1: Multiformat Parsing Kernel & Rasterizer
- **PDF Core Engine:** Fast, standards-compliant parsing of PDF 1.7 / 2.0 specifications, including compressed stream objects, type 1/2/3 fonts, TrueType/OpenType embedded fonts, and CID maps.
- **Multi-Format Adapters:** Dedicated backends for EPUB (reflowable text & HTML rendering), DjVu (bilevel & layered color image compression), PostScript (Ghostscript/libspectre vector rasterization), XPS, and CBZ/CBR (archive image extractors).
- **Memory-Mapped Page Cache:** Zero-copy page loading via `mmap`, permitting instant navigation across multi-thousand-page documents without memory bloat.

### Layer 2: Vector Layout & GPU Accelerated Rendering Engine
- **Asynchronous Tile Rasterization:** Off-main-thread multi-core rasterization producing crisp vector graphics and anti-aliased glyphs.
- **Wayland Fractional Scaling Support:** Crisp rendering at 125%, 150%, and 200% DPI scales without bitmap pixelation.
- **Smart Color Inversion / Dark Mode:** Intelligent document color inversion for night reading that preserves image and photograph natural colors while darkening text backgrounds.

### Layer 3: Interactive Annotations, Form Filling & Digital Signatures
- **Annotation Subsystem:** Freehand drawings, highlights, text notes, strikethroughs, underline overlays, and pop-up comments saved into PDF incremental updates or external sidecar JSON files.
- **Interactive Forms (AcroForms / XFA):** Text inputs, checkboxes, radio buttons, drop-down selects, and form field calculation engines.
- **Digital Signatures & PDF/A Validation:** Verification of X.509 PKCS#7 digital signatures, certificate chain validation, and PDF/A archival compliance checking.

### Layer 4: Sandboxed Document Parsing Engine
- **Process Isolation:** Document parsing and rendering run in dedicated worker subprocesses.
- **Landlock & OpenBSD Pledge/Unveil:** Parser subprocesses are restricted with `pledge("stdio rpath")` and `unveil` strictly to the target document path, preventing malformed PDFs from reading `~/.ssh` or executing shell code.
- **Memory Bounds & Panic Guards:** Safe Rust memory management guaranteeing protection against classic heap buffer overflows, use-after-free, and double-free exploits common in legacy C document viewers.

### Layer 5: Wayland UI & Zenith Desktop Integration
- **Zenith Native Interface:** Built with Zenith Wayland widgets featuring thumbnail sidebar, document outline (table of contents), bookmark manager, and text search overlay.
- **Keyboard-Driven Vim Navigation:** Full vim keybinding modes (`j`/`k` scroll, `Ctrl+F`/`Ctrl+B` page jump, `/` regex search, `zi`/`zo` zoom).
- **Accessibility & AT-SPI2:** Full screen reader accessibility integration, exporting document text layout trees and structural headings to assistive technologies.

---

## 4. Implementation Roadmap

| Milestone | Target Component | Objectives | Status |
| :--- | :--- | :--- | :--- |
| **Milestone 1** | Multiformat Kernel | Build core document parser traits, PDF/EPUB/DjVu file header detection, and page count extraction. | Implemented |
| **Milestone 2** | Rasterizer & Layout | Implement async tile rasterizer, memory-mapped page cache, continuous scrolling, and dark mode color inversion. | Implemented |
| **Milestone 3** | Annotations & Forms | Implement PDF AcroForms interactive input handling, text annotations, and PKCS#7 signature verification. | Implemented |
| **Milestone 4** | Security Sandbox | Apply Landlock filesystem restrictions and OpenBSD `pledge`/`unveil` boundaries to document worker subprocesses. | Implemented |
| **Milestone 5** | Desktop & Accessibility | Integrate Zenith Wayland UI, thumbnail sidebars, vim keyboard navigation, and AT-SPI2 screen reader exports. | Implemented |

---

## 5. Verification & Testing Strategy

1. **Unit Testing:** Executing standalone Rust test suites in `src/media/` and `src/desktop/` to verify document parsing, page navigation, and annotation serialization.
2. **Security Sandboxing Validation:** Verifying that sandboxed worker processes cannot access paths outside `unveil` boundaries.
3. **Automated Continuous Integration:** Running `./run_sigma_tests.sh` to ensure all native unit tests pass cleanly without regressions.
