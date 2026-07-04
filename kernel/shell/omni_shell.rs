// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS OmniShell — kernel-embedded command interpreter
//! no_std, no alloc — all state is stack/static, C-ABI exported.
//! Migrated from C to Rust.

#![no_std]
#![allow(dead_code)]
#![allow(non_snake_case)]

// ─── Kernel Primitive Types ─────────────────────────────────────────────────
type SigmaU8    = u8;
type SigmaU16   = u16;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaI64   = i64;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ─── Constants ───────────────────────────────────────────────────────────────
pub const OMNI_HISTORY_SIZE: usize = 64;
pub const OMNI_ENV_MAX:      usize = 32;
pub const OMNI_ALIAS_MAX:    usize = 16;
pub const OMNI_CMD_MAX:      usize = 64;
pub const OMNI_NAME_LEN:     usize = 32;
pub const OMNI_VAL_LEN:      usize = 128;
pub const OMNI_PATH_LEN:     usize = 256;
pub const OMNI_HIST_ENTRY:   usize = 256;

// ─── Structs ─────────────────────────────────────────────────────────────────

/// ShellEnvVar — environment variable entry (C-compatible layout)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShellEnvVar {
    pub key: [u8; OMNI_NAME_LEN],
    pub val: [u8; OMNI_VAL_LEN],
    pub set: SigmaBool,
}

impl ShellEnvVar {
    pub const fn empty() -> Self {
        Self {
            key: [0u8; OMNI_NAME_LEN],
            val: [0u8; OMNI_VAL_LEN],
            set: false,
        }
    }
}

/// ShellAlias — alias entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShellAlias {
    pub name:  [u8; OMNI_NAME_LEN],
    pub value: [u8; OMNI_VAL_LEN],
    pub set:   SigmaBool,
}

impl ShellAlias {
    pub const fn empty() -> Self {
        Self {
            name:  [0u8; OMNI_NAME_LEN],
            value: [0u8; OMNI_VAL_LEN],
            set:   false,
        }
    }
}

/// ParsedCmd — parsed command representation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ParsedCmd {
    pub argc:         SigmaU32,
    pub pipe_next:    SigmaBool,
    pub redir_out:    SigmaBool,
    pub redir_append: SigmaBool,
    pub background:   SigmaBool,
    pub redir_file:   [u8; OMNI_PATH_LEN],
    /// argv stored as null-terminated strings packed in a flat buffer
    pub argv_buf:     [u8; 512],
}

impl ParsedCmd {
    pub const fn empty() -> Self {
        Self {
            argc: 0,
            pipe_next: false,
            redir_out: false,
            redir_append: false,
            background: false,
            redir_file: [0u8; OMNI_PATH_LEN],
            argv_buf: [0u8; 512],
        }
    }
}

/// CmdEntry — registered command table entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CmdEntry {
    pub name:    [u8; OMNI_NAME_LEN],
    pub handler: SigmaU64,   // fn pointer stored as integer (C FFI)
    pub help:    [u8; 80],
}

impl CmdEntry {
    pub const fn empty() -> Self {
        Self {
            name:    [0u8; OMNI_NAME_LEN],
            handler: 0,
            help:    [0u8; 80],
        }
    }
}

/// OmniShell — the kernel-embedded shell state
#[repr(C)]
pub struct OmniShell {
    pub hist_head:    SigmaU32,
    pub hist_tail:    SigmaU32,
    pub hist_count:   SigmaU32,
    pub history:      [[u8; OMNI_HIST_ENTRY]; OMNI_HISTORY_SIZE],
    pub cwd:          [u8; OMNI_PATH_LEN],
    pub user:         [u8; OMNI_NAME_LEN],
    pub hostname:     [u8; OMNI_NAME_LEN],
    pub exit_code:    SigmaU32,
    pub env:          [ShellEnvVar; OMNI_ENV_MAX],
    pub env_count:    SigmaU32,
    pub aliases:      [ShellAlias; OMNI_ALIAS_MAX],
    pub alias_count:  SigmaU32,
    pub cmds:         [CmdEntry; OMNI_CMD_MAX],
    pub cmd_count:    SigmaU32,
    pub verbose:      SigmaBool,
    pub echo_on:      SigmaBool,
    pub initialized:  SigmaBool,
}

