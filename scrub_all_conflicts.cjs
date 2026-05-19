const fs = require('fs');
const path = require('path');

function scrubDirectory(dir) {
    const files = fs.readdirSync(dir);
    for (const file of files) {
        const fullPath = path.join(dir, file);
        if (fs.statSync(fullPath).isDirectory()) {
            if (file !== 'node_modules' && file !== '.git' && file !== '.system_generated') {
                scrubDirectory(fullPath);
            }
        } else {
            const ext = path.extname(file);
            if (['.c', '.cpp', '.h', '.hpp', '.md', '.asm'].includes(ext)) {
                let content = fs.readFileSync(fullPath, 'utf8');
                if (content.includes('<<<<<<<') || content.includes('>>>>>>>')) {
                    console.log(`Scrubbing merge conflict in: ${fullPath}`);
                    const lines = content.split(/\r?\n/);
                    const newLines = [];
                    let inTheirs = false;
                    for (const line of lines) {
                        if (line.startsWith('<<<<<<<')) {
                            continue;
                        }
                        if (line.startsWith('=======')) {
                            inTheirs = true;
                            continue;
                        }
                        if (line.startsWith('>>>>>>>')) {
                            inTheirs = false;
                            continue;
                        }
                        if (!inTheirs) {
                            newLines.push(line);
                        }
                    }
                    fs.writeFileSync(fullPath, newLines.join('\n'), 'utf8');
                }
            }
        }
    }
}

scrubDirectory(__dirname);
console.log('All git merge conflicts successfully scrubbed.');
