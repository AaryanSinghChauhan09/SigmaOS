/*
 * Σ SigmaOS Zenith — sigma-sh: The Sovereign Shell
 * Absorbs: BusyBox philosophy (small, self-contained), Dash shell architecture
 * Zero-Dependency: No libc, no stdlib, no unistd.h, no predefined headers.
 */

/* ─────────────── Sovereign Types ─────────────── */
typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

/* ─────────────── External Kernel Services (no headers needed) ─────────────── */
/* These are resolved by the linker from their respective .cpp files */
extern "C" void sigma_vga_puts(const char* str);
extern "C" void sigma_vga_putchar(char c);
extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" void sigma_vga_set_color(int fg, int bg);
extern "C" u32  sigma_fat32_read(const char* name, u8* buf, u32 max_len);

/* ─────────────── Sovereign Utility Declarations ─────────────── */
extern "C" int sigma_pwd_main(int argc, char** argv);
extern "C" int sigma_uname_main(int argc, char** argv);
extern "C" int sigma_ps_main(int argc, char** argv);
extern "C" int sigma_top_main(int argc, char** argv);
extern "C" int sigma_kill_main(int argc, char** argv);
extern "C" int sigma_cp_main(int argc, char** argv);
extern "C" int sigma_mv_main(int argc, char** argv);
extern "C" int sigma_rm_main(int argc, char** argv);
extern "C" int sigma_chmod_main(int argc, char** argv);
extern "C" int sigma_df_main(int argc, char** argv);
extern "C" int sigma_grep_main(int argc, char** argv);
extern "C" int sigma_dmesg_main(int argc, char** argv);
extern "C" int sigma_wc_main(int argc, char** argv);
extern "C" int sigma_head_main(int argc, char** argv);
extern "C" int sigma_hexdump_main(int argc, char** argv);
extern "C" int sigma_ifconfig_main(int argc, char** argv);
extern "C" int sigma_ping_main(int argc, char** argv);
extern "C" int sigma_mount_main(int argc, char** argv);
extern "C" int sigma_lspci_main(int argc, char** argv);
extern "C" int sigma_zfs_main(int argc, char** argv);
extern "C" int sigma_cgroup_main(int argc, char** argv);
extern "C" int sigma_overlayfs_main(int argc, char** argv);
extern "C" int sigma_systemctl_main(int argc, char** argv);

/* ─────────────── Sovereign String Utilities ─────────────── */
static u32 sh_strlen(const char* s) {
    u32 n = 0; while (s[n]) n++; return n;
}

static bool sh_streq(const char* a, const char* b) {
    while (*a && *b && *a == *b) { a++; b++; }
    return *a == *b;
}

static void sh_strncpy(char* dst, const char* src, u32 n) {
    u32 i;
    for (i = 0; i < n - 1 && src[i]; i++) dst[i] = src[i];
    dst[i] = '\0';
}

/* ─────────────── Command Buffer ─────────────── */
#define CMD_BUF_SIZE  256
#define MAX_ARGS      16
#define SIGMA_SH_VER  "1.0.0"

static char cmd_buf[CMD_BUF_SIZE];
static u32  cmd_len;

/* ─────────────── Command History (Arch-inspired minimal history) ─────────────── */
#define HISTORY_SIZE 10
static char history[HISTORY_SIZE][CMD_BUF_SIZE];
static u32  history_count = 0;
static u32  history_cursor = 0;

static void history_push(const char* cmd) {
    if (sh_strlen(cmd) == 0) return;
    u32 idx = history_count % HISTORY_SIZE;
    sh_strncpy(history[idx], cmd, CMD_BUF_SIZE);
    history_count++;
    history_cursor = history_count;
}

/* ─────────────── Argument Parsing ─────────────── */
static char  arg_store[CMD_BUF_SIZE];
static char* argv[MAX_ARGS];
static u32   argc;

static void parse_args(const char* input) {
    argc = 0;
    u32 i = 0, j = 0;
    bool in_arg = false;

    while (input[i] && argc < MAX_ARGS) {
        if (input[i] == ' ' || input[i] == '\t') {
            if (in_arg) {
                arg_store[j++] = '\0';
                in_arg = false;
            }
        } else {
            if (!in_arg) {
                argv[argc++] = arg_store + j;
                in_arg = true;
            }
            arg_store[j++] = input[i];
        }
        i++;
    }
    if (in_arg) arg_store[j] = '\0';
}

/* ─────────────── Built-in Commands ─────────────── */

/* echo: write arguments to terminal */
static void builtin_echo() {
    for (u32 i = 1; i < argc; i++) {
        sigma_vga_puts(argv[i]);
        if (i + 1 < argc) sigma_vga_putchar(' ');
    }
    sigma_vga_putchar('\n');
}

