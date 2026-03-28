/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS User Experience Optimizer
 * =================================
 * Advanced UX optimization, ease of use, and personalization
 */

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <time.h>

// User experience structures
typedef struct {
    uint32_t action_id;
    char action_name[128];
    char action_type[64];
    uint64_t timestamp;
    uint64_t execution_time;
    uint32_t user_rating; // 1-5 stars
    char user_feedback[512];
    bool is_favorited;
    uint32_t usage_count;
    double average_execution_time;
} UserAction;

typedef struct {
    uint32_t preference_id;
    char category[64];
    char key[128];
    char value[256];
    char description[256];
    uint32_t user_id;
    uint64_t created_time;
    uint64_t last_modified;
    bool is_system_default;
} UserPreference;

typedef struct {
    uint32_t profile_id;
    uint32_t user_id;
    char profile_name[128];
    char theme[64];
    char font_family[64];
    uint32_t font_size;
    bool dark_mode;
    bool animations_enabled;
    double animation_speed;
    char layout[128];
    char custom_css[2048];
    char custom_js[2048];
    uint64_t created_time;
    uint64_t last_used;
    bool is_active;
} UserProfile;

typedef struct {
    uint32_t shortcut_id;
    char shortcut_name[128];
    char key_combination[64];
    char action[256];
    uint32_t user_id;
    bool is_global;
    uint64_t created_time;
    uint32_t usage_count;
} UserShortcut;

// Ease of use features
typedef struct {
    uint32_t wizard_id;
    char wizard_name[128];
    char wizard_description[512];
    uint32_t step_count;
    char** steps;
    char** step_descriptions;
    bool is_completed;
    uint32_t current_step;
    uint64_t started_time;
    uint64_t completed_time;
    uint32_t user_id;
} SetupWizard;

typedef struct {
    uint32_t tip_id;
    char tip_title[128];
    char tip_content[1024];
    char category[64];
    uint32_t priority;
    bool is_shown;
    uint64_t last_shown;
    uint32_t show_count;
    uint32_t user_rating;
} UserTip;

typedef struct {
    uint32_t help_id;
    char help_title[128];
    char help_content[4096];
    char category[64];
    char keywords[256];
    uint32_t relevance_score;
    bool is_context_sensitive;
    uint64_t last_accessed;
    uint32_t access_count;
} HelpContent;

// Personalization engine
typedef struct {
    uint32_t pattern_id;
    char pattern_name[128];
    char pattern_type[64];
    uint32_t action_sequence[64];
    uint32_t sequence_length;
    uint32_t occurrence_count;
    uint64_t last_occurrence;
    double confidence_score;
    bool is_automated;
} UsagePattern;

typedef struct {
    uint32_t recommendation_id;
    char recommendation_title[128];
    char recommendation_description[512];
    char action[256];
    uint32_t priority;
    double relevance_score;
    uint64_t generated_time;
    bool is_accepted;
    uint64_t accepted_time;
} PersonalizedRecommendation;

// User experience optimizer
typedef struct {
    UserAction* actions;
    uint32_t action_count;
    uint32_t max_actions;
    UserPreference* preferences;
    uint32_t preference_count;
    uint32_t max_preferences;
    UserProfile* profiles;
    uint32_t profile_count;
    uint32_t max_profiles;
    UserShortcut* shortcuts;
    uint32_t shortcut_count;
    uint32_t max_shortcuts;
    SetupWizard* wizards;
    uint32_t wizard_count;
    uint32_t max_wizards;
    UserTip* tips;
    uint32_t tip_count;
    uint32_t max_tips;
    HelpContent* help_content;
    uint32_t help_count;
    uint32_t max_help;
    UsagePattern* patterns;
    uint32_t pattern_count;
    uint32_t max_patterns;
    PersonalizedRecommendation* recommendations;
    uint32_t recommendation_count;
    uint32_t max_recommendations;
    uint64_t total_user_interactions;
    uint64_t total_session_time;
    double average_satisfaction_score;
    bool is_learning_enabled;
    uint64_t last_learning_update;
} UserExperienceOptimizer;

