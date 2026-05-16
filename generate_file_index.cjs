const fs = require('fs');
const path = require('path');

function getDirectoryTree(dir, prefix = '') {
    let output = '';
    let files;
    try {
        files = fs.readdirSync(dir);
    } catch (e) {
        return '';
    }

    files = files.filter(f => f !== '.git' && f !== 'node_modules' && f !== 'build' && f !== 'wiki_repo');
    files.sort((a, b) => {
        const aIsDir = fs.statSync(path.join(dir, a)).isDirectory();
        const bIsDir = fs.statSync(path.join(dir, b)).isDirectory();
        if (aIsDir && !bIsDir) return -1;
        if (!aIsDir && bIsDir) return 1;
        return a.localeCompare(b);
    });

    for (let i = 0; i < files.length; i++) {
        const file = files[i];
        const isLast = i === files.length - 1;
        const fullPath = path.join(dir, file);
        const stats = fs.statSync(fullPath);

        const pointer = isLast ? '└── ' : '├── ';
        output += `${prefix}${pointer}${file}\n`;

        if (stats.isDirectory()) {
            const nextPrefix = prefix + (isLast ? '    ' : '│   ');
            output += getDirectoryTree(fullPath, nextPrefix);
        }
    }
    return output;
}

const rootDir = process.cwd();
const tree = getDirectoryTree(rootDir);

const markdown = `# SigmaOS File Index\n\nThis document provides a comprehensive index of all files within the SigmaOS repository structure.\n\n\`\`\`text\nSigmaOS/\n${tree}\n\`\`\`\n`;

fs.writeFileSync(path.join(rootDir, 'wiki_repo', 'File-Index.md'), markdown);
console.log("File index generated at wiki_repo/File-Index.md");
