
# Customization Guide: Personalizing the Sovereign Lattice


SigmaOS is designed for absolute flexibility. Whether you are a power user or an industrial operator, the system adapts to your workflow.


## 1. Zenith Dashboard Themes

The **Sovereign Theming Engine** allows you to switch between predefined profiles or define your own CSS variables.

```javascript
SigmaThemingEngine.applyTheme('matrix_green');
```

Available Profiles:
- `sovereign_dark`: Default industrial aesthetic.
- `matrix_green`: Pure silicon diagnostic mode.
- `serenity_retro`: Retro-inspired high-contrast UI.


## 2. Desktop Widgets (Conky-Style)

Use the **Widget Engine** to create real-time monitors for any shard in the lattice.

```javascript
SigmaWidgetEngine.createWidget('cpu', 'CPU LOAD', () => {
    return `${VitalsEngine.stats.cpu}% [${'#'.repeat(VitalsEngine.stats.cpu/10)}]`;
});
```


## 3. Plugin Architecture

Extend the Zenith Dashboard with external modules via the **Plugin Loader**. Plugins can inject new shards, visualizations, or automation tasks.

```javascript
SigmaPluginLoader.loadPlugin('NetMonitor', '/plugins/network_monitor.js');
```


## 4. Declarative Layouts (i3/Sway-Style)

Future updates will include tiled window management within the Zenith Canvas, allowing for keyboard-driven orchestration of all 33 suites.
