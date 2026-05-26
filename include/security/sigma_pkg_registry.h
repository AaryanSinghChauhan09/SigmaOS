#ifndef SIGMA_PKG_REGISTRY_H
#define SIGMA_PKG_REGISTRY_H

#include "sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    CURATION_UNVERIFIED = 0,
    CURATION_COMMUNITY = 1,
    CURATION_OFFICIAL = 2
} CurationLevel_t;

typedef struct {
    char name[64];
    char version[16];
    CurationLevel_t curation;
    sigma_bool seated;
} SovereignPkgEntry_t;

CurationLevel_t SovereignPkg_GetCuration(const char* name);

#ifdef __cplusplus
}
#endif

#endif // SIGMA_PKG_REGISTRY_H
