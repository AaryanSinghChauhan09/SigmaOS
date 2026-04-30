#ifndef SIGMA_RECOVER_H
#define SIGMA_RECOVER_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    SIGMA_RECOVER_HEALTHY = 0,
    SIGMA_RECOVER_HEALING = 1,
    SIGMA_RECOVER_CRITICAL = 2
} sigma_recovery_state_t;

typedef struct {
    uint32_t shard_id;
    uint32_t heal_count;
    bool permanent_failure;
} sigma_recovery_record_t;

/* --- Recovery Primitives --- */
void recover_init(void);
void recover_trigger_healing(uint32_t shard_id);
sigma_recovery_state_t recover_get_lattice_state(void);

#ifdef __cplusplus
}

class SovereignRecover {
public:
    static SovereignRecover& getInstance() {
        static SovereignRecover instance;
        return instance;
    }

    void init();
    void triggerHealing(uint32_t shard_id);
    sigma_recovery_state_t getLatticeState() const;
    void setLatticeState(sigma_recovery_state_t state);

private:
    SovereignRecover() : lattice_state(SIGMA_RECOVER_HEALTHY), registry_ptr(0) {}
    
    sigma_recovery_state_t lattice_state;
    sigma_recovery_record_t healing_registry[32];
    uint32_t registry_ptr;
};
#endif

#endif /* SIGMA_RECOVER_H */
