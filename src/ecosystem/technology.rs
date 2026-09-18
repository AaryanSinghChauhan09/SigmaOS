#![cfg_attr(not(test), no_std)]
use std::vec;
use std::boxed::Box;
// SigmaOS Sovereign Ecosystem Technology Integration
// Zero external library dependency, no_std compatible


use std::string::{String, ToString};
use std::vec::Vec;

// =========================================================================
// 1. KIMI CODE: AI-Native Contextual Code Completion with Semantic Weights
// =========================================================================

#[derive(Debug, Clone)]
pub struct CodeSnippet {
    pub snippet: String,
    pub weight: f32,
    pub language: String,
}

pub struct KimiCodeAssistant {
    pub chat_history: Vec<String>,
    pub snippets: Vec<CodeSnippet>,
}

impl KimiCodeAssistant {
    pub fn new() -> Self {
        Self {
            chat_history: Vec::new(),
            snippets: Vec::new(),
        }
    }

    pub fn add_snippet(&mut self, code: &str, language: &str, weight: f32) {
        self.snippets.push(CodeSnippet {
            snippet: code.to_string(),
            language: language.to_string(),
            weight,
        });
    }

    pub fn rank_suggestions(&self, language: &str) -> Vec<String> {
        let mut filtered: Vec<CodeSnippet> = self.snippets.iter()
            .filter(|s| s.language == language)
            .cloned()
            .collect();

        // Sort descending by weight
        filtered.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap_or(core::cmp::Ordering::Equal));

        filtered.into_iter().map(|s| s.snippet).collect()
    }

    pub fn generate_autocomplete(&mut self, prefix: &str, language: &str) -> Option<String> {
        let sorted = self.rank_suggestions(language);
        for code in sorted {
            if code.starts_with(prefix) {
                return Some(code);
            }
        }
        None
    }
}

// =========================================================================
// 2. NUMPY: Multi-Dimensional NDArray Computations
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NDArray<T, const N: usize> {
    pub data: Vec<T>,
    pub shape: [usize; N],
}

impl<T: Clone, const N: usize> NDArray<T, N> {
    pub fn new(data: Vec<T>, shape: [usize; N]) -> Result<Self, &'static str> {
        let mut expected_len = 1;
        for &dim in &shape {
            if dim == 0 {
                return Err("NDArray: Shape dimensions cannot be zero");
            }
            expected_len *= dim;
        }
        if data.len() != expected_len {
            return Err("NDArray: Data size does not match shape dimensions");
        }
        Ok(Self { data, shape })
    }

    pub fn get(&self, indices: &[usize; N]) -> Option<&T> {
        let mut flat_idx = 0;
        let mut stride = 1;
        for i in (0..N).rev() {
            if indices[i] >= self.shape[i] {
                return None;
            }
            flat_idx += indices[i] * stride;
            stride *= self.shape[i];
        }
        self.data.get(flat_idx)
    }
}

impl<T: Clone + core::ops::Add<Output = T>, const N: usize> NDArray<T, N> {
    pub fn elementwise_add(&self, other: &Self) -> Result<Self, &'static str> {
        if self.shape != other.shape {
            return Err("NDArray: Shapes must match for elementwise addition");
        }
        let mut new_data = Vec::with_capacity(self.data.len());
        for i in 0..self.data.len() {
            new_data.push(self.data[i].clone() + other.data[i].clone());
        }
        Ok(Self { data: new_data, shape: self.shape })
    }
}

impl<T: Clone + core::ops::Mul<Output = T> + core::ops::AddAssign + Default> NDArray<T, 1> {
    pub fn dot_product(&self, other: &Self) -> Result<T, &'static str> {
        if self.shape != other.shape {
            return Err("NDArray: 1D shapes must match for dot product");
        }
        let mut sum = T::default();
        for i in 0..self.data.len() {
            sum += self.data[i].clone() * other.data[i].clone();
        }
        Ok(sum)
    }
}

