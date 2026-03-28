/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS Advanced Error Handling System
 * ====================================
 * Comprehensive error handling with automation and recovery
 */

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>

// Error severity levels
typedef enum {
    SIGMA_SEVERITY_INFO = 0,
    SIGMA_SEVERITY_WARNING,
    SIGMA_SEVERITY_ERROR,
    SIGMA_SEVERITY_CRITICAL,
    SIGMA_SEVERITY_FATAL
} SigmaSeverity;

// Error categories
typedef enum {
    SIGMA_CATEGORY_SYSTEM = 0,
    SIGMA_CATEGORY_MEMORY,
    SIGMA_CATEGORY_FILESYSTEM,
    SIGMA_CATEGORY_NETWORK,
    SIGMA_CATEGORY_SECURITY,
    SIGMA_CATEGORY_PERFORMANCE,
    SIGMA_CATEGORY_USER_INTERFACE,
    SIGMA_CATEGORY_AUTOMATION,
    SIGMA_CATEGORY_PERSONALIZATION,
    SIGMA_CATEGORY_CUSTOM
} SigmaErrorCategory;

// Error recovery actions
typedef enum {
    SIGMA_RECOVERY_NONE = 0,
    SIGMA_RECOVERY_RETRY,
    SIGMA_RECOVERY_RESET,
    SIGMA_RECOVERY_FALLBACK,
    SIGMA_RECOVERY_RESTART,
    SIGMA_RECOVERY_REPAIR,
    SIGMA_RECOVERY_USER_INTERVENTION,
    SIGMA_RECOVERY_AUTOMATED
} SigmaRecoveryAction;

// Error context information
typedef struct {
    char function_name[128];
    char file_name[256];
    int line_number;
    uint64_t timestamp;
    uint32_t thread_id;
    uint32_t process_id;
    char additional_info[512];
} SigmaErrorContext;

// Error information structure
typedef struct {
    uint32_t error_id;
    SigmaErrorCategory category;
    SigmaSeverity severity;
    SigmaRecoveryAction recovery_action;
    SigmaErrorContext context;
    char error_message[512];
    char technical_details[1024];
    char user_friendly_message[256];
    bool is_recoverable;
    bool is_automated_recovery;
    uint32_t occurrence_count;
    uint64_t first_occurrence;
    uint64_t last_occurrence;
} SigmaError;

// Error recovery callback
typedef SigmaRecoveryAction (*SigmaErrorRecoveryCallback)(const SigmaError* error, void* user_data);

// Error handler configuration
typedef struct {
    bool enable_automated_recovery;
    bool enable_user_notifications;
    bool enable_error_logging;
    bool enable_error_statistics;
    bool enable_predictive_analysis;
    uint32_t max_error_history;
    uint32_t recovery_retry_count;
    uint64_t recovery_timeout_ms;
    SigmaSeverity min_notification_level;
} SigmaErrorHandlerConfig;

// Error statistics
typedef struct {
    uint32_t total_errors;
    uint32_t errors_by_category[10];
    uint32_t errors_by_severity[5];
    uint32_t successful_recoveries;
    uint32_t failed_recoveries;
    uint32_t automated_recoveries;
    uint32_t manual_interventions;
    double average_recovery_time_ms;
    uint64_t last_error_time;
} SigmaErrorStatistics;

// Error handler instance
typedef struct {
    SigmaErrorHandlerConfig config;
    SigmaErrorStatistics statistics;
    SigmaError* error_history;
    size_t error_history_size;
    size_t error_history_capacity;
    SigmaErrorRecoveryCallback* recovery_callbacks;
    size_t callback_count;
    size_t callback_capacity;
    uint32_t next_error_id;
    bool is_initialized;
} SigmaErrorHandler;

// Global error handler instance
static SigmaErrorHandler* global_error_handler = NULL;

