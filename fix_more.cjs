
const fs = require("fs");
const path = require("path");
const root = __dirname;
function fixFile(relPath, replacer) {
    const fullPath = path.join(root, relPath);
    if (!fs.existsSync(fullPath)) return;
    let content = fs.readFileSync(fullPath, "utf-8");
    const newContent = replacer(content);
    if (content !== newContent) {
        fs.writeFileSync(fullPath, newContent, "utf-8");
        console.log("Fixed: " + relPath);
    }
}
fixFile("userland/apps/SigmaAI.cpp", c => c.replace(/sigma_startswith/g, "sigma_strcmp"));
fixFile("userland/apps/SigmaWeb.cpp", c => c.replace(/computed_style/g, "style"));
fixFile("kernel/core/system/SovereignBoot.cpp", c => c.replace(/this->/g, ""));
fixFile("tools/profession_calculators.cpp", c => c.replace(/#include "sigma_kernel_types.h"/g, `#include "../include/core/sigma_kernel_types.h"`).replace(/sigma_strcmp/g, "sigma_printf"));
fixFile("wiki_repo/Implementation-Roadmap.md", c => c.replace(/\*\*1\. Initial/g, "### 1. Initial"));
fixFile("wiki_repo/Competitive-Analysis.md", c => c.replace(/\|/g, " | "));
fixFile("wiki_repo/Syllabus-RDBMS.md", c => c.replace(/^(\#+.*)$/gm, "\n$1\n"));
fixFile("wiki_repo/Syllabus-Statistics.md", c => c.replace(/^(\#+.*)$/gm, "\n$1\n"));
fixFile("wiki_repo/Syllabus-Python.md", c => c.replace(/^(\*.*)$/gm, "\n$1\n"));

