#include "suites/S01_Genesis/shards/sigma_libc.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"
#include "suites/S01_Genesis/shards/sigma_types.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"

/* Mocking sigma_ kernel functions for host-side simulation */
void sigma_printf(const char* format, ...) {
    va_list args;
    va_start(args, format);
    vprintf(format, args);
    va_end(args);
}

void* sigma_malloc(size_t size) { return sigma_malloc(size); }
void  sigma_free(void* ptr) { sigma_free(ptr); }
void  sigma_memset(void* s, int c, size_t n) { sigma_memset(s, c, n); }
void  sigma_memcpy(void* d, const void* s, size_t n) { sigma_memcpy(d, s, n); }
int   sigma_strcmp(const char* s1, const char* s2) { return sigma_strcmp(s1, s2); }
int   sigma_streq(const char* s1, const char* s2) { return sigma_strcmp(s1, s2) == 0; }
char* sigma_strcpy(char* dest, const char* src) { return sigma_strcpy(dest, src); }
char* sigma_strncpy(char* dest, const char* src, size_t n) { return sigma_strncpy(dest, src, n); }
const char* sigma_strstr(const char* h, const char* n) { return strstr(h, n); }

/* Simulation entry */
extern void kmain(void);

int main() {
    sigma_printf("[SIMULATOR]: Starting SigmaOS Sovereign Simulation...\n");
    kmain();
    return 0;
}
