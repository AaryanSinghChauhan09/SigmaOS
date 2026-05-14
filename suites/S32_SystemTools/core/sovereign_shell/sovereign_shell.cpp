#include "../../../../../include/libc/SovereignLibC.h"
#include "../../../../../include/sigma_log.h"
#include "../../../../../include/core/sigma_types.h"

// ---- VT100 control codes (no ncurses) ----
#define VT_RESET    "\033[0m"
#define VT_BOLD     "\033[1m"
#define VT_CYAN     "\033[36m"
#define VT_GREEN    "\033[32m"
#define VT_YELLOW   "\033[33m"
#define VT_RED      "\033[31m"
#define VT_MAGENTA  "\033[35m"
#define VT_BLUE     "\033[34m"

#define SIGMA_SHELL_VERSION   "2.1.0"
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
            "  |  Sovereign Shell  -  type 'help' for commands   |\n"
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
        sigma_log_info(VT_BOLD VT_MAGENTA "S" VT_RESET VT_CYAN " sigma" VT_RESET VT_GREEN " > " VT_RESET);
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
        // Built-in handlers (simplified for class context)
        if (sigma_hardened_strcmp(argv[0], "help") == 0) return builtinHelp();
        if (sigma_hardened_strcmp(argv[0], "exit") == 0) { sigma_exit(0); return 0; }
        if (sigma_hardened_strcmp(argv[0], "history") == 0) return builtinHistory();
        if (sigma_hardened_strcmp(argv[0], "clear") == 0) { sigma_log_info("\033[2J\033[H"); return 0; }
        
        sigma_log_info(VT_RED "  [sigma_sh] Command not found: %s  (type 'help')\n" VT_RESET, argv[0]);
        return 1;
    }

    int builtinHelp() {
        sigma_log_info(VT_BOLD VT_CYAN "\n  sigma_sh v%s - Sovereign Shell\n" VT_RESET, SIGMA_SHELL_VERSION);
        sigma_log_info(VT_BOLD "  Command         Description\n" VT_RESET);
        sigma_log_info("  -------------- ------------------------------------------\n");
        sigma_log_info("  " VT_GREEN "%-14s" VT_RESET "  %s\n", "help", "List all commands");
        sigma_log_info("  " VT_GREEN "%-14s" VT_RESET "  %s\n", "exit", "Exit the shell");
        sigma_log_info("  " VT_GREEN "%-14s" VT_RESET "  %s\n", "history", "Show command history");
        sigma_log_info("  " VT_GREEN "%-14s" VT_RESET "  %s\n", "clear", "Clear terminal");
        sigma_log_info("\n");
        return 0;
    }

    int builtinHistory() {
        sigma_u32 start = (m_history_len < SIGMA_HISTORY_SIZE) ? 0 : m_history_head;
        for (sigma_u32 i = 0; i < m_history_len; i++)
            sigma_log_info("  %4u  %s\n", i + 1, m_history[(start + i) % SIGMA_HISTORY_SIZE]);
        return 0;
    }
};

} // namespace Userland
} // namespace SigmaOS

extern "C" int main(void) {
    SigmaOS::Userland::SovereignShell::getInstance().init();
    SigmaOS::Userland::SovereignShell::getInstance().run();
    return 0;
}
