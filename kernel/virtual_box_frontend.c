/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS Virtual Box Frontend
 * ==========================
 * Simple web-based virtual machine management
 * One-click VM creation and management
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

// Include virtual box manager
#include "virtual_box_manager.c"

// Web Server for Virtual Box Management
typedef struct {
    int port;
    bool is_running;
    SigmaVirtualBoxManager* vbox_manager;
} SigmaVirtualBoxWebServer;

// HTML Templates
const char* html_header = 
"<!DOCTYPE html>\n"
"<html>\n"
"<head>\n"
"    <title>SigmaOS Virtual Box Manager</title>\n"
"    <meta charset=\"UTF-8\">\n"
"    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n"
"    <style>\n"
"        body { font-family: Arial, sans-serif; margin: 20px; background: #f5f5f5; }\n"
"        .container { max-width: 1200px; margin: 0 auto; background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }\n"
"        h1 { color: #333; text-align: center; margin-bottom: 30px; }\n"
"        h2 { color: #555; border-bottom: 2px solid #007bff; padding-bottom: 10px; }\n"
"        .vm-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; margin: 20px 0; }\n"
"        .vm-card { background: #fff; border: 1px solid #ddd; border-radius: 8px; padding: 15px; box-shadow: 0 2px 5px rgba(0,0,0,0.1); }\n"
"        .vm-card h3 { margin: 0 0 10px 0; color: #333; }\n"
"        .vm-card p { margin: 5px 0; color: #666; }\n"
"        .vm-card .status { font-weight: bold; }\n"
"        .vm-card .running { color: #28a745; }\n"
"        .vm-card .stopped { color: #dc3545; }\n"
"        .btn { background: #007bff; color: white; border: none; padding: 10px 20px; border-radius: 5px; cursor: pointer; margin: 5px; text-decoration: none; display: inline-block; }\n"
"        .btn:hover { background: #0056b3; }\n"
"        .btn-success { background: #28a745; }\n"
"        .btn-success:hover { background: #1e7e34; }\n"
"        .btn-danger { background: #dc3545; }\n"
"        .btn-danger:hover { background: #c82333; }\n"
"        .form-group { margin: 15px 0; }\n"
"        .form-group label { display: block; margin-bottom: 5px; font-weight: bold; }\n"
"        .form-group input, .form-group select { width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; }\n"
"        .template-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 15px; margin: 20px 0; }\n"
"        .template-card { background: #f8f9fa; border: 1px solid #dee2e6; border-radius: 8px; padding: 15px; cursor: pointer; transition: all 0.3s; }\n"
"        .template-card:hover { background: #e9ecef; transform: translateY(-2px); }\n"
"        .template-card h4 { margin: 0 0 10px 0; color: #495057; }\n"
"        .template-card p { margin: 5px 0; color: #6c757d; font-size: 14px; }\n"
"        .nav { background: #343a40; padding: 10px; border-radius: 5px; margin-bottom: 20px; }\n"
"        .nav a { color: white; text-decoration: none; margin: 0 10px; padding: 5px 10px; border-radius: 3px; }\n"
"        .nav a:hover { background: #495057; }\n"
"        .nav a.active { background: #007bff; }\n"
"        .alert { padding: 15px; margin: 20px 0; border-radius: 5px; }\n"
"        .alert-success { background: #d4edda; color: #155724; border: 1px solid #c3e6cb; }\n"
"        .alert-danger { background: #f8d7da; color: #721c24; border: 1px solid #f5c6cb; }\n"
"    </style>\n"
"</head>\n"
"<body>\n"
"    <div class=\"container\">\n"
"        <h1>🖥️ SigmaOS Virtual Box Manager</h1>\n"
"        <div class=\"nav\">\n"
"            <a href=\"/\">Dashboard</a>\n"
"            <a href=\"/templates\">Templates</a>\n"
"            <a href=\"/create\">Create VM</a>\n"
"            <a href=\"/vms\">My VMs</a>\n"
"        </div>\n";

const char* html_footer = 
"    </div>\n"
"</body>\n"
"</html>\n";

