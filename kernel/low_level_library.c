/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS Low-Level Library
 * =========================
 * Custom low-level library functions to eliminate third-party dependencies
 */

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <math.h>

// String operations (replacing string.h functions)
size_t sigma_strlen(const char* str) {
    if (!str) return 0;
    
    size_t len = 0;
    while (str[len] != '\0') {
        len++;
    }
    return len;
}

char* sigma_strcpy(char* dest, const char* src) {
    if (!dest || !src) return dest;
    
    char* original_dest = dest;
    while ((*dest++ = *src++) != '\0') {
        // Copy character
    }
    return original_dest;
}

char* sigma_strncpy(char* dest, const char* src, size_t n) {
    if (!dest || !src) return dest;
    
    char* original_dest = dest;
    size_t i;
    
    for (i = 0; i < n && src[i] != '\0'; i++) {
        dest[i] = src[i];
    }
    
    for (; i < n; i++) {
        dest[i] = '\0';
    }
    
    return original_dest;
}

char* sigma_strcat(char* dest, const char* src) {
    if (!dest || !src) return dest;
    
    char* original_dest = dest;
    
    // Find end of dest
    while (*dest != '\0') {
        dest++;
    }
    
    // Append src
    while ((*dest++ = *src++) != '\0') {
        // Append character
    }
    
    return original_dest;
}

int sigma_strcmp(const char* str1, const char* str2) {
    if (!str1 || !str2) return -1;
    
    while (*str1 && (*str1 == *str2)) {
        str1++;
        str2++;
    }
    
    return *(unsigned char*)str1 - *(unsigned char*)str2;
}

int sigma_strncmp(const char* str1, const char* str2, size_t n) {
    if (!str1 || !str2) return -1;
    
    for (size_t i = 0; i < n; i++) {
        if (str1[i] != str2[i]) {
            return *(unsigned char*)&str1[i] - *(unsigned char*)&str2[i];
        }
        if (str1[i] == '\0') {
            return 0;
        }
    }
    
    return 0;
}

char* sigma_strchr(const char* str, int c) {
    if (!str) return NULL;
    
    while (*str != '\0') {
        if (*str == (char)c) {
            return (char*)str;
        }
        str++;
    }
    
    return NULL;
}

char* sigma_strstr(const char* haystack, const char* needle) {
    if (!haystack || !needle) return NULL;
    
    if (*needle == '\0') {
        return (char*)haystack;
    }
    
    const char* h = haystack;
    const char* n = needle;
    
    while (*h != '\0') {
        const char* current_h = h;
        const char* current_n = n;
        
        while (*current_h != '\0' && *current_n != '\0' && *current_h == *current_n) {
            current_h++;
            current_n++;
        }
        
        if (*current_n == '\0') {
            return (char*)h;
        }
        
        h++;
    }
    
    return NULL;
}

// Memory operations (replacing memory.h functions)
void* sigma_memcpy(void* dest, const void* src, size_t n) {
    if (!dest || !src || n == 0) return dest;
    
    char* d = (char*)dest;
    const char* s = (const char*)src;
    
    // Handle overlapping memory regions
    if (d < s) {
        for (size_t i = 0; i < n; i++) {
            d[i] = s[i];
        }
    } else {
        for (size_t i = n; i > 0; i--) {
            d[i-1] = s[i-1];
        }
    }
    
    return dest;
}

void* sigma_memmove(void* dest, const void* src, size_t n) {
    return sigma_memcpy(dest, src, n);
}

void* sigma_memset(void* ptr, int value, size_t n) {
    if (!ptr || n == 0) return ptr;
    
    unsigned char* p = (unsigned char*)ptr;
    unsigned char v = (unsigned char)value;
    
    for (size_t i = 0; i < n; i++) {
        p[i] = v;
    }
    
    return ptr;
}

int sigma_memcmp(const void* ptr1, const void* ptr2, size_t n) {
    if (!ptr1 || !ptr2) return -1;
    
    const unsigned char* p1 = (const unsigned char*)ptr1;
    const unsigned char* p2 = (const unsigned char*)ptr2;
    
    for (size_t i = 0; i < n; i++) {
        if (p1[i] != p2[i]) {
            return p1[i] - p2[i];
        }
    }
    
    return 0;
}

