const fs = require('fs');

const categories = ["Drivers", "Networking", "Storage", "Architecture", "Virtualization", "Legacy Shells", "Core Kernel", "Media"];
const prefixes = ["linux-drv", "gnu-tool", "sys-lib", "vfs-plugin", "net-stack", "arch-shim", "x11-module", "crypto-algo"];

const num_modules = 1000;
let packages = [];

// Keep our core 32 packages manually defined first
const core_packages = [
    { "id": "sv-legacy-x11", "name": "Legacy X11 Display Server", "description": "The obsolete monolithic Unix windowing system.", "category": "Graphics", "size_mb": 110, "status": "dormant" },
    { "id": "sv-systemd", "name": "Systemd Init Wrapper", "description": "Massive monolithic initialization suite.", "category": "Core", "size_mb": 45, "status": "dormant" },
    { "id": "sv-docker", "name": "Docker Engine", "description": "Legacy container management.", "category": "Virtualization", "size_mb": 190, "status": "dormant" },
    { "id": "sv-bash", "name": "GNU Bash Interpreter", "description": "Legacy text terminal.", "category": "Userland", "size_mb": 4, "status": "dormant" }
];

packages.push(...core_packages);

for (let i = 0; i < num_modules; i++) {
    const cat = categories[Math.floor(Math.random() * categories.length)];
    const pref = prefixes[Math.floor(Math.random() * prefixes.length)];
    const size = Math.floor(Math.random() * 550) + 1;

    packages.push({
        id: `${pref}-${i.toString().padStart(4, '0')}`,
        name: `Torvalds Legacy ${cat} - Part ${i}`,
        description: `Obsolete monolithic C code for bridging ${pref} sub-architectures.`,
        category: cat,
        size_mb: size,
        status: "dormant"
    });
}

fs.writeFileSync("web_ui/sigma_vault.json", JSON.stringify({ packages }, null, 4));
console.log(`[*] Generated ${packages.length} legacy Linux packages into the Sigma Vault.`);