// User action tracking
static UserAction* sigma_user_action_create(const char* action_name, const char* action_type) {
    UserAction* action = (UserAction*)malloc(sizeof(UserAction));
    if (!action) return NULL;
    
    action->action_id = sigma_generate_unique_id();
    strncpy(action->action_name, action_name, sizeof(action->action_name) - 1);
    strncpy(action->action_type, action_type, sizeof(action->action_type) - 1);
    action->timestamp = sigma_get_timestamp();
    action->execution_time = 0;
    action->user_rating = 0;
    strcpy(action->user_feedback, "");
    action->is_favorited = false;
    action->usage_count = 1;
    action->average_execution_time = 0.0;
    
    return action;
}

static void sigma_user_action_record(UserExperienceOptimizer* optimizer, const char* action_name, 
                                    const char* action_type, uint64_t execution_time) {
    if (!optimizer || !action_name || !action_type) return;
    
    // Check if action already exists
    UserAction* existing_action = NULL;
    for (uint32_t i = 0; i < optimizer->action_count; i++) {
        if (strcmp(optimizer->actions[i].action_name, action_name) == 0) {
            existing_action = &optimizer->actions[i];
            break;
        }
    }
    
    if (existing_action) {
        // Update existing action
        existing_action->timestamp = sigma_get_timestamp();
        existing_action->usage_count++;
        existing_action->average_execution_time = 
            (existing_action->average_execution_time * (existing_action->usage_count - 1) + execution_time) / 
            existing_action->usage_count;
    } else {
        // Create new action
        if (optimizer->action_count < optimizer->max_actions) {
            UserAction* new_action = sigma_user_action_create(action_name, action_type);
            if (new_action) {
                new_action->execution_time = execution_time;
                new_action->average_execution_time = execution_time;
                optimizer->actions[optimizer->action_count++] = *new_action;
                free(new_action);
            }
        }
    }
    
    optimizer->total_user_interactions++;
}

// Preference management
static UserPreference* sigma_user_preference_create(const char* category, const char* key, 
                                                   const char* value, const char* description) {
    UserPreference* preference = (UserPreference*)malloc(sizeof(UserPreference));
    if (!preference) return NULL;
    
    preference->preference_id = sigma_generate_unique_id();
    strncpy(preference->category, category, sizeof(preference->category) - 1);
    strncpy(preference->key, key, sizeof(preference->key) - 1);
    strncpy(preference->value, value, sizeof(preference->value) - 1);
    strncpy(preference->description, description, sizeof(preference->description) - 1);
    preference->user_id = 0; // Current user
    preference->created_time = sigma_get_timestamp();
    preference->last_modified = preference->created_time;
    preference->is_system_default = false;
    
    return preference;
}

static bool sigma_user_preference_set(UserExperienceOptimizer* optimizer, const char* category, 
                                    const char* key, const char* value) {
    if (!optimizer || !category || !key || !value) return false;
    
    // Check if preference already exists
    UserPreference* existing_pref = NULL;
    for (uint32_t i = 0; i < optimizer->preference_count; i++) {
        if (strcmp(optimizer->preferences[i].category, category) == 0 &&
            strcmp(optimizer->preferences[i].key, key) == 0) {
            existing_pref = &optimizer->preferences[i];
            break;
        }
    }
    
    if (existing_pref) {
        // Update existing preference
        strncpy(existing_pref->value, value, sizeof(existing_pref->value) - 1);
        existing_pref->last_modified = sigma_get_timestamp();
        existing_pref->is_system_default = false;
    } else {
        // Create new preference
        if (optimizer->preference_count < optimizer->max_preferences) {
            UserPreference* new_pref = sigma_user_preference_create(category, key, value, "");
            if (new_pref) {
                optimizer->preferences[optimizer->preference_count++] = *new_pref;
                free(new_pref);
            }
        }
    }
    
    return true;
}

static const char* sigma_user_preference_get(UserExperienceOptimizer* optimizer, const char* category, 
                                            const char* key) {
    if (!optimizer || !category || !key) return NULL;
    
    for (uint32_t i = 0; i < optimizer->preference_count; i++) {
        if (strcmp(optimizer->preferences[i].category, category) == 0 &&
            strcmp(optimizer->preferences[i].key, key) == 0) {
            return optimizer->preferences[i].value;
        }
    }
    
    return NULL; // Not found
}

