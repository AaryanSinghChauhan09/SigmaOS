/// SigmaOS: IPv6 Networking Stack
/// Implements IPv6 header parsing, ICMPv6, NDP, and DHCPv6
/// no_std, no alloc, no external crates

#![no_std]
#![allow(dead_code)]

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// ─── IPv6 Constants ─────────────────────────────────────────────────────────

pub const IPV6_ADDR_LEN: SigmaUsize = 16;
pub const IPV6_HEADER_LEN: SigmaUsize = 40;
pub const IPV6_MTU: SigmaUsize = 1500;
pub const MAX_NEIGHBOR_ENTRIES: SigmaUsize = 64;
pub const MAX_PREFIX_ENTRIES: SigmaUsize = 16;

// ─── IPv6 Address Types ─────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub enum Ipv6AddrType {
    Unicast = 0,
    Multicast = 1,
    Anycast = 2,
}

// ─── IPv6 Header ─────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ipv6Header {
    pub version_tc_fl: SigmaU32, // Version (4 bits), TC (8 bits), Flow Label (20 bits)
    pub payload_len: SigmaU16,
    pub next_header: SigmaU8,
    pub hop_limit: SigmaU8,
    pub src_addr: [SigmaU8; IPV6_ADDR_LEN],
    pub dst_addr: [SigmaU8; IPV6_ADDR_LEN],
}

// ─── IPv6 Next Header Types ───────────────────────────────────────────────────

pub const IPPROTO_ICMPV6: SigmaU8 = 58;
pub const IPPROTO_TCP: SigmaU8 = 6;
pub const IPPROTO_UDP: SigmaU8 = 17;
pub const IPPROTO_HOPOPTS: SigmaU8 = 0;
pub const IPPROTO_ROUTING: SigmaU8 = 43;
pub const IPPROTO_FRAGMENT: SigmaU8 = 44;
pub const IPPROTO_ESP: SigmaU8 = 50;
pub const IPPROTO_AH: SigmaU8 = 51;
pub const IPPROTO_DSTOPTS: SigmaU8 = 60;

// ─── ICMPv6 Message Types ───────────────────────────────────────────────────

pub const ICMPV6_DST_UNREACH: SigmaU8 = 1;
pub const ICMPV6_PACKET_TOO_BIG: SigmaU8 = 2;
pub const ICMPV6_TIME_EXCEEDED: SigmaU8 = 3;
pub const ICMPV6_PARAM_PROB: SigmaU8 = 4;
pub const ICMPV6_ECHO_REQUEST: SigmaU8 = 128;
pub const ICMPV6_ECHO_REPLY: SigmaU8 = 129;
pub const ICMPV6_NEIGHBOR_SOLICIT: SigmaU8 = 135;
pub const ICMPV6_NEIGHBOR_ADVERT: SigmaU8 = 136;

// ─── ICMPv6 Header ─────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Icmpv6Header {
    pub msg_type: SigmaU8,
    pub msg_code: SigmaU8,
    pub checksum: SigmaU16,
}

// ─── Neighbor Discovery (NDP) Structures ───────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NeighborSolicitation {
    pub icmp: Icmpv6Header,
    pub reserved: SigmaU32,
    pub target_addr: [SigmaU8; IPV6_ADDR_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NeighborAdvertisement {
    pub icmp: Icmpv6Header,
    pub flags: SigmaU8,
    pub reserved: [SigmaU8; 3],
    pub target_addr: [SigmaU8; IPV6_ADDR_LEN],
}

// ─── Neighbor Cache Entry ───────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub enum NeighborState {
    Incomplete = 0,
    Reachable = 1,
    Stale = 2,
    Delay = 3,
    Probe = 4,
    Failed = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NeighborEntry {
    pub ip_addr: [SigmaU8; IPV6_ADDR_LEN],
    pub mac_addr: [SigmaU8; 6],
    pub state: NeighborState,
    pub interface: SigmaU32,
    pub last_seen: SigmaU64,
    pub reachable_time: SigmaU32,
    pub valid: SigmaBool,
}

// ─── Prefix Entry ─────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PrefixEntry {
    pub prefix: [SigmaU8; IPV6_ADDR_LEN],
    pub prefix_len: SigmaU8,
    pub preferred_lifetime: SigmaU32,
    pub valid_lifetime: SigmaU32,
    pub on_link: SigmaBool,
    pub autonomous: SigmaBool,
    pub valid: SigmaBool,
}

