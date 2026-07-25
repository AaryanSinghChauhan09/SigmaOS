//! SigmaOS Useradd Compatibility
//! User account management (useradd command)
//! Zero external dependencies

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaU64 = u64;

/// User account
#[repr(C)]
pub struct UserAccount {
    pub username: [u8; 32],
    pub uid: SigmaU32,
    pub gid: SigmaU32,
    pub home_dir: [u8; 256],
    pub shell: [u8; 64],
    pub full_name: [u8; 128],
    pub created: SigmaU64,
}

/// User state
const MAX_USERS: usize = 1000;

static mut USERS: [UserAccount; MAX_USERS] = [UserAccount {
    username: [0; 32],
    uid: 0,
    gid: 0,
    home_dir: [0; 256],
    shell: [0; 64],
    full_name: [0; 128],
    created: 0,
}; MAX_USERS];

static mut USER_COUNT: SigmaU32 = 0;
static mut NEXT_UID: SigmaU32 = 1000;
static mut USERADD_INITIALIZED: SigmaBool = false;

/// Initialize useradd
#[no_mangle]
pub unsafe extern "C" fn useradd_init() -> SigmaI32 {
    USERADD_INITIALIZED = true;
    USER_COUNT = 0;
    NEXT_UID = 1000;
    
    // Add root user
    let mut root = UserAccount {
        username: [0; 32],
        uid: 0,
        gid: 0,
        home_dir: [0; 256],
        shell: [0; 64],
        full_name: [0; 128],
        created: 0,
    };
    
    for i in 0..31 {
        root.username[i] = b"root"[i.min(4)];
    }
    
    for i in 0..255 {
        root.home_dir[i] = b"/root"[i.min(5)];
    }
    
    for i in 0..63 {
        root.shell[i] = b"/bin/sh"[i.min(7)];
    }
    
    USERS[0] = root;
    USER_COUNT = 1;
    
    0 // Success
}

/// Add user
#[no_mangle]
pub unsafe extern "C" fn useradd(
    username: *const u8,
    uid: SigmaU32,
    gid: SigmaU32,
    home_dir: *const u8,
    shell: *const u8,
) -> SigmaI32 {
    if !USERADD_INITIALIZED || username.isnull() || USER_COUNT >= MAX_USERS as SigmaU32 {
        return -1;
    }
    
    let mut user = UserAccount {
        username: [0; 32],
        uid: if uid == 0 { NEXT_UID } else { uid },
        gid,
        home_dir: [0; 256],
        shell: [0; 64],
        full_name: [0; 128],
        created: 0,
    };
    
    for i in 0..31 {
        let byte = *username.add(i);
        if byte == 0 { break; }
        user.username[i] = byte;
    }
    
    if !home_dir.isnull() {
        for i in 0..255 {
            let byte = *home_dir.add(i);
            if byte == 0 { break; }
            user.home_dir[i] = byte;
        }
    } else {
        // Default home directory
        for i in 0..255 {
            if i < 5 {
                user.home_dir[i] = b"/home/"[i];
            } else if i < 5 + 32 {
                user.home_dir[i] = *username.add(i - 5);
                if user.home_dir[i] == 0 { break; }
            } else {
                break;
            }
        }
    }
    
    if !shell.isnull() {
        for i in 0..63 {
            let byte = *shell.add(i);
            if byte == 0 { break; }
            user.shell[i] = byte;
        }
    } else {
        // Default shell
        for i in 0..63 {
            user.shell[i] = b"/bin/sh"[i.min(7)];
        }
    }
    
    USERS[USER_COUNT as usize] = user;
    USER_COUNT += 1;
    
    if uid == 0 {
        NEXT_UID += 1;
    }
    
    0 // Success
}

/// Delete user
#[no_mangle]
pub unsafe extern "C" fn userdel(username: *const u8) -> SigmaI32 {
    if !USERADD_INITIALIZED || username.isnull() {
        return -1;
    }
    
    for i in 0..USER_COUNT as usize {
        let user = &USERS[i];
        
        let mut matches = true;
        for j in 0..32 {
            if user.username[j] != *username.add(j) {
                if user.username[j] == 0 && *username.add(j) == 0 {
                    break;
                }
                matches = false;
                break;
            }
            if user.username[j] == 0 {
                break;
            }
        }
        
        if matches {
            // Shift remaining users
            for k in i..USER_COUNT as usize - 1 {
                USERS[k] = USERS[k + 1];
            }
            USER_COUNT -= 1;
            return 0;
        }
    }
    
    -2 // User not found
}

/// Get user by name
#[no_mangle]
pub unsafe extern "C" fn useradd_get_by_name(username: *const u8, user: *mut UserAccount) -> SigmaI32 {
    if !USERADD_INITIALIZED || username.isnull() || user.isnull() {
        return -1;
    }
    
    for i in 0..USER_COUNT as usize {
        let u = &USERS[i];
        
        let mut matches = true;
        for j in 0..32 {
            if u.username[j] != *username.add(j) {
                if u.username[j] == 0 && *username.add(j) == 0 {
                    break;
                }
                matches = false;
                break;
            }
            if u.username[j] == 0 {
                break;
            }
        }
        
        if matches {
            *user = *u;
            return 0;
        }
    }
    
    -2 // User not found
}

/// Get user by UID
#[no_mangle]
pub unsafe extern "C" fn useradd_get_by_uid(uid: SigmaU32, user: *mut UserAccount) -> SigmaI32 {
    if !USERADD_INITIALIZED || user.isnull() {
        return -1;
    }
    
    for i in 0..USER_COUNT as usize {
        if USERS[i].uid == uid {
            *user = USERS[i];
            return 0;
        }
    }
    
    -2 // User not found
}

/// List users
#[no_mangle]
pub unsafe extern "C" fn useradd_list(users: *mut UserAccount, max_count: SigmaU32) -> SigmaU32 {
    if !USERADD_INITIALIZED || users.isnull() {
        return 0;
    }
    
    let mut count = 0;
    for i in 0..USER_COUNT as usize {
        if count >= max_count as usize {
            break;
        }
        *users.add(count) = USERS[i];
        count += 1;
    }
    
    count
}

/// Get user count
#[no_mangle]
pub unsafe extern "C" fn useradd_get_count() -> SigmaU32 {
    USER_COUNT
}