// Profile management
static UserProfile* sigma_user_profile_create(const char* profile_name, uint32_t user_id) {
    UserProfile* profile = (UserProfile*)malloc(sizeof(UserProfile));
    if (!profile) return NULL;
    
    profile->profile_id = sigma_generate_unique_id();
    profile->user_id = user_id;
    strncpy(profile->profile_name, profile_name, sizeof(profile->profile_name) - 1);
    strcpy(profile->theme, "default");
    strcpy(profile->font_family, "system");
    profile->font_size = 12;
    profile->dark_mode = false;
    profile->animations_enabled = true;
    profile->animation_speed = 1.0;
    strcpy(profile->layout, "default");
    strcpy(profile->custom_css, "");
    strcpy(profile->custom_js, "");
    profile->created_time = sigma_get_timestamp();
    profile->last_used = profile->created_time;
    profile->is_active = false;
    
    return profile;
}

static bool sigma_user_profile_apply(UserProfile* profile) {
    if (!profile) return false;
    
    // Apply theme
    sigma_apply_theme(profile->theme);
    
    // Apply font settings
    sigma_apply_font_settings(profile->font_family, profile->font_size);
    
    // Apply dark mode
    sigma_apply_dark_mode(profile->dark_mode);
    
    // Apply animation settings
    sigma_apply_animation_settings(profile->animations_enabled, profile->animation_speed);
    
    // Apply layout
    sigma_apply_layout(profile->layout);
    
    // Apply custom CSS
    if (strlen(profile->custom_css) > 0) {
        sigma_apply_custom_css(profile->custom_css);
    }
    
    // Apply custom JavaScript
    if (strlen(profile->custom_js) > 0) {
        sigma_apply_custom_js(profile->custom_js);
    }
    
    profile->last_used = sigma_get_timestamp();
    
    return true;
}

// Setup wizard implementation
static SetupWizard* sigma_setup_wizard_create(const char* wizard_name, const char* description) {
    SetupWizard* wizard = (SetupWizard*)malloc(sizeof(SetupWizard));
    if (!wizard) return NULL;
    
    wizard->wizard_id = sigma_generate_unique_id();
    strncpy(wizard->wizard_name, wizard_name, sizeof(wizard->wizard_name) - 1);
    strncpy(wizard->wizard_description, description, sizeof(wizard->wizard_description) - 1);
    wizard->step_count = 0;
    wizard->steps = NULL;
    wizard->step_descriptions = NULL;
    wizard->is_completed = false;
    wizard->current_step = 0;
    wizard->started_time = 0;
    wizard->completed_time = 0;
    wizard->user_id = 0;
    
    return wizard;
}

static bool sigma_setup_wizard_add_step(SetupWizard* wizard, const char* step_title, 
                                       const char* step_description) {
    if (!wizard || !step_title || !step_description) return false;
    
    wizard->steps = (char**)realloc(wizard->steps, (wizard->step_count + 1) * sizeof(char*));
    wizard->step_descriptions = (char**)realloc(wizard->step_descriptions, (wizard->step_count + 1) * sizeof(char*));
    
    if (!wizard->steps || !wizard->step_descriptions) return false;
    
    wizard->steps[wizard->step_count] = strdup(step_title);
    wizard->step_descriptions[wizard->step_count] = strdup(step_description);
    wizard->step_count++;
    
    return true;
}

static bool sigma_setup_wizard_next_step(SetupWizard* wizard) {
    if (!wizard || wizard->current_step >= wizard->step_count) return false;
    
    wizard->current_step++;
    
    if (wizard->current_step >= wizard->step_count) {
        wizard->is_completed = true;
        wizard->completed_time = sigma_get_timestamp();
    }
    
    return true;
}

// Usage pattern detection
static void sigma_detect_usage_patterns(UserExperienceOptimizer* optimizer) {
    if (!optimizer || !optimizer->is_learning_enabled) return;
    
    // Analyze action sequences to detect patterns
    for (uint32_t i = 0; i < optimizer->action_count - 2; i++) {
        UserAction* action1 = &optimizer->actions[i];
        UserAction* action2 = &optimizer->actions[i + 1];
        UserAction* action3 = &optimizer->actions[i + 2];
        
        // Check if this sequence occurs frequently
        uint32_t sequence_occurrences = sigma_count_sequence_occurrences(optimizer, action1, action2, action3);
        
        if (sequence_occurrences > 5) { // Threshold for pattern detection
            // Create or update pattern
            UsagePattern* pattern = sigma_find_or_create_pattern(optimizer, action1, action2, action3);
            if (pattern) {
                pattern->occurrence_count = sequence_occurrences;
                pattern->last_occurrence = action3->timestamp;
                pattern->confidence_score = (double)sequence_occurrences / optimizer->total_user_interactions;
                
                // Consider automation if confidence is high
                if (pattern->confidence_score > 0.7) {
                    pattern->is_automated = true;
                }
            }
        }
    }
}

