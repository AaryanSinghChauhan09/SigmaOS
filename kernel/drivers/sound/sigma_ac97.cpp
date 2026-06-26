/*
 * Σ SigmaOS Zenith — AC97 Sound Driver Shard
 * Absorbs: Linux sound/pci/ac97, ALSA ac97_codec.c
 * Zero-Dependency: No libc, no ALSA.
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_printf(const char* fmt, ...);

/* ─────────── Port I/O ─────────── */
static inline void sigma_outw(u16 port, u16 val) {
    __asm__ volatile("outw %0, %1" : : "a"(val), "Nd"(port));
}
static inline u16 sigma_inw(u16 port) {
    u16 val;
    __asm__ volatile("inw %1, %0" : "=a"(val) : "Nd"(port));
    return val;
}
static inline void sigma_outl(u16 port, u32 val) {
    __asm__ volatile("outl %0, %1" : : "a"(val), "Nd"(port));
}
static inline u32 sigma_inl(u16 port) {
    u32 val;
    __asm__ volatile("inl %1, %0" : "=a"(val) : "Nd"(port));
    return val;
}

/* ─────────── AC97 Mixer Registers ─────────── */
#define AC97_RESET          0x00
#define AC97_MASTER_VOL     0x02
#define AC97_PCM_OUT_VOL    0x18
#define AC97_SAMPLE_RATE    0x2C

/* ─────────── AC97 Bus Master Registers ─────────── */
#define AC97_NABM_PCM_OUT   0x10  // PCM Out offset in NABM bar
#define AC97_NABM_BDBAR     0x00  // Buffer Descriptor Base Address
#define AC97_NABM_CIV       0x04  // Current Index Value
#define AC97_NABM_LVI       0x05  // Last Valid Index
#define AC97_NABM_STATUS    0x06  // Status
#define AC97_NABM_CTRL      0x0B  // Control

/* Buffer Descriptor Entry */
struct __attribute__((packed)) ac97_bd_entry {
    u32 buffer_addr;
    u16 sample_count;
    u16 control;
};

#define AC97_BD_IOC  (1 << 15)  // Interrupt on Completion
#define AC97_BD_BUP  (1 << 14)  // Buffer Underrun Policy

static u16 mixer_base  = 0;
static u16 nabm_base   = 0;

static struct ac97_bd_entry bd_list[32] __attribute__((aligned(8)));

extern "C" bool sigma_ac97_init(u16 mixer_bar, u16 nabm_bar) {
    mixer_base = mixer_bar;
    nabm_base  = nabm_bar;

    // 1. Reset the codec
    sigma_outw(mixer_base + AC97_RESET, 0x0000);

    // 2. Set master volume to max (0x0000 = 0dB)
    sigma_outw(mixer_base + AC97_MASTER_VOL, 0x0000);
    sigma_outw(mixer_base + AC97_PCM_OUT_VOL, 0x0808);

    // 3. Set sample rate to 44100 Hz
    sigma_outw(mixer_base + AC97_SAMPLE_RATE, 44100);

    u16 actual_rate = sigma_inw(mixer_base + AC97_SAMPLE_RATE);
    sigma_vga_printf("AC97: Sample rate set to %u Hz\n", actual_rate);

    sigma_vga_printf("AC97: Initialized (Mixer=0x%x, NABM=0x%x)\n",
        mixer_base, nabm_base);
    return true;
}

extern "C" void sigma_ac97_set_volume(u16 left, u16 right) {
    // Volume: 0x00 = max, 0x1F = min, bit 15 = mute
    u16 val = ((left & 0x1F) << 8) | (right & 0x1F);
    sigma_outw(mixer_base + AC97_MASTER_VOL, val);
}

extern "C" bool sigma_ac97_play(u32 buffer_phys, u32 sample_count) {
    if (!nabm_base) return false;

    u16 pcm_out = nabm_base + AC97_NABM_PCM_OUT;

    // Stop DMA first
    u8 ctrl = 0;
    __asm__ volatile("outb %0, %1" : : "a"(ctrl), "Nd"((u16)(pcm_out + AC97_NABM_CTRL)));

    // Set up buffer descriptor list entry 0
    bd_list[0].buffer_addr  = buffer_phys;
    bd_list[0].sample_count = (u16)(sample_count & 0xFFFF);
    bd_list[0].control      = AC97_BD_IOC;

    // Write BDBAR
    sigma_outl(pcm_out + AC97_NABM_BDBAR, (u32)(u64)bd_list);

    // Set LVI to 0 (only 1 entry)
    __asm__ volatile("outb %0, %1" : : "a"((u8)0), "Nd"((u16)(pcm_out + AC97_NABM_LVI)));

    // Start DMA (bit 0 = run, bit 1 = reset)
    ctrl = 0x01;
    __asm__ volatile("outb %0, %1" : : "a"(ctrl), "Nd"((u16)(pcm_out + AC97_NABM_CTRL)));

    sigma_vga_printf("AC97: Playing %u samples from 0x%x\n", sample_count, buffer_phys);
    return true;
}
