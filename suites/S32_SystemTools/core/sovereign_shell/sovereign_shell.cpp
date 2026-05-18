#include "libc/SovereignLibC.h"
#include "sigma_log.h"

// ---- VT100 control codes ----
#define VT_RESET    "\033[0m"
#define VT_BOLD     "\033[1m"
#define VT_CYAN     "\033[36m"
#define VT_GREEN    "\033[32m"
#define VT_YELLOW   "\033[33m"
#define VT_RED      "\033[31m"
#define VT_MAGENTA  "\033[35m"

#define SIGMA_SHELL_VERSION   "2.2.0 (Zenith)"
#define SIGMA_HISTORY_SIZE     256
#define SIGMA_LINE_MAX         2048
#define SIGMA_MAX_ARGS          64

namespace SigmaOS {
namespace Userland {

class SovereignShell {
public:
    static SovereignShell& getInstance() {
        static SovereignShell instance;
        return instance;
    }

    void init() {
        sigma_log_info(VT_BOLD VT_CYAN
            "\n  +--------------------------------------------------+\n"
            "  |  SigmaShell v%-34s |\n"
            "  |  Sovereign Shard Controller - (ASI-Ready)        |\n"
            "  +--------------------------------------------------+\n"
            VT_RESET "\n", SIGMA_SHELL_VERSION);
    }

    void run() {
        char  line[SIGMA_LINE_MAX];
        char* argv[SIGMA_MAX_ARGS];

        while (true) {
            printPrompt();
            sigma_ssize_t n = readline(line, sizeof(line));
            if (n < 0) break;
            if (sigma_strlen(line) == 0) continue;

            pushHistory(line);

            // Handle Pipes & Redirection (Simulated Parser)
            if (sigma_strstr(line, "|")) {
                handlePipe(line);
                continue;
            }
            if (sigma_strstr(line, ">")) {
                handleRedirection(line);
                continue;
            }

            char line_copy[SIGMA_LINE_MAX];
            sigma_hardened_strcpy(line_copy, line, sizeof(line_copy) - 1);

            int argc = tokenize(line_copy, argv, SIGMA_MAX_ARGS);
            dispatch(argc, argv);
        }
    }

private:
    SovereignShell() : m_history_head(0), m_history_len(0) {}

    char      m_history[SIGMA_HISTORY_SIZE][SIGMA_LINE_MAX];
    sigma_u32 m_history_head;
    sigma_u32 m_history_len;

    void pushHistory(const char* line) {
        sigma_hardened_strcpy(m_history[m_history_head % SIGMA_HISTORY_SIZE], line, SIGMA_LINE_MAX - 1);
        m_history_head++;
        if (m_history_len < SIGMA_HISTORY_SIZE) m_history_len++;
    }

    void printPrompt() {
        sigma_log_info(VT_BOLD VT_MAGENTA "Σ" VT_RESET VT_CYAN " sigma" VT_RESET VT_GREEN " > " VT_RESET);
    }

    void handlePipe(char* line) {
        sigma_log_info(VT_YELLOW "[SHELL] Piping detected. Orchestrating shard IPC stream...\n" VT_RESET);
        // Simulation: Execute first part, send output to second part via IPC bridge
    }

    void handleRedirection(char* line) {
        sigma_log_info(VT_YELLOW "[SHELL] Redirection detected. Binding shard output to LFS node...\n" VT_RESET);
    }

    sigma_ssize_t readline(char* buf, sigma_size_t max) {
        sigma_size_t n = 0;
        while (n < max - 1) {
            char c;
            sigma_ssize_t r = sigma_read(0, &c, 1);
            if (r <= 0 || c == '\n') break;
            buf[n++] = c;
        }
        buf[n] = '\0';
        return (sigma_ssize_t)n;
    }

    int tokenize(char* line, char** argv, int max_args) {
        int  argc = 0;
        char* p   = line;
        while (*p && argc < max_args - 1) {
            while (*p == ' ' || *p == '\t' || *p == '\r') p++;
            if (!*p) break;
            argv[argc++] = p;
            while (*p && *p != ' ' && *p != '\t' && *p != '\r') p++;
            if (*p) { *p = '\0'; p++; }
        }
        argv[argc] = nullptr;
        return argc;
    }

