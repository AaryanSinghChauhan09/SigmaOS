// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS Blender Pipeline — Headless Blender 3D render pipeline
// Provides scene graph, material/mesh/camera/light abstractions, render dispatch,
// GLB export, and geometry node definitions for offline and real-time rendering.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

// ---------------------------------------------------------------------------
// Render engine selection
// ---------------------------------------------------------------------------

/// Selects the Blender render engine for a given scene.
#[derive(Debug, Clone)]
pub enum RenderEngine {
    /// Path-traced engine with configurable sample count.
    Cycles { samples: u32 },
    /// Real-time rasterisation engine with configurable shadow map resolution.
    Eevee { shadow_map_res: u32 },
}

// ---------------------------------------------------------------------------
// Scene primitives
// ---------------------------------------------------------------------------

/// PBR material definition compatible with Blender's Principled BSDF.
#[derive(Debug, Clone)]
pub struct BlendMaterial {
    pub name: String,
    /// RGBA base colour in linear colour space [0.0, 1.0].
    pub base_color: [f32; 4],
    /// Metallic factor [0.0, 1.0].
    pub metallic: f32,
    /// Roughness factor [0.0, 1.0].
    pub roughness: f32,
}

impl BlendMaterial {
    pub fn new(name: &str, base_color: [f32; 4], metallic: f32, roughness: f32) -> Self {
        Self {
            name: name.to_string(),
            base_color,
            metallic,
            roughness,
        }
    }
}

/// Triangulated mesh with per-face material binding.
#[derive(Debug, Clone)]
pub struct BlendMesh {
    /// XYZ positions of each vertex.
    pub vertices: Vec<[f32; 3]>,
    /// Triangle faces: each element is a triplet of vertex indices.
    pub faces: Vec<[u32; 3]>,
    /// Index into the owning scene's material list.
    pub material_idx: u32,
}

impl BlendMesh {
    pub fn new(vertices: Vec<[f32; 3]>, faces: Vec<[u32; 3]>, material_idx: u32) -> Self {
        Self { vertices, faces, material_idx }
    }
}

/// Camera placement and optics.
#[derive(Debug, Clone)]
pub struct BlendCamera {
    /// World-space XYZ location.
    pub location: [f32; 3],
    /// Euler XYZ rotation in radians.
    pub rotation: [f32; 3],
    /// Horizontal field-of-view in degrees.
    pub fov_degrees: f32,
}

impl BlendCamera {
    pub fn new(location: [f32; 3], rotation: [f32; 3], fov_degrees: f32) -> Self {
        Self { location, rotation, fov_degrees }
    }
}

/// Light source variants supported by the pipeline.
#[derive(Debug, Clone)]
pub enum BlendLight {
    /// Omnidirectional point light.
    Point { location: [f32; 3], power: f32 },
    /// Distant directional sun light.
    Sun { direction: [f32; 3], strength: f32 },
    /// Rectangular area light.
    Area { size: f32 },
}

/// Full Blender scene graph.
#[derive(Debug, Clone)]
pub struct BlendScene {
    /// Mesh objects in the scene.
    pub objects: Vec<BlendMesh>,
    /// Material palette referenced by meshes.
    pub materials: Vec<BlendMaterial>,
    /// Active render camera.
    pub camera: BlendCamera,
    /// All lights contributing to illumination.
    pub lights: Vec<BlendLight>,
}

impl BlendScene {
    pub fn new(
        objects: Vec<BlendMesh>,
        materials: Vec<BlendMaterial>,
        camera: BlendCamera,
        lights: Vec<BlendLight>,
    ) -> Self {
        Self { objects, materials, camera, lights }
    }
}

// ---------------------------------------------------------------------------
// Geometry nodes
// ---------------------------------------------------------------------------

/// Boolean set operation used by the Boolean geometry node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoolOp {
    Union,
    Difference,
    Intersect,
}

/// Subset of Blender geometry node types relevant to procedural modelling.
#[derive(Debug, Clone)]
pub enum GeometryNode {
    /// Apply a local transform (translate/rotate/scale).
    Transform,
    /// Catmull-Clark subdivision surface.
    Subdivide,
    /// Constructive solid geometry boolean.
    Boolean { operation: BoolOp },
    /// Scatter instances across a surface.
    Distribute,
}

// ---------------------------------------------------------------------------
// Blender pipeline
// ---------------------------------------------------------------------------

/// Headless Blender rendering pipeline.
///
/// Encapsulates scene state and provides frame / animation rendering,
/// GLB export, and Python scripting hooks. In a kernel context the
/// actual I/O is delegated to the IPC layer; the stubs here return
/// deterministic placeholder data so that the pipeline compiles and
/// unit-tests in `no_std` environments.
pub struct BlenderPipeline {
    pub scene: BlendScene,
    pub engine: RenderEngine,
    /// Geometry modifier stack applied before rendering.
    pub modifiers: Vec<GeometryNode>,
}

impl BlenderPipeline {
    /// Construct a new pipeline for the given scene and render engine.
    pub fn new(scene: BlendScene, engine: RenderEngine) -> Self {
        Self { scene, engine, modifiers: Vec::new() }
    }

