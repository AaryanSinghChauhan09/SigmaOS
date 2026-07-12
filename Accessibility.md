# Accessibility

SigmaOS is dedicated to providing a **Sovereign Experience for All**. We prioritize inclusive design and low-latency assistive technologies.

---

## Core Principles

When building new shards for SigmaOS, please adhere to:

### 1. Contrast Ratios
- Ensure a minimum 4.5:1 ratio for text
- 3:1 ratio for large text and UI components
- Support high-contrast mode for visually impaired users

### 2. Focus States
- All interactive elements must have a visible focal glow
- Keyboard navigation must be fully functional
- Focus indicators must be clearly visible in all themes

### 3. Semantic Sharding
- Use standard UI primitives to ensure screen reader compatibility
- Proper heading hierarchy (h1, h2, h3, etc.)
- Semantic HTML elements for all UI components

### 4. ARIA Labels
- Every `<input>` or interactive component must have a descriptive `aria-label`
- Live regions for dynamic content updates
- ARIA roles for custom components

---

## Built-in Accessibility Features

### Screen Reader Support
- Native screen reader integration with Orca, NVDA, and VoiceOver
- Real-time UI state announcements
- Braille display support via BRLTTY

### Keyboard Navigation
- Full keyboard navigation for all system functions
- Customizable keyboard shortcuts
- Sticky keys and filter keys support

### Visual Assistance
- System-wide magnification
- Color blindness compensation
- Reduced motion for vestibular disorders
- High contrast mode

### Audio Assistance
- Text-to-speech for system messages
- Visual alerts for audio cues
- Customizable sound schemes

---

## Development Guidelines

### Testing Requirements
- Test with screen readers before release
- Verify keyboard-only navigation
- Validate with accessibility checkers (axe, WAVE)
- User testing with disabled community members

### Documentation
- Document all keyboard shortcuts
- Provide accessibility conformance reports
- Include accessibility in changelog
- Maintain accessibility issue tracker

---

## Compliance Standards

SigmaOS aims to comply with:
- WCAG 2.1 Level AA
- Section 508 (US federal accessibility)
- EN 301 549 (European accessibility)
- ISO/IEC 40500 (international WCAG standard)