// Error handling function prototypes
SigmaErrorHandler* sigma_error_handler_init(const SigmaErrorHandlerConfig* config);
void sigma_error_handler_destroy(SigmaErrorHandler* handler);
SigmaError sigma_error_create(SigmaErrorCategory category, SigmaSeverity severity, const char* message, const char* function, const char* file, int line);
SigmaError sigma_error_create_with_context(SigmaErrorCategory category, SigmaSeverity severity, const char* message, const SigmaErrorContext* context);
SigmaRecoveryAction sigma_error_handle(SigmaErrorHandler* handler, const SigmaError* error);
SigmaRecoveryAction sigma_error_handle_with_recovery(SigmaErrorHandler* handler, const SigmaError* error, SigmaErrorRecoveryCallback callback);
void sigma_error_log(const SigmaError* error);
void sigma_error_notify_user(const SigmaError* error);
bool sigma_error_register_recovery_callback(SigmaErrorHandler* handler, SigmaErrorRecoveryCallback callback);
SigmaErrorStatistics* sigma_error_get_statistics(SigmaErrorHandler* handler);
void sigma_error_clear_history(SigmaErrorHandler* handler);
SigmaError* sigma_error_get_recent_errors(SigmaErrorHandler* handler, size_t count);
bool sigma_error_predict_issues(SigmaErrorHandler* handler, SigmaErrorCategory category);

// Error handling macros
#define SIGMA_ERROR_CREATE(category, severity, message) \
    sigma_error_create(category, severity, message, __FUNCTION__, __FILE__, __LINE__)

#define SIGMA_ERROR_HANDLE(handler, error) \
    sigma_error_handle(handler, error)

#define SIGMA_ERROR_CHECK(condition, category, severity, message) \
    do { \
        if (!(condition)) { \
            SigmaError error = SIGMA_ERROR_CREATE(category, severity, message); \
            sigma_error_handle(global_error_handler, &error); \
        } \
    } while(0)

#define SIGMA_ERROR_RETURN(condition, category, severity, message, return_value) \
    do { \
        if (!(condition)) { \
            SigmaError error = SIGMA_ERROR_CREATE(category, severity, message); \
            sigma_error_handle(global_error_handler, &error); \
            return return_value; \
        } \
    } while(0)

#define SIGMA_ERROR_PANIC(message) \
    do { \
        SigmaError error = SIGMA_ERROR_CREATE(SIGMA_CATEGORY_SYSTEM, SIGMA_SEVERITY_FATAL, message); \
        sigma_error_handle(global_error_handler, &error); \
        while(1) { /* Infinite loop for fatal error */ } \
    } while(0)

// Error handling implementation
SigmaErrorHandler* sigma_error_handler_init(const SigmaErrorHandlerConfig* config) {
    SigmaErrorHandler* handler = (SigmaErrorHandler*)malloc(sizeof(SigmaErrorHandler));
    if (!handler) return NULL;
    
    // Set default configuration if not provided
    if (config) {
        handler->config = *config;
    } else {
        handler->config.enable_automated_recovery = true;
        handler->config.enable_user_notifications = true;
        handler->config.enable_error_logging = true;
        handler->config.enable_error_statistics = true;
        handler->config.enable_predictive_analysis = true;
        handler->config.max_error_history = 1000;
        handler->config.recovery_retry_count = 3;
        handler->config.recovery_timeout_ms = 5000;
        handler->config.min_notification_level = SIGMA_SEVERITY_WARNING;
    }
    
    // Initialize statistics
    memset(&handler->statistics, 0, sizeof(SigmaErrorStatistics));
    
    // Allocate error history
    handler->error_history_capacity = handler->config.max_error_history;
    handler->error_history = (SigmaError*)malloc(handler->error_history_capacity * sizeof(SigmaError));
    if (!handler->error_history) {
        free(handler);
        return NULL;
    }
    
    handler->error_history_size = 0;
    handler->callback_count = 0;
    handler->callback_capacity = 10;
    handler->recovery_callbacks = (SigmaErrorRecoveryCallback*)malloc(handler->callback_capacity * sizeof(SigmaErrorRecoveryCallback));
    if (!handler->recovery_callbacks) {
        free(handler->error_history);
        free(handler);
        return NULL;
    }
    
    handler->next_error_id = 1;
    handler->is_initialized = true;
    
    return handler;
}

