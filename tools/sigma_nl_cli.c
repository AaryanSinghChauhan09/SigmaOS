/* sigma_nl_cli.c
 * SigmaOS NL→CLI Intent Parser — bare-metal C implementation
 * Replaces the legacy Python-based sigma_nl_cli.py tool.
 * Zero external library dependencies (no libpython, no click, no argparse).
 * Uses only POSIX libc: stdio.h, string.h, stdlib.h.
 */

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

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

/* ── Intent Matcher ──────────────────────────────────────────────────────── */
static const char *match_intent(const char *input) {
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
    char input[256];

    printf("SigmaOS Natural Language CLI — v0.1\n");
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
