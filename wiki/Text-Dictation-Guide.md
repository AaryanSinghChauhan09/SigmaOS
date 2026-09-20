# Text Dictation Guide: On-Device Speech-to-Text in SigmaOS

## Introduction

SigmaOS includes a fast, private, on-device voice dictation feature (`Zenith Dictation Engine`). You can convert spoken speech directly into text inside any text field, document editor, terminal, or web browser without sending audio data to the cloud.

## Quick Start

1. **Activate Dictation**: Press `Super + D` (or click the microphone icon in the Zenith System Bar).
2. **Speak Naturally**: Dictate text clearly into your microphone.
3. **Deactivate Dictation**: Press `Super + D` again or pause speaking for more than 3 seconds.

## Voice Formatting Commands

While dictating, you can use natural spoken formatting commands:

| Spoken Command | Result Inserted |
|---|---|
| "period" or "full stop" | `.` |
| "comma" | `,` |
| "question mark" | `?` |
| "exclamation mark" | `!` |
| "new line" / "newline" | Line break (`\n`) |
| "new paragraph" | Double line break (`\n\n`) |
| "delete last word" | Erases previous word |

## Privacy & Security

- **100% Offline Execution**: All acoustic model inference runs locally on your CPU/NPU using Whisper quantized models.
- **Automatic Password Protection**: Dictation pauses automatically whenever focus moves to a password input box.
