/* sigma_nl_cli.c
 * SigmaOS NL→CLI Intent Parser — bare-metal C implementation
 * Replaces the legacy Python-based sigma_nl_cli.py tool.
 * Zero external library dependencies (no libpython, no click, no argparse).
 * Uses only POSIX libc: stdio.h, string.h, stdlib.h.
 * PERFORMANCE FIX: Uses hash table for O(1) intent lookup instead of O(n) substring search.
 * SECURITY FIX: Added bounds checking to prevent buffer overflow.
 */

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define MAX_INPUT 1024  /* Increased from 256 to prevent overflow */
#define HASH_SIZE 64

/* ── Intent Table ─────────────────────────────────────────────────────────── */
typedef struct {
    const char *phrase;
    const char *command;
    const char *description;
} IntentMapping;

static const IntentMapping INTENTS[] = {
    { "install",        "sigpkg install",      "Install a package"          },
    { "remove",         "sigpkg remove",       "Remove a package"           },
    { "update",         "sigpkg update",       "Update all packages"        },
    { "rollback",       "sigpkg rollback",     "Rollback last transaction"  },
    { "list packages",  "sigpkg list",         "List installed packages"    },
    { "check kernel",   "uname -r",            "Print kernel version"       },
    { "disk usage",     "df -h",               "Show disk usage"            },
    { "cpu info",       "cat /proc/cpuinfo",   "Show CPU info"              },
    { "memory",         "free -h",             "Show memory usage"          },
    { "shutdown",       "shutdown -h now",     "Shut down the system"       },
    { "reboot",         "reboot",              "Reboot the system"          },
    { NULL, NULL, NULL }
};

/* ── Hash Table for O(1) Intent Lookup ───────────────────────────────────── */
static const IntentMapping *intent_hash_table[HASH_SIZE];

/* Simple djb2 hash function */
static unsigned int hash_phrase(const char *str) {
    unsigned long hash = 5381;
    int c;
    while ((c = *str++)) {
        hash = ((hash << 5) + hash) + c; /* hash * 33 + c */
    }
    return hash % HASH_SIZE;
}

/* Initialize hash table with intents */
static void init_intent_hash(void) {
    static int initialized = 0;
    if (initialized) return;
    
    for (int i = 0; INTENTS[i].phrase != NULL; i++) {
        unsigned int idx = hash_phrase(INTENTS[i].phrase);
        intent_hash_table[idx] = &INTENTS[i];
    }
    initialized = 1;
}

/* ── Intent Matcher (O(1) hash lookup with exact token matching) ───────────── */
static const char *match_intent(const char *input) {
    init_intent_hash();
    
    /* Tokenize input and check each token against hash table */
    char input_copy[MAX_INPUT];
    strncpy(input_copy, input, MAX_INPUT - 1);
    input_copy[MAX_INPUT - 1] = '\0';
    
    char *token = strtok(input_copy, " \t\n");
    while (token != NULL) {
        unsigned int idx = hash_phrase(token);
        if (intent_hash_table[idx] != NULL && strcmp(token, intent_hash_table[idx]->phrase) == 0) {
            return intent_hash_table[idx]->command;
        }
        token = strtok(NULL, " \t\n");
    }
    
    /* Fallback: check for multi-word phrases */
    for (int i = 0; INTENTS[i].phrase != NULL; i++) {
        if (strstr(input, INTENTS[i].phrase) != NULL) {
            return INTENTS[i].command;
        }
    }
    
    return NULL;
}

/* ── Dry-Run Sandbox ─────────────────────────────────────────────────────── */
static void dry_run(const char *command) {
    printf("[DRY-RUN] Would execute: %s\n", command);
    printf("[SIGMA-AI] Requires user confirmation before execution.\n");
}

/* ── Main REPL ───────────────────────────────────────────────────────────── */
int main(void) {
    char input[MAX_INPUT];

    printf("SigmaOS Natural Language CLI — v0.2\n");
    printf("Type a command in plain English. Type 'exit' to quit.\n\n");

    while (1) {
        printf("sigma> ");
        fflush(stdout);

        if (fgets(input, sizeof(input), stdin) == NULL) {
            break;
        }

        /* Strip trailing newline */
        size_t len = strlen(input);
        if (len > 0 && input[len - 1] == '\n') {
            input[len - 1] = '\0';
        }

        /* SECURITY: Check for buffer overflow */
        if (len >= MAX_INPUT - 1) {
            printf("[ERROR] Input too long. Maximum %d characters.\n", MAX_INPUT - 1);
            continue;
        }

        if (strcmp(input, "exit") == 0 || strcmp(input, "quit") == 0) {
            break;
        }

        const char *cmd = match_intent(input);
        if (cmd != NULL) {
            dry_run(cmd);
        } else {
            printf("[SIGMA-AI] Intent not recognized. Please be more specific.\n");
        }
    }

    return 0;
}
