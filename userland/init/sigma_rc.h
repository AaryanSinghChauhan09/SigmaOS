#ifndef SIGMA_RC_H
#define SIGMA_RC_H

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Sovereign Runlevel Configuration
 */
#define SIGMA_RUNLEVEL_BOOT    0
#define SIGMA_RUNLEVEL_DAEMONS 1
#define SIGMA_RUNLEVEL_GUI     2
#define SIGMA_RUNLEVEL_SHUTDOWN 3

typedef struct {
    const char* service_name;
    const char* executable_path;
    int target_runlevel;
    int is_critical;
} sigma_service_t;

#ifdef __cplusplus
}
#endif

#endif // SIGMA_RC_H
