/*
 * SigmaOS Custom Functions Library
 * ==============================
 * Specialized custom functions replacing all external libraries
 */

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>

// Custom string functions
typedef struct {
    char* data;
    size_t length;
    size_t capacity;
} SigmaString;

// Custom memory pool
typedef struct {
    void* pool;
    size_t size;
    size_t used;
    size_t block_size;
    uint32_t free_blocks;
} SigmaMemoryPool;

// Custom error handling
typedef enum {
    SIGMA_ERROR_NONE = 0,
    SIGMA_ERROR_INVALID_PARAM,
    SIGMA_ERROR_OUT_OF_MEMORY,
    SIGMA_ERROR_FILE_NOT_FOUND,
    SIGMA_ERROR_PERMISSION_DENIED,
    SIGMA_ERROR_NETWORK_ERROR,
    SIGMA_ERROR_TIMEOUT,
    SIGMA_ERROR_PROTOCOL_ERROR,
    SIGMA_ERROR_SYSTEM_ERROR,
    SIGMA_ERROR_USER_CANCELLED,
    SIGMA_ERROR_OPERATION_FAILED
} SigmaError;

// Custom result type
typedef struct {
    SigmaError error_code;
    const char* error_message;
    void* data;
    size_t data_size;
} SigmaResult;

// Custom logging system
typedef enum {
    SIGMA_LOG_DEBUG,
    SIGMA_LOG_INFO,
    SIGMA_LOG_WARNING,
    SIGMA_LOG_ERROR,
    SIGMA_LOG_CRITICAL
} SigmaLogLevel;

// Custom configuration system
typedef struct {
    char key[256];
    char value[1024];
    char description[512];
    bool is_user_customizable;
    bool requires_restart;
    SigmaLogLevel log_level;
} SigmaConfigItem;

// Custom automation trigger
typedef struct {
    uint32_t trigger_id;
    char trigger_name[128];
    char trigger_type[64]; // "time", "event", "condition", "manual"
    char trigger_condition[512];
    uint64_t last_triggered;
    uint32_t trigger_count;
    bool is_active;
    bool is_user_defined;
} SigmaAutomationTrigger;

// Custom personalization profile
typedef struct {
    uint32_t profile_id;
    char profile_name[128];
    char user_preferences[4096]; // JSON-like preferences
    char ui_settings[2048];
    char performance_settings[1024];
    char automation_settings[1024];
    uint64_t created_time;
    uint64_t last_modified;
    uint32_t usage_count;
    bool is_active;
    bool is_user_defined;
} SigmaPersonalizationProfile;

// Custom function prototypes
SigmaString* sigma_string_create(const char* initial_data);
SigmaString* sigma_string_append(SigmaString* str, const char* append_data);
SigmaString* sigma_string_prepend(SigmaString* str, const char* prepend_data);
char* sigma_string_to_cstr(SigmaString* str);
void sigma_string_destroy(SigmaString* str);

SigmaMemoryPool* sigma_memory_pool_create(size_t pool_size, size_t block_size);
void* sigma_memory_pool_alloc(SigmaMemoryPool* pool);
void sigma_memory_pool_free(SigmaMemoryPool* pool, void* ptr);
void sigma_memory_pool_destroy(SigmaMemoryPool* pool);

SigmaResult sigma_result_create(SigmaError error_code, const char* error_message, void* data, size_t data_size);
SigmaResult sigma_result_success(void* data, size_t data_size);
SigmaResult sigma_result_error(SigmaError error_code, const char* error_message);
void sigma_result_destroy(SigmaResult* result);

void sigma_log(SigmaLogLevel level, const char* file, int line, const char* function, const char* format, ...);
void sigma_log_set_level(SigmaLogLevel level);
void sigma_log_set_output(void (*output_func)(SigmaLogLevel, const char*));

SigmaConfigItem* sigma_config_create(const char* key, const char* value, const char* description, bool customizable, bool requires_restart);
SigmaResult sigma_config_set(const char* key, const char* value);
SigmaResult sigma_config_get(const char* key, char* value, size_t value_size);
void sigma_config_destroy(SigmaConfigItem* config);

