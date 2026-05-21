#include "sigma_kernel_types.h"
#include "sigma_slab.h"

// Initramfs Loader (CPIO format)

typedef struct {
    char magic[6];
    char ino[8];
    char mode[8];
    char uid[8];
    char gid[8];
    char nlink[8];
    char mtime[8];
    char filesize[8];
    char devmajor[8];
    char devminor[8];
    char rdevmajor[8];
    char rdevminor[8];
    char namesize[8];
    char check[8];
} cpio_header_t;

// Simulated external VFS API
extern int vfs_create_file(const char* path, void* data, size_t size);

static int parse_hex_8(const char* s) {
    int val = 0;
    for (int i=0; i<8; i++) {
        val <<= 4;
        if (s[i] >= '0' && s[i] <= '9') val |= (s[i] - '0');
        else if (s[i] >= 'a' && s[i] <= 'f') val |= (s[i] - 'a' + 10);
        else if (s[i] >= 'A' && s[i] <= 'F') val |= (s[i] - 'A' + 10);
    }
    return val;
}

void sigma_initramfs_load(void* initrd_addr, size_t size) {
    if (!initrd_addr || size == 0) return;
    
    uint8_t* ptr = (uint8_t*)initrd_addr;
    uint8_t* end = ptr + size;
    
    while (ptr < end) {
        cpio_header_t* header = (cpio_header_t*)ptr;
        
        // Check "070701" new ASCII format magic
        if (header->magic[0] != '0' || header->magic[1] != '7' ||
            header->magic[2] != '0' || header->magic[3] != '7' ||
            header->magic[4] != '0' || header->magic[5] != '1') {
            break; 
        }
        
        int namesize = parse_hex_8(header->namesize);
        int filesize = parse_hex_8(header->filesize);
        
        char* name = (char*)(ptr + sizeof(cpio_header_t));
        
        // CPIO padding rules
        int name_padding = (4 - ((sizeof(cpio_header_t) + namesize) % 4)) % 4;
        uint8_t* data = (uint8_t*)(name + namesize + name_padding);
        
        int data_padding = (4 - (filesize % 4)) % 4;
        
        // If "TRAILER!!!" we are done
        int is_trailer = 1;
        const char* tr = "TRAILER!!!";
        for (int i=0; i<10; i++) {
            if (name[i] != tr[i]) {
                is_trailer = 0;
                break;
            }
        }
        if (is_trailer) break;
        
        // VFS Create (stubbed)
        // vfs_create_file(name, data, filesize);
        
        ptr = data + filesize + data_padding;
    }
}
