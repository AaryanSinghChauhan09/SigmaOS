const fs = require('fs');
const path = require('path');

const filePath = path.join("c:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS", "wiki_repo", "GAP_ANALYSIS_INDUSTRIAL_COMPETITORS.md");

if (fs.existsSync(filePath)) {
    let content = fs.readFileSync(filePath, "utf-8");
    
    // Use regex to insert newlines
    content = content.replace(/\*\*Observation:\*\*/g, "\n\n**Observation:** ");
    content = content.replace(/\*\*Implication:\*\*/g, "\n\n**Implication:** ");
    
    // Fix the markdown table
    content = content.replace(/ \| Feature \|/g, "\n\n| Feature |");
    content = content.replace(/ \| \| :---/g, "\n| :---");
    content = content.replace(/ \| \| \*\*Primary Goal\*\*/g, "\n| **Primary Goal**");
    content = content.replace(/ \| \| \*\*Kernel Type\*\*/g, "\n| **Kernel Type**");
    content = content.replace(/ \| \| \*\*Portability\*\*/g, "\n| **Portability**");
    content = content.replace(/ \| \| \*\*Scaling\*\*/g, "\n| **Scaling**");
    
    // Some lines might end up with ` | \n\n**Implication:**` which is what we want for the end of the table
    content = content.replace(/ \| \n\n\*\*Implication:\*\*/g, " |\n\n**Implication:**");

    // Clean up excessive newlines
    content = content.replace(/\n{3,}/g, "\n\n");
    
    fs.writeFileSync(filePath, content, "utf-8");
    console.log("Fixed GAP_ANALYSIS_INDUSTRIAL_COMPETITORS.md");
}