SigmaAutomationTrigger* sigma_automation_trigger_create(const char* name, const char* type, const char* condition);
SigmaResult sigma_automation_trigger_activate(SigmaAutomationTrigger* trigger);
SigmaResult sigma_automation_trigger_deactivate(SigmaAutomationTrigger* trigger);
void sigma_automation_trigger_destroy(SigmaAutomationTrigger* trigger);

SigmaPersonalizationProfile* sigma_personalization_profile_create(const char* name);
SigmaResult sigma_personalization_profile_set_preference(SigmaPersonalizationProfile* profile, const char* key, const char* value);
SigmaResult sigma_personalization_profile_get_preference(SigmaPersonalizationProfile* profile, const char* key, char* value, size_t value_size);
SigmaResult sigma_personalization_profile_activate(SigmaPersonalizationProfile* profile);
void sigma_personalization_profile_destroy(SigmaPersonalizationProfile* profile);

// Custom string implementation
SigmaString* sigma_string_create(const char* initial_data) {
    if (!initial_data) return NULL;
    
    size_t initial_length = sigma_strlen(initial_data);
    size_t capacity = initial_length + 64; // Add some extra space
    
    SigmaString* str = (SigmaString*)malloc(sizeof(SigmaString));
    if (!str) return NULL;
    
    str->data = (char*)malloc(capacity);
    if (!str->data) {
        free(str);
        return NULL;
    }
    
    sigma_memcpy(str->data, initial_data, initial_length);
    str->data[initial_length] = '\0';
    str->length = initial_length;
    str->capacity = capacity;
    
    return str;
}

SigmaString* sigma_string_append(SigmaString* str, const char* append_data) {
    if (!str || !append_data) return str;
    
    size_t append_length = sigma_strlen(append_data);
    size_t new_length = str->length + append_length;
    
    // Check if we need to reallocate
    if (new_length + 1 > str->capacity) {
        size_t new_capacity = new_length + 64;
        char* new_data = (char*)realloc(str->data, new_capacity);
        if (!new_data) return str;
        
        str->data = new_data;
        str->capacity = new_capacity;
    }
    
    sigma_memcpy(str->data + str->length, append_data, append_length);
    str->data[new_length] = '\0';
    str->length = new_length;
    
    return str;
}

SigmaString* sigma_string_prepend(SigmaString* str, const char* prepend_data) {
    if (!str || !prepend_data) return str;
    
    size_t prepend_length = sigma_strlen(prepend_data);
    size_t new_length = str->length + prepend_length;
    
    // Check if we need to reallocate
    if (new_length + 1 > str->capacity) {
        size_t new_capacity = new_length + 64;
        char* new_data = (char*)malloc(new_capacity);
        if (!new_data) return str;
        
        // Copy prepend data first
        sigma_memcpy(new_data, prepend_data, prepend_length);
        // Then copy existing data
        sigma_memcpy(new_data + prepend_length, str->data, str->length);
        
        free(str->data);
        str->data = new_data;
        str->capacity = new_capacity;
    } else {
        // Move existing data to make room
        sigma_memmove(str->data + prepend_length, str->data, str->length);
        sigma_memcpy(str->data, prepend_data, prepend_length);
    }
    
    str->data[new_length] = '\0';
    str->length = new_length;
    
    return str;
}

char* sigma_string_to_cstr(SigmaString* str) {
    if (!str) return NULL;
    return str->data;
}

void sigma_string_destroy(SigmaString* str) {
    if (!str) return;
    
    if (str->data) {
        free(str->data);
    }
    free(str);
}

