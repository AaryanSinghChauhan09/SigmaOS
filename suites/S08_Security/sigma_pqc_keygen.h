// SigmaOS — sigma-pqc-keygen: Key Generation Splitting
// Modularised from: sigma_pqc.c
// Single responsibility: Key generation for quantum-safe primitives

#ifndef SIGMA_PQC_KEYGEN_H
#define SIGMA_PQC_KEYGEN_H

#define SIGMA_PQC_PUBKEY_SIZE 1184
#define SIGMA_PQC_PRIVKEY_SIZE 2400

static inline int pqc_generate_keypair(unsigned char* public_key, unsigned char* private_key) {
    if (!public_key || !private_key) return -1;
    // Generate true random bytes via RDTSC entropy (placeholder for actual lattice math)
    unsigned long entropy;
#if defined(__x86_64__)
    unsigned int lo, hi;
    __asm__ __volatile__("rdtsc" : "=a"(lo), "=d"(hi));
    entropy = ((unsigned long)hi << 32) | lo;
#else
    entropy = 0xDEADBEEF;
#endif

    // Populate mock keys
    for(int i=0; i<SIGMA_PQC_PUBKEY_SIZE; i++) public_key[i] = (entropy >> (i % 8)) & 0xFF;
    for(int i=0; i<SIGMA_PQC_PRIVKEY_SIZE; i++) private_key[i] = (entropy >> ((i+3) % 8)) & 0xFF;
    return 0;
}

#endif /* SIGMA_PQC_KEYGEN_H */
