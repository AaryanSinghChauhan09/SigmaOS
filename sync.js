import { spawnSync } from 'child_process';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

console.log("Staging final files...");
spawnSync('git', ['add', 'zenith.html', 'tools/sigma_vr_studio.cpp'], { stdio: 'inherit' });

console.log("Committing changes...");
spawnSync('git', ['commit', '-m', "fix: Final HTML class merge and VR studio typo"], { stdio: 'inherit' });

console.log("Running branch sync engine...");
spawnSync('node', [path.join(__dirname, 'tools', 'sync_all_branches.js')], { stdio: 'inherit' });

console.log("Synced!");
