// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_voice_control.cpp — Voice navigation using sigma-ai (local)
 *
 * Speech pipeline:
 *   Microphone → sigma-audio capture → Whisper.cpp (local STT)
 *   → command parser → sigma-bus action dispatch
 *
 * All processing is 100% local — no data leaves the device.
 * Uses sigma-ai at localhost:17392 for speech-to-text.
 *
 * Voice commands (examples):
 *   "Open settings"          → launches sigma-settings
 *   "New invoice"            → opens sigma-accounts new voucher
 *   "Search files [query]"   → runs sigma-search
 *   "Read screen"            → triggers screen reader on focused element
 *   "Volume up / down"       → sigma.Audio.Volume bus event
 *   "Switch to workspace 2"  → sigma.Workspace.Switch
 *   "Lock screen"            → sigma.Session.Lock
 */

#include "sigma_voice_control.h"
#include <userland/ipc/sigma_bus.h>
#include <userland/a11y/sigma-l10n/sigma_locale.h>
#include <klib/sigma_trace.cpp>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>

namespace sigma::a11y {

// ── Command table ─────────────────────────────────────────────────────────
struct VoiceCommand {
    const char *phrase;           /* spoken phrase (lowercase) */
    const char *bus_interface;
    const char *bus_signal;
    const char *bus_body;         /* JSON body; nullptr = build dynamically */
};

static const VoiceCommand COMMANDS[] = {
    { "open settings",     "sigma.Apps",      "Launch",  "{\"app\":\"sigma-settings\"}" },
    { "new invoice",       "sigma.Accounts",  "NewVoucher", "{\"type\":\"sales\"}" },
    { "lock screen",       "sigma.Session",   "Lock",    "{}" },
    { "read screen",       "sigma.A11y",      "ReadFocus","{}"},
    { "volume up",         "sigma.Audio",     "Volume",  "{\"delta\":10}" },
    { "volume down",       "sigma.Audio",     "Volume",  "{\"delta\":-10}" },
    { "mute",              "sigma.Audio",     "Mute",    "{\"mute\":true}" },
    { "unmute",            "sigma.Audio",     "Mute",    "{\"mute\":false}" },
    { "workspace one",     "sigma.Workspace", "Switch",  "{\"index\":0}" },
    { "workspace two",     "sigma.Workspace", "Switch",  "{\"index\":1}" },
    { "workspace three",   "sigma.Workspace", "Switch",  "{\"index\":2}" },
    { "switch workspace",  "sigma.Workspace", "Next",    "{}" },
    { "take screenshot",   "sigma.Display",   "Screenshot", "{}" },
    { "open terminal",     "sigma.Apps",      "Launch",  "{\"app\":\"sigma-terminal\"}" },
    { "open files",        "sigma.Apps",      "Launch",  "{\"app\":\"sigma-files\"}" },
    { "open accounts",     "sigma.Apps",      "Launch",  "{\"app\":\"sigma-accounts\"}" },
    { "file gst return",   "sigma.GST",       "FileReturn", "{\"type\":\"GSTR-1\"}" },
    { nullptr, nullptr, nullptr, nullptr }
};

// ── Lowercase and strip punctuation ──────────────────────────────────────
static void normalise(const char *in, char *out, size_t max) {
    size_t j = 0;
    for (size_t i = 0; in[i] && j < max-1; i++) {
        char c = (char)tolower((unsigned char)in[i]);
        if (isalnum((unsigned char)c) || c == ' ')
            out[j++] = c;
    }
    out[j] = '\0';
}

// ── Match a transcription against the command table ───────────────────────
static const VoiceCommand *match_command(const char *text) {
    char norm[256];
    normalise(text, norm, sizeof(norm));

    for (int i = 0; COMMANDS[i].phrase; i++) {
        if (strstr(norm, COMMANDS[i].phrase))
            return &COMMANDS[i];
    }
    return nullptr;
}

// ── Handle a transcribed utterance ───────────────────────────────────────
int VoiceControl::process_utterance(const char *text) {
    if (!enabled || !text || !text[0]) return 0;

    SIGMA_DTRACE_PROBE1(voice_control, utterance, text);
    utterance_count++;

    const VoiceCommand *cmd = match_command(text);
    if (!cmd) {
        fprintf(stderr, "[sigma-voice] no match for: \"%s\"\n", text);
        unrecognised_count++;
        return -1;
    }

    fprintf(stdout, "[sigma-voice] matched: \"%s\" → %s::%s\n",
            cmd->phrase, cmd->bus_interface, cmd->bus_signal);

    // Dispatch via sigma-bus
    // sigma_bus_emit(bus, cmd->bus_interface, cmd->bus_signal, cmd->bus_body);
    // (Real impl: use the bus handle stored in VoiceControl)
    commands_dispatched++;
    return 0;
}

// ── STT via sigma-ai (Whisper.cpp at port 17392) ──────────────────────────
int VoiceControl::transcribe_audio(const float *pcm_samples,
                                    size_t n_samples,
                                    char *text_out, size_t max_len) {
    // Real impl: POST PCM to http://localhost:17392/v1/transcribe
    // Response JSON: {"text": "open terminal"}
    // For now: stub returns empty
    (void)pcm_samples; (void)n_samples;
    if (text_out && max_len > 0) text_out[0] = '\0';
    return 0;
}

// ── Lifecycle ─────────────────────────────────────────────────────────────
VoiceControl::VoiceControl()
    : enabled(false), wake_word_enabled(true),
      utterance_count(0), commands_dispatched(0), unrecognised_count(0)
{
    strncpy(wake_word, "sigma", sizeof(wake_word)-1);
}

void VoiceControl::enable() {
    enabled = true;
    fprintf(stdout, "[sigma-voice] voice control enabled (wake word: \"%s\")\n",
            wake_word);
}

void VoiceControl::disable() {
    enabled = false;
    fprintf(stdout, "[sigma-voice] voice control disabled\n");
}

void VoiceControl::set_wake_word(const char *word) {
    strncpy(wake_word, word, sizeof(wake_word)-1);
}

// ── List all available commands ───────────────────────────────────────────
void VoiceControl::list_commands(void (*cb)(const char *phrase, void *ctx),
                                   void *ctx) {
    for (int i = 0; COMMANDS[i].phrase; i++)
        cb(COMMANDS[i].phrase, ctx);
}

} // namespace sigma::a11y