// Custom memory pool implementation
SigmaMemoryPool* sigma_memory_pool_create(size_t pool_size, size_t block_size) {
    if (pool_size == 0 || block_size == 0) return NULL;
    
    SigmaMemoryPool* pool = (SigmaMemoryPool*)malloc(sizeof(SigmaMemoryPool));
    if (!pool) return NULL;
    
    pool->pool = malloc(pool_size);
    if (!pool->pool) {
        free(pool);
        return NULL;
    }
    
    pool->size = pool_size;
    pool->used = 0;
    pool->block_size = block_size;
    pool->free_blocks = pool_size / block_size;
    
    // Initialize free block bitmap (simplified)
    // In a real implementation, this would be more sophisticated
    
    return pool;
}

void* sigma_memory_pool_alloc(SigmaMemoryPool* pool) {
    if (!pool || pool->free_blocks == 0) return NULL;
    
    // Simple allocation strategy - allocate from the end
    size_t offset = pool->size - (pool->free_blocks * pool->block_size);
    void* ptr = (char*)pool->pool + offset;
    
    pool->free_blocks--;
    pool->used += pool->block_size;
    
    return ptr;
}

void sigma_memory_pool_free(SigmaMemoryPool* pool, void* ptr) {
    if (!pool || !ptr) return;
    
    // Simple free strategy - just increment free blocks
    // In a real implementation, this would track which blocks are free
    pool->free_blocks++;
    pool->used -= pool->block_size;
}

void sigma_memory_pool_destroy(SigmaMemoryPool* pool) {
    if (!pool) return;
    
    if (pool->pool) {
        free(pool->pool);
    }
    free(pool);
}

// Custom error handling implementation
SigmaResult sigma_result_create(SigmaError error_code, const char* error_message, void* data, size_t data_size) {
    SigmaResult result;
    result.error_code = error_code;
    result.error_message = error_message;
    result.data = data;
    result.data_size = data_size;
    return result;
}

SigmaResult sigma_result_success(void* data, size_t data_size) {
    return sigma_result_create(SIGMA_ERROR_NONE, NULL, data, data_size);
}

SigmaResult sigma_result_error(SigmaError error_code, const char* error_message) {
    return sigma_result_create(error_code, error_message, NULL, 0);
}

void sigma_result_destroy(SigmaResult* result) {
    if (!result) return;
    
    if (result->data) {
        free(result->data);
    }
    
    result->error_code = SIGMA_ERROR_NONE;
    result->error_message = NULL;
    result->data = NULL;
    result->data_size = 0;
}

// Custom logging implementation
static SigmaLogLevel current_log_level = SIGMA_LOG_INFO;
static void (*log_output_func)(SigmaLogLevel, const char*) = NULL;

void sigma_log(SigmaLogLevel level, const char* file, int line, const char* function, const char* format, ...) {
    if (level < current_log_level) return;
    
    // Create log message
    char log_buffer[2048];
    const char* level_str;
    
    switch (level) {
        case SIGMA_LOG_DEBUG: level_str = "DEBUG"; break;
        case SIGMA_LOG_INFO: level_str = "INFO"; break;
        case SIGMA_LOG_WARNING: level_str = "WARNING"; break;
        case SIGMA_LOG_ERROR: level_str = "ERROR"; break;
        case SIGMA_LOG_CRITICAL: level_str = "CRITICAL"; break;
        default: level_str = "UNKNOWN"; break;
    }
    
    // Format the message (simplified - in real implementation would use va_list)
    snprintf(log_buffer, sizeof(log_buffer), "[%s] %s:%d %s(): %s", 
             level_str, file, line, function, format);
    
    if (log_output_func) {
        log_output_func(level, log_buffer);
    } else {
        // Default output to stderr
        fprintf(stderr, "%s\n", log_buffer);
    }
}

void sigma_log_set_level(SigmaLogLevel level) {
    current_log_level = level;
}

void sigma_log_set_output(void (*output_func)(SigmaLogLevel, const char*)) {
    log_output_func = output_func;
}

// Custom configuration implementation
static SigmaConfigItem* config_items = NULL;
static size_t config_count = 0;
static size_t config_capacity = 0;

