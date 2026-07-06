// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS sigma_udev — Sovereign Device Manager
//! Linux udev equivalent: hotplug events, device nodes, rules engine.
//! no_std, no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

// ─── Kernel Primitive Types ─────────────────────────────────────────────────
type SigmaU8    = u8;
type SigmaU16   = u16;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ─── Constants ──────────────────────────────────────────────────────────────
pub const UDEV_MAX_DEVICES:   usize = 256;
pub const UDEV_MAX_RULES:     usize = 64;
pub const UDEV_MAX_LISTENERS: usize = 16;
pub const UDEV_NAME_LEN:      usize = 64;
pub const UDEV_PATH_LEN:      usize = 128;
pub const UDEV_ATTR_MAX:      usize = 16;
pub const UDEV_ATTR_VAL_LEN:  usize = 64;
pub const UDEV_EVENT_QUEUE:   usize = 64;

// ─── Device Class ───────────────────────────────────────────────────────────

/// Device class — mirrors Linux device subsystems
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum DeviceClass {
    Block     = 0,   // /sys/class/block — disks, partitions
    Char      = 1,   // /sys/class/tty, misc character devices
    Net       = 2,   // /sys/class/net — NICs, Wi-Fi, loopback
    Input     = 3,   // /sys/class/input — keyboards, mice, touchscreens
    Usb       = 4,   // /sys/bus/usb — USB devices
    Pci       = 5,   // /sys/bus/pci — PCI/PCIe devices
    Gpu       = 6,   // /sys/class/drm — GPUs, display outputs
    Sound     = 7,   // /sys/class/sound — ALSA/audio devices
    Power     = 8,   // /sys/class/power_supply — batteries, AC
    Thermal   = 9,   // /sys/class/thermal — thermal zones, cooling
    Platform  = 10,  // /sys/bus/platform — SoC peripherals
    Serial    = 11,  // /sys/class/tty — serial ports
    Storage   = 12,  // /sys/class/scsi_disk — SCSI/NVMe
    Virtual   = 13,  // Virtual/emulated devices
    Unknown   = 255,
}

/// Hotplug event action
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum UdevAction {
    Add       = 0,
    Remove    = 1,
    Change    = 2,
    Move      = 3,
    Online    = 4,
    Offline   = 5,
    Bind      = 6,
    Unbind    = 7,
}

/// Device attribute key-value pair
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceAttribute {
    pub key:   [u8; UDEV_NAME_LEN],
    pub value: [u8; UDEV_ATTR_VAL_LEN],
    pub set:   SigmaBool,
}

impl DeviceAttribute {
    pub const fn empty() -> Self {
        Self {
            key:   [0u8; UDEV_NAME_LEN],
            value: [0u8; UDEV_ATTR_VAL_LEN],
            set:   false,
        }
    }
}

/// Device node — represents a single hardware device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceNode {
    pub name:       [u8; UDEV_NAME_LEN],
    pub devpath:    [u8; UDEV_PATH_LEN],    // sysfs path
    pub devnode:    [u8; UDEV_PATH_LEN],    // /dev/ path
    pub subsystem:  [u8; UDEV_NAME_LEN],
    pub class:      DeviceClass,
    pub major:      SigmaU32,
    pub minor:      SigmaU32,
    pub vendor_id:  SigmaU16,
    pub product_id: SigmaU16,
    pub parent_idx: SigmaI32,               // index into device table, -1 = root
    pub attrs:      [DeviceAttribute; UDEV_ATTR_MAX],
    pub attr_count: SigmaU32,
    pub active:     SigmaBool,
}

impl DeviceNode {
    pub const fn empty() -> Self {
        Self {
            name:       [0u8; UDEV_NAME_LEN],
            devpath:    [0u8; UDEV_PATH_LEN],
            devnode:    [0u8; UDEV_PATH_LEN],
            subsystem:  [0u8; UDEV_NAME_LEN],
            class:      DeviceClass::Unknown,
            major:      0,
            minor:      0,
            vendor_id:  0,
            product_id: 0,
            parent_idx: -1,
            attrs:      [DeviceAttribute::empty(); UDEV_ATTR_MAX],
            attr_count: 0,
            active:     false,
        }
    }
}

// ─── Udev Rule Engine ───────────────────────────────────────────────────────

