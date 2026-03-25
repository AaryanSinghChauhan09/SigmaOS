/*
 * SigmaOS Automation Engine
 * =========================
 * Advanced automation, customization, and personalization system
 */

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <time.h>

// Automation task structures
typedef struct {
    uint32_t task_id;
    char name[256];
    char description[512];
    uint32_t task_type;
    uint32_t priority;
    uint32_t status;
    uint64_t created_time;
    uint64_t scheduled_time;
    uint64_t execution_time;
    uint64_t completion_time;
    uint32_t retry_count;
    uint32_t max_retries;
    char* parameters;
    char* results;
    bool is_recurring;
    uint64_t recurrence_interval;
    uint32_t dependency_count;
    uint32_t* dependencies;
} AutomationTask;

typedef struct {
    uint32_t trigger_id;
    uint32_t trigger_type;
    char condition[512];
    uint32_t action_count;
    uint32_t* actions;
    bool is_enabled;
    uint64_t last_triggered;
    uint64_t trigger_count;
} AutomationTrigger;

typedef struct {
    uint32_t workflow_id;
    char name[256];
    char description[512];
    uint32_t task_count;
    AutomationTask* tasks;
    uint32_t trigger_count;
    AutomationTrigger* triggers;
    uint32_t current_task;
    uint32_t status;
    uint64_t start_time;
    uint64_t completion_time;
    bool is_parallel;
    uint32_t max_concurrent_tasks;
} AutomationWorkflow;

// Personalization profiles
typedef struct {
    uint32_t profile_id;
    char name[256];
    char preferences[1024];
    char ui_theme[128];
    char font_family[64];
    uint32_t font_size;
    bool dark_mode;
    bool animations_enabled;
    double animation_speed;
    char custom_css[2048];
    char custom_js[2048];
    uint64_t created_time;
    uint64_t last_modified;
    bool is_active;
} PersonalizationProfile;

// Customization settings
typedef struct {
    uint32_t setting_id;
    char category[64];
    char key[128];
    char value[256];
    char description[256];
    uint32_t data_type;
    bool is_user_defined;
    uint64_t created_time;
    uint64_t last_modified;
} CustomizationSetting;

// Automation engine
typedef struct {
    AutomationTask* tasks;
    uint32_t task_count;
    uint32_t max_tasks;
    AutomationWorkflow* workflows;
    uint32_t workflow_count;
    uint32_t max_workflows;
    AutomationTrigger* triggers;
    uint32_t trigger_count;
    uint32_t max_triggers;
    PersonalizationProfile* profiles;
    uint32_t profile_count;
    uint32_t max_profiles;
    CustomizationSetting* settings;
    uint32_t setting_count;
    uint32_t max_settings;
    uint64_t total_tasks_executed;
    uint64_t successful_tasks;
    uint64_t failed_tasks;
    uint64_t total_execution_time;
    uint32_t active_tasks;
    uint32_t active_workflows;
    bool is_running;
    uint64_t last_maintenance;
} AutomationEngine;

// Task types
typedef enum {
    TASK_TYPE_SYSTEM,
    TASK_TYPE_FILE,
    TASK_TYPE_NETWORK,
    TASK_TYPE_APPLICATION,
    TASK_TYPE_SECURITY,
    TASK_TYPE_BACKUP,
    TASK_TYPE_MONITORING,
    TASK_TYPE_CUSTOM
} TaskType;

// Task status
typedef enum {
    STATUS_PENDING,
    STATUS_RUNNING,
    STATUS_COMPLETED,
    STATUS_FAILED,
    STATUS_CANCELLED,
    STATUS_RETRYING
} TaskStatus;

// Trigger types
typedef enum {
    TRIGGER_TIME_BASED,
    TRIGGER_EVENT_BASED,
    TRIGGER_CONDITION_BASED,
    TRIGGER_MANUAL,
    TRIGGER_SCHEDULED
} TriggerType;

