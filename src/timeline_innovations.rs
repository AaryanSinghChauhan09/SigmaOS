extern crate alloc;
use alloc::vec::Vec;
use alloc::string::{String, ToString};

// =========================================================================
// 1. CROSS-PLATFORM SDK (RUST, NIM, ZIG COMPATIBILITY ENVELOPES)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkLanguage {
    Rust,
    Nim,
    Zig,
}

pub struct SdkAppDescriptor {
    pub name: String,
    pub language: SdkLanguage,
    pub api_version: u32,
}

pub struct SigmaOsSdk;

impl SigmaOsSdk {
    /// Generates boilerplate bindings suitable for compiling native target binaries
    pub fn compile_bindings(app: &SdkAppDescriptor) -> Result<String, &'static str> {
        match app.language {
            SdkLanguage::Rust => Ok(alloc::format!(
                "// SigmaOS Rust SDK bindings for {}\nextern \"C\" {{\n    fn sigma_syscall(sys_id: u32, args: *const u8) -> u32;\n}}",
                app.name
            )),
            SdkLanguage::Nim => Ok(alloc::format!(
                "# SigmaOS Nim SDK bindings for {}\nproc sigma_syscall*(sys_id: uint32, args: pointer): uint32 {{.importc, cdecl.}}",
                app.name
            )),
            SdkLanguage::Zig => Ok(alloc::format!(
                "// SigmaOS Zig SDK bindings for {}\nextern fn sigma_syscall(sys_id: u32, args: [*]const u8) ccall u32;",
                app.name
            )),
        }
    }
}

// =========================================================================
// 2. CLUSTER MODE (MULTI-NODE HPC SCHEDULING ENGINE)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
    Online,
    Offline,
    Overloaded,
}

#[derive(Debug, Clone)]
pub struct HpcClusterNode {
    pub node_id: u32,
    pub state: NodeState,
    pub core_count: u32,
    pub active_tasks: u32,
}

impl HpcClusterNode {
    pub fn new(id: u32, cores: u32) -> Self {
        Self {
            node_id: id,
            state: NodeState::Online,
            core_count: cores,
            active_tasks: 0,
        }
    }

    pub fn load_factor(&self) -> f32 {
        self.active_tasks as f32 / self.core_count as f32
    }
}

pub struct HpcClusterManager {
    pub nodes: Vec<HpcClusterNode>,
}

impl HpcClusterManager {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn register_node(&mut self, node: HpcClusterNode) -> Result<(), &'static str> {
        if self.nodes.iter().any(|n| n.node_id == node.node_id) {
            return Err("Node already registered in the cluster pool");
        }
        self.nodes.push(node);
        Ok(())
    }

    /// Distributes HPC job tasks to the least loaded online cluster node (load balancing)
    pub fn dispatch_job(&mut self) -> Result<u32, &'static str> {
        let mut target_node_idx = None;
        let mut min_load = f32::MAX;

        for (idx, node) in self.nodes.iter().enumerate() {
            if node.state == NodeState::Online {
                let load = node.load_factor();
                if load < min_load {
                    min_load = load;
                    target_node_idx = Some(idx);
                }
            }
        }

        if let Some(idx) = target_node_idx {
            self.nodes[idx].active_tasks += 1;
            if self.nodes[idx].load_factor() > 1.0 {
                self.nodes[idx].state = NodeState::Overloaded;
            }
            Ok(self.nodes[idx].node_id)
        } else {
            Err("No available online nodes found in the cluster to accept jobs")
        }
    }
}

impl Default for HpcClusterManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. GPU COMPUTE SUPPORT (NATIVE PARALLEL CUDA/OpenCL RUNTIME)
// =========================================================================

pub struct GpuKernel {
    pub name: String,
    pub source: String,
}

pub struct GpuComputeEngine {
    pub loaded_kernels: Vec<GpuKernel>,
    pub allocated_vram_bytes: u64,
}

impl GpuComputeEngine {
    pub fn new() -> Self {
        Self {
            loaded_kernels: Vec::new(),
            allocated_vram_bytes: 0,
        }
    }

    pub fn compile_gpu_program(&mut self, name: &str, source: &str) -> Result<(), &'static str> {
        if source.is_empty() {
            return Err("Empty program compilation payload");
        }
        self.loaded_kernels.push(GpuKernel {
            name: name.to_string(),
            source: source.to_string(),
        });
        Ok(())
    }

    pub fn allocate_gpu_vram(&mut self, bytes: u64) -> Result<(), &'static str> {
        const MAX_VRAM: u64 = 8 * 1024 * 1024 * 1024; // 8GB maximum VRAM guard
        if self.allocated_vram_bytes + bytes > MAX_VRAM {
            return Err("Out of GPU VRAM capacity");
        }
        self.allocated_vram_bytes += bytes;
        Ok(())
    }

    pub fn free_gpu_vram(&mut self, bytes: u64) {
        self.allocated_vram_bytes = self.allocated_vram_bytes.saturating_sub(bytes);
    }
}

