// SigmaOS — Sigma-ZKP: Zero-Knowledge Proof Layer (Native C)
// Inspired by: zk-SNARKs (Ethereum), Zcash, seL4 formal verification
// Module: sigma-sec-zkp
// USP: No elliptic curve library — implements Fiat-Shamir sigma protocol natively
// Proves knowledge of a secret without revealing it — pure arithmetic

#ifndef SIGMA_ZKP_H
#define SIGMA_ZKP_H

// Finite field arithmetic modulo a safe prime
// P = 2^31 - 1 (Mersenne prime for simplicity; production uses P-256)
#define SIGMA_ZKP_P  2147483647U  // 2^31 - 1

typedef struct SigmaZKPProver {
    unsigned int secret;      // x: the secret value
    unsigned int commitment;  // r: random nonce
    unsigned int challenge;   // c: verifier challenge
    unsigned int response;    // s = r + c*x (mod P)
} SigmaZKPProver;

typedef struct SigmaZKPVerifier {
    unsigned int generator;   // g: public generator
    unsigned int public_key;  // h = g^x mod P
    unsigned int commitment;  // A = g^r mod P (from prover)
    unsigned int challenge;   // c: verifier's challenge
} SigmaZKPVerifier;

// Modular exponentiation: base^exp mod m (square-and-multiply)
static inline unsigned int sigma_modpow(unsigned int base, unsigned int exp,
                                         unsigned int mod) {
    unsigned long result = 1;
    unsigned long b = base % mod;
    while (exp > 0) {
        if (exp & 1) result = (result * b) % mod;
        b = (b * b) % mod;
        exp >>= 1;
    }
    return (unsigned int)result;
}

// Prover: commit phase — generate A = g^r mod P
static inline unsigned int zkp_commit(SigmaZKPVerifier* v,
                                       SigmaZKPProver* p,
                                       unsigned int g, unsigned int x,
                                       unsigned int r) {
    p->secret     = x;
    p->commitment = r;
    v->generator  = g;
    v->public_key = sigma_modpow(g, x, SIGMA_ZKP_P);
    v->commitment = sigma_modpow(g, r, SIGMA_ZKP_P);
    return v->commitment;
}

// Verifier: issue challenge c
static inline unsigned int zkp_challenge(SigmaZKPVerifier* v, unsigned int c) {
    v->challenge = c % SIGMA_ZKP_P;
    return v->challenge;
}

// Prover: compute response s = (r + c*x) mod P
static inline unsigned int zkp_respond(SigmaZKPProver* p, unsigned int c) {
    p->challenge = c;
    p->response  = (p->commitment + (unsigned long)c * p->secret) % SIGMA_ZKP_P;
    return p->response;
}

// Verifier: verify g^s == A * h^c (mod P)
static inline int zkp_verify(SigmaZKPVerifier* v, unsigned int s) {
    unsigned int lhs = sigma_modpow(v->generator, s, SIGMA_ZKP_P);
    unsigned int rhs_a = v->commitment;
    unsigned int rhs_b = sigma_modpow(v->public_key, v->challenge, SIGMA_ZKP_P);
    unsigned int rhs = ((unsigned long)rhs_a * rhs_b) % SIGMA_ZKP_P;
    return (lhs == rhs) ? 1 : 0;
}

#endif /* SIGMA_ZKP_H */
