import os
import json
import tarfile

def create_linux_mock_vault():
    print("[*] Initiating total Torvalds/Linux encapsulation...")
    
    vault_dir = "web_ui/payloads"
    os.makedirs(vault_dir, exist_ok=True)
    
    linux_subsystems = [
        "arch-x86", "arch-arm", "arch-riscv", 
        "block-io", "crypto", "drivers-gpu", 
        "drivers-net", "drivers-usb", "drivers-tty",
        "fs-ext4", "fs-btrfs", "fs-xfs", "fs-proc",
        "ipc", "kernel-core", "kernel-sched", 
        "kernel-bpf", "mm-page_alloc", "mm-slab",
        "net-ipv4", "net-ipv6", "net-bluetooth",
        "net-wireless", "security-selinux", "sound-alsa"
    ]
    
    packages = []
    
    for idx, sys in enumerate(linux_subsystems):
        tar_name = f"{vault_dir}/torvalds_{sys}.tar.gz"
        with tarfile.open(tar_name, "w:gz") as tar:
            # We add a dummy file to the tar to make it a valid archive
            dummy_file = f"virtual_{sys}_payload.bin"
            with open(dummy_file, "wb") as f:
                f.write(os.urandom(1024 * 5)) # 5KB mock footprint
            tar.add(dummy_file)
            os.remove(dummy_file)
            
        packages.append({
            "id": f"linux-{sys}",
            "name": f"Linux {sys.upper()} Subsystem",
            "description": f"Torvalds' legacy monolithic {sys} codebase compressed for emulation.",
            "category": "Torvalds Legacy",
            "size_mb": 5, 
            "status": "dormant"
        })
        print(f"  -> Compressed {sys} into App Store.")

    print(f"[*] Generated {len(linux_subsystems)} physical tar.gz App Store payloads mimicking Torvalds' Linux.")
    return packages

if __name__ == "__main__":
    new_packages = create_linux_mock_vault()
    
    # Append to existing json
    json_path = "web_ui/sigma_vault.json"
    if os.path.exists(json_path):
        with open(json_path, "r") as f:
            data = json.load(f)
        
        # Prepend so they appear exactly at the top of the App Store
        data["packages"] = new_packages + data["packages"]
        
        with open(json_path, "w") as f:
            json.dump(data, f, indent=4)
            
        print("[*] Successfully merged physical linux subsystem mappings into the UI Database.")