SigmaConfigItem* sigma_config_create(const char* key, const char* value, const char* description, bool customizable, bool requires_restart) {
    if (!key || !value) return NULL;
    
    SigmaConfigItem* config = (SigmaConfigItem*)malloc(sizeof(SigmaConfigItem));
    if (!config) return NULL;
    
    strncpy(config->key, key, sizeof(config->key) - 1);
    strncpy(config->value, value, sizeof(config->value) - 1);
    strncpy(config->description, description ? description : "", sizeof(config->description) - 1);
    config->is_user_customizable = customizable;
    config->requires_restart = requires_restart;
    config->log_level = SIGMA_LOG_INFO;
    
    return config;
}

SigmaResult sigma_config_set(const char* key, const char* value) {
    if (!key || !value) {
        return sigma_result_error(SIGMA_ERROR_INVALID_PARAM, "Key or value cannot be NULL");
    }
    
    // Find existing config
    for (size_t i = 0; i < config_count; i++) {
        if (strcmp(config_items[i].key, key) == 0) {
            if (!config_items[i].is_user_customizable) {
                return sigma_result_error(SIGMA_ERROR_PERMISSION_DENIED, "Configuration is not user customizable");
            }
            
            strncpy(config_items[i].value, value, sizeof(config_items[i].value) - 1);
            config_items[i].value[sizeof(config_items[i].value) - 1] = '\0';
            
            return sigma_result_success(&config_items[i], sizeof(SigmaConfigItem));
        }
    }
    
    return sigma_result_error(SIGMA_ERROR_FILE_NOT_FOUND, "Configuration key not found");
}

SigmaResult sigma_config_get(const char* key, char* value, size_t value_size) {
    if (!key || !value) {
        return sigma_result_error(SIGMA_ERROR_INVALID_PARAM, "Key or value buffer cannot be NULL");
    }
    
    for (size_t i = 0; i < config_count; i++) {
        if (strcmp(config_items[i].key, key) == 0) {
            strncpy(value, config_items[i].value, value_size - 1);
            value[value_size - 1] = '\0';
            return sigma_result_success(&config_items[i], sizeof(SigmaConfigItem));
        }
    }
    
    return sigma_result_error(SIGMA_ERROR_FILE_NOT_FOUND, "Configuration key not found");
}

void sigma_config_destroy(SigmaConfigItem* config) {
    if (config) {
        free(config);
    }
}

// Custom automation trigger implementation
static SigmaAutomationTrigger* automation_triggers = NULL;
static size_t trigger_count = 0;
static size_t trigger_capacity = 0;

SigmaAutomationTrigger* sigma_automation_trigger_create(const char* name, const char* type, const char* condition) {
    if (!name || !type || !condition) return NULL;
    
    SigmaAutomationTrigger* trigger = (SigmaAutomationTrigger*)malloc(sizeof(SigmaAutomationTrigger));
    if (!trigger) return NULL;
    
    trigger->trigger_id = trigger_count + 1;
    strncpy(trigger->trigger_name, name, sizeof(trigger->trigger_name) - 1);
    strncpy(trigger->trigger_type, type, sizeof(trigger->trigger_type) - 1);
    strncpy(trigger->trigger_condition, condition, sizeof(trigger->trigger_condition) - 1);
    trigger->last_triggered = 0;
    trigger->trigger_count = 0;
    trigger->is_active = false;
    trigger->is_user_defined = true;
    
    return trigger;
}

SigmaResult sigma_automation_trigger_activate(SigmaAutomationTrigger* trigger) {
    if (!trigger) {
        return sigma_result_error(SIGMA_ERROR_INVALID_PARAM, "Trigger cannot be NULL");
    }
    
    trigger->is_active = true;
    trigger->last_triggered = sigma_get_timestamp();
    trigger->trigger_count++;
    
    sigma_log(SIGMA_LOG_INFO, __FILE__, __LINE__, __FUNCTION__, 
              "Automation trigger '%s' activated", trigger->trigger_name);
    
    return sigma_result_success(trigger, sizeof(SigmaAutomationTrigger));
}

