## SigmaOS: SYS_IOCTL */, fb_fd, SIGMA_IOCTL_FB_GET_INFO, (long)&current_fb, 0, 0, 0);
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

proc DG_Init*() {.exportc.} =
  discard

proc DG_DrawFrame*() {.exportc.} =
  discard

proc DG_SleepMs*() {.exportc.} =
  discard

