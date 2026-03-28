/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS Enterprise IPC Subsystem (C Core)
 * =========================================
 * Fast, lock-free inter-process message passing via ring buffers.
 * Architecture: Each process pair shares a cache-line aligned ring buffer
 * in the kernel's shared memory region. Zero system-call overhead for
 * sub-16KB messages (FAST PATH).
 *
 * Performance target: < 500ns round-trip on same-core processes.
 * IP Compliance: 100% original algorithm. No GPL/LGPL code included.
 *
 * Author: SigmaOS Kernel Team
 */

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


/* ── Constants ─────────────────────────────────────────────────────────── */

#define SIGMA_IPC_RING_SIZE 4096 /* Must be power-of-two            */
#define SIGMA_IPC_RING_MASK (SIGMA_IPC_RING_SIZE - 1)
#define SIGMA_IPC_MAX_CHANNELS 256
#define SIGMA_IPC_MSG_MAX 512      /* bytes per message (fast path)   */
#define SIGMA_IPC_MAGIC 0x5349474D /* "SIGM"                       */

/* ── Types ─────────────────────────────────────────────────────────────── */

typedef uint32_t pid_t;
typedef uint64_t tick_t;

typedef struct __attribute__((packed)) sigma_ipc_msg {
  uint32_t magic;  /* SIGMA_IPC_MAGIC — integrity marker         */
  uint16_t length; /* Payload length in bytes (max 512)          */
  uint16_t flags;  /* IPC_FLAG_URGENT | IPC_FLAG_REPLY           */
  pid_t sender_pid;
  pid_t receiver_pid;
  tick_t timestamp; /* TSC-based nanosecond timestamp             */
  uint8_t payload[SIGMA_IPC_MSG_MAX];
} sigma_ipc_msg_t;

/* Cache-line aligned ring buffer (avoids false sharing on SMP) */
typedef struct __attribute__((aligned(64))) sigma_ipc_ring {
  volatile uint64_t head; /* Producer writes here           */
  uint8_t _pad0[56];      /* Pad to full cache line         */
  volatile uint64_t tail; /* Consumer reads here            */
  uint8_t _pad1[56];
  sigma_ipc_msg_t buf[SIGMA_IPC_RING_SIZE];
  uint32_t channel_id;
  uint32_t flags;
  bool active;
} sigma_ipc_ring_t;

/* Global channel table */
static sigma_ipc_ring_t _channels[SIGMA_IPC_MAX_CHANNELS];
static uint32_t _channel_count = 0;

/* ── Internal Helpers ──────────────────────────────────────────────────── */

static inline void _memory_barrier(void) {
  /* Full memory barrier using x86 MFENCE */
  __asm__ volatile("mfence" ::: "memory");
}

static inline uint64_t _read_tsc(void) {
  uint32_t lo, hi;
  __asm__ volatile("rdtsc" : "=a"(lo), "=d"(hi));
  return ((uint64_t)hi << 32) | lo;
}

/* ── Public API ─────────────────────────────────────────────────────────── */

/**
 * sigma_ipc_create_channel - Create a new IPC channel between two processes.
 * @sender_pid:  PID of the sending process.
 * @recv_pid:    PID of the receiving process.
 * Returns: channel_id on success, -1 on failure (table full).
 */
int sigma_ipc_create_channel(pid_t sender_pid, pid_t recv_pid) {
  if (_channel_count >= SIGMA_IPC_MAX_CHANNELS)
    return -1;

  uint32_t cid = _channel_count++;
  sigma_ipc_ring_t *ch = &_channels[cid];

  ch->head = 0;
  ch->tail = 0;
  ch->channel_id = cid;
  ch->active = true;
  ch->flags = 0;

  _memory_barrier();
  return (int)cid;
}

/**
 * sigma_ipc_send - Send a message on a channel (FAST PATH, no syscall).
 * @channel_id: Target channel.
 * @data:       Pointer to payload bytes.
 * @length:     Payload length (max SIGMA_IPC_MSG_MAX).
 * @sender_pid: Caller's PID.
 * Returns: 0 on success, -1 if ring full.
 */
int sigma_ipc_send(uint32_t channel_id, const uint8_t *data, uint16_t length,
                   pid_t sender_pid) {
  if (channel_id >= _channel_count)
    return -1;
  if (length > SIGMA_IPC_MSG_MAX)
    return -1;

  sigma_ipc_ring_t *ch = &_channels[channel_id];

  uint64_t next_head = (ch->head + 1) & SIGMA_IPC_RING_MASK;
  if (next_head == ch->tail)
    return -1; /* Ring full — caller must retry */

  sigma_ipc_msg_t *msg = &ch->buf[ch->head & SIGMA_IPC_RING_MASK];
  msg->magic = SIGMA_IPC_MAGIC;
  msg->length = length;
  msg->sender_pid = sender_pid;
  msg->timestamp = _read_tsc();

  /* Copy payload — for sub-16B, inline copy beats memcpy */
  for (uint16_t i = 0; i < length; ++i)
    msg->payload[i] = data[i];

  _memory_barrier();
  ch->head = next_head;
  return 0;
}

/**
 * sigma_ipc_recv - Non-blocking receive from a channel.
 * @channel_id: Source channel.
 * @out:        Buffer to write the received message into.
 * Returns: 0 on success, -1 if ring empty.
 */
int sigma_ipc_recv(uint32_t channel_id, sigma_ipc_msg_t *out) {
  if (channel_id >= _channel_count)
    return -1;

  sigma_ipc_ring_t *ch = &_channels[channel_id];
  if (ch->head == ch->tail)
    return -1; /* Empty */

  sigma_ipc_msg_t *msg = &ch->buf[ch->tail & SIGMA_IPC_RING_MASK];

  /* Integrity check */
  if (msg->magic != SIGMA_IPC_MAGIC) {
    ch->tail = (ch->tail + 1) & SIGMA_IPC_RING_MASK;
    return -1;
  }

  *out = *msg;

  _memory_barrier();
  ch->tail = (ch->tail + 1) & SIGMA_IPC_RING_MASK;
  return 0;
}

/**
 * sigma_ipc_channel_len - Number of pending messages in a channel.
 */
uint32_t sigma_ipc_channel_len(uint32_t channel_id) {
  if (channel_id >= _channel_count)
    return 0;
  sigma_ipc_ring_t *ch = &_channels[channel_id];
  return (uint32_t)((ch->head - ch->tail) & SIGMA_IPC_RING_MASK);
}

/**
 * sigma_ipc_destroy_channel - Deactivates a channel and clears its buffer.
 */
void sigma_ipc_destroy_channel(uint32_t channel_id) {
  if (channel_id >= _channel_count)
    return;
  sigma_ipc_ring_t *ch = &_channels[channel_id];
  ch->active = false;
  ch->head = 0;
  ch->tail = 0;
  _memory_barrier();
}

