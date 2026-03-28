/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS Init System (Native Core)
 * =================================
 * Complete system initialization and service management
 */

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <time.h>

// Service states
typedef enum {
    SERVICE_STOPPED = 0,
    SERVICE_STARTING = 1,
    SERVICE_RUNNING = 2,
    SERVICE_STOPPING = 3,
    SERVICE_FAILED = 4,
    SERVICE_RESTARTING = 5
} service_state_t;

// Service types
typedef enum {
    SERVICE_TYPE_SIMPLE = 1,
    SERVICE_TYPE_FORKING = 2,
    SERVICE_TYPE_ONESHOT = 3,
    SERVICE_TYPE_DBUS = 4,
    SERVICE_TYPE_NOTIFY = 5,
    SERVICE_TYPE_IDLE = 6
} service_type_t;

// Restart policies
typedef enum {
    RESTART_NO = 0,
    RESTART_ON_SUCCESS = 1,
    RESTART_ON_FAILURE = 2,
    RESTART_ON_ABNORMAL = 3,
    RESTART_ALWAYS = 4,
    RESTART_UNLESS_STOPPED = 5
} restart_policy_t;

// Service dependency types
typedef enum {
    DEP_REQUIRES = 1,
    DEP_REQUIRES_OVERRIDABLE = 2,
    DEP_REQUISITE = 3,
    DEP_WANTS = 4,
    DEP_BINDS_TO = 5,
    DEP_PART_OF = 6,
    DEP_CONFLICTS = 7,
    DEP_BEFORE = 8,
    DEP_AFTER = 9,
    DEP_ON_FAILURE = 10
} dependency_type_t;

// Maximum limits
#define MAX_SERVICES 256
#define MAX_DEPS_PER_SERVICE 16
#define MAX_ENV_VARS 64
#define MAX_SERVICE_NAME 64
#define MAX_EXEC_ARGS 32
#define MAX_ENV_LEN 256

// Service structure
typedef struct {
    char name[MAX_SERVICE_NAME];
    char description[256];
    service_type_t type;
    service_state_t state;
    restart_policy_t restart_policy;
    
    // Execution parameters
    char exec_path[256];
    char *exec_args[MAX_EXEC_ARGS];
    char working_dir[256];
    char *environment[MAX_ENV_VARS];
    uid_t uid;
    gid_t gid;
    
    // Resource limits
    uint64_t memory_limit;
    uint64_t cpu_limit;
    uint32_t file_limit;
    
    // Timing
    uint32_t start_timeout;
    uint32_t stop_timeout;
    uint32_t restart_delay;
    
    // Dependencies
    struct {
        dependency_type_t type;
        char service_name[MAX_SERVICE_NAME];
    } dependencies[MAX_DEPS_PER_SERVICE];
    int dependency_count;
    
    // Runtime information
    pid_t pid;
    time_t start_time;
    time_t last_restart_time;
    uint32_t restart_count;
    int exit_code;
    bool enabled;
    bool mandatory;
    
    // Callbacks
    void (*start_callback)(struct service *svc);
    void (*stop_callback)(struct service *svc);
    void (*restart_callback)(struct service *svc);
} service_t;

// System target structure
typedef struct {
    char name[MAX_SERVICE_NAME];
    char description[256];
    service_t *services[MAX_SERVICES];
    int service_count;
    bool is_default;
} target_t;

// Global init system state
static service_t services[MAX_SERVICES];
static target_t targets[MAX_SERVICES];
static int service_count = 0;
static int target_count = 0;
static target_t *current_target = NULL;
static bool system_shutdown = false;

// Logging levels
typedef enum {
    LOG_EMERG = 0,
    LOG_ALERT = 1,
    LOG_CRIT = 2,
    LOG_ERR = 3,
    LOG_WARNING = 4,
    LOG_NOTICE = 5,
    LOG_INFO = 6,
    LOG_DEBUG = 7
} log_level_t;

// Logging function
void sigma_init_log(log_level_t level, const char *service, const char *format, ...) {
    // This would interface with the system logger
    // For now, just print to console
    const char *level_str[] = {"EMERG", "ALERT", "CRIT", "ERR", "WARN", "NOTICE", "INFO", "DEBUG"};
    printf("[%s] %s: ", level_str[level], service ? service : "init");
    
    va_list args;
    va_start(args, format);
    vprintf(format, args);
    va_end(args);
    printf("\n");
}