/// Rule match type
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum RuleMatchType {
    MatchSubsystem  = 0,  // SUBSYSTEM=="net"
    MatchAttr       = 1,  // ATTR{idVendor}=="1234"
    MatchDriver     = 2,  // DRIVER=="e1000"
    MatchDevpath    = 3,  // DEVPATH=="/devices/pci*"
    MatchAction     = 4,  // ACTION=="add"
    MatchKernel     = 5,  // KERNEL=="sd[a-z]"
}

/// Rule action type
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum RuleActionType {
    SetName       = 0,   // NAME="my_device"
    SetSymlink    = 1,   // SYMLINK+="disk/by-id/..."
    SetPermission = 2,   // MODE="0660"
    SetOwner      = 3,   // OWNER="root"
    SetGroup      = 4,   // GROUP="disk"
    RunProgram    = 5,   // RUN+="/usr/bin/notify"
    SetTag        = 6,   // TAG+="systemd"
}

/// A single udev rule
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UdevRule {
    pub match_type:   RuleMatchType,
    pub match_key:    [u8; UDEV_NAME_LEN],
    pub match_value:  [u8; UDEV_ATTR_VAL_LEN],
    pub action_type:  RuleActionType,
    pub action_value: [u8; UDEV_ATTR_VAL_LEN],
    pub priority:     SigmaU32,
    pub active:       SigmaBool,
}

impl UdevRule {
    pub const fn empty() -> Self {
        Self {
            match_type:   RuleMatchType::MatchSubsystem,
            match_key:    [0u8; UDEV_NAME_LEN],
            match_value:  [0u8; UDEV_ATTR_VAL_LEN],
            action_type:  RuleActionType::SetName,
            action_value: [0u8; UDEV_ATTR_VAL_LEN],
            priority:     0,
            active:       false,
        }
    }
}

// ─── Hotplug Event ──────────────────────────────────────────────────────────

/// Hotplug event emitted on device state changes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UdevEvent {
    pub action:     UdevAction,
    pub device_idx: SigmaU32,
    pub class:      DeviceClass,
    pub timestamp:  SigmaU64,      // kernel ticks since boot
    pub seqnum:     SigmaU64,      // monotonic event sequence number
    pub processed:  SigmaBool,
}

impl UdevEvent {
    pub const fn empty() -> Self {
        Self {
            action:     UdevAction::Add,
            device_idx: 0,
            class:      DeviceClass::Unknown,
            timestamp:  0,
            seqnum:     0,
            processed:  false,
        }
    }
}

/// Listener callback registration (fn pointer as u64 for FFI)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UdevListener {
    pub callback: SigmaU64,     // fn(*const UdevEvent) -> ()
    pub filter:   DeviceClass,  // only receive events for this class
    pub active:   SigmaBool,
}

impl UdevListener {
    pub const fn empty() -> Self {
        Self { callback: 0, filter: DeviceClass::Unknown, active: false }
    }
}

// ─── Global State ───────────────────────────────────────────────────────────

struct UdevState {
    devices:      [DeviceNode; UDEV_MAX_DEVICES],
    device_count: SigmaU32,
    rules:        [UdevRule; UDEV_MAX_RULES],
    rule_count:   SigmaU32,
    listeners:    [UdevListener; UDEV_MAX_LISTENERS],
    listener_cnt: SigmaU32,
    events:       [UdevEvent; UDEV_EVENT_QUEUE],
    ev_head:      SigmaU32,
    ev_tail:      SigmaU32,
    ev_count:     SigmaU32,
    next_seqnum:  SigmaU64,
    initialized:  SigmaBool,
}

static mut UDEV: UdevState = UdevState {
    devices:      [DeviceNode::empty(); UDEV_MAX_DEVICES],
    device_count: 0,
    rules:        [UdevRule::empty(); UDEV_MAX_RULES],
    rule_count:   0,
    listeners:    [UdevListener::empty(); UDEV_MAX_LISTENERS],
    listener_cnt: 0,
    events:       [UdevEvent::empty(); UDEV_EVENT_QUEUE],
    ev_head:      0,
    ev_tail:      0,
    ev_count:     0,
    next_seqnum:  1,
    initialized:  false,
};

// ─── Internal Helpers ───────────────────────────────────────────────────────

unsafe fn udev_strncpy(dst: *mut u8, src: *const u8, n: usize) {
    let mut i = 0;
    while i < n {
        let b = *src.add(i);
        *dst.add(i) = b;
        if b == 0 { return; }
        i += 1;
    }
    if n > 0 { *dst.add(n - 1) = 0; }
}

