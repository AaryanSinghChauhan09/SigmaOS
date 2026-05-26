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
void SovereignPkg_InitRegistry(void);
int SovereignPkg_Register(const char* name, const char* version, CurationLevel_t curation);
void SovereignPkg_Audit(void);
void SovereignPkg_SnapshotState(void);
int SovereignPkg_Rollback(sigma_u32 generation_id);
void SovereignPkg_LoadManifest(const char* manifest_data);

#ifdef __cplusplus
}
#endif

#endif // SIGMA_PKG_REGISTRY_H
