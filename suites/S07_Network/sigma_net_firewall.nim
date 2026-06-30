## SigmaOS: SIGMA_NET_FIREWALL_H */
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
  SigmaFirewallRule* = object
    src_ip*: SigmaI32
    src_mask*: SigmaI32
    dst_ip*: SigmaI32
    dst_mask*: SigmaI32
    src_port*: SigmaU64
    dst_port*: SigmaU64
    protocol*: uint8
    action*: uint8
    hit_count*: SigmaU64

type
  SigmaFirewall* = object
    count*: SigmaI32
    default_action*: uint8

proc firewall_init*() {.exportc.} =
  discard