unsafe fn udev_strcmp(a: *const u8, b: *const u8) -> i32 {
    let mut i = 0usize;
    loop {
        let ca = *a.add(i);
        let cb = *b.add(i);
        if ca != cb { return ca as i32 - cb as i32; }
        if ca == 0  { return 0; }
        i += 1;
    }
}

/// Simple glob match: supports '*' and '?' only (no brackets)
unsafe fn udev_glob_match(pattern: *const u8, text: *const u8) -> bool {
    let mut pi = 0usize;
    let mut ti = 0usize;
    let mut star_p: isize = -1;
    let mut star_t: usize = 0;

    loop {
        let pc = *pattern.add(pi);
        let tc = *text.add(ti);

        if pc == b'*' {
            star_p = pi as isize;
            star_t = ti;
            pi += 1;
            continue;
        }
        if tc != 0 && (pc == b'?' || pc == tc) {
            pi += 1;
            ti += 1;
            continue;
        }
        if star_p >= 0 {
            pi = star_p as usize + 1;
            star_t += 1;
            ti = star_t;
            continue;
        }
        return pc == 0 && tc == 0;
    }
}

// ─── C-ABI Exports ──────────────────────────────────────────────────────────

/// Initialise the udev subsystem
#[no_mangle]
pub unsafe extern "C" fn sigma_udev_init() -> SigmaI32 {
    let u = &mut UDEV;
    u.device_count = 0;
    u.rule_count   = 0;
    u.listener_cnt = 0;
    u.ev_head      = 0;
    u.ev_tail      = 0;
    u.ev_count     = 0;
    u.next_seqnum  = 1;
    u.initialized  = true;
    0
}

/// Register a new device (called by bus drivers on probe)
#[no_mangle]
pub unsafe extern "C" fn sigma_udev_register_device(
    name:       *const u8,
    devpath:    *const u8,
    subsystem:  *const u8,
    class:      DeviceClass,
    major:      SigmaU32,
    minor:      SigmaU32,
    vendor_id:  SigmaU16,
    product_id: SigmaU16,
    parent_idx: SigmaI32,
) -> SigmaI32 {
    let u = &mut UDEV;
    if u.device_count as usize >= UDEV_MAX_DEVICES { return -1; }

    let idx = u.device_count as usize;
    let d = &mut u.devices[idx];

    udev_strncpy(d.name.as_mut_ptr(),      name,      UDEV_NAME_LEN);
    udev_strncpy(d.devpath.as_mut_ptr(),    devpath,   UDEV_PATH_LEN);
    udev_strncpy(d.subsystem.as_mut_ptr(),  subsystem, UDEV_NAME_LEN);
    d.class      = class;
    d.major      = major;
    d.minor      = minor;
    d.vendor_id  = vendor_id;
    d.product_id = product_id;
    d.parent_idx = parent_idx;
    d.attr_count = 0;
    d.active     = true;

    // Generate /dev/ node path: /dev/<name>
    let dev_prefix = b"/dev/\0";
    udev_strncpy(d.devnode.as_mut_ptr(), dev_prefix.as_ptr(), 5);
    let name_len = {
        let mut l = 0usize;
        while l < UDEV_NAME_LEN && *name.add(l) != 0 { l += 1; }
        l
    };
    let copy_len = if name_len > UDEV_PATH_LEN - 6 { UDEV_PATH_LEN - 6 } else { name_len };
    for i in 0..copy_len {
        d.devnode[5 + i] = *name.add(i);
    }
    d.devnode[5 + copy_len] = 0;

    u.device_count += 1;

    // Apply matching rules
    sigma_udev_apply_rules(idx as SigmaU32);

    // Emit ADD event
    sigma_udev_emit_event(UdevAction::Add, idx as SigmaU32, class);

    idx as SigmaI32
}

/// Remove a device (called on unplug)
#[no_mangle]
pub unsafe extern "C" fn sigma_udev_remove_device(device_idx: SigmaU32) -> SigmaI32 {
    let u = &mut UDEV;
    if device_idx >= u.device_count { return -1; }

    let d = &mut u.devices[device_idx as usize];
    if !d.active { return -2; }

    let class = d.class;
    d.active = false;

    sigma_udev_emit_event(UdevAction::Remove, device_idx, class);
    0
}

