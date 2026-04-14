/* Σ SIGMAOS: SOVEREIGN TTY SHARD HEADER */
#ifndef SOVEREIGN_TTY_SHARD_H
#define SOVEREIGN_TTY_SHARD_H
#include "sigma_types.h"
typedef enum { PANE_SHELL, PANE_LOG, PANE_MONITOR } SigmaPaneType_t;
sigma_err_t sigma_tty_new_session (const char* name);
sigma_err_t sigma_tty_new_window  (sigma_u32 session_id, const char* win_name);
sigma_err_t sigma_tty_split_pane  (sigma_u32 session_id, sigma_u32 win_idx,
                                    SigmaPaneType_t type, const char* title,
                                    sigma_u32 rows, sigma_u32 cols);
void        sigma_tty_write       (sigma_u32 master_fd, const char* data);
void        sigma_tty_detach      (sigma_u32 session_id);
void        sigma_tty_attach      (sigma_u32 session_id);
void        SovereignTTYShard_Init (void);
void        SovereignTTY_Audit     (void);
#endif
