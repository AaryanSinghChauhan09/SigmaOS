# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_voice.nim — Voice-to-command input module
# Captures microphone audio, transcribes via Whisper (local STT),
# pipes transcription to sigma-agent as natural language command.
#
# Inspiration:
#   llama.cpp whisper.cpp (local Whisper STT)
#   Claude Code voice mode (experimental)
#   ai-shell voice pipeline
#
# Pipeline:
#   Mic → [ALSA/PipeWire capture] → WAV → [whisper.cpp / sigma-voice] → text → sigma-agent
#
# Backends (in priority order):
#   1. sigma-voice daemon  (/run/sigma/voice.sock) — sovereign STT
#   2. whisper.cpp         (local GGML Whisper model)
#   3. SpeechRecognition   (sox + Google API fallback)
#   4. Text input fallback (always available)
#
# Language: Nim (stdlib only — audio capture via system commands)

import std/[os, osproc, strutils, strformat, times]

# ── Audio capture config ──────────────────────────────────────────────────────
const
  SAMPLE_RATE   = 16000    # Whisper requires 16kHz mono
  CHANNELS      = 1
  RECORD_SECS   = 5        # default recording window (seconds)
  TEMP_WAV      = "/tmp/sigma_voice_input.wav"
  WHISPER_MODEL = "~/.cache/sigma/models/whisper-base.en.bin"

# ── Backend detection ─────────────────────────────────────────────────────────
type VoiceBackend = enum VbSigmaVoice, VbWhisperCpp, VbSpeechRecognition, VbNone

proc detect_voice_backend(): VoiceBackend =
  if fileExists("/run/sigma/voice.sock"):                         return VbSigmaVoice
  if execCmdEx("which whisper-cpp 2>/dev/null")[1] == 0 or
     execCmdEx("which whisper 2>/dev/null")[1] == 0:             return VbWhisperCpp
  if execCmdEx("python3 -c 'import speech_recognition' 2>/dev/null")[1] == 0:
                                                                  return VbSpeechRecognition
  VbNone

# ── Audio capture ─────────────────────────────────────────────────────────────
proc detect_capture_tool(): string =
  let candidates = ["arecord", "sox", "ffmpeg", "pw-record"]
  for tool in candidates:
    if execCmdEx(fmt"which {tool} 2>/dev/null")[1] == 0: return tool
  ""

proc capture_audio(secs = RECORD_SECS, output = TEMP_WAV): bool =
  let tool = detect_capture_tool()
  if tool.len == 0:
    echo "✗ No audio capture tool found. Install: sigma-pkg install alsa-utils"
    return false

  echo fmt"\e[38;2;69;243;255m🎤 Recording for {secs} seconds... (speak now)\e[0m"

  let cmd = case tool
    of "arecord":
      fmt"arecord -f S16_LE -r {SAMPLE_RATE} -c {CHANNELS} -d {secs} {output.quoteShell} 2>/dev/null"
    of "sox":
      fmt"sox -d -r {SAMPLE_RATE} -c {CHANNELS} -b 16 {output.quoteShell} trim 0 {secs} 2>/dev/null"
    of "ffmpeg":
      fmt"ffmpeg -y -f alsa -i default -ar {SAMPLE_RATE} -ac {CHANNELS} -t {secs} {output.quoteShell} 2>/dev/null"
    of "pw-record":
      fmt"pw-record --rate={SAMPLE_RATE} --channels={CHANNELS} {output.quoteShell} &  sleep {secs} && kill $!"
    else: ""

  if cmd.len == 0: return false
  let (_, code) = execCmdEx(cmd)
  code == 0 and fileExists(output)

# ── Transcription backends ────────────────────────────────────────────────────
proc transcribe_sigma_voice(wav_path: string): string =
  ## sigma-voice daemon transcription via Unix socket
  let (out, code) = execCmdEx(
    fmt"""echo '{{"wav":"{wav_path}"}}' | nc -U /run/sigma/voice.sock 2>/dev/null""")
  if code == 0:
    try:
      import std/json
      let j = parseJson(out)
      return j.getOrDefault("text").getStr(out.strip())
    except: return out.strip()
  ""

proc transcribe_whisper(wav_path: string): string =
  ## Local Whisper.cpp transcription
  let model = WHISPER_MODEL.expandTilde()
  let binary = block:
    var b = ""
    for c in ["whisper-cpp","whisper"]:
      if execCmdEx(fmt"which {c} 2>/dev/null")[1] == 0: b = c; break
    b

  if binary.len == 0: return ""

  var cmd = fmt"{binary} {wav_path.quoteShell} --no-timestamps --language en"
  if fileExists(model):
    cmd &= fmt" --model {model.quoteShell}"

  let (out, code) = execCmdEx(cmd & " 2>/dev/null")
  if code == 0:
    # Whisper outputs "[00:00.000 --> 00:03.000] text" format
    var lines: seq[string]
    for line in out.strip().splitLines():
      let l = line.strip()
      if l.startsWith("[") and "]" in l:
        lines.add(l.split("]")[1].strip())
      elif l.len > 0 and not l.startsWith("["):
        lines.add(l)
    return lines.join(" ").strip()
  ""

proc transcribe_python(wav_path: string): string =
  ## Python SpeechRecognition fallback (requires network for Google API)
  let script = fmt"""
import speech_recognition as sr
r = sr.Recognizer()
with sr.AudioFile("{wav_path}") as src:
    audio = r.record(src)
try:
    print(r.recognize_google(audio))
except:
    print("")
"""
  let script_path = "/tmp/sigma_stt.py"
  writeFile(script_path, script)
  let (out, code) = execCmdEx(fmt"python3 {script_path.quoteShell} 2>/dev/null")
  removeFile(script_path)
  if code == 0: return out.strip()
  ""

