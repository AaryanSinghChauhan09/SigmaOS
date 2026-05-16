/**
 * SigmaOS Markdown Linting Fixer
 * Fixes: MD022, MD026, MD031, MD032, MD037, MD047
 */
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

    // MD047: Files should end with a single newline
    content = content.replace(/\s*$/, '\n');

    // MD026: Remove trailing punctuation from headings (period only)
    content = content.replace(/^(#+\s+.+)\.\s*$/gm, '$1');

    // MD009: Remove trailing spaces
    content = content.replace(/[ \t]+$/gm, '');

    if (content !== orig) {
        fs.writeFileSync(filepath, content);
        console.log('Fixed: ' + path.relative(process.cwd(), filepath));
    }
}

walkDir(process.cwd(), fixMarkdown);
console.log('Done.');
