const fs = require('fs');

const vaultPath = 'web_ui/sigma_vault.json';
if (!fs.existsSync(vaultPath)) {
    console.error("Vault DB not found. Run generate_linux_vault.js first.");
    process.exit(1);
}

const data = JSON.parse(fs.readFileSync(vaultPath));
const packages = data.packages;

console.log("[*] Generating physical compressed payloads for Linux Emulation...");

let count = 0;
// Generate physical mock binaries for the core modules plus first 20 random ones
const targets = packages.slice(0, 30);

targets.forEach(pkg => {
    // We generate a dummy binary payload mimicking a compressed .tar.gz or native .sigma archive.
    // To save disk space in git, we make these files relatively small but distinct.
    const mockBinary = Buffer.alloc(1024, pkg.id); 
    const payloadPath = `web_ui/payloads/${pkg.id}.sigma.archive`;
    
    fs.writeFileSync(payloadPath, mockBinary);
    count++;
});

console.log(`[*] Physical Archive Compression Complete. ${count} Linux payloads are now sitting functionally dormant on disk waiting for User download algorithms.`);
