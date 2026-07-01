## SigmaOS: SIGMA_MEDIA_CODEC_H */
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
  SigmaCodec* = object
    codec_id*: SigmaI32
    decode*: SigmaU64
    encode*: SigmaU64
    decode_calls*: SigmaU64
    encode_calls*: SigmaU64
    total_bytes_decoded*: SigmaU64
    total_bytes_encoded*: SigmaU64

type
  SigmaCodecRegistry* = object
    count*: SigmaI32

proc codec_registry_init*() {.exportc.} =
  discard

