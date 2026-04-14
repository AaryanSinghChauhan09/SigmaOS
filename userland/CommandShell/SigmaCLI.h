#ifndef SIGMA_CLI_SHELL_H
#define SIGMA_CLI_SHELL_H

#include <sigma_types.h>


// SigmaOS Command Shell (CLI)
// Absorbing the pipeline power of PowerShell and the speed of Bash/Zsh

typedef struct {
    const char* command_name;
    const char* description;
    int (*execute)(int argc, char** argv);
} SigmaCommand;

// Built-in SigmaOS CLI Commands

// 1. Shard/Service Management (systemctl equivalent)
int cmd_shardctl(int argc, char** argv); // e.g., shardctl start S07_Network

// 2. Advanced Task Monitor (htop/top equivalent)
int cmd_sigmatop(int argc, char** argv);

// 3. Network Discovery and Routing (ip/ifconfig/nmap equivalent)
int cmd_netmesh(int argc, char** argv);

// 4. File System Operations (ls/tree with object capabilities)
int cmd_siglist(int argc, char** argv);

// 5. Zero-Trust Audit Logs (journalctl/Event Viewer equivalent)
int cmd_audittrail(int argc, char** argv);

// 6. Cross-Device Continuity Trigger (AirDrop/Handoff via CLI)
int cmd_handoff(int argc, char** argv);

// 7. Security Enclave & Crypto (ssh-keygen / openssl equivalent)
int cmd_sigmacrypt(int argc, char** argv);

// Shell Registration API
void shell_register_command(SigmaCommand* cmd);
void shell_start_interactive(void);

#endif // SIGMA_CLI_SHELL_H

