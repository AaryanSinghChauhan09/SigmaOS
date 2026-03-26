/*
 * SigmaOS Comprehensive Command Library
 * ======================================
 * Complete collection of bash commands from all Linux distributions plus custom
 * SigmaOS commands for automation, customization, personalization, data science,
 * machine learning, graph plotting, and camera features
 */

#ifndef SIGMA_COMMAND_LIBRARY_H
#define SIGMA_COMMAND_LIBRARY_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

// Command Categories
typedef enum {
    SIGMA_CMD_SYSTEM = 0,
    SIGMA_CMD_PACKAGE,
    SIGMA_CMD_FILE,
    SIGMA_CMD_NETWORK,
    SIGMA_CMD_PROCESS,
    SIGMA_CMD_USER,
    SIGMA_CMD_DISK,
    SIGMA_CMD_AUTOMATION,
    SIGMA_CMD_CUSTOMIZATION,
    SIGMA_CMD_PERSONALIZATION,
    SIGMA_CMD_DATA_SCIENCE,
    SIGMA_CMD_ML,
    SIGMA_CMD_VISUALIZATION,
    SIGMA_CMD_CAMERA,
    SIGMA_CMD_SETUP,
    SIGMA_CMD_SECURITY,
    SIGMA_CMD_QUANTUM,
    SIGMA_CMD_CATEGORY_COUNT
} SigmaCommandCategory;

// Command Structure
typedef struct {
    char name[256];
    char description[1024];
    char syntax[2048];
    char example[2048];
    char source_distro[128];
    bool is_custom;
    bool requires_root;
    SigmaCommandCategory category;
    char usage_notes[1024];
} SigmaCommand;

// Command Library Manager
typedef struct {
    SigmaCommand* commands;
    uint32_t command_count;
    uint32_t command_capacity;
    char distro_sources[20][64];
    uint32_t distro_count;
    char documentation[500000];
} SigmaCommandLibrary;

// Global Command Library
static SigmaCommandLibrary* g_command_library = NULL;

// Initialize Command Library
void sigma_command_library_initialize(void);

// Load commands from all Linux distros
void sigma_load_ubuntu_commands(void);
void sigma_load_fedora_commands(void);
void sigma_load_debian_commands(void);
void sigma_load_arch_commands(void);
void sigma_load_gentoo_commands(void);
void sigma_load_opensuse_commands(void);
void sigma_load_centos_commands(void);
void sigma_load_alpine_commands(void);
void sigma_load_void_commands(void);
void sigma_load_nixos_commands(void);

// Load custom SigmaOS commands
void sigma_load_automation_commands(void);
void sigma_load_customization_commands(void);
void sigma_load_personalization_commands(void);
void sigma_load_data_science_commands(void);
void sigma_load_ml_commands(void);
void sigma_load_visualization_commands(void);
void sigma_load_camera_commands(void);
void sigma_load_setup_commands(void);
void sigma_load_security_commands(void);
void sigma_load_quantum_commands(void);

// Search and execute commands
SigmaCommand* sigma_find_command(const char* name);
bool sigma_execute_command(const char* name, char** args, uint32_t arg_count);
void sigma_list_commands_by_category(SigmaCommandCategory category);
void sigma_search_commands(const char* keyword);

// Print command help
void sigma_print_command_help(const char* name);
void sigma_print_category_help(SigmaCommandCategory category);
void sigma_print_all_commands(void);

// Generate command documentation
void sigma_generate_command_documentation(char* output, size_t output_size);

// Cleanup
void sigma_command_library_cleanup(void);

#endif // SIGMA_COMMAND_LIBRARY_H
