import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

function walkDir(dir, callback) {
    if (dir.includes('.git') || dir.includes('node_modules') || dir.includes('wiki_repo')) {
        return;
    }
    let files;
    try {
        files = fs.readdirSync(dir);
    } catch (e) {
        return;
    }
    for (const file of files) {
        const filepath = path.join(dir, file);
        let stat;
        try {
            stat = fs.statSync(filepath);
        } catch (e) {
            continue;
        }
        if (stat.isDirectory()) {
            walkDir(filepath, callback);
        } else if (stat.isFile()) {
            callback(filepath);
        }
    }
}

function trackProblems() {
    console.log("Σ SigmaOS @current_problems Tracker [ACTIVE]");
    
    const problems = [];
    
    walkDir(".", (filepath) => {
        const basename = path.basename(filepath);
        if (basename === 'problem_tracker.js' || basename === 'CURRENT_PROBLEMS_MANIFEST.md' || basename === 'CONTRIBUTOR_ROADMAP.md') {
            return;
        }
        const ext = path.extname(filepath);
        if ([".cpp", ".hpp", ".h", ".js", ".cjs", ".md"].includes(ext)) {
            let content;
            try {
                content = fs.readFileSync(filepath, 'utf8');
            } catch (e) {
                return;
            }
            const lines = content.split('\n');
            lines.forEach((line, i) => {
                if (line.includes("@current_problems") || line.includes("FIXME")) {
                    const problem = `[${path.basename(filepath)}:${i + 1}] ${line.trim()}`;
                    problems.push(problem);
                    console.log(`[FOUND] ${problem}`);
                }
            });
        }
    });

    const reportPath = "CURRENT_PROBLEMS_MANIFEST.md";
    let reportContent = "# Σ SigmaOS Current Problems Manifest\n\n";
    if (problems.length === 0) {
        reportContent += "✅ **Status: ALL CLEAR. No industrial blockers detected.**\n";
    } else {
        reportContent += "⚠️ **Status: Blockers Detected. Resolution Required.**\n\n";
        problems.forEach(p => {
            reportContent += `- ${p}\n`;
        });
    }
    
    fs.writeFileSync(reportPath, reportContent);
    console.log(`[SYNC] Problems Manifest generated at ${reportPath}`);
}

trackProblems();
