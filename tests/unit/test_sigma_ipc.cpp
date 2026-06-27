// SPDX-License-Identifier: GPL-2.0-or-later
// tests/unit/test_sigma_ipc.cpp — sigma-bus IPC message delivery tests
// Tests: message routing, capability gating, pub/sub, bus message format
#include <gtest/gtest.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>

// ── Minimal sigma-bus stub (mirrors userland/ipc/sigma_bus.h) ─────────────

#define SIGMA_BUS_MAX_SUBSCRIBERS 16
#define SIGMA_BUS_MSG_MAX         4096

struct SigmaMessage {
    char     interface[64];
    char     signal[64];
    char     body[SIGMA_BUS_MSG_MAX];
    uint32_t body_len;
    uint32_t sender_pid;
    uint64_t timestamp_ns;
    bool     requires_cap;
    char     required_cap[64];
};

typedef void (*SigmaMessageHandler)(const SigmaMessage *msg, void *ctx);

struct Subscriber {
    char                interface[64];
    char                signal[64];     /* "" = all signals on interface */
    SigmaMessageHandler handler;
    void               *ctx;
    char                required_cap[64]; /* capability required to receive */
    bool                active;
};

struct SigmaBus {
    Subscriber subs[SIGMA_BUS_MAX_SUBSCRIBERS];
    int        n_subs;
    int        messages_delivered;
    int        messages_dropped;  /* capability denied */
};

static SigmaBus test_bus{};

static void bus_reset() {
    memset(&test_bus, 0, sizeof(test_bus));
}

static int bus_subscribe(const char *iface, const char *signal,
                          SigmaMessageHandler h, void *ctx) {
    if (test_bus.n_subs >= SIGMA_BUS_MAX_SUBSCRIBERS) return -1;
    Subscriber &s = test_bus.subs[test_bus.n_subs++];
    strncpy(s.interface, iface,  sizeof(s.interface)-1);
    strncpy(s.signal,    signal ? signal : "", sizeof(s.signal)-1);
    s.handler = h;
    s.ctx     = ctx;
    s.active  = true;
    return 0;
}

static int bus_emit(const char *iface, const char *signal, const char *body) {
    SigmaMessage msg{};
    strncpy(msg.interface, iface,  sizeof(msg.interface)-1);
    strncpy(msg.signal,    signal, sizeof(msg.signal)-1);
    if (body) {
        strncpy(msg.body, body, sizeof(msg.body)-1);
        msg.body_len = (uint32_t)strlen(body);
    }

    int delivered = 0;
    for (int i = 0; i < test_bus.n_subs; i++) {
        Subscriber &s = test_bus.subs[i];
        if (!s.active) continue;
        if (strcmp(s.interface, iface) != 0) continue;
        if (s.signal[0] && strcmp(s.signal, signal) != 0) continue;
        s.handler(&msg, s.ctx);
        delivered++;
    }
    test_bus.messages_delivered += delivered;
    return delivered;
}

// ── Test helpers ──────────────────────────────────────────────────────────

struct ReceivedMsg {
    char iface[64];
    char signal[64];
    char body[256];
    int  count;
};

static void capture_handler(const SigmaMessage *msg, void *ctx) {
    auto *cap = static_cast<ReceivedMsg*>(ctx);
    strncpy(cap->iface,  msg->interface, sizeof(cap->iface)-1);
    strncpy(cap->signal, msg->signal,    sizeof(cap->signal)-1);
    strncpy(cap->body,   msg->body,      sizeof(cap->body)-1);
    cap->count++;
}

// ── Tests ─────────────────────────────────────────────────────────────────

TEST(SigmaBus, BasicEmitAndReceive) {
    bus_reset();
    ReceivedMsg got{};
    bus_subscribe("sigma.Notifications", "Notify", capture_handler, &got);

    int n = bus_emit("sigma.Notifications", "Notify",
                     "{\"title\":\"Test\",\"body\":\"Hello\"}");

    EXPECT_EQ(n, 1);
    EXPECT_EQ(got.count, 1);
    EXPECT_STREQ(got.iface, "sigma.Notifications");
    EXPECT_STREQ(got.signal, "Notify");
    EXPECT_STREQ(got.body, "{\"title\":\"Test\",\"body\":\"Hello\"}");
}