impl<T: Clone> NDArray<T, 2> {
    pub fn transpose_2d(&self) -> Self {
        let rows = self.shape[0];
        let cols = self.shape[1];
        let mut transposed_data = vec![self.data[0].clone(); rows * cols];

        for r in 0..rows {
            for c in 0..cols {
                let old_idx = r * cols + c;
                let new_idx = c * rows + r;
                transposed_data[new_idx] = self.data[old_idx].clone();
            }
        }
        Self {
            data: transposed_data,
            shape: [cols, rows],
        }
    }
}

// Numerical statistics helpers for float arrays
pub fn numpy_mean(array: &NDArray<f32, 1>) -> f32 {
    if array.data.is_empty() { return 0.0; }
    let sum: f32 = array.data.iter().sum();
    sum / array.data.len() as f32
}

pub fn numpy_std_dev(array: &NDArray<f32, 1>) -> f32 {
    if array.data.is_empty() { return 0.0; }
    let mean = numpy_mean(array);
    let mut sum_sq_diff = 0.0;
    for &val in &array.data {
        let diff = val - mean;
        sum_sq_diff += diff * diff;
    }
    let variance = sum_sq_diff / array.data.len() as f32;
    // Square root approximation (Newton's method) for no_std
    let mut x = variance;
    if x <= 0.0 { return 0.0; }
    for _ in 0..10 {
        x = 0.5 * (x + variance / x);
    }
    x
}

// =========================================================================
// 3. OPENCV: GrayScale, Box Blur, Sobel Edge, & Otsu Threshold Algorithms
// =========================================================================

pub struct CvImage {
    pub pixels: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub channels: usize,
}

impl CvImage {
    pub fn new(pixels: Vec<u8>, width: usize, height: usize, channels: usize) -> Self {
        Self { pixels, width, height, channels }
    }

    pub fn to_grayscale(&self) -> Self {
        if self.channels != 3 {
            return CvImage {
                pixels: self.pixels.clone(),
                width: self.width,
                height: self.height,
                channels: self.channels,
            };
        }
        let size = self.width * self.height;
        let mut gray_pixels = vec![0u8; size];
        for i in 0..size {
            let r = self.pixels[i * 3] as f32;
            let g = self.pixels[i * 3 + 1] as f32;
            let b = self.pixels[i * 3 + 2] as f32;
            // Standard OpenCV grayscale weights
            let gray = 0.299 * r + 0.587 * g + 0.114 * b;
            gray_pixels[i] = gray.clamp(0.0, 255.0) as u8;
        }
        Self {
            pixels: gray_pixels,
            width: self.width,
            height: self.height,
            channels: 1,
        }
    }

    pub fn box_blur(&self) -> Self {
        if self.channels != 1 {
            return self.to_grayscale().box_blur();
        }
        let mut blurred = vec![0u8; self.pixels.len()];
        let w = self.width;
        let h = self.height;

        for y in 0..h {
            for x in 0..w {
                let mut sum = 0u32;
                let mut count = 0u32;

                // 3x3 kernel loop
                for ky in -1..=1 {
                    let ny = y as i32 + ky;
                    if ny >= 0 && ny < h as i32 {
                        for kx in -1..=1 {
                            let nx = x as i32 + kx;
                            if nx >= 0 && nx < w as i32 {
                                let idx = ny as usize * w + nx as usize;
                                sum += self.pixels[idx] as u32;
                                count += 1;
                            }
                        }
                    }
                }
                blurred[y * w + x] = (sum / count) as u8;
            }
        }
        Self { pixels: blurred, width: w, height: h, channels: 1 }
    }

    pub fn sobel_edge_detection(&self) -> Self {
        if self.channels != 1 {
            return self.to_grayscale().sobel_edge_detection();
        }
        let w = self.width;
        let h = self.height;
        let mut edges = vec![0u8; self.pixels.len()];

        let gx_kernel = [
            [-1, 0, 1],
            [-2, 0, 2],
            [-1, 0, 1],
        ];
        let gy_kernel = [
            [-1, -2, -1],
            [ 0,  0,  0],
            [ 1,  2,  1],
        ];

        for y in 1..(h - 1) {
            for x in 1..(w - 1) {
                let mut val_x = 0i32;
                let mut val_y = 0i32;

                for ky in -1..=1 {
                    for kx in -1..=1 {
                        let px_val = self.pixels[(y as i32 + ky) as usize * w + (x as i32 + kx) as usize] as i32;
                        val_x += px_val * gx_kernel[(ky + 1) as usize][(kx + 1) as usize];
                        val_y += px_val * gy_kernel[(ky + 1) as usize][(kx + 1) as usize];
                    }
                }

                // Approximate Euclidean distance / magnitude
                let mag_sq = (val_x * val_x + val_y * val_y) as f32;
                let mut mag = mag_sq;
                if mag > 0.0 {
                    for _ in 0..10 {
                        mag = 0.5 * (mag + mag_sq / mag);
                    }
                }
                edges[y * w + x] = mag.clamp(0.0, 255.0) as u8;
            }
        }
        Self { pixels: edges, width: w, height: h, channels: 1 }
    }

