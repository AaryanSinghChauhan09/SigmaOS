#include "../../include/sigma_vfs.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"

/* =========================================================================
 * VFS (VIRTUAL FILE SYSTEM) KERNEL ROUTINES
 * Abstracts away explicit fopen/fread logic from the network layers.
 * ========================================================================= */

const char* sigma_vfs_resolve_path(const char* request_path, char* resolved_buffer, int buffer_size) {
    if (buffer_size < 1024) return NULL;
    
    sigma_strcpy(resolved_buffer, "../web_ui/");
    int len = 0; while (resolved_buffer[len]) len++;
    if (sigma_strcmp(request_path, "/") == 0) {
        strncat(resolved_buffer, "index.html", buffer_size - len - 1);
    } else {
        strncat(resolved_buffer, request_path + 1, buffer_size - len - 1); // Ignore leading slash
    }
    resolved_buffer[buffer_size - 1] = '\0';
    return resolved_buffer;
}

char* sigma_vfs_read_file(const char* path, long* out_size) {
    FILE* file = fopen(path, "rb");
    if (!file) {
        *out_size = 0;
        return NULL;
    }

    fseek(file, 0, SEEK_END);
    *out_size = ftell(file);
    fseek(file, 0, SEEK_SET);

    if (*out_size < 0) {
        fclose(file);
        *out_size = 0;
        return NULL;
    }

    char* content = (char*)sigma_malloc(*out_size + 1);
    if (!content) {
        fclose(file);
        *out_size = 0;
        return NULL;
    }

    fread(content, 1, *out_size, file);
    content[*out_size] = '\0'; // Null-terminate safely
    fclose(file);
    
    return content;
}