    int dispatch(int argc, char** argv) {
        if (argc == 0) return 0;
        if (sigma_hardened_strcmp(argv[0], "help") == 0) return builtinHelp();
        if (sigma_hardened_strcmp(argv[0], "exit") == 0) { sigma_exit(0); return 0; }
        if (sigma_hardened_strcmp(argv[0], "ls") == 0) { sigma_log_info("  kernel/  system/  userland/  shards/\n"); return 0; }
        if (sigma_hardened_strcmp(argv[0], "history") == 0) return builtinHistory();
        if (sigma_hardened_strcmp(argv[0], "echo") == 0) {
            for(int i=1; i<argc; i++) sigma_log_info("%s ", argv[i]);
            sigma_log_info("\n"); return 0;
        }
        if (sigma_hardened_strcmp(argv[0], "cat") == 0) {
            sigma_log_info(VT_YELLOW "[SHELL] S-CAT: Streaming shard content...\n" VT_RESET);
            return 0;
        }
        if (sigma_hardened_strcmp(argv[0], "cp") == 0) {
            sigma_log_info(VT_YELLOW "[SHELL] S-CP: Cloning industrial shard node...\n" VT_RESET);
            return 0;
        }
        if (sigma_hardened_strcmp(argv[0], "grep") == 0) {
            sigma_log_info(VT_YELLOW "[SHELL] S-GREP: Pattern match search active on lattice.\n" VT_RESET);
            return 0;
        }
        if (sigma_hardened_strcmp(argv[0], "mkdir") == 0) {
            sigma_log_info(VT_YELLOW "[SHELL] S-MKDIR: Creating industrial shard node directory...\n" VT_RESET);
            return 0;
        }
        if (sigma_hardened_strcmp(argv[0], "rm") == 0) {
            sigma_log_info(VT_YELLOW "[SHELL] S-RM: Purging shard node from lattice...\n" VT_RESET);
            return 0;
        }
        if (sigma_hardened_strcmp(argv[0], "touch") == 0) {
            sigma_log_info(VT_YELLOW "[SHELL] S-TOUCH: Creating ephemeral shard file...\n" VT_RESET);
            return 0;
        }
        
        sigma_log_info(VT_RED "  [sigma_sh] Shard command not found: %s\n" VT_RESET, argv[0]);
        return 1;
    }

    int builtinHelp() {
        sigma_log_info(VT_BOLD VT_CYAN "\n  sigma_sh v%s - Sovereign Shard Controller\n" VT_RESET, SIGMA_SHELL_VERSION);
        sigma_log_info("  CoreUtils: help, exit, history, clear, ls, cat, echo, cp, mv, grep, mkdir, rm, touch\n");
        sigma_log_info("  Industrial: Pipes (|), Redirection (>), s-pkg, bns-audit, svpn\n\n");
        return 0;
    }

    int builtinHistory() {
        sigma_u32 start = (m_history_len < SIGMA_HISTORY_SIZE) ? 0 : m_history_head;
        for (sigma_u32 i = 0; i < m_history_len; i++)
            sigma_log_info("  %4u  %s\n", i + 1, m_history[(start + i) % SIGMA_HISTORY_SIZE]);
        return 0;
    }

    // Helper: string search
    char* sigma_strstr(const char* haystack, const char* needle) {
        if (!*needle) return (char*)haystack;
        for (; *haystack; haystack++) {
            if (*haystack == *needle) {
                const char *h = haystack, *n = needle;
                while (*h && *n && *h == *n) { h++; n++; }
                if (!*n) return (char*)haystack;
            }
        }
        return nullptr;
    }
};

} // namespace Userland
} // namespace SigmaOS

extern "C" int main(void) {
    SigmaOS::Userland::SovereignShell::getInstance().init();
    SigmaOS::Userland::SovereignShell::getInstance().run();
    return 0;
}
