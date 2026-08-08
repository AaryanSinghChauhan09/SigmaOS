#ifndef ATOMIC_SEC_TOKEN_HPP
#define ATOMIC_SEC_TOKEN_HPP

#include "include/sigma_kernel_types.h"

class SecurityEnforcer {
public:
    virtual ~SecurityEnforcer() {}
    virtual sigma_bool validate_token(sigma_u64 token, sigma_u32 required_cap) = 0;
};

class SovereignTokenValidator : public SecurityEnforcer {
public:
    SovereignTokenValidator();
    virtual ~SovereignTokenValidator() {}
    virtual sigma_bool validate_token(sigma_u64 token, sigma_u32 required_cap) override;
};

#endif // ATOMIC_SEC_TOKEN_HPP
