#ifndef SIGMA_ZERO_TRUST_H
#define SIGMA_ZERO_TRUST_H

// SigmaOS Zero Trust & Security Shard
// Absorbing Defender/SELinux/AppArmor ideas
// Includes: Sandboxing, Secure Boot validation, ASLR
#include <stdint.h>

void security_init_secure_boot_aslr();
void security_enforce_zero_trust_policy();
void security_sandbox_process(uint64_t pid);

#endif // SIGMA_ZERO_TRUST_H