    /// Push a geometry node modifier onto the stack.
    pub fn add_modifier(&mut self, node: GeometryNode) {
        self.modifiers.push(node);
    }

    /// Render a single frame and write it to *output* path.
    ///
    /// Returns the raw PNG/EXR pixel bytes. In this stub the bytes
    /// encode a minimal 1×1 PNG header.
    pub fn render_frame(&self, output: &str) -> Vec<u8> {
        let _ = output;
        // Minimal PNG signature stub (8 bytes) so callers can check non-empty.
        alloc::vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
    }

    /// Render an animation range [start, end] at *fps* frames per second.
    ///
    /// In a real implementation this would call Blender's Python API
    /// via an IPC channel for each frame.
    pub fn render_animation(&self, start: u32, end: u32, fps: u32) {
        let _ = (start, end, fps);
    }

    /// Export the current scene to the GL Binary (GLB) format.
    ///
    /// Returns GLB magic bytes as a placeholder.
    pub fn export_glb(&self) -> Vec<u8> {
        // GLB magic: "glTF" + version 2
        alloc::vec![0x67, 0x6C, 0x54, 0x46, 0x02, 0x00, 0x00, 0x00]
    }

    /// Execute an arbitrary Python script inside the Blender runtime.
    pub fn run_python_script(&self, _script: &str) -> Result<(), &'static str> {
        // In a real kernel context this would IPC to a Blender worker.
        Ok(())
    }

    /// Return the number of polygons across all meshes in the scene.
    pub fn total_face_count(&self) -> usize {
        self.scene.objects.iter().map(|m| m.faces.len()).sum()
    }

    /// Return the sample count for Cycles, or 0 for Eevee.
    pub fn cycles_samples(&self) -> u32 {
        match &self.engine {
            RenderEngine::Cycles { samples } => *samples,
            _ => 0,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_scene() -> BlendScene {
        let mat = BlendMaterial::new(
            "metal",
            [0.8, 0.8, 0.8, 1.0],
            1.0,
            0.2,
        );
        let mesh = BlendMesh::new(
            alloc::vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.5, 1.0, 0.0]],
            alloc::vec![[0, 1, 2]],
            0,
        );
        let cam = BlendCamera::new([0.0, -5.0, 2.0], [1.1, 0.0, 0.0], 60.0);
        let light = BlendLight::Sun {
            direction: [0.0, 0.0, -1.0],
            strength: 5.0,
        };
        BlendScene::new(alloc::vec![mesh], alloc::vec![mat], cam, alloc::vec![light])
    }

    #[test]
    fn test_scene_construction() {
        let scene = make_scene();
        assert_eq!(scene.objects.len(), 1);
        assert_eq!(scene.materials.len(), 1);
        assert_eq!(scene.materials[0].name, "metal");
        assert_eq!(scene.objects[0].faces.len(), 1);
    }

    #[test]
    fn test_pipeline_cycles() {
        let scene = make_scene();
        let pipeline = BlenderPipeline::new(scene, RenderEngine::Cycles { samples: 128 });
        assert_eq!(pipeline.cycles_samples(), 128);
        assert_eq!(pipeline.total_face_count(), 1);
    }

    #[test]
    fn test_pipeline_eevee() {
        let scene = make_scene();
        let pipeline = BlenderPipeline::new(scene, RenderEngine::Eevee { shadow_map_res: 1024 });
        assert_eq!(pipeline.cycles_samples(), 0);
    }

    #[test]
    fn test_render_frame_returns_bytes() {
        let scene = make_scene();
        let pipeline = BlenderPipeline::new(scene, RenderEngine::Cycles { samples: 64 });
        let bytes = pipeline.render_frame("/tmp/frame.png");
        assert!(!bytes.is_empty());
        // Check PNG magic header
        assert_eq!(bytes[0], 0x89);
        assert_eq!(bytes[1], 0x50); // 'P'
    }

    #[test]
    fn test_export_glb() {
        let scene = make_scene();
        let pipeline = BlenderPipeline::new(scene, RenderEngine::Cycles { samples: 32 });
        let glb = pipeline.export_glb();
        assert_eq!(&glb[0..4], b"glTF");
    }

    #[test]
    fn test_run_python_script() {
        let scene = make_scene();
        let pipeline = BlenderPipeline::new(scene, RenderEngine::Eevee { shadow_map_res: 512 });
        assert!(pipeline.run_python_script("import bpy; bpy.ops.render.render()").is_ok());
    }

    #[test]
    fn test_modifiers() {
        let scene = make_scene();
        let mut pipeline = BlenderPipeline::new(scene, RenderEngine::Cycles { samples: 16 });
        pipeline.add_modifier(GeometryNode::Subdivide);
        pipeline.add_modifier(GeometryNode::Boolean { operation: BoolOp::Union });
        assert_eq!(pipeline.modifiers.len(), 2);
    }

    #[test]
    fn test_bool_ops() {
        assert_ne!(BoolOp::Union, BoolOp::Difference);
        assert_eq!(BoolOp::Intersect, BoolOp::Intersect);
    }
}
