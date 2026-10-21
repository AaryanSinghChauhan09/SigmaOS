# FigCraft: Technical Diagramming & Flowchart Engine

FigCraft is designed for systems architecture diagrams, network topologies, UML, and engineering flowcharts.

---

## 🎯 Parity Targets & Inspiration
- **Primary Targets**: draw.io, Microsoft Visio, OmniGraffle
- **Architectural Philosophy**: Built in `#![no_std]` Rust; zero telemetry, zero electron wrappers, sub-millisecond input response, native shared memory pipeline with the Zenith compositor.

---

## 🏛️ Key Structures & Types
```rust
pub enum ConnectorRouting { Straight, Orthogonal, CurvedBezier }
pub struct DiagramShape { pub shape_id: u64, pub shape_kind: u16, pub bounding_box: [f32; 4] }
```

---

## ✨ Features & Capabilities
- **Automatic obstacle-avoiding orthogonal line routing**
- **UML, cloud infrastructure, and network architecture icon libraries**
- **Live mathematical notation rendering using embedded TeX font engine**
- **Scalable SVG, PDF, and PNG export**

---

## ⚡ Integration with SigmaOS
This module shares the unified `CraftLibs` foundation, allowing direct drag-and-drop asset sharing with other Craft applications, instant theme adaptation, and GPU-accelerated drawing.
