/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN CLOUD & DISTRIBUTED STORAGE (S-CLOUD)
 * =========================================================================
 * Mission: Shard-based distributed storage and cloud orchestration.
 * Inspired by Ceph / MinIO / OpenStack.
 * =========================================================================
 */

#ifndef SIGMA_CLOUD_H
#define SIGMA_CLOUD_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    CLOUD_NODE_MASTER,
    CLOUD_NODE_WORKER,
    CLOUD_NODE_STORAGE
} sigma_cloud_node_type_t;

/* --- Cloud Primitives --- */
void      cloud_init(void);
void      cloud_join_lattice(const char* cluster_secret);
void      cloud_replicate_shard(const char* shard_id, sigma_u32 redundancy);
void      cloud_report_cluster_stats(void);

#ifdef __cplusplus
}

namespace SigmaOS {
namespace Kernel {
namespace Cloud {

class SovereignCloudNexus {
public:
    static SovereignCloudNexus& getInstance() {
        static SovereignCloudNexus instance;
        return instance;
    }

    void init();
    void join(const char* secret);
    void replicate(const char* id, sigma_u32 redundancy);
    void reportStats();

private:
    SovereignCloudNexus() : m_node_type(CLOUD_NODE_MASTER) {}
    sigma_cloud_node_type_t m_node_type;
};

} // namespace Cloud
} // namespace Kernel
} // namespace SigmaOS
#endif

#endif /* SIGMA_CLOUD_H */
