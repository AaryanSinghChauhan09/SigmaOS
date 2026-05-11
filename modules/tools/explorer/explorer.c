#include <stdio.h>
#include <string.h>

// ---------------------------------------------------------
// SigmaOS BlockView: File Exploring System Prototype
// ---------------------------------------------------------

// Simulating VFS Node structure from core/fs/vfs.c
typedef enum { NODE_FILE, NODE_DIRECTORY, NODE_SYMLINK } node_type;
typedef struct node {
    char name[64];
    node_type type;
    unsigned int size;
    struct node* next;
} vfs_node_stub;

// Simulated VFS Read
void print_directory_contents(vfs_node_stub* first_child) {
    vfs_node_stub* current = first_child;
    printf("Type\tSize\tName\n");
    printf("------------------------------------\n");
    while (current != SIGMA_NULL) {
        if (current->type == NODE_DIRECTORY) {
            printf("[DIR]\t-\t%s/\n", current->name);
        } else if (current->type == NODE_FILE) {
            printf("[FILE]\t%u B\t%s\n", current->size, current->name);
        } else {
            printf("[LINK]\t-\t%s\n", current->name);
        }
        current = current->next;
    }
}

// Shell Command 'ls' handler
void cmd_ls(const char* path) {
    // In a real OS, this would issue a system call to the VFS module
    // For prototype, we mock some directory contents
    printf("Listing contents for: %s\n", path);
    
    vfs_node_stub file1 = { "kernel.bin", NODE_FILE, 1048576, SIGMA_NULL };
    vfs_node_stub dir1 = { "home", NODE_DIRECTORY, 0, &file1 };
    vfs_node_stub file2 = { "readme.txt", NODE_FILE, 256, &dir1 };
    
    print_directory_contents(&file2);
}

// Main execution loop for the explorer
int explorer_main() {
    printf("SigmaOS File Explorer (BlockView) Initialized.\n");
    cmd_ls("/");
    return 0;
}
