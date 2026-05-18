const fs = require('fs');
const path = require('path');

const html_path = "c:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\zenith.html";
if (fs.existsSync(html_path)) {
    let content = fs.readFileSync(html_path, "utf-8");

    content = content.replace(/<input[^>]*>/g, (match) => {
        let text = match;
        if (!text.includes("title=")) text = text.replace(">", ' title="input">');
        if (!text.includes("placeholder=")) text = text.replace(">", ' placeholder="input">');
        return text;
    });
    content = content.replace(/<select[^>]*>/g, (match) => {
        let text = match;
        if (!text.includes("title=")) text = text.replace(">", ' title="input">');
        return text;
    });
    content = content.replace(/<textarea[^>]*>/g, (match) => {
        let text = match;
        if (!text.includes("title=")) text = text.replace(">", ' title="input">');
        if (!text.includes("placeholder=")) text = text.replace(">", ' placeholder="input">');
        return text;
    });

    fs.writeFileSync(html_path, content, "utf-8");
}

const problem_files = {
    "c:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\kernel\\core\\SovereignAllocator.cpp": ["sigma_hal.h"],
    "c:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\kernel\\core\\drivers\\SovereignVideo.cpp": ["SigmaOOP.hpp", "sigma_types.h"],
    "c:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\kernel\\core\\scheduler.cpp": ["sigma_hal.h", "SovereignLibC.h"],
    "c:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\kernel\\core\\SovereignCgroup.cpp": ["sigma_hal.h"],
    "c:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\kernel\\core\\SovereignContainer.cpp": ["sigma_hal.h"],
    "c:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\kernel\\core\\SovereignLBU.cpp": ["sigma_hal.h"],
    "c:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\kernel\\core\\SovereignOverlayFS.cpp": ["sigma_hal.h"],
    "c:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\kernel\\core\\SovereignSyscall.cpp": ["SovereignLibC.h"],
    "c:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\kernel\\core\\SovereignTuner.cpp": ["sigma_kernel_types.h", "sigma_hal.h", "SovereignLibC.h"],
    "c:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\kernel\\core\\syscall\\dispatcher.h": ["syscalls.h"],
    "c:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\kernel\\scheduler\\SovereignScheduler.cpp": ["SovereignLibC.h", "sigma_hal.h"],
    "c:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\tests\\kselftest\\kselftest_sigma.h": ["string.h"],
    "c:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\userland\\apps\\SigmaAIIntegration.cpp": ["SovereignLibC.h"],
    "c:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\include\\sigma_boot.h": ["sigma_kernel_types.h"],
};

for (const [filepath, headers] of Object.entries(problem_files)) {
    if (fs.existsSync(filepath)) {
        const lines = fs.readFileSync(filepath, "utf-8").split("\n");
        const newLines = lines.filter(line => {
            if (!line.includes("#include")) return true;
            return !headers.some(h => line.includes(h));
        });
        fs.writeFileSync(filepath, newLines.join("\n"), "utf-8");
    }
}

const profPath = "c:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\tools\\profession_calculators.cpp";
if (fs.existsSync(profPath)) {
    const lines = fs.readFileSync(profPath, "utf-8").split("\n");
    let count = 0;
    const newLines = lines.filter(line => {
        if (line.includes("sigma_log.h")) {
            count++;
            if (count === 2) return false;
        }
        return true;
    });
    fs.writeFileSync(profPath, newLines.join("\n"), "utf-8");
}

const dispPath = "c:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS\\kernel\\core\\syscall\\sigma_syscall_dispatcher.h";
if (fs.existsSync(dispPath)) {
    let content = fs.readFileSync(dispPath, "utf-8");
    content = content.replace(/#define K_ERR_INVAL.*\n/, "#ifndef K_ERR_INVAL\n#define K_ERR_INVAL 22\n#endif\n");
    fs.writeFileSync(dispPath, content, "utf-8");
}
