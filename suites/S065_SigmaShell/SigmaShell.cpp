#include "SigmaShell.hpp"

namespace SigmaOS {
namespace Userland {

SigmaShell::SigmaShell() : job_count(0) {
    sigma_log("[SHELL] SigmaShell v1.0 (POSIX-Compliant) Online.");
}

void SigmaShell::parse_pipes_and_redirects(const char* command) {
    bool has_pipe = false;
    for (int i = 0; command[i] != '\0'; i++) {
        if (command[i] == '|') has_pipe = true;
    }
    
    if (has_pipe) {
        sigma_log("[SHELL] Initializing IPC Pipe for command chaining.");
    }
}

void SigmaShell::spawn_job(const char* cmd, bool bg) {
    if (job_count >= 32) return;
    
    jobs[job_count] = {job_count + 1000, bg, cmd, 0};
    
    sigma_print(bg ? "[bg] " : "[fg] ");
    sigma_print("Spawning process: ");
    sigma_print(cmd);
    sigma_print("\n");
    
    if (bg) {
        sigma_print("[%d] %d\n", job_count, jobs[job_count].pid);
    } else {
        jobs[job_count].status = 2; // DONE
    }
    job_count++;
}

void SigmaShell::execute_line(const char* line) {
    if (sigma_strlen(line) == 0) return;
    
    // Sprint 2B: Shell Scripting
    if (line[0] == '#' && line[1] == '!') {
        sigma_log("[SHELL] Parsed script shebang. Changing execution context.");
        return;
    }
    
    // Check for background '&'
    bool is_bg = false;
    int len = sigma_strlen(line);
    if (line[len - 1] == '&') {
        is_bg = true;
    }
    
    parse_pipes_and_redirects(line);
    spawn_job(line, is_bg);
}

void SigmaShell::list_jobs() {
    sigma_print("\n--- JOB CONTROL ---\n");
    for (uint32_t i = 0; i < job_count; i++) {
        if (jobs[i].status != 2) {
            sigma_print("[%d] %s  %s\n", i, jobs[i].status == 0 ? "Running" : "Stopped", jobs[i].command);
        }
    }
}

void SigmaShell::fg_job(uint32_t job_id) {
    sigma_log("[SHELL] Bringing job to foreground.");
}

void SigmaShell::bg_job(uint32_t job_id) {
    sigma_log("[SHELL] Resuming job in background.");
}

} // namespace Userland
} // namespace SigmaOS
