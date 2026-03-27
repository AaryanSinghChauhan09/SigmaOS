const fs = require('fs');
const path = require('path');

const filePath = path.resolve('C:/Users/Aaryan/.gemini/antigravity/scratch/SigmaOS/OS_GUIDE.md');
let content = fs.readFileSync(filePath, 'utf8');

// MD012: No multiple blank lines
content = content.replace(/\n{3,}/g, '\n\n');

// MD022: Blanks around headings
content = content.replace(/^#+(.*)$/gm, (match) => {
    return '\n' + match + '\n';
});
// Clean up double blanks created by the above
content = content.replace(/\n{3,}/g, '\n\n');

// MD032: Blanks around lists
content = content.replace(/([^\n])\n( *[\*\-\d]+\. )/g, '$1\n\n$2');
content = content.replace(/(\n *[\*\-\d]+\. [^\n]+)\n([^\n\* \-\d])/g, '$1\n\n$2');

// Clean up triple blanks again
content = content.replace(/\n{3,}/g, '\n\n');

// Ensure only one H1 (MD025) - Convert others to H2
let h1Count = 0;
content = content.replace(/^# (.*)$/gm, (match) => {
    h1Count++;
    return h1Count > 1 ? '## ' + match.substring(2) : match;
});

fs.writeFileSync(filePath, content);
console.log('Fixed bulk lints in OS_GUIDE.md');