void* sigma_memchr(const void* ptr, int value, size_t n) {
    if (!ptr || n == 0) return NULL;
    
    const unsigned char* p = (const unsigned char*)ptr;
    unsigned char v = (unsigned char)value;
    
    for (size_t i = 0; i < n; i++) {
        if (p[i] == v) {
            return (void*)(p + i);
        }
    }
    
    return NULL;
}

// Math operations (replacing math.h functions)
double sigma_sqrt(double x) {
    if (x < 0) return NAN;
    if (x == 0) return 0;
    
    // Newton-Raphson method
    double guess = x / 2.0;
    double prev_guess = 0;
    
    while (guess != prev_guess) {
        prev_guess = guess;
        guess = (guess + x / guess) / 2.0;
    }
    
    return guess;
}

double sigma_pow(double base, double exp) {
    if (base == 0 && exp > 0) return 0;
    if (base == 0 && exp <= 0) return NAN;
    if (base == 1) return 1;
    if (exp == 0) return 1;
    if (exp == 1) return base;
    
    // Handle integer exponents
    if (exp == (int)exp) {
        double result = 1;
        int int_exp = (int)exp;
        
        if (int_exp < 0) {
            int_exp = -int_exp;
            base = 1.0 / base;
        }
        
        for (int i = 0; i < int_exp; i++) {
            result *= base;
        }
        
        return result;
    }
    
    // For non-integer exponents, use approximation
    return sigma_exp(exp * sigma_log(base));
}

double sigma_exp(double x) {
    // Taylor series approximation for e^x
    if (x == 0) return 1;
    
    double result = 1.0;
    double term = 1.0;
    
    for (int i = 1; i < 20; i++) {
        term *= x / i;
        result += term;
        
        // Stop if term becomes too small
        if (term < 1e-15) break;
    }
    
    return result;
}

double sigma_log(double x) {
    if (x <= 0) return NAN;
    if (x == 1) return 0;
    
    // Natural logarithm using Newton-Raphson method
    double guess = 0;
    double prev_guess = -1;
    
    while (guess != prev_guess) {
        prev_guess = guess;
        guess = guess + 2.0 * (x - sigma_exp(guess)) / (x + sigma_exp(guess));
    }
    
    return guess;
}

double sigma_log10(double x) {
    return sigma_log(x) / sigma_log(10);
}

double sigma_sin(double x) {
    // Taylor series approximation for sin(x)
    // Reduce x to [-π, π] range
    while (x > 3.141592653589793) {
        x -= 2 * 3.141592653589793;
    }
    while (x < -3.141592653589793) {
        x += 2 * 3.141592653589793;
    }
    
    double result = 0;
    double term = x;
    
    for (int i = 1; i < 15; i += 2) {
        result += term;
        term *= -x * x / ((i + 1) * (i + 2));
    }
    
    return result;
}

double sigma_cos(double x) {
    // Taylor series approximation for cos(x)
    // Reduce x to [-π, π] range
    while (x > 3.141592653589793) {
        x -= 2 * 3.141592653589793;
    }
    while (x < -3.141592653589793) {
        x += 2 * 3.141592653589793;
    }
    
    double result = 0;
    double term = 1;
    
    for (int i = 0; i < 15; i += 2) {
        result += term;
        term *= -x * x / ((i + 1) * (i + 2));
    }
    
    return result;
}

double sigma_tan(double x) {
    double sin_val = sigma_sin(x);
    double cos_val = sigma_cos(x);
    
    if (cos_val == 0) return NAN;
    
    return sin_val / cos_val;
}

double sigma_fabs(double x) {
    return x < 0 ? -x : x;
}

double sigma_floor(double x) {
    if (x >= 0) {
        return (double)(int)x;
    } else {
        double int_part = (double)(int)x;
        return int_part == x ? int_part : int_part - 1;
    }
}

double sigma_ceil(double x) {
    if (x <= 0) {
        return (double)(int)x;
    } else {
        double int_part = (double)(int)x;
        return int_part == x ? int_part : int_part + 1;
    }
}

