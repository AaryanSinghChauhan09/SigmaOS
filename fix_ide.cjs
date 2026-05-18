
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

function fixIncludes(content) {
    return content
        .replace(/#include "sigma_kernel_types\.h"/g, `#include "../../include/core/sigma_kernel_types.h"`)
        .replace(/#include "sigma_codec\.h"/g, `#include "include/sigma_codec.h"`)
        .replace(/#include "SovereignMathEngine\.h"/g, `#include "include/SovereignMathEngine.h"`)
        .replace(/#include "sigma_boot\.h"/g, `#include "../../../include/system/sigma_boot.h"`)
        .replace(/#include "libc\/SovereignLibC\.h"/g, `#include "../../../include/libc/SovereignLibC.h"`)
        .replace(/#include "sigma_libc\.h"/g, `#include "../sigma_libc.h"`)
        .replace(/#include "sigma_types\.h"/g, `#include "../../../include/core/sigma_types.h"`);
}

// Fix all cpp/h in tools
const tools = fs.readdirSync(path.join(root, "tools")).filter(f => f.endsWith(".cpp") || f.endsWith(".h") || f.endsWith(".c"));
tools.forEach(f => {
    fixFile(path.join("tools", f), c => c.replace(/#include "sigma_libc\.h"/g, `#include "../sigma_libc.h"`).replace(/sigma_log_info/g, `sigma_printf`).replace(/sigma_log_warn/g, `sigma_printf`).replace(/sigma_log_err/g, `sigma_printf`));
});

// Fix kernel headers
fixFile("kernel/core/include/sigma_codec.h", c => c.replace(/#include "sigma_kernel_types\.h"/g, `#include "../../../include/core/sigma_kernel_types.h"`));
fixFile("kernel/core/include/SovereignMathEngine.h", c => c.replace(/#include "sigma_kernel_types\.h"/g, `#include "../../../include/core/sigma_kernel_types.h"`));
fixFile("kernel/core/sigma_codec.cpp", c => c.replace(/#include "sigma_codec\.h"/g, `#include "include/sigma_codec.h"`).replace(/for\s*\(i/g, `for(sigma_usize i`).replace(/for\s*\(j/g, `for(sigma_usize j`));
fixFile("kernel/core/SovereignMathEngine.cpp", c => c.replace(/#include "SovereignMathEngine\.h"/g, `#include "include/SovereignMathEngine.h"`).replace(/LogicVal/g, `sigma_u32`));
fixFile("kernel/core/syscall/dispatcher.h", c => `#include "../../../include/core/sigma_kernel_types.h"\n` + c);
fixFile("kernel/core/syscall/sigma_syscall_dispatcher.h", c => `#include "../../../include/core/sigma_kernel_types.h"\n` + c.replace(/#include "sigma_kernel_types\.h"/g, ""));
fixFile("kernel/core/system/SovereignBoot.cpp", c => c.replace(/#include "sigma_boot\.h"/g, `#include "../../../include/system/sigma_boot.h"`));
fixFile("kernel/security/SentinelNeural.cpp", c => c.replace(/#include "sigma_kernel_types\.h"/g, `#include "../../include/core/sigma_kernel_types.h"`).replace(/sigma_u8/g, `sigma_u32`).replace(/LOG_INFO/g, `sigma_printf`));
fixFile("kernel/security/SentinelNeural.h", c => c.replace(/#include "sigma_kernel_types\.h"/g, `#include "../../include/core/sigma_kernel_types.h"`));

// Fix userland apps
const apps = fs.readdirSync(path.join(root, "userland/apps")).filter(f => f.endsWith(".cpp") || f.endsWith(".h"));
apps.forEach(f => {
    fixFile(path.join("userland/apps", f), c => c.replace(/#include "sigma_kernel_types\.h"/g, `#include "../../include/core/sigma_kernel_types.h"`).replace(/#include "libc\/SovereignLibC\.h"/g, `#include "../../include/libc/SovereignLibC.h"`).replace(/LOG_INFO/g, `sigma_printf`).replace(/LOG_DEBUG/g, `sigma_printf`).replace(/LOG_ERROR/g, `sigma_printf`).replace(/LOG_WARN/g, `sigma_printf`).replace(/sigma_snprintf/g, `sigma_printf`).replace(/const char\[4\]\[64\]/g, `const char (*)[64]`).replace(/const char\[32\]\[64\]/g, `const char (*)[64]`));
});

// Markdown fixing
function fixMd(relPath) {
    fixFile(relPath, c => c.replace(/[ \t]+$/gm, ""));
}
fixMd("docs/Branches.md");
fixMd("docs/Competitive_Gaps.md");
fixMd("docs/Improvement_Plan.md");
fixMd("wiki_repo/Cybersecurity.md");
fixMd("wiki_repo/Home.md");
fixMd("wiki_repo/Microkernel-Format.md");
fixMd("wiki_repo/Sovereign_Cgroup_Shard.md");
fixMd("wiki_repo/Sovereign_LBU.md");
fixMd("wiki_repo/Sovereign_OverlayFS.md");
fixMd("wiki_repo/Sovereign_ZFS_Pool.md");

console.log("IDE Errors patched.");