/// Set a device attribute (sysfs attribute equivalent)
#[no_mangle]
pub unsafe extern "C" fn sigma_udev_set_attr(
    device_idx: SigmaU32,
    key:        *const u8,
    value:      *const u8,
) -> SigmaI32 {
    let u = &mut UDEV;
    if device_idx >= u.device_count { return -1; }

    let d = &mut u.devices[device_idx as usize];

    // Check for existing attribute — overwrite
    for i in 0..d.attr_count as usize {
        if udev_strcmp(d.attrs[i].key.as_ptr(), key) == 0 {
            udev_strncpy(d.attrs[i].value.as_mut_ptr(), value, UDEV_ATTR_VAL_LEN);
            return 0;
        }
    }

    // New attribute
    if d.attr_count as usize >= UDEV_ATTR_MAX { return -2; }
    let ai = d.attr_count as usize;
    udev_strncpy(d.attrs[ai].key.as_mut_ptr(),   key,   UDEV_NAME_LEN);
    udev_strncpy(d.attrs[ai].value.as_mut_ptr(), value, UDEV_ATTR_VAL_LEN);
    d.attrs[ai].set = true;
    d.attr_count += 1;

    sigma_udev_emit_event(UdevAction::Change, device_idx, d.class);
    0
}

/// Get a device attribute value
#[no_mangle]
pub unsafe extern "C" fn sigma_udev_get_attr(
    device_idx: SigmaU32,
    key:        *const u8,
    out:        *mut u8,
    out_len:    SigmaU32,
) -> SigmaBool {
    let u = &UDEV;
    if device_idx >= u.device_count { return false; }
    let d = &u.devices[device_idx as usize];

    for i in 0..d.attr_count as usize {
        if udev_strcmp(d.attrs[i].key.as_ptr(), key) == 0 {
            udev_strncpy(out, d.attrs[i].value.as_ptr(), out_len as usize);
            return true;
        }
    }
    false
}

/// Add a udev rule
#[no_mangle]
pub unsafe extern "C" fn sigma_udev_add_rule(
    match_type:   RuleMatchType,
    match_key:    *const u8,
    match_value:  *const u8,
    action_type:  RuleActionType,
    action_value: *const u8,
    priority:     SigmaU32,
) -> SigmaI32 {
    let u = &mut UDEV;
    if u.rule_count as usize >= UDEV_MAX_RULES { return -1; }

    let ri = u.rule_count as usize;
    let r = &mut u.rules[ri];
    r.match_type  = match_type;
    r.action_type = action_type;
    r.priority    = priority;
    r.active      = true;
    udev_strncpy(r.match_key.as_mut_ptr(),    match_key,    UDEV_NAME_LEN);
    udev_strncpy(r.match_value.as_mut_ptr(),  match_value,  UDEV_ATTR_VAL_LEN);
    udev_strncpy(r.action_value.as_mut_ptr(), action_value, UDEV_ATTR_VAL_LEN);

    u.rule_count += 1;
    ri as SigmaI32
}

/// Apply matching rules to a device
unsafe fn sigma_udev_apply_rules(device_idx: SigmaU32) {
    let u = &mut UDEV;
    let d = &u.devices[device_idx as usize];

    for ri in 0..u.rule_count as usize {
        let r = &u.rules[ri];
        if !r.active { continue; }

        let matched = match r.match_type {
            RuleMatchType::MatchSubsystem => {
                udev_strcmp(d.subsystem.as_ptr(), r.match_value.as_ptr()) == 0
            }
            RuleMatchType::MatchDevpath => {
                udev_glob_match(r.match_value.as_ptr(), d.devpath.as_ptr())
            }
            RuleMatchType::MatchKernel => {
                udev_glob_match(r.match_value.as_ptr(), d.name.as_ptr())
            }
            RuleMatchType::MatchAttr => {
                let mut found = false;
                for ai in 0..d.attr_count as usize {
                    if udev_strcmp(d.attrs[ai].key.as_ptr(), r.match_key.as_ptr()) == 0
                        && udev_strcmp(d.attrs[ai].value.as_ptr(), r.match_value.as_ptr()) == 0
                    {
                        found = true;
                        break;
                    }
                }
                found
            }
            _ => false,
        };

        if matched {
            let d_mut = &mut UDEV.devices[device_idx as usize];
            match r.action_type {
                RuleActionType::SetName => {
                    udev_strncpy(d_mut.devnode.as_mut_ptr(), r.action_value.as_ptr(), UDEV_PATH_LEN);
                }
                RuleActionType::SetSymlink => {
                    // In full implementation: create symlink in /dev/
                    // Stub: store symlink target as attribute
                }
                RuleActionType::SetPermission | RuleActionType::SetOwner
                | RuleActionType::SetGroup | RuleActionType::RunProgram
                | RuleActionType::SetTag => {
                    // Each would trigger the appropriate kernel subsystem
                    // Stub: log action applied
                }
            }
        }
    }
}

