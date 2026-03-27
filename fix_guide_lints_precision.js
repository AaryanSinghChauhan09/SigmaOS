const fs = require('fs');
const path = require('path');

const filePath = path.resolve('C:/Users/Aaryan/.gemini/antigravity/scratch/SigmaOS/OS_GUIDE.md');
let content = fs.readFileSync(filePath, 'utf8');

// 1. Fix MD032: Blanks around lists
// Ensure there is a blank line before any line starting with '-' or '*' that isn't already preceded by one
content = content.replace(/([^\n])\n(\s*[-*] )/g, '$1\n\n$2');
// Ensure there is a blank line after the list ends
content = content.replace(/(\s*[-*] .*)\n([^\n\s-*])/g, '$1\n\n$2');

// 2. Fix MD007: Unordered list indentation
// Convert 4-space indentation to 2-space indentation for lists
content = content.replace(/^    ([-*]) /gm, '  $1 ');
content = content.replace(/^        ([-*]) /gm, '    $1 ');

// 3. Fix MD012: Multiple blanks
while (content.includes('\n\n\n')) {
    content = content.replace(/\n\n\n/g, '\n\n');
}

fs.writeFileSync(filePath, content.trim() + '\n');
console.log('Fixed Indentation and Spacing lints in OS_GUIDE.md (Precision Pass)');
