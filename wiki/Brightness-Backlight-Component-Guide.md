# Brightness Backlight & DDC/CI Hardware Governor Component Guide

## Component Overview
Governs display backlight PWM modulation, logarithmic perceptual brightness curves, DDC/CI I2C bus monitor communication, and keyboard backlight stepping inspired by Omarchy Linux brightnessctl.

## Linux Inspiration
brightnessctl, ddcutil I2C monitor controller, and Linux sysfs /sys/class/backlight.

## BSD Inspiration
OpenBSD wscons display brightness, FreeBSD backlight driver.

## Current SigmaOS Status
Exponential PWM calculation, step controls, and monitor detection implemented in BrightnessBacklightEngine and Nim sigma_backlight_controller.nim.

## Critical Missing Features
Direct I2C /dev/i2c bus DDC/CI command transmission for external HDMI/DisplayPort desktop monitors, and ambient light auto-tuning.

## Implementation Priority
HIGH (Phase 1)

## Key Files to Create/Improve
src/desktop/omarchy_media_tools.rs, src/desktop/sigma_backlight_controller.nim, src/power/acpi.rs

## Testing Strategy
Backlight stepping tests across laptop displays, DDC/CI query response latency tests.

## Dependencies
HARDWARE_HAL_DRIVERS_AGENT, POWER_MANAGEMENT_AGENT

## Success Criteria
Smooth, visually linear brightness adjustments, instant response to laptop brightness hotkeys, zero backlight flicker.

## Open Source Competitors Analysis
Combines internal laptop backlight control with external monitor DDC/CI control into a single unified driver.

## Future Enhancements
Circadian rhythm dynamic brightness matching indoor ambient light color temperature.