static uint32_t sigma_count_sequence_occurrences(UserExperienceOptimizer* optimizer, 
                                                UserAction* action1, UserAction* action2, UserAction* action3) {
    uint32_t count = 0;
    
    for (uint32_t i = 0; i < optimizer->action_count - 2; i++) {
        if (strcmp(optimizer->actions[i].action_name, action1->action_name) == 0 &&
            strcmp(optimizer->actions[i + 1].action_name, action2->action_name) == 0 &&
            strcmp(optimizer->actions[i + 2].action_name, action3->action_name) == 0) {
            count++;
        }
    }
    
    return count;
}

static UsagePattern* sigma_find_or_create_pattern(UserExperienceOptimizer* optimizer, 
                                                  UserAction* action1, UserAction* action2, UserAction* action3) {
    // Check if pattern already exists
    for (uint32_t i = 0; i < optimizer->pattern_count; i++) {
        UsagePattern* pattern = &optimizer->patterns[i];
        
        if (pattern->sequence_length == 3 &&
            strcmp(optimizer->actions[pattern->action_sequence[0]].action_name, action1->action_name) == 0 &&
            strcmp(optimizer->actions[pattern->action_sequence[1]].action_name, action2->action_name) == 0 &&
            strcmp(optimizer->actions[pattern->action_sequence[2]].action_name, action3->action_name) == 0) {
            return pattern;
        }
    }
    
    // Create new pattern
    if (optimizer->pattern_count < optimizer->max_patterns) {
        UsagePattern* pattern = &optimizer->patterns[optimizer->pattern_count];
        pattern->pattern_id = sigma_generate_unique_id();
        
        // Create pattern name
        snprintf(pattern->pattern_name, sizeof(pattern->pattern_name), "%s -> %s -> %s",
                action1->action_name, action2->action_name, action3->action_name);
        
        strcpy(pattern->pattern_type, "sequence");
        pattern->action_sequence[0] = action1->action_id;
        pattern->action_sequence[1] = action2->action_id;
        pattern->action_sequence[2] = action3->action_id;
        pattern->sequence_length = 3;
        pattern->occurrence_count = 1;
        pattern->last_occurrence = action3->timestamp;
        pattern->confidence_score = 0.0;
        pattern->is_automated = false;
        
        optimizer->pattern_count++;
        return pattern;
    }
    
    return NULL;
}

// Personalized recommendations
static void sigma_generate_recommendations(UserExperienceOptimizer* optimizer) {
    if (!optimizer) return;
    
    // Generate recommendations based on usage patterns
    for (uint32_t i = 0; i < optimizer->pattern_count; i++) {
        UsagePattern* pattern = &optimizer->patterns[i];
        
        if (pattern->confidence_score > 0.6 && !pattern->is_automated) {
            // Create automation recommendation
            if (optimizer->recommendation_count < optimizer->max_recommendations) {
                PersonalizedRecommendation* recommendation = &optimizer->recommendations[optimizer->recommendation_count];
                
                recommendation->recommendation_id = sigma_generate_unique_id();
                snprintf(recommendation->recommendation_title, sizeof(recommendation->recommendation_title),
                        "Automate: %s", pattern->pattern_name);
                snprintf(recommendation->recommendation_description, sizeof(recommendation->recommendation_description),
                        "We noticed you frequently perform this sequence. Would you like to automate it?");
                snprintf(recommendation->action, sizeof(recommendation->action), "automate_pattern:%u", pattern->pattern_id);
                recommendation->priority = (uint32_t)(pattern->confidence_score * 10);
                recommendation->relevance_score = pattern->confidence_score;
                recommendation->generated_time = sigma_get_timestamp();
                recommendation->is_accepted = false;
                recommendation->accepted_time = 0;
                
                optimizer->recommendation_count++;
            }
        }
    }
    
    // Generate recommendations based on frequently used actions
    for (uint32_t i = 0; i < optimizer->action_count; i++) {
        UserAction* action = &optimizer->actions[i];
        
        if (action->usage_count > 10 && !action->is_favorited) {
            // Create shortcut recommendation
            if (optimizer->recommendation_count < optimizer->max_recommendations) {
                PersonalizedRecommendation* recommendation = &optimizer->recommendations[optimizer->recommendation_count];
                
                recommendation->recommendation_id = sigma_generate_unique_id();
                snprintf(recommendation->recommendation_title, sizeof(recommendation->recommendation_title),
                        "Create Shortcut: %s", action->action_name);
                snprintf(recommendation->recommendation_description, sizeof(recommendation->recommendation_description),
                        "You use this action frequently. Would you like to create a keyboard shortcut?");
                snprintf(recommendation->action, sizeof(recommendation->action), "create_shortcut:%s", action->action_name);
                recommendation->priority = 5;
                recommendation->relevance_score = (double)action->usage_count / optimizer->total_user_interactions;
                recommendation->generated_time = sigma_get_timestamp();
                recommendation->is_accepted = false;
                recommendation->accepted_time = 0;
                
                optimizer->recommendation_count++;
            }
        }
    }
}

