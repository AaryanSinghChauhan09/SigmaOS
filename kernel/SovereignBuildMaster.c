#include "SovereignLibC.h"
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
    HANDLE hFile = CreateFileA(path, GENERIC_READ, FILE_SHARE_READ, NULL, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
    if (hFile == INVALID_HANDLE_VALUE) return;

    DWORD fileSize = GetFileSize(hFile, NULL);
    if (fileSize == 0 || fileSize == INVALID_FILE_SIZE) {
        CloseHandle(hFile);
        return;
    }

    char* buffer = (char*)sigma_malloc(fileSize + 1);
    DWORD read;
    if (ReadFile(hFile, buffer, fileSize, &read, NULL)) {
        buffer[read] = '\0';
        char* line = buffer;
        int lineno = 0;
        char* next_line;

        while (line && *line) {
            lineno++;
            next_line = (char*)sigma_strstr(line, "\n");
            if (next_line) *next_line = '\0';

            for (int i = 0; i < 16; i++) {
                if (sigma_strstr(line, FORBIDDEN[i])) {
                    sigma_printf("  %s✗%s  %s:%d  →  %s\n", C_RED, C_RESET, path, lineno, line);
                }
            }

            if (next_line) line = next_line + 1;
            else line = NULL;
        }
    }

    sigma_free(buffer);
    CloseHandle(hFile);
}

void scan_dir(const char* dir_path) {
    char search_path[MAX_PATH];
    sigma_snprintf(search_path, MAX_PATH, "%s\\*", dir_path);

    WIN32_FIND_DATA find_data;
    HANDLE hFind = FindFirstFile(search_path, &find_data);

    if (hFind == INVALID_HANDLE_VALUE) return;

    do {
        if (sigma_streq(find_data.cFileName, ".") || sigma_streq(find_data.cFileName, ".."))
            continue;

        char path[MAX_PATH];
        sigma_snprintf(path, MAX_PATH, "%s\\%s", dir_path, find_data.cFileName);

        if (find_data.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) {
            scan_dir(path);
        } else {
            const char* ext = sigma_strrchr(find_data.cFileName, '.');
            if (ext && (sigma_streq(ext, ".c") || sigma_streq(ext, ".h"))) {
                verify_file(path);
            }
        }
    } while (FindNextFile(hFind, &find_data));

    FindClose(hFind);
}

int main_build_master(int argc, char** argv) {
    sigma_printf("\n%sΣ SOVEREIGN BUILD MASTER (v11.0 WIN32)%s\n", C_BOLD, C_RESET);
    sigma_printf("Scanning for sovereignty violations...\n");
    
    scan_dir(".");
    
    sigma_printf("\n%s[OK]%s Sovereignty Audit Complete.\n\n", C_GREEN, C_RESET);
    return 0;
}