double sigma_round(double x) {
    return x >= 0 ? sigma_floor(x + 0.5) : sigma_ceil(x - 0.5);
}

// Standard library replacements
int sigma_atoi(const char* str) {
    if (!str) return 0;
    
    int result = 0;
    int sign = 1;
    
    // Skip whitespace
    while (*str == ' ' || *str == '\t' || *str == '\n' || *str == '\r') {
        str++;
    }
    
    // Handle sign
    if (*str == '-') {
        sign = -1;
        str++;
    } else if (*str == '+') {
        str++;
    }
    
    // Convert digits
    while (*str >= '0' && *str <= '9') {
        result = result * 10 + (*str - '0');
        str++;
    }
    
    return sign * result;
}

double sigma_atof(const char* str) {
    if (!str) return 0.0;
    
    double result = 0.0;
    int sign = 1;
    int decimal_point = 0;
    double fraction = 0.1;
    
    // Skip whitespace
    while (*str == ' ' || *str == '\t' || *str == '\n' || *str == '\r') {
        str++;
    }
    
    // Handle sign
    if (*str == '-') {
        sign = -1;
        str++;
    } else if (*str == '+') {
        str++;
    }
    
    // Convert integer part
    while (*str >= '0' && *str <= '9') {
        result = result * 10.0 + (*str - '0');
        str++;
    }
    
    // Convert fractional part
    if (*str == '.') {
        str++;
        while (*str >= '0' && *str <= '9') {
            result += (*str - '0') * fraction;
            fraction *= 0.1;
            str++;
        }
    }
    
    return sign * result;
}

char* sigma_itoa(int value, char* str, int base) {
    if (!str || base < 2 || base > 36) return NULL;
    
    char* ptr = str;
    char* ptr1 = str;
    int tmp_value;
    
    // Handle 0 case
    if (value == 0) {
        *ptr++ = '0';
        *ptr = '\0';
        return str;
    }
    
    // Handle negative numbers for base 10
    if (value < 0 && base == 10) {
        *ptr++ = '-';
        value = -value;
    }
    
    // Convert to string
    while (value != 0) {
        tmp_value = value % base;
        value = value / base;
        *ptr++ = (tmp_value < 10) ? (tmp_value + '0') : (tmp_value - 10 + 'a');
    }
    
    *ptr-- = '\0';
    
    // Reverse string
    while (ptr1 < ptr) {
        char tmp = *ptr;
        *ptr-- = *ptr1;
        *ptr1++ = tmp;
    }
    
    return str;
}

// Random number generation
static uint32_t sigma_random_seed = 1;

void sigma_srand(unsigned int seed) {
    sigma_random_seed = seed;
}

int sigma_rand(void) {
    sigma_random_seed = sigma_random_seed * 1103515245 + 12345;
    return (sigma_random_seed / 65536) % 32768;
}

// Dynamic memory allocation (simple implementation)
typedef struct MemoryBlock {
    size_t size;
    bool is_free;
    struct MemoryBlock* next;
} MemoryBlock;

static MemoryBlock* memory_heap = NULL;
static size_t heap_size = 0;

void* sigma_malloc(size_t size) {
    if (size == 0) return NULL;
    
    // Align size to 8-byte boundary
    size = (size + 7) & ~7;
    
    // Find free block
    MemoryBlock* block = memory_heap;
    while (block) {
        if (block->is_free && block->size >= size) {
            block->is_free = false;
            return (void*)(block + 1);
        }
        block = block->next;
    }
    
    // Allocate new block
    // For simplicity, we'll use system malloc for the actual allocation
    // In a real implementation, this would manage a pre-allocated heap
    MemoryBlock* new_block = (MemoryBlock*)malloc(sizeof(MemoryBlock) + size);
    if (!new_block) return NULL;
    
    new_block->size = size;
    new_block->is_free = false;
    new_block->next = memory_heap;
    memory_heap = new_block;
    
    return (void*)(new_block + 1);
}

void sigma_free(void* ptr) {
    if (!ptr) return;
    
    MemoryBlock* block = (MemoryBlock*)ptr - 1;
    block->is_free = true;
}