// Generate VM List HTML
void generate_vm_list_html(SigmaVirtualBoxManager* manager, char* output, size_t output_size) {
    snprintf(output, output_size, "%s", html_header);
    
    strcat(output, "<h2>🖥️ My Virtual Machines</h2>\n");
    strcat(output, "<div class=\"vm-grid\">\n");
    
    for (uint32_t i = 0; i < manager->virt_manager->vm_count; i++) {
        SigmaVirtualMachine* vm = &manager->virt_manager->vms[i];
        
        char vm_html[1024];
        const char* status_class = (vm->state == SIGMA_VM_RUNNING) ? "running" : "stopped";
        const char* status_text = (vm->state == SIGMA_VM_RUNNING) ? "Running" : "Stopped";
        
        snprintf(vm_html, sizeof(vm_html),
            "        <div class=\"vm-card\">\n"
            "            <h3>%s</h3>\n"
            "            <p><strong>Type:</strong> %s</p>\n"
            "            <p><strong>CPU:</strong> %u cores</p>\n"
            "            <p><strong>Memory:</strong> %llu MB</p>\n"
            "            <p><strong>Disk:</strong> %llu GB</p>\n"
            "            <p><strong>Status:</strong> <span class=\"status %s\">%s</span></p>\n"
            "            <div style=\"margin-top: 15px;\">\n",
            vm->config.vm_name,
            (vm->config.vm_type == SIGMA_VM_WINDOWS) ? "Windows" :
            (vm->config.vm_type == SIGMA_VM_LINUX) ? "Linux" :
            (vm->config.vm_type == SIGMA_VM_MACOS) ? "macOS" : "Other",
            vm->config.cpu_cores,
            vm->config.memory_mb,
            vm->config.disk_gb,
            status_class, status_text);
        
        strcat(output, vm_html);
        
        if (vm->state == SIGMA_VM_RUNNING) {
            strcat(output, 
                "                <a href=\"/stop/");
            strcat(output, vm->config.vm_name);
            strcat(output, "\" class=\"btn btn-danger\">Stop</a>\n");
            strcat(output, 
                "                <a href=\"/vnc/");
            strcat(output, vm->config.vm_name);
            strcat(output, "\" class=\"btn btn-success\">VNC</a>\n");
        } else {
            strcat(output, 
                "                <a href=\"/start/");
            strcat(output, vm->config.vm_name);
            strcat(output, "\" class=\"btn btn-success\">Start</a>\n");
        }
        
        strcat(output, 
            "            </div>\n"
            "        </div>\n");
    }
    
    strcat(output, "</div>\n");
    strcat(output, html_footer);
}

