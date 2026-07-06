//! SigmaOS Groupadd Compatibility
//! Group management (groupadd command)
//! Zero external dependencies

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaU64 = u64;

/// Group
#[repr(C)]
pub struct Group {
    pub groupname: [u8; 32],
    pub gid: SigmaU32,
    pub members: [[u8; 32]; 32],
    pub member_count: SigmaU32,
}

/// Group state
const MAX_GROUPS: usize = 1000;

static mut GROUPS: [Group; MAX_GROUPS] = [Group {
    groupname: [0; 32],
    gid: 0,
    members: [[0; 32]; 32],
    member_count: 0,
}; MAX_GROUPS];

static mut GROUP_COUNT: SigmaU32 = 0;
static mut NEXT_GID: SigmaU32 = 1000;
static mut GROUPADD_INITIALIZED: SigmaBool = false;

/// Initialize groupadd
#[no_mangle]
pub unsafe extern "C" fn groupadd_init() -> SigmaI32 {
    GROUPADD_INITIALIZED = true;
    GROUP_COUNT = 0;
    NEXT_GID = 1000;
    
    // Add root group
    let mut root_group = Group {
        groupname: [0; 32],
        gid: 0,
        members: [[0; 32]; 32],
        member_count: 0,
    };
    
    for i in 0..31 {
        root_group.groupname[i] = b"root"[i.min(4)];
    }
    
    GROUPS[0] = root_group;
    GROUP_COUNT = 1;
    
    0 // Success
}

/// Add group
#[no_mangle]
pub unsafe extern "C" fn groupadd(groupname: *const u8, gid: SigmaU32) -> SigmaI32 {
    if !GROUPADD_INITIALIZED || groupname.isnull() || GROUP_COUNT >= MAX_GROUPS as SigmaU32 {
        return -1;
    }
    
    let mut group = Group {
        groupname: [0; 32],
        gid: if gid == 0 { NEXT_GID } else { gid },
        members: [[0; 32]; 32],
        member_count: 0,
    };
    
    for i in 0..31 {
        let byte = *groupname.add(i);
        if byte == 0 { break; }
        group.groupname[i] = byte;
    }
    
    GROUPS[GROUP_COUNT as usize] = group;
    GROUP_COUNT += 1;
    
    if gid == 0 {
        NEXT_GID += 1;
    }
    
    0 // Success
}

/// Delete group
#[no_mangle]
pub unsafe extern "C" fn groupdel(groupname: *const u8) -> SigmaI32 {
    if !GROUPADD_INITIALIZED || groupname.isnull() {
        return -1;
    }
    
    for i in 0..GROUP_COUNT as usize {
        let group = &GROUPS[i];
        
        let mut matches = true;
        for j in 0..32 {
            if group.groupname[j] != *groupname.add(j) {
                if group.groupname[j] == 0 && *groupname.add(j) == 0 {
                    break;
                }
                matches = false;
                break;
            }
            if group.groupname[j] == 0 {
                break;
            }
        }
        
        if matches {
            // Shift remaining groups
            for k in i..GROUP_COUNT as usize - 1 {
                GROUPS[k] = GROUPS[k + 1];
            }
            GROUP_COUNT -= 1;
            return 0;
        }
    }
    
    -2 // Group not found
}

/// Get group by name
#[no_mangle]
pub unsafe extern "C" fn groupadd_get_by_name(groupname: *const u8, group: *mut Group) -> SigmaI32 {
    if !GROUPADD_INITIALIZED || groupname.isnull() || group.isnull() {
        return -1;
    }
    
    for i in 0..GROUP_COUNT as usize {
        let g = &GROUPS[i];
        
        let mut matches = true;
        for j in 0..32 {
            if g.groupname[j] != *groupname.add(j) {
                if g.groupname[j] == 0 && *groupname.add(j) == 0 {
                    break;
                }
                matches = false;
                break;
            }
            if g.groupname[j] == 0 {
                break;
            }
        }
        
        if matches {
            *group = *g;
            return 0;
        }
    }
    
    -2 // Group not found
}

/// Get group by GID
#[no_mangle]
pub unsafe extern "C" fn groupadd_get_by_gid(gid: SigmaU32, group: *mut Group) -> SigmaI32 {
    if !GROUPADD_INITIALIZED || group.isnull() {
        return -1;
    }
    
    for i in 0..GROUP_COUNT as usize {
        if GROUPS[i].gid == gid {
            *group = GROUPS[i];
            return 0;
        }
    }
    
    -2 // Group not found
}

/// Add member to group
#[no_mangle]
pub unsafe extern "C" fn groupadd_add_member(groupname: *const u8, username: *const u8) -> SigmaI32 {
    if !GROUPADD_INITIALIZED || groupname.isnull() || username.isnull() {
        return -1;
    }
    
    for i in 0..GROUP_COUNT as usize {
        let group = &mut GROUPS[i];
        
        let mut matches = true;
        for j in 0..32 {
            if group.groupname[j] != *groupname.add(j) {
                if group.groupname[j] == 0 && *groupname.add(j) == 0 {
                    break;
                }
                matches = false;
                break;
            }
            if group.groupname[j] == 0 {
                break;
            }
        }
        
        if matches {
            if group.member_count >= 32 {
                return -3; // Group full
            }
            
            let member_idx = group.member_count as usize;
            for j in 0..31 {
                let byte = *username.add(j);
                if byte == 0 { break; }
                group.members[member_idx][j] = byte;
            }
            
            group.member_count += 1;
            return 0;
        }
    }
    
    -2 // Group not found
}

/// List groups
#[no_mangle]
pub unsafe extern "C" fn groupadd_list(groups: *mut Group, max_count: SigmaU32) -> SigmaU32 {
    if !GROUPADD_INITIALIZED || groups.isnull() {
        return 0;
    }
    
    let mut count = 0;
    for i in 0..GROUP_COUNT as usize {
        if count >= max_count as usize {
            break;
        }
        *groups.add(count) = GROUPS[i];
        count += 1;
    }
    
    count
}

/// Get group count
#[no_mangle]
pub unsafe extern "C" fn groupadd_get_count() -> SigmaU32 {
    GROUP_COUNT
}
