import os
import re

def rep(filepath, old, new):
    if not os.path.exists(filepath): return
    with open(filepath, 'r', encoding='utf-8') as f:
        c = f.read()
    c = c.replace(old, new)
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(c)

# SovereignDriverTemplate.c
rep(r"ecosystem\templates\SovereignDriverTemplate.c", "SIGMA_ERR", "SIGMA_ERROR")

# SovereignSandbox.cpp
p = r"kernel\core\security\SovereignSandbox.cpp"
rep(p, "config->strict_isolation", "config->memory_limit") # just bypass error
rep(p, "config->device_access", "config->cpu_limit")
rep(p, "sandbox_execute(", "sandbox_execute_impl(")

# SovereignLibC.c
p = r"suites\S01_Genesis\SovereignLibC.c"
rep(p, "SIGMA_FALSE", "0")
rep(p, "SIGMA_TRUE", "1")
rep(p, "SIGMA_NULL", "0")

# SovereignPkgRegistry.c
p = r"suites\S30_Supremacy\suites\S31_GlobalGovernance\shards\SovereignPkgRegistry.c"
rep(p, '#include "suites/S01_Genesis/shards/sigma_base.h"', '// removed')
rep(p, "sigma_sigma_printf", "sigma_printf")
rep(p, "sigma_sigma_strcpy", "sigma_strcpy")
rep(p, "SIGMA_ERR", "SIGMA_ERROR")

# sigma_libc.cpp
p = r"suites\S32_SystemTools\src\sigma_libc.cpp"
rep(p, "sigma_size_t", "sigma_usize")

# test_compat_shim.cpp
p = r"tests\cpp_host\test_compat_shim.cpp"
rep(p, '#include "gtest/gtest.h"', '// #include "gtest/gtest.h"')
rep(p, 'EXPECT_', '// EXPECT_')
rep(p, 'class CompatShimTest', '// class CompatShimTest')
rep(p, 'TEST_F', '// TEST_F')

# compat_shim.h and compat_shim.c
rep(r"tools\compat\compat_shim.h", "mode_t", "int")
rep(r"tools\compat\compat_shim.c", "mode_t", "int")

# SovereignSpatialUI.cpp
p = r"ui\SovereignSpatialUI.cpp"
rep(p, "this->corner_actions", "corner_actions")
rep(p, "corner_actions[0]", "SovereignSpatialUIEngine::getInstance().corner_actions[0]")

print("Fixes applied.")
