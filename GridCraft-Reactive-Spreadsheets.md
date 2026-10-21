# GridCraft: High-Performance Reactive Spreadsheet Engine

GridCraft delivers lightning-fast calculation on million-row datasets with a reactive dependency DAG and modern charts.

---

## 🎯 Parity Targets & Inspiration
- **Primary Targets**: Microsoft Excel, Google Sheets, LibreOffice Calc
- **Architectural Philosophy**: Built in `#![no_std]` Rust; zero telemetry, zero electron wrappers, sub-millisecond input response, native shared memory pipeline with the Zenith compositor.

---

## 🏛️ Key Structures & Types
```rust
pub struct CellCoordinate { pub row: u32, pub col: u16 }
pub enum CellValue { Empty, Text(String), Number(f64), Boolean(bool), Error(String) }
pub struct FormulaNode { pub formula_text: String, pub dependencies: Vec<CellCoordinate> }
```

---

## ✨ Features & Capabilities
- **Topologically sorted formula evaluation DAG for instant recalculation**
- **400+ mathematical, statistical, financial, and string functions**
- **Virtual view scrolling supporting 10,000,000+ cells at 120 FPS**
- **ODS, XLSX, and CSV file reading and writing**

---

## ⚡ Integration with SigmaOS
This module shares the unified `CraftLibs` foundation, allowing direct drag-and-drop asset sharing with other Craft applications, instant theme adaptation, and GPU-accelerated drawing.
