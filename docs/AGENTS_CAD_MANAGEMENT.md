# SigmaOS AI Agent Computer Aided Design (CAD) Management Specification

This document specifies mandatory 2D/3D vector drafting standards, boundary representation (B-rep) geometry invariants, DXF/STEP/IGES translation pipelines, and GPU-accelerated mesh tessellation rules for autonomous AI engineering agents (Jules, Sentinel, Palette, Bolt) contributing to SigmaOS.

## 1. 2D & 3D Vector Drafting Engine Architecture
- **AutoCAD-Style Vector Engine (`src/unimplemented_tools.rs`)**:
  - Vector drafting engines (`AutocadVectorDraftEngine`) must manage parametric entities (lines, polylines, arcs, splines, 3D meshes) with double-precision floating point coordinate accuracy.
  - Dimension constraints (coincident, parallel, perpendicular, concentric) must be solved via iterative geometric constraint solvers.

## 2. Structural Engineering & Quantity Estimation Integration
- **Civil BOQ Estimation (`src/compatibility/india_professional_tools.rs`)**:
  - Quantity takeoff calculations (`CharteredEngineersBoqEstimator`) must extract volume and material weights directly from CAD entity geometry attributes.

## 3. Geometric Interchange Format Pipelines
- **DXF, STEP & IGES Parsing**:
  - Interchange parsers must sanitize entity layers, color tables, and block references before converting geometry into SigmaOS native CAD scene graphs.
- **GPU Mesh Tessellation**:
  - Curvatures and NURBS surfaces must be tessellated into GPU vertex buffers (`#[repr(align(64))]`) for direct scanout rendering via Zenith compositor graphics pipelines.

## 4. AI Agent CAD Management Directives
1. **Double Precision Geometry**: Use `f64` for all CAD vertex coordinates to prevent floating point accumulation errors during spatial transformations.
2. **Memory-Efficient Mesh Buffers**: Vertex and index buffers must use indexed triangle fan/strip representations to minimize RAM usage.
