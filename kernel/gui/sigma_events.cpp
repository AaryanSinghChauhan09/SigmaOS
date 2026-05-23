/*
 * Σ SigmaOS — sigma_events: Zenith Event Routing System
 * Zero-Dependency: Coordinates between hardware drivers (mouse, keyboard) and GUI Widgets.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);

/* Input Event Types */
#define EVENT_MOUSE_CLICK  1
#define EVENT_MOUSE_MOVE   2
#define EVENT_KEY_PRESS    3
#define EVENT_KEY_RELEASE  4

struct ZenithEvent {
    u32 type;
    u32 x;       /* Mouse X */
    u32 y;       /* Mouse Y */
    u32 keycode; /* Keyboard Scancode */
};

/* Definition from sigma_widgets.cpp */
struct ZenithWidget {
    u32 type;
    u32 x, y, width, height;
    u32 bg_color;
    u32 fg_color;
    const char* text;
    void (*on_click)();
    ZenithWidget* next;
};
extern ZenithWidget* root_widget; /* Expected from widgets module */

/*
 * Dispatch an event from the hardware interrupt layer into the GUI
 */
extern "C" void zenith_dispatch_event(ZenithEvent* event) {
    if (event->type == EVENT_MOUSE_CLICK) {
        sigma_vga_printf("[GUI] Mouse Click at %d, %d\n", event->x, event->y);
        
        /* Hit testing against widget tree */
        ZenithWidget* curr = root_widget;
        while (curr) {
            if (event->x >= curr->x && event->x <= (curr->x + curr->width) &&
                event->y >= curr->y && event->y <= (curr->y + curr->height)) {
                
                /* Hit! Fire callback */
                if (curr->on_click) {
                    curr->on_click();
                }
                break;
            }
            curr = curr->next;
        }
    } else if (event->type == EVENT_KEY_PRESS) {
        sigma_vga_printf("[GUI] Key pressed: %X\n", event->keycode);
        /* Route to active focused widget */
    }
}