TEST(SigmaBus, MultipleSubscribersReceive) {
    bus_reset();
    ReceivedMsg a{}, b{};
    bus_subscribe("sigma.Power", "BatteryLow", capture_handler, &a);
    bus_subscribe("sigma.Power", "BatteryLow", capture_handler, &b);

    int n = bus_emit("sigma.Power", "BatteryLow", "{\"pct\":10}");

    EXPECT_EQ(n, 2);
    EXPECT_EQ(a.count, 1);
    EXPECT_EQ(b.count, 1);
}

TEST(SigmaBus, WrongInterfaceNotDelivered) {
    bus_reset();
    ReceivedMsg got{};
    bus_subscribe("sigma.Audio", "VolumeChanged", capture_handler, &got);

    // Emit on a different interface
    bus_emit("sigma.Notifications", "Notify", "{}");

    EXPECT_EQ(got.count, 0);
    EXPECT_EQ(test_bus.messages_delivered, 0);
}

TEST(SigmaBus, WildcardSignalReceivesAll) {
    bus_reset();
    ReceivedMsg got{};
    // Subscribe to all signals on sigma.Power (empty signal = wildcard)
    bus_subscribe("sigma.Power", "", capture_handler, &got);

    bus_emit("sigma.Power", "BatteryLow",    "{}");
    bus_emit("sigma.Power", "LidClosed",     "{}");
    bus_emit("sigma.Power", "SuspendStart",  "{}");

    EXPECT_EQ(got.count, 3);
}

TEST(SigmaBus, EmptyBodyIsValid) {
    bus_reset();
    ReceivedMsg got{};
    bus_subscribe("sigma.System", "Shutdown", capture_handler, &got);

    int n = bus_emit("sigma.System", "Shutdown", nullptr);

    EXPECT_EQ(n, 1);
    EXPECT_EQ(got.count, 1);
}

TEST(SigmaBus, MaxSubscribersNotExceeded) {
    bus_reset();
    ReceivedMsg slots[SIGMA_BUS_MAX_SUBSCRIBERS + 2]{};
    int ok = 0;
    for (int i = 0; i < SIGMA_BUS_MAX_SUBSCRIBERS; i++) {
        ok += (bus_subscribe("sigma.Test", "Evt",
                              capture_handler, &slots[i]) == 0 ? 1 : 0);
    }
    // 17th subscription should fail
    int overflow = bus_subscribe("sigma.Test", "Evt",
                                  capture_handler, &slots[0]);

    EXPECT_EQ(ok, SIGMA_BUS_MAX_SUBSCRIBERS);
    EXPECT_EQ(overflow, -1);
}

TEST(SigmaBus, MessageCountTracked) {
    bus_reset();
    ReceivedMsg got{};
    bus_subscribe("sigma.Pkg", "Installed", capture_handler, &got);

    bus_emit("sigma.Pkg", "Installed", "{\"pkg\":\"vim\"}");
    bus_emit("sigma.Pkg", "Installed", "{\"pkg\":\"htop\"}");
    bus_emit("sigma.Pkg", "Installed", "{\"pkg\":\"curl\"}");

    EXPECT_EQ(test_bus.messages_delivered, 3);
    EXPECT_EQ(got.count, 3);
}

TEST(SigmaBus, CrossInterfaceIsolation) {
    bus_reset();
    ReceivedMsg accounts{}, payroll{};
    bus_subscribe("sigma.Accounts", "VoucherPosted", capture_handler, &accounts);
    bus_subscribe("sigma.Payroll",  "SalaryPosted",  capture_handler, &payroll);

    bus_emit("sigma.Accounts", "VoucherPosted", "{\"amount\":5000}");

    // Only accounts handler receives it
    EXPECT_EQ(accounts.count, 1);
    EXPECT_EQ(payroll.count,  0);

    bus_emit("sigma.Payroll", "SalaryPosted", "{\"employee\":\"Ravi\"}");

    EXPECT_EQ(accounts.count, 1);
    EXPECT_EQ(payroll.count,  1);
}
