// SigmaOS — Capability Token System
// Module: sigma-sec-caps
// Single responsibility: issue, verify, and revoke capability tokens
// Implements zero-trust access control without external libraries

#ifndef SIGMA_CAPS_H
#define SIGMA_CAPS_H

#define SIGMA_CAP_READ    0x01
#define SIGMA_CAP_WRITE   0x02
#define SIGMA_CAP_EXEC    0x04
#define SIGMA_CAP_NET     0x08
#define SIGMA_CAP_ADMIN   0x80

typedef struct SigmaCapToken {
    unsigned int owner_pid;
    unsigned char permissions;
    unsigned int nonce;          /* simple replay protection */
    unsigned char revoked;
} SigmaCapToken;

/* Mint a new capability token */
static inline SigmaCapToken cap_issue(unsigned int pid,
                                      unsigned char perms,
                                      unsigned int nonce) {
    SigmaCapToken tok;
    tok.owner_pid   = pid;
    tok.permissions = perms;
    tok.nonce       = nonce;
    tok.revoked     = 0;
    return tok;
}

/* Check if token has a specific permission */
static inline int cap_check(const SigmaCapToken* tok, unsigned char perm) {
    if (tok->revoked) return 0;
    return (tok->permissions & perm) != 0;
}

/* Revoke a token */
static inline void cap_revoke(SigmaCapToken* tok) {
    tok->revoked = 1;
}

#endif /* SIGMA_CAPS_H */
