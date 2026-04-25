#pragma once
#include <stdint.h>

// Track 2: Developer Needs - SDKs & APIs
namespace SigmaOS {
namespace SDK {

class SovereignAPI {
public:
    // Graphics Hook
    static void draw_window(uint32_t x, uint32_t y, uint32_t w, uint32_t h, const char* title);
    
    // Network Hook
    static int open_socket(uint32_t protocol);
    
    // Automation Hook
    static void schedule_task(void (*task_func)(), uint32_t delay_ms);
    
    // System Config
    static void apply_theme(const char* theme_name);
};

} // namespace SDK
} // namespace SigmaOS