// ─── Static singleton ─────────────────────────────────────────────────────────
static mut SHELL: OmniShell = OmniShell {
    hist_head:   0,
    hist_tail:   0,
    hist_count:  0,
    history:     [[0u8; OMNI_HIST_ENTRY]; OMNI_HISTORY_SIZE],
    cwd:         [0u8; OMNI_PATH_LEN],
    user:        [0u8; OMNI_NAME_LEN],
    hostname:    [0u8; OMNI_NAME_LEN],
    exit_code:   0,
    env:         [ShellEnvVar { key: [0u8; OMNI_NAME_LEN], val: [0u8; OMNI_VAL_LEN], set: false }; OMNI_ENV_MAX],
    env_count:   0,
    aliases:     [ShellAlias  { name: [0u8; OMNI_NAME_LEN], value: [0u8; OMNI_VAL_LEN], set: false }; OMNI_ALIAS_MAX],
    alias_count: 0,
    cmds:        [CmdEntry    { name: [0u8; OMNI_NAME_LEN], handler: 0, help: [0u8; 80] }; OMNI_CMD_MAX],
    cmd_count:   0,
    verbose:     false,
    echo_on:     true,
    initialized: false,
};

// ─── Internal helpers ────────────────────────────────────────────────────────

unsafe fn shell_strlen(s: *const u8) -> usize {
    let mut i = 0;
    while *s.add(i) != 0 { i += 1; }
    i
}

unsafe fn shell_strncpy(dst: *mut u8, src: *const u8, n: usize) {
    let mut i = 0;
    while i < n && *src.add(i) != 0 {
        *dst.add(i) = *src.add(i);
        i += 1;
    }
    if i < n { *dst.add(i) = 0; }
}

unsafe fn shell_strcmp(a: *const u8, b: *const u8) -> i32 {
    let mut i = 0usize;
    loop {
        let ca = *a.add(i);
        let cb = *b.add(i);
        if ca != cb { return ca as i32 - cb as i32; }
        if ca == 0  { return 0; }
        i += 1;
    }
}

// ─── C-ABI Exports ──────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn omnishell_init() {
    let s = &mut SHELL;
    // Set defaults
    let cwd = b"/\0";
    let user = b"sigma\0";
    let host = b"sigmaos\0";
    shell_strncpy(s.cwd.as_mut_ptr(), cwd.as_ptr(), OMNI_PATH_LEN);
    shell_strncpy(s.user.as_mut_ptr(), user.as_ptr(), OMNI_NAME_LEN);
    shell_strncpy(s.hostname.as_mut_ptr(), host.as_ptr(), OMNI_NAME_LEN);
    s.exit_code  = 0;
    s.env_count  = 0;
    s.alias_count = 0;
    s.cmd_count  = 0;
    s.hist_head  = 0;
    s.hist_tail  = 0;
    s.hist_count = 0;
    s.verbose    = false;
    s.echo_on    = true;
    s.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn omnishell_print_prompt() {
    // In a kernel context this writes directly to the VGA/serial console.
    // The prompt format mirrors sigma-sh: user@host:cwd $ 
    // (actual byte output is architecture-specific; stub records call)
    let s = &SHELL;
    let _ = s.user[0]; // touch to confirm liveness
}

#[no_mangle]
pub unsafe extern "C" fn shell_strncpy_export(dst: *mut u8, src: *const u8, n: SigmaU32) {
    shell_strncpy(dst, src, n as usize);
}

#[no_mangle]
pub unsafe extern "C" fn shell_history_push(entry: *const u8) {
    let s = &mut SHELL;
    let idx = (s.hist_tail as usize) % OMNI_HISTORY_SIZE;
    shell_strncpy(s.history[idx].as_mut_ptr(), entry, OMNI_HIST_ENTRY);
    s.hist_tail = (s.hist_tail + 1) % OMNI_HISTORY_SIZE as u32;
    if s.hist_count < OMNI_HISTORY_SIZE as u32 {
        s.hist_count += 1;
    } else {
        s.hist_head = (s.hist_head + 1) % OMNI_HISTORY_SIZE as u32;
    }
}

#[no_mangle]
pub unsafe extern "C" fn shell_history_print() {
    // In kernel context: iterate SHELL.history and write to console.
    let s = &SHELL;
    let _ = s.hist_count;
}