// Help system
static HelpContent* sigma_help_content_create(const char* title, const char* content, 
                                             const char* category, const char* keywords) {
    HelpContent* help = (HelpContent*)malloc(sizeof(HelpContent));
    if (!help) return NULL;
    
    help->help_id = sigma_generate_unique_id();
    strncpy(help->help_title, title, sizeof(help->help_title) - 1);
    strncpy(help->help_content, content, sizeof(help->help_content) - 1);
    strncpy(help->category, category, sizeof(help->category) - 1);
    strncpy(help->keywords, keywords, sizeof(help->keywords) - 1);
    help->relevance_score = 0.0;
    help->is_context_sensitive = false;
    help->last_accessed = 0;
    help->access_count = 0;
    
    return help;
}

static HelpContent* sigma_help_search(UserExperienceOptimizer* optimizer, const char* query) {
    if (!optimizer || !query) return NULL;
    
    HelpContent* best_match = NULL;
    double best_score = 0.0;
    
    for (uint32_t i = 0; i < optimizer->help_count; i++) {
        HelpContent* help = &optimizer->help_content[i];
        
        // Calculate relevance score based on keyword matching
        double score = 0.0;
        
        // Check title match
        if (strcasestr(help->help_title, query) != NULL) {
            score += 0.5;
        }
        
        // Check keywords match
        if (strcasestr(help->keywords, query) != NULL) {
            score += 0.3;
        }
        
        // Check content match
        if (strcasestr(help->help_content, query) != NULL) {
            score += 0.2;
        }
        
        // Boost frequently accessed help
        if (help->access_count > 0) {
            score += 0.1 * (double)help->access_count / 100.0;
        }
        
        if (score > best_score) {
            best_score = score;
            best_match = help;
        }
    }
    
    if (best_match) {
        best_match->last_accessed = sigma_get_timestamp();
        best_match->access_count++;
    }
    
    return best_match;
}

