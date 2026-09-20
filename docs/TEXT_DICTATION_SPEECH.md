# SigmaOS Text Dictation & Neural Speech-to-Text Architecture Specification

## 1. Executive Summary

The SigmaOS Text Dictation Subsystem (`Zenith Dictation Engine`) provides zero-latency, 100% on-device neural voice dictation across all system applications. Inspired by Linux Speech-Dispatcher, Whisper.cpp, and BSD SNDIO privacy isolation, it converts microphone audio streams into real-time text input without sending audio data to external cloud servers.

## 2. System Architecture

```
+------------------------------------------------------------------+
|                    Microphone Audio Capture                      |
|            (PipeWire / SNDIO Isolated Input Stream)             |
+-------------------------------+----------------------------------+
                                |
                    PCM Audio Frames (16kHz Mono)
                                |
+-------------------------------+----------------------------------+
|                 Zenith Dictation Engine Daemon                   |
|  +-------------------------------------------------------------+ |
|  | 1. VAD (Voice Activity Detection & Silence Trimming)       | |
|  | 2. On-Device Neural Model (Whisper / Vosk Quantized INT8)   | |
|  | 3. Voice Formatting Rules ("period" -> ".", "new line" -> \n) | |
|  +-------------------------------------------------------------+ |
+-------------------------------+----------------------------------+
                                |
                   Recognized UTF-8 Text String
                                |
+-------------------------------+----------------------------------+
|                  Zenith Input Engine Injector                    |
|       (Commits text to active focused application surface)       |
+------------------------------------------------------------------+
```

## 3. Core Features & Capabilities

1. **Global Shortcut Toggle (`Super + D`)**: Instant activation/deactivation overlay displaying microphone audio level HUD.
2. **Punctuation & Voice Commands**:
   - Spoken "period" or "full stop" -> `.`
   - Spoken "comma" -> `,`
   - Spoken "question mark" -> `?`
   - Spoken "new line" or "newline" -> `\n`
   - Spoken "delete last word" -> deletes previous word in focus buffer.
3. **Multi-Language Support**: Quantized multilingual models supporting English, Spanish, German, French, Chinese, Japanese, and Korean.
4. **Privacy & Security Isolation**: Microphone streams are sandboxed via PipeWire capability gates. Dictation pauses automatically when password fields (`input_type = password`) receive focus.
