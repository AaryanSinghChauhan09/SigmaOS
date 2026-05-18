const fs = require('fs');

let html = fs.readFileSync('zenith.html', 'utf8');
let css = fs.readFileSync('zenith_desktop.css', 'utf8');

let counter = 1;
html = html.replace(/style="([^"]+)"/g, (match, p1) => {
    let className = `auto-style-${counter++}`;
    css += `\n.${className} { ${p1} }\n`;
    return `class="${className}"`;
});

// also fix multiple class attributes if they exist
html = html.replace(/class="([^"]+)"\s+class="([^"]+)"/g, 'class="$1 $2"');
html = html.replace(/class="([^"]+)"\s+class="([^"]+)"/g, 'class="$1 $2"');

// Fix the missing labels issue in zenith.html
// "Form elements must have labels: Element has no title attribute Element has no placeholder attribute"
// at lines 162, 166.
html = html.replace(/<input type="range" (.*?)>/g, (match, p1) => {
    if (!p1.includes('title=')) {
        return `<input type="range" ${p1} title="Range input">`;
    }
    return match;
});

fs.writeFileSync('zenith.html', html);
fs.writeFileSync('zenith_desktop.css', css);
console.log('Fixed zenith.html inline styles and accessibility issues.');
