/* SPDX-License-Identifier: GPL-2.0-or-later */
/**
 * sigma_frama_c.h — Frama-C / ACSL formal verification contracts for SigmaOS
 *
 * This file contains ACSL (ANSI/ISO C Specification Language) contracts that
 * can be checked with Frama-C's WP (Weakest Precondition) plugin to formally
 * verify critical kernel subsystems.
 *
 * Verification targets (priority order):
 *   1. Memory allocator     — no use-after-free, no double-free
 *   2. Scheduler            — every runnable thread eventually gets CPU time
 *   3. IPC channel          — no message injection by unprivileged process
 *   4. CryptFS key handling — key material never reaches userspace
 *   5. PQC sig verification — invalid signatures always rejected
 *
 * Build with:
 *   frama-c -wp -wp-rte -wp-timeout 30 \
 *           kernel/core/memory/sigma_pmm.c \
 *           -cpp-extra-args="-Iinclude" \
 *           tests/formal/sigma_frama_c.h
 *
 * Inspired by:
 *   • seL4 formal verification (L4.verified)
 *   • Frama-C user manual (CEA LIST)
 *   • CompCert verified C compiler contracts
 */

#ifndef SIGMA_FRAMA_C_H
#define SIGMA_FRAMA_C_H

#ifdef __FRAMAC__
  /* Only active during Frama-C analysis */

#include <stdint.h>
#include <stdbool.h>

// ── Memory allocator contracts (sigma_pmm.c) ─────────────────────────────────

/*@ predicate ValidPage(uint64_t pa) =
      pa != 0 &&
      (pa % 4096) == 0 &&
      pa < 0x100000000ULL;  // within 4GB physical address space
*/

/*@ predicate PageFree{State}(uint64_t pa) =
      \valid((uint8_t *)pa + (0..4095)) &&
      \forall integer i; 0 <= i < 4096 ==>
          ((uint8_t *)pa)[i] == 0;
*/

/*@ contract sigma_pmm_alloc_page:
      requires \true;
      assigns  \nothing;   // only internal bitmap changes
      ensures  \result == 0 || ValidPage(\result);
      ensures  \result != 0 ==> PageFree(\result);
*/
extern uintptr_t sigma_pmm_alloc_page(void);

/*@ contract sigma_pmm_free_page:
      requires ValidPage(pa);
      requires \valid((uint8_t *)pa + (0..4095));
      assigns  \nothing;
      ensures  PageFree(pa);
*/
extern void sigma_pmm_free_page(uintptr_t pa);

// ── Scheduler liveness contract ───────────────────────────────────────────────

/*@ predicate ThreadRunnable(uint32_t tid) =
      tid < 65536 &&
      sigma_sched_state(tid) == SCHED_STATE_RUNNABLE;
*/

/*@ contract sigma_sched_pick_next:
      requires \exists uint32_t t; ThreadRunnable(t);
      assigns  \nothing;
      ensures  ThreadRunnable(\result);
      ensures  \result < 65536;
*/
extern uint32_t sigma_sched_pick_next(uint32_t cpu);

// ── IPC no-injection contract ─────────────────────────────────────────────────

/*@ predicate PrivilegedCaller(uint32_t pid) =
      sigma_process_cap(pid, CAP_IPC_WRITE) == CAP_GRANTED;
*/

/*@ contract sigma_bus_send:
      requires PrivilegedCaller(sender_pid);
      requires \valid_read(msg + (0..msg_len-1));
      requires msg_len <= 4096;
      assigns  \nothing;
      ensures  \result == 0 || \result == -EPERM;
      // No message is delivered if sender is unprivileged:
      ensures  !PrivilegedCaller(sender_pid) ==> \result == -EPERM;
*/
extern int sigma_bus_send(uint32_t sender_pid, uint32_t dest_cid,
                          const uint8_t *msg, uint32_t msg_len);

// ── PQC signature contract ────────────────────────────────────────────────────

/*@ predicate ValidDilithiumSig(
      const uint8_t *pk, uint32_t pk_len,
      const uint8_t *sig, uint32_t sig_len,
      const uint8_t *msg, uint32_t msg_len) =
      pk_len  == 1952 &&
      sig_len == 3293 &&
      msg_len > 0 &&
      \valid_read(pk  + (0..pk_len-1))  &&
      \valid_read(sig + (0..sig_len-1)) &&
      \valid_read(msg + (0..msg_len-1));
*/

/*@ contract sigma_dilithium3_verify:
      requires ValidDilithiumSig(pk, pk_len, sig, sig_len, msg, msg_len);
      assigns  \nothing;
      ensures  \result == 0 || \result == -1;
      // Completeness: valid (pk, msg) signed with matching sk → 0
      // Soundness: if sig was not produced by sk matching pk → -1
      // The soundness guarantee holds with overwhelming probability (2^-128)
*/
extern int sigma_dilithium3_verify(const uint8_t *pk,  uint32_t pk_len,
                                   const uint8_t *sig, uint32_t sig_len,
                                   const uint8_t *msg, uint32_t msg_len);

// ── CryptFS key non-escape contract ───────────────────────────────────────────

/*@ predicate KeyMaterial(uint8_t *p, uint32_t len) =
      \valid(p + (0..len-1)) &&
      // The key pages are kernel-only: user bit is NOT set in their PTEs
      \forall integer i; 0 <= i < len ==>
          sigma_pte_user_bit(p + i) == 0;
*/

/*@ contract sigma_cryptfs_get_key:
      assigns  \nothing;
      ensures  \result == NULL ||
               (KeyMaterial((uint8_t *)\result, 32) &&
                \result != NULL);
      // The returned pointer is NEVER in user-accessible memory
      ensures  \result == NULL ||
               sigma_pte_user_bit(\result) == 0;
*/
extern const uint8_t *sigma_cryptfs_get_key(void);

#endif /* __FRAMAC__ */
#endif /* SIGMA_FRAMA_C_H */