// Automation task management
static AutomationTask* sigma_automation_task_create(const char* name, const char* description, 
                                                  uint32_t task_type, uint32_t priority) {
    AutomationTask* task = (AutomationTask*)malloc(sizeof(AutomationTask));
    if (!task) return NULL;
    
    task->task_id = sigma_generate_unique_id();
    strncpy(task->name, name, sizeof(task->name) - 1);
    strncpy(task->description, description, sizeof(task->description) - 1);
    task->task_type = task_type;
    task->priority = priority;
    task->status = STATUS_PENDING;
    task->created_time = sigma_get_timestamp();
    task->scheduled_time = 0;
    task->execution_time = 0;
    task->completion_time = 0;
    task->retry_count = 0;
    task->max_retries = 3;
    task->parameters = NULL;
    task->results = NULL;
    task->is_recurring = false;
    task->recurrence_interval = 0;
    task->dependency_count = 0;
    task->dependencies = NULL;
    
    return task;
}

static bool sigma_automation_task_execute(AutomationTask* task) {
    if (!task) return false;
    
    task->status = STATUS_RUNNING;
    task->execution_time = sigma_get_timestamp();
    
    bool success = false;
    
    switch (task->task_type) {
        case TASK_TYPE_SYSTEM:
            success = sigma_execute_system_task(task);
            break;
        case TASK_TYPE_FILE:
            success = sigma_execute_file_task(task);
            break;
        case TASK_TYPE_NETWORK:
            success = sigma_execute_network_task(task);
            break;
        case TASK_TYPE_APPLICATION:
            success = sigma_execute_application_task(task);
            break;
        case TASK_TYPE_SECURITY:
            success = sigma_execute_security_task(task);
            break;
        case TASK_TYPE_BACKUP:
            success = sigma_execute_backup_task(task);
            break;
        case TASK_TYPE_MONITORING:
            success = sigma_execute_monitoring_task(task);
            break;
        case TASK_TYPE_CUSTOM:
            success = sigma_execute_custom_task(task);
            break;
        default:
            success = false;
            break;
    }
    
    task->completion_time = sigma_get_timestamp();
    
    if (success) {
        task->status = STATUS_COMPLETED;
    } else {
        task->status = STATUS_FAILED;
        if (task->retry_count < task->max_retries) {
            task->status = STATUS_RETRYING;
            task->retry_count++;
            // Schedule retry
            task->scheduled_time = sigma_get_timestamp() + (1000000 * task->retry_count); // 1 second * retry count
        }
    }
    
    return success;
}

static bool sigma_execute_system_task(AutomationTask* task) {
    if (!task || !task->parameters) return false;
    
    // Parse system command
    char command[512];
    if (sscanf(task->parameters, "command:%511s", command) != 1) {
        return false;
    }
    
    // Execute system command
    int result = sigma_system_command(command);
    
    // Store result
    char result_str[256];
    snprintf(result_str, sizeof(result_str), "exit_code:%d", result);
    task->results = strdup(result_str);
    
    return result == 0;
}

static bool sigma_execute_file_task(AutomationTask* task) {
    if (!task || !task->parameters) return false;
    
    char operation[64];
    char path[512];
    
    if (sscanf(task->parameters, "%63s:%511s", operation, path) != 2) {
        return false;
    }
    
    bool success = false;
    
    if (strcmp(operation, "copy") == 0) {
        char destination[512];
        if (sscanf(task->parameters, "%*[^:]:%*[^:]:%511s", destination) == 1) {
            success = sigma_copy_file(path, destination);
        }
    } else if (strcmp(operation, "move") == 0) {
        char destination[512];
        if (sscanf(task->parameters, "%*[^:]:%*[^:]:%511s", destination) == 1) {
            success = sigma_move_file(path, destination);
        }
    } else if (strcmp(operation, "delete") == 0) {
        success = sigma_delete_file(path);
    } else if (strcmp(operation, "create") == 0) {
        success = sigma_create_file(path);
    } else if (strcmp(operation, "backup") == 0) {
        success = sigma_backup_file(path);
    }
    
    return success;
}