void sigma_error_handler_destroy(SigmaErrorHandler* handler) {
    if (!handler) return;
    
    if (handler->error_history) {
        free(handler->error_history);
    }
    
    if (handler->recovery_callbacks) {
        free(handler->recovery_callbacks);
    }
    
    free(handler);
}

SigmaError sigma_error_create(SigmaErrorCategory category, SigmaSeverity severity, const char* message, const char* function, const char* file, int line) {
    SigmaError error;
    
    error.error_id = 0; // Will be set by handler
    error.category = category;
    error.severity = severity;
    error.recovery_action = SIGMA_RECOVERY_NONE;
    
    // Set context
    strncpy(error.context.function_name, function ? function : "unknown", sizeof(error.context.function_name) - 1);
    strncpy(error.context.file_name, file ? file : "unknown", sizeof(error.context.file_name) - 1);
    error.context.line_number = line;
    error.context.timestamp = sigma_get_timestamp();
    error.context.thread_id = sigma_get_thread_id();
    error.context.process_id = sigma_get_process_id();
    strcpy(error.context.additional_info, "");
    
    // Set messages
    strncpy(error.error_message, message ? message : "Unknown error", sizeof(error.error_message) - 1);
    strcpy(error.technical_details, "");
    strcpy(error.user_friendly_message, message ? message : "An error occurred");
    
    error.is_recoverable = true;
    error.is_automated_recovery = false;
    error.occurrence_count = 1;
    error.first_occurrence = error.context.timestamp;
    error.last_occurrence = error.context.timestamp;
    
    return error;
}

SigmaError sigma_error_create_with_context(SigmaErrorCategory category, SigmaSeverity severity, const char* message, const SigmaErrorContext* context) {
    SigmaError error;
    
    error.error_id = 0;
    error.category = category;
    error.severity = severity;
    error.recovery_action = SIGMA_RECOVERY_NONE;
    
    if (context) {
        error.context = *context;
    } else {
        memset(&error.context, 0, sizeof(SigmaErrorContext));
        error.context.timestamp = sigma_get_timestamp();
        error.context.thread_id = sigma_get_thread_id();
        error.context.process_id = sigma_get_process_id();
    }
    
    strncpy(error.error_message, message ? message : "Unknown error", sizeof(error.error_message) - 1);
    strcpy(error.technical_details, "");
    strcpy(error.user_friendly_message, message ? message : "An error occurred");
    
    error.is_recoverable = true;
    error.is_automated_recovery = false;
    error.occurrence_count = 1;
    error.first_occurrence = error.context.timestamp;
    error.last_occurrence = error.context.timestamp;
    
    return error;
}

