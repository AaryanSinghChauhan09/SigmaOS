#ifndef SIGMA_DIAG_H
#define SIGMA_DIAG_H

#include "sigma_types.h"

typedef struct {
    sigma_u32 component_id;
    sigma_u32 error_vector;
    sigma_u32 silicon_tick;
    bool      is_critical;
} sigma_diag_event_t;

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace Observability {

class SovereignDiagEngine {
public:
    static SovereignDiagEngine& getInstance();

    void init();
    void reportFault(sigma_u32 component_id, sigma_u32 error_code);
    void localizeFault();

private:
    SovereignDiagEngine() : fault_count(0) {}
    
    sigma_diag_event_t fault_lattice[256];
    sigma_u32          fault_count;
};

} // namespace Observability
} // namespace Kernel
} // namespace SigmaOS
#endif

#ifdef __cplusplus
extern "C" {
#endif

/* --- Diagnostic Primitives --- */
void      diag_init(void);
void      diag_report_fault(sigma_u32 component_id, sigma_u32 error_code);
void      diag_localize_fault(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_DIAG_H */
