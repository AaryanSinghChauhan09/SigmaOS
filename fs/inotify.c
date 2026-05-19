/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: FILESYSTEM EVENT NOTIFICATION (INOTIFY)
 * =============================================================================
 * Inspired by: Linux kernel fs/notify/inotify/inotify_user.c
 *              FreeBSD sys/kern/vfs_kqueue.c
 * =============================================================================
 * Monitors inodes for access, modification, or deletion and queues events.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define IN_ACCESS        0x00000001
#define IN_MODIFY        0x00000002
#define IN_ATTRIB        0x00000004
#define IN_CLOSE_WRITE   0x00000008
#define IN_CLOSE_NOWRITE 0x00000010
#define IN_OPEN          0x00000020
#define IN_MOVED_FROM    0x00000040
#define IN_MOVED_TO      0x00000080
#define IN_CREATE        0x00000100
#define IN_DELETE        0x00000200
#define IN_DELETE_SELF   0x00000400
#define IN_MOVE_SELF     0x00000800

#define MAX_INOTIFY_WATCHES 64
#define MAX_INOTIFY_EVENTS  128

typedef struct {
    sigma_u32 wd; /* Watch Descriptor */
    sigma_u32 mask;
    sigma_u32 cookie;
    sigma_u32 len;
    char      name[32];
} inotify_event_t;

typedef struct {
    sigma_u32  wd;
    sigma_u32  inode_num;
    sigma_u32  mask;
    sigma_bool active;
} inotify_watch_t;

static inotify_watch_t watch_table[MAX_INOTIFY_WATCHES];
static inotify_event_t event_queue[MAX_INOTIFY_EVENTS];
static sigma_u32 evt_head = 0;
static sigma_u32 evt_tail = 0;
static sigma_u32 next_wd = 1;

void inotify_init(void) {
    sigma_memset(watch_table, 0, sizeof(watch_table));
    sigma_memset(event_queue, 0, sizeof(event_queue));
    sigma_printf("[inotify] Filesystem event notification initialized\n");
}

int inotify_add_watch(sigma_u32 inode, sigma_u32 mask) {
    for (sigma_u32 i = 0; i < MAX_INOTIFY_WATCHES; i++) {
        if (!watch_table[i].active) {
            watch_table[i].wd = next_wd++;
            watch_table[i].inode_num = inode;
            watch_table[i].mask = mask;
            watch_table[i].active = SIGMA_TRUE;
            sigma_printf("[inotify] Added watch (WD %u) for inode %u, mask 0x%08X\n", 
                         watch_table[i].wd, inode, mask);
            return (int)watch_table[i].wd;
        }
    }
    return -1;
}

void inotify_rm_watch(sigma_u32 wd) {
    for (sigma_u32 i = 0; i < MAX_INOTIFY_WATCHES; i++) {
        if (watch_table[i].active && watch_table[i].wd == wd) {
            watch_table[i].active = SIGMA_FALSE;
            sigma_printf("[inotify] Removed watch (WD %u)\n", wd);
            return;
        }
    }
}

void inotify_fsnotify(sigma_u32 inode, sigma_u32 action, const char* name) {
    for (sigma_u32 i = 0; i < MAX_INOTIFY_WATCHES; i++) {
        if (watch_table[i].active && watch_table[i].inode_num == inode) {
            if (watch_table[i].mask & action) {
                sigma_u32 next_head = (evt_head + 1) % MAX_INOTIFY_EVENTS;
                if (next_head != evt_tail) { /* Queue not full */
                    event_queue[evt_head].wd = watch_table[i].wd;
                    event_queue[evt_head].mask = action;
                    event_queue[evt_head].cookie = 0;
                    
                    if (name) {
                        sigma_u32 len = 0;
                        while (len < 31 && name[len]) {
                            event_queue[evt_head].name[len] = name[len];
                            len++;
                        }
                        event_queue[evt_head].name[len] = '\0';
                        event_queue[evt_head].len = len + 1;
                    } else {
                        event_queue[evt_head].name[0] = '\0';
                        event_queue[evt_head].len = 0;
                    }
                    
                    evt_head = next_head;
                    sigma_printf("[inotify] Queued event for WD %u (mask 0x%X, file: %s)\n", 
                                 watch_table[i].wd, action, name ? name : "<self>");
                }
            }
        }
    }
}
