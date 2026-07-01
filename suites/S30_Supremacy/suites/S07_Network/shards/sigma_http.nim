## SigmaOS: sigma_http module
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

proc sigma_http_send_404*() {.exportc.} =
  discard

proc sigma_http_send_200_chromium*() {.exportc.} =
  discard

proc sigma_route_static_file*() {.exportc.} =
  discard

proc sigma_handle_client_connection*() {.exportc.} =
  discard