static bool sigma_execute_network_task(AutomationTask* task) {
    if (!task || !task->parameters) return false;
    
    char operation[64];
    char url[512];
    
    if (sscanf(task->parameters, "%63s:%511s", operation, url) != 2) {
        return false;
    }
    
    bool success = false;
    
    if (strcmp(operation, "download") == 0) {
        char destination[512];
        if (sscanf(task->parameters, "%*[^:]:%*[^:]:%511s", destination) == 1) {
            success = sigma_download_file(url, destination);
        }
    } else if (strcmp(operation, "upload") == 0) {
        success = sigma_upload_file(url);
    } else if (strcmp(operation, "ping") == 0) {
        success = sigma_ping_host(url);
    } else if (strcmp(operation, "scan") == 0) {
        success = sigma_scan_network(url);
    }
    
    return success;
}

static bool sigma_execute_application_task(AutomationTask* task) {
    if (!task || !task->parameters) return false;
    
    char operation[64];
    char application[256];
    
    if (sscanf(task->parameters, "%63s:%255s", operation, application) != 2) {
        return false;
    }
    
    bool success = false;
    
    if (strcmp(operation, "launch") == 0) {
        success = sigma_launch_application(application);
    } else if (strcmp(operation, "close") == 0) {
        success = sigma_close_application(application);
    } else if (strcmp(operation, "restart") == 0) {
        sigma_close_application(application);
        success = sigma_launch_application(application);
    } else if (strcmp(operation, "install") == 0) {
        success = sigma_install_application(application);
    } else if (strcmp(operation, "uninstall") == 0) {
        success = sigma_uninstall_application(application);
    }
    
    return success;
}

// Workflow management
static AutomationWorkflow* sigma_automation_workflow_create(const char* name, const char* description) {
    AutomationWorkflow* workflow = (AutomationWorkflow*)malloc(sizeof(AutomationWorkflow));
    if (!workflow) return NULL;
    
    workflow->workflow_id = sigma_generate_unique_id();
    strncpy(workflow->name, name, sizeof(workflow->name) - 1);
    strncpy(workflow->description, description, sizeof(workflow->description) - 1);
    workflow->task_count = 0;
    workflow->tasks = NULL;
    workflow->trigger_count = 0;
    workflow->triggers = NULL;
    workflow->current_task = 0;
    workflow->status = STATUS_PENDING;
    workflow->start_time = 0;
    workflow->completion_time = 0;
    workflow->is_parallel = false;
    workflow->max_concurrent_tasks = 4;
    
    return workflow;
}

static bool sigma_automation_workflow_add_task(AutomationWorkflow* workflow, AutomationTask* task) {
    if (!workflow || !task) return false;
    
    workflow->tasks = (AutomationTask*)realloc(workflow->tasks, 
                                              (workflow->task_count + 1) * sizeof(AutomationTask));
    if (!workflow->tasks) return false;
    
    workflow->tasks[workflow->task_count] = *task;
    workflow->task_count++;
    
    return true;
}

