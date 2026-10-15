# WirePlumber Audio Session & Node Graph Governor Component Guide

## Component Overview
Governs PipeWire session management, dynamic audio node graph linking, automatic default sink failover, application volume ducking during voice calls, and Bluetooth audio profile switching.

## Linux Inspiration
WirePlumber PipeWire session manager, pipewire-media-session, and PulseAudio module-ducking.

## BSD Inspiration
OpenBSD sndio audio routing and sub-stream volume control, FreeBSD virtual_oss.

## Current SigmaOS Status
Audio node registry, node volume ducking, and default sink routing implemented in WireplumberAudioPolicyEngine.

## Critical Missing Features
Dynamic PipeWire wireplumber-compat Lua/Rust rule engine, and automatic priority-based audio device fallbacks.

## Implementation Priority
HIGH (Phase 1)

## Key Files to Create/Improve
src/desktop/omarchy_media_tools.rs, src/audio/mixer.rs, src/audio/graph.rs

## Testing Strategy
Voice communication audio ducking response test (music volume decreases to 30% within 10ms of speech), device disconnect failover.

## Dependencies
AUDIO_SUBSYSTEM_AGENT, BLUETOOTH_STACK_AGENT, IPC_AGENT

## Success Criteria
Sub-10ms audio routing re-negotiation on headphone disconnect, seamless voice call audio prioritization.

## Open Source Competitors Analysis
Replaces sluggish Lua-based WirePlumber configs with high-speed, type-safe Rust audio policy handlers.

## Future Enhancements
Per-application spatial audio positioning allowing developers to place notification sounds in 3D acoustic space.