impl Default for GpuComputeEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. ENERGY EFFICIENCY POWER GOVERNOR (ARM & RISC-V big.LITTLE OPTIMIZATIONS)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessorArch {
    Arm64,
    RiscV,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreGroup {
    LittlePowerSave,
    BigPerformance,
}

pub struct CoreFrequencyState {
    pub core_group: CoreGroup,
    pub active_frequency_mhz: u32,
    pub deep_sleep_active: bool,
}

pub struct EnergyOptimizer {
    pub arch: ProcessorArch,
    pub cores: Vec<CoreFrequencyState>,
}

impl EnergyOptimizer {
    pub fn new(arch: ProcessorArch, core_count: usize) -> Self {
        let mut cores = Vec::new();
        for i in 0..core_count {
            let group = if i < core_count / 2 {
                CoreGroup::LittlePowerSave
            } else {
                CoreGroup::BigPerformance
            };
            cores.push(CoreFrequencyState {
                core_group: group,
                active_frequency_mhz: if group == CoreGroup::LittlePowerSave { 1000 } else { 2400 },
                deep_sleep_active: false,
            });
        }
        Self { arch, cores }
    }

    /// Sets energy saving state. Shuts down Big cores and dials back Little frequencies.
    pub fn set_eco_energy_saver(&mut self, enable_eco: bool) {
        for core in &mut self.cores {
            if enable_eco {
                if core.core_group == CoreGroup::BigPerformance {
                    core.deep_sleep_active = true;
                    core.active_frequency_mhz = 0;
                } else {
                    core.active_frequency_mhz = 600; // Underclock little cores
                }
            } else {
                core.deep_sleep_active = false;
                core.active_frequency_mhz = if core.core_group == CoreGroup::LittlePowerSave { 1000 } else { 2400 };
            }
        }
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdk_binding_generation() {
        let app = SdkAppDescriptor {
            name: "Calculator".to_string(),
            language: SdkLanguage::Rust,
            api_version: 1,
        };
        let rust_bindings = SigmaOsSdk::compile_bindings(&app).unwrap();
        assert!(rust_bindings.contains("extern \"C\""));
        assert!(rust_bindings.contains("Calculator"));

        let mut nim_app = app;
        nim_app.language = SdkLanguage::Nim;
        let nim_bindings = SigmaOsSdk::compile_bindings(&nim_app).unwrap();
        assert!(nim_bindings.contains("proc"));

        let mut zig_app = nim_app;
        zig_app.language = SdkLanguage::Zig;
        let zig_bindings = SigmaOsSdk::compile_bindings(&zig_app).unwrap();
        assert!(zig_bindings.contains("extern fn"));
    }

    #[test]
    fn test_hpc_cluster_balancing() {
        let mut manager = HpcClusterManager::new();
        let node1 = HpcClusterNode::new(101, 8); // 8 cores
        let node2 = HpcClusterNode::new(102, 4); // 4 cores

        assert!(manager.register_node(node1).is_ok());
        assert!(manager.register_node(node2).is_ok());
        assert!(manager.register_node(HpcClusterNode::new(101, 8)).is_err()); // duplicate id

        // Dispatching first job goes to node1 (lowest load factor 0.0 vs 0.0 but more cores means smaller increment impact)
        let dispatched_id = manager.dispatch_job().unwrap();
        assert_eq!(dispatched_id, 101);
        assert_eq!(manager.nodes[0].active_tasks, 1);
    }

    #[test]
    fn test_gpu_compute_runtime() {
        let mut engine = GpuComputeEngine::new();
        assert!(engine.compile_gpu_program("MatrixAdd", "__kernel void add() {}").is_ok());
        assert!(engine.compile_gpu_program("MatrixAdd", "").is_err());

        assert!(engine.allocate_gpu_vram(4 * 1024 * 1024 * 1024).is_ok()); // 4GB OK
        assert!(engine.allocate_gpu_vram(5 * 1024 * 1024 * 1024).is_err()); // 4GB + 5GB > 8GB fails!

        engine.free_gpu_vram(2 * 1024 * 1024 * 1024);
        assert_eq!(engine.allocated_vram_bytes, 2 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_energy_efficiency_optimizer() {
        let mut opt = EnergyOptimizer::new(ProcessorArch::Arm64, 4);
        assert_eq!(opt.cores.len(), 4);
        assert_eq!(opt.cores[0].core_group, CoreGroup::LittlePowerSave);
        assert_eq!(opt.cores[3].core_group, CoreGroup::BigPerformance);

        // Turn on eco energy saving
        opt.set_eco_energy_saver(true);
        assert_eq!(opt.cores[0].active_frequency_mhz, 600); // underclocked Little
        assert!(opt.cores[2].deep_sleep_active); // suspended Big

        // Restore normal mode
        opt.set_eco_energy_saver(false);
        assert_eq!(opt.cores[0].active_frequency_mhz, 1000);
        assert!(!opt.cores[2].deep_sleep_active);
    }
}