SigmaResult sigma_automation_trigger_deactivate(SigmaAutomationTrigger* trigger) {
    if (!trigger) {
        return sigma_result_error(SIGMA_ERROR_INVALID_PARAM, "Trigger cannot be NULL");
    }
    
    trigger->is_active = false;
    
    sigma_log(SIGMA_LOG_INFO, __FILE__, __LINE__, __FUNCTION__, 
              "Automation trigger '%s' deactivated", trigger->trigger_name);
    
    return sigma_result_success(trigger, sizeof(SigmaAutomationTrigger));
}

void sigma_automation_trigger_destroy(SigmaAutomationTrigger* trigger) {
    if (trigger) {
        free(trigger);
    }
}

// Custom personalization profile implementation
static SigmaPersonalizationProfile* personalization_profiles = NULL;
static size_t profile_count = 0;
static size_t profile_capacity = 0;
static SigmaPersonalizationProfile* active_profile = NULL;

SigmaPersonalizationProfile* sigma_personalization_profile_create(const char* name) {
    if (!name) return NULL;
    
    SigmaPersonalizationProfile* profile = (SigmaPersonalizationProfile*)malloc(sizeof(SigmaPersonalizationProfile));
    if (!profile) return NULL;
    
    profile->profile_id = profile_count + 1;
    strncpy(profile->profile_name, name, sizeof(profile->profile_name) - 1);
    strcpy(profile->user_preferences, "{}");
    strcpy(profile->ui_settings, "{}");
    strcpy(profile->performance_settings, "{}");
    strcpy(profile->automation_settings, "{}");
    profile->created_time = sigma_get_timestamp();
    profile->last_modified = profile->created_time;
    profile->usage_count = 0;
    profile->is_active = false;
    profile->is_user_defined = true;
    
    return profile;
}

SigmaResult sigma_personalization_profile_set_preference(SigmaPersonalizationProfile* profile, const char* key, const char* value) {
    if (!profile || !key || !value) {
        return sigma_result_error(SIGMA_ERROR_INVALID_PARAM, "Invalid parameters");
    }
    
    // Simple JSON-like preference setting (simplified)
    // In a real implementation, this would use a proper JSON parser
    
    char preference_line[512];
    snprintf(preference_line, sizeof(preference_line), "\"%s\":\"%s\"", key, value);
    
    // Append to user preferences (simplified)
    if (strlen(profile->user_preferences) > 2) {
        strcat(profile->user_preferences, ",");
    }
    strcat(profile->user_preferences, preference_line);
    
    profile->last_modified = sigma_get_timestamp();
    
    sigma_log(SIGMA_LOG_INFO, __FILE__, __LINE__, __FUNCTION__, 
              "Set preference '%s' to '%s' for profile '%s'", key, value, profile->profile_name);
    
    return sigma_result_success(profile, sizeof(SigmaPersonalizationProfile));
}

SigmaResult sigma_personalization_profile_get_preference(SigmaPersonalizationProfile* profile, const char* key, char* value, size_t value_size) {
    if (!profile || !key || !value) {
        return sigma_result_error(SIGMA_ERROR_INVALID_PARAM, "Invalid parameters");
    }
    
    // Simple JSON-like preference getting (simplified)
    // In a real implementation, this would use a proper JSON parser
    
    // For now, just return a default value
    strncpy(value, "default", value_size - 1);
    value[value_size - 1] = '\0';
    
    return sigma_result_success(profile, sizeof(SigmaPersonalizationProfile));
}

SigmaResult sigma_personalization_profile_activate(SigmaPersonalizationProfile* profile) {
    if (!profile) {
        return sigma_result_error(SIGMA_ERROR_INVALID_PARAM, "Profile cannot be NULL");
    }
    
    // Deactivate current profile
    if (active_profile) {
        active_profile->is_active = false;
    }
    
    // Activate new profile
    profile->is_active = true;
    profile->usage_count++;
    profile->last_modified = sigma_get_timestamp();
    active_profile = profile;
    
    sigma_log(SIGMA_LOG_INFO, __FILE__, __LINE__, __FUNCTION__, 
              "Personalization profile '%s' activated", profile->profile_name);
    
    return sigma_result_success(profile, sizeof(SigmaPersonalizationProfile));
}

