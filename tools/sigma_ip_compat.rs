//! SigmaOS IP Command Compatibility
//! Network configuration (ip command)
//! Zero external dependencies

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaU64 = u64;

/// Network interface
#[repr(C)]
pub struct NetworkInterface {
    pub name: [u8; 16],
    pub index: SigmaU32,
    pub up: SigmaBool,
    pub mtu: SigmaU32,
    pub mac: [u8; 6],
    pub ipv4_address: [u8; 4],
    pub ipv4_netmask: [u8; 4],
    pub ipv6_address: [u8; 16],
    pub ipv6_prefix: SigmaU32,
}

/// Route entry
#[repr(C)]
pub struct RouteEntry {
    pub destination: [u8; 16],
    pub gateway: [u8; 16],
    pub netmask: SigmaU32,
    pub interface: SigmaU32,
    pub metric: SigmaU32,
}

/// IP state
const MAX_INTERFACES: usize = 32;
const MAX_ROUTES: usize = 128;

static mut INTERFACES: [NetworkInterface; MAX_INTERFACES] = [NetworkInterface {
    name: [0; 16],
    index: 0,
    up: false,
    mtu: 1500,
    mac: [0; 6],
    ipv4_address: [0; 4],
    ipv4_netmask: [0; 4],
    ipv6_address: [0; 16],
    ipv6_prefix: 64,
}; MAX_INTERFACES];

static mut ROUTES: [RouteEntry; MAX_ROUTES] = [RouteEntry {
    destination: [0; 16],
    gateway: [0; 16],
    netmask: 0,
    interface: 0,
    metric: 0,
}; MAX_ROUTES];

static mut INTERFACE_COUNT: SigmaU32 = 0;
static mut ROUTE_COUNT: SigmaU32 = 0;
static mut IP_INITIALIZED: SigmaBool = false;

/// Initialize IP command
#[no_mangle]
pub unsafe extern "C" fn ip_init() -> SigmaI32 {
    IP_INITIALIZED = true;
    INTERFACE_COUNT = 0;
    ROUTE_COUNT = 0;
    
    // Add loopback interface
    let mut lo = NetworkInterface {
        name: [0; 16],
        index: 1,
        up: true,
        mtu: 65536,
        mac: [0; 6],
        ipv4_address: [127, 0, 0, 1],
        ipv4_netmask: [255, 0, 0, 0],
        ipv6_address: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        ipv6_prefix: 128,
    };
    
    for i in 0..15 {
        lo.name[i] = b"lo"[i.min(2)];
    }
    
    INTERFACES[0] = lo;
    INTERFACE_COUNT = 1;
    
    0 // Success
}

/// List interfaces
#[no_mangle]
pub unsafe extern "C" fn ip_link_list(interfaces: *mut NetworkInterface, max_count: SigmaU32) -> SigmaU32 {
    if !IP_INITIALIZED || interfaces.is_null() {
        return 0;
    }
    
    let mut count = 0;
    for i in 0..INTERFACE_COUNT as usize {
        if count >= max_count as usize {
            break;
        }
        *interfaces.add(count) = INTERFACES[i];
        count += 1;
    }
    
    count
}

/// Get interface by name
#[no_mangle]
pub unsafe extern "C" fn ip_link_get_by_name(name: *const u8, interface: *mut NetworkInterface) -> SigmaI32 {
    if !IP_INITIALIZED || name.is_null() || interface.is_null() {
        return -1;
    }
    
    for i in 0..INTERFACE_COUNT as usize {
        let iface = &INTERFACES[i];
        
        let mut matches = true;
        for j in 0..16 {
            if iface.name[j] != *name.add(j) {
                if iface.name[j] == 0 && *name.add(j) == 0 {
                    break;
                }
                matches = false;
                break;
            }
            if iface.name[j] == 0 {
                break;
            }
        }
        
        if matches {
            *interface = *iface;
            return 0;
        }
    }
    
    -2 // Interface not found
}

/// Set interface up
#[no_mangle]
pub unsafe extern "C" fn ip_link_set_up(name: *const u8) -> SigmaI32 {
    if !IP_INITIALIZED || name.is_null() {
        return -1;
    }
    
    for i in 0..INTERFACE_COUNT as usize {
        let iface = &mut INTERFACES[i];
        
        let mut matches = true;
        for j in 0..16 {
            if iface.name[j] != *name.add(j) {
                if iface.name[j] == 0 && *name.add(j) == 0 {
                    break;
                }
                matches = false;
                break;
            }
            if iface.name[j] == 0 {
                break;
            }
        }
        
        if matches {
            iface.up = true;
            return 0;
        }
    }
    
    -2 // Interface not found
}

