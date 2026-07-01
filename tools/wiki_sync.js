import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const docsDir = path.join(__dirname, '..', 'docs');
const wikiDir = path.join(__dirname, '..', 'wiki_repo');

console.log("SIGMAOS Node.js Wiki Sync [RUNNING]");

if (!fs.existsSync(wikiDir)) {
    fs.mkdirSync(wikiDir, { recursive: true });
}

if (fs.existsSync(docsDir)) {
    const files = fs.readdirSync(docsDir);
    files.forEach(file => {
        if (file.endsWith('.md')) {
            console.log(`[SYNC] Migrating ${file} -> wiki_repo/`);
            fs.copyFileSync(path.join(docsDir, file), path.join(wikiDir, file));
        }
    });
}

console.log("[SYNC] Wiki Repositories Synchronized. Parity ACHIEVED.");