// Initialize init system
void sigma_init_system_init(void) {
    // Clear service array
    for (int i = 0; i < MAX_SERVICES; i++) {
        memset(&services[i], 0, sizeof(service_t));
        services[i].state = SERVICE_STOPPED;
    }
    
    // Clear target array
    for (int i = 0; i < MAX_SERVICES; i++) {
        memset(&targets[i], 0, sizeof(target_t));
    }
    
    service_count = 0;
    target_count = 0;
    current_target = NULL;
    system_shutdown = false;
    
    sigma_init_log(LOG_INFO, NULL, "SigmaOS init system initialized");
}

// Register a service
int sigma_init_register_service(const char *name, const char *description, 
                               service_type_t type, const char *exec_path) {
    if (service_count >= MAX_SERVICES) {
        sigma_init_log(LOG_ERR, NULL, "Maximum services reached");
        return -1;
    }
    
    service_t *svc = &services[service_count];
    strncpy(svc->name, name, MAX_SERVICE_NAME - 1);
    strncpy(svc->description, description, sizeof(svc->description) - 1);
    strncpy(svc->exec_path, exec_path, sizeof(svc->exec_path) - 1);
    
    svc->type = type;
    svc->state = SERVICE_STOPPED;
    svc->restart_policy = RESTART_ON_FAILURE;
    svc->uid = 0;
    svc->gid = 0;
    svc->start_timeout = 30;
    svc->stop_timeout = 30;
    svc->restart_delay = 5;
    svc->memory_limit = 0; // No limit
    svc->cpu_limit = 0; // No limit
    svc->file_limit = 1024;
    svc->enabled = true;
    svc->mandatory = false;
    
    service_count++;
    sigma_init_log(LOG_INFO, NULL, "Registered service: %s", name);
    
    return service_count - 1;
}

// Add dependency to service
int sigma_init_add_dependency(int service_id, const char *dep_name, dependency_type_t type) {
    if (service_id < 0 || service_id >= service_count) {
        return -1;
    }
    
    service_t *svc = &services[service_id];
    if (svc->dependency_count >= MAX_DEPS_PER_SERVICE) {
        return -1;
    }
    
    svc->dependencies[svc->dependency_count].type = type;
    strncpy(svc->dependencies[svc->dependency_count].service_name, dep_name, 
            MAX_SERVICE_NAME - 1);
    svc->dependency_count++;
    
    return 0;
}

// Find service by name
service_t* sigma_init_find_service(const char *name) {
    for (int i = 0; i < service_count; i++) {
        if (strcmp(services[i].name, name) == 0) {
            return &services[i];
        }
    }
    return NULL;
}

// Check if service dependencies are satisfied
bool sigma_init_check_dependencies(service_t *svc) {
    for (int i = 0; i < svc->dependency_count; i++) {
        service_t *dep = sigma_init_find_service(svc->dependencies[i].service_name);
        if (!dep) {
            sigma_init_log(LOG_ERR, svc->name, "Dependency not found: %s", 
                          svc->dependencies[i].service_name);
            return false;
        }
        
        switch (svc->dependencies[i].type) {
            case DEP_REQUIRES:
            case DEP_REQUISITE:
                if (dep->state != SERVICE_RUNNING) {
                    sigma_init_log(LOG_DEBUG, svc->name, "Waiting for dependency: %s", 
                                  dep->name);
                    return false;
                }
                break;
            case DEP_WANTS:
                // Wants are optional
                break;
            case DEP_CONFLICTS:
                if (dep->state != SERVICE_STOPPED) {
                    sigma_init_log(LOG_ERR, svc->name, "Conflicting service running: %s", 
                                  dep->name);
                    return false;
                }
                break;
            default:
                break;
        }
    }
    return true;
}

// Start a service
int sigma_init_start_service(service_t *svc) {
    if (!svc || svc->state != SERVICE_STOPPED) {
        return -1;
    }
    
    if (!svc->enabled) {
        sigma_init_log(LOG_INFO, svc->name, "Service disabled, not starting");
        return 0;
    }
    
    if (!sigma_init_check_dependencies(svc)) {
        sigma_init_log(LOG_WARNING, svc->name, "Dependencies not satisfied");
        return -1;
    }
    
    sigma_init_log(LOG_INFO, svc->name, "Starting service");
    svc->state = SERVICE_STARTING;
    svc->start_time = time(NULL);
    
    // Fork and execute service
    pid_t pid = fork();
    if (pid == 0) {
        // Child process
        // Set working directory
        if (strlen(svc->working_dir) > 0) {
            chdir(svc->working_dir);
        }
        
        // Set user/group if specified
        if (svc->uid != 0) {
            setuid(svc->uid);
        }
        if (svc->gid != 0) {
            setgid(svc->gid);
        }
        
        // Set environment variables
        for (int i = 0; i < MAX_ENV_VARS && svc->environment[i]; i++) {
            putenv(svc->environment[i]);
        }
        
        // Execute service
        execv(svc->exec_path, svc->exec_args);
        
        // If we get here, exec failed
        sigma_init_log(LOG_ERR, svc->name, "Failed to execute service");
        exit(1);
    } else if (pid > 0) {
        // Parent process
        svc->pid = pid;
        svc->state = SERVICE_RUNNING;
        sigma_init_log(LOG_INFO, svc->name, "Service started with PID %d", pid);
        
        if (svc->start_callback) {
            svc->start_callback(svc);
        }
        
        return 0;
    } else {
        // Fork failed
        svc->state = SERVICE_FAILED;
        sigma_init_log(LOG_ERR, svc->name, "Failed to fork service");
        return -1;
    }
}