SigmaRecoveryAction sigma_error_handle(SigmaErrorHandler* handler, const SigmaError* error) {
    if (!handler || !error) return SIGMA_RECOVERY_NONE;
    
    // Update statistics
    handler->statistics.total_errors++;
    handler->statistics.errors_by_category[error->category]++;
    handler->statistics.errors_by_severity[error->severity]++;
    handler->statistics.last_error_time = error->context.timestamp;
    
    // Log error
    if (handler->config.enable_error_logging) {
        sigma_error_log(error);
    }
    
    // Notify user
    if (handler->config.enable_user_notifications && error->severity >= handler->config.min_notification_level) {
        sigma_error_notify_user(error);
    }
    
    // Add to error history
    if (handler->error_history_size < handler->error_history_capacity) {
        SigmaError* stored_error = &handler->error_history[handler->error_history_size];
        *stored_error = *error;
        stored_error->error_id = handler->next_error_id++;
        handler->error_history_size++;
    }
    
    // Determine recovery action
    SigmaRecoveryAction recovery_action = SIGMA_RECOVERY_NONE;
    
    // Check for existing similar errors
    for (size_t i = 0; i < handler->error_history_size - 1; i++) {
        SigmaError* existing_error = &handler->error_history[i];
        
        if (existing_error->category == error->category && 
            existing_error->severity == error->severity &&
            strcmp(existing_error->error_message, error->error_message) == 0) {
            
            // Update occurrence count
            existing_error->occurrence_count++;
            existing_error->last_occurrence = error->context.timestamp;
            
            // Use previous recovery action if it was successful
            if (existing_error->recovery_action != SIGMA_RECOVERY_NONE) {
                recovery_action = existing_error->recovery_action;
            }
            
            break;
        }
    }
    
    // Determine recovery action based on error type and severity
    if (recovery_action == SIGMA_RECOVERY_NONE) {
        recovery_action = sigma_determine_recovery_action(error);
    }
    
    // Execute recovery action
    if (handler->config.enable_automated_recovery && recovery_action != SIGMA_RECOVERY_NONE) {
        uint64_t recovery_start = sigma_get_timestamp();
        
        // Try recovery callbacks
        for (size_t i = 0; i < handler->callback_count; i++) {
            if (handler->recovery_callbacks[i]) {
                SigmaRecoveryAction callback_result = handler->recovery_callbacks[i](error, NULL);
                if (callback_result != SIGMA_RECOVERY_NONE) {
                    recovery_action = callback_result;
                    handler->statistics.automated_recoveries++;
                    break;
                }
            }
        }
        
        uint64_t recovery_end = sigma_get_timestamp();
        double recovery_time = (double)(recovery_end - recovery_start);
        
        // Update recovery statistics
        if (recovery_action != SIGMA_RECOVERY_NONE) {
            handler->statistics.successful_recoveries++;
            
            // Update average recovery time
            if (handler->statistics.average_recovery_time_ms == 0) {
                handler->statistics.average_recovery_time_ms = recovery_time;
            } else {
                handler->statistics.average_recovery_time_ms = 
                    (handler->statistics.average_recovery_time_ms + recovery_time) / 2.0;
            }
        } else {
            handler->statistics.failed_recoveries++;
        }
    }
    
    // Predictive analysis
    if (handler->config.enable_predictive_analysis) {
        sigma_error_predict_issues(handler, error->category);
    }
    
    return recovery_action;
}

SigmaRecoveryAction sigma_error_handle_with_recovery(SigmaErrorHandler* handler, const SigmaError* error, SigmaErrorRecoveryCallback callback) {
    if (!handler || !error) return SIGMA_RECOVERY_NONE;
    
    // Register the callback temporarily
    SigmaRecoveryAction original_callbacks[10];
    size_t original_count = handler->callback_count;
    
    // Save original callbacks
    for (size_t i = 0; i < original_count; i++) {
        original_callbacks[i] = handler->recovery_callbacks[i];
    }
    
    // Add new callback
    if (sigma_error_register_recovery_callback(handler, callback)) {
        // Handle the error
        SigmaRecoveryAction result = sigma_error_handle(handler, error);
        
        // Restore original callbacks
        handler->callback_count = original_count;
        for (size_t i = 0; i < original_count; i++) {
            handler->recovery_callbacks[i] = original_callbacks[i];
        }
        
        return result;
    }
    
    return SIGMA_RECOVERY_NONE;
}

void sigma_error_log(const SigmaError* error) {
    if (!error) return;
    
    const char* category_names[] = {
        "SYSTEM", "MEMORY", "FILESYSTEM", "NETWORK", "SECURITY",
        "PERFORMANCE", "USER_INTERFACE", "AUTOMATION", "PERSONALIZATION", "CUSTOM"
    };
    
    const char* severity_names[] = {
        "INFO", "WARNING", "ERROR", "CRITICAL", "FATAL"
    };
    
    printf("[ERROR] [%s] [%s] %s\n", 
           category_names[error->category],
           severity_names[error->severity],
           error->error_message);
    
    printf("  Function: %s\n", error->context.function_name);
    printf("  File: %s:%d\n", error->context.file_name, error->context.line_number);
    printf("  Timestamp: %llu\n", error->context.timestamp);
    printf("  Thread ID: %u\n", error->context.thread_id);
    printf("  Process ID: %u\n", error->context.process_id);
    
    if (strlen(error->technical_details) > 0) {
        printf("  Technical Details: %s\n", error->technical_details);
    }
    
    if (strlen(error->context.additional_info) > 0) {
        printf("  Additional Info: %s\n", error->context.additional_info);
    }
}