static bool sigma_automation_workflow_execute(AutomationWorkflow* workflow) {
    if (!workflow || workflow->task_count == 0) return false;
    
    workflow->status = STATUS_RUNNING;
    workflow->start_time = sigma_get_timestamp();
    
    bool success = true;
    
    if (workflow->is_parallel) {
        // Execute tasks in parallel
        uint32_t active_count = 0;
        uint32_t completed_count = 0;
        
        for (uint32_t i = 0; i < workflow->task_count; i++) {
            if (active_count < workflow->max_concurrent_tasks) {
                sigma_automation_task_execute(&workflow->tasks[i]);
                active_count++;
            } else {
                // Wait for a task to complete
                for (uint32_t j = 0; j < i; j++) {
                    if (workflow->tasks[j].status == STATUS_COMPLETED || 
                        workflow->tasks[j].status == STATUS_FAILED) {
                        active_count--;
                        break;
                    }
                }
                i--; // Retry this task
            }
        }
        
        // Wait for all tasks to complete
        for (uint32_t i = 0; i < workflow->task_count; i++) {
            while (workflow->tasks[i].status == STATUS_RUNNING) {
                sigma_thread_sleep(100); // 100ms
            }
            
            if (workflow->tasks[i].status == STATUS_FAILED) {
                success = false;
            }
        }
    } else {
        // Execute tasks sequentially
        for (uint32_t i = 0; i < workflow->task_count; i++) {
            workflow->current_task = i;
            
            if (!sigma_automation_task_execute(&workflow->tasks[i])) {
                success = false;
                break;
            }
        }
    }
    
    workflow->completion_time = sigma_get_timestamp();
    workflow->status = success ? STATUS_COMPLETED : STATUS_FAILED;
    
    return success;
}

// Trigger management
static AutomationTrigger* sigma_automation_trigger_create(uint32_t trigger_type, const char* condition) {
    AutomationTrigger* trigger = (AutomationTrigger*)malloc(sizeof(AutomationTrigger));
    if (!trigger) return NULL;
    
    trigger->trigger_id = sigma_generate_unique_id();
    trigger->trigger_type = trigger_type;
    strncpy(trigger->condition, condition, sizeof(trigger->condition) - 1);
    trigger->action_count = 0;
    trigger->actions = NULL;
    trigger->is_enabled = true;
    trigger->last_triggered = 0;
    trigger->trigger_count = 0;
    
    return trigger;
}

static bool sigma_automation_trigger_evaluate(AutomationTrigger* trigger) {
    if (!trigger || !trigger->is_enabled) return false;
    
    bool condition_met = false;
    
    switch (trigger->trigger_type) {
        case TRIGGER_TIME_BASED:
            condition_met = sigma_evaluate_time_trigger(trigger);
            break;
        case TRIGGER_EVENT_BASED:
            condition_met = sigma_evaluate_event_trigger(trigger);
            break;
        case TRIGGER_CONDITION_BASED:
            condition_met = sigma_evaluate_condition_trigger(trigger);
            break;
        case TRIGGER_MANUAL:
            condition_met = false; // Manual triggers are handled separately
            break;
        case TRIGGER_SCHEDULED:
            condition_met = sigma_evaluate_scheduled_trigger(trigger);
            break;
        default:
            condition_met = false;
            break;
    }
    
    if (condition_met) {
        trigger->last_triggered = sigma_get_timestamp();
        trigger->trigger_count++;
        
        // Execute associated actions
        for (uint32_t i = 0; i < trigger->action_count; i++) {
            sigma_execute_action(trigger->actions[i]);
        }
    }
    
    return condition_met;
}

// Personalization management
static PersonalizationProfile* sigma_personalization_profile_create(const char* name) {
    PersonalizationProfile* profile = (PersonalizationProfile*)malloc(sizeof(PersonalizationProfile));
    if (!profile) return NULL;
    
    profile->profile_id = sigma_generate_unique_id();
    strncpy(profile->name, name, sizeof(profile->name) - 1);
    strcpy(profile->preferences, "");
    strcpy(profile->ui_theme, "default");
    strcpy(profile->font_family, "system");
    profile->font_size = 12;
    profile->dark_mode = false;
    profile->animations_enabled = true;
    profile->animation_speed = 1.0;
    strcpy(profile->custom_css, "");
    strcpy(profile->custom_js, "");
    profile->created_time = sigma_get_timestamp();
    profile->last_modified = profile->created_time;
    profile->is_active = false;
    
    return profile;
}

