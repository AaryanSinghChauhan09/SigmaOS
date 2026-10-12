[[Category:Applications]]
[[Category:Graphics]]

# Creative Suite

SigmaOS includes a comprehensive creative suite inspired by ArtCraft's Crafting Apps, providing professional-grade creative tools built in Rust.

## Overview

The SigmaOS Creative Suite consists of the following applications:

- **PhotoCraft** - Image editing with PSD support, adjustment layers, and smart objects
- **VectorCraft** - Vector illustration with hardware shaders and exact booleans
- **FilmCraft** - Video editing, color grading, and sound design
- **LightCraft** - Photo library and raw development
- **PdfCraft** - PDF reading, organizing, and protection
- **EffectCraft** - Motion graphics and visual effects
- **DesignCraft** - Page layout and publishing
- **ArtCraft** - AI image and video studio for artists

## PhotoCraft

PhotoCraft is a clean-room reimplementation of Adobe Photoshop in pure Rust, featuring:

### Features

- **Layer System**
  - Multiple layer types: Raster, Adjustment, Shape, Type, SmartObject
  - Layer masks with vector and clipping mask support
  - Layer styles: drop shadow, inner shadow, glow, bevel, stroke, overlays
  - 26 blend modes including: Normal, Multiply, Screen, Overlay, Color Dodge, Hard Light, Soft Light, Difference, Exclusion, and more

- **Selection Tools**
  - Rectangular and elliptical marquee
  - Lasso, polygonal lasso, magnetic lasso
  - Quick selection and magic wand
  - Color range selection
  - Subject and object selection (AI-powered)

- **Adjustment Layers**
  - Brightness/Contrast
  - Curves
  - Levels
  - Hue/Saturation
  - Color Balance
  - Vibrance
  - Photo Filter
  - Channel Mixer
  - Gradient Map
  - Selective Color

- **Filters**
  - Blur variants: Gaussian, Motion, Radial, Smart, Surface, Lens, Box
  - Sharpening: Unsharp Mask, High Pass
  - Noise: Add, Reduce, Gradient
  - AI-powered: Content-Aware Fill, Content-Aware Scale, Content-Aware Move
  - Warping: Liquify, Puppet Warp, Perspective Warp, Vanishing Point

- **Smart Filters**
  - Non-destructive filter application
  - Filter masks for selective effects
  - Smart object integration

### Usage

```rust
use sigmaos::creative::{PhotoEditor, LayerType, SelectionTool, Filter};

// Initialize PhotoCraft
let mut editor = PhotoEditor::new();
editor.init();

// Create layers
let bg_layer = editor.create_layer("Background".to_string(), LayerType::Raster);
let adj_layer = editor.create_layer("Curves Adjustment".to_string(), LayerType::Adjustment);

// Add layer mask
editor.add_layer_mask(bg_layer).unwrap();

// Apply filter
editor.apply_filter(bg_layer, Filter::GaussianBlur, 5.0).unwrap();

// Set selection tool
editor.set_selection_tool(SelectionTool::QuickSelection);
```

## VectorCraft

VectorCraft is a fast vector illustration engine built in pure Rust with hardware acceleration.

### Features

- **Shape Types**
  - Basic: Rectangle, Circle, Ellipse, Line
  - Advanced: Polygon, Star, Spiral, Arc
  - Freeform: Freehand, Pen, Pencil

- **Path System**
  - Bezier curves with control points
  - Fill rules: NonZero, EvenOdd
  - Smooth and corner path options

- **Boolean Operations**
  - Union
  - Intersection
  - Difference
  - Exclusion
  - Exact booleans that never fail

- **Pen Tool Modes**
  - Line mode
  - Curve mode
  - Corner mode

- **Hardware Acceleration**
  - GPU-accelerated rendering
  - Hardware shaders
  - 20,000 shapes rendered in 27ms

- **Shape Properties**
  - Fill and stroke colors
  - Stroke width
  - Opacity
  - Blend modes

### Usage

```rust
use sigmaos::creative::{VectorEditor, Shape, PathPoint, BooleanOperation, PenToolMode};

// Initialize VectorCraft
let mut editor = VectorEditor::new();
editor.init();

// Create layer
let layer_id = editor.create_layer("Vector Layer".to_string());

// Add shapes
editor.add_shape(layer_id, Shape::Rectangle).unwrap();

// Create bezier path
let point = PathPoint::with_controls(10.0, 20.0, 5.0, 15.0, 15.0, 25.0);
let path = Path {
    points: vec![point],
    closed: true,
    smooth: true,
    fill_rule: FillRule::NonZero,
};
editor.add_path(layer_id, path).unwrap();

// Perform boolean operation
editor.boolean_operation(layer_id, BooleanOperation::Union).unwrap();

// Set pen tool mode
editor.set_pen_tool_mode(PenToolMode::Curve);
```

## Installation

The creative suite is included in the SigmaOS base installation. Additional creative tools can be installed via the Software Manager:

```bash
sigma install photocraft
sigma install vectorcraft
sigma install filmcraft
sigma install lightcraft
```

## Configuration

### PhotoCraft Configuration

PhotoCraft settings are stored in `~/.config/sigmaos/photocraft/`:

- `config.toml` - Main configuration file
- `presets/` - Custom filter presets
- `profiles/` - Color profiles

### VectorCraft Configuration

VectorCraft settings are stored in `~/.config/sigmaos/vectorcraft/`:

- `config.toml` - Main configuration file
- `templates/` - Shape templates
- `palettes/` - Color palettes

## Tips and Tricks

### PhotoCraft

- Use adjustment layers for non-destructive editing
- Combine smart filters with layer masks for selective effects
- Keyboard shortcuts mirror Adobe Photoshop for easy transition
- PSD files are fully compatible with Adobe Photoshop

### VectorCraft

- Enable hardware shaders in settings for maximum performance
- Use exact booleans for precise shape operations
- Pen tool curves can be adjusted with control points
- Export to SVG for web compatibility

## Troubleshooting

### PhotoCraft

**Issue: Slow performance with large images**

Solution: Enable GPU acceleration in settings and increase memory allocation.

**Issue: PSD files not loading**

Solution: Ensure PSD compatibility is enabled in configuration. Some advanced Photoshop features may not be supported.

### VectorCraft

**Issue: Hardware shaders not working**

Solution: Check that your GPU drivers are up to date and Vulkan is properly configured.

**Issue: Boolean operations failing**

Solution: Ensure shapes are closed paths and don't have self-intersections.

## See also

- [PhotoCraft Website](https://getartcraft.com/apps/photocraft)
- [VectorCraft Website](https://getartcraft.com/apps/vectorcraft)
- [ArtCraft Suite](https://getartcraft.com/)
- [Graphics and Display](Graphics-and-Display.md)
- [Package Management](Package-Management.md)