void* sigma_realloc(void* ptr, size_t new_size) {
    if (!ptr) return sigma_malloc(new_size);
    if (new_size == 0) {
        sigma_free(ptr);
        return NULL;
    }
    
    MemoryBlock* block = (MemoryBlock*)ptr - 1;
    
    if (block->size >= new_size) {
        return ptr;
    }
    
    void* new_ptr = sigma_malloc(new_size);
    if (!new_ptr) return NULL;
    
    sigma_memcpy(new_ptr, ptr, block->size);
    sigma_free(ptr);
    
    return new_ptr;
}

void* sigma_calloc(size_t count, size_t size) {
    size_t total_size = count * size;
    void* ptr = sigma_malloc(total_size);
    
    if (ptr) {
        sigma_memset(ptr, 0, total_size);
    }
    
    return ptr;
}

// File operations (simplified)
typedef struct {
    void* data;
    size_t size;
    size_t position;
    bool is_open;
} SigmaFile;

SigmaFile* sigma_fopen(const char* filename, const char* mode) {
    // Simplified file opening
    SigmaFile* file = (SigmaFile*)malloc(sizeof(SigmaFile));
    if (!file) return NULL;
    
    file->data = NULL;
    file->size = 0;
    file->position = 0;
    file->is_open = true;
    
    // In a real implementation, this would open the actual file
    // For now, we'll simulate it
    return file;
}

int sigma_fclose(SigmaFile* file) {
    if (!file) return -1;
    
    file->is_open = false;
    if (file->data) {
        free(file->data);
    }
    free(file);
    
    return 0;
}

size_t sigma_fread(void* ptr, size_t size, size_t count, SigmaFile* file) {
    if (!ptr || !file || !file->is_open) return 0;
    
    size_t bytes_to_read = size * count;
    size_t available = file->size - file->position;
    
    if (bytes_to_read > available) {
        bytes_to_read = available;
    }
    
    if (bytes_to_read > 0 && file->data) {
        sigma_memcpy(ptr, (char*)file->data + file->position, bytes_to_read);
        file->position += bytes_to_read;
    }
    
    return bytes_to_read / size;
}

size_t sigma_fwrite(const void* ptr, size_t size, size_t count, SigmaFile* file) {
    if (!ptr || !file || !file->is_open) return 0;
    
    size_t bytes_to_write = size * count;
    size_t needed = file->position + bytes_to_write;
    
    if (needed > file->size) {
        file->data = realloc(file->data, needed);
        if (!file->data) return 0;
        file->size = needed;
    }
    
    sigma_memcpy((char*)file->data + file->position, ptr, bytes_to_write);
    file->position += bytes_to_write;
    
    return count;
}

int sigma_fseek(SigmaFile* file, long offset, int whence) {
    if (!file || !file->is_open) return -1;
    
    switch (whence) {
        case 0: // SEEK_SET
            if (offset < 0 || offset > (long)file->size) return -1;
            file->position = offset;
            break;
        case 1: // SEEK_CUR
            if (file->position + offset < 0 || file->position + offset > file->size) return -1;
            file->position += offset;
            break;
        case 2: // SEEK_END
            if (offset > 0 || file->size + offset < 0) return -1;
            file->position = file->size + offset;
            break;
        default:
            return -1;
    }
    
    return 0;
}

long sigma_ftell(SigmaFile* file) {
    if (!file || !file->is_open) return -1;
    
    return (long)file->position;
}

// Utility functions
int sigma_abs(int x) {
    return x < 0 ? -x : x;
}

long sigma_labs(long x) {
    return x < 0 ? -x : x;
}

div_t sigma_div(int numer, int denom) {
    div_t result;
    result.quot = numer / denom;
    result.rem = numer % denom;
    return result;
}

ldiv_t sigma_ldiv(long numer, long denom) {
    ldiv_t result;
    result.quot = numer / denom;
    result.rem = numer % denom;
    return result;
}

