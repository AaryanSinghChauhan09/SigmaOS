/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN PRINT SUBSYSTEM (S-PRINT)
 * =========================================================================
 * Mission: Kernel-native print spooler with zero-driver-bloat rendering.
 * Competitor parity: Linux CUPS, Windows Print Spooler, macOS CUPS/AirPrint.
 * ZERO-DEPENDENCY: Direct IPP/RAW socket printing; no CUPS daemon.
 * =========================================================================
 */

#ifndef SIGMA_PRINT_H
#define SIGMA_PRINT_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Print Job Priority --- */
#define SIGMA_PRINT_PRIORITY_LOW     0u
#define SIGMA_PRINT_PRIORITY_NORMAL  1u
#define SIGMA_PRINT_PRIORITY_HIGH    2u
#define SIGMA_PRINT_PRIORITY_URGENT  3u

/* --- Output Formats --- */
#define SIGMA_PRINT_FMT_RAW     0u  /* Direct PCL/PostScript      */
#define SIGMA_PRINT_FMT_PDF     1u  /* Sovereign PDF renderer      */
#define SIGMA_PRINT_FMT_TEXT    2u  /* Plain text                  */
#define SIGMA_PRINT_FMT_IMAGE   3u  /* Rasterised image            */

#define SIGMA_PRINT_JOB_MAX     64u
#define SIGMA_PRINT_NAME_LEN    48u

typedef struct {
    sigma_u32 job_id;
    sigma_u32 priority;
    sigma_u32 format;
    sigma_u32 page_count;
    sigma_u32 bytes_total;
    char      description[SIGMA_PRINT_NAME_LEN];
    sigma_u32 completed;
} sigma_print_job_t;

typedef struct {
    sigma_u32 jobs_queued;
    sigma_u32 jobs_completed;
    sigma_u32 jobs_failed;
    sigma_u32 spooler_active;
} sigma_print_state_t;

/* --- Print Primitives --- */
void      print_init(void);
sigma_u32 print_submit_job(sigma_u32 format, sigma_u32 priority,
                           const void* data, sigma_u32 bytes,
                           const char* description);
void      print_cancel_job(sigma_u32 job_id);
void      print_flush_spooler(void);
const sigma_print_state_t* print_get_state(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_PRINT_H */
