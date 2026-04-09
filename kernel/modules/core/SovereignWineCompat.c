/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN WINE COMPAT + DXVK — IMPLEMENTATION (v1.0 — C11)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignWineCompat.h"

/* Global Wine context */
SigmaWineCtx_t g_sigma_wine;

/* Last Win32 error code (per-thread in a real Wine) */
static sigma_u32 s_last_error = 0;

/* =========================================================================
 * §1  PE LOADER
 * ====================================================================== */

sigma_err_t sigma_pe_load(SigmaWineCtx_t *w, const char *path) {
    if (w->image_count >= 32) return SIGMA_ENOSPC;

    SigmaPEImage_t *img = &w->loaded_images[w->image_count];
    sigma_memset(img, 0, sizeof(*img));

    /* Derive basename for the name field */
    const char *base = sigma_strrchr(path, '/');
    sigma_strcpy(img->name, base ? base + 1 : path, PE_NAME_MAX);

    /*
     * In a live kernel: mmap the PE file, validate MZ magic (0x5A4D),
     * follow e_lfanew to the PE signature (0x4550), parse COFF and
     * optional headers, map each section to VMA, process imports/exports.
     * Here we simulate the outcome.
     */
    img->is_64bit    = SIGMA_TRUE;
    img->is_dll      = (sigma_strstr(img->name, ".dll") != SIGMA_NULL);
    img->image_size  = 4 * 1024 * 1024;   /* 4 MB simulated */
    img->base        = (void*)0x140000000ULL;
    img->entry_rva   = 0x1000;

    /* Fake a .text section */
    if (img->n_sections < PE_MAX_SECTIONS) {
        SigmaPESection_t *s = &img->sections[img->n_sections++];
        sigma_memset(s, 0, sizeof(*s));
        s->VirtualAddress = 0x1000;
        s->VirtualSize    = 0x80000;
        s->Characteristics= 0x60000020; /* RX */
        sigma_memcpy(s->Name, ".text", 5);
    }

    w->image_count++;
    sigma_printf("Σ [WINE]: PE loaded: %s  base=0x%llx  size=%lluKB  %s\n",
                 img->name,
                 (unsigned long long)(sigma_uptr)img->base,
                 (unsigned long long)(img->image_size / 1024),
                 img->is_dll ? "DLL" : "EXE");
    return SIGMA_OK;
}

sigma_err_t sigma_pe_run(SigmaWineCtx_t *w, const char *name, const char *args) {
    /* Find the loaded image */
    for (sigma_u32 i = 0; i < w->image_count; i++) {
        if (sigma_strstr(w->loaded_images[i].name, name)) {
            SigmaPEImage_t *img = &w->loaded_images[i];
            if (img->is_dll) {
                sigma_printf("Σ [WINE]: Cannot exec DLL '%s' directly.\n", name);
                return SIGMA_EINVAL;
            }
            sigma_printf("Σ [WINE]: Launching '%s' args='%s'\n"
                         "  EntryPoint: 0x%llx\n",
                         name, args ? args : "",
                         (unsigned long long)((sigma_uptr)img->base + img->entry_rva));
            /* In a live kernel: sigma_fork() + set RIP = base + entry_rva */
            return SIGMA_OK;
        }
    }
    sigma_printf("Σ [WINE]: '%s' not loaded. Call sigma_pe_load() first.\n", name);
    return SIGMA_ENOENT;
}

void sigma_pe_list(const SigmaWineCtx_t *w) {
    sigma_printf("Σ [WINE]: Loaded PE images (%u):\n", w->image_count);
    for (sigma_u32 i = 0; i < w->image_count; i++) {
        const SigmaPEImage_t *img = &w->loaded_images[i];
        sigma_printf("  %-32s  base=0x%llx  %s  sections=%u\n",
                     img->name,
                     (unsigned long long)(sigma_uptr)img->base,
                     img->is_dll ? "DLL" : "EXE",
                     img->n_sections);
    }
}

/* =========================================================================
 * §2  REGISTRY EMULATION
 * ====================================================================== */

static sigma_u32 reg_hash(const char *hive, const char *key, const char *name) {
    sigma_u32 h = 5381;
    while (*hive)  { h = ((h << 5) + h) ^ (sigma_u8)*hive++;  }
    while (*key)   { h = ((h << 5) + h) ^ (sigma_u8)*key++;   }
    while (*name)  { h = ((h << 5) + h) ^ (sigma_u8)*name++;   }
    return h % SIGMA_REG_ENTRIES;
}

