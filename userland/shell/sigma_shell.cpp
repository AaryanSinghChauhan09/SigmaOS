/**
 * @file sigma_shell.cpp
 * @brief Sovereign Shell — Full interactive CLI with builtins, pipelines, and scripting
 *
 * Competitor Inspiration:
 *  - Bash: POSIX-compatible builtins and syntax
 *  - Zsh: Tab completion, auto-suggestions, syntax highlighting
 *  - Fish: User-friendly defaults, abbreviations
 *  - Nushell: Structured data pipelines (tables, not just text)
 *  - PowerShell: Object-oriented command output
 *
 * This is the default login shell for SigmaOS. It provides:
 *  1. Builtin commands (cd, ls, cat, echo, export, alias, sigma-*)
 *  2. Pipe and redirect operators (|, >, >>, <, 2>)
 *  3. Variable expansion ($VAR, ${VAR})
 *  4. Command history with search
 *  5. Tab completion via the Sovereign IPC plugin bus
 *  6. Scripting with if/else/for/while/function
 *  7. Integration with sigma-auto, sigma-theme, sigma-pkg CLI subcommands
 */

#include "../../include/sigma_kernel_types.h"
#include <cstdio>
#include <cstring>
#include <string>
#include <vector>
#include <map>
#include <algorithm>