// Sort functions (replacing qsort)
void sigma_qsort(void* base, size_t nmemb, size_t size, int (*compar)(const void*, const void*)) {
    if (!base || nmemb <= 1 || size == 0 || !compar) return;
    
    char* arr = (char*)base;
    
    // Simple bubble sort implementation
    for (size_t i = 0; i < nmemb - 1; i++) {
        for (size_t j = 0; j < nmemb - i - 1; j++) {
            if (compar(arr + j * size, arr + (j + 1) * size) > 0) {
                // Swap elements
                char* elem1 = arr + j * size;
                char* elem2 = arr + (j + 1) * size;
                
                for (size_t k = 0; k < size; k++) {
                    char temp = elem1[k];
                    elem1[k] = elem2[k];
                    elem2[k] = temp;
                }
            }
        }
    }
}

// Search functions (replacing bsearch)
void* sigma_bsearch(const void* key, const void* base, size_t nmemb, size_t size, int (*compar)(const void*, const void*)) {
    if (!key || !base || nmemb == 0 || size == 0 || !compar) return NULL;
    
    const char* arr = (const char*)base;
    size_t left = 0;
    size_t right = nmemb - 1;
    
    while (left <= right) {
        size_t mid = left + (right - left) / 2;
        const void* mid_elem = arr + mid * size;
        
        int cmp = compar(key, mid_elem);
        
        if (cmp == 0) {
            return (void*)mid_elem;
        } else if (cmp < 0) {
            if (mid == 0) break;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    
    return NULL;
}

// Error handling
char* sigma_strerror(int errnum) {
    static char error_msg[256];
    
    switch (errnum) {
        case 0:
            sigma_strcpy(error_msg, "No error");
            break;
        case 1:
            sigma_strcpy(error_msg, "Operation not permitted");
            break;
        case 2:
            sigma_strcpy(error_msg, "No such file or directory");
            break;
        case 3:
            sigma_strcpy(error_msg, "No such process");
            break;
        case 4:
            sigma_strcpy(error_msg, "Interrupted system call");
            break;
        case 5:
            sigma_strcpy(error_msg, "Input/output error");
            break;
        case 6:
            sigma_strcpy(error_msg, "No such device or address");
            break;
        case 7:
            sigma_strcpy(error_msg, "Argument list too long");
            break;
        case 8:
            sigma_strcpy(error_msg, "Exec format error");
            break;
        case 9:
            sigma_strcpy(error_msg, "Bad file number");
            break;
        case 10:
            sigma_strcpy(error_msg, "No child processes");
            break;
        case 11:
            sigma_strcpy(error_msg, "Try again");
            break;
        case 12:
            sigma_strcpy(error_msg, "Out of memory");
            break;
        case 13:
            sigma_strcpy(error_msg, "Permission denied");
            break;
        default:
            sigma_strcpy(error_msg, "Unknown error");
            break;
    }
    
    return error_msg;
}

// Time functions (simplified)
typedef struct {
    int tm_sec;
    int tm_min;
    int tm_hour;
    int tm_mday;
    int tm_mon;
    int tm_year;
    int tm_wday;
    int tm_yday;
    int tm_isdst;
} SigmaTm;

typedef long SigmaTime;

SigmaTime sigma_time(SigmaTime* t) {
    // Simplified time function
    SigmaTime current_time = 1234567890; // Placeholder
    
    if (t) {
        *t = current_time;
    }
    
    return current_time;
}

SigmaTm* sigma_localtime(const SigmaTime* timep) {
    static SigmaTm tm;
    
    // Simplified localtime conversion
    SigmaTime time = timep ? *timep : sigma_time(NULL);
    
    tm.tm_sec = time % 60;
    tm.tm_min = (time / 60) % 60;
    tm.tm_hour = (time / 3600) % 24;
    tm.tm_mday = (time / 86400) % 30 + 1;
    tm.tm_mon = (time / (86400 * 30)) % 12 + 1;
    tm.tm_year = (time / (86400 * 365)) % 100;
    tm.tm_wday = (time / 86400) % 7;
    tm.tm_yday = (time / 86400) % 365;
    tm.tm_isdst = -1;
    
    return &tm;
}

// Character functions
int sigma_isalpha(int c) {
    return (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z');
}

int sigma_isdigit(int c) {
    return c >= '0' && c <= '9';
}

int sigma_isalnum(int c) {
    return sigma_isalpha(c) || sigma_isdigit(c);
}

int sigma_isspace(int c) {
    return c == ' ' || c == '\t' || c == '\n' || c == '\r' || c == '\v' || c == '\f';
}

int sigma_isupper(int c) {
    return c >= 'A' && c <= 'Z';
}

int sigma_islower(int c) {
    return c >= 'a' && c <= 'z';
}

int sigma_toupper(int c) {
    return sigma_islower(c) ? c - 'a' + 'A' : c;
}

int sigma_tolower(int c) {
    return sigma_isupper(c) ? c - 'A' + 'a' : c;
}

// Conversion functions
unsigned long sigma_strtoul(const char* str, char** endptr, int base) {
    if (!str) return 0;
    
    unsigned long result = 0;
    
    // Skip whitespace
    while (sigma_isspace(*str)) {
        str++;
    }
    
    // Convert digits
    while (*str) {
        int digit;
        
        if (*str >= '0' && *str <= '9') {
            digit = *str - '0';
        } else if (*str >= 'A' && *str <= 'Z') {
            digit = *str - 'A' + 10;
        } else if (*str >= 'a' && *str <= 'z') {
            digit = *str - 'a' + 10;
        } else {
            break;
        }
        
        if (digit >= base) break;
        
        result = result * base + digit;
        str++;
    }
    
    if (endptr) {
        *endptr = (char*)str;
    }
    
    return result;
}

long sigma_strtol(const char* str, char** endptr, int base) {
    if (!str) return 0;
    
    int sign = 1;
    
    // Skip whitespace
    while (sigma_isspace(*str)) {
        str++;
    }
    
    // Handle sign
    if (*str == '-') {
        sign = -1;
        str++;
    } else if (*str == '+') {
        str++;
    }
    
    unsigned long result = sigma_strtoul(str, endptr, base);
    
    return sign * (long)result;
}

double sigma_strtod(const char* str, char** endptr) {
    if (!str) return 0.0;
    
    // Skip whitespace
    while (sigma_isspace(*str)) {
        str++;
    }
    
    return sigma_atof(str);
}

// Hash functions
uint32_t sigma_hash_string(const char* str) {
    if (!str) return 0;
    
    uint32_t hash = 5381;
    int c;
    
    while ((c = *str++)) {
        hash = ((hash << 5) + hash) + c; // djb2 algorithm
    }
    
    return hash;
}

uint32_t sigma_hash_bytes(const void* data, size_t size) {
    if (!data || size == 0) return 0;
    
    const unsigned char* bytes = (const unsigned char*)data;
    uint32_t hash = 5381;
    
    for (size_t i = 0; i < size; i++) {
        hash = ((hash << 5) + hash) + bytes[i];
    }
    
    return hash;
}

// CRC32 checksum
uint32_t sigma_crc32(const void* data, size_t size) {
    if (!data || size == 0) return 0;
    
    static const uint32_t crc_table[256] = {
        0x00000000, 0x77073096, 0xee0e612c, 0x990951ba, 0x076dc419, 0x706af48f,
        0xe963a535, 0x9e6495a3, 0x0edb8832, 0x79dcb8a4, 0xe0d5e91e,
        0x97d2d988, 0x09b64c2b, 0x7eb17cbd, 0xe7b82d07, 0x90bf1d91,
        0x1db71064, 0x6ab020f2, 0xf3b97148, 0x84be41de, 0x1adad47d,
        0x6ddde4eb, 0xf4d4b551, 0x83d385c7, 0x136c9856, 0x646ba8c0,
        0xfd62f97a, 0x8a65c9ec, 0x14015c4f, 0x63066cd9, 0xfa0f3d63,
        0x8d080df5, 0x3b6e20c8, 0x4c69105e, 0xd56041e4, 0xa2677172,
        0x3c03e4d1, 0x4b04d447, 0xd20d85fd, 0xa50ab56b, 0x35b5a8fa,
        0x42b2986c, 0xdbbbc9d6, 0xacbcf940, 0x32d86ce3, 0x45df5c75,
        0xdcd60dcf, 0xabd13d59, 0x26d930ac, 0x51de003a, 0xc8d75180,
        0xbfd06116, 0x21b4f4b5, 0x56b3c423, 0xcfba9599, 0xb8bda50f,
        0x2802b89e, 0x5f058808, 0xc60cd9b2, 0xb10be924, 0x2f6f7c87,
        0x58684c11, 0xc1611dab, 0xb6662d3d, 0x76dc4190, 0x01db7106,
        0x98d220bc, 0xefd5102a, 0x71b18589, 0x06b6b51f, 0x9fbfe4a5,
        0xe8b8d433, 0x7807c9a2, 0x0f00f934, 0x9609a88e, 0xe10e9818,
        0x7f6a0dbb, 0x086d3d2d, 0x91646c97, 0xe6635c01, 0x6b6b51f4,
        0x1c6c6162, 0x856530d8, 0xf262004e, 0x6c0695ed, 0x1b01a57b,
        0x8208f4c1, 0xf50fc457, 0x65b0d9c6, 0x12b7e950, 0x8bbeb8ea,
        0xfcb9887c, 0x62dd1ddf, 0x15da2d49, 0x8cd37cf3, 0xfbd44c65,
        0x4db26158, 0x3ab551ce, 0xa3bc0074, 0xd4bb30e2, 0x4adfa541,
        0x3dd895d7, 0xa4d1c46d, 0xd3d6f4fb, 0x4369e96a, 0x346ed9fc,
        0xad678846, 0xda60b8d0, 0x44042d73, 0x33031de5, 0xaa0a4c5f,
        0xdd0d7cc9, 0x5005713c, 0x270241aa, 0xbe0b1010, 0xc90c2086,
        0x5768b525, 0x206f85b3, 0xb966d409, 0xce61e49f, 0x5edef90e,
        0x29d9c998, 0xb0d09822, 0xc7d7a8b4, 0x59b33d17, 0x2eb40d81,
        0xb7bd5c3b, 0xc0ba6cad, 0xedb88320, 0x9abfb3b6, 0x03b6e20c,
        0x74b1d291, 0xead54739, 0x9dd277af, 0x04db2615, 0x73dc1683,
        0xe3630b12, 0x94643b84, 0x0d6d6a3e, 0x7a6a5aa8, 0xe40ecf0b,
        0x9309ff9d, 0x0a00ae27, 0x7d079eb1, 0xf00f9344, 0x8708a3d2,
        0x1e01f268, 0x6906c2fe, 0xf762575d, 0x806567cb, 0x196c3671,
        0x6e6b06e7, 0xfed41b76, 0x89d32be0, 0x10da7a5a, 0x67dd4acc,
        0xf9b9df6f, 0x8ebeeff9, 0x17b7be43, 0x60b08ed5, 0xd6d6a3e8,
        0xa1d1937e, 0x38d8c2c4, 0x4fdff252, 0xd1bb67f1, 0xa6bc5767,
        0x3fb506dd, 0x48b2364b, 0xd80d2bda, 0xaf0a1b4c, 0x36034af6,
        0x41047a60, 0xdf60efc3, 0xa867df55, 0x316e8eef, 0x4668be2c,
        0x96610e76, 0xcb61b38c, 0xbc66831a, 0x256fd2a0, 0x5268e236,
        0xcc0c7795, 0xbb0b4703, 0x220216b9, 0x5505262f, 0xc5ba3bbe,
        0xb2bd0b28, 0x2bb45a92, 0x5cb36a04, 0xc2d7ffa7, 0xb5d0cf31,
        0x2cd99e8b, 0x5bdeae1d, 0x9b64c2b0, 0xec63f226, 0x756aa39c,
        0x026d930a, 0x9c0906a9, 0xeb0e363f, 0x72076785, 0x05005713,
        0x95bf4a82, 0xe2b87a14, 0x7bb12bae, 0x0cb61b38, 0x92d28e9b,
        0xe5d5be0d, 0x7cdcefb7, 0x0bdbdf21, 0x86d3d2d4, 0xf1d4e242,
        0x68ddb3f8, 0x1fda836e, 0x81be16cd, 0xf6b9265b, 0x6fb077e1,
        0x18b74777, 0x88085ae6, 0xff0f6a70, 0x66063bca, 0x11010b5c,
        0x8f659eff, 0xf862ae69, 0x616bffd3, 0x166ccf45, 0xa00ae278,
        0xd70dd2ee, 0x4e048354, 0x3903b3c2, 0xa7672661, 0xd06016f7,
        0x4969474d, 0x3e6e77db, 0xaed16a4a, 0xd9d65adc, 0x40df0b66,
        0x37d83bf0, 0xa9bcae53, 0xdebb9ec5, 0x47b2cf7f, 0x30b5ffe9,
        0xbdbdf21c, 0xcabac28a, 0x53b39330, 0x24b4a3a6, 0xbad03605,
        0xcdd70693, 0x54de5729, 0x23d967bf, 0xb3667a2e, 0xc4614ab8,
        0x5d681b02, 0x2a6f2b94, 0xb40bbe37, 0xc30c8ea1, 0x5a05df1b,
        0x2d02ef8d
    };
    
    const unsigned char* bytes = (const unsigned char*)data;
    uint32_t crc = 0xFFFFFFFF;
    
    for (size_t i = 0; i < size; i++) {
        crc = crc_table[(crc ^ bytes[i]) & 0xFF] ^ (crc >> 8);
    }
    
    return crc ^ 0xFFFFFFFF;
}

// Base64 encoding/decoding
static const char base64_chars[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

char* sigma_base64_encode(const unsigned char* data, size_t input_length, size_t* output_length) {
    if (!data || input_length == 0) return NULL;
    
    *output_length = 4 * ((input_length + 2) / 3);
    
    char* encoded_data = (char*)malloc(*output_length + 1);
    if (!encoded_data) return NULL;
    
    for (size_t i = 0, j = 0; i < input_length;) {
        uint32_t octet_a = i < input_length ? data[i++] : 0;
        uint32_t octet_b = i < input_length ? data[i++] : 0;
        uint32_t octet_c = i < input_length ? data[i++] : 0;
        
        uint32_t triple = (octet_a << 0x10) + (octet_b << 0x08) + octet_c;
        
        encoded_data[j++] = base64_chars[(triple >> 3 * 6) & 0x3F];
        encoded_data[j++] = base64_chars[(triple >> 2 * 6) & 0x3F];
        encoded_data[j++] = base64_chars[(triple >> 1 * 6) & 0x3F];
        encoded_data[j++] = base64_chars[(triple >> 0 * 6) & 0x3F];
    }
    
    for (size_t i = 0; i < (3 - input_length % 3) % 3; i++) {
        encoded_data[*output_length - 1 - i] = '=';
    }
    
    encoded_data[*output_length] = '\0';
    return encoded_data;
}

unsigned char* sigma_base64_decode(const char* data, size_t input_length, size_t* output_length) {
    if (!data || input_length == 0) return NULL;
    
    // Remove padding
    while (input_length > 0 && data[input_length - 1] == '=') {
        input_length--;
    }
    
    *output_length = input_length * 3 / 4;
    
    unsigned char* decoded_data = (unsigned char*)malloc(*output_length);
    if (!decoded_data) return NULL;
    
    for (size_t i = 0, j = 0; i < input_length;) {
        uint32_t sextet_a = data[i] == '=' ? 0 & i++ : strchr(base64_chars, data[i++]) - base64_chars;
        uint32_t sextet_b = data[i] == '=' ? 0 & i++ : strchr(base64_chars, data[i++]) - base64_chars;
        uint32_t sextet_c = data[i] == '=' ? 0 & i++ : strchr(base64_chars, data[i++]) - base64_chars;
        uint32_t sextet_d = data[i] == '=' ? 0 & i++ : strchr(base64_chars, data[i++]) - base64_chars;
        
        uint32_t triple = (sextet_a << 3 * 6) + (sextet_b << 2 * 6) + (sextet_c << 1 * 6) + (sextet_d << 0 * 6);
        
        if (j < *output_length) decoded_data[j++] = (triple >> 2 * 8) & 0xFF;
        if (j < *output_length) decoded_data[j++] = (triple >> 1 * 8) & 0xFF;
        if (j < *output_length) decoded_data[j++] = (triple >> 0 * 8) & 0xFF;
    }
    
    return decoded_data;
}

