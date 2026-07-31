# XSS Prevention Guidelines

## Overview
Based on security learnings from .jules/sentinel.md, this document provides guidelines for preventing DOM-based XSS vulnerabilities in SigmaOS web interfaces.

## Vulnerability: DOM-based XSS in AI Web Interface

**Learning:** Legacy inline HTML generation with string concatenation easily leads to HTML injection when dealing with user-controlled inputs in browser environments.

## Prevention Guidelines

### 1. Use Safe DOM Manipulation Methods

**DO:使用安全的DOM操作方法**
```javascript
// Safe: Use textContent for plain text
element.textContent = userInput;

// Safe: Use innerText for plain text with layout awareness
element.innerText = userInput;
```

**DON'T:**
```javascript
// Unsafe: Direct innerHTML with untrusted data
element.innerHTML = userInput;
```

### 2. HTML Sanitization

When HTML rendering is necessary, always sanitize inputs first:

```javascript
import DOMPurify from 'dompurify';

// Sanitize before using innerHTML
const sanitized = DOMPurify.sanitize(userInput);
element.innerHTML = sanitized;
```

### 3. Template Literals with User Input

**DO:**
```javascript
// Safe: Use textContent with template literals
element.textContent = `Hello, ${userName}`;
```

**DON'T:**
```javascript
// Unsafe: Template literals in innerHTML
element.innerHTML = `<div>Hello, ${userName}</div>`;
```

### 4. Attribute Values

**DO:**
```javascript
// Safe: Set attributes directly
element.setAttribute('title', userInput);
```

**DON'T:**
```javascript
// Unsafe: Attribute values in innerHTML
element.innerHTML = `<div title="${userInput}">Content</div>`;
```

## Implementation Checklist

- [ ] Audit all `.innerHTML` usage in web shell and applications
- [ ] Replace unsafe innerHTML with textContent/innerText where possible
- [ ] Implement DOMPurify or similar sanitizer for necessary HTML rendering
- [ ] Add CI static analysis for innerHTML usage patterns
- [ ] Document approved HTML rendering patterns

## References

- Original learning from: .jules/sentinel.md (2026-07-12)
- OWASP XSS Prevention Cheat Sheet
- MDN Web Security Guide
