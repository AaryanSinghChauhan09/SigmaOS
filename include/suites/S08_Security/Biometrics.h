#ifndef SIGMA_BIOMETRICS_H
#define SIGMA_BIOMETRICS_H

#include <sigma_types.h>


// SigmaOS Biometric Authentication Module
// Ties into the Zero Trust architecture

void sec_init_biometric_hardware(void);

// Triggers Face ID scanning matching against secure enclave data
bool sec_authenticate_face_id(uint32_t user_id);

// Triggers Fingerprint scanner via secure SPI bus
bool sec_authenticate_fingerprint(uint32_t user_id);

// Generate biometric entropy for cryptographic seeding
void sec_gather_biometric_entropy(uint8_t* buffer, uint32_t length);

#endif // SIGMA_BIOMETRICS_H

