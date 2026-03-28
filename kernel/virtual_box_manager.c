/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS Virtual Box Manager
 * ========================
 * Simple virtual machine management interface
 * One-click VM creation and management
 */

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>

// Include virtualization engine
#include "virtualization_engine.c"

// Quick VM Templates
typedef struct {
    char template_name[128];
    SigmaVMType vm_type;
    uint32_t default_cpu_cores;
    uint64_t default_memory_mb;
    uint64_t default_disk_gb;
    char default_iso[512];
    char description[256];
} SigmaVMTemplate;

// Predefined VM Templates
static SigmaVMTemplate vm_templates[] = {
    {
        "Windows 11",
        SIGMA_VM_WINDOWS,
        4,
        8192,
        64,
        "/var/lib/sigmaos/isos/windows11.iso",
        "Latest Windows 11 with all updates"
    },
    {
        "Windows 10",
        SIGMA_VM_WINDOWS,
        2,
        4096,
        32,
        "/var/lib/sigmaos/isos/windows10.iso",
        "Windows 10 with latest updates"
    },
    {
        "Ubuntu 22.04",
        SIGMA_VM_LINUX,
        2,
        2048,
        20,
        "/var/lib/sigmaos/isos/ubuntu-22.04.iso",
        "Ubuntu 22.04 LTS Desktop"
    },
    {
        "Ubuntu 24.04",
        SIGMA_VM_LINUX,
        2,
        4096,
        25,
        "/var/lib/sigmaos/isos/ubuntu-24.04.iso",
        "Ubuntu 24.04 LTS Desktop"
    },
    {
        "Fedora 39",
        SIGMA_VM_LINUX,
        2,
        4096,
        25,
        "/var/lib/sigmaos/isos/fedora-39.iso",
        "Fedora 39 Workstation"
    },
    {
        "Arch Linux",
        SIGMA_VM_LINUX,
        2,
        2048,
        20,
        "/var/lib/sigmaos/isos/archlinux.iso",
        "Arch Linux with latest packages"
    },
    {
        "macOS Monterey",
        SIGMA_VM_MACOS,
        4,
        8192,
        64,
        "/var/lib/sigmaos/isos/macos-monterey.iso",
        "macOS Monterey for virtualization"
    },
    {
        "macOS Ventura",
        SIGMA_VM_MACOS,
        4,
        8192,
        64,
        "/var/lib/sigmaos/isos/macos-ventura.iso",
        "macOS Ventura for virtualization"
    },
    {
        "Debian 12",
        SIGMA_VM_LINUX,
        2,
        2048,
        20,
        "/var/lib/sigmaos/isos/debian-12.iso",
        "Debian 12 Bookworm"
    },
    {
        "CentOS 9",
        SIGMA_VM_LINUX,
        2,
        2048,
        20,
        "/var/lib/sigmaos/isos/centos-9.iso",
        "CentOS Stream 9"
    }
};

static const uint32_t vm_template_count = sizeof(vm_templates) / sizeof(vm_templates[0]);

// Virtual Box Manager
typedef struct {
    SigmaVirtualizationManager* virt_manager;
    char current_directory[512];
    bool auto_start_enabled;
    bool auto_snapshot_enabled;
    uint32_t snapshot_interval_hours;
} SigmaVirtualBoxManager;

// Global Virtual Box Manager
static SigmaVirtualBoxManager* g_vbox_manager = NULL;

// Initialize Virtual Box Manager
SigmaVirtualBoxManager* sigma_virtual_box_manager_create(void) {
    SigmaVirtualBoxManager* manager = (SigmaVirtualBoxManager*)malloc(sizeof(SigmaVirtualBoxManager));
    if (!manager) return NULL;
    
    manager->virt_manager = sigma_virtualization_manager_create();
    if (!manager->virt_manager) {
        free(manager);
        return NULL;
    }
    
    getcwd(manager->current_directory, sizeof(manager->current_directory));
    manager->auto_start_enabled = false;
    manager->auto_snapshot_enabled = false;
    manager->snapshot_interval_hours = 24;
    
    return manager;
}

void sigma_virtual_box_manager_destroy(SigmaVirtualBoxManager* manager) {
    if (!manager) return;
    
    if (manager->virt_manager) {
        sigma_virtualization_manager_destroy(manager->virt_manager);
    }
    
    free(manager);
}