sigma_err_t sigma_reg_set(SigmaWineCtx_t *w,
                           const char *hive, const char *key,
                           const char *name, SigmaRegType_t type,
                           const void *data, sigma_u32 len) {
    sigma_u32 probe = reg_hash(hive, key, name);
    for (sigma_u32 i = 0; i < SIGMA_REG_ENTRIES; i++) {
        sigma_u32 idx = (probe + i) % SIGMA_REG_ENTRIES;
        SigmaRegEntry_t *e = &w->registry[idx];
        if (!e->occupied || (sigma_streq(e->hive, hive) &&
                              sigma_streq(e->key,  key)  &&
                              sigma_streq(e->name, name))) {
            sigma_strcpy(e->hive, hive, 16);
            sigma_strcpy(e->key,  key,  SIGMA_REG_KEY_MAX);
            sigma_strcpy(e->name, name, SIGMA_REG_KEY_MAX);
            e->type = type;
            if (len > SIGMA_REG_VAL_MAX) len = SIGMA_REG_VAL_MAX;
            sigma_memcpy(e->data, data, len);
            e->data_len = len;
            if (!e->occupied) { e->occupied = SIGMA_TRUE; w->reg_count++; }
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOSPC;
}

sigma_err_t sigma_reg_get(const SigmaWineCtx_t *w,
                           const char *hive, const char *key,
                           const char *name, void *out, sigma_u32 max) {
    sigma_u32 probe = reg_hash(hive, key, name);
    for (sigma_u32 i = 0; i < SIGMA_REG_ENTRIES; i++) {
        sigma_u32 idx = (probe + i) % SIGMA_REG_ENTRIES;
        const SigmaRegEntry_t *e = &w->registry[idx];
        if (!e->occupied) return SIGMA_ENOENT;
        if (sigma_streq(e->hive, hive) && sigma_streq(e->key, key) &&
            sigma_streq(e->name, name)) {
            sigma_u32 copy = e->data_len < max ? e->data_len : max;
            sigma_memcpy(out, e->data, copy);
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOENT;
}

void sigma_reg_dump(const SigmaWineCtx_t *w, const char *hive) {
    sigma_printf("Σ [WINE-REG]: Dump %s (%u total):\n", hive, w->reg_count);
    for (sigma_u32 i = 0; i < SIGMA_REG_ENTRIES; i++) {
        const SigmaRegEntry_t *e = &w->registry[i];
        if (!e->occupied) continue;
        if (!sigma_streq(e->hive, hive)) continue;
        sigma_printf("  [%s\\%s]  \"%s\" = ", e->hive, e->key, e->name);
        if (e->type == SIGMA_REG_SZ) {
            sigma_printf("\"%s\"\n", (const char *)e->data);
        } else if (e->type == SIGMA_REG_DWORD) {
            sigma_u32 v; sigma_memcpy(&v, e->data, 4);
            sigma_printf("0x%08x\n", v);
        } else {
            sigma_printf("<binary len=%u>\n", e->data_len);
        }
    }
}

/* =========================================================================
 * §3  DXVK  (DirectX → Vulkan translation)
 * ====================================================================== */

static void dxvk_seed_map(SigmaWineCtx_t *w) {
    /* Seed the most common D3D11 → Vulkan mappings */
    static const struct { const char *dx; const char *vk; sigma_u32 ver; } kMap[] = {
        {"ID3D11Device::CreateBuffer",         "vkCreateBuffer",          11},
        {"ID3D11Device::CreateTexture2D",       "vkCreateImage",           11},
        {"ID3D11DeviceContext::Draw",           "vkCmdDraw",               11},
        {"ID3D11DeviceContext::DrawIndexed",    "vkCmdDrawIndexed",        11},
        {"ID3D11DeviceContext::IASetVertexBuffers","vkCmdBindVertexBuffers",11},
        {"ID3D11DeviceContext::RSSetViewports", "vkCmdSetViewport",        11},
        {"ID3D11DeviceContext::OMSetRenderTargets","vkCmdBeginRenderPass", 11},
        {"IDXGISwapChain::Present",             "vkQueuePresentKHR",       11},
        {"IDirect3DDevice9::DrawPrimitive",     "vkCmdDraw",                9},
        {"IDirect3DDevice9::Present",           "vkQueuePresentKHR",        9},
        {SIGMA_NULL, SIGMA_NULL, 0}
    };
    for (sigma_u32 i = 0; kMap[i].dx && w->dxvk_count < SIGMA_DXVK_MAP_MAX; i++) {
        SigmaDXVKEntry_t *e = &w->dxvk_map[w->dxvk_count++];
        sigma_strcpy(e->dx_call, kMap[i].dx, 64);
        sigma_strcpy(e->vk_call, kMap[i].vk, 64);
        e->d3d_version      = kMap[i].ver;
        e->translated_count = 0;
    }
}

sigma_err_t sigma_dxvk_translate(SigmaWineCtx_t *w,
                                  const char *dx_call, sigma_u32 d3d_ver) {
    for (sigma_u32 i = 0; i < w->dxvk_count; i++) {
        SigmaDXVKEntry_t *e = &w->dxvk_map[i];
        if (e->d3d_version == d3d_ver && sigma_streq(e->dx_call, dx_call)) {
            e->translated_count++;
            sigma_printf("Σ [DXVK]: D3D%u %s -> %s\n",
                         d3d_ver, dx_call, e->vk_call);
            return SIGMA_OK;
        }
    }
    sigma_printf("Σ [DXVK]: No mapping for D3D%u::%s\n", d3d_ver, dx_call);
    return SIGMA_ENOENT;
}

void sigma_dxvk_stats(const SigmaWineCtx_t *w) {
    sigma_printf("Σ [DXVK]: Translation stats (%u mappings):\n", w->dxvk_count);
    sigma_u64 total = 0;
    for (sigma_u32 i = 0; i < w->dxvk_count; i++)
        total += w->dxvk_map[i].translated_count;
    sigma_printf("  Total translations: %llu\n", (unsigned long long)total);
    for (sigma_u32 i = 0; i < w->dxvk_count && i < 5; i++) {
        const SigmaDXVKEntry_t *e = &w->dxvk_map[i];
        if (e->translated_count > 0)
            sigma_printf("  D3D%u %-40s -> %-28s [%llu calls]\n",
                         e->d3d_version, e->dx_call, e->vk_call,
                         (unsigned long long)e->translated_count);
    }
}

sigma_err_t sigma_esync_create(SigmaWineCtx_t *w) {
    w->esync_enabled = SIGMA_TRUE;
    sigma_printf("Σ [WINE]: esync enabled (eventfd-based NT sync primitives).\n");
    return SIGMA_OK;
}

sigma_err_t sigma_fsync_create(SigmaWineCtx_t *w) {
    w->fsync_enabled = SIGMA_TRUE;
    sigma_printf("Σ [WINE]: fsync enabled (futex-based NT sync, Proton-GE).\n");
    return SIGMA_OK;
}

/* =========================================================================
 * §4  Win32 API STUBS
 * ====================================================================== */

sigma_u32 sigma_win32_GetLastError(void)  { return s_last_error; }
void      sigma_win32_SetLastError(sigma_u32 e) { s_last_error = e; }

void *sigma_win32_VirtualAlloc(void *addr, sigma_size_t size,
                                sigma_u32 type, sigma_u32 protect) {
    (void)addr; (void)type; (void)protect;
    /* Map to sigma_mmap internally */
    return sigma_mmap(SIGMA_NULL, size, 3, 0x22, -1, 0);
}

sigma_err_t sigma_win32_VirtualFree(void *addr) {
    (void)addr;
    /* Would call sigma munmap */
    return SIGMA_OK;
}

int sigma_win32_CreateThread(void *(*fn)(void*), void *arg) {
    (void)fn; (void)arg;
    sigma_printf("Σ [WINE]: CreateThread -> sigma_clone(CLONE_THREAD)\n");
    return 0;
}

void sigma_win32_ExitProcess(sigma_u32 code) {
    sigma_printf("Σ [WINE]: ExitProcess(%u)\n", code);
    sigma_exit((int)code);
}

void sigma_win32_MessageBoxA(const char *title, const char *msg) {
    sigma_printf("Σ [WINE]: MessageBox [%s] %s\n", title, msg);
}

/* =========================================================================
 * SovereignWineCompat_Init
 * ====================================================================== */
void SovereignWineCompat_Init(void) {
    sigma_printf("Σ [WINE]: Initialising Sovereign Wine Compat Layer "
                 "(Wine + DXVK + Proton parity)...\n");

    sigma_memset(&g_sigma_wine, 0, sizeof(g_sigma_wine));
    dxvk_seed_map(&g_sigma_wine);

    /* Load some PEs */
    sigma_pe_load(&g_sigma_wine, "/wine/drive_c/windows/system32/ntdll.dll");
    sigma_pe_load(&g_sigma_wine, "/wine/drive_c/windows/system32/kernel32.dll");
    sigma_pe_load(&g_sigma_wine, "/wine/drive_c/Program Files/game/game.exe");
    sigma_pe_list(&g_sigma_wine);

    /* Registry */
    static const char ver[] = "SigmaOS Wine 9.0";
    sigma_reg_set(&g_sigma_wine, "HKLM",
                  "SOFTWARE\\Wine", "Version",
                  SIGMA_REG_SZ, ver, (sigma_u32)sigma_strlen(ver) + 1);
    static const sigma_u32 dword_one = 1;
    sigma_reg_set(&g_sigma_wine, "HKLM",
                  "SOFTWARE\\Wine", "HardwareAccel",
                  SIGMA_REG_DWORD, &dword_one, 4);
    sigma_reg_dump(&g_sigma_wine, "HKLM");

    /* DXVK */
    sigma_esync_create(&g_sigma_wine);
    sigma_fsync_create(&g_sigma_wine);
    sigma_dxvk_translate(&g_sigma_wine,
                          "IDXGISwapChain::Present", 11);
    sigma_dxvk_translate(&g_sigma_wine,
                          "ID3D11DeviceContext::DrawIndexed", 11);
    sigma_dxvk_translate(&g_sigma_wine,
                          "IDirect3DDevice9::DrawPrimitive", 9);
    sigma_dxvk_stats(&g_sigma_wine);

    /* Win32 stub demo */
    sigma_win32_MessageBoxA("SigmaOS", "Windows app running on Sovereign Wine!");

    sigma_printf("Σ [WINE]: Sovereign Wine Compat + DXVK online.\n");
}
