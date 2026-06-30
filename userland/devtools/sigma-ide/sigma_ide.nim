## SigmaOS: sigma_ide module
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
  sigma_buffer* = object
    gap_start*: SigmaU64
    gap_end*: SigmaU64
    capacity*: SigmaU64
    modified*: SigmaBool
    language*: SigmaU64
    lsp_doc_version*: SigmaU64

type
  lsp_completion* = object
    kind*: SigmaU64

type
  ai_debug_result* = object

type
  build_result* = object
    exit_code*: SigmaI32
    errors*: SigmaU64
    warnings*: SigmaU64

proc buf_insert*() {.exportc.} =
  discard

proc buf_delete_before*() {.exportc.} =
  discard

proc buf_move_right*() {.exportc.} =
  discard

proc buf_move_left*() {.exportc.} =
  discard

proc lsp_send_did_change*() {.exportc.} =
  discard

proc sigma_ide_run*() {.exportc.} =
  discard

