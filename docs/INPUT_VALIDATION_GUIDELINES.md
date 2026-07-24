# Input Validation Guidelines

## Overview
Based on security learnings from .jules/sentinel.md, this document provides guidelines for preventing path traversal and injection attacks through proper input validation.

## Vulnerability: Input Path Traversal Prevention in Package Name

**Learning:** Raw input must always be vetted at the entry point of the operation rather than relying on sanitizers inside nested utility layers.

## Prevention Guidelines

### 1. Early Whitelist-Based Validation

**DO:**
```go
// Validate at entry point with strict whitelist
func ValidatePackageName(name string) error {
    // Allow only ASCII alphanumerics, dashes, and underscores
    matched, _ := regexp.MatchString(`^[a-zA-Z0-9\-_]+$`, name)
    if !matched {
        return fmt.Errorf("invalid package name: contains disallowed characters")
    }
    
    // Enforce length limits
    if len(name) < 1 || len(name) > 64 {
        return fmt.Errorf("package name must be 1-64 characters")
    }
    
    return nil
}
```

**DON'T:**
```go
// Unsafe: Rely on nested sanitizers
func ProcessPackage(name string) {
    // No validation at entry point
    sanitized := someNestedSanitizer(name)
    // Assumes sanitizer catches everything
}
```

### 2. Path Traversal Prevention

**DO:**
```go
import "path/filepath"

func ResolvePackagePath(baseDir, packageName string) (string, error) {
    // Validate package name first
    if err := ValidatePackageName(packageName); err != nil {
        return "", err
    }
    
    // Use filepath.Join to construct paths
    fullPath := filepath.Join(baseDir, packageName)
    
    // Verify the resolved path is within baseDir
    resolved, err := filepath.EvalSymlinks(fullPath)
    if err != nil {
        return "", err
    }
    
    if !strings.HasPrefix(resolved, baseDir) {
        return "", fmt.Errorf("path traversal attempt detected")
    }
    
    return resolved, nil
}
```

**DON'T:**
```go
// Unsafe: Direct path construction without validation
func ResolvePackagePath(baseDir, packageName string) string {
    return baseDir + "/" + packageName  // Vulnerable to ../
}
```

### 3. Shell Metacharacter Prevention

**DO:**
```go
import "os/exec"

func ExecutePackageCommand(pkgName, command string) error {
    if err := ValidatePackageName(pkgName); err != nil {
        return err
    }
    
    // Use exec.Command with separate arguments (no shell expansion)
    cmd := exec.Command(command, pkgName)
    return cmd.Run()
}
```

**DON'T:**
```go
// Unsafe: Shell command with user input
func ExecutePackageCommand(pkgName, command string) error {
    // Shell expansion allows command injection
    cmd := exec.Command("sh", "-c", fmt.Sprintf("%s %s", command, pkgName))
    return cmd.Run()
}
```

## Implementation Checklist

- [ ] Identify all entry points accepting user input
- [ ] Implement whitelist-based validators for each input type
- [ ] Add path traversal checks for all file system operations
- [ ] Use safe command execution (no shell expansion)
- [ ] Add unit tests for validation bypass attempts
- [ ] Document validation requirements for each input type

## Common Validation Patterns

### Package Names
```regex
^[a-zA-Z0-9\-_]{1,64}$
```

### File Paths
- Use `filepath.Join()` for path construction
- Verify resolved path is within expected directory
- Reject paths containing `..` segments

### URLs
```regex
^https?://[a-zA-Z0-9\-\.]+\.[a-zA-Z]{2,}(/.*)?$
```

### Email Addresses
```regex
^[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}$
```

## References

- Original learning from: .jules/sentinel.md (2026-07-14)
- CWE-22: Path Traversal
- CWE-78: OS Command Injection
- OWASP Input Validation Cheat Sheet
