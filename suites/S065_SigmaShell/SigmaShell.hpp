#pragma once
#include <stdint.h>
#include "libc/sigma_libc.h"

namespace SigmaOS {
namespace Userland {

// Phase 2B (Sprint 1): POSIX-Compliant Terminal Shell
struct ProcessJob {
    uint32_t pid;
    bool is_background;
    const char* command;
    int status; // 0: RUNNING, 1: STOPPED, 2: DONE
};

class SigmaShell {
private:
    ProcessJob jobs[32];
    uint32_t job_count;
    
    void parse_pipes_and_redirects(const char* command);
    void spawn_job(const char* cmd, bool bg);

public:
    SigmaShell();
    
    void execute_line(const char* line);
    void list_jobs();
    void fg_job(uint32_t job_id);
    void bg_job(uint32_t job_id);
};

} // namespace Userland
} // namespace SigmaOS