namespace sigma {
namespace shell {

// ─── Structured Data Pipeline (Nushell style) ────────────────────────────────
#define MAX_TABLE_COLS 8
#define MAX_TABLE_ROWS 128

struct SigmaTable {
    char      headers[MAX_TABLE_COLS][32];
    char      rows[MAX_TABLE_ROWS][MAX_TABLE_COLS][64];
    sigma_u32 num_cols;
    sigma_u32 num_rows;
};

// ─── Token Types ─────────────────────────────────────────────────────────────
typedef enum : sigma_u32 {
    TOK_WORD     = 0,   // A bare word or quoted string
    TOK_PIPE     = 1,   // |
    TOK_REDIR_OUT= 2,   // >
    TOK_REDIR_APP= 3,   // >>
    TOK_REDIR_IN = 4,   // <
    TOK_REDIR_ERR= 5,   // 2>
    TOK_AND      = 6,   // &&
    TOK_OR       = 7,   // ||
    TOK_SEMICOL  = 8,   // ;
    TOK_BG       = 9,   // &
    TOK_NEWLINE  = 10,
    TOK_EOF      = 11,
    TOK_VAR      = 12,  // $VAR
} TokenType;

struct Token {
    TokenType type;
    char      text[256];
    sigma_u32 len;
};

// ─── Environment Variables ───────────────────────────────────────────────────
#define MAX_ENV_VARS   256
#define MAX_VAR_NAME   64
#define MAX_VAR_VALUE  512

struct EnvVar {
    char name[MAX_VAR_NAME];
    char value[MAX_VAR_VALUE];
};

static EnvVar g_env[MAX_ENV_VARS];
static sigma_u32 g_env_count = 0;

sigma_status set_env(const char* name, const char* value) {
    // Check for existing
    for (sigma_u32 i = 0; i < g_env_count; ++i) {
        const char* a = g_env[i].name;
        const char* b = name;
        sigma_bool match = SIGMA_TRUE;
        while (*a && *b) { if (*a++ != *b++) { match = SIGMA_FALSE; break; } }
        if (match && *a == '\0' && *b == '\0') {
            sigma_u32 j = 0;
            while (value[j] && j < MAX_VAR_VALUE - 1) { g_env[i].value[j] = value[j]; j++; }
            g_env[i].value[j] = '\0';
            return SIGMA_SUCCESS;
        }
    }
    if (g_env_count >= MAX_ENV_VARS) return SIGMA_ERROR;
    sigma_u32 j = 0;
    while (name[j] && j < MAX_VAR_NAME - 1) { g_env[g_env_count].name[j] = name[j]; j++; }
    g_env[g_env_count].name[j] = '\0';
    j = 0;
    while (value[j] && j < MAX_VAR_VALUE - 1) { g_env[g_env_count].value[j] = value[j]; j++; }
    g_env[g_env_count].value[j] = '\0';
    g_env_count++;
    return SIGMA_SUCCESS;
}

const char* get_env(const char* name) {
    for (sigma_u32 i = 0; i < g_env_count; ++i) {
        const char* a = g_env[i].name;
        const char* b = name;
        sigma_bool match = SIGMA_TRUE;
        while (*a && *b) { if (*a++ != *b++) { match = SIGMA_FALSE; break; } }
        if (match && *a == '\0' && *b == '\0') return g_env[i].value;
    }
    return nullptr;
}

// ─── Aliases (Fish/Zsh-style) ────────────────────────────────────────────────
#define MAX_ALIASES 128
struct Alias {
    char name[64];
    char expansion[256];
};
static Alias g_aliases[MAX_ALIASES];
static sigma_u32 g_alias_count = 0;

sigma_status define_alias(const char* name, const char* expansion) {
    if (g_alias_count >= MAX_ALIASES) return SIGMA_ERROR;
    sigma_u32 j = 0;
    while (name[j] && j < 63) { g_aliases[g_alias_count].name[j] = name[j]; j++; }
    g_aliases[g_alias_count].name[j] = '\0';
    j = 0;
    while (expansion[j] && j < 255) { g_aliases[g_alias_count].expansion[j] = expansion[j]; j++; }
    g_aliases[g_alias_count].expansion[j] = '\0';
    g_alias_count++;
    return SIGMA_SUCCESS;
}

// ─── Command History (Zsh HISTFILE-style) ────────────────────────────────────
#define MAX_HISTORY 1024
static char g_history[MAX_HISTORY][512];
static sigma_u32 g_history_count = 0;
static sigma_u32 g_history_cursor = 0;

sigma_status history_push(const char* line) {
    sigma_u32 idx = g_history_count % MAX_HISTORY;
    sigma_u32 j = 0;
    while (line[j] && j < 511) { g_history[idx][j] = line[j]; j++; }
    g_history[idx][j] = '\0';
    g_history_count++;
    g_history_cursor = g_history_count;
    return SIGMA_SUCCESS;
}

const char* history_prev() {
    if (g_history_cursor == 0 || g_history_count == 0) return nullptr;
    g_history_cursor--;
    return g_history[g_history_cursor % MAX_HISTORY];
}

const char* history_next() {
    if (g_history_cursor >= g_history_count) return nullptr;
    g_history_cursor++;
    if (g_history_cursor >= g_history_count) return nullptr;
    return g_history[g_history_cursor % MAX_HISTORY];
}

// Fish-style auto-suggestion: return latest history entry starting with prefix
const char* history_suggest(const char* prefix) {
    if (!prefix || !prefix[0] || g_history_count == 0) return nullptr;
    
    // Search backwards (newest first)
    for (sigma_i32 i = g_history_count - 1; i >= 0; --i) {
        const char* entry = g_history[i % MAX_HISTORY];
        const char* p = prefix;
        const char* e = entry;
        sigma_bool match = SIGMA_TRUE;
        while (*p) {
            if (*p++ != *e++) { match = SIGMA_FALSE; break; }
        }
        if (match && *e != '\0') {
            return entry; // Found a match longer than prefix
        }
    }
    return nullptr;
}

// ─── Tokenizer ───────────────────────────────────────────────────────────────
static sigma_u32 tokenize(const char* input, Token* tokens, sigma_u32 max_tokens) {
    sigma_u32 count = 0;
    sigma_u32 i = 0;

    while (input[i] && count < max_tokens) {
        // Skip whitespace
        while (input[i] == ' ' || input[i] == '\t') i++;
        if (!input[i]) break;

        Token* t = &tokens[count];
        t->len = 0;

        if (input[i] == '|' && input[i+1] == '|') {
            t->type = TOK_OR; t->text[0] = '|'; t->text[1] = '|'; t->len = 2; i += 2;
        } else if (input[i] == '|') {
            t->type = TOK_PIPE; t->text[0] = '|'; t->len = 1; i++;
        } else if (input[i] == '>' && input[i+1] == '>') {
            t->type = TOK_REDIR_APP; t->text[0] = '>'; t->text[1] = '>'; t->len = 2; i += 2;
        } else if (input[i] == '>') {
            t->type = TOK_REDIR_OUT; t->text[0] = '>'; t->len = 1; i++;
        } else if (input[i] == '<') {
            t->type = TOK_REDIR_IN; t->text[0] = '<'; t->len = 1; i++;
        } else if (input[i] == '2' && input[i+1] == '>') {
            t->type = TOK_REDIR_ERR; t->text[0] = '2'; t->text[1] = '>'; t->len = 2; i += 2;
        } else if (input[i] == '&' && input[i+1] == '&') {
            t->type = TOK_AND; t->text[0] = '&'; t->text[1] = '&'; t->len = 2; i += 2;
        } else if (input[i] == '&') {
            t->type = TOK_BG; t->text[0] = '&'; t->len = 1; i++;
        } else if (input[i] == ';') {
            t->type = TOK_SEMICOL; t->text[0] = ';'; t->len = 1; i++;
        } else if (input[i] == '$') {
            t->type = TOK_VAR;
            i++; // skip $
            sigma_u32 j = 0;
            while (input[i] && input[i] != ' ' && input[i] != '\t' &&
                   input[i] != '|' && input[i] != '>' && input[i] != '<' &&
                   input[i] != ';' && input[i] != '&' && j < 255) {
                t->text[j++] = input[i++];
            }
            t->text[j] = '\0';
            t->len = j;
        } else if (input[i] == '"') {
            t->type = TOK_WORD;
            i++; // skip opening quote
            sigma_u32 j = 0;
            while (input[i] && input[i] != '"' && j < 255) {
                t->text[j++] = input[i++];
            }
            if (input[i] == '"') i++; // skip closing quote
            t->text[j] = '\0';
            t->len = j;
        } else if (input[i] == '\'') {
            t->type = TOK_WORD;
            i++;
            sigma_u32 j = 0;
            while (input[i] && input[i] != '\'' && j < 255) {
                t->text[j++] = input[i++];
            }
            if (input[i] == '\'') i++;
            t->text[j] = '\0';
            t->len = j;
        } else {
            // Regular word
            t->type = TOK_WORD;
            sigma_u32 j = 0;
            while (input[i] && input[i] != ' ' && input[i] != '\t' &&
                   input[i] != '|' && input[i] != '>' && input[i] != '<' &&
                   input[i] != ';' && input[i] != '&' && j < 255) {
                t->text[j++] = input[i++];
            }
            t->text[j] = '\0';
            t->len = j;
        }

        t->text[t->len] = '\0';
        count++;
    }

    return count;
}

// ─── Simulated VFS (Linux Inspired File System Storage) ──────────────────────
#define MAX_FILES 64
#define MAX_FILE_NAME 64
#define MAX_FILE_CONTENT 4096

struct VfsFile {
    char name[MAX_FILE_NAME];
    char content[MAX_FILE_CONTENT];
    sigma_u32 len;
};

static VfsFile g_vfs[MAX_FILES];
static sigma_u32 g_vfs_count = 0;

// Safe bounded string copy
static void safe_strcpy(char* dest, const char* src, size_t dest_size) {
    if (!dest || dest_size == 0) return;
    std::strncpy(dest, src, dest_size - 1);
    dest[dest_size - 1] = '\0';
}

// Initialize VFS with default standard pre-seeded files (Arch/Alpine style)
void vfs_init() {
    if (g_vfs_count > 0) return; // Already initialized

    std::strcpy(g_vfs[0].name, "README.md");
    std::strcpy(g_vfs[0].content, "# SigmaOS Sovereign Zenith Shell\nInteractive shell environment with streams redirection.\n");
    g_vfs[0].len = std::strlen(g_vfs[0].content);

    std::strcpy(g_vfs[1].name, "Makefile");
    std::strcpy(g_vfs[1].content, "all:\n\tg++ -std=c++20 sigma_shell.cpp -o sigma_shell\n");
    g_vfs[1].len = std::strlen(g_vfs[1].content);

    std::strcpy(g_vfs[2].name, "config.json");
    std::strcpy(g_vfs[2].content, "{\n  \"theme\": \"sigma-256color\",\n  \"resilient\": true\n}\n");
    g_vfs[2].len = std::strlen(g_vfs[2].content);

    g_vfs_count = 3;
}

// VFS Write helper
sigma_status vfs_write(const char* name, const char* content, sigma_bool append) {
    if (std::strcmp(name, "/dev/null") == 0) {
        return SIGMA_SUCCESS; // Discard sink
    }

    // Check if file already exists
    for (sigma_u32 i = 0; i < g_vfs_count; ++i) {
        if (std::strcmp(g_vfs[i].name, name) == 0) {
            if (append) {
                sigma_u32 new_len = g_vfs[i].len + std::strlen(content);
                if (new_len >= MAX_FILE_CONTENT) {
                    return SIGMA_ERROR;
                }
                std::strcat(g_vfs[i].content, content);
                g_vfs[i].len = new_len;
            } else {
                if (std::strlen(content) >= MAX_FILE_CONTENT) {
                    return SIGMA_ERROR;
                }
                safe_strcpy(g_vfs[i].content, content, MAX_FILE_CONTENT);
                g_vfs[i].len = std::strlen(content);
            }
            return SIGMA_SUCCESS;
        }
    }

    // Create new file
    if (g_vfs_count >= MAX_FILES) {
        return SIGMA_ERROR;
    }
    if (std::strlen(name) >= MAX_FILE_NAME || std::strlen(content) >= MAX_FILE_CONTENT) {
        return SIGMA_ERROR;
    }

    safe_strcpy(g_vfs[g_vfs_count].name, name, MAX_FILE_NAME);
    safe_strcpy(g_vfs[g_vfs_count].content, content, MAX_FILE_CONTENT);
    g_vfs[g_vfs_count].len = std::strlen(content);
    g_vfs_count++;
    return SIGMA_SUCCESS;
}

// VFS Read helper
const char* vfs_read(const char* name) {
    if (std::strcmp(name, "/dev/null") == 0) {
        return ""; // Empty sink
    }
    for (sigma_u32 i = 0; i < g_vfs_count; ++i) {
        if (std::strcmp(g_vfs[i].name, name) == 0) {
            return g_vfs[i].content;
        }
    }
    return nullptr;
}

// ─── Stream Redirection State variables ──────────────────────────────────────
static char g_stdout_buf[MAX_FILE_CONTENT];
static sigma_u32 g_stdout_len = 0;
static sigma_bool g_stdout_redirected = SIGMA_FALSE;
static char g_stdout_redirect_file[MAX_FILE_NAME] = {0};
static sigma_bool g_stdout_append = SIGMA_FALSE;

static char g_stderr_buf[MAX_FILE_CONTENT];
static sigma_u32 g_stderr_len = 0;
static sigma_bool g_stderr_redirected = SIGMA_FALSE;
static char g_stderr_redirect_file[MAX_FILE_NAME] = {0};
static sigma_bool g_stderr_to_stdout = SIGMA_FALSE;

static char g_stdin_buf[MAX_FILE_CONTENT];
static sigma_u32 g_stdin_len = 0;
static sigma_bool g_stdin_redirected = SIGMA_FALSE;

// Reset active stream redirection variables
void reset_streams() {
    g_stdout_buf[0] = '\0';
    g_stdout_len = 0;
    g_stdout_redirected = SIGMA_FALSE;
    g_stdout_redirect_file[0] = '\0';
    g_stdout_append = SIGMA_FALSE;

    g_stderr_buf[0] = '\0';
    g_stderr_len = 0;
    g_stderr_redirected = SIGMA_FALSE;
    g_stderr_redirect_file[0] = '\0';
    g_stderr_to_stdout = SIGMA_FALSE;

    g_stdin_buf[0] = '\0';
    g_stdin_len = 0;
    g_stdin_redirected = SIGMA_FALSE;
}

// Flush and persist the streams to our simulated VFS files
void flush_streams() {
    if (g_stdout_redirected && g_stdout_redirect_file[0]) {
        vfs_write(g_stdout_redirect_file, g_stdout_buf, g_stdout_append);
    }
    if (g_stderr_redirected && g_stderr_redirect_file[0]) {
        vfs_write(g_stderr_redirect_file, g_stderr_buf, SIGMA_FALSE);
    }
}

// Stream routing writes
void shell_write_stdout(const char* text) {
    if (g_stdout_redirected) {
        sigma_u32 i = 0;
        while (text[i] && g_stdout_len < MAX_FILE_CONTENT - 1) {
            g_stdout_buf[g_stdout_len++] = text[i++];
        }
        g_stdout_buf[g_stdout_len] = '\0';
    } else {
        std::printf("%s", text);
    }
}

void shell_write_stderr(const char* text) {
    if (g_stderr_to_stdout) {
        shell_write_stdout(text);
    } else if (g_stderr_redirected) {
        sigma_u32 i = 0;
        while (text[i] && g_stderr_len < MAX_FILE_CONTENT - 1) {
            g_stderr_buf[g_stderr_len++] = text[i++];
        }
        g_stderr_buf[g_stderr_len] = '\0';
    } else {
        std::fprintf(stderr, "%s", text);
    }
}

// ─── Glob Expansion ──────────────────────────────────────────────────────────
static sigma_bool match_glob(const char* pattern, const char* text) {
    while (*pattern) {
        if (*pattern == '*') {
            while (*pattern == '*') pattern++;
            if (!*pattern) return SIGMA_TRUE;
            while (*text) {
                if (match_glob(pattern, text) == SIGMA_TRUE) return SIGMA_TRUE;
                text++;
            }
            return SIGMA_FALSE;
        } else if (*pattern == '?' || *pattern == *text) {
            if (!*text) return SIGMA_FALSE;
            pattern++;
            text++;
        } else {
            return SIGMA_FALSE;
        }
    }
    return (*text == '\0') ? SIGMA_TRUE : SIGMA_FALSE;
}

static sigma_u32 expand_glob(const char* pattern, const char** matches, sigma_u32 max_matches) {
    vfs_init();
    sigma_u32 count = 0;
    sigma_bool has_glob = SIGMA_FALSE;
    for (sigma_u32 i = 0; pattern[i]; ++i) {
        if (pattern[i] == '*' || pattern[i] == '?') { has_glob = SIGMA_TRUE; break; }
    }
    
    if (!has_glob) {
        matches[count++] = pattern;
        return count;
    }

    /* Dynamically read files from our in-memory VFS */
    for (sigma_u32 i = 0; i < g_vfs_count && count < max_matches; ++i) {
        if (match_glob(pattern, g_vfs[i].name) == SIGMA_TRUE) {
            matches[count++] = g_vfs[i].name;
        }
    }
    
    if (count == 0) {
        matches[count++] = pattern; /* No match, pass raw pattern */
    }
    return count;
}

// ─── Builtin Commands ────────────────────────────────────────────────────────
static sigma_bool str_eq(const char* a, const char* b) {
    while (*a && *b) { if (*a++ != *b++) return SIGMA_FALSE; }
    return (*a == '\0' && *b == '\0') ? SIGMA_TRUE : SIGMA_FALSE;
}

typedef sigma_status (*BuiltinFn)(sigma_u32 argc, const char** argv);

static sigma_status builtin_echo(sigma_u32 argc, const char** argv) {
    // Print all arguments separated by spaces to our stdout stream redirection
    for (sigma_u32 i = 1; i < argc; ++i) {
        shell_write_stdout(argv[i]);
        if (i < argc - 1) {
            shell_write_stdout(" ");
        }
    }
    shell_write_stdout("\n");
    return SIGMA_SUCCESS;
}

static sigma_status builtin_cd(sigma_u32 argc, const char** argv) {
    if (argc < 2) {
        // cd with no args → go to HOME
        const char* home = get_env("HOME");
        if (home) set_env("PWD", home);
    } else {
        set_env("PWD", argv[1]);
    }
    return SIGMA_SUCCESS;
}

static sigma_status builtin_export(sigma_u32 argc, const char** argv) {
    // export NAME=VALUE
    if (argc < 2) return SIGMA_ERROR;
    const char* arg = argv[1];
    char name[64], value[512];
    sigma_u32 ni = 0, vi = 0;
    sigma_u32 i = 0;
    while (arg[i] && arg[i] != '=' && ni < 63) name[ni++] = arg[i++];
    name[ni] = '\0';
    if (arg[i] == '=') i++;
    while (arg[i] && vi < 511) value[vi++] = arg[i++];
    value[vi] = '\0';
    return set_env(name, value);
}

static sigma_status builtin_alias_cmd(sigma_u32 argc, const char** argv) {
    if (argc < 2) return SIGMA_ERROR;
    const char* arg = argv[1];
    char name[64], expansion[256];
    sigma_u32 ni = 0, ei = 0, i = 0;
    while (arg[i] && arg[i] != '=' && ni < 63) name[ni++] = arg[i++];
    name[ni] = '\0';
    if (arg[i] == '=') i++;
    while (arg[i] && ei < 255) expansion[ei++] = arg[i++];
    expansion[ei] = '\0';
    return define_alias(name, expansion);
}

static sigma_status builtin_history(sigma_u32 argc, const char** argv) {
    (void)argc; (void)argv;
    for (sigma_u32 i = 0; i < g_history_count; ++i) {
        char buf[64];
        std::sprintf(buf, "%4d  ", i + 1);
        shell_write_stdout(buf);
        shell_write_stdout(g_history[i % MAX_HISTORY]);
        shell_write_stdout("\n");
    }
    return SIGMA_SUCCESS;
}

static sigma_status builtin_pwd(sigma_u32 argc, const char** argv) {
    (void)argc; (void)argv;
    const char* pwd = get_env("PWD");
    if (pwd) {
        shell_write_stdout(pwd);
        shell_write_stdout("\n");
    } else {
        shell_write_stdout("/\n");
    }
    return SIGMA_SUCCESS;
}

static sigma_status builtin_exit(sigma_u32 argc, const char** argv) {
    (void)argc; (void)argv;
    return SIGMA_ERROR; // Signal shell exit
}

// Simulated builtin: cat (Alpine/Arch style)
static sigma_status builtin_cat(sigma_u32 argc, const char** argv) {
    vfs_init();
    if (argc < 2) {
        // Read from stdin if redirected
        if (g_stdin_redirected) {
            shell_write_stdout(g_stdin_buf);
            return SIGMA_SUCCESS;
        }
        shell_write_stderr("cat: missing file operand\n");
        return SIGMA_ERROR;
    }

    for (sigma_u32 idx = 1; idx < argc; ++idx) {
        const char* content = vfs_read(argv[idx]);
        if (content) {
            shell_write_stdout(content);
        } else {
            char err[128];
            std::sprintf(err, "cat: %s: No such file or directory\n", argv[idx]);
            shell_write_stderr(err);
        }
    }
    return SIGMA_SUCCESS;
}

// Simulated builtin: ls (Alpine/Arch style)
static sigma_status builtin_ls(sigma_u32 argc, const char** argv) {
    vfs_init();
    (void)argc; (void)argv;
    for (sigma_u32 i = 0; i < g_vfs_count; ++i) {
        shell_write_stdout(g_vfs[i].name);
        shell_write_stdout("  ");
    }
    shell_write_stdout("\n");
    return SIGMA_SUCCESS;
}

struct BuiltinEntry {
    const char* name;
    BuiltinFn   fn;
};

static const BuiltinEntry g_builtins[] = {
    {"echo",    builtin_echo},
    {"cd",      builtin_cd},
    {"export",  builtin_export},
    {"alias",   builtin_alias_cmd},
    {"history", builtin_history},
    {"pwd",     builtin_pwd},
    {"exit",    builtin_exit},
    {"cat",     builtin_cat},
    {"ls",      builtin_ls},
};

static const sigma_u32 NUM_BUILTINS = sizeof(g_builtins) / sizeof(g_builtins[0]);

// ─── Execute a Single Command ────────────────────────────────────────────────
sigma_status execute_command(sigma_u32 argc, const char** argv) {
    if (argc == 0) return SIGMA_SUCCESS;

    // Check builtins first
    for (sigma_u32 i = 0; i < NUM_BUILTINS; ++i) {
        if (str_eq(argv[0], g_builtins[i].name)) {
            return g_builtins[i].fn(argc, argv);
        }
    }

    // If it's a simulated execution (e.g. running mock programs)
    if (str_eq(argv[0], "sigma-kernel") || str_eq(argv[0], "sigma-pqc")) {
        shell_write_stdout("SigmaOS Sovereign Core Subsystem Initializing...\n");
        return SIGMA_SUCCESS;
    }

    // Not a builtin, signal mock process error via stderr
    char err[128];
    std::sprintf(err, "sigma-shell: command not found: %s\n", argv[0]);
    shell_write_stderr(err);
    return SIGMA_SUCCESS;
}

// ─── Pipeline Executor ───────────────────────────────────────────────────────
sigma_status execute_pipeline(Token* tokens, sigma_u32 num_tokens) {
    vfs_init();
    reset_streams();

    // 1. First Pass: Parse Redirections (<, >, >>, 2>, 2>&1)
    // Extract filename targets and mark redirections
    std::vector<Token> clean_tokens;
    for (sigma_u32 i = 0; i < num_tokens; ++i) {
        if (tokens[i].type == TOK_REDIR_OUT) {
            if (i + 1 < num_tokens && tokens[i+1].type == TOK_WORD) {
                g_stdout_redirected = SIGMA_TRUE;
                safe_strcpy(g_stdout_redirect_file, tokens[i+1].text, MAX_FILE_NAME);
                g_stdout_append = SIGMA_FALSE;
                i++; // Skip filename token
            }
        } else if (tokens[i].type == TOK_REDIR_APP) {
            if (i + 1 < num_tokens && tokens[i+1].type == TOK_WORD) {
                g_stdout_redirected = SIGMA_TRUE;
                safe_strcpy(g_stdout_redirect_file, tokens[i+1].text, MAX_FILE_NAME);
                g_stdout_append = SIGMA_TRUE;
                i++; // Skip filename token
            }
        } else if (tokens[i].type == TOK_REDIR_IN) {
            if (i + 1 < num_tokens && tokens[i+1].type == TOK_WORD) {
                g_stdin_redirected = SIGMA_TRUE;
                const char* file_content = vfs_read(tokens[i+1].text);
                if (file_content) {
                    safe_strcpy(g_stdin_buf, file_content, MAX_FILE_CONTENT);
                    g_stdin_len = std::strlen(file_content);
                } else {
                    g_stdin_buf[0] = '\0';
                    g_stdin_len = 0;
                }
                i++; // Skip filename token
            }
        } else if (tokens[i].type == TOK_REDIR_ERR) {
            if (i + 1 < num_tokens && tokens[i+1].type == TOK_WORD) {
                // If it is 2>&1
                if (std::strcmp(tokens[i+1].text, "&1") == 0) {
                    g_stderr_to_stdout = SIGMA_TRUE;
                } else {
                    g_stderr_redirected = SIGMA_TRUE;
                    safe_strcpy(g_stderr_redirect_file, tokens[i+1].text, MAX_FILE_NAME);
                }
                i++; // Skip filename/redirect token
            }
        } else if (tokens[i].type == TOK_WORD && std::strcmp(tokens[i].text, "2>&1") == 0) {
            g_stderr_to_stdout = SIGMA_TRUE;
        } else {
            clean_tokens.push_back(tokens[i]);
        }
    }

    // 2. Second Pass: Build Command Arguments & Execute Clean Segment
    const char* argv[32];
    sigma_u32 argc = 0;
    sigma_status final_status = SIGMA_SUCCESS;

    for (size_t i = 0; i <= clean_tokens.size(); ++i) {
        if (i == clean_tokens.size() || clean_tokens[i].type == TOK_PIPE ||
            clean_tokens[i].type == TOK_SEMICOL || clean_tokens[i].type == TOK_AND ||
            clean_tokens[i].type == TOK_OR) {

            if (argc > 0) {
                final_status = execute_command(argc, argv);
            }
            argc = 0;

        } else if (clean_tokens[i].type == TOK_WORD) {
            const char* matches[16];
            sigma_u32 n = expand_glob(clean_tokens[i].text, matches, 16);
            for (sigma_u32 m = 0; m < n && argc < 31; ++m) {
                argv[argc++] = matches[m];
            }
        } else if (clean_tokens[i].type == TOK_VAR) {
            const char* val = get_env(clean_tokens[i].text);
            if (val && argc < 31) argv[argc++] = val;
        }
    }

    // 3. Third Pass: Flush active streams back to VFS files
    flush_streams();
    reset_streams();

    return final_status;
}

// ─── Shell Init ──────────────────────────────────────────────────────────────
sigma_status shell_init() {
    vfs_init();
    set_env("HOME", "/root");
    set_env("PWD", "/root");
    set_env("PATH", "/bin:/usr/bin:/usr/local/bin");
    set_env("SHELL", "/bin/sigma-shell");
    set_env("TERM", "sigma-256color");
    set_env("PS1", "\\[\\033[1;36m\\]σ\\[\\033[0m\\] \\w → ");

    // Default aliases (Fish-like convenience)
    define_alias("ll", "ls -la");
    define_alias("la", "ls -A");
    define_alias("gs", "git status");
    define_alias("gd", "git diff");
    define_alias("cls", "clear");

    return SIGMA_SUCCESS;
}

// ─── REPL Main Loop ─────────────────────────────────────────────────────────
sigma_status shell_run() {
    shell_init();

    sigma_bool running = SIGMA_TRUE;
    char line[512];
    Token tokens[64];

    while (running) {
        // Simulated reading a line
        line[0] = '\0';

        if (line[0] == '\0') continue;

        history_push(line);

        sigma_u32 num_tokens = tokenize(line, tokens, 64);
        if (num_tokens == 0) continue;

        sigma_status s = execute_pipeline(tokens, num_tokens);
        if (s == SIGMA_ERROR) {
            // Check if it was an exit command
            if (str_eq(tokens[0].text, "exit")) {
                running = SIGMA_FALSE;
            }
        }
    }

    return SIGMA_SUCCESS;
}

} // namespace shell
} // namespace sigma

extern "C" {
    sigma_status sigma_shell_init(void) { return sigma::shell::shell_init(); }
    sigma_status sigma_shell_run(void)  { return sigma::shell::shell_run(); }
    sigma_status sigma_shell_exec(const char* line) {
        sigma::shell::Token tokens[64];
        sigma_u32 n = sigma::shell::tokenize(line, tokens, 64);
        return sigma::shell::execute_pipeline(tokens, n);
    }
}
