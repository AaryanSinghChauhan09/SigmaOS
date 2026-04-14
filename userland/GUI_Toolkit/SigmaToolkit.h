#ifndef SIGMA_GUI_TOOLKIT_H
#define SIGMA_GUI_TOOLKIT_H

#include <stdint.h>

// SigmaOS Native GUI Toolkit Wrapper
// Absorbing the object-oriented structure of Qt and the styling flexibility of GTK

typedef struct {
    uint32_t widget_id;
    uint32_t width;
    uint32_t height;
    const char* theme_class;
} Widget;

Widget* ui_create_window(uint32_t w, uint32_t h, const char* title);
void ui_apply_theme(Widget* widget, const char* css_style);
void ui_render(Widget* widget);
void ui_set_event_listener(Widget* widget, void (*event_handler)(uint32_t event_type));

#endif // SIGMA_GUI_TOOLKIT_H
