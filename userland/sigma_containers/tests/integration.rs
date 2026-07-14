use sigma_containers::{ContainerConfig, SigmaContainer};
use sigma_containers::microvm::{MicroVMConfig, MicroVMEngine};

#[test]
fn test_container_creation_and_pull() {
    let mut system = SigmaContainer::new();
    
    let config = ContainerConfig {
        memory_limit_mb: 512,
        capabilities: vec!["CAP_NET_BIND".to_string()],
    };

    let container = system.create_container("ubuntu:latest", config).expect("Failed to create container");
    
    // Assert image resolution logic worked
    assert!(container.image_ref.starts_with("sha256:"));
    
    // Validate startup pipeline
    assert!(system.run_container(&container).is_ok());
}

#[test]
fn test_microvm_spawn() {
    let engine = MicroVMEngine::new();
    
    let config = MicroVMConfig {
        vcpu_count: 4,
        memory_mb: 2048,
        kernel_path: "/boot/sigma_kernel.bin".to_string(),
    };

    let vm_id = engine.spawn_vm(config).expect("Failed to spawn VM");
    assert_eq!(vm_id, 9999);
}

#[test]
fn test_microvm_invalid_config() {
    let engine = MicroVMEngine::new();
    
    let config = MicroVMConfig {
        vcpu_count: 0,
        memory_mb: 16,
        kernel_path: "/boot/sigma_kernel.bin".to_string(),
    };

    assert!(engine.spawn_vm(config).is_err());
}