static bool sigma_personalization_profile_apply(PersonalizationProfile* profile) {
    if (!profile) return false;
    
    // Apply UI theme
    sigma_apply_ui_theme(profile->ui_theme);
    
    // Apply font settings
    sigma_apply_font_settings(profile->font_family, profile->font_size);
    
    // Apply dark mode
    sigma_apply_dark_mode(profile->dark_mode);
    
    // Apply animation settings
    sigma_apply_animation_settings(profile->animations_enabled, profile->animation_speed);
    
    // Apply custom CSS
    if (strlen(profile->custom_css) > 0) {
        sigma_apply_custom_css(profile->custom_css);
    }
    
    // Apply custom JavaScript
    if (strlen(profile->custom_js) > 0) {
        sigma_apply_custom_js(profile->custom_js);
    }
    
    // Apply preferences
    sigma_apply_preferences(profile->preferences);
    
    profile->last_modified = sigma_get_timestamp();
    
    return true;
}

// Customization management
static CustomizationSetting* sigma_customization_setting_create(const char* category, const char* key, 
                                                              const char* value, const char* description) {
    CustomizationSetting* setting = (CustomizationSetting*)malloc(sizeof(CustomizationSetting));
    if (!setting) return NULL;
    
    setting->setting_id = sigma_generate_unique_id();
    strncpy(setting->category, category, sizeof(setting->category) - 1);
    strncpy(setting->key, key, sizeof(setting->key) - 1);
    strncpy(setting->value, value, sizeof(setting->value) - 1);
    strncpy(setting->description, description, sizeof(setting->description) - 1);
    setting->data_type = sigma_determine_data_type(value);
    setting->is_user_defined = true;
    setting->created_time = sigma_get_timestamp();
    setting->last_modified = setting->created_time;
    
    return setting;
}

static bool sigma_customization_setting_apply(CustomizationSetting* setting) {
    if (!setting) return false;
    
    return sigma_apply_setting(setting->category, setting->key, setting->value);
}

// Automation engine implementation
AutomationEngine* sigma_automation_engine_init(void) {
    AutomationEngine* engine = (AutomationEngine*)calloc(1, sizeof(AutomationEngine));
    if (!engine) return NULL;
    
    engine->tasks = (AutomationTask*)malloc(1024 * sizeof(AutomationTask));
    engine->workflows = (AutomationWorkflow*)malloc(256 * sizeof(AutomationWorkflow));
    engine->triggers = (AutomationTrigger*)malloc(512 * sizeof(AutomationTrigger));
    engine->profiles = (PersonalizationProfile*)malloc(64 * sizeof(PersonalizationProfile));
    engine->settings = (CustomizationSetting*)malloc(1024 * sizeof(CustomizationSetting));
    
    if (!engine->tasks || !engine->workflows || !engine->triggers || !engine->profiles || !engine->settings) {
        free(engine->tasks);
        free(engine->workflows);
        free(engine->triggers);
        free(engine->profiles);
        free(engine->settings);
        free(engine);
        return NULL;
    }
    
    engine->task_count = 0;
    engine->max_tasks = 1024;
    engine->workflow_count = 0;
    engine->max_workflows = 256;
    engine->trigger_count = 0;
    engine->max_triggers = 512;
    engine->profile_count = 0;
    engine->max_profiles = 64;
    engine->setting_count = 0;
    engine->max_settings = 1024;
    
    engine->total_tasks_executed = 0;
    engine->successful_tasks = 0;
    engine->failed_tasks = 0;
    engine->total_execution_time = 0;
    engine->active_tasks = 0;
    engine->active_workflows = 0;
    engine->is_running = false;
    engine->last_maintenance = sigma_get_timestamp();
    
    return engine;
}

