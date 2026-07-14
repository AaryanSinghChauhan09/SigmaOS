## sigma_audio_pipewire.nim — SigmaOS Audio Daemon (PipeWire absorption)
## Language: Nim (freestanding OOP; no stdlib; no third-party)
## OOP: AudioDevice (abstract), AlsaDevice/SigmaDevice (derived), AudioGraph (composition)
## Specification: wiki_repo/OSS_Absorption_PipeWire.md
{.push raises: [].}

# ══════════════════════════════════════════════════════════════
#  § 1. Primitive types
# ══════════════════════════════════════════════════════════════

type
  SigmaU8*    = uint8
  SigmaU16*   = uint16
  SigmaU32*   = uint32
  SigmaI16*   = int16
  SigmaI32*   = int32
  SigmaUsize* = uint
  SigmaBool*  = bool

# ══════════════════════════════════════════════════════════════
#  § 2. Audio format parameters
# ══════════════════════════════════════════════════════════════

type
  SampleFormat* = enum
    sfS16LE  = 0   ## Signed 16-bit Little Endian
    sfS32LE  = 1   ## Signed 32-bit LE
    sfF32LE  = 2   ## Float 32-bit LE
    sfU8     = 3   ## Unsigned 8-bit

  AudioParams* = object
    sampleRate*:   SigmaU32    ## Hz (e.g. 48000)
    channels*:     SigmaU8     ## 1 = mono, 2 = stereo
    format*:       SampleFormat
    periodFrames*: SigmaU32    ## Frames per period (e.g. 256)

proc defaultParams*(): AudioParams =
  AudioParams(sampleRate: 48000, channels: 2, format: sfS16LE, periodFrames: 256)

# ══════════════════════════════════════════════════════════════
#  § 3. Fixed-size audio buffer (no heap)
# ══════════════════════════════════════════════════════════════

const MAX_PERIOD_FRAMES = 1024
const MAX_CHANNELS       = 8

type
  AudioBuffer* = object
    frames*:    array[MAX_PERIOD_FRAMES * MAX_CHANNELS, SigmaI16]
    nFrames*:   SigmaU32
    nChannels*: SigmaU8

proc clear*(buf: var AudioBuffer) =
  var i: SigmaUsize = 0
  while i < (buf.nFrames * buf.nChannels.SigmaU32).SigmaUsize:
    buf.frames[i] = 0
    i += 1

proc mix*(dst: var AudioBuffer; src: AudioBuffer) =
  ## Additive mix (clamp to int16 range).
  let n = (dst.nFrames * dst.nChannels.SigmaU32).SigmaUsize
  var i: SigmaUsize = 0
  while i < n:
    let v: SigmaI32 = dst.frames[i].SigmaI32 + src.frames[i].SigmaI32
    dst.frames[i] = if v > 32767: 32767 elif v < -32768: -32768 else: v.SigmaI16
    i += 1

# ══════════════════════════════════════════════════════════════
#  § 4. OOP: AudioDevice (abstract base)
# ══════════════════════════════════════════════════════════════

type
  DeviceState* = enum
    dsIdle     = 0
    dsRunning  = 1
    dsPaused   = 2
    dsError    = 3

  AudioDevice* = ref object of RootObj
    deviceId*:  SigmaU32
    params*:    AudioParams
    state*:     DeviceState
    volume*:    SigmaU8   ## 0..255

method open*(self: AudioDevice): SigmaBool {.base.} = false
method close*(self: AudioDevice) {.base.} = discard
method start*(self: AudioDevice): SigmaBool {.base.} = false
method stop*(self: AudioDevice): SigmaBool {.base.} = false
method readCapture*(self: AudioDevice; buf: var AudioBuffer): SigmaBool {.base.} = false
method writePlayback*(self: AudioDevice; buf: AudioBuffer): SigmaBool {.base.} = false

proc setVolume*(self: AudioDevice; v: SigmaU8) = self.volume = v

# ══════════════════════════════════════════════════════════════
#  § 5. SigmaAudioDevice — sovereign ALSA-bypass sink
#        (Writes to a ring buffer; no kernel ALSA dependency)
# ══════════════════════════════════════════════════════════════

const SINK_RING_PERIODS = 8

type
  SigmaAudioDevice* = ref object of AudioDevice
    ring*:     array[SINK_RING_PERIODS, AudioBuffer]
    writeHead*: SigmaUsize
    readHead*:  SigmaUsize
    filled*:    SigmaUsize