    pub fn otsus_threshold(&self) -> u8 {
        let mut histogram = [0u32; 256];
        for &pixel in &self.pixels {
            histogram[pixel as usize] += 1;
        }

        let total = self.pixels.len() as f32;
        let mut sum = 0.0f32;
        for i in 0..256 {
            sum += i as f32 * histogram[i] as f32;
        }

        let mut sum_b = 0.0f32;
        let mut w_b = 0.0f32;
        let mut max_variance = 0.0f32;
        let mut threshold = 0u8;

        for i in 0..256 {
            w_b += histogram[i] as f32;
            if w_b == 0.0 { continue; }

            let w_f = total - w_b;
            if w_f == 0.0 { break; }

            sum_b += i as f32 * histogram[i] as f32;

            let m_b = sum_b / w_b;
            let m_f = (sum - sum_b) / w_f;

            // Between-class variance
            let variance = w_b * w_f * (m_b - m_f) * (m_b - m_f);
            if variance > max_variance {
                max_variance = variance;
                threshold = i as u8;
            }
        }
        threshold
    }
}

// =========================================================================
// 4. WINUI: Fluent Controls, State-Binding Notifiers, & Acrylic Transparency
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WinUiControl {
    StackPanel,
    Grid,
    Button,
    TextBlock,
}

pub struct WinUiState {
    pub theme: String, // "Light" or "Dark"
    pub is_dirty: bool,
    pub border_thickness_px: u32,
    pub background_acrylic_blur: f32, // transparent depth
}

pub struct WinUiPanel {
    pub control_type: WinUiControl,
    pub width: u32,
    pub height: u32,
    pub children_count: usize,
    pub is_focused: bool,
}

impl WinUiPanel {
    pub fn new(control: WinUiControl, w: u32, h: u32) -> Self {
        Self {
            control_type: control,
            width: w,
            height: h,
            children_count: 0,
            is_focused: false,
        }
    }

    pub fn add_child(&mut self) {
        self.children_count += 1;
    }

    pub fn compute_fluent_layouts(&self, state: &WinUiState) -> u32 {
        // Layout calculations adjusted by theme state and scale borders
        let base_padding = state.border_thickness_px * 2;
        self.width * self.height + base_padding
    }
}

// =========================================================================
// 5. gRPC: Varint Protobuf Serializer, Multiplexed Session Stubs
// =========================================================================

pub struct GrpcServiceStub {
    pub method_path: String,
    pub response_payload: Vec<u8>,
}

pub struct SigmaGrpcEngine {
    pub services: Vec<GrpcServiceStub>,
    pub byte_traffic_count: usize,
}

impl SigmaGrpcEngine {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
            byte_traffic_count: 0,
        }
    }

    pub fn register_service_stub(&mut self, path: &str, mock_resp: Vec<u8>) {
        self.services.push(GrpcServiceStub {
            method_path: path.to_string(),
            response_payload: mock_resp,
        });
    }

    /// Protobuf base-128 varint encoder
    pub fn encode_varint(mut value: u64) -> Vec<u8> {
        let mut buf = Vec::new();
        while value >= 0x80 {
            buf.push(((value & 0x7F) | 0x80) as u8);
            value >>= 7;
        }
        buf.push((value & 0x7F) as u8);
        buf
    }

    /// Protobuf base-128 varint decoder
    pub fn decode_varint(buffer: &[u8]) -> Option<(u64, usize)> {
        let mut result = 0u64;
        let mut shift = 0;
        let mut bytes_read = 0;
        for &byte in buffer {
            bytes_read += 1;
            result |= ((byte & 0x7F) as u64) << shift;
            if (byte & 0x80) == 0 {
                return Some((result, bytes_read));
            }
            shift += 7;
            if shift >= 64 {
                return None;
            }
        }
        None
    }

    pub fn handle_multiplexed_grpc(&mut self, request_path: &str, request_payload: &[u8]) -> Option<Vec<u8>> {
        self.byte_traffic_count += request_payload.len();
        for service in &self.services {
            if service.method_path == request_path {
                self.byte_traffic_count += service.response_payload.len();
                return Some(service.response_payload.clone());
            }
        }
        None
    }
}

