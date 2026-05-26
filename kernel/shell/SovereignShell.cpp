/**
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SHELL (sigma-sh v1.0)
 * =========================================================================
 * Minimal kernel-mode interactive shell with command registration,
 * tokenizer, history buffer, and pipe parsing support.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/kernel/sigma_shell.h"
#include "../../include/kernel/sigma_process_manager.h"
#include "../../include/kernel/sigma_device_manager.h"
#include "../../include/kernel/sigma_init_system.h"
#include "../../include/kernel/sigma_ipc_manager.h"
#include "../../include/security/sigma_sandbox.h"

/* Simple string comparison wrapper */
static int shell_strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) { s1++; s2++; }
    return *(const unsigned char*)s1 - *(const unsigned char*)s2;
}

namespace SigmaOS {
namespace Kernel {

class SovereignShell {
public:
    static SovereignShell& getInstance() {
        static SovereignShell instance;
        return instance;
    }

    void init() {
        m_cmd_count = 0;
        m_history_head = 0;
        m_history_count = 0;

        for (int i = 0; i < SHELL_HISTORY_SIZE; i++) {
            m_history[i][0] = '\0';
        }

        /* Register core built-ins */
        registerCmd("help", "List available commands", cmd_help, SIGMA_FALSE);
        registerCmd("ps", "List active processes", cmd_ps, SIGMA_FALSE);
        registerCmd("kill", "Kill a process by PID", cmd_kill, SIGMA_TRUE);
        registerCmd("devices", "List hardware device tree", cmd_devices, SIGMA_FALSE);
        registerCmd("services", "List init system services", cmd_services, SIGMA_FALSE);
        registerCmd("ipc", "Show IPC status (queues/shm)", cmd_ipc, SIGMA_FALSE);
        registerCmd("clear", "Clear terminal output", cmd_clear, SIGMA_FALSE);
        registerCmd("echo", "Print text to standard output", cmd_echo, SIGMA_FALSE);
        registerCmd("sandbox", "Show Sovereign Sandbox status", cmd_sandbox, SIGMA_FALSE);
        registerCmd("reboot", "Restart the system", cmd_reboot, SIGMA_TRUE);

        sigma_log("[SHELL] Sovereign Shell (sigma-sh) initialized.");
        sigma_log_info("[SHELL] Registered %u built-in commands.\n", m_cmd_count);
    }

    int registerCmd(const char* name, const char* desc,
                    sigma_shell_handler_t handler, sigma_bool kernel_only) {
        if (m_cmd_count >= SHELL_MAX_COMMANDS) return K_ERR_NOMEM;

        sigma_shell_cmd_t& cmd = m_commands[m_cmd_count++];
        sigma_strncpy(cmd.name, name, SHELL_CMD_NAME_LEN);
        sigma_strncpy(cmd.description, desc, SHELL_CMD_DESC_LEN);
        cmd.handler = handler;
        cmd.kernel_only = kernel_only;
        return K_OK;
    }

    int execute(const char* input_line) {
        if (!input_line || input_line[0] == '\0') return K_OK;

        /* Save to history */
        sigma_strncpy(m_history[m_history_head], input_line, SHELL_INPUT_BUF_LEN);
        m_history_head = (m_history_head + 1) % SHELL_HISTORY_SIZE;
        if (m_history_count < SHELL_HISTORY_SIZE) m_history_count++;

        /* Tokenizer */
        char buf[SHELL_INPUT_BUF_LEN];
        sigma_strncpy(buf, input_line, SHELL_INPUT_BUF_LEN);

        const char* argv[SHELL_MAX_ARGS];
        int argc = 0;
        
        /* Basic space-delimited tokenization (doesn't handle quotes yet) */
        char* token = buf;
        char* current = buf;
        
        while (*current && argc < SHELL_MAX_ARGS) {
            while (*current == ' ' || *current == '\t') current++;
            if (!*current) break;
            
            token = current;
            while (*current && *current != ' ' && *current != '\t') current++;
            
            if (*current) {
                *current = '\0';
                current++;
            }
            argv[argc++] = token;
        }

        if (argc == 0) return K_OK;

        /* Command lookup */
        for (sigma_u32 i = 0; i < m_cmd_count; i++) {
            if (shell_strcmp(m_commands[i].name, argv[0]) == 0) {
                /* Execute handler */
                return m_commands[i].handler(argc, argv);
            }
        }

        sigma_log_info("sigma-sh: command not found: %s\n", argv[0]);
        return K_ERR_NOTFOUND;
    }