void sigma_error_notify_user(const SigmaError* error) {
    if (!error) return;
    
    // Create user-friendly notification
    char notification[1024];
    
    switch (error->severity) {
        case SIGMA_SEVERITY_INFO:
            snprintf(notification, sizeof(notification), 
                    "ℹ️ Information: %s", error->user_friendly_message);
            break;
            
        case SIGMA_SEVERITY_WARNING:
            snprintf(notification, sizeof(notification), 
                    "⚠️ Warning: %s", error->user_friendly_message);
            break;
            
        case SIGMA_SEVERITY_ERROR:
            snprintf(notification, sizeof(notification), 
                    "❌ Error: %s\n\nThe system will attempt to recover automatically.", error->user_friendly_message);
            break;
            
        case SIGMA_SEVERITY_CRITICAL:
            snprintf(notification, sizeof(notification), 
                    "🚨 Critical Error: %s\n\nImmediate action is required.", error->user_friendly_message);
            break;
            
        case SIGMA_SEVERITY_FATAL:
            snprintf(notification, sizeof(notification), 
                    "💀 Fatal Error: %s\n\nThe system cannot continue.", error->user_friendly_message);
            break;
            
        default:
            snprintf(notification, sizeof(notification), 
                    "⚙️ System Message: %s", error->user_friendly_message);
            break;
    }
    
    // Send notification to user interface
    sigma_ui_show_notification(notification, error->severity);
    
    // Log the notification
    printf("[USER_NOTIFICATION] %s\n", notification);
}

bool sigma_error_register_recovery_callback(SigmaErrorHandler* handler, SigmaErrorRecoveryCallback callback) {
    if (!handler || !callback) return false;
    
    // Check if we need to expand the callback array
    if (handler->callback_count >= handler->callback_capacity) {
        size_t new_capacity = handler->callback_capacity * 2;
        SigmaErrorRecoveryCallback* new_callbacks = (SigmaErrorRecoveryCallback*)realloc(
            handler->recovery_callbacks, new_capacity * sizeof(SigmaErrorRecoveryCallback));
        
        if (!new_callbacks) return false;
        
        handler->recovery_callbacks = new_callbacks;
        handler->callback_capacity = new_capacity;
    }
    
    handler->recovery_callbacks[handler->callback_count++] = callback;
    return true;
}

SigmaErrorStatistics* sigma_error_get_statistics(SigmaErrorHandler* handler) {
    if (!handler) return NULL;
    
    return &handler->statistics;
}

void sigma_error_clear_history(SigmaErrorHandler* handler) {
    if (!handler) return;
    
    handler->error_history_size = 0;
    memset(&handler->statistics, 0, sizeof(SigmaErrorStatistics));
}

SigmaError* sigma_error_get_recent_errors(SigmaErrorHandler* handler, size_t count) {
    if (!handler || count == 0) return NULL;
    
    size_t start_index = 0;
    if (handler->error_history_size > count) {
        start_index = handler->error_history_size - count;
    }
    
    size_t actual_count = handler->error_history_size - start_index;
    SigmaError* recent_errors = (SigmaError*)malloc(actual_count * sizeof(SigmaError));
    if (!recent_errors) return NULL;
    
    for (size_t i = 0; i < actual_count; i++) {
        recent_errors[i] = handler->error_history[start_index + i];
    }
    
    return recent_errors;
}