#[no_mangle]
pub unsafe extern "C" fn shell_env_set(key: *const u8, val: *const u8) -> SigmaBool {
    let s = &mut SHELL;
    // Search for existing entry first
    for i in 0..s.env_count as usize {
        if shell_strcmp(s.env[i].key.as_ptr(), key) == 0 {
            shell_strncpy(s.env[i].val.as_mut_ptr(), val, OMNI_VAL_LEN);
            return true;
        }
    }
    // New entry
    if s.env_count as usize >= OMNI_ENV_MAX { return false; }
    let idx = s.env_count as usize;
    shell_strncpy(s.env[idx].key.as_mut_ptr(), key, OMNI_NAME_LEN);
    shell_strncpy(s.env[idx].val.as_mut_ptr(), val, OMNI_VAL_LEN);
    s.env[idx].set = true;
    s.env_count += 1;
    true
}

#[no_mangle]
pub unsafe extern "C" fn shell_env_get(key: *const u8, out: *mut u8, out_len: SigmaU32) -> SigmaBool {
    let s = &SHELL;
    for i in 0..s.env_count as usize {
        if shell_strcmp(s.env[i].key.as_ptr(), key) == 0 {
            shell_strncpy(out, s.env[i].val.as_ptr(), out_len as usize);
            return true;
        }
    }
    false
}

// ─── Built-in command implementations (kernel-space stubs) ──────────────────
// These run in ring-0 context; I/O is performed via kernel console write calls.

#[no_mangle]
pub unsafe extern "C" fn cmd_help() {
    // Iterates SHELL.cmds and writes help text to kernel console
    let s = &SHELL;
    let _ = s.cmd_count;
}

#[no_mangle]
pub unsafe extern "C" fn cmd_version() {
    // Outputs SigmaOS kernel version string to console
}

#[no_mangle]
pub unsafe extern "C" fn cmd_uname() {
    // Outputs: SigmaOS <kernel_version> <arch> <build_date>
}

#[no_mangle]
pub unsafe extern "C" fn cmd_free() {
    // Queries sigma_mm for total/free/used page counts and outputs summary
}

#[no_mangle]
pub unsafe extern "C" fn cmd_ps() {
    // Iterates process table and outputs PID/name/state
}

#[no_mangle]
pub unsafe extern "C" fn cmd_top() {
    // Like cmd_ps but sorted by CPU ticks, refreshed every scheduler tick
}

#[no_mangle]
pub unsafe extern "C" fn cmd_ls() {
    // Calls sigma_vfs_readdir() on SHELL.cwd and outputs entries
}

#[no_mangle]
pub unsafe extern "C" fn cmd_cat() {
    // Opens file via sigma_vfs_open(), reads, writes to console
}

#[no_mangle]
pub unsafe extern "C" fn cmd_mkdir() {
    // Calls sigma_vfs_mkdir()
}

#[no_mangle]
pub unsafe extern "C" fn cmd_rm() {
    // Calls sigma_vfs_unlink() with confirmation
}

#[no_mangle]
pub unsafe extern "C" fn cmd_clear() {
    // Writes ANSI clear sequence to kernel console
}

#[no_mangle]
pub unsafe extern "C" fn cmd_echo() {
    // Prints argv[1..] to kernel console
}

#[no_mangle]
pub unsafe extern "C" fn cmd_exit_shell() {
    // Sets SHELL.exit_code and signals the shell REPL to exit
    let s = &mut SHELL;
    s.initialized = false;
}

#[no_mangle]
pub unsafe extern "C" fn cmd_ip() {
    // Queries sigma_net for interface list and IP addresses
}

#[no_mangle]
pub unsafe extern "C" fn cmd_ping() {
    // Sends ICMP echo via sigma_net_icmp_send()
}

#[no_mangle]
pub unsafe extern "C" fn cmd_lsblk() {
    // Enumerates sigma_blk_devices and outputs block device list
}

#[no_mangle]
pub unsafe extern "C" fn cmd_pqc_gen() {
    // Calls sigma_pqc_keygen() to generate Dilithium-5 keypair
}

#[no_mangle]
pub unsafe extern "C" fn cmd_forensic_scan() {
    // Calls sigma_forensic_engine_scan() and streams results to console
}

#[no_mangle]
pub unsafe extern "C" fn cmd_ml_infer() {
    // Loads a .sigma-model and runs inference via sigma_tensor
}

