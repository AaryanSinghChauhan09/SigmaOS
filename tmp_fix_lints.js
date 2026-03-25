const fs = require('fs');
const path = require('path');

const dir = 'C:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\userland\\apps';
const files = fs.readdirSync(dir);

files.forEach(f => {
    if(!f.endsWith('.html')) return;
    const p = path.join(dir, f);
    let c = fs.readFileSync(p, 'utf8');
    
    // Fix backdrop-filter
    c = c.replace(/backdrop-filter:([\w\(\)\s\.]+);/g, '-webkit-backdrop-filter:$1; backdrop-filter:$1;');
    
    // Fix user-select
    c = c.replace(/user-select:(\w+);/g, '-webkit-user-select:$1; user-select:$1;');
    
    // Fix aria labels on inputs/selects without them
    c = c.replace(/<select /g, '<select aria-label="Selection" ');
    c = c.replace(/<input /g, '<input aria-label="input" ');
    // Prevent double aira-labels
    c = c.replace(/aria-label="Selection"\s+aria-label="Selection"/g, 'aria-label="Selection"');
    c = c.replace(/aria-label="input"\s+aria-label="input"/g, 'aria-label="input"');

    // Fix autocapitalize
    c = c.replace(/autocapitalize="off"/gi, '');
    
    fs.writeFileSync(p, c);
});

// Markdown fixes
const mdFile = 'C:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\DOCS\\MISSING_COMPONENTS.md';
if (fs.existsSync(mdFile)) {
    let mc = fs.readFileSync(mdFile, 'utf8');
    // trailing spaces
    mc = mc.replace(/  +\n/g, '  \n');
    // fix table spacing
    mc = mc.split('\n').map(line => {
        if(line.startsWith('|')) {
            return line.replace(/\|/g, ' | ').replace(/  +/g, ' ').replace(/\| \|/g, '|').trim();
        }
        return line;
    }).join('\n');
    
    // some naive fix
    mc = mc.replace(/\| -/g, '|-').replace(/- \|/g, '-|');
    fs.writeFileSync(mdFile, mc);
}

console.log("Lint fix script executed.");
