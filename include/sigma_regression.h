/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN HARDWARE REGRESSION MATRIX (S-REGRESS)
 * =========================================================================
 * Mission: Nightly hardware-level regression and certification.
 * Inspired by Ubuntu / Enterprise Regression Pipelines.
 * =========================================================================
 */

#ifndef SIGMA_REGRESSION_H
#define SIGMA_REGRESSION_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    REGRESSION_PASS,
    REGRESSION_FAIL,
    REGRESSION_WARN
} sigma_regress_status_t;

typedef struct {
    char component_name[64];
    sigma_regress_status_t status;
    sigma_u32 execution_time_ms;
} sigma_regress_result_t;

/* --- Regression Primitives --- */
void      regress_init(void);
void      regress_run_matrix(void);
void      regress_verify_peripheral(const char* id);
void      regress_report_certification(void);

#ifdef __cplusplus
}

namespace SigmaOS {
namespace Kernel {
namespace Certification {

class SovereignRegressionMatrix {
public:
    static SovereignRegressionMatrix& getInstance() {
        static SovereignRegressionMatrix instance;
        return instance;
    }

    void init();
    void runMatrix();
    void verifyComponent(const char* name);
    void report();

private:
    SovereignRegressionMatrix() : m_tests_passed(0), m_tests_failed(0) {}
    sigma_u32 m_tests_passed;
    sigma_u32 m_tests_failed;
};

} // namespace Certification
} // namespace Kernel
} // namespace SigmaOS
#endif

#endif /* SIGMA_REGRESSION_H */
