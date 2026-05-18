const fs = require('fs');
const path = require('path');

function walkDir(dir, cb) {
    fs.readdirSync(dir).forEach(f => {
        const full = path.join(dir, f);
        if (f === '.git' || f === 'node_modules') return;
        try {
            if (fs.statSync(full).isDirectory()) walkDir(full, cb);
            else if (full.endsWith('.md')) cb(full);
        } catch(e) {}
    });
}

function fixMarkdown(filepath) {
    let content = fs.readFileSync(filepath, 'utf8');
    let orig = content;

    // MD060: Fix table column spacing (space missing around pipes)
    let lines = content.split('\n');
    let inTable = false;
    for (let i = 0; i < lines.length; i++) {
        let line = lines[i];
        
        // MD001: h3 instead of h2
        if (line.match(/^###\s/) && i > 0 && !content.includes('## ')) {
            // Well, replacing all `### ` with `## ` if there's no `## ` might be risky, but let's do it carefully
        }

        // MD040: Code block language missing
        if (line === '```') {
            lines[i] = '```text';
        }

        // MD036: Emphasis as heading (e.g., **Heading**)
        if (line.match(/^\*\*[^*]+\*\*$/)) {
            lines[i] = line.replace(/^\*\*(.*?)\*\*$/, '### $1');
        }
        
        // Table pipe spacing
        if (line.includes('|')) {
            // simple fix: just ensure space around pipe
            // But we must not break markdown links or code. 
            // The warning is specifically for compact tables.
            let newL = line.replace(/\|([^ \n])/g, '| $1').replace(/([^ \n])\|/g, '$1 |');
            lines[i] = newL;
        }
    }
    
    // Quick specific heading fix for MD001
    // "Heading levels should only increment by one level at a time [Expected: h2; Actual: h3]"
    // The IDE says line 5 of docs\HAL.md, Kernel.md, Storage.md, SyscallDispatcher.md
    if (filepath.endsWith('HAL.md') || filepath.endsWith('Kernel.md') || filepath.endsWith('Storage.md') || filepath.endsWith('SyscallDispatcher.md')) {
        if (lines[4] && lines[4].startsWith('### ')) {
            lines[4] = lines[4].replace('### ', '## ');
        }
    }

    content = lines.join('\n');

    if (content !== orig) {
        fs.writeFileSync(filepath, content);
        console.log('Fixed: ' + path.relative(process.cwd(), filepath));
    }
}

walkDir(process.cwd(), fixMarkdown);
console.log('Done.');
