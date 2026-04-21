#include "sigma_libc.h"
#include "sigma_types.h"

/**
 * SigmaOS SigmaNLP (Natural Language Parser)
 * Subsystem: S02 (Zenith UI)
 * Mission: Lower the learning curve by mapping English directives to Sovereign CLI commands.
 */

typedef struct {
    char phrase[64];
    char command[64];
} NLPMapping;

static NLPMapping nlp_table[] = {
    {"install",         "sigpkg install"},
    {"find",            "sigsearch"},
    {"remove",          "sigpkg remove"},
    {"update",          "sigupdate"},
    {"how is system",   "sigmon"},
    {"clean memory",    "sigpurge mem"},
    {"fix audit",       "sig-audit --fix"}
};

void sigma_nlp_parse(const char* input, char* output_cmd) {
    uint32_t count = 7;
    for (uint32_t i = 0; i < count; i++) {
        if (sigma_strstr(input, nlp_table[i].phrase) != SIGMA_NULL) {
            sigma_strcpy(output_cmd, nlp_table[i].command);
            
            // Append the rest of the arguments if any
            const char* args = sigma_strstr(input, nlp_table[i].phrase) + sigma_strlen(nlp_table[i].phrase);
            sigma_strncat(output_cmd, args, 32);
            return;
        }
    }
    // Fallback to direct passthrough
    sigma_strcpy(output_cmd, input);
}

void S02_Register_NLP(void) {
    sigma_printf("S02 [ZENITH UI]: SigmaNLP (Natural Language Parser) Online.\n");
}
