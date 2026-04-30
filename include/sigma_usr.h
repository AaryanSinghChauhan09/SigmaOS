#ifndef SIGMA_USR_H
#define SIGMA_USR_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t shard_id;
    char name[64];
    bool is_active;
    uint32_t quantum_key; // For amnesic-protected discovery
} sigma_usr_entry_t;

/* --- USR Primitives --- */
void usr_init(void);
uint32_t usr_register_shard(const char* name, uint32_t quantum_key);
bool usr_activate_shard(uint32_t shard_id);

#ifdef __cplusplus
}

class SovereignUSRManager {
public:
    static SovereignUSRManager& getInstance() {
        static SovereignUSRManager instance;
        return instance;
    }

    void init();
    uint32_t registerShard(const char* name, uint32_t quantum_key);
    bool activateShard(uint32_t shard_id);

private:
    SovereignUSRManager() : count(0) {}
    
    sigma_usr_entry_t registry[512];
    uint32_t count;
};

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_USR_H */