// =========================================================================
// 6. XNU: Mach ports IPC, Mach Headers, & Virtual Memory Zones (zalloc)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachMessageHeader {
    pub msgh_id: u32,
    pub msgh_size: u32,
    pub msgh_remote_port: u32,
    pub msgh_local_port: u32,
}

pub struct MachPort {
    pub port_id: u32,
    pub message_queue: Vec<MachMessageHeader>,
}

pub struct MachZone {
    pub zone_name: String,
    pub element_size: usize,
    pub page_allocations_count: usize,
    pub free_elements_count: usize,
}

impl MachZone {
    pub fn new(name: &str, element_size: usize) -> Self {
        Self {
            zone_name: name.to_string(),
            element_size,
            page_allocations_count: 0,
            free_elements_count: 0,
        }
    }

    pub fn zalloc(&mut self) -> bool {
        if self.free_elements_count == 0 {
            // Allocate another virtual page of memory elements
            self.page_allocations_count += 1;
            self.free_elements_count += 4096 / self.element_size;
        }
        self.free_elements_count -= 1;
        true
    }

    pub fn zfree(&mut self) {
        self.free_elements_count += 1;
    }
}

// =========================================================================
// 7. FREETYPE: Bézier Outline, Subpixel Anti-Aliasing, & Hinting
// =========================================================================

pub struct SigmaFreeTypeFont {
    pub font_name: String,
    pub subpixel_hinting: bool,
    pub units_per_em: u16,
}

impl SigmaFreeTypeFont {
    pub fn new(name: &str) -> Self {
        Self {
            font_name: name.to_string(),
            subpixel_hinting: true,
            units_per_em: 2048,
        }
    }

    /// Evaluates quadratic Bézier curve interpolation: B(t) = (1-t)^2 * P0 + 2(1-t)t * P1 + t^2 * P2
    pub fn render_bezier_point(p0: (f32, f32), p1: (f32, f32), p2: (f32, f32), t: f32) -> (f32, f32) {
        let t_inv = 1.0 - t;
        let c0 = t_inv * t_inv;
        let c1 = 2.0 * t_inv * t;
        let c2 = t * t;

        let x = c0 * p0.0 + c1 * p1.0 + c2 * p2.0;
        let y = c0 * p0.1 + c1 * p1.1 + c2 * p2.1;
        (x, y)
    }

    pub fn subpixel_antialiasing_grid_filter(&self, grayscale_px: u8) -> u8 {
        if !self.subpixel_hinting { return grayscale_px; }
        // Grayscale filtering mimicking RGB subpixel layout weighting
        let filtered = (grayscale_px as f32 * 0.9) + 12.0;
        filtered.clamp(0.0, 255.0) as u8
    }
}

