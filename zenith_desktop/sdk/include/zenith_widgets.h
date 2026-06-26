/*
 * =========================================================================
 * Σ SIGMAOS: ZENITH WIDGETS
 * =========================================================================
 * Native C++ UI toolkit primitives, bypassing Qt and GTK entirely.
 * Built for the Sovereign Compositor.
 * =========================================================================
 */

#ifndef ZENITH_WIDGETS_H
#define ZENITH_WIDGETS_H

#include <string>
#include <vector>
#include <functional>

namespace zenith {

struct Rect {
    int x, y, width, height;
};

class Widget {
protected:
    Rect bounds;
    bool visible = true;
    Widget* parent = nullptr;
    std::vector<Widget*> children;
    
public:
    Widget(int x, int y, int w, int h) : bounds{x, y, w, h} {}
    virtual ~Widget() = default;
    
    virtual void Draw() = 0; // Implemented by compositor backends
    
    void AddChild(Widget* child) {
        child->parent = this;
        children.push_back(child);
    }
};

class Button : public Widget {
    std::string label;
    std::function<void()> on_click;
public:
    Button(int x, int y, int w, int h, const std::string& text) 
        : Widget(x, y, w, h), label(text) {}
        
    void SetOnClick(std::function<void()> cb) { on_click = cb; }
    
    void Draw() override {
        /* TODO: Direct drawing call to Sovereign framebuffer */
    }
};

class Window : public Widget {
    std::string title;
public:
    Window(int x, int y, int w, int h, const std::string& title) 
        : Widget(x, y, w, h), title(title) {}
        
    void Draw() override {
        /* TODO: Draw window decorations and children */
    }
};

} // namespace zenith

#endif /* ZENITH_WIDGETS_H */
