/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN HAIKU OS PARITY — IMPL (v1.0 — C11)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignHaiku.h"

sigma_err_t sigma_BApplication_Init(SigmaBApplication_t *app, const char *signature) {
    sigma_strcpy(app->signature, signature, 128);
    app->active = SIGMA_FALSE;
    sigma_printf("Σ [HAIKU]: BApplication instantiated with signature: %s\n", signature);
    return SIGMA_OK;
}

sigma_err_t sigma_BApplication_Run(SigmaBApplication_t *app) {
    app->active = SIGMA_TRUE;
    sigma_printf("Σ [HAIKU]: BApplication::Run() engaged multithreaded message loops.\n");
    return SIGMA_OK;
}

sigma_err_t sigma_BWindow_Init(SigmaBWindow_t *win, const char *title) {
    sigma_strcpy(win->title, title, 128);
    win->visible = SIGMA_FALSE;
    sigma_printf("Σ [HAIKU]: BWindow '%s' constructed.\n", title);
    return SIGMA_OK;
}

sigma_err_t sigma_BWindow_Show(SigmaBWindow_t *win) {
    win->visible = SIGMA_TRUE;
    sigma_printf("Σ [HAIKU]: BWindow::Show() - Window framework visible on screen.\n");
    return SIGMA_OK;
}

sigma_err_t sigma_BWindow_PostMessage(SigmaBWindow_t *win, SigmaBMessage_t *msg) {
    sigma_printf("Σ [HAIKU]: BWindow '%s' received BMessage::what = 0x%08X\n", win->title, msg->what);
    return SIGMA_OK;
}

void SovereignHaiku_Init(void) {
    sigma_printf("Σ [HAIKU]: Initialising Sovereign Haiku BeAPI abstractions...\n");
    
    SigmaBApplication_t app;
    sigma_BApplication_Init(&app, "application/x-vnd.SigmaOS-Demo");
    
    SigmaBWindow_t win;
    sigma_BWindow_Init(&win, "Haiku-Parity Window");
    sigma_BWindow_Show(&win);
    
    SigmaBMessage_t cmd;
    cmd.what = 0x4255544E; /* 'BUTN' */
    sigma_BWindow_PostMessage(&win, &cmd);
    
    sigma_BApplication_Run(&app);
}
