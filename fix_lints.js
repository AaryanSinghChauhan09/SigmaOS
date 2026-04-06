const fs = require('fs');
const path = require('path');

function walk(dir) {
    let results = [];
    const list = fs.readdirSync(dir);
    list.forEach(function(file) {
        file = dir + '/' + file;
        const stat = fs.statSync(file);
        if (stat && stat.isDirectory()) {
            if (!file.includes('.git')) {
                results = results.concat(walk(file));
            }
        } else {
            if (file.endsWith('.md')) {
                results.push(file);
            }
        }
    });
    return results;
}

const files = walk('.');

files.forEach(file => {
    let lines = fs.readFileSync(file, 'utf8').split('\n');
    let outLines = [];
    
    // Fix Trailing Spaces and List Marker spacing
    lines.forEach((line) => {
        line = line.replace(/[ \t]+$/, ''); // trailing space
        line = line.replace(/^(\s*[-*])\s{2,}/, '$1 '); // list marker space
        outLines.push(line);
    });
    
    let finalLines = [];
    outLines.forEach((line, i) => {
        // Fix Headings format
        if (line.match(/^#{1,6}\s/)) {
            if (i > 0 && outLines[i-1].trim() !== '') {
                finalLines.push('');
            }
            finalLines.push(line);
            if (i + 1 < outLines.length && outLines[i+1].trim() !== '') {
                finalLines.push('');
            }
            return;
        }
        
        // Blank lines before lists
        let isList = !!line.match(/^\s*[-*]\s/);
        let wasList = i > 0 && !!outLines[i-1].match(/^\s*[-*]\s/);
        if (isList && !wasList && finalLines.length > 0 && finalLines[finalLines.length-1].trim() !== '') {
            finalLines.push('');
        }
        
        // Fix Table style MD060
        if (line.includes('|') && line.replace(/\s/g, '').includes('-|-')) {
            line = line.replace(/\|-/g, '| -').replace(/-\|/g, '- |').replace(/\|\s*-\s*\|/g, '| --- |');
            // Ensure no tightly packed pipe segments like |---|---|
            let parts = line.split('|');
            parts = parts.map(p => {
               if (p.trim().match(/^-+$/)) {
                   return ' --- ';
               }
               return p;
            });
            line = parts.join('|');
            // Remove multiple leading/trailing borders if duplicated
            if (line.startsWith('|  --- ')) line = '| --- ';
        }
        
        finalLines.push(line);
    });
    
    fs.writeFileSync(file, finalLines.join('\n'));
});
