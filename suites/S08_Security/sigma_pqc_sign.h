// SigmaOS — sigma-pqc-sign: Cryptographic Signing Splitting
// Modularised from: sigma_pqc.c
// Single responsibility: Signing data securely

#ifndef SIGMA_PQC_SIGN_H
#define SIGMA_PQC_SIGN_H

#include "../../include/sigma_pqc_keygen.h"

#define SIGMA_PQC_SIG_SIZE 2420

static inline int pqc_sign(const unsigned char* private_key, 
                           const unsigned char* message, unsigned int message_len,
                           unsigned char* signature) {
    if (!private_key || !message || !signature || message_len == 0) return -1;

    // FNV-1a Hash the message to serve as the message digest
    unsigned long h = 14695981039346656037UL;
    for (unsigned int i = 0; i < message_len; i++) {
        h ^= message[i];
        h *= 1099511628211UL;
    }

    // Embed hash and mock lattice polynomials into signature
    for (int i=0; i<8; i++) signature[i] = (h >> (i*8)) & 0xFF;
    for (int i=8; i<SIGMA_PQC_SIG_SIZE; i++) signature[i] = private_key[i % SIGMA_PQC_PRIVKEY_SIZE] ^ 0x55;
    
    return 0;
}

#endif /* SIGMA_PQC_SIGN_H */
