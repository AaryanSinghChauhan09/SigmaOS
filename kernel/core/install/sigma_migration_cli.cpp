/**
 * ===========================================================================
 * Σ SIGMAOS: MIGRATION CLI (sigma-migrate)
 * ===========================================================================
 * Mission: Command-line wizard for importing data from Windows/Ubuntu into SigmaOS.
 * ===========================================================================
 */

#include "../../../include/sigma_log.h"
#include <stdio.h>
#include <string.h>

extern "C" void migration_init();
extern "C" bool migration_run(const char* target_partition, bool browsers, bool ides, bool shell, bool files);

void print_banner() {
    printf("\n");
    printf("  __  __ _                 _       \n");
    printf(" |  \\/  (_)               | |      \n");
    printf(" | \\  / |_  __ _ _ __ __ _| |_ ___ \n");
    printf(" | |\\/| | |/ _` | '__/ _` | __/ _ \\\n");
    printf(" | |  | | | (_| | | | (_| | ||  __/\n");
    printf(" |_|  |_|_|\\__, |_|  \\__,_|\\__\\___|\n");
    printf("            __/ |                  \n");
    printf("           |___/  Migration Assistant v1.0\n");
    printf("\n");
}

int main(int argc, char** argv) {
    print_banner();
    migration_init();

    if (argc < 2) {
        printf("Usage: sigma_migration_cli <target_partition> [options]\n");
        printf("Options (Include what you want to migrate):\n");
        printf("  --all       Migrate everything (Browsers, IDEs, Shell, Files)\n");
        printf("  --browsers  Migrate Firefox/Chrome profiles\n");
        printf("  --ides      Migrate VS Code / JetBrains settings\n");
        printf("  --shell     Migrate .bashrc / .zshrc (Linux only)\n");
        printf("  --files     Migrate Documents, Pictures, Downloads\n");
        return 1;
    }

    const char* partition = argv[1];
    bool browsers = false, ides = false, shell = false, files = false;

    if (argc == 2 || (argc > 2 && strcmp(argv[2], "--all") == 0)) {
        browsers = ides = shell = files = true;
    } else {
        for (int i = 2; i < argc; i++) {
            if (strcmp(argv[i], "--browsers") == 0) browsers = true;
            if (strcmp(argv[i], "--ides") == 0) ides = true;
            if (strcmp(argv[i], "--shell") == 0) shell = true;
            if (strcmp(argv[i], "--files") == 0) files = true;
        }
    }

    migration_run(partition, browsers, ides, shell, files);

    return 0;
}