/* cat: read and print a file from SigmaFAT32 */
static void builtin_cat() {
    if (argc < 2) {
        sigma_vga_puts("cat: missing filename\n");
        return;
    }
    static u8 file_buf[65536]; /* 64 KB max */
    u32 bytes = sigma_fat32_read(argv[1], file_buf, sizeof(file_buf) - 1);
    if (bytes == 0) {
        sigma_vga_puts("cat: file not found: ");
        sigma_vga_puts(argv[1]);
        sigma_vga_putchar('\n');
        return;
    }
    file_buf[bytes] = '\0';
    sigma_vga_puts((const char*)file_buf);
    sigma_vga_putchar('\n');
}

/* ls: list files (stubbed to known filesystem table) */
/* In real usage this would walk the FAT32 root cluster directory */
static void builtin_ls() {
    sigma_vga_puts("SigmaFAT Root Directory:\n");
    sigma_vga_puts("  [DIR]  .\n");
    sigma_vga_puts("  [DIR]  ..\n");
    sigma_vga_puts("  [DIR]  bin\n");
    sigma_vga_puts("  [DIR]  etc\n");
    sigma_vga_puts("  [DIR]  usr\n");
    sigma_vga_puts("  [DIR]  var\n");
    sigma_vga_puts("  [FILE] kernel.elf\n");
    sigma_vga_puts("  [FILE] sigma-sh\n");
    sigma_vga_puts("  [FILE] sigma.conf\n");
}

/* clear: clear the VGA screen */
static void builtin_clear() {
    /* VGA clear by printing newlines */
    for (u32 i = 0; i < 25; i++) sigma_vga_putchar('\n');
}

/* help: list built-in commands */
static void builtin_help() {
    sigma_vga_printf("sigma-sh v%s — The Sovereign Shell\n", SIGMA_SH_VER);
    sigma_vga_puts("Built-in Commands:\n");
    sigma_vga_puts("  echo  [args...]   Print text to terminal\n");
    sigma_vga_puts("  cat   [file]      Print file contents\n");
    sigma_vga_puts("  ls                List root directory\n");
    sigma_vga_puts("  clear             Clear the screen\n");
    sigma_vga_puts("  history           Show command history\n");
    sigma_vga_puts("  help              Show this help text\n");
    sigma_vga_puts("  halt              Halt the system\n");
    sigma_vga_puts("Sovereign Utilities:\n");
    sigma_vga_puts("  pwd               Print working directory\n");
    sigma_vga_puts("  uname             Display system info\n");
    sigma_vga_puts("  ps                Show process status\n");
    sigma_vga_puts("  top               System monitor\n");
    sigma_vga_puts("  kill  [pid]       Terminate process\n");
    sigma_vga_puts("  cp    [src] [dst] Copy file\n");
    sigma_vga_puts("  mv    [src] [dst] Move file\n");
    sigma_vga_puts("  rm    [file]      Remove file\n");
    sigma_vga_puts("  chmod [mode] [f]  Change file permissions\n");
    sigma_vga_puts("  df                Display disk free space\n");
    sigma_vga_puts("  grep  [pat] [f]   Search for pattern\n");
    sigma_vga_puts("  dmesg             Print kernel ring buffer\n");
    sigma_vga_puts("  wc    [file]      Count lines, words, bytes\n");
    sigma_vga_puts("  head  [-n N] [f]  Display first lines of a file\n");
    sigma_vga_puts("  hexdump [file]    Display file in hexadecimal\n");
    sigma_vga_puts("  ifconfig          Display network interfaces\n");
    sigma_vga_puts("  ping  [ip]        Ping a network host\n");
    sigma_vga_puts("  mount -t [fs] [l] Mount partition/LBA\n");
    sigma_vga_puts("  lspci             List all PCI devices\n");
    sigma_vga_puts("  zfs   [args...]   Manage Copy-on-Write ZFS pools/datasets\n");
    sigma_vga_puts("  cgroup [args...]  Manage silicon resource and weights governance\n");
    sigma_vga_puts("  overlayfs [args...] Mount live overlay layers and directory union\n");
    sigma_vga_puts("  systemctl [args...] Start, stop, or check active background services\n");
}

/* history: print command history */
static void builtin_history() {
    u32 start = (history_count > HISTORY_SIZE) ? history_count - HISTORY_SIZE : 0;
    for (u32 i = start; i < history_count; i++) {
        u32 idx = i % HISTORY_SIZE;
        sigma_vga_printf("  %u  %s\n", i + 1, history[idx]);
    }
}

/* halt: system halt */
static void builtin_halt() {
    sigma_vga_puts("Halting the Sovereign System...\n");
    __asm__ volatile ("cli; hlt");
}

