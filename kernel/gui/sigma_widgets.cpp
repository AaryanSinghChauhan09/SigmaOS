/*
 * Σ SigmaOS — sigma_widgets: Zenith Framebuffer Widget Toolkit
 * Zero-Dependency: Bypasses GTK, Qt, and X11 entirely.
 * Renders primitive UI elements directly via the Framebuffer driver.
 */

typedef unsigned int   u32;
typedef unsigned short u16;
typedef unsigned char  u8;

extern "C" void sigma_vga_printf(const char* fmt, ...);

/* Basic Framebuffer API (expected to be provided by sigma_fb.cpp) */
extern "C" void fb_draw_rect(u32 x, u32 y, u32 w, u32 h, u32 color);
extern "C" void fb_draw_text(u32 x, u32 y, const char* text, u32 color);

/* Widget Types */
#define WIDGET_BUTTON 1
#define WIDGET_WINDOW 2
#define WIDGET_LABEL  3

struct ZenithWidget {
    u32 type;
    u32 x, y, width, height;
    u32 bg_color;
    u32 fg_color;
    const char* text;
    void (*on_click)();
    ZenithWidget* next;
};

static ZenithWidget* root_widget = 0;

/*
 * Create a simple push button
 */
extern "C" ZenithWidget* zenith_create_button(u32 x, u32 y, u32 w, u32 h, const char* text, void (*callback)()) {
    /* Stubbed allocator call */
    // ZenithWidget* btn = (ZenithWidget*)sigma_malloc(sizeof(ZenithWidget));
    static ZenithWidget btn_stub; /* Static allocation for stub */
    ZenithWidget* btn = &btn_stub;
    
    btn->type = WIDGET_BUTTON;
    btn->x = x;
    btn->y = y;
    btn->width = w;
    btn->height = h;
    btn->bg_color = 0xCCCCCC; /* Light gray */
    btn->fg_color = 0x000000; /* Black text */
    btn->text = text;
    btn->on_click = callback;
    btn->next = root_widget;
    root_widget = btn;
    
    return btn;
}

/*
 * Traverse the widget tree and render to framebuffer
 */
extern "C" void zenith_render_all() {
    ZenithWidget* curr = root_widget;
    while (curr) {
        if (curr->type == WIDGET_BUTTON) {
            fb_draw_rect(curr->x, curr->y, curr->width, curr->height, curr->bg_color);
            fb_draw_text(curr->x + 10, curr->y + 10, curr->text, curr->fg_color);
        }
        curr = curr->next;
    }
}