bool sigma_error_predict_issues(SigmaErrorHandler* handler, SigmaErrorCategory category) {
    if (!handler) return false;
    
    // Simple predictive analysis based on error patterns
    uint32_t category_errors = handler->statistics.errors_by_category[category];
    uint32_t total_errors = handler->statistics.total_errors;
    
    if (total_errors == 0) return false;
    
    // If this category represents more than 20% of all errors, predict issues
    double category_ratio = (double)category_errors / (double)total_errors;
    
    if (category_ratio > 0.2) {
        // Check if errors are increasing
        uint64_t current_time = sigma_get_timestamp();
        uint64_t time_window = 60000000; // 1 minute in microseconds
        
        uint32_t recent_errors = 0;
        for (size_t i = 0; i < handler->error_history_size; i++) {
            SigmaError* error = &handler->error_history[i];
            if (error->category == category && 
                (current_time - error->context.timestamp) < time_window) {
                recent_errors++;
            }
        }
        
        // If we have more than 5 errors in the last minute, predict issues
        if (recent_errors > 5) {
            printf("[PREDICTION] High error rate detected in category %d. System may become unstable.\n", category);
            return true;
        }
    }
    
    return false;
}

// Recovery action determination
SigmaRecoveryAction sigma_determine_recovery_action(const SigmaError* error) {
    if (!error) return SIGMA_RECOVERY_NONE;
    
    switch (error->category) {
        case SIGMA_CATEGORY_MEMORY:
            switch (error->severity) {
                case SIGMA_SEVERITY_INFO:
                case SIGMA_SEVERITY_WARNING:
                    return SIGMA_RECOVERY_RETRY;
                case SIGMA_SEVERITY_ERROR:
                    return SIGMA_RECOVERY_RESET;
                case SIGMA_SEVERITY_CRITICAL:
                    return SIGMA_RECOVERY_RESTART;
                case SIGMA_SEVERITY_FATAL:
                    return SIGMA_RECOVERY_USER_INTERVENTION;
            }
            break;
            
        case SIGMA_CATEGORY_FILESYSTEM:
            switch (error->severity) {
                case SIGMA_SEVERITY_INFO:
                case SIGMA_SEVERITY_WARNING:
                    return SIGMA_RECOVERY_RETRY;
                case SIGMA_SEVERITY_ERROR:
                    return SIGMA_RECOVERY_REPAIR;
                case SIGMA_SEVERITY_CRITICAL:
                    return SIGMA_RECOVERY_FALLBACK;
                case SIGMA_SEVERITY_FATAL:
                    return SIGMA_RECOVERY_USER_INTERVENTION;
            }
            break;
            
        case SIGMA_CATEGORY_NETWORK:
            switch (error->severity) {
                case SIGMA_SEVERITY_INFO:
                case SIGMA_SEVERITY_WARNING:
                    return SIGMA_RECOVERY_RETRY;
                case SIGMA_SEVERITY_ERROR:
                    return SIGMA_RECOVERY_RESET;
                case SIGMA_SEVERITY_CRITICAL:
                    return SIGMA_RECOVERY_FALLBACK;
                case SIGMA_SEVERITY_FATAL:
                    return SIGMA_RECOVERY_USER_INTERVENTION;
            }
            break;
            
        case SIGMA_CATEGORY_SECURITY:
            switch (error->severity) {
                case SIGMA_SEVERITY_INFO:
                case SIGMA_SEVERITY_WARNING:
                    return SIGMA_RECOVERY_AUTOMATED;
                case SIGMA_SEVERITY_ERROR:
                    return SIGMA_RECOVERY_REPAIR;
                case SIGMA_SEVERITY_CRITICAL:
                    return SIGMA_RECOVERY_RESTART;
                case SIGMA_SEVERITY_FATAL:
                    return SIGMA_RECOVERY_USER_INTERVENTION;
            }
            break;
            
        case SIGMA_CATEGORY_PERFORMANCE:
            switch (error->severity) {
                case SIGMA_SEVERITY_INFO:
                case SIGMA_SEVERITY_WARNING:
                    return SIGMA_RECOVERY_AUTOMATED;
                case SIGMA_SEVERITY_ERROR:
                    return SIGMA_RECOVERY_RESET;
                case SIGMA_SEVERITY_CRITICAL:
                    return SIGMA_RECOVERY_RESTART;
                case SIGMA_SEVERITY_FATAL:
                    return SIGMA_RECOVERY_USER_INTERVENTION;
            }
            break;
            
        case SIGMA_CATEGORY_USER_INTERFACE:
            switch (error->severity) {
                case SIGMA_SEVERITY_INFO:
                case SIGMA_SEVERITY_WARNING:
                    return SIGMA_RECOVERY_RETRY;
                case SIGMA_SEVERITY_ERROR:
                    return SIGMA_RECOVERY_RESET;
                case SIGMA_SEVERITY_CRITICAL:
                    return SIGMA_RECOVERY_FALLBACK;
                case SIGMA_SEVERITY_FATAL:
                    return SIGMA_RECOVERY_RESTART;
            }
            break;
            
        case SIGMA_CATEGORY_AUTOMATION:
            switch (error->severity) {
                case SIGMA_SEVERITY_INFO:
                case SIGMA_SEVERITY_WARNING:
                    return SIGMA_RECOVERY_AUTOMATED;
                case SIGMA_SEVERITY_ERROR:
                    return SIGMA_RECOVERY_RETRY;
                case SIGMA_SEVERITY_CRITICAL:
                    return SIGMA_RECOVERY_RESET;
                case SIGMA_SEVERITY_FATAL:
                    return SIGMA_RECOVERY_USER_INTERVENTION;
            }
            break;
            
        case SIGMA_CATEGORY_PERSONALIZATION:
            switch (error->severity) {
                case SIGMA_SEVERITY_INFO:
                case SIGMA_SEVERITY_WARNING:
                    return SIGMA_RECOVERY_AUTOMATED;
                case SIGMA_SEVERITY_ERROR:
                    return SIGMA_RECOVERY_FALLBACK;
                case SIGMA_SEVERITY_CRITICAL:
                    return SIGMA_RECOVERY_RESET;
                case SIGMA_SEVERITY_FATAL:
                    return SIGMA_RECOVERY_USER_INTERVENTION;
            }
            break;
            
        case SIGMA_CATEGORY_SYSTEM:
        default:
            switch (error->severity) {
                case SIGMA_SEVERITY_INFO:
                case SIGMA_SEVERITY_WARNING:
                    return SIGMA_RECOVERY_RETRY;
                case SIGMA_SEVERITY_ERROR:
                    return SIGMA_RECOVERY_RESET;
                case SIGMA_SEVERITY_CRITICAL:
                    return SIGMA_RECOVERY_RESTART;
                case SIGMA_SEVERITY_FATAL:
                    return SIGMA_RECOVERY_USER_INTERVENTION;
            }
            break;
    }
    
    return SIGMA_RECOVERY_NONE;
}

