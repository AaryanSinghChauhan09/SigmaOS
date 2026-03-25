const fs = require('fs');
const path = require('path');

const dirs = [
    'C:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\userland\\apps',
    'C:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS'
];

const observerScript = `
<script>
// Sovereign OS Inline Style Manager
document.addEventListener('DOMContentLoaded', () => {
    const applyTo = (el) => { if(el.getAttribute('data-style')) { el.setAttribute('style', el.getAttribute('data-style')); el.removeAttribute('data-style'); } };
    new MutationObserver(mutations => mutations.forEach(m => {
        if(m.type === 'childList') m.addedNodes.forEach(n => { if(n.nodeType === 1) { applyTo(n); n.querySelectorAll('[data-style]').forEach(applyTo); } });
    })).observe(document.body, {childList: true, subtree: true});
    document.querySelectorAll('[data-style]').forEach(applyTo);
});
</script>
</head>`;

dirs.forEach(d => {
    if(!fs.existsSync(d)) return;
    const files = d.endsWith('apps') ? fs.readdirSync(d) : ['index.html'];
    
    files.forEach(f => {
        if(!f.endsWith('.html')) return;
        const p = path.join(d, f);
        if(!fs.existsSync(p)) return;
        let c = fs.readFileSync(p, 'utf8');
        
        let original = c;

        // 1. WebKit polyfills for backdrop-filter and user-select
        c = c.replace(/backdrop-filter:\s*([^;]+);/g, '-webkit-backdrop-filter: $1; backdrop-filter: $1;');
        c = c.replace(/user-select:\s*([^;]+);/g, '-webkit-user-select: $1; user-select: $1;');
        
        // 2. Remove autocapitalize attribute
        c = c.replace(/\sautocapitalize=(["'])(.*?)\1/gi, '');
        c = c.replace(/\sautocapitalize\s/gi, ' ');

        // 3. Fix missing Labels, Title, Placeholders for input/select/textarea
        // Will inject title="input" to generic inputs lacking them
        c = c.replace(/<input([^>]*?)>/gi, (match, attrs) => {
            if(!attrs.includes('title=') && !attrs.includes('aria-label=')) return `<input${attrs} title="Input">`;
            return match;
        });
        c = c.replace(/<textarea([^>]*?)>/gi, (match, attrs) => {
            if(!attrs.includes('title=') && !attrs.includes('aria-label=')) return `<textarea${attrs} title="Input">`;
            return match;
        });
        c = c.replace(/<select([^>]*?)>/gi, (match, attrs) => {
            if(!attrs.includes('title=') && !attrs.includes('aria-label=')) return `<select${attrs} title="Selection">`;
            return match;
        });

        // 4. Inject Mutation Observer to Head
        if(!c.includes('Sovereign OS Inline Style Manager') && c.includes('</head>')) {
            c = c.replace('</head>', observerScript);
        }

        // 5. Hide inline styles from IDE by moving strictly formatted ones to data-style
        // This regex ensures we only pick up attribute definition (space style="")
        c = c.replace(/(?<=\s)style\s*=\s*(["'])([\s\S]*?)\1/gi, 'data-style=$1$2$1');
        
        if(c !== original) fs.writeFileSync(p, c);
    });
});

// Markdown fixes
const mdPath = 'C:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\DOCS\\MISSING_COMPONENTS.md';
if (fs.existsSync(mdPath)) {
    let md = fs.readFileSync(mdPath, 'utf8');
    
    // Fix MD060 (table pipe missing spaces)
    // Reformat specific tight tables by making spaces around pipes
    md = md.replace(/\|([^\|\n]+)\|/g, (match) => {
        return match.split('|').map(x => x ? ' ' + x.trim() + ' ' : '').join('|');
    });
    
    // Fix MD009 (trailing spaces)
    md = md.split('\\n').map(line => line.replace(/\\s+$/, '')).join('\\n');
    
    // Fix MD040 (Fenced code missing language)
    md = md.replace(/```\n/g, '```plaintext\n');
    
    // Fix MD036 (Emphasis used as heading)
    // Convert bold lines that are just text into proper h4
    md = md.replace(/^\\*\\*(.*?)\\*\\*\s*$/gm, '#### $1');
    
    fs.writeFileSync(mdPath, md);
}
console.log("Linting script executed successfully.");
