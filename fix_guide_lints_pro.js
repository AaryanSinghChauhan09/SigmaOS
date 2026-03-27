const fs = require('fs');
const path = require('path');

const filePath = path.resolve('C:/Users/Aaryan/.gemini/antigravity/scratch/SigmaOS/OS_GUIDE.md');
let content = fs.readFileSync(filePath, 'utf8');

// Normalize newlines
content = content.replace(/\r\n/g, '\n');

// MD012: No multiple blank lines
while (content.includes('\n\n\n')) {
    content = content.replace(/\n\n\n/g, '\n\n');
}

// MD022: Blanks around headings
content = content.replace(/^#+(.*)$/gm, (match) => {
    return '\n\n' + match.trim() + '\n\n';
});

// MD032: Blanks around lists
content = content.replace(/([^\n])\n( *[\*\-\d]+\. )/g, '$1\n\n$2');
content = content.replace(/(\n *[\*\-\d]+\. [^\n]+)\n([^\n\* \-\d])/g, '$1\n\n$2');

// Final pass on blanks
while (content.includes('\n\n\n')) {
    content = content.replace(/\n\n\n/g, '\n\n');
}

// Ensure only one H1 at top
let h1Found = false;
const lines = content.split('\n');
const newLines = lines.map(line => {
    if (line.startsWith('# ')) {
        if (!h1Found) {
            h1Found = true;
            return line;
        } else {
            return '## ' + line.substring(2);
        }
    }
    return line;
});

fs.writeFileSync(filePath, newLines.join('\n').trim() + '\n');
console.log('Fixed bulk lints in OS_GUIDE.md (Surgical Pass)');