/// Emit a hotplug event into the event queue
unsafe fn sigma_udev_emit_event(action: UdevAction, device_idx: SigmaU32, class: DeviceClass) {
    let u = &mut UDEV;
    let idx = (u.ev_tail % UDEV_EVENT_QUEUE as u32) as usize;

    u.events[idx] = UdevEvent {
        action,
        device_idx,
        class,
        timestamp:  0,  // filled by arch timer in real impl
        seqnum:     u.next_seqnum,
        processed:  false,
    };
    u.next_seqnum += 1;
    u.ev_tail = u.ev_tail.wrapping_add(1);
    if u.ev_count < UDEV_EVENT_QUEUE as u32 {
        u.ev_count += 1;
    } else {
        u.ev_head = u.ev_head.wrapping_add(1); // overflow: drop oldest
    }

    // Notify listeners
    sigma_udev_dispatch_listeners(&u.events[idx]);
}

/// Dispatch event to registered listeners
unsafe fn sigma_udev_dispatch_listeners(ev: &UdevEvent) {
    let u = &UDEV;
    for i in 0..u.listener_cnt as usize {
        let l = &u.listeners[i];
        if !l.active { continue; }
        // Filter by class (Unknown = wildcard = all classes)
        if l.filter != DeviceClass::Unknown && l.filter != ev.class { continue; }
        // In real impl: cast l.callback to fn pointer and call
        let _ = l.callback;
    }
}

/// Register a listener for hotplug events
#[no_mangle]
pub unsafe extern "C" fn sigma_udev_add_listener(
    callback: SigmaU64,
    filter:   DeviceClass,
) -> SigmaI32 {
    let u = &mut UDEV;
    if u.listener_cnt as usize >= UDEV_MAX_LISTENERS { return -1; }
    let li = u.listener_cnt as usize;
    u.listeners[li] = UdevListener { callback, filter, active: true };
    u.listener_cnt += 1;
    li as SigmaI32
}

/// Dequeue the next unprocessed event (returns false if queue empty)
#[no_mangle]
pub unsafe extern "C" fn sigma_udev_poll_event(out: *mut UdevEvent) -> SigmaBool {
    let u = &mut UDEV;
    if u.ev_count == 0 { return false; }

    let idx = (u.ev_head % UDEV_EVENT_QUEUE as u32) as usize;
    *out = u.events[idx];
    u.events[idx].processed = true;
    u.ev_head = u.ev_head.wrapping_add(1);
    u.ev_count -= 1;
    true
}

/// Enumerate all active devices of a given class
#[no_mangle]
pub unsafe extern "C" fn sigma_udev_enumerate_class(
    class:   DeviceClass,
    out_ids: *mut SigmaU32,
    max:     SigmaU32,
) -> SigmaU32 {
    let u = &UDEV;
    let mut count = 0u32;
    for i in 0..u.device_count as usize {
        if u.devices[i].active && (class == DeviceClass::Unknown || u.devices[i].class == class) {
            if count < max {
                *out_ids.add(count as usize) = i as SigmaU32;
            }
            count += 1;
        }
    }
    count
}

/// Get device info by index
#[no_mangle]
pub unsafe extern "C" fn sigma_udev_get_device(
    device_idx: SigmaU32,
    out:        *mut DeviceNode,
) -> SigmaBool {
    let u = &UDEV;
    if device_idx >= u.device_count { return false; }
    if !u.devices[device_idx as usize].active { return false; }
    *out = u.devices[device_idx as usize];
    true
}

/// Get total device count
#[no_mangle]
pub unsafe extern "C" fn sigma_udev_device_count() -> SigmaU32 {
    unsafe { UDEV.device_count }
}

/// Trigger a rescan of all devices (re-applies rules)
#[no_mangle]
pub unsafe extern "C" fn sigma_udev_trigger() -> SigmaI32 {
    let u = &UDEV;
    let count = u.device_count;
    for i in 0..count {
        if UDEV.devices[i as usize].active {
            sigma_udev_apply_rules(i);
        }
    }
    0
}
