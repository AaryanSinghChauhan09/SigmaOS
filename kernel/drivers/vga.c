/* 
 Σ SIGMAOS ZENITH: SOVEREIGN VIDEO SUBSYSTEM (v2700.0)
 Mission: Hardware-Direct VGA/VESA Text Buffer Orchestration.
*/

#include "../sigma_kernel_types.h"
#include "../SigmaSovereignInternal.h"

#define VGA_BUFFER 0xB8000
#define VGA_WIDTH 80
#define VGA_HEIGHT 25

static uint16_t* g_VideoBuffer = (uint16_t*)VGA_BUFFER;
static int g_CursorX = 0;
static int g_CursorY = 0;

// Σ SOVEREIGN PRINT
void sigma_print(const char* s) {
    while (*s) {
        if (*s == '\n') {
            g_CursorX = 0;
            g_CursorY++;
        } else {
            int index = g_CursorY * VGA_WIDTH + g_CursorX;
            g_VideoBuffer[index] = (uint16_t)*s | (0x07 << 8); // White on Black
            g_CursorX++;
        }
        
        if (g_CursorX >= VGA_WIDTH) {
            g_CursorX = 0;
            g_CursorY++;
        }
        s++;
    }
}

// Σ SOVEREIGN CLEAR
void sigma_clear_screen() {
    for (int i = 0; i < VGA_WIDTH * VGA_HEIGHT; i++) {
        g_VideoBuffer[i] = (uint16_t)' ' | (0x07 << 8);
    }
    g_CursorX = 0;
    g_CursorY = 0;
}