// Stop a service
int sigma_init_stop_service(service_t *svc) {
    if (!svc || (svc->state != SERVICE_RUNNING && svc->state != SERVICE_STARTING)) {
        return -1;
    }
    
    sigma_init_log(LOG_INFO, svc->name, "Stopping service");
    svc->state = SERVICE_STOPPING;
    
    // Send SIGTERM
    if (kill(svc->pid, SIGTERM) == 0) {
        // Wait for graceful shutdown
        time_t start_time = time(NULL);
        while (svc->state == SERVICE_STOPPING) {
            int status;
            pid_t result = waitpid(svc->pid, &status, WNOHANG);
            if (result == svc->pid) {
                svc->state = SERVICE_STOPPED;
                svc->exit_code = WEXITSTATUS(status);
                sigma_init_log(LOG_INFO, svc->name, "Service stopped with exit code %d", 
                              svc->exit_code);
                break;
            } else if (result == 0) {
                // Still running
                if (time(NULL) - start_time > svc->stop_timeout) {
                    sigma_init_log(LOG_WARNING, svc->name, "Service did not stop gracefully, killing");
                    kill(svc->pid, SIGKILL);
                    svc->state = SERVICE_STOPPED;
                    break;
                }
                sleep(1);
            } else {
                // Error
                sigma_init_log(LOG_ERR, svc->name, "Error waiting for service to stop");
                svc->state = SERVICE_FAILED;
                break;
            }
        }
    } else {
        sigma_init_log(LOG_ERR, svc->name, "Failed to send SIGTERM to service");
        svc->state = SERVICE_FAILED;
    }
    
    if (svc->stop_callback) {
        svc->stop_callback(svc);
    }
    
    return 0;
}

// Restart a service
int sigma_init_restart_service(service_t *svc) {
    if (!svc) return -1;
    
    sigma_init_log(LOG_INFO, svc->name, "Restarting service");
    
    if (svc->state == SERVICE_RUNNING || svc->state == SERVICE_STARTING) {
        sigma_init_stop_service(svc);
    }
    
    // Wait before restarting
    if (svc->restart_delay > 0) {
        sleep(svc->restart_delay);
    }
    
    svc->restart_count++;
    svc->last_restart_time = time(NULL);
    
    int result = sigma_init_start_service(svc);
    
    if (svc->restart_callback) {
        svc->restart_callback(svc);
    }
    
    return result;
}

// Handle service exit
void sigma_init_handle_service_exit(pid_t pid, int exit_code) {
    for (int i = 0; i < service_count; i++) {
        if (services[i].pid == pid) {
            services[i].exit_code = exit_code;
            
            if (services[i].state == SERVICE_RUNNING || services[i].state == SERVICE_STARTING) {
                sigma_init_log(LOG_INFO, services[i].name, "Service exited with code %d", exit_code);
                
                // Check if service should be restarted
                bool should_restart = false;
                switch (services[i].restart_policy) {
                    case RESTART_ALWAYS:
                        should_restart = true;
                        break;
                    case RESTART_ON_FAILURE:
                        should_restart = (exit_code != 0);
                        break;
                    case RESTART_ON_SUCCESS:
                        should_restart = (exit_code == 0);
                        break;
                    case RESTART_UNLESS_STOPPED:
                        should_restart = !system_shutdown;
                        break;
                    default:
                        should_restart = false;
                        break;
                }
                
                if (should_restart && services[i].enabled) {
                    sigma_init_log(LOG_INFO, services[i].name, "Restarting service due to policy");
                    sigma_init_restart_service(&services[i]);
                } else {
                    services[i].state = SERVICE_STOPPED;
                }
            }
            break;
        }
    }
}

