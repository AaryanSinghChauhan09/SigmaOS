// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_trust_labels.h — Qubes-inspired trust label MAC policy matrix
 *
 * Replaces sigma_mac_enforce()'s always-GRANTED stub with a real information
 * flow control matrix. Labels are integers (fast comparison), not strings.
 *
 * Information flow rule: src → dst is allowed iff sigma_iflow_matrix[src][dst]
 * is true. Higher-numbered labels (less trusted) cannot send to lower-numbered
 * labels (more trusted) — this is the core Qubes isolation guarantee.
 */
#include <sigma_kernel_types.h>

typedef sigma_u32 sigma_trust_label_t;

typedef enum {
    SIGMA_TRUST_KERNEL     = 0,  /* sigma kernel itself — highest trust       */
    SIGMA_TRUST_SYSTEM     = 1,  /* sigma daemons: trustd, init, pkg          */
    SIGMA_TRUST_PRIVILEGED = 2,  /* root-equivalent user processes            */
    SIGMA_TRUST_USER       = 3,  /* normal user processes                     */
    SIGMA_TRUST_ISOLATED   = 4,  /* sandboxed / jailed processes              */
    SIGMA_TRUST_UNTRUSTED  = 5,  /* browser tabs, downloaded code, WASM       */
    SIGMA_TRUST_COUNT      = 6,
} sigma_trust_level_t;

/*
 * Information flow matrix — row=src, col=dst, true=ALLOW
 * Rule: untrusted code cannot IPC to system/kernel services.
 * Matches Qubes OS's Qubes-RPC policy model.
 */
static const bool sigma_iflow_matrix[SIGMA_TRUST_COUNT][SIGMA_TRUST_COUNT] = {
/*              KERNEL  SYSTEM  PRIV    USER    ISOLATE UNTRUST */
/* KERNEL  */ { true,   true,   true,   true,   true,   true  },
/* SYSTEM  */ { false,  true,   true,   true,   true,   true  },
/* PRIV    */ { false,  false,  true,   true,   true,   true  },
/* USER    */ { false,  false,  false,  true,   true,   false },
/* ISOLATE */ { false,  false,  false,  false,  true,   false },
/* UNTRUST */ { false,  false,  false,  false,  false,  true  },
};

/*
 * sigma_mac_check_iflow — the function previously called by sigma_avc.cpp
 * as a stub. Now backed by the real matrix.
 */
static inline int sigma_mac_check_iflow(sigma_trust_label_t src,
                                         sigma_trust_label_t dst) {
    if (src >= SIGMA_TRUST_COUNT || dst >= SIGMA_TRUST_COUNT) return -1;
    return sigma_iflow_matrix[src][dst] ? 0 : -1;
}
