const fs = require('fs');
const path = require('path');
const dirs = ['C:/Users/SOVEREIGN_USER/.gemini/antigravity/scratch/SigmaOS/userland/apps', 'C:/Users/SOVEREIGN_USER/.gemini/antigravity/scratch/SigmaOS'];

dirs.forEach(dir => {
    fs.readdirSync(dir).forEach(file => {
        if (file.endsWith('.html')) {
            let p = path.join(dir, file);
            let content = fs.readFileSync(p, 'utf8');
            
            // Add missing viewport
            if (!content.includes('name="viewport"')) {
                content = content.replace('<head>', '<head>\n<meta name="viewport" content="width=device-width, initial-scale=1.0">');
            } else if (file === 'sigma_mobile.html' || file === 'sigma_jump.html') {
                // Fix the maximum-scale and user-scalable errors
                content = content.replace('maximum-scale=1.0, user-scalable=no', 'viewport-fit=cover');
            }
            
            // Fix input aria labels where mentioned
            content = content.replace(/<input([^>]*)>/g, (match, attrs) => {
                if (!attrs.includes('title') && !attrs.includes('aria-label')) {
                    if (attrs.includes('placeholder=')) {
                        return `<input aria-label="input" ${attrs}>`;
                    }
                }
                return match;
            });
            
            fs.writeFileSync(p, content);
        }
    });
});
console.log("Viewport & Accessibility injection complete!");
