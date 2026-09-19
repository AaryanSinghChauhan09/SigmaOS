# Power Management

## Overview

SigmaOS implements comprehensive power management subsystems inspired by Linux's ACPI/Power Management and FreeBSD's power control frameworks. The system provides CPU frequency scaling, device power states, battery management, and thermal control for optimal energy efficiency.

## CPU Power Management

### CPU Frequency Scaling

- **CPUFreq Governor**: Dynamic frequency scaling with multiple governor policies:
  - `performance`: Maximum frequency for high performance
  - `powersave`: Minimum frequency for energy savings
  - `ondemand`: Dynamic scaling based on CPU load
  - `conservative`: Gradual frequency transitions
  - `schedutil`: Scheduler-driven frequency scaling

### CPU Idle States

- **C-States**: CPU idle states for power savings:
  - `C0`: Active state
  - `C1`: Halt state
  - `C1E`: Enhanced halt with voltage reduction
  - `C3`: Sleep state with cache flush
  - `C6`: Deep power-down state

### Intel SpeedStep and AMD Cool'n'Quiet

- Support for Intel SpeedStep Technology (SST)
- Support for AMD Cool'n'Quiet power management
- Automatic voltage and frequency scaling

## Device Power Management

### PCI Power Management

- **D-States**: PCI device power states:
  - `D0`: Fully operational
  - `D1`: Partial power down
  - `D2`: More power down
  - `D3hot`: Context saved, minimal power
  - `D3cold`: Powered off

### USB Power Management

- **USB Selective Suspend**: Automatic device suspension
- **USB3 Link Power Management (LPM)**: Reduced power for idle links
- **Hub power control**: Individual port power management

### Storage Power Management

- **SATA Link Power Management (LPM)**:
  - HIPM (Host Initiated) power management
  - DIPM (Device Initiated) power management
- **NVMe Power States**: Multiple power states for NVMe SSDs
- **Hard Drive Idle**: Automatic spindle spin-down

## Battery Management

### Battery Monitoring

- **ACPI Battery**: Standard ACPI battery interface
- **Capacity Reporting**: Current, design, and last full capacity
- **Health Status**: Battery health and cycle count
- **Charging Status**: Charging, discharging, full, or critical

### Power Saving Profiles

- **Balanced**: Balance between performance and battery life
- **Power Saver**: Maximum battery life
- **High Performance**: Maximum performance
- **Custom**: User-configurable power settings

## Thermal Management

### CPU Thermal Control

- **Thermal Zones**: Multiple thermal zones with individual sensors
- **Thermal Trip Points**: Action triggers at specific temperatures
- **Passive Cooling**: Throttling before active cooling
- **Active Cooling**: Fan speed control

### System Thermal Management

- **Fan Control**: PWM-based fan speed control
- **Temperature Sensors**: Multiple temperature sensors
- **Thermal Throttling**: Performance reduction at high temperatures
- **Emergency Shutdown**: Protection against overheating

## Display Power Management

### Display Power Control

- **DPMS (Display Power Management Signaling)**:
  - `On`: Display fully powered
  - `Standby`: Reduced power
  - `Suspend`: Very low power
  - `Off`: Powered off
- **Backlight Control**: Display brightness adjustment
- **Panel Self-Refresh**: Display refresh power savings

## System Sleep States

### ACPI Sleep States

- **S0 (Working)**: Normal operation
- **S1 (Sleep)**: Low wake latency
- **S2 (Sleep)**: Deeper sleep
- **S3 (Suspend-to-RAM)**: Memory powered, devices off
- **S4 (Suspend-to-Disk)**: Hibernate to disk
- **S5 (Soft Off)**: Powered off, can wake from LAN

### Suspend/Resume

- **Memory Suspend**: Fast suspend to RAM
- **Disk Hibernate**: Deep suspend to disk
- **Hybrid Sleep**: Combination of memory and disk suspend
- **Wake Sources**: Configure wake-on-LAN, wake-on-USB, etc.

## Configuration

### Power Management Settings

```bash
# Set CPU governor
echo powersave > /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor

# Set display brightness
echo 50 > /sys/class/backlight/acpi_video0/brightness

# Enable laptop mode
echo 5 > /proc/sys/vm/laptop_mode

# Configure SATA LPM
echo min_power > /sys/class/scsi_host/host0/link_power_management_policy
```

### Power Profiles

```bash
# List power profiles
sigma-powerctl list

# Set power profile
sigma-powerctl set powersave

# Get current status
sigma-powerctl status
```

## Performance Optimization

### Power Saving Tips

1. **Enable CPU frequency scaling**: Use appropriate governor for workload
2. **Configure device power management**: Enable selective suspend for USB devices
3. **Optimize display settings**: Reduce brightness and enable DPMS
4. **Manage background processes**: Reduce CPU usage when on battery
5. **Use power-saving profiles**: Switch to power-saver mode on battery

### Benchmarking

```bash
# Measure power consumption
sudo sigma-power-benchmark --duration 60

# Compare power profiles
sigma-powerctl benchmark --profile performance
sigma-powerctl benchmark --profile powersave
```

## Troubleshooting

### Common Issues

1. **High battery drain**: Check background processes and device power states
2. **CPU stuck at high frequency**: Verify governor settings and thermal status
3. **Sleep/wake failures**: Check BIOS settings and driver support
4. **Fan always running**: Check thermal sensors and cooling configuration

### Debugging

```bash
# Check CPU frequency
cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq

# Check thermal zones
cat /sys/class/thermal/thermal_zone0/temp

# Check battery status
cat /sys/class/power_supply/BAT0/capacity
cat /sys/class/power_supply/BAT0/status

# Monitor power consumption
powertop --calibrate
powertop
```

## References

- [Linux Power Management Documentation](https://www.kernel.org/doc/html/latest/power/)
- [ACPI Specification](https://uefi.org/specifications)
- [Intel SpeedStep Technology](https://www.intel.com/content/www/us/en/docs/)
- [AMD Cool'n'Quiet Technology](https://www.amd.com/en/technologies/cool-n-quiet)