// List Available VM Templates
void sigma_virtual_box_manager_list_templates(void) {
    printf("\n=== Available VM Templates ===\n");
    printf("ID\tName\t\t\tType\t\tCPU\tMemory\tDisk\tDescription\n");
    printf("--\t----\t\t\t----\t\t---\t------\t----\t-----------\n");
    
    for (uint32_t i = 0; i < vm_template_count; i++) {
        SigmaVMTemplate* template = &vm_templates[i];
        
        const char* type_str = "Unknown";
        switch (template->vm_type) {
            case SIGMA_VM_WINDOWS: type_str = "Windows"; break;
            case SIGMA_VM_LINUX: type_str = "Linux"; break;
            case SIGMA_VM_MACOS: type_str = "macOS"; break;
            case SIGMA_VM_BSD: type_str = "BSD"; break;
            case SIGMA_VM_SOLARIS: type_str = "Solaris"; break;
            case SIGMA_VM_ANDROID: type_str = "Android"; break;
            case SIGMA_VM_IOS: type_str = "iOS"; break;
            default: type_str = "Custom"; break;
        }
        
        printf("%u\t%s\t\t%s\t\t%u\t%lluMB\t%lluGB\t%s\n",
               i, template->template_name, type_str,
               template->default_cpu_cores, template->default_memory_mb, template->default_disk_gb,
               template->description);
    }
    printf("\n");
}

// Create VM from Template
SigmaVirtualMachine* sigma_virtual_box_manager_create_from_template(SigmaVirtualBoxManager* manager,
                                                               uint32_t template_id,
                                                               const char* vm_name) {
    if (!manager || template_id >= vm_template_count || !vm_name) return NULL;
    
    SigmaVMTemplate* template = &vm_templates[template_id];
    
    printf("[VirtualBox] Creating VM '%s' from template '%s'\n", vm_name, template->template_name);
    
    SigmaVirtualMachine* vm = NULL;
    
    switch (template->vm_type) {
        case SIGMA_VM_WINDOWS:
            vm = sigma_create_windows_vm(manager->virt_manager, vm_name,
                                     template->default_cpu_cores,
                                     template->default_memory_mb,
                                     template->default_disk_gb,
                                     template->default_iso);
            break;
            
        case SIGMA_VM_LINUX:
            vm = sigma_create_linux_vm(manager->virt_manager, vm_name,
                                    template->default_cpu_cores,
                                    template->default_memory_mb,
                                    template->default_disk_gb,
                                    template->default_iso);
            break;
            
        case SIGMA_VM_MACOS:
            vm = sigma_create_macos_vm(manager->virt_manager, vm_name,
                                     template->default_cpu_cores,
                                     template->default_memory_mb,
                                     template->default_disk_gb,
                                     template->default_iso);
            break;
            
        default:
            printf("[VirtualBox] Unsupported VM type: %d\n", template->vm_type);
            return NULL;
    }
    
    if (vm) {
        printf("[VirtualBox] VM '%s' created successfully\n", vm_name);
        printf("[VirtualBox] VM ID: %u\n", vm->vm_id);
        printf("[VirtualBox] Type: %s\n", template->template_name);
        printf("[VirtualBox] CPU: %u cores\n", template->default_cpu_cores);
        printf("[VirtualBox] Memory: %llu MB\n", template->default_memory_mb);
        printf("[VirtualBox] Disk: %llu GB\n", template->default_disk_gb);
    } else {
        printf("[VirtualBox] Failed to create VM '%s'\n", vm_name);
    }
    
    return vm;
}

// Quick VM Creation Functions
SigmaVirtualMachine* sigma_virtual_box_manager_create_windows(SigmaVirtualBoxManager* manager,
                                                          const char* vm_name) {
    return sigma_virtual_box_manager_create_from_template(manager, 0, vm_name); // Windows 11
}

SigmaVirtualMachine* sigma_virtual_box_manager_create_ubuntu(SigmaVirtualBoxManager* manager,
                                                        const char* vm_name) {
    return sigma_virtual_box_manager_create_from_template(manager, 2, vm_name); // Ubuntu 22.04
}

SigmaVirtualMachine* sigma_virtual_box_manager_create_macos(SigmaVirtualBoxManager* manager,
                                                       const char* vm_name) {
    return sigma_virtual_box_manager_create_from_template(manager, 6, vm_name); // macOS Monterey
}