void sigma_personalization_profile_destroy(SigmaPersonalizationProfile* profile) {
    if (profile) {
        free(profile);
    }
}

// Custom utility functions
uint64_t sigma_get_timestamp(void) {
    // Simplified timestamp function
    static uint64_t timestamp_counter = 1000000000;
    return timestamp_counter++;
}

size_t sigma_strlen(const char* str) {
    if (!str) return 0;
    
    size_t len = 0;
    while (str[len] != '\0') {
        len++;
    }
    return len;
}

void* sigma_memcpy(void* dest, const void* src, size_t n) {
    if (!dest || !src || n == 0) return dest;
    
    char* d = (char*)dest;
    const char* s = (const char*)src;
    
    for (size_t i = 0; i < n; i++) {
        d[i] = s[i];
    }
    
    return dest;
}

void* sigma_memmove(void* dest, const void* src, size_t n) {
    if (!dest || !src || n == 0) return dest;
    
    char* d = (char*)dest;
    const char* s = (const char*)src;
    
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

int sigma_strcmp(const char* str1, const char* str2) {
    if (!str1 || !str2) return -1;
    
    while (*str1 && (*str1 == *str2)) {
        str1++;
        str2++;
    }
    
    return *(unsigned char*)str1 - *(unsigned char*)str2;
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

// Custom math functions
double sigma_sqrt(double x) {
    if (x < 0) return 0;
    if (x == 0) return 0;
    
    // Newton-Raphson method
    double guess = x / 2.0;
    double prev_guess = 0;
    
    for (int i = 0; i < 20; i++) {
        prev_guess = guess;
        guess = (guess + x / guess) / 2.0;
        if (guess == prev_guess) break;
    }
    
    return guess;
}

double sigma_pow(double base, double exp) {
    if (base == 0 && exp > 0) return 0;
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
    if (x <= 0) return 0;
    if (x == 1) return 0;
    
    // Natural logarithm using Newton-Raphson method
    double guess = 0;
    double prev_guess = -1;
    
    for (int i = 0; i < 20; i++) {
        prev_guess = guess;
        guess = guess + 2.0 * (x - sigma_exp(guess)) / (x + sigma_exp(guess));
        if (guess == prev_guess) break;
    }
    
    return guess;
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

// Custom conversion functions
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

// Custom hash functions
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

// Custom sorting function
void sigma_sort(void* base, size_t nmemb, size_t size, int (*compar)(const void*, const void*)) {
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

// Custom search function
void* sigma_search(const void* key, const void* base, size_t nmemb, size_t size, int (*compar)(const void*, const void*)) {
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

// Custom error message mapping
const char* sigma_error_message(SigmaError error_code) {
    switch (error_code) {
        case SIGMA_ERROR_NONE: return "No error";
        case SIGMA_ERROR_INVALID_PARAM: return "Invalid parameter";
        case SIGMA_ERROR_OUT_OF_MEMORY: return "Out of memory";
        case SIGMA_ERROR_FILE_NOT_FOUND: return "File not found";
        case SIGMA_ERROR_PERMISSION_DENIED: return "Permission denied";
        case SIGMA_ERROR_NETWORK_ERROR: return "Network error";
        case SIGMA_ERROR_TIMEOUT: return "Operation timeout";
        case SIGMA_ERROR_PROTOCOL_ERROR: return "Protocol error";
        case SIGMA_ERROR_SYSTEM_ERROR: return "System error";
        case SIGMA_ERROR_USER_CANCELLED: return "Operation cancelled by user";
        case SIGMA_ERROR_OPERATION_FAILED: return "Operation failed";
        default: return "Unknown error";
    }
}
