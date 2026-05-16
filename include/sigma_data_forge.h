/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN DATA FORGE (S-DATAFORGE)
 * =========================================================================
 * Mission: Lattice-scale distributed data processing and analytics.
 * Replaces: Apache Spark, Pandas, Dask, Airflow.
 * =========================================================================
 */

#ifndef SIGMA_DATA_FORGE_H
#define SIGMA_DATA_FORGE_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    FORGE_OP_MAP,
    FORGE_OP_REDUCE,
    FORGE_OP_FILTER,
    FORGE_OP_TRANSFORM
} sigma_forge_op_t;

typedef struct {
    sigma_u32 shard_id;
    sigma_u32 partition_id;
    sigma_size_t record_count;
} sigma_forge_partition_t;

/* --- Data Forge Primitives --- */
void      forge_init(void);
void      forge_dispatch_parallel(sigma_forge_op_t op, const void* dataset, sigma_size_t size);
void      forge_wait_all(void);
sigma_u64 forge_get_processed_bytes(void);

#ifdef __cplusplus
}

namespace SigmaOS {
namespace Kernel {
namespace Data {

class SovereignDataForge {
public:
    static SovereignDataForge& getInstance() {
        static SovereignDataForge instance;
        return instance;
    }

    void init();
    void dispatch(sigma_forge_op_t op, const void* data, sigma_size_t size);
    void reportStatus();

private:
    SovereignDataForge() : m_processed_bytes(0), m_active_pipelines(0) {}
    sigma_u64 m_processed_bytes;
    sigma_u32 m_active_pipelines;
};

} // namespace Data
} // namespace Kernel
} // namespace SigmaOS
#endif

#endif /* SIGMA_DATA_FORGE_H */
