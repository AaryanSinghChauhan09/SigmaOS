/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

// SigmaOS Native Networking (OOP Design)
// ===========================================
// Zero dependency. Replaces <sys/socket.h>, <arpa/inet.h>, <winsock2.h>.
// Pure low-level generic OS interface using basic machine-level networking syscalls.
// Designed for customisation, automation and personalisation (no heavy protocol bloat).

#ifndef SIGMA_NETWORK_SOCKET_HPP
#define SIGMA_NETWORK_SOCKET_HPP

#include "../types.h"
#include "../SigmaString.hpp"
#include "../MemoryAllocator.hpp"

// Forward assembly hook points
extern "C" i64 sigma_fast_syscall_linux(i64 sys_num, i64 arg1, i64 arg2, i64 arg3, i64 arg4, i64 arg5);
extern "C" i64 sigma_fast_syscall_windows(i64 sys_num, i64 arg1, i64 arg2, i64 arg3, i64 arg4, i64 arg5);

namespace Sigma {
namespace Network {

// Replaces standard struct sockaddr_in
struct NativeIPv4Addr {
    u16 family;
    u16 port;
    u32 addr;
    u8 zero[8];
};

class Socket {
private:
    i64 socket_descriptor;
    bool is_bound;
    bool is_connected;

    // Helper to flip byte order natively without <netinet/in.h> htons
    u16 NativeHtons(u16 hostshort) const {
        return (hostshort >> 8) | (hostshort << 8);
    }
    
    // Helper to flip byte order natively without <netinet/in.h> htonl
    u32 NativeHtonl(u32 hostlong) const {
        return ((hostlong & 0x000000FF) << 24) |
               ((hostlong & 0x0000FF00) << 8) |
               ((hostlong & 0x00FF0000) >> 8) |
               ((hostlong & 0xFF000000) >> 24);
    }

public:
    Socket() : socket_descriptor(-1), is_bound(false), is_connected(false) {}

    ~Socket() {
        Close();
    }

    bool CreateTCP() {
        if (socket_descriptor >= 0) return false;

#ifdef _WIN32
        // Emulated Native fast-call WSASocket mapping
        socket_descriptor = sigma_fast_syscall_windows(0x71, 2 /*AF_INET*/, 1 /*SOCK_STREAM*/, 6 /*IPPROTO_TCP*/, 0, 0);
#else
        // Linux: sys_socket (41) -> AF_INET(2), SOCK_STREAM(1), IPPROTO_TCP(0)
        socket_descriptor = sigma_fast_syscall_linux(41, 2, 1, 0, 0, 0);
#endif

        return socket_descriptor >= 0;
    }

    bool Bind(u16 port) {
        if (socket_descriptor < 0) return false;

        NativeIPv4Addr saddr;
        Core::MemoryAllocator::Set(&saddr, 0, sizeof(saddr));
        saddr.family = 2; // AF_INET
        saddr.port = NativeHtons(port);
        saddr.addr = 0; // INADDR_ANY natively

#ifdef _WIN32
        // Emulated native bind wrapper
        i64 res = sigma_fast_syscall_windows(0x72, socket_descriptor, (i64)&saddr, sizeof(saddr), 0, 0);
#else
        // Linux: sys_bind (49)
        i64 res = sigma_fast_syscall_linux(49, socket_descriptor, (i64)&saddr, sizeof(saddr), 0, 0);
#endif
        
        if (res == 0) {
            is_bound = true;
            return true;
        }
        return false;
    }

    bool Listen(i32 backlog = 10) {
        if (socket_descriptor < 0 || !is_bound) return false;

#ifdef _WIN32
        i64 res = sigma_fast_syscall_windows(0x73, socket_descriptor, backlog, 0, 0, 0);
#else
        // Linux: sys_listen (50)
        i64 res = sigma_fast_syscall_linux(50, socket_descriptor, backlog, 0, 0, 0);
#endif
        return res == 0;
    }

    Socket* Accept() {
        if (socket_descriptor < 0 || !is_bound) return NULL;

        NativeIPv4Addr client_addr;
        i32 addr_len = sizeof(client_addr);

#ifdef _WIN32
        i64 new_sd = sigma_fast_syscall_windows(0x74, socket_descriptor, (i64)&client_addr, (i64)&addr_len, 0, 0);
#else
        // Linux: sys_accept (43)
        i64 new_sd = sigma_fast_syscall_linux(43, socket_descriptor, (i64)&client_addr, (i64)&addr_len, 0, 0);
#endif

        if (new_sd >= 0) {
            // Allocate new Socket object via custom memory pool
            Socket* client_socket = new Socket();
            client_socket->socket_descriptor = new_sd;
            client_socket->is_connected = true;
            return client_socket;
        }
        return NULL;
    }

    size_t Send(const void* buffer, size_t len) {
        if (socket_descriptor < 0) return 0;
#ifdef _WIN32
        return (size_t)sigma_fast_syscall_windows(0x75, socket_descriptor, (i64)buffer, len, 0, 0);
#else
        // Linux: sys_sendto (44)
        return (size_t)sigma_fast_syscall_linux(44, socket_descriptor, (i64)buffer, len, 0, 0, 0);
#endif
    }

    size_t Receive(void* buffer, size_t max_len) {
        if (socket_descriptor < 0) return 0;
#ifdef _WIN32
        return (size_t)sigma_fast_syscall_windows(0x76, socket_descriptor, (i64)buffer, max_len, 0, 0);
#else
        // Linux: sys_recvfrom (45)
        return (size_t)sigma_fast_syscall_linux(45, socket_descriptor, (i64)buffer, max_len, 0, 0, 0);
#endif
    }

    void Close() {
        if (socket_descriptor >= 0) {
#ifdef _WIN32
            sigma_fast_syscall_windows(0x0F, socket_descriptor, 0, 0, 0, 0);
#else
            // Linux: sys_close (3)
            sigma_fast_syscall_linux(3, socket_descriptor, 0, 0, 0, 0);
#endif
            socket_descriptor = -1;
            is_bound = false;
            is_connected = false;
        }
    }
};

} // namespace Network
} // namespace Sigma

#endif // SIGMA_NETWORK_SOCKET_HPP