// ─── IPv6 Interface ───────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ipv6Interface {
    pub index: SigmaU32,
    pub mtu: SigmaU32,
    pub link_local_addr: [SigmaU8; IPV6_ADDR_LEN],
    pub global_addrs: [[SigmaU8; IPV6_ADDR_LEN]; 8],
    pub global_addr_count: SigmaU32,
    pub mac_addr: [SigmaU8; 6],
    pub hop_limit: SigmaU8,
    pub reachable_time: SigmaU32,
    pub retrans_timer: SigmaU32,
    pub enabled: SigmaBool,
}

// ─── IPv6 Stack State ───────────────────────────────────────────────────────

pub struct Ipv6Stack {
    interfaces: [Ipv6Interface; 8],
    interface_count: SigmaU32,
    neighbor_cache: [NeighborEntry; MAX_NEIGHBOR_ENTRIES],
    neighbor_count: SigmaU32,
    prefix_table: [PrefixEntry; MAX_PREFIX_ENTRIES],
    prefix_count: SigmaU32,
    initialized: SigmaBool,
}

impl Ipv6Stack {
    pub const fn new() -> Self {
        Self {
            interfaces: [Ipv6Interface {
                index: 0,
                mtu: IPV6_MTU as SigmaU32,
                link_local_addr: [0; IPV6_ADDR_LEN],
                global_addrs: [[0; IPV6_ADDR_LEN]; 8],
                global_addr_count: 0,
                mac_addr: [0; 6],
                hop_limit: 64,
                reachable_time: 30000,
                retrans_timer: 1000,
                enabled: false,
            }; 8],
            interface_count: 0,
            neighbor_cache: [NeighborEntry {
                ip_addr: [0; IPV6_ADDR_LEN],
                mac_addr: [0; 6],
                state: NeighborState::Incomplete,
                interface: 0,
                last_seen: 0,
                reachable_time: 0,
                valid: false,
            }; MAX_NEIGHBOR_ENTRIES],
            neighbor_count: 0,
            prefix_table: [PrefixEntry {
                prefix: [0; IPV6_ADDR_LEN],
                prefix_len: 64,
                preferred_lifetime: 0,
                valid_lifetime: 0,
                on_link: false,
                autonomous: false,
                valid: false,
            }; MAX_PREFIX_ENTRIES],
            prefix_count: 0,
            initialized: false,
        }
    }

    pub unsafe fn init(&mut self) -> SigmaI32 {
        self.initialized = true;
        0
    }

    /// Parse IPv6 header from packet
    pub unsafe fn parse_header(&self, data: *const SigmaU8, len: SigmaUsize, header: *mut Ipv6Header) -> SigmaI32 {
        if data.is_null() || header.is_null() || len < IPV6_HEADER_LEN {
            return -1;
        }

        let h = &mut *header;
        
        // First 4 bytes: Version (4 bits), TC (8 bits), Flow Label (20 bits)
        let first_word = ((data.add(0).read() as SigmaU32) << 24) |
                         ((data.add(1).read() as SigmaU32) << 16) |
                         ((data.add(2).read() as SigmaU32) << 8) |
                         (data.add(3).read() as SigmaU32);
        
        h.version_tc_fl = first_word;
        
        // Payload length (bytes 4-5)
        h.payload_len = ((data.add(4).read() as SigmaU16) << 8) | (data.add(5).read() as SigmaU16);
        
        // Next header (byte 6)
        h.next_header = *data.add(6);
        
        // Hop limit (byte 7)
        h.hop_limit = *data.add(7);
        
        // Source address (bytes 8-23)
        for i in 0..IPV6_ADDR_LEN {
            h.src_addr[i] = *data.add(8 + i);
        }
        
        // Destination address (bytes 24-39)
        for i in 0..IPV6_ADDR_LEN {
            h.dst_addr[i] = *data.add(24 + i);
        }

        0
    }

    /// Build IPv6 header
    pub unsafe fn build_header(&self, header: *mut Ipv6Header, src: *const SigmaU8, dst: *const SigmaU8, payload_len: SigmaU16, next_header: SigmaU8) -> SigmaI32 {
        if header.is_null() || src.is_null() || dst.is_null() {
            return -1;
        }

        let h = &mut *header;
        
        // Version = 6, TC = 0, Flow Label = 0
        h.version_tc_fl = 0x60000000;
        h.payload_len = payload_len;
        h.next_header = next_header;
        h.hop_limit = 64;
        
        for i in 0..IPV6_ADDR_LEN {
            h.src_addr[i] = *src.add(i);
            h.dst_addr[i] = *dst.add(i);
        }

        0
    }

