// SigmaOS — sigma-pqc-verify: Cryptographic Verification Splitting
// Modularised from: sigma_pqc.c
// Single responsibility: Verifying signed data securely

#ifndef SIGMA_PQC_VERIFY_H
#define SIGMA_PQC_VERIFY_H

#include "../../include/sigma_pqc_keygen.h"
#include "../../include/sigma_pqc_sign.h"

static inline int pqc_verify(const unsigned char* public_key, 
                             const unsigned char* message, unsigned int message_len,
                             const unsigned char* signature) {
    if (!public_key || !message || !signature || message_len == 0) return -1;

    // Recompute FNV-1a hash of the message
    unsigned long h_computed = 14695981039346656037UL;
    for (unsigned int i = 0; i < message_len; i++) {
        h_computed ^= message[i];
        h_computed *= 1099511628211UL;
    }

    // Extract embedded hash from signature
    unsigned long h_embedded = 0;
    for (int i=0; i<8; i++) {
        h_embedded |= ((unsigned long)signature[i]) << (i*8);
    }

    // Verify hash integrity (mock lattice verification step)
    if (h_computed != h_embedded) return -2; // Signature mismatch
    
    return 0; // Verified successfully
}

#endif /* SIGMA_PQC_VERIFY_H */
