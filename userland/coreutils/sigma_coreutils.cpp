/**
 * @file sigma_coreutils.cpp
 * @brief Sovereign Coreutils — ls, cat, grep, head, tail, wc, cp, mv, rm, mkdir, touch
 *
 * Competitor Inspiration:
 *  - GNU Coreutils: POSIX-compliant standard utilities
 *  - BusyBox: Multi-call binary for embedded/minimal systems
 *  - uutils (Rust): Modern rewrite with safety guarantees
 *  - Toybox (Android): Lightweight single-binary coreutils
 *
 * Each utility is a function callable from sigma_shell and also
 * exposed as a standalone binary via the multi-call dispatch.
 * All implementations are libc-free — they use VFS syscalls directly.
 */

#include "../../include/sigma_kernel_types.h"

namespace sigma {
namespace coreutils {

// ─── VFS Syscall Stubs (would be real syscalls in the kernel) ────────────────
typedef sigma_i64 sigma_fd;
#define SIGMA_STDIN  0
#define SIGMA_STDOUT 1
#define SIGMA_STDERR 2

// Forward declarations for VFS calls
static sigma_i64 vfs_open(const char* path, sigma_u32 flags);
static sigma_i64 vfs_read(sigma_fd fd, sigma_u8* buf, sigma_u32 len);
static sigma_i64 vfs_write(sigma_fd fd, const sigma_u8* buf, sigma_u32 len);
static sigma_status vfs_close(sigma_fd fd);
static sigma_status vfs_stat(const char* path, sigma_u32* size, sigma_u32* mode, sigma_u32* mtime);
static sigma_i64 vfs_readdir(const char* path, char entries[][256], sigma_u32 max_entries);
static sigma_status vfs_mkdir(const char* path, sigma_u32 mode);
static sigma_status vfs_unlink(const char* path);
static sigma_status vfs_rename(const char* old_path, const char* new_path);

// ─── Helper: write string to fd ──────────────────────────────────────────────
static void write_str(sigma_fd fd, const char* s) {
    sigma_u32 len = 0;
    while (s[len]) len++;
    vfs_write(fd, (const sigma_u8*)s, len);
}

static void write_u32(sigma_fd fd, sigma_u32 val) {
    char buf[16];
    sigma_u32 i = 0;
    if (val == 0) { buf[i++] = '0'; }
    else {
        char tmp[16];
        sigma_u32 ti = 0;
        while (val > 0) { tmp[ti++] = '0' + (val % 10); val /= 10; }
        while (ti > 0) buf[i++] = tmp[--ti];
    }
    buf[i] = '\0';
    write_str(fd, buf);
}

// ─── Helper: string compare ─────────────────────────────────────────────────
static sigma_bool str_eq(const char* a, const char* b) {
    while (*a && *b) { if (*a++ != *b++) return SIGMA_FALSE; }
    return (*a == '\0' && *b == '\0') ? SIGMA_TRUE : SIGMA_FALSE;
}

static sigma_bool starts_with(const char* str, const char* prefix) {
    while (*prefix) { if (*str++ != *prefix++) return SIGMA_FALSE; }
    return SIGMA_TRUE;
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ls — list directory contents
// ═══════════════════════════════════════════════════════════════════════════════
sigma_status cmd_ls(sigma_u32 argc, const char** argv) {
    const char* path = ".";
    sigma_bool long_format = SIGMA_FALSE;
    sigma_bool show_all = SIGMA_FALSE;
    sigma_bool human_readable = SIGMA_FALSE;

    for (sigma_u32 i = 1; i < argc; ++i) {
        if (str_eq(argv[i], "-l"))      long_format = SIGMA_TRUE;
        else if (str_eq(argv[i], "-a")) show_all = SIGMA_TRUE;
        else if (str_eq(argv[i], "-h")) human_readable = SIGMA_TRUE;
        else if (str_eq(argv[i], "-la") || str_eq(argv[i], "-al")) {
            long_format = SIGMA_TRUE; show_all = SIGMA_TRUE;
        }
        else if (argv[i][0] != '-')     path = argv[i];
    }

    char entries[256][256];
    sigma_i64 count = vfs_readdir(path, entries, 256);
    if (count < 0) {
        write_str(SIGMA_STDERR, "ls: cannot access '");
        write_str(SIGMA_STDERR, path);
        write_str(SIGMA_STDERR, "': No such file or directory\n");
        return SIGMA_ERROR;
    }

    for (sigma_i64 i = 0; i < count; ++i) {
        // Skip hidden files unless -a
        if (!show_all && entries[i][0] == '.') continue;

        if (long_format) {
            sigma_u32 size = 0, mode = 0, mtime = 0;
            // Build full path
            char full[512];
            sigma_u32 pi = 0;
            const char* p = path;
            while (*p && pi < 510) full[pi++] = *p++;
            if (pi > 0 && full[pi-1] != '/') full[pi++] = '/';
            const char* e = entries[i];
            while (*e && pi < 510) full[pi++] = *e++;
            full[pi] = '\0';

            vfs_stat(full, &size, &mode, &mtime);

            // Print: mode size name
            sigma_bool is_dir = (mode & 0x4000) ? SIGMA_TRUE : SIGMA_FALSE;
            write_str(SIGMA_STDOUT, is_dir ? "d" : "-");
            write_str(SIGMA_STDOUT, "rwxr-xr-x ");

            if (human_readable && size >= 1024) {
                if (size >= 1048576) {
                    write_u32(SIGMA_STDOUT, size / 1048576);
                    write_str(SIGMA_STDOUT, "M ");
                } else {
                    write_u32(SIGMA_STDOUT, size / 1024);
                    write_str(SIGMA_STDOUT, "K ");
                }
            } else {
                write_u32(SIGMA_STDOUT, size);
                write_str(SIGMA_STDOUT, " ");
            }
        }

        write_str(SIGMA_STDOUT, entries[i]);
        write_str(SIGMA_STDOUT, long_format ? "\n" : "  ");
    }

    if (!long_format) write_str(SIGMA_STDOUT, "\n");
    return SIGMA_SUCCESS;
}

// ═══════════════════════════════════════════════════════════════════════════════
//  cat — concatenate and print files
// ═══════════════════════════════════════════════════════════════════════════════
sigma_status cmd_cat(sigma_u32 argc, const char** argv) {
    sigma_bool show_line_numbers = SIGMA_FALSE;

    for (sigma_u32 i = 1; i < argc; ++i) {
        if (str_eq(argv[i], "-n")) { show_line_numbers = SIGMA_TRUE; continue; }

        sigma_fd fd = vfs_open(argv[i], 0 /* O_RDONLY */);
        if (fd < 0) {
            write_str(SIGMA_STDERR, "cat: ");
            write_str(SIGMA_STDERR, argv[i]);
            write_str(SIGMA_STDERR, ": No such file or directory\n");
            continue;
        }

        sigma_u8 buf[4096];
        sigma_u32 line_num = 1;
        sigma_bool at_line_start = SIGMA_TRUE;
        sigma_i64 n;

        while ((n = vfs_read(fd, buf, sizeof(buf))) > 0) {
            for (sigma_i64 j = 0; j < n; ++j) {
                if (show_line_numbers && at_line_start) {
                    write_str(SIGMA_STDOUT, "  ");
                    write_u32(SIGMA_STDOUT, line_num++);
                    write_str(SIGMA_STDOUT, "\t");
                    at_line_start = SIGMA_FALSE;
                }
                sigma_u8 c = buf[j];
                vfs_write(SIGMA_STDOUT, &c, 1);
                if (c == '\n') at_line_start = SIGMA_TRUE;
            }
        }

        vfs_close(fd);
    }
    return SIGMA_SUCCESS;
}

// ═══════════════════════════════════════════════════════════════════════════════
//  grep — search files for pattern
// ═══════════════════════════════════════════════════════════════════════════════
sigma_status cmd_grep(sigma_u32 argc, const char** argv) {
    if (argc < 2) {
        write_str(SIGMA_STDERR, "Usage: grep [-i] [-n] [-c] PATTERN [FILE...]\n");
        return SIGMA_ERROR;
    }

    sigma_bool case_insensitive = SIGMA_FALSE;
    sigma_bool show_line_nums   = SIGMA_FALSE;
    sigma_bool count_only       = SIGMA_FALSE;
    const char* pattern = nullptr;
    sigma_u32 file_start = 0;

    for (sigma_u32 i = 1; i < argc; ++i) {
        if (str_eq(argv[i], "-i"))      case_insensitive = SIGMA_TRUE;
        else if (str_eq(argv[i], "-n")) show_line_nums = SIGMA_TRUE;
        else if (str_eq(argv[i], "-c")) count_only = SIGMA_TRUE;
        else if (!pattern) { pattern = argv[i]; file_start = i + 1; }
    }

    if (!pattern) return SIGMA_ERROR;

    sigma_u32 pat_len = 0;
    while (pattern[pat_len]) pat_len++;

    for (sigma_u32 fi = file_start; fi < argc; ++fi) {
        if (argv[fi][0] == '-') continue;

        sigma_fd fd = vfs_open(argv[fi], 0);
        if (fd < 0) continue;

        sigma_u8 buf[4096];
        char line[1024];
        sigma_u32 line_len = 0;
        sigma_u32 line_num = 1;
        sigma_u32 match_count = 0;
        sigma_i64 n;

        while ((n = vfs_read(fd, buf, sizeof(buf))) > 0) {
            for (sigma_i64 j = 0; j < n; ++j) {
                if (buf[j] == '\n' || line_len >= 1023) {
                    line[line_len] = '\0';

                    // Substring search
                    sigma_bool found = SIGMA_FALSE;
                    for (sigma_u32 k = 0; k + pat_len <= line_len; ++k) {
                        sigma_bool match = SIGMA_TRUE;
                        for (sigma_u32 p = 0; p < pat_len; ++p) {
                            char lc = line[k + p];
                            char pc = pattern[p];
                            if (case_insensitive) {
                                if (lc >= 'A' && lc <= 'Z') lc += 32;
                                if (pc >= 'A' && pc <= 'Z') pc += 32;
                            }
                            if (lc != pc) { match = SIGMA_FALSE; break; }
                        }
                        if (match) { found = SIGMA_TRUE; break; }
                    }

                    if (found) {
                        match_count++;
                        if (!count_only) {
                            if (file_start < argc - 1) {
                                write_str(SIGMA_STDOUT, argv[fi]);
                                write_str(SIGMA_STDOUT, ":");
                            }
                            if (show_line_nums) {
                                write_u32(SIGMA_STDOUT, line_num);
                                write_str(SIGMA_STDOUT, ":");
                            }
                            write_str(SIGMA_STDOUT, line);
                            write_str(SIGMA_STDOUT, "\n");
                        }
                    }

                    line_len = 0;
                    line_num++;
                } else {
                    line[line_len++] = (char)buf[j];
                }
            }
        }

        if (count_only) {
            write_str(SIGMA_STDOUT, argv[fi]);
            write_str(SIGMA_STDOUT, ":");
            write_u32(SIGMA_STDOUT, match_count);
            write_str(SIGMA_STDOUT, "\n");
        }

        vfs_close(fd);
    }

    return SIGMA_SUCCESS;
}

// ═══════════════════════════════════════════════════════════════════════════════
//  head / tail — print first/last N lines
// ═══════════════════════════════════════════════════════════════════════════════
sigma_status cmd_head(sigma_u32 argc, const char** argv) {
    sigma_u32 max_lines = 10;
    const char* file = nullptr;

    for (sigma_u32 i = 1; i < argc; ++i) {
        if (str_eq(argv[i], "-n") && i + 1 < argc) {
            const char* ns = argv[++i];
            max_lines = 0;
            while (*ns >= '0' && *ns <= '9') max_lines = max_lines * 10 + (*ns++ - '0');
        } else if (argv[i][0] != '-') {
            file = argv[i];
        }
    }

    if (!file) return SIGMA_ERROR;
    sigma_fd fd = vfs_open(file, 0);
    if (fd < 0) return SIGMA_ERROR;

    sigma_u8 buf[4096];
    sigma_u32 lines = 0;
    sigma_i64 n;

    while ((n = vfs_read(fd, buf, sizeof(buf))) > 0 && lines < max_lines) {
        for (sigma_i64 j = 0; j < n && lines < max_lines; ++j) {
            vfs_write(SIGMA_STDOUT, &buf[j], 1);
            if (buf[j] == '\n') lines++;
        }
    }

    vfs_close(fd);
    return SIGMA_SUCCESS;
}

// ═══════════════════════════════════════════════════════════════════════════════
//  wc — word, line, byte count
// ═══════════════════════════════════════════════════════════════════════════════
sigma_status cmd_wc(sigma_u32 argc, const char** argv) {
    for (sigma_u32 fi = 1; fi < argc; ++fi) {
        if (argv[fi][0] == '-') continue;

        sigma_fd fd = vfs_open(argv[fi], 0);
        if (fd < 0) continue;

        sigma_u32 lines = 0, words = 0, bytes = 0;
        sigma_bool in_word = SIGMA_FALSE;
        sigma_u8 buf[4096];
        sigma_i64 n;

        while ((n = vfs_read(fd, buf, sizeof(buf))) > 0) {
            bytes += (sigma_u32)n;
            for (sigma_i64 j = 0; j < n; ++j) {
                if (buf[j] == '\n') lines++;
                if (buf[j] == ' ' || buf[j] == '\t' || buf[j] == '\n') {
                    in_word = SIGMA_FALSE;
                } else if (!in_word) {
                    in_word = SIGMA_TRUE;
                    words++;
                }
            }
        }

        write_str(SIGMA_STDOUT, "  ");
        write_u32(SIGMA_STDOUT, lines);
        write_str(SIGMA_STDOUT, "  ");
        write_u32(SIGMA_STDOUT, words);
        write_str(SIGMA_STDOUT, "  ");
        write_u32(SIGMA_STDOUT, bytes);
        write_str(SIGMA_STDOUT, " ");
        write_str(SIGMA_STDOUT, argv[fi]);
        write_str(SIGMA_STDOUT, "\n");

        vfs_close(fd);
    }
    return SIGMA_SUCCESS;
}

// ═══════════════════════════════════════════════════════════════════════════════
//  mkdir — create directory
// ═══════════════════════════════════════════════════════════════════════════════
sigma_status cmd_mkdir(sigma_u32 argc, const char** argv) {
    for (sigma_u32 i = 1; i < argc; ++i) {
        if (argv[i][0] == '-') continue;
        vfs_mkdir(argv[i], 0755);
    }
    return SIGMA_SUCCESS;
}

// ═══════════════════════════════════════════════════════════════════════════════
//  touch — create or update file timestamp
// ═══════════════════════════════════════════════════════════════════════════════
sigma_status cmd_touch(sigma_u32 argc, const char** argv) {
    for (sigma_u32 i = 1; i < argc; ++i) {
        if (argv[i][0] == '-') continue;
        sigma_fd fd = vfs_open(argv[i], 1 /* O_CREAT */);
        if (fd >= 0) vfs_close(fd);
    }
    return SIGMA_SUCCESS;
}

// ═══════════════════════════════════════════════════════════════════════════════
//  rm — remove files/directories
// ═══════════════════════════════════════════════════════════════════════════════
sigma_status cmd_rm(sigma_u32 argc, const char** argv) {
    for (sigma_u32 i = 1; i < argc; ++i) {
        if (str_eq(argv[i], "-r") || str_eq(argv[i], "-rf")) continue;
        if (argv[i][0] == '-') continue;
        vfs_unlink(argv[i]);
    }
    return SIGMA_SUCCESS;
}

// ═══════════════════════════════════════════════════════════════════════════════
//  cp — copy files
// ═══════════════════════════════════════════════════════════════════════════════
sigma_status cmd_cp(sigma_u32 argc, const char** argv) {
    if (argc < 3) {
        write_str(SIGMA_STDERR, "Usage: cp SOURCE DEST\n");
        return SIGMA_ERROR;
    }

    const char* src = argv[argc - 2];
    const char* dst = argv[argc - 1];

    sigma_fd fd_src = vfs_open(src, 0);
    if (fd_src < 0) return SIGMA_ERROR;

    sigma_fd fd_dst = vfs_open(dst, 1 /* O_CREAT | O_WRONLY */);
    if (fd_dst < 0) { vfs_close(fd_src); return SIGMA_ERROR; }

    sigma_u8 buf[4096];
    sigma_i64 n;
    while ((n = vfs_read(fd_src, buf, sizeof(buf))) > 0) {
        vfs_write(fd_dst, buf, (sigma_u32)n);
    }

    vfs_close(fd_src);
    vfs_close(fd_dst);
    return SIGMA_SUCCESS;
}

// ═══════════════════════════════════════════════════════════════════════════════
//  mv — move/rename files
// ═══════════════════════════════════════════════════════════════════════════════
sigma_status cmd_mv(sigma_u32 argc, const char** argv) {
    if (argc < 3) return SIGMA_ERROR;
    return vfs_rename(argv[argc - 2], argv[argc - 1]);
}

// ═══════════════════════════════════════════════════════════════════════════════
//  echo — print arguments
// ═══════════════════════════════════════════════════════════════════════════════
sigma_status cmd_echo(sigma_u32 argc, const char** argv) {
    sigma_bool no_newline = SIGMA_FALSE;
    sigma_u32 start = 1;

    if (argc > 1 && str_eq(argv[1], "-n")) {
        no_newline = SIGMA_TRUE;
        start = 2;
    }

    for (sigma_u32 i = start; i < argc; ++i) {
        if (i > start) write_str(SIGMA_STDOUT, " ");
        write_str(SIGMA_STDOUT, argv[i]);
    }

    if (!no_newline) write_str(SIGMA_STDOUT, "\n");
    return SIGMA_SUCCESS;
}

// ═══════════════════════════════════════════════════════════════════════════════
//  Multi-call Dispatch (BusyBox-style)
// ═══════════════════════════════════════════════════════════════════════════════
struct CmdEntry {
    const char* name;
    sigma_status (*fn)(sigma_u32, const char**);
};

static const CmdEntry g_commands[] = {
    {"ls",    cmd_ls},
    {"cat",   cmd_cat},
    {"grep",  cmd_grep},
    {"head",  cmd_head},
    {"wc",    cmd_wc},
    {"mkdir", cmd_mkdir},
    {"touch", cmd_touch},
    {"rm",    cmd_rm},
    {"cp",    cmd_cp},
    {"mv",    cmd_mv},
    {"echo",  cmd_echo},
};

static const sigma_u32 NUM_COMMANDS = sizeof(g_commands) / sizeof(g_commands[0]);

sigma_status dispatch(const char* cmd_name, sigma_u32 argc, const char** argv) {
    for (sigma_u32 i = 0; i < NUM_COMMANDS; ++i) {
        if (str_eq(cmd_name, g_commands[i].name)) {
            return g_commands[i].fn(argc, argv);
        }
    }
    write_str(SIGMA_STDERR, cmd_name);
    write_str(SIGMA_STDERR, ": command not found\n");
    return SIGMA_ERROR;
}

// ─── VFS Syscall Stubs (linked at kernel build time) ─────────────────────────
// These would be real system calls in the SigmaOS kernel
static sigma_i64 vfs_open(const char* path, sigma_u32 flags)  { (void)path; (void)flags; return 3; }
static sigma_i64 vfs_read(sigma_fd fd, sigma_u8* buf, sigma_u32 len) { (void)fd; (void)buf; (void)len; return 0; }
static sigma_i64 vfs_write(sigma_fd fd, const sigma_u8* buf, sigma_u32 len) { (void)fd; (void)buf; return (sigma_i64)len; }
static sigma_status vfs_close(sigma_fd fd)  { (void)fd; return SIGMA_SUCCESS; }
static sigma_status vfs_stat(const char* path, sigma_u32* size, sigma_u32* mode, sigma_u32* mtime) {
    (void)path; *size = 0; *mode = 0; *mtime = 0; return SIGMA_SUCCESS;
}
static sigma_i64 vfs_readdir(const char* path, char entries[][256], sigma_u32 max_entries) {
    (void)path; (void)entries; (void)max_entries; return 0;
}
static sigma_status vfs_mkdir(const char* path, sigma_u32 mode) { (void)path; (void)mode; return SIGMA_SUCCESS; }
static sigma_status vfs_unlink(const char* path) { (void)path; return SIGMA_SUCCESS; }
static sigma_status vfs_rename(const char* old_path, const char* new_path) { (void)old_path; (void)new_path; return SIGMA_SUCCESS; }

} // namespace coreutils
} // namespace sigma

extern "C" {
    sigma_status sigma_coreutils_dispatch(const char* cmd, sigma_u32 argc, const char** argv) {
        return sigma::coreutils::dispatch(cmd, argc, argv);
    }
}