    void printHelp() {
        sigma_log("Σ SigmaOS Shell Built-in Commands:");
        for (sigma_u32 i = 0; i < m_cmd_count; i++) {
            sigma_log_info("  %-12s - %s%s\n",
                           m_commands[i].name,
                           m_commands[i].description,
                           m_commands[i].kernel_only ? " [KERNEL]" : "");
        }
    }

    sigma_u32 getCommandCount() const { return m_cmd_count; }

private:
    SovereignShell() : m_cmd_count(0), m_history_head(0), m_history_count(0) {}

    sigma_shell_cmd_t m_commands[SHELL_MAX_COMMANDS];
    sigma_u32         m_cmd_count;
    char              m_history[SHELL_HISTORY_SIZE][SHELL_INPUT_BUF_LEN];
    sigma_u32         m_history_head;
    sigma_u32         m_history_count;

    /* --- Built-in Command Handlers --- */

    static int cmd_help(int argc, const char* argv[]) {
        SIGMA_UNUSED(argc); SIGMA_UNUSED(argv);
        SovereignShell::getInstance().printHelp();
        return 0;
    }

    static int cmd_ps(int argc, const char* argv[]) {
        SIGMA_UNUSED(argc); SIGMA_UNUSED(argv);
        process_list();
        return 0;
    }

    static int cmd_kill(int argc, const char* argv[]) {
        if (argc < 2) {
            sigma_log("Usage: kill <pid>");
            return -1;
        }
        /* Very basic atoi */
        int pid = 0;
        const char* p = argv[1];
        while (*p >= '0' && *p <= '9') { pid = pid * 10 + (*p - '0'); p++; }
        
        int res = process_kill((sigma_u32)pid);
        if (res != K_OK) sigma_log_info("kill: failed to terminate PID %d\n", pid);
        return res;
    }

    static int cmd_devices(int argc, const char* argv[]) {
        SIGMA_UNUSED(argc); SIGMA_UNUSED(argv);
        devmgr_print_tree();
        return 0;
    }

    static int cmd_services(int argc, const char* argv[]) {
        SIGMA_UNUSED(argc); SIGMA_UNUSED(argv);
        init_print_boot_log();
        return 0;
    }

    static int cmd_ipc(int argc, const char* argv[]) {
        SIGMA_UNUSED(argc); SIGMA_UNUSED(argv);
        ipc_print_status();
        return 0;
    }

    static int cmd_clear(int argc, const char* argv[]) {
        SIGMA_UNUSED(argc); SIGMA_UNUSED(argv);
        /* ANSI escape sequence for clear screen */
        sigma_log("\033[2J\033[H");
        return 0;
    }

    static int cmd_echo(int argc, const char* argv[]) {
        /* This isn't a true terminal echo, but prints to kernel log */
        for (int i = 1; i < argc; i++) {
            sigma_log_info("%s ", argv[i]);
        }
        sigma_log_info("\n");
        return 0;
    }

    static int cmd_sandbox(int argc, const char* argv[]) {
        SIGMA_UNUSED(argc); SIGMA_UNUSED(argv);
        sandbox_print_audit_log();
        return 0;
    }

    static int cmd_reboot(int argc, const char* argv[]) {
        SIGMA_UNUSED(argc); SIGMA_UNUSED(argv);
        sigma_log("System is going down for reboot NOW!");
        /* In real kernel: trigger ACPI reset or triple fault */
        return 0;
    }
};

} // namespace Kernel
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" {

void shell_init(void) { SigmaOS::Kernel::SovereignShell::getInstance().init(); }

int shell_register_cmd(const char* name, const char* desc,
                       sigma_shell_handler_t handler, sigma_bool kernel_only) {
    return SigmaOS::Kernel::SovereignShell::getInstance()
               .registerCmd(name, desc, handler, kernel_only);
}

int shell_execute(const char* input_line) {
    return SigmaOS::Kernel::SovereignShell::getInstance().execute(input_line);
}

void shell_run_interactive(void) {
    sigma_log("\n[sigma-sh] Interactive shell not fully supported in this stub.");
    sigma_log("[sigma-sh] Use shell_execute() programmatically.");
}

void shell_print_help(void) { SigmaOS::Kernel::SovereignShell::getInstance().printHelp(); }

void shell_print_prompt(void) {
    sigma_log_info("\nroot@sigmaos:~# ");
}

sigma_u32 shell_get_command_count(void) {
    return SigmaOS::Kernel::SovereignShell::getInstance().getCommandCount();
}

} // extern "C"