/// Set interface down
#[no_mangle]
pub unsafe extern "C" fn ip_link_set_down(name: *const u8) -> SigmaI32 {
    if !IP_INITIALIZED || name.is_null() {
        return -1;
    }
    
    for i in 0..INTERFACE_COUNT as usize {
        let iface = &mut INTERFACES[i];
        
        let mut matches = true;
        for j in 0..16 {
            if iface.name[j] != *name.add(j) {
                if iface.name[j] == 0 && *name.add(j) == 0 {
                    break;
                }
                matches = false;
                break;
            }
            if iface.name[j] == 0 {
                break;
            }
        }
        
        if matches {
            iface.up = false;
            return 0;
        }
    }
    
    -2 // Interface not found
}

/// Add IPv4 address
#[no_mangle]
pub unsafe extern "C" fn ip_addr_add(
    name: *const u8,
    address: *const u8,
    netmask: *const u8,
) -> SigmaI32 {
    if !IP_INITIALIZED || name.is_null() || address.is_null() || netmask.is_null() {
        return -1;
    }
    
    for i in 0..INTERFACE_COUNT as usize {
        let iface = &mut INTERFACES[i];
        
        let mut matches = true;
        for j in 0..16 {
            if iface.name[j] != *name.add(j) {
                if iface.name[j] == 0 && *name.add(j) == 0 {
                    break;
                }
                matches = false;
                break;
            }
            if iface.name[j] == 0 {
                break;
            }
        }
        
        if matches {
            for j in 0..4 {
                iface.ipv4_address[j] = *address.add(j);
                iface.ipv4_netmask[j] = *netmask.add(j);
            }
            return 0;
        }
    }
    
    -2 // Interface not found
}

/// Delete IPv4 address
#[no_mangle]
pub unsafe extern "C" fn ip_addr_del(name: *const u8) -> SigmaI32 {
    if !IP_INITIALIZED || name.is_null() {
        return -1;
    }
    
    for i in 0..INTERFACE_COUNT as usize {
        let iface = &mut INTERFACES[i];
        
        let mut matches = true;
        for j in 0..16 {
            if iface.name[j] != *name.add(j) {
                if iface.name[j] == 0 && *name.add(j) == 0 {
                    break;
                }
                matches = false;
                break;
            }
            if iface.name[j] == 0 {
                break;
            }
        }
        
        if matches {
            iface.ipv4_address = [0; 4];
            iface.ipv4_netmask = [0; 4];
            return 0;
        }
    }
    
    -2 // Interface not found
}

/// List routes
#[no_mangle]
pub unsafe extern "C" fn ip_route_list(routes: *mut RouteEntry, max_count: SigmaU32) -> SigmaU32 {
    if !IP_INITIALIZED || routes.is_null() {
        return 0;
    }
    
    let mut count = 0;
    for i in 0..ROUTE_COUNT as usize {
        if count >= max_count as usize {
            break;
        }
        *routes.add(count) = ROUTES[i];
        count += 1;
    }
    
    count
}

/// Add route
#[no_mangle]
pub unsafe extern "C" fn ip_route_add(
    destination: *const u8,
    gateway: *const u8,
    interface: SigmaU32,
) -> SigmaI32 {
    if !IP_INITIALIZED || ROUTE_COUNT >= MAX_ROUTES as SigmaU32 {
        return -1;
    }
    
    let mut route = RouteEntry {
        destination: [0; 16],
        gateway: [0; 16],
        netmask: 24,
        interface,
        metric: 100,
    };
    
    if !destination.is_null() {
        for i in 0..16 {
            let byte = *destination.add(i);
            if i < 4 {
                route.destination[i] = byte;
            }
            if byte == 0 { break; }
        }
    }
    
    if !gateway.is_null() {
        for i in 0..16 {
            let byte = *gateway.add(i);
            if i < 4 {
                route.gateway[i] = byte;
            }
            if byte == 0 { break; }
        }
    }
    
    ROUTES[ROUTE_COUNT as usize] = route;
    ROUTE_COUNT += 1;
    
    0 // Success
}

/// Get interface count
#[no_mangle]
pub unsafe extern "C" fn ip_get_interface_count() -> SigmaU32 {
    INTERFACE_COUNT
}

/// Get route count
#[no_mangle]
pub unsafe extern "C" fn ip_get_route_count() -> SigmaU32 {
    ROUTE_COUNT
}
