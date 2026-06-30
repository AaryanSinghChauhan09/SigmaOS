## SigmaOS: SIGMA_NET_CORE_H */
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

type
  SigmaEthHeader* = object
    etype*: SigmaU64

type
  SigmaEthFrame* = object
    hdr*: SigmaU64
    payload_len*: SigmaI32

type
  SigmaNetStats* = object
    tx_frames*: SigmaU64
    rx_frames*: SigmaU64
    tx_bytes*: SigmaU64
    rx_bytes*: SigmaU64
    rx_drops*: SigmaU64

proc eth_frame_init*() {.exportc.} =
  discard

proc net_stats_record_tx*() {.exportc.} =
  discard

proc net_stats_record_rx*() {.exportc.} =
  discard

proc net_stats_record_drop*() {.exportc.} =
  discard

