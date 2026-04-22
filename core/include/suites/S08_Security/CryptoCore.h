#ifndef SIGMA_RUST_CRYPTO_H
#define SIGMA_RUST_CRYPTO_H

#include <stddef.h>

// Extern declaration for the Rust implementation
extern int rust_crypto_verify_signature(const unsigned char* data, size_t data_len, const unsigned char* sig);

#endif // SIGMA_RUST_CRYPTO_H