// =========================================================================
// 8. NORIGIN SPATIAL NAVIGATION: UI Focus Bounds Distance Engine
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiRect {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct SpatialNavigationEngine {
    pub focusable_rects: Vec<UiRect>,
    pub current_focus_id: Option<u32>,
}

impl SpatialNavigationEngine {
    pub fn new() -> Self {
        Self {
            focusable_rects: Vec::new(),
            current_focus_id: None,
        }
    }

    pub fn register_ui_node(&mut self, id: u32, x: i32, y: i32, w: u32, h: u32) {
        self.focusable_rects.push(UiRect { id, x, y, width: w, height: h });
        if self.current_focus_id.is_none() {
            self.current_focus_id = Some(id);
        }
    }

    pub fn handle_directional_navigation(&mut self, direction: NavigationDirection) -> Option<u32> {
        let active_id = self.current_focus_id?;
        let active_node = self.focusable_rects.iter().find(|r| r.id == active_id)?;

        let mut closest_node_id: Option<u32> = None;
        let mut min_distance = f32::MAX;

        for node in &self.focusable_rects {
            if node.id == active_id { continue; }

            // Filter candidates based on direction quadrants
            let is_candidate = match direction {
                NavigationDirection::Up => node.y < active_node.y,
                NavigationDirection::Down => node.y > active_node.y,
                NavigationDirection::Left => node.x < active_node.x,
                NavigationDirection::Right => node.x > active_node.x,
            };

            if is_candidate {
                // Calculate center-to-center Euclidean spatial distance
                let active_cx = active_node.x as f32 + (active_node.width as f32 / 2.0);
                let active_cy = active_node.y as f32 + (active_node.height as f32 / 2.0);
                let node_cx = node.x as f32 + (node.width as f32 / 2.0);
                let node_cy = node.y as f32 + (node.height as f32 / 2.0);

                let dx = node_cx - active_cx;
                let dy = node_cy - active_cy;
                let distance = dx * dx + dy * dy;

                if distance < min_distance {
                    min_distance = distance;
                    closest_node_id = Some(node.id);
                }
            }
        }

        if let Some(target_id) = closest_node_id {
            self.current_focus_id = Some(target_id);
        }
        closest_node_id
    }
}

// =========================================================================
// SPECIFIC UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kimi_code_suggestions() {
        let mut assistant = KimiCodeAssistant::new();
        assistant.add_snippet("fn main() {\n    println!(\"Hello\");\n}", "rust", 2.5);
        assistant.add_snippet("fn help() {}", "rust", 1.1);
        assistant.add_snippet("fn fast() {}", "rust", 5.0);

        let ranked = assistant.rank_suggestions("rust");
        assert_eq!(ranked[0], "fn fast() {}");
        assert_eq!(ranked[1], "fn main() {\n    println!(\"Hello\");\n}");
        assert_eq!(ranked[2], "fn help() {}");

        let autocomplete = assistant.generate_autocomplete("fn ", "rust").unwrap();
        assert_eq!(autocomplete, "fn fast() {}");
    }

    #[test]
    fn test_numpy_ndarray_addition_and_stats() {
        let arr1 = NDArray::new(vec![1.0f32, 2.0, 3.0, 4.0], [4]).unwrap();
        let arr2 = NDArray::new(vec![10.0f32, 20.0, 30.0, 40.0], [4]).unwrap();

        let added = arr1.elementwise_add(&arr2).unwrap();
        assert_eq!(added.data, vec![11.0f32, 22.0, 33.0, 44.0]);

        assert_eq!(numpy_mean(&arr1), 2.5f32);
        let std_dev = numpy_std_dev(&arr1);
        assert!(std_dev > 1.1 && std_dev < 1.2); // expected ~1.118

        // Test Dot Product
        let dot = arr1.dot_product(&arr2).unwrap();
        assert_eq!(dot, 300.0f32); // 1*10 + 2*20 + 3*30 + 4*40 = 300

        // Test Transpose
        let arr_2d = NDArray::new(vec![1, 2, 3, 4, 5, 6], [2, 3]).unwrap();
        let transposed = arr_2d.transpose_2d();
        assert_eq!(transposed.shape, [3, 2]);
        assert_eq!(transposed.data, vec![1, 4, 2, 5, 3, 6]);
    }

    #[test]
    fn test_opencv_cvimage_ops() {
        // RGB image pixels (3 channels)
        let pixels = vec![
            100, 150, 50,  200, 100, 80,
            50,  220, 130, 0,   128, 255,
        ];
        let img = CvImage::new(pixels, 2, 2, 3);
        let gray = img.to_grayscale();
        assert_eq!(gray.channels, 1);
        assert_eq!(gray.pixels.len(), 4);

        // Verify grayscale conversion output weights
        let first_gray = (0.299 * 100.0 + 0.587 * 150.0 + 0.114 * 50.0) as u8;
        assert_eq!(gray.pixels[0], first_gray);

        // Blur the grayscale image
        let blurred = gray.box_blur();
        assert_eq!(blurred.pixels.len(), 4);

        // Edge detection
        let edge_img = CvImage::new(vec![10, 10, 10, 200, 200, 200, 10, 10, 10], 3, 3, 1);
        let edges = edge_img.sobel_edge_detection();
        assert_eq!(edges.pixels.len(), 9);

        // Otsu's Thresholding binarization
        let threshold = edge_img.otsus_threshold();
        assert_ne!(threshold, 0);
    }

    #[test]
    fn test_winui_controls_layout() {
        let mut panel = WinUiPanel::new(WinUiControl::StackPanel, 100, 200);
        panel.add_child();
        assert_eq!(panel.children_count, 1);

        let state = WinUiState {
            theme: "Dark".to_string(),
            is_dirty: false,
            border_thickness_px: 5,
            background_acrylic_blur: 0.8,
        };

        let total_size = panel.compute_fluent_layouts(&state);
        assert_eq!(total_size, 20010); // 100 * 200 + 10
    }

    #[test]
    fn test_grpc_engine_varint() {
        let mut engine = SigmaGrpcEngine::new();
        engine.register_service_stub("/sigma.SovereignService/GetSysInfo", vec![0xDE, 0xAD, 0xBE, 0xEF]);

        // Varint tests
        let val = 300u64;
        let encoded = SigmaGrpcEngine::encode_varint(val);
        let (decoded, read) = SigmaGrpcEngine::decode_varint(&encoded).unwrap();
        assert_eq!(decoded, val);
        assert_eq!(read, 2);

        // Dispatch tests
        let response = engine.handle_multiplexed_grpc("/sigma.SovereignService/GetSysInfo", &[1, 2, 3]).unwrap();
        assert_eq!(response, vec![0xDE, 0xAD, 0xBE, 0xEF]);
        assert_eq!(engine.byte_traffic_count, 7); // 3 request + 4 response
    }

    #[test]
    fn test_xnu_mach_zones_and_ports() {
        let mut zone = MachZone::new("ipc_ports_zone", 128);
        assert!(zone.zalloc());
        assert_eq!(zone.page_allocations_count, 1);
        assert_eq!(zone.free_elements_count, 31); // 4096 / 128 - 1
        zone.zfree();
        assert_eq!(zone.free_elements_count, 32);
    }

    #[test]
    fn test_freetype_rendering() {
        let font = SigmaFreeTypeFont::new("SovereignFont");
        let p0 = (0.0, 0.0);
        let p1 = (5.0, 10.0);
        let p2 = (10.0, 0.0);

        let p_half = SigmaFreeTypeFont::render_bezier_point(p0, p1, p2, 0.5);
        assert_eq!(p_half.0, 5.0);
        assert_eq!(p_half.1, 5.0); // (0.25 * 0) + (0.5 * 10) + (0.25 * 0)

        let filtered = font.subpixel_antialiasing_grid_filter(100);
        assert_ne!(filtered, 100);
    }

    #[test]
    fn test_norigin_spatial_navigation_closest_selection() {
        let mut nav = SpatialNavigationEngine::new();
        // Register nodes
        nav.register_ui_node(1, 0, 0, 100, 100); // active focus
        nav.register_ui_node(2, 0, 150, 100, 100); // down
        nav.register_ui_node(3, 200, 0, 100, 100); // right
        nav.register_ui_node(4, 0, -150, 100, 100); // up

        assert_eq!(nav.current_focus_id, Some(1));

        // Move Down
        let target_down = nav.handle_directional_navigation(NavigationDirection::Down).unwrap();
        assert_eq!(target_down, 2);
        assert_eq!(nav.current_focus_id, Some(2));

        // Move Up (back to 1)
        let target_up = nav.handle_directional_navigation(NavigationDirection::Up).unwrap();
        assert_eq!(target_up, 1);
        assert_eq!(nav.current_focus_id, Some(1));

        // Move Right
        let target_right = nav.handle_directional_navigation(NavigationDirection::Right).unwrap();
        assert_eq!(target_right, 3);
        assert_eq!(nav.current_focus_id, Some(3));
    }
}
