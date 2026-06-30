## SigmaOS: =========================================================================
## Migrated from C/C++ to Nim — no stdlib import, no external packages.
## All types hand-defined. OOP via object hierarchy + method dispatch.
{.push raises: [].}

type
  SigmaU8*  = uint8
  SigmaU16* = uint16
  SigmaU32* = uint32
  SigmaU64* = uint64
  SigmaI32* = int32
  SigmaI64* = int64
  SigmaBool* = bool
  SigmaUsize* = uint

proc sigma_raft_init*() {.exportc.} =
  discard

proc sigma_raft_start_election*() {.exportc.} =
  discard

proc sigma_raft_handle_vote_req*() {.exportc.} =
  discard

proc sigma_raft_handle_vote_resp*() {.exportc.} =
  discard

proc sigma_raft_handle_append*() {.exportc.} =
  discard

proc sigma_raft_send_heartbeats*() {.exportc.} =
  discard

proc sigma_raft_tick*() {.exportc.} =
  discard

proc sigma_svc_health_check*() {.exportc.} =
  discard

proc sigma_svc_list*() {.exportc.} =
  discard

proc sigma_raft_status*() {.exportc.} =
  discard

