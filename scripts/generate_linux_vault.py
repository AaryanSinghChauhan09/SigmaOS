import json
import random

categories = ["Drivers", "Networking", "Storage", "Architecture", "Virtualization", "Legacy Shells", "Core Kernel", "Media"]
prefixes = ["linux-drv", "gnu-tool", "sys-lib", "vfs-plugin", "net-stack", "arch-shim", "x11-module", "crypto-algo"]

num_modules = 1000
packages = []

# Keep our core 32 packages manually defined first
core_packages = [
    {"id": "sv-legacy-x11", "name": "Legacy X11 Display Server", "description": "The obsolete monolithic Unix windowing system.", "category": "Graphics", "size_mb": 110, "status": "dormant"},
    {"id": "sv-systemd", "name": "Systemd Init Wrapper", "description": "Massive monolithic initialization suite.", "category": "Core", "size_mb": 45, "status": "dormant"},
    {"id": "sv-docker", "name": "Docker Engine", "description": "Legacy container management.", "category": "Virtualization", "size_mb": 190, "status": "dormant"},
    {"id": "sv-bash", "name": "GNU Bash Interpreter", "description": "Legacy text terminal.", "category": "Userland", "size_mb": 4, "status": "dormant"}
]

packages.extend(core_packages)

for i in range(num_modules):
    cat = random.choice(categories)
    pref = random.choice(prefixes)
    size = random.randint(1, 550)
    
    pkg = {
        "id": f"{pref}-{i:04d}",
        "name": f"Torvalds Legacy {cat} - Part {i}",
        "description": f"Obsolete monolithic C code for bridging {pref} sub-architectures.",
        "category": cat,
        "size_mb": size,
        "status": "dormant"
    }
    packages.append(pkg)

vault_data = {"packages": packages}

with open("web_ui/sigma_vault.json", "w") as f:
    json.dump(vault_data, f, indent=4)

print(f"[*] Generated {len(packages)} legacy Linux packages into the Sigma Vault.")
