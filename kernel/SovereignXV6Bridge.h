#ifndef SOVEREIGN_XV6_BRIDGE_H
#define SOVEREIGN_XV6_BRIDGE_H

#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace XV6Parity {

// --- XV6 STYLE PROCESS & IPC (IIT BOMBAY / OSTEP) ---
class SovereignTrapHandler : public SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignTrapHandler"; }
    void HandleTrap(int trapno);
};

class SovereignPipeNode : public SigmaObject {
private:
    int m_fds[2];
public:
    const char* type_name() const noexcept override { return "SovereignPipeNode"; }
    void CreatePipe();
    void RedirectStdout(int fd);
};

class SovereignSleepWakeup : public SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignSleepWakeup"; }
    void Sleep(void* chan);
    void Wakeup(void* chan);
};

// --- ADVANCED LINUX SUBSYSTEM (PART Z) ---
class SovereignSocketMesh : public SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignSocketMesh"; }
    void EstablishTCP(const char* host, int port);
    void EpollWaitShard();
};

} // namespace XV6Parity
} // namespace SigmaOS

#endif

