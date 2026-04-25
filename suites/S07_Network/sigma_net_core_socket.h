// SigmaOS — sigma-net-core-socket: Native Socket Handling
// Module: sigma-net-core
// USP: POSIX-like socket API mapped directly to native zero-copy ring buffers

#ifndef SIGMA_NET_CORE_SOCKET_H
#define SIGMA_NET_CORE_SOCKET_H

#define SIGMA_SOCK_STREAM 1
#define SIGMA_SOCK_DGRAM  2

#define SIGMA_MAX_SOCKETS 64

typedef struct SigmaSocket {
    unsigned int fd;
    unsigned int type;
    unsigned int local_ip;
    unsigned short local_port;
    unsigned int remote_ip;
    unsigned short remote_port;
    unsigned char connected;
} SigmaSocket;

typedef struct SigmaSocketRegistry {
    SigmaSocket sockets[SIGMA_MAX_SOCKETS];
    unsigned int count;
} SigmaSocketRegistry;

static inline void socket_registry_init(SigmaSocketRegistry* r) {
    r->count = 0;
}

static inline int sigma_socket(SigmaSocketRegistry* r, int domain, int type, int protocol) {
    (void)domain; (void)protocol;
    if (r->count >= SIGMA_MAX_SOCKETS) return -1;
    SigmaSocket* s = &r->sockets[r->count];
    s->fd = r->count + 100; // Base FD 100
    s->type = type;
    s->connected = 0;
    r->count++;
    return s->fd;
}

static inline int sigma_bind(SigmaSocketRegistry* r, int fd, unsigned int ip, unsigned short port) {
    for (unsigned int i = 0; i < r->count; i++) {
        if (r->sockets[i].fd == (unsigned int)fd) {
            r->sockets[i].local_ip = ip;
            r->sockets[i].local_port = port;
            return 0;
        }
    }
    return -1;
}

static inline int sigma_connect(SigmaSocketRegistry* r, int fd, unsigned int ip, unsigned short port) {
    for (unsigned int i = 0; i < r->count; i++) {
        if (r->sockets[i].fd == (unsigned int)fd) {
            r->sockets[i].remote_ip = ip;
            r->sockets[i].remote_port = port;
            r->sockets[i].connected = 1;
            return 0;
        }
    }
    return -1;
}

#endif /* SIGMA_NET_CORE_SOCKET_H */