proc transcribe*(wav_path: string): string =
  let backend = detect_voice_backend()
  case backend
  of VbSigmaVoice:        return transcribe_sigma_voice(wav_path)
  of VbWhisperCpp:        return transcribe_whisper(wav_path)
  of VbSpeechRecognition: return transcribe_python(wav_path)
  of VbNone:
    echo "✗ No speech recognition backend available."
    echo "  Install whisper.cpp: sigma-pkg install whisper-cpp"
    echo "  Or sigma-voice: sigma-pkg install sigma-voice"
    return ""

# ── Voice command pipeline ─────────────────────────────────────────────────────
proc voice_command*(secs = RECORD_SECS, dry_run = false, no_color = false): string =
  ## Full pipeline: capture → transcribe → return text (caller passes to agent)
  if not capture_audio(secs, TEMP_WAV):
    return ""
  echo "\e[38;2;107;114;128m  Transcribing...\e[0m"
  let text = transcribe(TEMP_WAV)
  removeFile(TEMP_WAV)   # clean up

  if text.len == 0:
    echo "✗ Could not understand audio. Try speaking more clearly."
    return ""

  let display = "\e[38;2;69;243;255m🎤 Heard:\e[0m " & text
  echo display

  if dry_run:
    echo fmt"\e[38;2;251;191;36m[dry-run] Would execute: {text}\e[0m"
    return ""

  text

# ── Continuous voice session ───────────────────────────────────────────────────
proc voice_session*(secs_per_turn = RECORD_SECS, no_color = false) =
  let backend = detect_voice_backend()
  if backend == VbNone:
    echo "✗ No voice backend. Install: sigma-pkg install sigma-voice"
    return

  echo "\e[38;2;69;243;255m\e[1mΣ sigma-agent voice session\e[0m"
  echo fmt"\e[38;2;107;114;128m  Backend: {backend}  |  Press Ctrl+C to exit\e[0m\n"

  while true:
    let text = voice_command(secs_per_turn, no_color=no_color)
    if text.len == 0: continue

    # Check for stop words
    if text.toLowerAscii in ["stop","exit","quit","bye","goodbye"]:
      echo "\e[38;2;107;114;128mVoice session ended.\e[0m"
      break

    # Execute via sigma-agent
    let (out, _) = execCmdEx(fmt"sigma-agent-core --once {text.quoteShell} 2>&1")
    echo out.strip()
    echo ""

# ── CLI ────────────────────────────────────────────────────────────────────────
proc voice_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "help":
    echo """sigma-agent voice — Voice input mode

Usage:
  sigma-agent voice                    Record 5s and execute command
  sigma-agent voice --secs 10          Record for 10 seconds
  sigma-agent voice --dry-run          Record + transcribe, don't execute
  sigma-agent voice --session          Continuous voice session (Ctrl+C to stop)
  sigma-agent voice --transcribe <wav> Transcribe an existing WAV file
  sigma-agent voice --status           Show voice backend status

Backends (in priority order):
  sigma-voice    sigma-pkg install sigma-voice       (sovereign, offline)
  whisper.cpp    sigma-pkg install whisper-cpp       (any GGML Whisper model)
  Python SR      pip install SpeechRecognition pyaudio  (Google API, needs internet)

Model setup (whisper.cpp):
  mkdir -p ~/.cache/sigma/models
  sigma-pkg install whisper-model-base-en  # ~150MB base English model
  # Or manually:
  # wget https://huggingface.co/ggerganov/whisper.cpp/blob/main/ggml-base.en.bin
  # mv ggml-base.en.bin ~/.cache/sigma/models/whisper-base.en.bin

Examples:
  sigma-agent voice                    # say "install sigma-edit"
  sigma-agent voice --secs 8           # longer recording window
  sigma-agent voice --session          # hands-free continuous mode
  sigma-agent voice --transcribe /tmp/audio.wav
"""
    return

  let dry_run  = "--dry-run" in args or "-d" in args
  let session  = "--session" in args
  let no_color = "--no-color" in args

  var secs = RECORD_SECS
  let secs_idx = args.find("--secs")
  if secs_idx >= 0 and secs_idx + 1 < args.len:
    secs = try: parseInt(args[secs_idx + 1]) except: RECORD_SECS

  if "--status" in args:
    let backend = detect_voice_backend()
    let capture = detect_capture_tool()
    echo fmt"Voice backend:  {backend}"
    echo fmt"Capture tool:   {if capture.len > 0: capture else: \"none — install alsa-utils\"}"
    let model = WHISPER_MODEL.expandTilde()
    echo fmt"Whisper model:  {if fileExists(model): model else: \"not found — see sigma-agent voice help\"}"
    return

  if "--transcribe" in args:
    let wav_idx = args.find("--transcribe")
    if wav_idx + 1 < args.len:
      let wav = args[wav_idx + 1]
      let text = transcribe(wav)
      if text.len > 0: echo text
      else: echo "✗ Transcription failed"
    return

  if session:
    voice_session(secs, no_color)
    return

  let text = voice_command(secs, dry_run, no_color)
  if text.len > 0 and not dry_run:
    let (out, _) = execCmdEx(fmt"sigma-agent-core --once {text.quoteShell} 2>&1")
    echo out.strip()