// Generate Templates HTML
void generate_templates_html(char* output, size_t output_size) {
    snprintf(output, output_size, "%s", html_header);
    
    strcat(output, "<h2>📋 VM Templates</h2>\n");
    strcat(output, "<div class=\"template-grid\">\n");
    
    for (uint32_t i = 0; i < vm_template_count; i++) {
        SigmaVMTemplate* template = &vm_templates[i];
        
        char template_html[1024];
        snprintf(template_html, sizeof(template_html),
            "        <div class=\"template-card\" onclick=\"selectTemplate(%u)\">\n"
            "            <h4>%s</h4>\n"
            "            <p><strong>Type:</strong> %s</p>\n"
            "            <p><strong>CPU:</strong> %u cores</p>\n"
            "            <p><strong>Memory:</strong> %llu MB</p>\n"
            "            <p><strong>Disk:</strong> %llu GB</p>\n"
            "            <p><strong>Description:</strong> %s</p>\n"
            "        </div>\n",
            i, template->template_name,
            (template->vm_type == SIGMA_VM_WINDOWS) ? "Windows" :
            (template->vm_type == SIGMA_VM_LINUX) ? "Linux" :
            (template->vm_type == SIGMA_VM_MACOS) ? "macOS" : "Other",
            template->default_cpu_cores,
            template->default_memory_mb,
            template->default_disk_gb,
            template->description);
        
        strcat(output, template_html);
    }
    
    strcat(output, "</div>\n");
    
    strcat(output, 
        "<script>\n"
        "function selectTemplate(templateId) {\n"
        "    const templates = [\n");
    
    for (uint32_t i = 0; i < vm_template_count; i++) {
        SigmaVMTemplate* template = &vm_templates[i];
        char template_js[512];
        snprintf(template_js, sizeof(template_js,
            "        { id: %u, name: '%s', type: '%s', cpu: %u, memory: %llu, disk: %llu }%s\n",
            i, template->template_name,
            (template->vm_type == SIGMA_VM_WINDOWS) ? "Windows" :
            (template->vm_type == SIGMA_VM_LINUX) ? "Linux" :
            (template->vm_type == SIGMA_VM_MACOS) ? "macOS" : "Other",
            template->default_cpu_cores,
            template->default_memory_mb,
            template->default_disk_gb,
            (i < vm_template_count - 1) ? "," : "");
        strcat(output, template_js);
    }
    
    strcat(output, 
        "    ];\n"
        "    const template = templates[templateId];\n"
        "    document.getElementById('vmName').value = template.name + '-VM';\n"
        "    document.getElementById('templateId').value = templateId;\n"
        "}\n"
        "</script>\n");
    
    strcat(output, html_footer);
}

// Generate Create VM HTML
void generate_create_vm_html(char* output, size_t output_size) {
    snprintf(output, output_size, "%s", html_header);
    
    strcat(output, 
        "<h2>🚀 Create Virtual Machine</h2>\n"
        "<form method=\"post\" action=\"/create\">\n"
        "    <div class=\"form-group\">\n"
        "        <label for=\"templateId\">Template:</label>\n"
        "        <select id=\"templateId\" name=\"templateId\" required>\n"
        "            <option value=\"\">Select a template...</option>\n");
    
    for (uint32_t i = 0; i < vm_template_count; i++) {
        SigmaVMTemplate* template = &vm_templates[i];
        char option_html[256];
        snprintf(option_html, sizeof(option_html),
            "            <option value=\"%u\">%s</option>\n",
            i, template->template_name);
        strcat(output, option_html);
    }
    
    strcat(output, 
        "        </select>\n"
        "    </div>\n"
        "    <div class=\"form-group\">\n"
        "        <label for=\"vmName\">VM Name:</label>\n"
        "        <input type=\"text\" id=\"vmName\" name=\"vmName\" required placeholder=\"Enter VM name\">\n"
        "    </div>\n"
        "    <div class=\"form-group\">\n"
        "        <label for=\"cpuCores\">CPU Cores:</label>\n"
        "        <input type=\"number\" id=\"cpuCores\" name=\"cpuCores\" min=\"1\" max=\"16\" value=\"2\" required>\n"
        "    </div>\n"
        "    <div class=\"form-group\">\n"
        "        <label for=\"memoryMB\">Memory (MB):</label>\n"
        "        <input type=\"number\" id=\"memoryMB\" name=\"memoryMB\" min=\"512\" max=\"32768\" value=\"2048\" required>\n"
        "    </div>\n"
        "    <div class=\"form-group\">\n"
        "        <label for=\"diskGB\">Disk Size (GB):</label>\n"
        "        <input type=\"number\" id=\"diskGB\" name=\"diskGB\" min=\"10\" max=\"1000\" value=\"20\" required>\n"
        "    </div>\n"
        "    <button type=\"submit\" class=\"btn btn-success\">Create VM</button>\n"
        "    <a href=\"/templates\" class=\"btn\">View Templates</a>\n"
        "</form>\n");
    
    strcat(output, html_footer);
}

// Simple HTTP Server
void start_virtual_box_web_server(int port) {
    SigmaVirtualBoxWebServer server;
    server.port = port;
    server.is_running = true;
    server.vbox_manager = sigma_virtual_box_manager_create();
    
    if (!server.vbox_manager) {
        printf("[VirtualBox] Failed to initialize virtual box manager\n");
        return;
    }
    
    printf("[VirtualBox] Starting web server on port %d\n", port);
    printf("[VirtualBox] Open http://localhost:%d in your browser\n", port);
    
    // Simple HTTP server implementation
    char command[512];
    snprintf(command, sizeof(command),
             "python3 -m http.server %d --directory /tmp/sigmaos_vbox &",
             port);
    
    // Create temporary directory for web files
    system("mkdir -p /tmp/sigmaos_vbox");
    
    // Generate index.html
    char index_html[10000];
    generate_vm_list_html(server.vbox_manager, index_html, sizeof(index_html));
    
    FILE* index_file = fopen("/tmp/sigmaos_vbox/index.html", "w");
    if (index_file) {
        fputs(index_html, index_file);
        fclose(index_file);
    }
    
    // Start web server
    system(command);
    
    printf("[VirtualBox] Web server started\n");
    printf("[VirtualBox] Press Ctrl+C to stop\n");
    
    // Keep server running
    while (server.is_running) {
        sleep(1);
    }
}

// Command Line Interface
void show_virtual_box_help(void) {
    printf("SigmaOS Virtual Box Manager\n");
    printf("==========================\n\n");
    printf("Usage:\n");
    printf("  sigma-vbox                    - Start web interface\n");
    printf("  sigma-vbox web <port>         - Start web server on port\n");
    printf("  sigma-vbox create <name>       - Quick create Ubuntu VM\n");
    printf("  sigma-vbox windows <name>      - Quick create Windows VM\n");
    printf("  sigma-vbox macos <name>       - Quick create macOS VM\n");
    printf("  sigma-vbox list               - List all VMs\n");
    printf("  sigma-vbox start <name>       - Start VM\n");
    printf("  sigma-vbox stop <name>        - Stop VM\n");
    printf("  sigma-vbox help               - Show this help\n\n");
    printf("Examples:\n");
    printf("  sigma-vbox web 8080          - Start web interface on port 8080\n");
    printf("  sigma-vbox create my-ubuntu    - Create Ubuntu VM named 'my-ubuntu'\n");
    printf("  sigma-vbox windows my-win      - Create Windows VM named 'my-win'\n");
    printf("  sigma-vbox start my-ubuntu    - Start VM named 'my-ubuntu'\n");
    printf("  sigma-vbox stop my-ubuntu     - Stop VM named 'my-ubuntu'\n");
}

// Main Function
int main(int argc, char* argv[]) {
    if (argc < 2) {
        show_virtual_box_help();
        return 1;
    }
    
    const char* command = argv[1];
    
    if (strcmp(command, "help") == 0) {
        show_virtual_box_help();
        return 0;
    }
    
    if (strcmp(command, "web") == 0) {
        int port = 8080;
        if (argc > 2) {
            port = atoi(argv[2]);
        }
        start_virtual_box_web_server(port);
        return 0;
    }
    
    if (strcmp(command, "create") == 0) {
        if (argc < 3) {
            printf("Error: VM name required\n");
            return 1;
        }
        sigma_quick_start_ubuntu_vm(argv[2]);
        return 0;
    }
    
    if (strcmp(command, "windows") == 0) {
        if (argc < 3) {
            printf("Error: VM name required\n");
            return 1;
        }
        sigma_quick_start_windows_vm(argv[2]);
        return 0;
    }
    
    if (strcmp(command, "macos") == 0) {
        if (argc < 3) {
            printf("Error: VM name required\n");
            return 1;
        }
        sigma_quick_start_macos_vm(argv[2]);
        return 0;
    }
    
    if (strcmp(command, "list") == 0) {
        sigma_virtual_box_manager_initialize();
        SigmaVirtualBoxManager* manager = sigma_virtual_box_manager_get();
        if (manager) {
            sigma_virtual_box_manager_list_vms(manager);
        }
        sigma_virtual_box_manager_cleanup();
        return 0;
    }
    
    if (strcmp(command, "start") == 0) {
        if (argc < 3) {
            printf("Error: VM name required\n");
            return 1;
        }
        sigma_virtual_box_manager_initialize();
        SigmaVirtualBoxManager* manager = sigma_virtual_box_manager_get();
        if (manager) {
            sigma_virtual_box_manager_start_vm(manager, argv[2]);
        }
        sigma_virtual_box_manager_cleanup();
        return 0;
    }
    
    if (strcmp(command, "stop") == 0) {
        if (argc < 3) {
            printf("Error: VM name required\n");
            return 1;
        }
        sigma_virtual_box_manager_initialize();
        SigmaVirtualBoxManager* manager = sigma_virtual_box_manager_get();
        if (manager) {
            sigma_virtual_box_manager_stop_vm(manager, argv[2]);
        }
        sigma_virtual_box_manager_cleanup();
        return 0;
    }
    
    printf("Unknown command: %s\n", command);
    show_virtual_box_help();
    return 1;
}

