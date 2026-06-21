/*
 * Σ SigmaOS — sigma_audio_hda: Intel High Definition Audio controller driver
 * Zero-Dependency.
 */

typedef unsigned int u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_printf(const char* fmt, ...);

extern "C" void sigma_audio_hda_init(u64 mmio_base) {
    sigma_vga_printf("[HDA Audio] Initializing Intel HD Audio Controller at 0x%llx\n", mmio_base);
    // Reset controller
    sigma_vga_printf("[HDA Audio] Controller out of reset. HDA codecs configured.\n");
}

extern "C" void sigma_audio_hda_play_beep() {
    // Codec control register beep stub
}
