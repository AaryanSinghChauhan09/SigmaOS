/**
 * @file sigma_edition_builder.cpp
 * @brief Roadmap Features #88, #89, #91, #96 — Specialized Edition Builders
 *
 * Provides APIs for the build system to query or enforce constraints based 
 * on the active edition (e.g., IoT, Microkernel, Cloud, Research).
 */

#include "../../include/sigma_kernel_types.h"

namespace sigma {
namespace config {

/* ---- Target Editions ---- */
typedef enum {
    SIGMA_EDITION_STANDALONE  = 0,
    SIGMA_EDITION_IOT         = 1, /* Feature #88 */
    SIGMA_EDITION_MICROKERNEL = 2, /* Feature #96 */
    SIGMA_EDITION_CLOUD       = 3, /* Feature #91 */
    SIGMA_EDITION_RESEARCH    = 4  /* Feature #89 */
} sigma_edition_t;

/* Global defined at link-time by the build matrix */
extern "C" sigma_edition_t __sigma_active_edition;

/**
 * @brief Validates if a specific subsystem is permitted in the active edition.
 */
sigma_bool is_subsystem_allowed(const char* subsystem_name) {
    switch (__sigma_active_edition) {
        case SIGMA_EDITION_IOT:
            /* Block GUI and heavy network stacks */
            if (__builtin_strcmp(subsystem_name, "zenith_gui") == 0) return SIGMA_FALSE;
            if (__builtin_strcmp(subsystem_name, "virtualization") == 0) return SIGMA_FALSE;
            break;
            
        case SIGMA_EDITION_MICROKERNEL:
            /* Strictly limit to 120-shard core logic */
            if (__builtin_strcmp(subsystem_name, "legacy_compat") == 0) return SIGMA_FALSE;
            if (__builtin_strcmp(subsystem_name, "monolithic_fs") == 0) return SIGMA_FALSE;
            break;
            
        case SIGMA_EDITION_RESEARCH:
            /* Bypass standard sandboxing constraints for raw compute */
            // Note: In Research edition, all subsystems allowed, sandboxes disabled
            break;

        default:
            break;
    }
    return SIGMA_TRUE;
}

} /* namespace config */
} /* namespace sigma */

/* ---- C Bridge ---- */
extern "C" {
    sigma_bool sigma_edition_check_subsystem(const char* name) {
        return sigma::config::is_subsystem_allowed(name);
    }
}