proc newSigmaAudioDevice*(id: SigmaU32; params: AudioParams): SigmaAudioDevice =
  result = SigmaAudioDevice(
    deviceId: id,
    params:   params,
    state:    dsIdle,
    volume:   200,
    writeHead: 0, readHead: 0, filled: 0,
  )

method open*(self: SigmaAudioDevice): SigmaBool =
  self.state = dsIdle
  true

method start*(self: SigmaAudioDevice): SigmaBool =
  if self.state == dsIdle:
    self.state = dsRunning
    return true
  false

method stop*(self: SigmaAudioDevice): SigmaBool =
  self.state = dsIdle
  true

method writePlayback*(self: SigmaAudioDevice; buf: AudioBuffer): SigmaBool =
  if self.state != dsRunning: return false
  if self.filled >= SINK_RING_PERIODS.SigmaUsize: return false  # Ring full
  self.ring[self.writeHead] = buf
  self.writeHead = (self.writeHead + 1) mod SINK_RING_PERIODS.SigmaUsize
  self.filled += 1
  true

method readCapture*(self: SigmaAudioDevice; buf: var AudioBuffer): SigmaBool =
  if self.filled == 0: return false
  buf = self.ring[self.readHead]
  self.readHead = (self.readHead + 1) mod SINK_RING_PERIODS.SigmaUsize
  self.filled -= 1
  true

# ══════════════════════════════════════════════════════════════
#  § 6. AudioGraph — PipeWire-style node graph (composition)
#        Nodes: source → [mix] → sink
# ══════════════════════════════════════════════════════════════

const MAX_NODES = 16

type
  GraphNode* = object
    device*: AudioDevice
    valid*:  SigmaBool

  AudioGraph* = ref object
    nodes*:  array[MAX_NODES, GraphNode]
    count*:  SigmaUsize
    sink*:   SigmaAudioDevice
    mixBuf*: AudioBuffer

proc newAudioGraph*(params: AudioParams): AudioGraph =
  result = AudioGraph(
    count: 0,
    sink: newSigmaAudioDevice(0, params),
  )
  result.mixBuf.nFrames   = params.periodFrames
  result.mixBuf.nChannels = params.channels
  discard result.sink.open()
  discard result.sink.start()

proc addNode*(g: AudioGraph; dev: AudioDevice): SigmaBool =
  if g.count >= MAX_NODES.SigmaUsize: return false
  g.nodes[g.count] = GraphNode(device: dev, valid: true)
  g.count += 1
  true

proc processCycle*(g: AudioGraph) =
  ## Collect audio from all source nodes, mix, write to sink.
  g.mixBuf.clear()
  var i: SigmaUsize = 0
  while i < g.count:
    if g.nodes[i].valid:
      var srcBuf = AudioBuffer(nFrames: g.mixBuf.nFrames, nChannels: g.mixBuf.nChannels)
      discard g.nodes[i].device.readCapture(srcBuf)
      g.mixBuf.mix(srcBuf)
    i += 1
  discard g.sink.writePlayback(g.mixBuf)

# ══════════════════════════════════════════════════════════════
#  § 7. Unit tests
# ══════════════════════════════════════════════════════════════

proc testAudioMix*(): bool =
  var a = AudioBuffer(nFrames: 4, nChannels: 1)
  var b = AudioBuffer(nFrames: 4, nChannels: 1)
  a.frames[0] = 10000
  b.frames[0] = 10000
  a.mix(b)
  if a.frames[0] != 20000: return false
  # Clamp test
  a.frames[1] = 32767
  b.frames[1] = 1
  a.mix(b)
  if a.frames[1] != 32767: return false  # Clamped
  true

proc testSigmaAudioDeviceLifecycle*(): bool =
  let dev = newSigmaAudioDevice(1, defaultParams())
  if not dev.open(): return false
  if not dev.start(): return false
  let params = defaultParams()
  var buf = AudioBuffer(nFrames: params.periodFrames, nChannels: params.channels)
  buf.frames[0] = 1234
  if not dev.writePlayback(buf): return false
  var out_buf = AudioBuffer(nFrames: params.periodFrames, nChannels: params.channels)
  if not dev.readCapture(out_buf): return false
  if out_buf.frames[0] != 1234: return false
  if not dev.stop(): return false
  true
