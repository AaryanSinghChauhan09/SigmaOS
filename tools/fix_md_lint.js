const fs = require('fs');
const path = require('path');

const dir = path.join(__dirname, '../wiki_repo');
if (!fs.existsSync(dir)) {
    console.error("Directory not found:", dir);
    process.exit(1);
}

fs.readdirSync(dir).forEach(file => {
    if (!file.endsWith('.md')) return;
    const filepath = path.join(dir, file);
    let content = fs.readFileSync(filepath, 'utf8');
    
    // Normalize newlines to \n
    content = content.replace(/\r\n/g, '\n');
    let lines = content.split('\n');
    
    // 1. Fix MD022 (Headings should be surrounded by blank lines)
    let newLines = [];
    for (let i = 0; i < lines.length; i++) {
        let line = lines[i];
        if (line.match(/^#+\s/)) {
            // Heading detected.
            // Ensure there is a blank line before it if needed
            if (newLines.length > 0 && newLines[newLines.length - 1].trim() !== '') {
                newLines.push('');
            }
            newLines.push(line);
            // Ensure there is a blank line after it if needed
            if (i < lines.length - 1 && lines[i+1].trim() !== '' && !lines[i+1].match(/^#+\s/)) {
                newLines.push('');
            }
        } else {
            newLines.push(line);
        }
    }
    lines = newLines;

    // 2. Fix MD032 (Lists should be surrounded by blank lines)
    const isListLine = (line) => {
        return !!line.match(/^\s*([*\-+]|\d+\.)\s/);
    };
    
    newLines = [];
    for (let i = 0; i < lines.length; i++) {
        let line = lines[i];
        if (isListLine(line)) {
            if (newLines.length > 0) {
                let prevLine = newLines[newLines.length - 1];
                if (prevLine.trim() !== '' && !isListLine(prevLine)) {
                    newLines.push('');
                }
            }
            newLines.push(line);
            
            if (i < lines.length - 1 && lines[i+1].trim() !== '' && !isListLine(lines[i+1])) {
                newLines.push('');
            }
        } else {
            newLines.push(line);
        }
    }
    lines = newLines;

    // Join and trim trailing/leading spaces on empty lines, clean up consecutive empty lines
    let finalContent = lines.join('\n');
    finalContent = finalContent.replace(/\n{3,}/g, '\n\n');
    
    // Ensure single trailing newline (MD047)
    finalContent = finalContent.trimEnd() + '\n';
    
    if (finalContent !== content) {
        fs.writeFileSync(filepath, finalContent, 'utf8');
        console.log("Fixed Markdown Lint in:", file);
    }
});