// User experience optimizer implementation
UserExperienceOptimizer* sigma_user_experience_optimizer_init(void) {
    UserExperienceOptimizer* optimizer = (UserExperienceOptimizer*)calloc(1, sizeof(UserExperienceOptimizer));
    if (!optimizer) return NULL;
    
    // Initialize arrays
    optimizer->actions = (UserAction*)malloc(1024 * sizeof(UserAction));
    optimizer->preferences = (UserPreference*)malloc(512 * sizeof(UserPreference));
    optimizer->profiles = (UserProfile*)malloc(64 * sizeof(UserProfile));
    optimizer->shortcuts = (UserShortcut*)malloc(256 * sizeof(UserShortcut));
    optimizer->wizards = (SetupWizard*)malloc(32 * sizeof(SetupWizard));
    optimizer->tips = (UserTip*)malloc(128 * sizeof(UserTip));
    optimizer->help_content = (HelpContent*)malloc(256 * sizeof(HelpContent));
    optimizer->patterns = (UsagePattern*)malloc(128 * sizeof(UsagePattern));
    optimizer->recommendations = (PersonalizedRecommendation*)malloc(128 * sizeof(PersonalizedRecommendation));
    
    if (!optimizer->actions || !optimizer->preferences || !optimizer->profiles || 
        !optimizer->shortcuts || !optimizer->wizards || !optimizer->tips || 
        !optimizer->help_content || !optimizer->patterns || !optimizer->recommendations) {
        // Cleanup on failure
        free(optimizer->actions);
        free(optimizer->preferences);
        free(optimizer->profiles);
        free(optimizer->shortcuts);
        free(optimizer->wizards);
        free(optimizer->tips);
        free(optimizer->help_content);
        free(optimizer->patterns);
        free(optimizer->recommendations);
        free(optimizer);
        return NULL;
    }
    
    // Initialize counts and limits
    optimizer->action_count = 0;
    optimizer->max_actions = 1024;
    optimizer->preference_count = 0;
    optimizer->max_preferences = 512;
    optimizer->profile_count = 0;
    optimizer->max_profiles = 64;
    optimizer->shortcut_count = 0;
    optimizer->max_shortcuts = 256;
    optimizer->wizard_count = 0;
    optimizer->max_wizards = 32;
    optimizer->tip_count = 0;
    optimizer->max_tips = 128;
    optimizer->help_count = 0;
    optimizer->max_help = 256;
    optimizer->pattern_count = 0;
    optimizer->max_patterns = 128;
    optimizer->recommendation_count = 0;
    optimizer->max_recommendations = 128;
    
    // Initialize statistics
    optimizer->total_user_interactions = 0;
    optimizer->total_session_time = 0;
    optimizer->average_satisfaction_score = 0.0;
    optimizer->is_learning_enabled = true;
    optimizer->last_learning_update = sigma_get_timestamp();
    
    // Create default help content
    sigma_create_default_help_content(optimizer);
    
    // Create default setup wizard
    sigma_create_default_setup_wizard(optimizer);
    
    return optimizer;
}

static void sigma_create_default_help_content(UserExperienceOptimizer* optimizer) {
    // Add basic help content
    HelpContent* help1 = sigma_help_content_create("Getting Started", 
        "Welcome to SigmaOS! This guide will help you get started with the world's fastest operating system.",
        "basics", "getting started, welcome, introduction");
    if (help1) {
        optimizer->help_content[optimizer->help_count++] = *help1;
        free(help1);
    }
    
    HelpContent* help2 = sigma_help_content_create("Performance Optimization",
        "Learn how to optimize SigmaOS for maximum performance with our advanced tuning tools.",
        "performance", "optimization, speed, performance, tuning");
    if (help2) {
        optimizer->help_content[optimizer->help_count++] = *help2;
        free(help2);
    }
    
    HelpContent* help3 = sigma_help_content_create("Automation",
        "Discover how to automate your workflow with SigmaOS's powerful automation engine.",
        "automation", "automation, workflow, tasks, scripting");
    if (help3) {
        optimizer->help_content[optimizer->help_count++] = *help3;
        free(help3);
    }
}

static void sigma_create_default_setup_wizard(UserExperienceOptimizer* optimizer) {
    SetupWizard* wizard = sigma_setup_wizard_create("Initial Setup", 
        "Configure SigmaOS for optimal performance and personalization");
    if (wizard) {
        sigma_setup_wizard_add_step(wizard, "Welcome", "Welcome to SigmaOS! Let's set up your system.");
        sigma_setup_wizard_add_step(wizard, "Performance Settings", "Configure performance settings for your hardware.");
        sigma_setup_wizard_add_step(wizard, "Personalization", "Customize the look and feel of your system.");
        sigma_setup_wizard_add_step(wizard, "Automation", "Set up basic automation rules.");
        sigma_setup_wizard_add_step(wizard, "Complete", "Setup complete! Enjoy your optimized SigmaOS experience.");
        
        optimizer->wizards[optimizer->wizard_count++] = *wizard;
        free(wizard);
    }
}