// Create a target
int sigma_init_create_target(const char *name, const char *description, bool is_default) {
    if (target_count >= MAX_SERVICES) {
        return -1;
    }
    
    target_t *target = &targets[target_count];
    strncpy(target->name, name, MAX_SERVICE_NAME - 1);
    strncpy(target->description, description, sizeof(target->description) - 1);
    target->is_default = is_default;
    target->service_count = 0;
    
    target_count++;
    sigma_init_log(LOG_INFO, NULL, "Created target: %s", name);
    
    return target_count - 1;
}

// Add service to target
int sigma_init_add_service_to_target(int target_id, int service_id) {
    if (target_id < 0 || target_id >= target_count || 
        service_id < 0 || service_id >= service_count) {
        return -1;
    }
    
    target_t *target = &targets[target_id];
    if (target->service_count >= MAX_SERVICES) {
        return -1;
    }
    
    target->services[target->service_count] = &services[service_id];
    target->service_count++;
    
    return 0;
}

// Switch to target
int sigma_init_switch_target(const char *target_name) {
    target_t *target = NULL;
    for (int i = 0; i < target_count; i++) {
        if (strcmp(targets[i].name, target_name) == 0) {
            target = &targets[i];
            break;
        }
    }
    
    if (!target) {
        sigma_init_log(LOG_ERR, NULL, "Target not found: %s", target_name);
        return -1;
    }
    
    sigma_init_log(LOG_INFO, NULL, "Switching to target: %s", target->description);
    
    // Stop services not in new target
    for (int i = 0; i < service_count; i++) {
        bool in_new_target = false;
        for (int j = 0; j < target->service_count; j++) {
            if (target->services[j] == &services[i]) {
                in_new_target = true;
                break;
            }
        }
        
        if (!in_new_target && services[i].state == SERVICE_RUNNING) {
            sigma_init_stop_service(&services[i]);
        }
    }
    
    // Start services in new target
    for (int i = 0; i < target->service_count; i++) {
        if (target->services[i]->state == SERVICE_STOPPED) {
            sigma_init_start_service(target->services[i]);
        }
    }
    
    current_target = target;
    return 0;
}

// Main init system loop
void sigma_init_main_loop(void) {
    sigma_init_log(LOG_INFO, NULL, "Starting init system main loop");
    
    // Switch to default target
    for (int i = 0; i < target_count; i++) {
        if (targets[i].is_default) {
            sigma_init_switch_target(targets[i].name);
            break;
        }
    }
    
    // Main event loop
    while (!system_shutdown) {
        // Wait for child processes
        int status;
        pid_t pid = waitpid(-1, &status, WNOHANG);
        if (pid > 0) {
            sigma_init_handle_service_exit(pid, WEXITSTATUS(status));
        } else if (pid < 0 && errno != ECHILD) {
            sigma_init_log(LOG_ERR, NULL, "Error in waitpid: %s", strerror(errno));
        }
        
        // Check for service timeouts
        time_t current_time = time(NULL);
        for (int i = 0; i < service_count; i++) {
            if (services[i].state == SERVICE_STARTING) {
                if (current_time - services[i].start_time > services[i].start_timeout) {
                    sigma_init_log(LOG_WARNING, services[i].name, "Service start timeout");
                    sigma_init_stop_service(&services[i]);
                    services[i].state = SERVICE_FAILED;
                }
            }
        }
        
        // Sleep briefly to avoid busy waiting
        usleep(100000); // 100ms
    }
    
    sigma_init_log(LOG_INFO, NULL, "Init system shutting down");
}

// Shutdown system
void sigma_init_shutdown(void) {
    sigma_init_log(LOG_INFO, NULL, "Initiating system shutdown");
    system_shutdown = true;
    
    // Stop all services in reverse order
    for (int i = service_count - 1; i >= 0; i--) {
        if (services[i].state == SERVICE_RUNNING || services[i].state == SERVICE_STARTING) {
            sigma_init_stop_service(&services[i]);
        }
    }
}

// Get service status
typedef struct {
    char name[MAX_SERVICE_NAME];
    service_state_t state;
    pid_t pid;
    time_t start_time;
    uint32_t restart_count;
} service_status_t;

int sigma_init_get_service_status(service_status_t *status_array, int max_count) {
    int count = 0;
    for (int i = 0; i < service_count && count < max_count; i++) {
        strncpy(status_array[count].name, services[i].name, MAX_SERVICE_NAME - 1);
        status_array[count].state = services[i].state;
        status_array[count].pid = services[i].pid;
        status_array[count].start_time = services[i].start_time;
        status_array[count].restart_count = services[i].restart_count;
        count++;
    }
    return count;
}

