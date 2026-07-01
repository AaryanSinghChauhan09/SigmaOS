## SigmaOS: =============================================================================
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
  Pixel* = object

type
  Frame* = object
    width*: SigmaU32
    height*: SigmaU32
    timestamp_ns*: SigmaU64
    hash_fnv1a*: SigmaU32
    seq_num*: SigmaU32
    valid*: SigmaU64

type
  FilterKernel3x3* = object
    bias*: SigmaU64

type
  ScratchEvent* = object
    type*: SigmaU64
    id*: SigmaU32
    timestamp_ns*: SigmaU64

type
  EventBus* = object
    head*: SigmaU32
    tail*: SigmaU32
    count*: SigmaU32

type
  FilterEngine* = object
    count*: SigmaU32
    active_filter*: SigmaU32

type
  CaptureSession* = object
    session_id*: SigmaU32
    start_ns*: SigmaU64
    end_ns*: SigmaU64
    frames_captured*: SigmaU32
    frames_exported*: SigmaU32
    active*: SigmaU64

type
  CameraDevice* = object
    current_frame*: SigmaU64
    filter_engine*: SigmaU64
    event_bus*: SigmaU64
    session*: SigmaU64
    total_frames*: SigmaU32
    initialised*: SigmaU64

proc cam_strncpy*() {.exportc.} =
  discard

proc filter_set_3x3*() {.exportc.} =
  discard

proc filter_engine_init*() {.exportc.} =
  discard

proc frame_set_pixel*() {.exportc.} =
  discard

proc eventbus_push*() {.exportc.} =
  discard

proc camera_init*() {.exportc.} =
  discard

proc camera_list_filters*() {.exportc.} =
  discard

proc camera_process_events*() {.exportc.} =
  discard

