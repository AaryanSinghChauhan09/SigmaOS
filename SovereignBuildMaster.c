/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN BUILD MASTER (v11.0 - WINDOWS SILICON FORGE)
 * =============================================================================
 * Purpose: Low-level build verification and statistics engine (Win32 Native).
 * Principle: Zero-Dependency. Pure C11. Direct System Logic.
 * =============================================================================
 */

#include <stdio.h>
#include <string.h>
#include <windows.h>

#define C_BOLD   "\033[1m"
#define C_RESET  "\033[0m"
#define C_GREEN  "\033[0;32m"
#define C_RED    "\033[0;31m"

const char* FORBIDDEN[] = {
    "<iostream>", "<string>", "<vector>", "<map>", "<set>",
    "<algorithm>", "<memory>", "<functional>", "<thread>",
    "<mutex>", "<condition_variable>", "<chrono>",
    "<stdexcept>", "<exception>", "stdlib.h", "stdio.h"
};

void verify_file(const char* path) {
    FILE* f = fopen(path, "r");
    if (!f) return;

    char line[1024];
    int lineno = 0;
    while (fgets(line, sizeof(line), f)) {
        lineno++;
        for (int i = 0; i < 16; i++) {
            if (strstr(line, FORBIDDEN[i])) {
                printf("  %s✗%s  %s:%d  →  %s", C_RED, C_RESET, path, lineno, line);
            }
        }
    }
    fclose(f);
}

void scan_dir(const char* dir_path) {
    char search_path[MAX_PATH];
    snprintf(search_path, MAX_PATH, "%s\\*", dir_path);

    WIN32_FIND_DATA find_data;
    HANDLE hFind = FindFirstFile(search_path, &find_data);

    if (hFind == INVALID_HANDLE_VALUE) return;

    do {
        if (strcmp(find_data.cFileName, ".") == 0 || strcmp(find_data.cFileName, "..") == 0)
            continue;

        char path[MAX_PATH];
        snprintf(path, MAX_PATH, "%s\\%s", dir_path, find_data.cFileName);

        if (find_data.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) {
            scan_dir(path);
        } else {
            const char* ext = strrchr(find_data.cFileName, '.');
            if (ext && (strcmp(ext, ".c") == 0 || strcmp(ext, ".h") == 0)) {
                verify_file(path);
            }
        }
    } while (FindNextFile(hFind, &find_data));

    FindClose(hFind);
}

int main(int argc, char** argv) {
    printf("\n%sΣ SOVEREIGN BUILD MASTER (v11.0 WIN32)%s\n", C_BOLD, C_RESET);
    printf("Scanning for sovereignty violations...\n");
    
    scan_dir(".");
    
    printf("\n%s[OK]%s Sovereignty Audit Complete.\n\n", C_GREEN, C_RESET);
    return 0;
}
