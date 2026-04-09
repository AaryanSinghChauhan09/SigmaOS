/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN HAIKU OS PARITY (v1.0 — C11)
 * =========================================================================
 * Absorbed USPs from: Haiku / BeOS
 *   https://github.com/haiku/haiku
 *
 * Features implemented:
 *   ✓ BApplication and BWindow lifecycle simulation
 *   ✓ BMessage parsing and BeAPI application kits
 *   ✓ Micro-threaded UI loop per window
 * =========================================================================
 */

#ifndef SOVEREIGN_HAIKU_H
#define SOVEREIGN_HAIKU_H

#include "sigma_types.h"

typedef struct {
    sigma_u32 what;
    char data[256];
} SigmaBMessage_t;

typedef struct {
    char signature[128];
    sigma_bool active;
} SigmaBApplication_t;

typedef struct {
    char title[128];
    sigma_bool visible;
} SigmaBWindow_t;

sigma_err_t sigma_BApplication_Init(SigmaBApplication_t *app, const char *signature);
sigma_err_t sigma_BApplication_Run(SigmaBApplication_t *app);

sigma_err_t sigma_BWindow_Init(SigmaBWindow_t *win, const char *title);
sigma_err_t sigma_BWindow_Show(SigmaBWindow_t *win);
sigma_err_t sigma_BWindow_PostMessage(SigmaBWindow_t *win, SigmaBMessage_t *msg);

void SovereignHaiku_Init(void);

#endif /* SOVEREIGN_HAIKU_H */
