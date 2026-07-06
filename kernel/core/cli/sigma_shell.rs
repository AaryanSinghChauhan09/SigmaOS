#![no_std]
#![allow(dead_code)]

/// SigmaOS Native CLI Shell
/// A zero-allocation system shell implementing basic coreutils logic.

const MAX_CMD_LEN: usize = 256;
const MAX_ARGS: usize = 16;
const MAX_ARG_LEN: usize = 64;

pub struct SigmaShell {
    cwd: [u8; MAX_CMD_LEN],
    cwd_len: usize,
}

impl SigmaShell {
    pub const fn new() -> Self {
        Self {
            cwd: [0; MAX_CMD_LEN],
            cwd_len: 1, // root "/"
        }
    }

    /// Primary command dispatch loop for the shell
    pub fn execute_command(&mut self, cmd_line: &[u8]) -> i32 {
        if cmd_line.is_empty() { return 0; }

        let mut args: [[u8; MAX_ARG_LEN]; MAX_ARGS] = [[0; MAX_ARG_LEN]; MAX_ARGS];
        let mut arg_lens: [usize; MAX_ARGS] = [0; MAX_ARGS];
        let mut arg_count = 0;

        let mut i = 0;
        while i < cmd_line.len() && arg_count < MAX_ARGS {
            // Skip whitespace
            while i < cmd_line.len() && (cmd_line[i] == b' ' || cmd_line[i] == b'\t') {
                i += 1;
            }
            if i >= cmd_line.len() { break; }

            // Parse argument
            let mut arg_len = 0;
            while i < cmd_line.len() && cmd_line[i] != b' ' && cmd_line[i] != b'\t' && arg_len < MAX_ARG_LEN {
                args[arg_count][arg_len] = cmd_line[i];
                arg_len += 1;
                i += 1;
            }
            arg_lens[arg_count] = arg_len;
            arg_count += 1;
        }

        if arg_count == 0 { return 0; }

        let cmd = &args[0][0..arg_lens[0]];

        if Self::match_cmd(cmd, b"ls") {
            self.cmd_ls(&args, &arg_lens, arg_count)
        } else if Self::match_cmd(cmd, b"cat") {
            self.cmd_cat(&args, &arg_lens, arg_count)
        } else if Self::match_cmd(cmd, b"grep") {
            self.cmd_grep(&args, &arg_lens, arg_count)
        } else if Self::match_cmd(cmd, b"top") {
            self.cmd_top()
        } else {
            -1 // Command not found
        }
    }

    fn match_cmd(arg: &[u8], expected: &[u8]) -> bool {
        if arg.len() != expected.len() { return false; }
        for i in 0..arg.len() {
            if arg[i] != expected[i] { return false; }
        }
        true
    }

    fn cmd_ls(&self, _args: &[[u8; MAX_ARG_LEN]; MAX_ARGS], _arg_lens: &[usize; MAX_ARGS], _arg_count: usize) -> i32 {
        // Stub: Would interface with sigma_vfs to read directory entries
        0
    }

    fn cmd_cat(&self, _args: &[[u8; MAX_ARG_LEN]; MAX_ARGS], _arg_lens: &[usize; MAX_ARGS], _arg_count: usize) -> i32 {
        // Stub: Would interface with sigma_vfs to read and print file
        0
    }

    fn cmd_grep(&self, _args: &[[u8; MAX_ARG_LEN]; MAX_ARGS], _arg_lens: &[usize; MAX_ARGS], _arg_count: usize) -> i32 {
        // Stub: Would scan a target file for a substring match
        0
    }

    fn cmd_top(&self) -> i32 {
        // Stub: Would display system stats (cpu, mem) and IDS anomaly scores
        0
    }
}

static mut G_SIGMA_SHELL: SigmaShell = SigmaShell::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_shell_exec(cmd_ptr: *const u8, len: usize) -> i32 {
    if cmd_ptr.is_null() || len == 0 { return -1; }
    let cmd_slice = core::slice::from_raw_parts(cmd_ptr, len);
    G_SIGMA_SHELL.execute_command(cmd_slice)
}