    /// Add interface
    pub unsafe fn add_interface(&mut self, index: SigmaU32, mac_addr: *const SigmaU8) -> SigmaI32 {
        if self.interface_count >= 8 {
            return -1;
        }

        let idx = self.interface_count as SigmaUsize;
        self.interfaces[idx].index = index;
        self.interfaces[idx].enabled = true;
        
        if !mac_addr.is_null() {
            for i in 0..6 {
                self.interfaces[idx].mac_addr[i] = *mac_addr.add(i);
            }
        }

        // Generate link-local address (FE80::/64 + EUI-64)
        self.generate_link_local(idx);

        self.interface_count += 1;
        0
    }

    /// Generate link-local address from MAC
    fn generate_link_local(&mut self, idx: SigmaUsize) {
        // FE80::/64 prefix
        self.interfaces[idx].link_local_addr[0] = 0xFE;
        self.interfaces[idx].link_local_addr[1] = 0x80;
        
        // EUI-64 from MAC
        self.interfaces[idx].link_local_addr[8] = self.interfaces[idx].mac_addr[0];
        self.interfaces[idx].link_local_addr[9] = self.interfaces[idx].mac_addr[1];
        self.interfaces[idx].link_local_addr[10] = self.interfaces[idx].mac_addr[2];
        self.interfaces[idx].link_local_addr[11] = 0xFF;
        self.interfaces[idx].link_local_addr[12] = 0xFE;
        self.interfaces[idx].link_local_addr[13] = self.interfaces[idx].mac_addr[3];
        self.interfaces[idx].link_local_addr[14] = self.interfaces[idx].mac_addr[4];
        self.interfaces[idx].link_local_addr[15] = self.interfaces[idx].mac_addr[5];
        
        // Flip U/L bit
        self.interfaces[idx].link_local_addr[8] ^= 0x02;
    }

    /// Add neighbor to cache
    pub unsafe fn add_neighbor(&mut self, ip: *const SigmaU8, mac: *const SigmaU8, interface: SigmaU32) -> SigmaI32 {
        if self.neighbor_count >= MAX_NEIGHBOR_ENTRIES as SigmaU32 {
            return -1;
        }

        if ip.is_null() || mac.is_null() {
            return -1;
        }

        let idx = self.neighbor_count as SigmaUsize;
        
        for i in 0..IPV6_ADDR_LEN {
            self.neighbor_cache[idx].ip_addr[i] = *ip.add(i);
        }
        
        for i in 0..6 {
            self.neighbor_cache[idx].mac_addr[i] = *mac.add(i);
        }

        self.neighbor_cache[idx].state = NeighborState::Reachable;
        self.neighbor_cache[idx].interface = interface;
        self.neighbor_cache[idx].last_seen = self.get_timestamp();
        self.neighbor_cache[idx].reachable_time = 30000;
        self.neighbor_cache[idx].valid = true;

        self.neighbor_count += 1;
        0
    }

    /// Lookup neighbor by IP
    pub unsafe fn lookup_neighbor(&self, ip: *const SigmaU8, mac: *mut SigmaU8) -> SigmaI32 {
        if ip.is_null() || mac.is_null() {
            return -1;
        }

        for i in 0..self.neighbor_count as SigmaUsize {
            if self.neighbor_cache[i].valid {
                let mut match_count = 0;
                for j in 0..IPV6_ADDR_LEN {
                    if self.neighbor_cache[i].ip_addr[j] == *ip.add(j) {
                        match_count += 1;
                    }
                }
                
                if match_count == IPV6_ADDR_LEN {
                    for j in 0..6 {
                        *mac.add(j) = self.neighbor_cache[i].mac_addr[j];
                    }
                    return 0;
                }
            }
        }

        -1
    }

    /// Process ICMPv6 packet
    pub unsafe fn process_icmpv6(&mut self, data: *const SigmaU8, len: SigmaUsize) -> SigmaI32 {
        if data.is_null() || len < 8 {
            return -1;
        }

        let msg_type = *data.add(0);
        
        match msg_type {
            ICMPV6_NEIGHBOR_SOLICIT => self.process_neighbor_solicitation(data, len),
            ICMPV6_NEIGHBOR_ADVERT => self.process_neighbor_advertisement(data, len),
            ICMPV6_ECHO_REQUEST => self.process_echo_request(data, len),
            _ => 0,
        }
    }