// Start VM with Simple Interface
bool sigma_virtual_box_manager_start_vm(SigmaVirtualBoxManager* manager, const char* vm_name) {
    if (!manager || !vm_name) return false;
    
    // Find VM by name
    for (uint32_t i = 0; i < manager->virt_manager->vm_count; i++) {
        SigmaVirtualMachine* vm = &manager->virt_manager->vms[i];
        if (strcmp(vm->config.vm_name, vm_name) == 0) {
            printf("[VirtualBox] Starting VM: %s\n", vm_name);
            
            bool result = sigma_virtualization_manager_start_vm(manager->virt_manager, vm->vm_id);
            
            if (result) {
                printf("[VirtualBox] VM '%s' started successfully\n", vm_name);
                printf("[VirtualBox] VNC Display: :%u\n", vm->vm_id + 5900);
                printf("[VirtualBox] Connect with: vncviewer localhost:%u\n", vm->vm_id + 5900);
                
                // Auto-open VNC viewer if available
                char vnc_cmd[512];
                snprintf(vnc_cmd, sizeof(vnc_cmd), "vncviewer localhost:%u &", vm->vm_id + 5900);
                system(vnc_cmd);
                
                return true;
            } else {
                printf("[VirtualBox] Failed to start VM '%s'\n", vm_name);
                return false;
            }
        }
    }
    
    printf("[VirtualBox] VM '%s' not found\n", vm_name);
    return false;
}

// Stop VM with Simple Interface
bool sigma_virtual_box_manager_stop_vm(SigmaVirtualBoxManager* manager, const char* vm_name) {
    if (!manager || !vm_name) return false;
    
    // Find VM by name
    for (uint32_t i = 0; i < manager->virt_manager->vm_count; i++) {
        SigmaVirtualMachine* vm = &manager->virt_manager->vms[i];
        if (strcmp(vm->config.vm_name, vm_name) == 0) {
            printf("[VirtualBox] Stopping VM: %s\n", vm_name);
            
            bool result = sigma_virtualization_manager_stop_vm(manager->virt_manager, vm->vm_id);
            
            if (result) {
                printf("[VirtualBox] VM '%s' stopped successfully\n", vm_name);
                return true;
            } else {
                printf("[VirtualBox] Failed to stop VM '%s'\n", vm_name);
                return false;
            }
        }
    }
    
    printf("[VirtualBox] VM '%s' not found\n", vm_name);
    return false;
}

// List All VMs
void sigma_virtual_box_manager_list_vms(SigmaVirtualBoxManager* manager) {
    if (!manager) return;
    
    sigma_virtualization_manager_list_vms(manager->virt_manager);
}

// One-Click VM Creation and Start
bool sigma_virtual_box_manager_create_and_start(SigmaVirtualBoxManager* manager,
                                           uint32_t template_id,
                                           const char* vm_name) {
    if (!manager || !vm_name) return false;
    
    printf("[VirtualBox] One-click create and start: %s\n", vm_name);
    
    // Create VM
    SigmaVirtualMachine* vm = sigma_virtual_box_manager_create_from_template(manager, template_id, vm_name);
    if (!vm) return false;
    
    // Start VM
    return sigma_virtual_box_manager_start_vm(manager, vm_name);
}

// Interactive VM Creation
void sigma_virtual_box_manager_interactive_create(SigmaVirtualBoxManager* manager) {
    printf("\n=== Interactive VM Creation ===\n");
    
    // List templates
    sigma_virtual_box_manager_list_templates();
    
    uint32_t template_id;
    printf("Enter template ID (0-%u): ", vm_template_count - 1);
    scanf("%u", &template_id);
    
    if (template_id >= vm_template_count) {
        printf("[VirtualBox] Invalid template ID\n");
        return;
    }
    
    char vm_name[256];
    printf("Enter VM name: ");
    scanf("%255s", vm_name);
    
    // Create VM
    SigmaVirtualMachine* vm = sigma_virtual_box_manager_create_from_template(manager, template_id, vm_name);
    if (vm) {
        printf("[VirtualBox] VM created successfully!\n");
        
        char start_choice;
        printf("Start VM now? (y/n): ");
        scanf(" %c", &start_choice);
        
        if (start_choice == 'y' || start_choice == 'Y') {
            sigma_virtual_box_manager_start_vm(manager, vm_name);
        }
    }
}

// VM Management Menu
void sigma_virtual_box_manager_show_menu(SigmaVirtualBoxManager* manager) {
    printf("\n=== SigmaOS Virtual Box Manager ===\n");
    printf("1. List VM Templates\n");
    printf("2. Create VM from Template\n");
    printf("3. Quick Create Windows VM\n");
    printf("4. Quick Create Ubuntu VM\n");
    printf("5. Quick Create macOS VM\n");
    printf("6. List VMs\n");
    printf("7. Start VM\n");
    printf("8. Stop VM\n");
    printf("9. Interactive VM Creation\n");
    printf("0. Exit\n");
    printf("Choice: ");
}

