// SigmaOS — sigma-ui-theme-loader: Theme Management
// Module: sigma-ui-theme-loader
// USP: Parses binary theme structures instead of JSON/CSS for instant zero-copy theme loads.

#ifndef SIGMA_UI_THEME_LOADER_H
#define SIGMA_UI_THEME_LOADER_H

namespace sigma {
namespace ui {

struct ColorRGBA {
    unsigned char r, g, b, a;
};

// Binary-packed theme definition
struct BinaryTheme {
    ColorRGBA primary_bg;
    ColorRGBA secondary_bg;
    ColorRGBA primary_fg;
    ColorRGBA accent;
    float     base_blur;
    unsigned int font_id;
};

class ThemeLoader {
private:
    BinaryTheme current_theme;

public:
    ThemeLoader() {
        // Default Dark Theme
        current_theme.primary_bg = { 10, 10, 12, 255 };
        current_theme.secondary_bg = { 20, 20, 24, 255 };
        current_theme.primary_fg = { 240, 240, 240, 255 };
        current_theme.accent = { 0, 120, 255, 255 };
        current_theme.base_blur = 15.0f;
        current_theme.font_id = 0;
    }

    void load_from_memory(const unsigned char* raw_data, unsigned int size) {
        if (size < sizeof(BinaryTheme)) return;
        // Zero-copy binary struct mapping (assuming endianness matches)
        const BinaryTheme* mapped = reinterpret_cast<const BinaryTheme*>(raw_data);
        current_theme = *mapped;
    }

    const BinaryTheme& get_theme() const { return current_theme; }
};

} // namespace ui
} // namespace sigma

#endif /* SIGMA_UI_THEME_LOADER_H */