/* ─────────────── Command Dispatch ─────────────── */
static void dispatch_command(const char* input) {
    if (sh_strlen(input) == 0) return;

    parse_args(input);
    if (argc == 0) return;

    history_push(input);

    if      (sh_streq(argv[0], "echo"))    builtin_echo();
    else if (sh_streq(argv[0], "cat"))     builtin_cat();
    else if (sh_streq(argv[0], "ls"))      builtin_ls();
    else if (sh_streq(argv[0], "clear"))   builtin_clear();
    else if (sh_streq(argv[0], "help"))    builtin_help();
    else if (sh_streq(argv[0], "history")) builtin_history();
    else if (sh_streq(argv[0], "halt"))    builtin_halt();
    else if (sh_streq(argv[0], "pwd"))     sigma_pwd_main(argc, argv);
    else if (sh_streq(argv[0], "uname"))   sigma_uname_main(argc, argv);
    else if (sh_streq(argv[0], "ps"))      sigma_ps_main(argc, argv);
    else if (sh_streq(argv[0], "top"))     sigma_top_main(argc, argv);
    else if (sh_streq(argv[0], "kill"))    sigma_kill_main(argc, argv);
    else if (sh_streq(argv[0], "cp"))      sigma_cp_main(argc, argv);
    else if (sh_streq(argv[0], "mv"))      sigma_mv_main(argc, argv);
    else if (sh_streq(argv[0], "rm"))      sigma_rm_main(argc, argv);
    else if (sh_streq(argv[0], "chmod"))   sigma_chmod_main(argc, argv);
    else if (sh_streq(argv[0], "df"))      sigma_df_main(argc, argv);
    else if (sh_streq(argv[0], "grep"))    sigma_grep_main(argc, argv);
    else if (sh_streq(argv[0], "dmesg"))   sigma_dmesg_main(argc, argv);
    else if (sh_streq(argv[0], "wc"))      sigma_wc_main(argc, argv);
    else if (sh_streq(argv[0], "head"))    sigma_head_main(argc, argv);
    else if (sh_streq(argv[0], "hexdump")) sigma_hexdump_main(argc, argv);
    else if (sh_streq(argv[0], "ifconfig")) sigma_ifconfig_main(argc, argv);
    else if (sh_streq(argv[0], "ping"))    sigma_ping_main(argc, argv);
    else if (sh_streq(argv[0], "mount"))   sigma_mount_main(argc, argv);
    else if (sh_streq(argv[0], "lspci"))   sigma_lspci_main(argc, argv);
    else if (sh_streq(argv[0], "zfs"))     sigma_zfs_main(argc, argv);
    else if (sh_streq(argv[0], "cgroup"))  sigma_cgroup_main(argc, argv);
    else if (sh_streq(argv[0], "overlayfs")) sigma_overlayfs_main(argc, argv);
    else if (sh_streq(argv[0], "systemctl")) sigma_systemctl_main(argc, argv);
    else {
        sigma_vga_puts("sigma-sh: command not found: ");
        sigma_vga_puts(argv[0]);
        sigma_vga_putchar('\n');
    }
}

/* ─────────────── Keyboard Read Stub ─────────────── */
/* In full implementation, this polls PS/2 port 0x60 */
static inline u8 sigma_kbd_read_scancode() {
    u8 scancode;
    __asm__ volatile ("inb $0x60, %0" : "=a"(scancode));
    return scancode;
}

/* ASCII scan code table (US QWERTY, partial) */
static const char scancode_to_ascii[58] = {
    0, 0, '1','2','3','4','5','6','7','8','9','0','-','=','\b',
    '\t','q','w','e','r','t','y','u','i','o','p','[',']','\n',
    0,'a','s','d','f','g','h','j','k','l',';','\'','`',
    0,'\\','z','x','c','v','b','n','m',',','.','/',
    0,'*',0,' '
};

/* ─────────────── API: Shell Entry Point ─────────────── */
extern "C" void sigma_sh_run() {
    sigma_vga_puts("\n");
    sigma_vga_printf("  Σ sigma-sh v%s\n", SIGMA_SH_VER);
    sigma_vga_puts("  Type 'help' for a list of commands.\n\n");

    cmd_len = 0;
    bool shift_held = false;

    while (true) {
        sigma_vga_puts("[sigma-sh]# ");
        cmd_len = 0;

        /* Input loop — polls keyboard until Enter */
        while (true) {
            /* Wait for key press */
            while (!(__builtin_ia32_readeflags_u64() & (1 << 9)));
            /* In a real system, we'd hook the keyboard interrupt handler */

            u8 sc = sigma_kbd_read_scancode();
            if (sc & 0x80) continue; /* Key release, ignore */

            char ch = 0;
            if (sc == 0x2A || sc == 0x36) { shift_held = true; continue; }
            if (sc < 58) ch = scancode_to_ascii[sc];
            if (!ch) continue;

            if (ch == '\n') {
                sigma_vga_putchar('\n');
                cmd_buf[cmd_len] = '\0';
                dispatch_command(cmd_buf);
                cmd_len = 0;
                break;
            }

            if (ch == '\b') {
                if (cmd_len > 0) {
                    cmd_len--;
                    sigma_vga_putchar('\b');
                }
                continue;
            }

            if (cmd_len < CMD_BUF_SIZE - 1) {
                if (shift_held && ch >= 'a' && ch <= 'z') ch -= 32;
                cmd_buf[cmd_len++] = ch;
                sigma_vga_putchar(ch);
            }

            shift_held = false;
        }
    }
}
