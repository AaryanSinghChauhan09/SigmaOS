/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN BIOMETRICS ENGINE (S-BIOMETRICS)
 * =========================================================================
 * Mission: Silicon-level biometric authentication (fingerprint, face, iris)
 * providing instantaneous, zero-trust user login and personalization.
 * =========================================================================
 */

#ifndef SIGMA_BIOMETRICS_H
#define SIGMA_BIOMETRICS_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    BIO_TYPE_FINGERPRINT,
    BIO_TYPE_FACIAL_RECOGNITION,
    BIO_TYPE_IRIS_SCAN
} sigma_bio_type_t;

/* --- Biometrics Primitives --- */
void biometrics_init(void);
bool biometrics_authenticate(sigma_bio_type_t type, const void* sensor_data);
void biometrics_enroll(sigma_bio_type_t type, const void* sensor_data);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_BIOMETRICS_H */
