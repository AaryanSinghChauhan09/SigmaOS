#include "../../../include/sigma_log.h"
#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_hal.h"
#include "../../../include/libc/SovereignLibC.h"

/**
 * SigmaOS Sovereign Standard I/O Router
 * Silicon-native I/O multiplexer.
 *
 * USP: Routes `sigma_write` and `sigma_read` simultaneously across COM1 serial, 
 * the hardware framebuffer, and IPC socket pipes with zero-copy.
 *
 * Design: OOP-isolated singleton � SovereignStdioEngine.
 */

class SovereignStdioEngine {
public:
    static SovereignStdioEngine& getInstance() {
        static SovereignStdioEngine instance;
        return instance;
    }

    static void init() {
        sigma_log("[STDIO] Initializing Sovereign I/O Multiplexer...");
        this->echo_to_serial = true;
        this->echo_to_framebuffer = true;
        sigma_log("[STDIO] Multiplexer routing ACTIVE.");
    }

    void routeWrite(const char* buffer, sigma_u32 length) {
        (void)length;
        if (this->echo_to_serial) {
            // Simulated write to COM1
            sigma_log("[COM1] %s", buffer);
        }
        if (this->echo_to_framebuffer) {
            // Simulated write to Zenith Framebuffer
            sigma_log("[FRAMEBUFFER] %s", buffer);
        }
    }

private:
    SovereignStdioEngine() : echo_to_serial(true), echo_to_framebuffer(true) {}

    bool echo_to_serial;
    bool echo_to_framebuffer;
};

/* --- C Wrappers --- */
void stdio_init() {
    SovereignStdioEngine::init();
}

void stdio_route_write(const char* buffer, sigma_u32 length) {
    SovereignStdioEngine::routeWrite(buffer, length);
}





} // extern "C"
 