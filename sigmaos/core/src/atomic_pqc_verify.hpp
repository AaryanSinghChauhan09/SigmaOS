#ifndef ATOMIC_PQC_VERIFY_HPP
#define ATOMIC_PQC_VERIFY_HPP

#include "include/sigma_kernel_types.h"

class SignatureVerifier {
public:
    virtual ~SignatureVerifier() {}
    virtual sigma_status verify_pqc_sig(const sigma_u8* pubkey, const sigma_u8* msg, sigma_size_t msg_len, const sigma_u8* sig) = 0;
};

class Dilithium5Verifier : public SignatureVerifier {
public:
    Dilithium5Verifier();
    virtual ~Dilithium5Verifier() {}
    virtual sigma_status verify_pqc_sig(const sigma_u8* pubkey, const sigma_u8* msg, sigma_size_t msg_len, const sigma_u8* sig) override;
};

#endif // ATOMIC_PQC_VERIFY_HPP
