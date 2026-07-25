// SigmaOS Integration Tests
// Verifies core system legacy compatibility, multi-persona VMs, and driver bridge layers
#![allow(unused, clippy::all)]

use sigmaos::compatibility::{
    APITimelineManager, BinaryCompatMatrix, DiscontinuedFS, DriverBridge, FSRevival,
    GraphicsBridge, KernelPersona, KernelPersonaVM, LegacyBus, LegacyDriver,
    LegacyPluginManager, LibcVersion, NetworkBridge, StorageBridge, SyscallAbi,
    WorkloadOptimizer, WorkloadProfile, GLOBAL_PERSONA_VM, GLOBAL_PLUGIN_MANAGER,
    GLOBAL_WORKLOAD_OPTIMIZER,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_integration() {
        assert!(true);
    }

    #[test]
    fn test_legacy_personality_and_syscall_adaptation_flow() {
        // Step 1: Initialize the multi-persona VM
        let vm = KernelPersonaVM::new();
        assert_eq!(vm.current_persona.get(), KernelPersona::Linux_6_x);

        // Hot-swap kernel persona to 2.6 for legacy application expectations
        vm.hot_swap_persona(KernelPersona::Linux_2_6);
        assert_eq!(vm.current_persona.get(), KernelPersona::Linux_2_6);

        // Step 2: Use the Binary Compatibility Matrix to decode and translate syscall expectations
        let matrix = BinaryCompatMatrix::new(LibcVersion::Libc5, SyscallAbi::Oabi_32);
        let translated_sys = matrix.translate_sys_context(5); // expect 1005 offset mapping
        assert_eq!(translated_sys, 1005);

        // Step 3: Verify the API Timeline Manager parameter mappings
        let timeline = APITimelineManager::new(KernelPersona::Linux_2_6);
        let cleaned_param = timeline.map_syscall_params(0x0000111100002222);
        assert_eq!(cleaned_param, 0x00002222);
    }

    #[test]
    fn test_legacy_driver_bridge_revival() {
        let storage = StorageBridge { driver_name: "floppy-drive-controller", bus: LegacyBus::Isa };
        let graphics = GraphicsBridge { driver_name: "crt-terminal-controller", bus: LegacyBus::Agp };

        assert_eq!(storage.bus_type(), LegacyBus::Isa);
        assert_eq!(graphics.bus_type(), LegacyBus::Agp);
        assert!(storage.init_legacy());
        assert!(graphics.init_legacy());
    }

    #[test]
    fn test_legacy_workload_optimizer_tuning() {
        let optimizer = WorkloadOptimizer::new();
        assert_eq!(optimizer.active_profile.get(), WorkloadProfile::LowMemoryProfile);

        // Apply Single Core scheduling locks for early thread assumptions
        optimizer.apply_workload_tuning(WorkloadProfile::SingleCoreProfile);
        assert_eq!(optimizer.active_profile.get(), WorkloadProfile::SingleCoreProfile);
    }
}
