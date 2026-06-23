# Current Problems Log

## Status: ✅ ALL RESOLVED — 2026-06-23

All IDE-reported errors have been resolved as of v15.2:

### Fixed in v15.2 (2026-06-23)

- ✅ `sigma_log.h` missing → created in `klib/include/sigma_log.h` (wraps `sigma_kernel_types.h`)
- ✅ `sigma_test_framework.h` missing → created in `klib/include/sigma_test_framework.h` with `SIGMA_ASSERT`, `SIGMA_ASSERT_EQ`, `SIGMA_ASSERT_NE`, `SIGMA_ASSERT_NONNULL` macros
- ✅ `sigma_kernel_types.h` not found (relative include) → fixed in:
  - `kernel/core/ai/sigma_inference_engine.cpp`
  - `kernel/core/hal/sigma_device_tree.cpp`
  - `kernel/drivers/audio/sigma_hda.cpp`
  - `kernel/drivers/net/wifi/sigma_80211.cpp`
  - `kernel/net/mesh/sigma_fleet_protocol.cpp`
  - `kernel/fs/semantic_fs/sigma_semantic_fs.cpp`
- ✅ `sigma_log.h` not found (relative include) → fixed in same 6 files above (changed `"..."` → `<...>`)
- ✅ `compile_flags.txt` → added `-Iklib/include` relative path and `-std=c++17` for clangd compatibility
- ✅ `Unknown type name 'sigma_inference_req_t'` → resolved (defined in `sigma_ai.h`, now properly included via chain)
- ✅ `Unknown type name 'uint32_t'`, `uint8_t`, `uint16_t`, `uint64_t`, `sigma_size_t` → resolved via `sigma_kernel_types.h`
- ✅ `Unknown type name 'sigma_process_t'` → resolved via `sigma_kernel_types.h`
- ✅ `Use of undeclared identifier 'sigma_log_info'` → resolved via `sigma_log.h` → `sigma_kernel_types.h`
- ✅ `Use of undeclared identifier 'sigma_printf'` → resolved via `sigma_stdio.h` (already in klib/include, angle-bracket includes were already correct)
- ✅ `Use of undeclared identifier 'SIGMA_CTX_SYSTEM'`, `SIGMA_PERM_WRITE` → resolved via `sigma_kernel_types.h`
- ✅ Template errors in `zenith_terminal.cpp` (`no type named 'char_type'`, `is_void_v`) → cascading parse errors now resolved since the root header is findable

### Previously Fixed (v15.1)

- ✅ `sigma_boot.h` missing header → created with boot stage defines
- ✅ `SovereignBoot.cpp` invalid `this` usage → refactored class structure
- ✅ `sigma_vr_studio.cpp` stray type name → leading byte stripped
- ✅ `SovereignVideo.cpp` unused headers → `SigmaOOP.hpp` & `sigma_types.h` removed
- ✅ `zenith_desktop.css` WebKit prefixes → `-webkit-backdrop-filter` and `-webkit-user-select` injected
- ✅ `index.html` / `zenith.html` inline styles → extracted to `external_styles.css`
- ✅ HTML accessibility → `title` and `aria-label` added to form elements and iframes
- ✅ stdlib dependencies → purged across all kernel `.cpp`/`.h` files
