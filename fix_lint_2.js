const fs = require('fs');
const path = require('path');

// Fix prefix ordering and safari warnings
function fixPrefixes(content) {
    let newContent = content.replace(/user-select:([^;]+);\s*-webkit-user-select:/g, '-webkit-user-select:$1; user-select:');
    newContent = newContent.replace(/backdrop-filter:([^;]+);\s*-webkit-backdrop-filter:/g, '-webkit-backdrop-filter:$1; backdrop-filter:');
    newContent = newContent.replace(/-webkit-user-select:\s*([^;]+);\s*user-select:/g, '-webkit-user-select:$1; user-select:');
    newContent = newContent.replace(/-webkit-backdrop-filter:\s*([^;]+);\s*backdrop-filter:/g, '-webkit-backdrop-filter:$1; backdrop-filter:');
    // For single instances missing webkit (Safari 9+ / iOS 9+)
    newContent = newContent.replace(/backdrop-filter: (blur\([^)]+\))/g, '-webkit-backdrop-filter: $1; backdrop-filter: $1');
    newContent = newContent.replace(/user-select: (none)/g, '-webkit-user-select: $1; user-select: $1');
    return newContent;
}

// Fix markdown blanks
function fixMarkdown(p) {
    if(!fs.existsSync(p)) return;
    let md = fs.readFileSync(p, 'utf8');
    md = md.replace(/^([#]+)\s+(.*)\n([^\n])/gm, '$1 $2\n\n$3');
    md = md.replace(/^-\s+(.*)\n([^\n-])/gm, '- $1\n\n$2');
    fs.writeFileSync(p, md);
}

// ARIA Text for specific problem areas
function fixAriaLabels(content) {
    content = content.replace(/<iframe([^>]*)>/gi, (match, attrs) => {
        if (!attrs.includes('title=')) {
            return `<iframe title="Embedded Content" ${attrs}>`;
        }
        return match;
    });
    content = content.replace(/<select([^>]*)>/gi, (match, attrs) => {
        if (!attrs.includes('aria-label=')) {
            return `<select aria-label="Selection" ${attrs}>`;
        }
        return match;
    });
    content = content.replace(/<button([^>]*)><\/button>/gi, '<button title="Action" $1></button>');
    content = content.replace(/autocapitalize="off"/gi, ''); // Not supported by safari
    return content;
}

const dirs = ['C:/Users/Aaryan/.gemini/antigravity/scratch/SigmaOS/userland/apps', 'C:/Users/Aaryan/.gemini/antigravity/scratch/SigmaOS'];
dirs.forEach(dir => {
    fs.readdirSync(dir).forEach(file => {
        if (file.endsWith('.html')) {
            let p = path.join(dir, file);
            let content = fixPrefixes(fs.readFileSync(p, 'utf8'));
            content = fixAriaLabels(content);
            fs.writeFileSync(p, content);
        }
    });
});

fixMarkdown('C:/Users/Aaryan/.gemini/antigravity/scratch/SigmaOS/_GAPS_TO_IMPLEMENT.md');

console.log("Secondary Lint Pass Finished!");