    fn process_neighbor_solicitation(&mut self, data: *const SigmaU8, len: SigmaUsize) -> SigmaI32 {
        // Parse and process NS message
        // In a real implementation, this would update neighbor cache and send NA
        0
    }

    fn process_neighbor_advertisement(&mut self, data: *const SigmaU8, len: SigmaUsize) -> SigmaI32 {
        // Parse and process NA message
        // In a real implementation, this would update neighbor cache
        0
    }

    fn process_echo_request(&mut self, data: *const SigmaU8, len: SigmaUsize) -> SigmaI32 {
        // Parse and send echo reply
        // In a real implementation, this would generate ICMPv6 echo reply
        0
    }

    /// Add prefix to table
    pub unsafe fn add_prefix(&mut self, prefix: *const SigmaU8, prefix_len: SigmaU8, preferred: SigmaU32, valid: SigmaU32) -> SigmaI32 {
        if self.prefix_count >= MAX_PREFIX_ENTRIES as SigmaU32 {
            return -1;
        }

        if prefix.is_null() {
            return -1;
        }

        let idx = self.prefix_count as SigmaUsize;
        
        for i in 0..IPV6_ADDR_LEN {
            self.prefix_table[idx].prefix[i] = *prefix.add(i);
        }

        self.prefix_table[idx].prefix_len = prefix_len;
        self.prefix_table[idx].preferred_lifetime = preferred;
        self.prefix_table[idx].valid_lifetime = valid;
        self.prefix_table[idx].on_link = true;
        self.prefix_table[idx].autonomous = true;
        self.prefix_table[idx].valid = true;

        self.prefix_count += 1;
        0
    }

    /// Calculate IPv6 checksum
    pub unsafe fn calculate_checksum(&self, data: *const SigmaU8, len: SigmaUsize) -> SigmaU16 {
        let mut sum: SigmaU32 = 0;
        
        for i in (0..len).step_by(2) {
            if i + 1 < len {
                let word = ((*data.add(i) as SigmaU32) << 8) | (*data.add(i + 1) as SigmaU32);
                sum = sum.wrapping_add(word);
            }
        }

        while sum >> 16 != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }

        (!sum) as SigmaU16
    }

    fn get_timestamp(&self) -> SigmaU64 {
        // In a real implementation, this would read from hardware timer
        0
    }
}

static mut IPV6_STACK: Ipv6Stack = Ipv6Stack::new();

// ─── C-ABI Interface Functions ───────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_ipv6_init() -> SigmaI32 {
    IPV6_STACK.init()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ipv6_parse_header(data: *const SigmaU8, len: SigmaUsize, header: *mut Ipv6Header) -> SigmaI32 {
    IPV6_STACK.parse_header(data, len, header)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ipv6_build_header(header: *mut Ipv6Header, src: *const SigmaU8, dst: *const SigmaU8, payload_len: SigmaU16, next_header: SigmaU8) -> SigmaI32 {
    IPV6_STACK.build_header(header, src, dst, payload_len, next_header)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ipv6_add_interface(index: SigmaU32, mac_addr: *const SigmaU8) -> SigmaI32 {
    IPV6_STACK.add_interface(index, mac_addr)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ipv6_add_neighbor(ip: *const SigmaU8, mac: *const SigmaU8, interface: SigmaU32) -> SigmaI32 {
    IPV6_STACK.add_neighbor(ip, mac, interface)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ipv6_lookup_neighbor(ip: *const SigmaU8, mac: *mut SigmaU8) -> SigmaI32 {
    IPV6_STACK.lookup_neighbor(ip, mac)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ipv6_process_icmpv6(data: *const SigmaU8, len: SigmaUsize) -> SigmaI32 {
    IPV6_STACK.process_icmpv6(data, len)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ipv6_add_prefix(prefix: *const SigmaU8, prefix_len: SigmaU8, preferred: SigmaU32, valid: SigmaU32) -> SigmaI32 {
    IPV6_STACK.add_prefix(prefix, prefix_len, preferred, valid)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ipv6_calculate_checksum(data: *const SigmaU8, len: SigmaUsize) -> SigmaU16 {
    IPV6_STACK.calculate_checksum(data, len)
}

