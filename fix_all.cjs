const fs = require('fs');
const path = require('path');

const rootDir = "c:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS";
const cssPath = path.join(rootDir, 'zenith_desktop.css');

if (fs.existsSync(cssPath)) {
    let content = fs.readFileSync(cssPath, 'utf8');
    
    // Clean up messed up user-select
    content = content.replace(/-webkit--webkit.*?-webkit-user-select: none;/g, '    -webkit-user-select: none; user-select: none;');
    content = content.replace(/.*user-select: none.*user-select: none.*/g, '    -webkit-user-select: none; user-select: none;');
    
    // Clean up messed up backdrop-filter
    content = content.replace(/-webkit--webkit.*backdrop-filter:[^\n]*/g, '    -webkit-backdrop-filter: blur(var(--glass-blur)); backdrop-filter: blur(var(--glass-blur));');
    
    // Normalizing other backdrop-filter
    const lines = content.split('\n');
    for (let i = 0; i < lines.length; i++) {
        if (lines[i].includes('backdrop-filter:')) {
            if (!lines[i].includes('-webkit-backdrop-filter:')) {
                // If previous line has it, ensure order is correct
                if (i > 0 && lines[i-1].includes('-webkit-backdrop-filter:')) {
                    // good
                } else if (i < lines.length - 1 && lines[i+1].includes('-webkit-backdrop-filter:')) {
                    const temp = lines[i];
                    lines[i] = lines[i+1];
                    lines[i+1] = temp;
                } else {
                    const match = lines[i].match(/^(\s*)backdrop-filter:\s*(.*?);/);
                    if (match) {
                        lines[i] = `${match[1]}-webkit-backdrop-filter: ${match[2]};\n${lines[i]}`;
                    }
                }
            }
        }
    }
    fs.writeFileSync(cssPath, lines.join('\n'));
}

const filesToFix = [
    ["include/sigma_ui_toolkit.h", "sigma_kernel_types.h"],
    ["kernel/core/drivers/SovereignVideo.cpp", "SigmaOOP.hpp"],
    ["kernel/core/network/SovereignFirewall.cpp", "sigma_kernel_types.h"],
    ["kernel/core/SovereignAISched.cpp", "sigma_hal.h"],
    ["kernel/core/SovereignNUMA.cpp", "sigma_hal.h"],
    ["kernel/core/SovereignTelemetry.cpp", "sigma_kernel_types.h"],
    ["kernel/core/SovereignTelemetryUI.cpp", "sigma_hal.h"],
    ["kernel/core/SovereignThemeEngine.cpp", "sigma_hal.h"],
    ["kernel/core/SovereignThemeEngine.cpp", "sigma_time.h"],
    ["kernel/core/SovereignUIToolkit.cpp", "SigmaOOP.hpp"],
    ["tests/UniversalOSFormatTest.cpp", "sigma_kernel_types.h"],
    ["tools/sigma-pkg.cpp", "sigma_net.h"],
];

for (const [relPath, header] of filesToFix) {
    const p = path.join(rootDir, relPath.replace(/\//g, path.sep));
    if (fs.existsSync(p)) {
        const lines = fs.readFileSync(p, 'utf8').split('\n');
        const newLines = lines.filter(line => !(line.includes(header) && line.includes('#include')));
        fs.writeFileSync(p, newLines.join('\n'));
    }
}

// README fixes
const readmePath = path.join(rootDir, "README.md");
if (fs.existsSync(readmePath)) {
    let readme = fs.readFileSync(readmePath, 'utf8').split('\n');
    for (let i = 0; i < readme.length; i++) {
        if (readme[i].startsWith(' - ') || readme[i].startsWith('- ')) {
            readme[i] = readme[i].replace(/- /, '* ');
        }
        if (readme[i].startsWith('  - ')) {
            readme[i] = readme[i].replace(/  - /, '  * ');
        }
    }
    while (readme.length > 0 && readme[readme.length - 1].trim() === '') {
        readme.pop();
    }
    readme.push('');
    fs.writeFileSync(readmePath, readme.join('\n'));
}

// Competitive-Analysis.md fixes
const wikiPath = path.join(rootDir, "wiki_repo", "Competitive-Analysis.md");
if (fs.existsSync(wikiPath)) {
    let wiki = fs.readFileSync(wikiPath, 'utf8').split('\n');
    for (let i = 0; i < wiki.length; i++) {
        if (wiki[i].includes('|')) {
            wiki[i] = wiki[i].replace(/\|([^ ])/g, '| $1').replace(/([^ ])\|/g, '$1 |');
        }
    }
    fs.writeFileSync(wikiPath, wiki.join('\n'));
}

// Performance.md fixes
const perfPath = path.join(rootDir, "wiki_repo", "Performance.md");
if (fs.existsSync(perfPath)) {
    let perf = fs.readFileSync(perfPath, 'utf8');
    perf = perf.replace(/\*\*(.*?)\*\*\n/g, '### $1\n');
    fs.writeFileSync(perfPath, perf);
}

// Zenith HTML fixes
const zenithHtml = path.join(rootDir, "zenith.html");
if (fs.existsSync(zenithHtml)) {
    let content = fs.readFileSync(zenithHtml, 'utf8');
    // Remove all style="..." inline styles and put them in classes
    // Well, a bit complicated. I'll just remove style="..." and we'll add classes manually later if needed.
    // Actually, to just resolve the warning without breaking too much, I can extract them to a <style> block,
    // but the problem is it says "move styles to an external CSS file".
    // For now, I will just strip inline styles that cause warnings to pass linting.
    // wait, stripping inline styles breaks layout.
}

console.log('Done!');
