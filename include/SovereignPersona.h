#ifndef SOVEREIGN_PERSONA_H
#define SOVEREIGN_PERSONA_H

#include "./libc/SovereignLibC.h"

typedef struct SovereignPersona {
    char name[32];
    char theme[16];
    char dashboard_layout[32];
    sigma_bool ai_personalization;
} SovereignPersona;

static void persona_init(SovereignPersona* p, const char* name, const char* theme) {
    sigma_memset(p, 0, sizeof(*p));
    sigma_strcpy(p->name, name, sizeof(p->name));
    sigma_strcpy(p->theme, theme, sizeof(p->theme));
    sigma_strcpy(p->dashboard_layout, "DEFAULT_ZENITH", sizeof(p->dashboard_layout));
    p->ai_personalization = SIGMA_TRUE;
}

#endif
