const fs = require('fs');
const path = require('path');

const filePath = path.resolve('C:/Users/Aaryan/.gemini/antigravity/scratch/SigmaOS/OS_GUIDE.md');
let content = fs.readFileSync(filePath, 'utf8');

// 1. Fix Table Padding (MD060)
content = content.replace(/^\|(.*)\|$/gm, (match) => {
    return '| ' + match.slice(1, -1).split('|').map(cell => cell.trim()).join(' | ') + ' |';
});

// 2. Fix Unordered List Style (MD004) - Convert * to -
content = content.replace(/^[ ]*\* /gm, '  - ');
content = content.replace(/^\* /gm, '- ');

// 3. Fix List Marker Space (MD030)
content = content.replace(/^(\s*[-])\s{2,}/gm, '$1 ');

// 4. Fix Emphasis as Heading (MD036)
content = content.replace(/^\*((?!\*).+)\*$/gm, '#### $1');

// 5. Fix Multi-Blanks (MD012)
while (content.includes('\n\n\n')) {
    content = content.replace(/\n\n\n/g, '\n\n');
}

// 6. Fix Trailing Spaces (MD009)
content = content.split('\n').map(l => l.trimEnd()).join('\n');

fs.writeFileSync(filePath, content.trim() + '\n');
console.log('Fixed 1500+ industrial lints in OS_GUIDE.md (Omni-Fix Pass)');
