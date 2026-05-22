// sigma_doom.c
// Minimal integration code for Doom port on SigmaOS
// Emulates generic keyboard and video hooks

#include "../../include/sigma_syscall.h"
#include "../../include/sigma_framebuffer.h"

// System call stubs for userland
static long syscall(int sysno, long arg1, long arg2, long arg3, long arg4, long arg5, long arg6) {
    long ret;
    __asm__ volatile (
        "mov %1, %%rax\n"
        "mov %2, %%rdi\n"
        "mov %3, %%rsi\n"
        "mov %4, %%rdx\n"
        "mov %5, %%r10\n"
        "mov %6, %%r8\n"
        "mov %7, %%r9\n"
        "int $0x80\n"
        "mov %%rax, %0\n"
        : "=r" (ret)
        : "g" (sysno), "g" (arg1), "g" (arg2), "g" (arg3), "g" (arg4), "g" (arg5), "g" (arg6)
        : "rax", "rdi", "rsi", "rdx", "r10", "r8", "r9", "memory"
    );
    return ret;
}

// ---------------------------------------------------------
// DOOMGeneric required hooks
// ---------------------------------------------------------

static struct sigma_fb_info current_fb;
static unsigned int* fb_ptr = 0;

void DG_Init() {
    // 1. Open framebuffer (assuming fd 3 is /dev/fb0 for SigmaOS Doom demo parity)
    int fb_fd = 3; 

    // 2. IOCTL to get FB info
    syscall(35 /* SYS_IOCTL */, fb_fd, SIGMA_IOCTL_FB_GET_INFO, (long)&current_fb, 0, 0, 0);

    // 3. MMAP the framebuffer
    fb_ptr = (unsigned int*)syscall(10 /* SYS_MMAP */, 0, current_fb.width * current_fb.height * 4, 3 /* PROT_READ|WRITE */, 1 /* MAP_SHARED */, fb_fd, 0);
}

void DG_DrawFrame(unsigned int* pixel_buffer) {
    if (!fb_ptr) return;
    
    // Copy the pixel buffer into the framebuffer memory mapping
    unsigned int num_pixels = current_fb.width * current_fb.height;
    for (unsigned int i = 0; i < num_pixels; i++) {
        fb_ptr[i] = pixel_buffer[i];
    }
}

void DG_SleepMs(uint32_t ms) {
    syscall(13 /* SYS_SLEEP */, ms, 0, 0, 0, 0, 0);
}

uint32_t DG_GetTicksMs() {
    long ms = syscall(38 /* SYS_GETTIMEOFDAY */, 0, 0, 0, 0, 0, 0);
    return (uint32_t)ms;
}

int DG_GetKey(int* pressed, unsigned char* doomKey) {
    // Read from standard input (fd 0) which is wired to our PS/2 keyboard buffer
    char key;
    long bytes_read = syscall(2 /* SYS_READ */, 0, (long)&key, 1, 0, 0, 0);
    
    if (bytes_read == 1) {
        *pressed = 1; // Assuming keydown for simplicity
        *doomKey = (unsigned char)key;
        return 1;
    }
    return 0; // No keys pending
}
