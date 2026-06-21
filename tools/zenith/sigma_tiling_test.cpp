/**
 * Host / CI smoke test for Zenith tiling layout math (no kernel linkage).
 */
#include <cstdio>
#include <cstdint>

struct Rect {
    int32_t x, y;
    uint32_t w, h;
};

static void master_stack_layout(uint32_t screen_w, uint32_t screen_h, uint32_t count, Rect* out) {
    if (count == 0) return;
    uint32_t master_h = screen_h / 2;
    out[0] = {0, 0, screen_w, master_h};
    uint32_t stack_h = (count > 1) ? (screen_h - master_h) / (count - 1) : 0;
    for (uint32_t i = 1; i < count; i++) {
        out[i] = {0, (int32_t)(master_h + stack_h * (i - 1)), screen_w, stack_h};
    }
}

int main() {
    Rect frames[4];
    master_stack_layout(1920, 1080, 3, frames);
    std::printf("sigma_tiling_test: w0=%u h0=%u w1=%u y1=%d\n",
                frames[0].w, frames[0].h, frames[1].w, frames[1].y);
    return 0;
}
