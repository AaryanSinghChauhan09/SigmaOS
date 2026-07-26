# UI/UX Best Practices

## Overview
Based on UX/UI learnings from .Jules/palette.md, this document provides guidelines for creating accessible, performant, and delightful user interfaces in SigmaOS.

## Keyboard Navigation and Focus Indicators

### Learning: Keyboard navigation (WCAG 2.1 Level AA) requires highly visible focus indicators to distinguish focused controls from surrounding elements.

### Implementation

**DO:**
```css
/* High-contrast focus indicators */
:focus-visible {
    outline: 3px solid var(--accent-color);
    outline-offset: 3px;
    box-shadow: 0 0 0 6px rgba(255, 255, 255, 0.2),
                0 0 15px var(--accent-glow);
}

/* High-contrast mode support */
body.high-contrast-active :focus-visible {
    outline: 4px solid #ffffff;
    outline-offset: 4px;
    box-shadow: 0 0 0 8px #000000,
                0 0 0 12px #ffffff;
}
```

**DON'T:**
```css
/* Insufficient contrast in glassmorphic UI */
:focus {
    outline: 1px solid rgba(255, 255, 255, 0.3);
}
```

## Empty States and Actionable CTAs

### Learning: When queries return empty lists, users are confused about next actions. Color-coded warnings with protips reduce friction.

### Implementation

**DO:**
```javascript
function renderEmptyState(query, container) {
    container.innerHTML = `
        <div class="empty-state">
            <div class="empty-state-icon">🔍</div>
            <h3>No results found for "${query}"</h3>
            <p class="warning-text">Your search didn't match any items</p>
            <div class="protip">
                <strong>💡 Protip:</strong> Try different keywords,
                check your spelling, or browse categories
            </div>
            <button class="action-button">
                Browse All Categories
            </button>
        </div>
    `;
}
```

**DON'T:**
```javascript
// Confusing: No guidance
function renderEmptyState(query, container) {
    container.innerHTML = `<p>No results</p>`;
}
```

## Compositor Damage Tracking

### Learning: Blindly redrawing entire screen wastes GPU cycles. Track damaged surface IDs to limit redraws.

### Implementation

**DO:**
```rust
use std::collections::HashSet;

pub struct Compositor {
    damaged_surfaces: HashSet<u64>,
}

impl Compositor {
    pub fn mark_damaged(&mut self, surface_id: u64) {
        self.damaged_surfaces.insert(surface_id);
    }

    pub fn composite_frame(&mut self) {
        // Atomically drain damaged set
        let damaged: Vec<u64> = self.damaged_surfaces.drain().collect();

        // Only composite damaged surfaces
        for surface_id in damaged {
            self.composite_surface(surface_id);
        }
    }
}
```

**DON'T:**
```rust
// Wasteful: Redraws everything every frame
pub fn composite_frame(&mut self) {
    for surface_id in 0..MAX_SURFACES {
        self.composite_surface(surface_id);
    }
}
```

## Predictable Spatial Models in Desktop Panels

### Learning: Scattering system tray icons and launchers haphazardly reduces spatial memory retention.

### Implementation

**DO:**
```css
/* Fixed spatial anchoring */
.panel {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.launcher-group {
    order: 1;
    justify-self: flex-start;
}

.clock-group {
    order: 2;
    justify-self: center;
}

.tray-group {
    order: 3;
    justify-self: flex-end;
}
```

**DON'T:**
```css
/* Unpredictable positioning */
.panel {
    display: flex;
    /* Elements shift position based on content */
}
```

## Neural UI Asynchronous Loading States

### Learning: AI inference blocking main thread makes OS feel sluggish. Use skeleton UI and background threads.

### Implementation

**DO:**
```javascript
class NeuralUI {
    async predictLayout(context) {
        // Show skeleton UI immediately
        this.showSkeleton();

        // Run inference in background
        const prediction = await this.runInference(context);

        // Swap layout when ready
        this.applyLayout(prediction);
        this.hideSkeleton();
    }

    showSkeleton() {
        this.container.innerHTML = `
            <div class="skeleton-loader">
                <div class="skeleton-block"></div>
                <div class="skeleton-block"></div>
                <div class="skeleton-block"></div>
            </div>
        `;
    }
}
```

**DON'T:**
```javascript
// Blocks UI during inference
async predictLayout(context) {
    const prediction = await this.runInference(context);
    this.applyLayout(prediction);
}
```

## Implementation Checklist

- [ ] Add high-contrast `:focus-visible` styles to all interactive elements
- [ ] Implement high-contrast mode fallback support
- [ ] Add delightful empty states with actionable protips
- [ ] Implement damage tracking in compositor
- [ ] Enforce fixed spatial anchoring in desktop panels
- [ ] Use skeleton UI for asynchronous operations
- [ ] Test keyboard navigation throughout the UI
- [ ] Verify WCAG 2.1 Level AA compliance

## Accessibility Standards

### WCAG 2.1 Level AA Requirements

- **Focus Indicators**: Visible focus indicators on all interactive elements
- **Color Contrast**: Minimum 4.5:1 for normal text, 3:1 for large text
- **Keyboard Navigation**: All functionality available via keyboard
- **Error Identification**: Clear error messages and suggestions
- **Resize Text**: Text scales up to 200% without loss of content

### Testing Checklist

- [ ] Navigate entire interface using keyboard only
- [ ] Test with screen reader (NVDA, VoiceOver)
- [ ] Verify color contrast using accessibility tools
- [ ] Test with high-contrast mode enabled
- [ ] Verify focus order is logical
- [ ] Test empty states with various scenarios

## References

- Original learnings from: .Jules/palette.md (2026-07-12 to 2026-07-14)
- WCAG 2.1 Guidelines
- Wayland Damage Tracking Documentation
- Material Design Empty States
