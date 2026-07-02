# Zenith GUI Toolkit

Sovereign compositor + auto-tiling window manager + personalization.

## Components

| Component | Path |
|-----------|------|
| Compositor | `zenith_desktop/compositor/sigma_compositor.cpp` |
| Tiling WM | `zenith_desktop/wm/sigma_tiling_wm.cpp` |
| Theme engine | `zenith_desktop/theme/sigma_theme_engine.cpp` |
| Profile engine | `zenith_desktop/personalization/sigma_profile_engine.cpp` |

## Personalization (`~/.sigma_profile`)

Example keys:

```
theme=zenith-dark
accent=007AFF
wm_layout=master-stack
gap_inner=4
gap_outer=8
auto_tile=1
```

Template: [sigma_profile.example](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/examples/sigma_profile.example)

## API entry points

- `zenith_compositor_init()` / `zenith_compositor_run_loop(frames)`
- `sigma_wm_auto_tile()` — re-tile active workspace
- `zenith_profile_init()` — load profile and apply theme/WM IPC

## Tests

```bash
./tools/zenith/build_tiling_test.sh
```

## Competitive angle

- **SteamOS / Solus**: native stack, auto-tiling, declarative profile (no GNOME/KDE dependency)
- **i3/bspwm**: BSP + master-stack layouts in `sigma_tiling_wm.cpp`