// Utility functions
uint64_t sigma_get_timestamp(void) {
    static uint64_t timestamp_counter = 1000000000;
    return timestamp_counter++;
}

uint32_t sigma_get_thread_id(void) {
    static uint32_t thread_counter = 1;
    return thread_counter++;
}

uint32_t sigma_get_process_id(void) {
    return 1234; // Simplified process ID
}

void sigma_ui_show_notification(const char* message, SigmaSeverity severity) {
    // This would integrate with the UI system
    printf("[UI_NOTIFICATION] %s\n", message);
}

// Initialize global error handler
void sigma_init_error_handling(void) {
    if (!global_error_handler) {
        SigmaErrorHandlerConfig config = {
            .enable_automated_recovery = true,
            .enable_user_notifications = true,
            .enable_error_logging = true,
            .enable_error_statistics = true,
            .enable_predictive_analysis = true,
            .max_error_history = 1000,
            .recovery_retry_count = 3,
            .recovery_timeout_ms = 5000,
            .min_notification_level = SIGMA_SEVERITY_WARNING
        };
        
        global_error_handler = sigma_error_handler_init(&config);
    }
}

// Cleanup global error handler
void sigma_cleanup_error_handling(void) {
    if (global_error_handler) {
        sigma_error_handler_destroy(global_error_handler);
        global_error_handler = NULL;
    }
}

// Get global error handler
SigmaErrorHandler* sigma_get_error_handler(void) {
    return global_error_handler;
}

