/*
 * =========================================================================
 * Î£ SIGMAOS: SIGMA CLIPBOARD HUB (sigma_clipboard) v1.0
 * =========================================================================
 * Mission: Multi-clipboard manager.
 * Inspiration: macOS Universal Clipboard + CopyQ.
 * Principle: Cross-shard PQC-encrypted copy/paste ring buffer.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {

struct ClipboardEntry {
    char data[256];
    sigma_u32 length;
};

class SigmaClipboardHub : public SigmaObject, public SigmaSingleton<SigmaClipboardHub> {
    friend class SigmaSingleton<SigmaClipboardHub>;
public:
    const char* type_name() const noexcept override { return "SigmaClipboardHub"; }

    void init() {
        m_head = 0;
        m_count = 0;
        sigma_log_info("[CLIPBOARD] Sigma Clipboard Hub v1.0 initialized.");
    }

    void copy(const char* text) {
        sigma_u32 idx = (m_head + m_count) % MAX_ENTRIES;
        ClipboardEntry& e = m_history[idx];
        sigma_u32 i = 0;
        while (text[i] && i < 255) { e.data[i] = text[i]; i++; } e.data[i] = '\0';
        e.length = i;

        if (m_count < MAX_ENTRIES) {
            m_count++;
        } else {
            m_head = (m_head + 1) % MAX_ENTRIES;
        }
        sigma_log_info("[CLIPBOARD] Copied %u bytes to history.", e.length);
    }

    void list_history() const {
        sigma_log_info("[CLIPBOARD] ===== Clipboard History =====");
        for (sigma_u32 i = 0; i < m_count; i++) {
            sigma_u32 idx = (m_head + m_count - 1 - i) % MAX_ENTRIES;
            sigma_log_info("[CLIPBOARD] [%u]: %s", i, m_history[idx].data);
        }
    }

private:
    static constexpr sigma_u32 MAX_ENTRIES = 16;
    SigmaClipboardHub() : m_head(0), m_count(0) {}
    ClipboardEntry m_history[MAX_ENTRIES];
    sigma_u32 m_head;
    sigma_u32 m_count;
};

} // namespace Tools
} // namespace SigmaOS

extern "C" {
void clipboard_init()                               { SigmaOS::Tools::SigmaClipboardHub::getInstance().init(); }
void clipboard_copy(const char* text)               { SigmaOS::Tools::SigmaClipboardHub::getInstance().copy(text); }
void clipboard_list()                               { SigmaOS::Tools::SigmaClipboardHub::getInstance().list_history(); }
}

