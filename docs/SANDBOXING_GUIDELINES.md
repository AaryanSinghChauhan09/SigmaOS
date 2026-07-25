# Application Sandboxing Guidelines

## Overview
Based on security learnings from .jules/sentinel.md, this document provides guidelines for sandboxing application launchers to prevent privilege escalation and persistence attacks.

## Vulnerability: Desktop Launcher Capability Sandboxing

**Learning:** Application launchers are a prime pivot point for userland persistence. The desktop launcher must never blindly exec processes.

## Prevention Guidelines

### 1. Manifest-Based Capability Declaration

**DO:**
```json
{
  "name": "calculator",
  "version": "1.0.0",
  "entry": "/usr/bin/calculator",
  "capabilities": [
    "display",
    "audio.playback"
  ],
  "filesystem": {
    "read": ["/home/user/Documents"],
    "write": ["/home/user/.config/calculator"]
  }
}
```

**DON'T:**
```json
// Unsafe: No capability restrictions
{
  "name": "calculator",
  "entry": "/usr/bin/calculator"
  // Missing capabilities - would get full user privileges
}
```

### 2. Sandbox Integration with Launchers

**DO:**
```go
type ApplicationLauncher struct {
    sandboxManager *SandboxManager
}

func (l *ApplicationLauncher) Launch(manifest *AppManifest) error {
    // Parse manifest capabilities
    capabilities := manifest.Capabilities

    // Build sandbox configuration
    sandboxConfig := l.sandboxManager.BuildConfig(capabilities)

    // Launch in sandbox
    return l.sandboxManager.Execute(manifest.Entry, sandboxConfig)
}
```

**DON'T:**
```go
// Unsafe: Direct exec without sandbox
func (l *ApplicationLauncher) Launch(manifest *AppManifest) error {
    // Blindly execute with full user privileges
    return exec.Command(manifest.Entry).Run()
}
```

### 3. Bubblewrap Sandbox Configuration

**DO:**
```go
func BuildBwrapCommand(manifest *AppManifest) []string {
    cmd := []string{
        "bwrap",
        "--ro-bind", "/usr", "/usr",
        "--ro-bind", "/lib", "/lib",
        "--ro-bind", "/lib64", "/lib64",
        "--tmpfs", "/tmp",
        "--unshare-all",
        "--die-with-parent",
    }

    // Add network capability if requested
    if manifest.HasCapability("network") {
        cmd = append(cmd, "--share-net")
    } else {
        cmd = append(cmd, "--unshare-net")
    }

    // Add filesystem bindings from manifest
    for _, path := range manifest.Filesystem.Read {
        cmd = append(cmd, "--ro-bind", path, path)
    }

    for _, path := range manifest.Filesystem.Write {
        cmd = append(cmd, "--bind", path, path)
    }

    // Add device access if requested
    if manifest.HasCapability("audio") {
        cmd = append(cmd, "--dev-bind", "/dev/snd", "/dev/snd")
    }

    cmd = append(cmd, manifest.Entry)
    return cmd
}
```

### 4. Capability Enforcement

**DO:**
```go
type Capability string

const (
    CapDisplay    Capability = "display"
    CapAudio      Capability = "audio"
    CapNetwork    Capability = "network"
    CapCamera     Capability = "camera"
    CapMicrophone Capability = "microphone"
    CapFilesystem Capability = "filesystem"
)

func ValidateCapabilities(caps []Capability) error {
    allowedCaps := map[Capability]bool{
        CapDisplay:    true,
        CapAudio:      true,
        CapNetwork:    true,
        CapCamera:     true,
        CapMicrophone: true,
        CapFilesystem: true,
    }

    for _, cap := range caps {
        if !allowedCaps[cap] {
            return fmt.Errorf("unknown capability: %s", cap)
        }
    }

    return nil
}
```

### 5. Desktop Launcher Integration

**DO:**
```javascript
// sigma_desktop Dash integration
class ApplicationLauncher {
    async launchApp(appId) {
        const manifest = await this.getAppManifest(appId);

        // Request capability approval from user
        const approved = await this.requestCapabilityApproval(manifest);
        if (!approved) {
            throw new Error('User denied capability request');
        }

        // Launch through sandbox manager
        const result = await navigator.sigmaos.sandbox.execute(manifest);

        if (result.success) {
            this.showNotification(`Launched ${manifest.name}`);
        } else {
            this.showError(`Failed to launch: ${result.error}`);
        }
    }

    async requestCapabilityApproval(manifest) {
        // Show capability request dialog
        const dialog = new CapabilityDialog(manifest);
        return dialog.show();
    }
}
```

**DON'T:**
```javascript
// Unsafe: Direct execution without sandbox
async launchApp(appId) {
    const manifest = await this.getAppManifest(appId);

    // Blindly execute - vulnerable to malicious .desktop files
    await navigator.sigmaos.process.exec(manifest.entry);
}
```

### 6. Malicious .desktop File Prevention

**DO:**
```go
func ParseDesktopFile(path string) (*AppManifest, error) {
    // Parse .desktop file
    content, err := os.ReadFile(path)
    if err != nil {
        return nil, err
    }

    manifest := &AppManifest{}

    // Extract Exec line
    for _, line := range strings.Split(string(content), "\n") {
        if strings.HasPrefix(line, "Exec=") {
            // Validate command is safe
            cmd := strings.TrimPrefix(line, "Exec=")
            if strings.Contains(cmd, "&&") || strings.Contains(cmd, ";") {
                return nil, fmt.Errorf("malicious .desktop file: command chaining detected")
            }
            manifest.Entry = sanitizeCommand(cmd)
        }
        // ... other fields
    }

    // Require manifest file for capability declaration
    manifestPath := strings.Replace(path, ".desktop", ".json", 1)
    if _, err := os.Stat(manifestPath); os.IsNotExist(err) {
        return nil, fmt.Errorf("missing capability manifest: %s", manifestPath)
    }

    return manifest, nil
}
```

## Implementation Checklist

- [ ] Integrate desktop launcher with sandbox manager
- [ ] Require capability manifest for all applications
- [ ] Implement user approval for capability requests
- [ ] Validate .desktop files for malicious patterns
- [ ] Enforce namespace isolation (PID, net, mount, IPC)
- [ ] Add seccomp syscall filtering
- [ ] Log capability violations to audit log
- [ ] Add unit tests for sandbox bypass attempts

## Capability Reference

### Display
- Allows access to Wayland/X11 display server
- Required for GUI applications

### Audio
- Allows audio playback/recording
- Requires device access to `/dev/snd`

### Network
- Allows network access
- Can be restricted to specific ports/protocols

### Camera
- Allows camera access
- Requires device access to video devices

### Microphone
- Allows microphone access
- Requires device access to audio input devices

### Filesystem
- Allows filesystem access
- Must specify read/write paths explicitly

## References

- Original learning from: .jules/sentinel.md (2026-07-14)
- Bubblewrap Documentation
- Linux Namespacespaces
- Seccomp Filter Documentation
- Flatpak Security Model
