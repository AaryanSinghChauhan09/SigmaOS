// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_bus.h — Inter-process communication bus (D-Bus replacement)
 *
 * Lightweight, binary-protocol IPC over Unix domain sockets.
 * Faster than D-Bus (no XML marshalling), sovereign (no external dep).
 *
 * Architecture:
 *   sigma-busd           — central router daemon at /run/sigma/bus.sock
 *   sigma_bus_client_t   — per-process connection handle
 *   sigma_bus_interface  — named service (e.g. "sigma.Network")
 *
 * Usage — sending a signal:
 *   sigma_bus_client_t* bus = sigma_bus_connect();
 *   sigma_bus_emit(bus, "sigma.Network", "WiFiConnected",
 *                  "{\"ssid\":\"MyNet\",\"signal\":-65}");
 *
 * Usage — calling a method:
 *   char reply[1024];
 *   sigma_bus_call(bus, "sigma.Network", "GetStatus", NULL, reply, sizeof(reply));
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

#define SIGMA_BUS_SOCK       "/run/sigma/bus.sock"
#define SIGMA_BUS_MAX_MSG    65536   /* 64KB max message                      */
#define SIGMA_BUS_MAX_IFACE  64      /* bytes for interface name              */
#define SIGMA_BUS_MAX_MEMBER 64      /* bytes for method/signal name          */

/* ── Message types ────────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_BUS_MSG_CALL   = 1,   /* method call — expects a reply             */
    SIGMA_BUS_MSG_REPLY  = 2,   /* reply to a CALL                           */
    SIGMA_BUS_MSG_SIGNAL = 3,   /* broadcast signal — no reply expected      */
    SIGMA_BUS_MSG_ERROR  = 4,   /* error reply                               */
} sigma_bus_msg_type_t;

/* ── Wire header (fixed-size, little-endian) ─────────────────────────────── */
typedef struct __attribute__((packed)) {
    sigma_u32 serial;           /* monotonic message ID                      */
    sigma_u32 reply_serial;     /* for REPLY: serial of the CALL we answer   */
    sigma_u8  type;             /* sigma_bus_msg_type_t                      */
    char      iface[SIGMA_BUS_MAX_IFACE];   /* e.g. "sigma.Network"          */
    char      member[SIGMA_BUS_MAX_MEMBER]; /* e.g. "GetStatus" / "WiFiUp"   */
    sigma_u32 body_len;         /* bytes of JSON/msgpack body following      */
} sigma_bus_hdr_t;

/* ── Client context ──────────────────────────────────────────────────────── */
typedef struct sigma_bus_client sigma_bus_client_t;

typedef void (*sigma_bus_signal_cb_t)(const char* iface,
                                       const char* member,
                                       const char* body,
                                       void*       userdata);
typedef char* (*sigma_bus_method_cb_t)(const char* member,
                                        const char* body,
                                        void*       userdata);

/* ── API ──────────────────────────────────────────────────────────────────── */

/* Connect to sigma-busd; returns NULL on failure */
sigma_bus_client_t* sigma_bus_connect(void);
void                sigma_bus_disconnect(sigma_bus_client_t* client);

/* Register this process as a named service */
int sigma_bus_register_interface(sigma_bus_client_t* client,
                                   const char* iface,
                                   sigma_bus_method_cb_t method_handler,
                                   void* userdata);

/* Subscribe to signals from an interface */
int sigma_bus_subscribe(sigma_bus_client_t* client,
                          const char* iface,
                          sigma_bus_signal_cb_t signal_handler,
                          void* userdata);

/* Emit a signal (broadcast to all subscribers) */
int sigma_bus_emit(sigma_bus_client_t* client,
                    const char* iface,
                    const char* signal,
                    const char* body_json);  /* NULL = empty body */

/* Synchronous method call — blocks until reply arrives (timeout_ms=0 → 5s) */
int sigma_bus_call(sigma_bus_client_t* client,
                    const char* iface,
                    const char* method,
                    const char* body_json,
                    char* reply_out,
                    sigma_size_t reply_max,
                    sigma_u32 timeout_ms);

/* Process pending messages (call in event loop) */
int sigma_bus_dispatch(sigma_bus_client_t* client);

/* Get the underlying fd for poll/select integration */
int sigma_bus_fd(const sigma_bus_client_t* client);

/* ── Well-known interfaces ───────────────────────────────────────────────── */
#define SIGMA_IFACE_NETWORK      "sigma.Network"
#define SIGMA_IFACE_POWER        "sigma.Power"
#define SIGMA_IFACE_AUDIO        "sigma.Audio"
#define SIGMA_IFACE_NOTIFY       "sigma.Notifications"
#define SIGMA_IFACE_SESSION      "sigma.Session"
#define SIGMA_IFACE_CLIPBOARD    "sigma.Clipboard"
#define SIGMA_IFACE_PKG          "sigma.PackageManager"
#define SIGMA_IFACE_ZEROTRUST    "sigma.ZeroTrust"