static void sigma_automation_engine_run(AutomationEngine* engine) {
    if (!engine) return;
    
    engine->is_running = true;
    
    while (engine->is_running) {
        uint64_t current_time = sigma_get_timestamp();
        
        // Process scheduled tasks
        for (uint32_t i = 0; i < engine->task_count; i++) {
            AutomationTask* task = &engine->tasks[i];
            
            if (task->status == STATUS_PENDING && task->scheduled_time > 0 && 
                current_time >= task->scheduled_time) {
                
                // Check dependencies
                bool dependencies_met = true;
                for (uint32_t j = 0; j < task->dependency_count; j++) {
                    uint32_t dep_id = task->dependencies[j];
                    AutomationTask* dep_task = sigma_find_task_by_id(engine, dep_id);
                    if (!dep_task || dep_task->status != STATUS_COMPLETED) {
                        dependencies_met = false;
                        break;
                    }
                }
                
                if (dependencies_met) {
                    sigma_automation_task_execute(task);
                    engine->total_tasks_executed++;
                    
                    if (task->status == STATUS_COMPLETED) {
                        engine->successful_tasks++;
                    } else {
                        engine->failed_tasks++;
                    }
                    
                    engine->total_execution_time += task->completion_time - task->execution_time;
                    
                    // Handle recurring tasks
                    if (task->is_recurring && task->status == STATUS_COMPLETED) {
                        task->scheduled_time = current_time + task->recurrence_interval;
                        task->status = STATUS_PENDING;
                        task->retry_count = 0;
                    }
                }
            }
        }
        
        // Evaluate triggers
        for (uint32_t i = 0; i < engine->trigger_count; i++) {
            sigma_automation_trigger_evaluate(&engine->triggers[i]);
        }
        
        // Execute workflows
        for (uint32_t i = 0; i < engine->workflow_count; i++) {
            AutomationWorkflow* workflow = &engine->workflows[i];
            
            if (workflow->status == STATUS_PENDING) {
                sigma_automation_workflow_execute(workflow);
                engine->active_workflows++;
            } else if (workflow->status == STATUS_COMPLETED || workflow->status == STATUS_FAILED) {
                engine->active_workflows--;
            }
        }
        
        // Maintenance tasks
        if (current_time - engine->last_maintenance > 60000000) { // Every minute
            sigma_automation_engine_maintenance(engine);
            engine->last_maintenance = current_time;
        }
        
        // Sleep for a short time to prevent busy waiting
        sigma_thread_sleep(1000); // 1ms
    }
}

static void sigma_automation_engine_maintenance(AutomationEngine* engine) {
    if (!engine) return;
    
    // Clean up completed tasks
    for (uint32_t i = 0; i < engine->task_count; i++) {
        AutomationTask* task = &engine->tasks[i];
        
        if (task->status == STATUS_COMPLETED || task->status == STATUS_FAILED) {
            // Remove old completed tasks (older than 1 hour)
            uint64_t current_time = sigma_get_timestamp();
            if (current_time - task->completion_time > 3600000000) { // 1 hour
                sigma_automation_task_cleanup(task);
                
                // Remove from array
                for (uint32_t j = i; j < engine->task_count - 1; j++) {
                    engine->tasks[j] = engine->tasks[j + 1];
                }
                engine->task_count--;
                i--;
            }
        }
    }
    
    // Optimize memory usage
    sigma_automation_engine_optimize_memory(engine);
}

// Performance monitoring
typedef struct {
    uint64_t tasks_per_second;
    uint64_t workflows_per_second;
    uint64_t triggers_per_second;
    double success_rate;
    double average_execution_time;
    uint32_t active_tasks;
    uint32_t active_workflows;
    uint64_t total_memory_usage;
    uint64_t cpu_usage;
} AutomationPerformanceStats;

