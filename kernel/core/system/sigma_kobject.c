/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: KOBJECT FRAMEWORK
 * =============================================================================
 * Inspired by: Linux kernel lib/kobject.c
 *              Linux Sysfs (fs/sysfs/)
 * =============================================================================
 * Provides reference counting and hierarchical organization for kernel objects,
 * acting as the foundation for the device driver model (sysfs).
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define MAX_KOBJECTS 256
#define KOBJ_NAME_LEN 32

struct sigma_kobject;

typedef void (*kobj_release_fn)(struct sigma_kobject* kobj);

typedef struct sigma_kobject {
    char                  name[KOBJ_NAME_LEN];
    struct sigma_kobject* parent;
    sigma_u32             refcount;
    sigma_bool            active;
    kobj_release_fn       release;
    
    /* Simulate sysfs attributes linked list head */
    void*                 attributes; 
} sigma_kobject_t;

static sigma_kobject_t kobj_pool[MAX_KOBJECTS];
static sigma_kobject_t* kobj_root = SIGMA_NULL;

void kobject_subsystem_init(void) {
    sigma_memset(kobj_pool, 0, sizeof(kobj_pool));
    sigma_printf("[kobject] Subsystem initialized\n");
    
    /* Create root 'sys' object */
    kobj_root = &kobj_pool[0];
    sigma_strcpy(kobj_root->name, "sys", KOBJ_NAME_LEN);
    kobj_root->parent   = SIGMA_NULL;
    kobj_root->refcount = 1;
    kobj_root->active   = SIGMA_TRUE;
}

sigma_kobject_t* kobject_create(const char* name, sigma_kobject_t* parent) {
    if (!name) return SIGMA_NULL;
    
    for (sigma_u32 i = 0; i < MAX_KOBJECTS; i++) {
        if (!kobj_pool[i].active) {
            sigma_strcpy(kobj_pool[i].name, name, KOBJ_NAME_LEN);
            kobj_pool[i].parent   = parent ? parent : kobj_root;
            kobj_pool[i].refcount = 1;
            kobj_pool[i].active   = SIGMA_TRUE;
            kobj_pool[i].release  = SIGMA_NULL;
            
            if (kobj_pool[i].parent) {
                kobj_pool[i].parent->refcount++;
            }
            
            sigma_printf("[kobject] Created '%s' (parent: '%s')\n", 
                         name, kobj_pool[i].parent ? kobj_pool[i].parent->name : "none");
            return &kobj_pool[i];
        }
    }
    sigma_printf("[kobject] ERR: Pool exhausted\n");
    return SIGMA_NULL;
}

sigma_kobject_t* kobject_get(sigma_kobject_t* kobj) {
    if (kobj && kobj->active) {
        kobj->refcount++;
        return kobj;
    }
    return SIGMA_NULL;
}

void kobject_put(sigma_kobject_t* kobj) {
    if (!kobj || !kobj->active) return;
    
    if (kobj->refcount > 0) {
        kobj->refcount--;
    }
    
    if (kobj->refcount == 0) {
        sigma_printf("[kobject] Released '%s'\n", kobj->name);
        
        if (kobj->release) {
            kobj->release(kobj);
        }
        
        sigma_kobject_t* parent = kobj->parent;
        kobj->active = SIGMA_FALSE;
        
        /* Cascade release to parent */
        if (parent) {
            kobject_put(parent);
        }
    }
}

void kobject_dump_tree(void) {
    sigma_printf("\n--- Σ KOBJECT TREE ---\n");
    for (sigma_u32 i = 0; i < MAX_KOBJECTS; i++) {
        if (kobj_pool[i].active) {
            sigma_printf("| [%d] %-12s (ref: %u) -> Parent: %s\n",
                         i, kobj_pool[i].name, kobj_pool[i].refcount,
                         kobj_pool[i].parent ? kobj_pool[i].parent->name : "NULL");
        }
    }
    sigma_printf("----------------------\n");
}