// Main Virtual Box Manager Loop
void sigma_virtual_box_manager_run(SigmaVirtualBoxManager* manager) {
    if (!manager) return;
    
    printf("[VirtualBox] SigmaOS Virtual Box Manager\n");
    printf("[VirtualBox] Simple VM management interface\n");
    
    int choice;
    do {
        sigma_virtual_box_manager_show_menu(manager);
        scanf("%d", &choice);
        
        switch (choice) {
            case 1:
                sigma_virtual_box_manager_list_templates();
                break;
                
            case 2: {
                uint32_t template_id;
                char vm_name[256];
                
                printf("Enter template ID: ");
                scanf("%u", &template_id);
                
                printf("Enter VM name: ");
                scanf("%255s", vm_name);
                
                sigma_virtual_box_manager_create_from_template(manager, template_id, vm_name);
                break;
            }
            
            case 3: {
                char vm_name[256];
                printf("Enter VM name: ");
                scanf("%255s", vm_name);
                sigma_virtual_box_manager_create_windows(manager, vm_name);
                break;
            }
            
            case 4: {
                char vm_name[256];
                printf("Enter VM name: ");
                scanf("%255s", vm_name);
                sigma_virtual_box_manager_create_ubuntu(manager, vm_name);
                break;
            }
            
            case 5: {
                char vm_name[256];
                printf("Enter VM name: ");
                scanf("%255s", vm_name);
                sigma_virtual_box_manager_create_macos(manager, vm_name);
                break;
            }
            
            case 6:
                sigma_virtual_box_manager_list_vms(manager);
                break;
                
            case 7: {
                char vm_name[256];
                printf("Enter VM name: ");
                scanf("%255s", vm_name);
                sigma_virtual_box_manager_start_vm(manager, vm_name);
                break;
            }
            
            case 8: {
                char vm_name[256];
                printf("Enter VM name: ");
                scanf("%255s", vm_name);
                sigma_virtual_box_manager_stop_vm(manager, vm_name);
                break;
            }
            
            case 9:
                sigma_virtual_box_manager_interactive_create(manager);
                break;
                
            case 0:
                printf("[VirtualBox] Exiting Virtual Box Manager\n");
                break;
                
            default:
                printf("[VirtualBox] Invalid choice\n");
                break;
        }
        
    } while (choice != 0);
}

// Initialize Virtual Box Manager
void sigma_virtual_box_manager_initialize(void) {
    if (!g_vbox_manager) {
        g_vbox_manager = sigma_virtual_box_manager_create();
        
        if (g_vbox_manager) {
            printf("[VirtualBox] SigmaOS Virtual Box Manager initialized\n");
            printf("[VirtualBox] VM Templates: %u\n", vm_template_count);
            printf("[VirtualBox] Ready for simple VM management\n");
        }
    }
}

// Cleanup Virtual Box Manager
void sigma_virtual_box_manager_cleanup(void) {
    if (g_vbox_manager) {
        sigma_virtual_box_manager_destroy(g_vbox_manager);
        g_vbox_manager = NULL;
    }
}

// Get Global Virtual Box Manager
SigmaVirtualBoxManager* sigma_virtual_box_manager_get(void) {
    return g_vbox_manager;
}

// Quick Start Functions
void sigma_quick_start_windows_vm(const char* vm_name) {
    sigma_virtual_box_manager_initialize();
    SigmaVirtualBoxManager* manager = sigma_virtual_box_manager_get();
    
    if (manager) {
        sigma_virtual_box_manager_create_and_start(manager, 0, vm_name); // Windows 11 template
    }
}

void sigma_quick_start_ubuntu_vm(const char* vm_name) {
    sigma_virtual_box_manager_initialize();
    SigmaVirtualBoxManager* manager = sigma_virtual_box_manager_get();
    
    if (manager) {
        sigma_virtual_box_manager_create_and_start(manager, 2, vm_name); // Ubuntu 22.04 template
    }
}

void sigma_quick_start_macos_vm(const char* vm_name) {
    sigma_virtual_box_manager_initialize();
    SigmaVirtualBoxManager* manager = sigma_virtual_box_manager_get();
    
    if (manager) {
        sigma_virtual_box_manager_create_and_start(manager, 6, vm_name); // macOS Monterey template
    }
}

