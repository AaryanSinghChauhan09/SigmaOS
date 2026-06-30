## SigmaOS: SIGMA_BPF_H */
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
  SigmaBPFFilter* = object
    fn*: SigmaU64
    hit_count*: SigmaU64
    drop_count*: SigmaU64

type
  SigmaBPFChain* = object
    count*: SigmaI32

proc bpf_chain_init*() {.exportc.} =
  discard

