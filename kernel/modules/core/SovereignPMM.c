#include "SovereignPMM.h"

static sigma_u64 s_frame_bitmap[FRAME_BITMAP_WORDS]; /* 1=free, 0=used */
static sigma_u32 s_free_frames = (sigma_u32)TOTAL_FRAMES;
static sigma_u32 s_next_frame_hint = 0;

void pmm_init(void) {
    /* Mark all frames free */
    for (sigma_u32 i = 0; i < FRAME_BITMAP_WORDS; i++)
        s_frame_bitmap[i] = ~0ULL;
    /* Reserve first 2 MB (kernel identity map) */
    for (sigma_u32 i = 0; i < 8; i++)
        s_frame_bitmap[i] = 0;
    s_free_frames = (sigma_u32)TOTAL_FRAMES - 512;
    sigma_printf("Σ [PMM]: %u frames available (%u MB)\n",
                 s_free_frames, s_free_frames * 4 / 1024);
}

sigma_u64 pmm_alloc_frame(void) {
    for (sigma_u32 word = s_next_frame_hint / 64;
         word < FRAME_BITMAP_WORDS; word++) {
        if (s_frame_bitmap[word] == 0) continue;
        for (int bit = 0; bit < 64; bit++) {
            if ((s_frame_bitmap[word] >> bit) & 1ULL) {
                s_frame_bitmap[word] &= ~(1ULL << bit);
                s_free_frames--;
                sigma_u64 frame = (sigma_u64)(word * 64 + bit);
                s_next_frame_hint = (sigma_u32)frame + 1;
                return frame * PAGE_SIZE; /* physical address */
            }
        }
    }
    return 0; /* out of memory */
}

void pmm_free_frame(sigma_u64 phys) {
    sigma_u64 frame = phys / PAGE_SIZE;
    sigma_u64 word  = frame / 64;
    sigma_u64 bit   = frame % 64;
    if (word < FRAME_BITMAP_WORDS && !((s_frame_bitmap[word] >> bit) & 1)) {
        s_frame_bitmap[word] |= (1ULL << bit);
        s_free_frames++;
    }
}

sigma_u32 pmm_get_free_count(void) {
    return s_free_frames;
}
