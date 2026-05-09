const fs = require('fs');
const path = require('path');

function findDuplicates(dir, fileMap = {}) {
    const files = fs.readdirSync(dir);
    for (const file of files) {
        const fullPath = path.join(dir, file);
        if (file === '.git' || file === 'node_modules') continue;
        const stats = fs.statSync(fullPath);
        if (stats.isDirectory()) {
            findDuplicates(fullPath, fileMap);
        } else {
            if (!fileMap[file]) fileMap[file] = [];
            fileMap[file].push(fullPath);
        }
    }
    return fileMap;
}

const map = findDuplicates('.');
for (const file in map) {
    if (map[file].length > 1) {
        console.log(`${file}:`);
        for (const p of map[file]) {
            console.log(`  ${p}`);
        }
    }
}