#[no_mangle]
pub unsafe extern "C" fn cmd_ml_train() {
    // Triggers a training run on sigma_tensor with specified dataset
}

#[no_mangle]
pub unsafe extern "C" fn cmd_sigma_deploy() {
    // Deploys a compiled .shard from a sigma.json manifest
}

#[no_mangle]
pub unsafe extern "C" fn cmd_git() {
    // Lightweight kernel-space git status / log (read-only)
}

#[no_mangle]
pub unsafe extern "C" fn cmd_apt() {
    // Bridges to sigma-pkg for Debian-compatible package names
}

#[no_mangle]
pub unsafe extern "C" fn cmd_pacman() {
    // Bridges to sigma-pkg for Arch-compatible package names
}

#[no_mangle]
pub unsafe extern "C" fn cmd_systemctl() {
    // Maps systemctl verbs (start|stop|status|enable|disable) to sigma-svc
}

#[no_mangle]
pub unsafe extern "C" fn cmd_heatmap() {
    // Renders an ASCII heat map of CPU/mem activity to console
}

#[no_mangle]
pub unsafe extern "C" fn cmd_tensor_core() {
    // Reports sigma_tensor core utilisation and active tasks
}

#[no_mangle]
pub unsafe extern "C" fn cmd_shard_rebase() {
    // Calls sigma_shard_manager_rebase() to reconcile shard graph
}

#[no_mangle]
pub unsafe extern "C" fn cmd_data_plot() {
    // Renders an ASCII sparkline/bar chart of a numeric data stream
}

#[no_mangle]
pub unsafe extern "C" fn cmd_graph_plot() {
    // Renders an ASCII line graph via sigma_tui_graph()
}

#[no_mangle]
pub unsafe extern "C" fn cmd_sync_gh() {
    // Calls sigma_net_github_sync() for lightweight git fetch
}

#[no_mangle]
pub unsafe extern "C" fn cmd_law_query() {
    // Queries the sigma_legal_engine for a statutory reference
}

#[no_mangle]
pub unsafe extern "C" fn cmd_bsa_cert() {
    // Generates a BSA compliance certificate via sigma_compliance
}

#[no_mangle]
pub unsafe extern "C" fn cmd_cam_cap() {
    // Captures a frame from sigma_cam_hal
}

#[no_mangle]
pub unsafe extern "C" fn cmd_cam_filt() {
    // Lists/applies camera filter presets
}

#[no_mangle]
pub unsafe extern "C" fn cmd_cam_filters() {
    // Displays all available camera filter effects
}

#[no_mangle]
pub unsafe extern "C" fn cmd_cam_forensic_start() {
    // Begins continuous camera forensic capture to encrypted ring buffer
}

#[no_mangle]
pub unsafe extern "C" fn cmd_cam_forensic_stop() {
    // Stops forensic capture and seals the evidence archive
}

#[no_mangle]
pub unsafe extern "C" fn cmd_cam_events() {
    // Lists timestamped camera events from the forensic ring buffer
}

#[no_mangle]
pub unsafe extern "C" fn cmd_auto_setup() {
    // Runs sigma_autosetup_wizard for first-boot configuration
}

#[no_mangle]
pub unsafe extern "C" fn cmd_personalize() {
    // Applies user persona settings from /etc/sigma/persona.toml
}

#[no_mangle]
pub unsafe extern "C" fn cmd_ncert_sim() {
    // Simulates NCERT exam scenarios via sigma_edu module
}

#[no_mangle]
pub unsafe extern "C" fn cmd_alias_set() {
    // Parses "name=value" and calls shell_alias_set()
}

#[no_mangle]
pub unsafe extern "C" fn cmd_export() {
    // Sets env var via shell_env_set() and marks it for child processes
}

#[no_mangle]
pub unsafe extern "C" fn cmd_env_list() {
    // Iterates SHELL.env and prints "KEY=VALUE" lines to console
    let s = &SHELL;
    let _ = s.env_count;
}

#[no_mangle]
pub unsafe extern "C" fn cmd_checklist_report() {
    // Generates a compliance checklist report via sigma_compliance
}

#[no_mangle]
pub unsafe extern "C" fn cmd_checklist_ls() {
    // Lists all available compliance checklists
}
