# Craft Creative & Office Productivity Suite: Overview

SigmaOS features a natively integrated, 20-module creative and professional office suite known as the **Craft Suite**. Built with `#![no_std]` Rust foundations, hardware-accelerated rendering, and modular IPC, the Craft Suite provides immediate out-of-the-box readiness for creators, engineers, and knowledge workers.

---

## 🎨 The 20 Craft Modules

| Module | Functional Domain | Inspiration / Parity Target | Key Capabilities |
| :--- | :--- | :--- | :--- |
| **1. 3DCraft** | 3D Modeling & Rendering | Blender, Maya | Polygon mesh editing, NURBS, BVH acceleration, shader graphs |
| **2. AniCraft** | 2D Animation Studio | OpenToonz, Krita, Flash | Frame-by-frame, vector tweening, onion skinning, easing curves |
| **3. ArtCraft** | Digital Painting & Concept Art | Krita, Procreate, Photoshop | Dynamic brush engine, pressure curves, multi-layer compositing |
| **4. CADCraft** | Parametric CAD & Engineering | FreeCAD, SolidWorks | 2D constraint sketches, 3D CSG extrude/revolve, STEP/IGES export |
| **5. CraftLibs** | Core Creative Foundation | Skia, Cairo, GEGL | ICC color profiles, SIMD math, canvas pipelines, undo/redo DAG |
| **6. DeckCraft** | Presentations & Slide Decks | Keynote, PowerPoint | Vector slide layout, smooth transitions, dual-screen presenter mode |
| **7. DesignCraft** | UI/UX Design & Prototyping | Figma, Penpot, Sketch | Vector UI layout, auto-layout flexbox, reusable components, tokens |
| **8. EffectCraft** | VFX Compositor & Motion FX | After Effects, Nuke | Node-based visual compositing, green screen keying, particle FX |
| **9. FigCraft** | Technical Diagrams & Figures | draw.io, Visio, OmniGraffle | Orthogonal routing, UML/architecture templates, auto-snapping |
| **10. FilmCraft** | Non-Linear Video Editor (NLE) | DaVinci Resolve, Premiere | Multi-track timeline, audio waveform sync, color grading LUTs |
| **11. FontCraft** | Typography & Font Studio | Glyphs, FontForge | Bézier glyph outlines, OpenType tables, kerning pairs, TTF export |
| **12. GridCraft** | High-Performance Spreadsheets | Excel, Google Sheets | Reactive formula DAG, 400+ math functions, 1M+ cell virtualization |
| **13. LightCraft** | Lighting Design & Grading | DaVinci Color, Capture | Stage lighting, DMX512 universe control, color temperature calibration |
| **14. PDFCraft** | PDF Document Suite | Adobe Acrobat, Preview | Vector rendering, form filling, text reflow, cryptographic signing |
| **15. PhotoCraft** | RAW Photo Development | Lightroom, Darktable | Non-destructive RAW pipeline, demosaicing, exposure curves |
| **16. PrintCraft** | Desktop Publishing (DTP) | InDesign, Scribus | CMYK color separations, bleed/slug marks, typographical hyphenation |
| **17. SoundCraft** | Digital Audio Workstation (DAW)| Ableton, Logic, Ardour | Multi-track recording, MIDI sequencing, DSP plugins, low-latency audio |
| **18. VectorCraft** | Vector Illustration | Illustrator, Inkscape | SVG2 compliance, Boolean path operations, gradient mesh editor |
| **19. VoiceCraft** | Speech Synthesis & Dictation | Whisper, ElevenLabs | Real-time speech-to-text dictation, neural TTS, voice commands |
| **20. WordCraft** | Typographic Word Processor | Word, Pages, LibreOffice | Rich typography, footnote/endnote tracking, table of contents, DOCX |

---

## 🏛️ Shared Architectural Foundation (CraftLibs)
All 20 Craft modules share a unified memory model:
- **Zero-Copy Canvas Buffers**: Framebuffers are shared directly with the Zenith desktop compositor via shared memory handles.
- **Hardware-Accelerated Math**: Vector transformations and raster math utilize AVX-512 and NEON SIMD intrinsics.
- **Unified Undo/Redo DAG**: Non-linear history branches enabling experimentation without state loss.