AutomationPerformanceStats* sigma_automation_get_performance_stats(AutomationEngine* engine) {
    AutomationPerformanceStats* stats = (AutomationPerformanceStats*)malloc(sizeof(AutomationPerformanceStats));
    if (!stats) return NULL;
    
    uint64_t current_time = sigma_get_timestamp();
    uint64_t time_delta = current_time - engine->start_time;
    
    if (time_delta > 0) {
        stats->tasks_per_second = engine->total_tasks_executed * 1000000 / time_delta;
        stats->workflows_per_second = engine->active_workflows * 1000000 / time_delta;
        stats->triggers_per_second = engine->trigger_count * 1000000 / time_delta;
    } else {
        stats->tasks_per_second = 0;
        stats->workflows_per_second = 0;
        stats->triggers_per_second = 0;
    }
    
    stats->success_rate = engine->total_tasks_executed > 0 ? 
                         (double)engine->successful_tasks / engine->total_tasks_executed : 0.0;
    stats->average_execution_time = engine->total_tasks_executed > 0 ?
                                   (double)engine->total_execution_time / engine->total_tasks_executed : 0.0;
    stats->active_tasks = engine->active_tasks;
    stats->active_workflows = engine->active_workflows;
    stats->total_memory_usage = sigma_get_memory_usage();
    stats->cpu_usage = sigma_get_cpu_usage();
    
    return stats;
}

// Cleanup functions
void sigma_automation_engine_destroy(AutomationEngine* engine) {
    if (!engine) return;
    
    if (engine->tasks) {
        for (uint32_t i = 0; i < engine->task_count; i++) {
            sigma_automation_task_cleanup(&engine->tasks[i]);
        }
        free(engine->tasks);
    }
    
    if (engine->workflows) {
        for (uint32_t i = 0; i < engine->workflow_count; i++) {
            sigma_automation_workflow_cleanup(&engine->workflows[i]);
        }
        free(engine->workflows);
    }
    
    if (engine->triggers) {
        for (uint32_t i = 0; i < engine->trigger_count; i++) {
            sigma_automation_trigger_cleanup(&engine->triggers[i]);
        }
        free(engine->triggers);
    }
    
    if (engine->profiles) {
        for (uint32_t i = 0; i < engine->profile_count; i++) {
            sigma_personalization_profile_cleanup(&engine->profiles[i]);
        }
        free(engine->profiles);
    }
    
    if (engine->settings) {
        free(engine->settings);
    }
    
    free(engine);
}

void sigma_automation_task_cleanup(AutomationTask* task) {
    if (!task) return;
    
    if (task->parameters) {
        free(task->parameters);
    }
    
    if (task->results) {
        free(task->results);
    }
    
    if (task->dependencies) {
        free(task->dependencies);
    }
}

void sigma_automation_workflow_cleanup(AutomationWorkflow* workflow) {
    if (!workflow) return;
    
    if (workflow->tasks) {
        for (uint32_t i = 0; i < workflow->task_count; i++) {
            sigma_automation_task_cleanup(&workflow->tasks[i]);
        }
        free(workflow->tasks);
    }
    
    if (workflow->triggers) {
        for (uint32_t i = 0; i < workflow->trigger_count; i++) {
            sigma_automation_trigger_cleanup(&workflow->triggers[i]);
        }
        free(workflow->triggers);
    }
}

void sigma_automation_trigger_cleanup(AutomationTrigger* trigger) {
    if (!trigger) return;
    
    if (trigger->actions) {
        free(trigger->actions);
    }
}

void sigma_personalization_profile_cleanup(PersonalizationProfile* profile) {
    if (!profile) return;
    
    // No dynamic allocation in profile structure
}

// Utility functions
static uint32_t sigma_generate_unique_id(void) {
    static uint32_t counter = 1;
    return counter++;
}

static uint32_t sigma_determine_data_type(const char* value) {
    if (!value) return 0;
    
    // Check if it's a boolean
    if (strcmp(value, "true") == 0 || strcmp(value, "false") == 0) {
        return 1; // Boolean
    }
    
    // Check if it's an integer
    char* endptr;
    strtol(value, &endptr, 10);
    if (*endptr == '\0') {
        return 2; // Integer
    }
    
    // Check if it's a float
    strtod(value, &endptr);
    if (*endptr == '\0') {
        return 3; // Float
    }
    
    // Default to string
    return 4; // String
}
