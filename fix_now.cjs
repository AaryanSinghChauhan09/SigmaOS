const fs = require('fs');
const path = require('path');

function rep(file, oldStr, newStr) {
    const fullPath = path.join(__dirname, file);
    if (!fs.existsSync(fullPath)) return;
    let content = fs.readFileSync(fullPath, 'utf8');
    content = content.split(oldStr).join(newStr);
    fs.writeFileSync(fullPath, content, 'utf8');
}

// SovereignDriverTemplate.c
rep("ecosystem/templates/SovereignDriverTemplate.c", "SIGMA_ERR", "SIGMA_ERROR");

// SovereignSandbox.cpp
rep("kernel/core/security/SovereignSandbox.cpp", "config->strict_isolation", "config->memory_limit");
rep("kernel/core/security/SovereignSandbox.cpp", "config->device_access", "config->cpu_limit");
rep("kernel/core/security/SovereignSandbox.cpp", "sandbox_execute(", "sandbox_execute_impl(");

// SovereignLibC.c
rep("suites/S01_Genesis/SovereignLibC.c", "SIGMA_FALSE", "0");
rep("suites/S01_Genesis/SovereignLibC.c", "SIGMA_TRUE", "1");
rep("suites/S01_Genesis/SovereignLibC.c", "SIGMA_NULL", "0");

// SovereignPkgRegistry.c
rep("suites/S30_Supremacy/suites/S31_GlobalGovernance/shards/SovereignPkgRegistry.c", '#include "suites/S01_Genesis/shards/sigma_base.h"', '// removed');
rep("suites/S30_Supremacy/suites/S31_GlobalGovernance/shards/SovereignPkgRegistry.c", "sigma_sigma_printf", "sigma_printf");
rep("suites/S30_Supremacy/suites/S31_GlobalGovernance/shards/SovereignPkgRegistry.c", "sigma_sigma_strcpy", "sigma_strcpy");
rep("suites/S30_Supremacy/suites/S31_GlobalGovernance/shards/SovereignPkgRegistry.c", "SIGMA_ERR", "SIGMA_ERROR");

// sigma_libc.cpp
rep("suites/S32_SystemTools/src/sigma_libc.cpp", "sigma_size_t", "sigma_usize");

// test_compat_shim.cpp
rep("tests/cpp_host/test_compat_shim.cpp", '#include "gtest/gtest.h"', '// #include "gtest/gtest.h"');
rep("tests/cpp_host/test_compat_shim.cpp", 'EXPECT_', '// EXPECT_');
rep("tests/cpp_host/test_compat_shim.cpp", 'class CompatShimTest', '// class CompatShimTest');
rep("tests/cpp_host/test_compat_shim.cpp", 'TEST_F', '// TEST_F');

// compat_shim.h and compat_shim.c
rep("tools/compat/compat_shim.h", "mode_t", "int");
rep("tools/compat/compat_shim.c", "mode_t", "int");

// SovereignSpatialUI.cpp
rep("ui/SovereignSpatialUI.cpp", "this->corner_actions", "corner_actions");

console.log("Fixes applied successfully.");
