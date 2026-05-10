#ifndef SIGMA_PLUGIN_API_H
#define SIGMA_PLUGIN_API_H

#include "core/sigma_types.h"

typedef struct {
    const char* name;
    const char* version;
    void (*init)(void);
    void (*shutdown)(void);
} sigma_shard_plugin_t;

void sigma_register_plugin(sigma_shard_plugin_t* plugin);

#endif
