#ifndef SIGMA_ASSISTANT_H
#define SIGMA_ASSISTANT_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    char user_name[32];
    bool voice_active;
    uint32_t intelligence_level;
} sigma_assistant_config_t;

/* --- Assistant Primitives --- */
void     assistant_init(void);
void     assistant_query(const char* prompt);
void     assistant_report_status(void);
sigma_u32 assistant_get_query_count(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_ASSISTANT_H */