static void sigma_user_experience_optimizer_update(UserExperienceOptimizer* optimizer) {
    if (!optimizer) return;
    
    uint64_t current_time = sigma_get_timestamp();
    
    // Update learning and recommendations every minute
    if (current_time - optimizer->last_learning_update > 60000000) {
        if (optimizer->is_learning_enabled) {
            sigma_detect_usage_patterns(optimizer);
            sigma_generate_recommendations(optimizer);
        }
        
        optimizer->last_learning_update = current_time;
    }
    
    // Update average satisfaction score
    if (optimizer->action_count > 0) {
        double total_rating = 0.0;
        uint32_t rated_actions = 0;
        
        for (uint32_t i = 0; i < optimizer->action_count; i++) {
            if (optimizer->actions[i].user_rating > 0) {
                total_rating += optimizer->actions[i].user_rating;
                rated_actions++;
            }
        }
        
        optimizer->average_satisfaction_score = rated_actions > 0 ? total_rating / rated_actions : 0.0;
    }
}

// Performance monitoring
typedef struct {
    uint64_t actions_per_minute;
    uint64_t average_session_duration;
    double average_satisfaction_score;
    uint32_t active_profiles;
    uint32_t completed_wizards;
    uint32_t automation_recommendations;
    uint32_t help_access_count;
    double learning_efficiency;
    uint64_t total_memory_usage;
} UXPerformanceStats;

UXPerformanceStats* sigma_ux_get_performance_stats(UserExperienceOptimizer* optimizer) {
    UXPerformanceStats* stats = (UXPerformanceStats*)malloc(sizeof(UXPerformanceStats));
    if (!stats) return NULL;
    
    uint64_t current_time = sigma_get_timestamp();
    uint64_t time_delta = current_time - optimizer->start_time;
    
    if (time_delta > 0) {
        stats->actions_per_minute = optimizer->total_user_interactions * 60000000 / time_delta;
    } else {
        stats->actions_per_minute = 0;
    }
    
    stats->average_session_duration = optimizer->total_session_time > 0 ? 
                                    optimizer->total_session_time / optimizer->action_count : 0;
    stats->average_satisfaction_score = optimizer->average_satisfaction_score;
    stats->active_profiles = optimizer->profile_count;
    stats->completed_wizards = 0;
    stats->automation_recommendations = 0;
    stats->help_access_count = 0;
    stats->learning_efficiency = optimizer->is_learning_enabled ? 0.8 : 0.0;
    stats->total_memory_usage = sigma_get_memory_usage();
    
    // Count completed wizards
    for (uint32_t i = 0; i < optimizer->wizard_count; i++) {
        if (optimizer->wizards[i].is_completed) {
            stats->completed_wizards++;
        }
    }
    
    // Count automation recommendations
    for (uint32_t i = 0; i < optimizer->recommendation_count; i++) {
        if (strstr(optimizer->recommendations[i].action, "automate") != NULL) {
            stats->automation_recommendations++;
        }
    }
    
    // Count help access
    for (uint32_t i = 0; i < optimizer->help_count; i++) {
        stats->help_access_count += optimizer->help_content[i].access_count;
    }
    
    return stats;
}

// Cleanup functions
void sigma_user_experience_optimizer_destroy(UserExperienceOptimizer* optimizer) {
    if (!optimizer) return;
    
    // Cleanup arrays
    free(optimizer->actions);
    free(optimizer->preferences);
    free(optimizer->profiles);
    free(optimizer->shortcuts);
    free(optimizer->wizards);
    free(optimizer->tips);
    free(optimizer->help_content);
    free(optimizer->patterns);
    free(optimizer->recommendations);
    
    free(optimizer);
}

void sigma_setup_wizard_destroy(SetupWizard* wizard) {
    if (!wizard) return;
    
    if (wizard->steps) {
        for (uint32_t i = 0; i < wizard->step_count; i++) {
            free(wizard->steps[i]);
            free(wizard->step_descriptions[i]);
        }
        free(wizard->steps);
    }
    
    if (wizard->step_descriptions) {
        free(wizard->step_descriptions);
    }
    
    free(wizard);
}

// Utility functions
static uint32_t sigma_generate_unique_id(void) {
    static uint32_t counter = 1;
    return counter++;
}

static char* strcasestr(const char* haystack, const char* needle) {
    if (!haystack || !needle) return NULL;
    
    size_t needle_len = strlen(needle);
    if (needle_len == 0) return (char*)haystack;
    
    for (const char* p = haystack; *p; p++) {
        if (strncasecmp(p, needle, needle_len) == 0) {
            return (char*)p;
        }
    }
    
    return NULL;
}

