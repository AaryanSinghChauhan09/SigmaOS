# SigmaOS-Themes Consolidation Plan

## Objective

Consolidate 4 theming repositories into 1 unified repository following Arch Linux's KISS principle and Omarchy's modular structure.

## Source Repositories

1. **SigmaOS-theme** - Core theming, Design Tokens & Color Palettes (Rust)
2. **SigmaOS-theme-icon** - Icon theme pack (Rust)
3. **SigmaOS-theme-gtk3** - GTK3 theme engine (Rust)
4. **SigmaOS-theme-desktop-environment** - Desktop-shell theme (Rust)

All repos have similar structure:
- Cargo.toml, Cargo.lock
- COMPONENT.toml
- src/ directory
- GitHub Actions workflows
- Tests

## Target Repository

**Name**: SigmaOS-themes
**Structure**:
```
SigmaOS-themes/
├── core/          # Core theming engine (from SigmaOS-theme)
├── icons/         # Icon themes (from SigmaOS-theme-icon)
├── gtk3/          # GTK3 themes (from SigmaOS-theme-gtk3)
├── desktop/        # Desktop themes (from SigmaOS-theme-desktop-environment)
├── Cargo.toml      # Unified workspace
├── COMPONENT.tomL  # Component metadata
└── README.md       # Consolidated documentation
```

## Consolidation Steps

### Step 1: Create SigmaOS-themes Repository
```bash
gh repo create SigmaOS-themes --public --description "SigmaOS unified theming system: icons, GTK3, desktop themes, and design tokens"
cd SigmaOS-themes
git init
```

### Step 2: Create Unified Cargo.toml
```toml
[workspace]
members = ["core", "icons", "gtk3", "desktop"]

[workspace.package]
version = "1.0.0"
edition = "2021"
```

### Step 3: Move Code from Source Repos

#### Move core theming
```bash
cd SigmaOS-theme
cp -r src ../SigmaOS-themes/core/
cp Cargo.toml ../SigmaOS-themes/core/
cp Cargo.lock ../SigmaOS-themes/core/
cp COMPONENT.toml ../SigmaOS-themes/core/
```

#### Move icons
```bash
cd SigmaOS-theme-icon
cp -r src ../SigmaOS-themes/icons/
cp Cargo.toml ../SigmaOS-themes/icons/
cp Cargo.lock ../SigmaOS-themes/icons/
cp COMPONENT.toml ../SigmaOS-themes/icons/
```

#### Move gtk3
```bash
cd SigmaOS-theme-gtk3
cp -r src ../SigmaOS-themes/gtk3/
cp Cargo.toml ../SigmaOS-themes/gtk3/
cp Cargo.lock ../SigmaOS-themes/gtk3/
cp COMPONENT.toml ../SigmaOS-themes/gtk3/
```

#### Move desktop
```bash
cd SigmaOS-theme-desktop-environment
cp -r src ../SigmaOS-themes/desktop/
cp Cargo.toml ../SigmaOS-themes/desktop/
cp Cargo.lock ../SigmaOS-themes/desktop/
cp COMPONENT.toml ../SigmaOS-themes/desktop/
```

### Step 4: Update COMPONENT.toml in Each Subdirectory

Each subdirectory needs its COMPONENT.toml updated to reflect being part of SigmaOS-themes:

**core/COMPONENT.toml**:
```toml
[component]
name = "sigmaos-theme-core"
description = "SigmaOS core theming engine, design tokens & color palettes"
repo = "https://github.com/AaryanSinghChauhan09/SigmaOS-themes"
engine = "theme_core"
category = "Desktop/UI"
language = "rust"
inspirations = "Omarchy Linux"
visibility = "public"
depends_on = []
```

**icons/COMPONENT.toml**:
```toml
[component]
name = "sigmaos-theme-icons"
description = "SigmaOS icon theme pack"
repo = "https://github.com/AaryanSinghChauhan09/SigmaOS-themes"
engine = "theme_icons"
category = "Desktop/UI"
language = "rust"
inspirations = "Linux Mint"
visibility = "public"
depends_on = ["sigmaos-theme-core"]
```

**gtk3/COMPONENT.toml**:
```toml
[component]
name = "sigmaos-theme-gtk3"
description = "SigmaOS GTK3 theme engine"
repo = "https://github.com/AaryanSinghChauhan09/SigmaOS-themes"
engine = "theme_gtk3"
category = "Desktop/UI"
language = "rust"
inspirations = "Omarchy Linux"
visibility = "public"
depends_on = ["sigmaos-theme-core"]
```

**desktop/COMPONENT.toml**:
```toml
[component]
name = "sigmaos-theme-desktop"
description = "SigmaOS desktop-shell theme"
repo = "https://github.com/AaryanSinghChauhan09/SigmaOS-themes"
engine = "theme_desktop"
category = "Desktop/UI"
language = "rust"
inspirations = "Cinnamon"
visibility = "public"
depends_on = ["sigmaos-theme-core"]
```

### Step 5: Create Unified README.md

```markdown
# SigmaOS-Themes

Unified theming system for SigmaOS, providing icons, GTK3 themes, desktop themes, and design tokens.

## Components

- **core**: Core theming engine, design tokens & color palettes
- **icons**: Icon theme pack
- **gtk3**: GTK3 theme engine
- **desktop**: Desktop-shell theme

## Installation

See individual component directories for installation instructions.

## Building

```bash
cargo build --workspace
```

## Testing

```bash
cargo test --workspace
```

## License

MIT OR GPL-2.0
```

### Step 6: Commit and Push SigmaOS-themes
```bash
cd SigmaOS-themes
git add .
git commit -m "feat: Consolidate SigmaOS theming into unified repository

Consolidate 4 theming repos into 1:
- SigmaOS-theme → core/
- SigmaOS-theme-icon → icons/
- SigmaOS-theme-gtk3 → gtk3/
- SigmaOS-theme-desktop-environment → desktop/

Following CONSOLIDATE GITHUB REPOS and MODULARITY principles."
git remote add origin git@github.com:AaryanSinghChauhan09/SigmaOS-themes.git
git push -u origin main
```

### Step 7: Update Main SigmaOS COMPONENTS-FOCUSED.toml

Replace the 4 separate theme entries with one:

```toml
[[component]]
name = "SigmaOS-themes"
description = "Unified theming system: icons, GTK3, desktop themes, design tokens"
repo = "https://github.com/AaryanSinghChauhan09/SigmaOS-themes"
engine = "theme_system"
category = "Desktop/UI"
language = "rust"
inspirations = "Omarchy Linux"
visibility = "public"
depends_on = []
```

## Benefits

1. **Simpler Maintenance** - One repo instead of 4
2. **Consistent Build** - Unified workspace
3. **Easier Discovery** - All theming in one place
4. **Follows Arch KISS** - Keep It Simple, Stupid
5. **Follows Omarchy** - Modular but consolidated where logical

## Rollback Plan

If consolidation fails:
1. Keep original 4 repos intact
2. Delete SigmaOS-themes
3. Revert COMPONENTS-FOCUSED.toml changes

## Approval Required

This consolidation requires approval because it:
- Creates a new repository
- Moves code between repositories
- Changes component registry

Generated following MODULARITY, CONSOLIDATE GITHUB REPOS, and COMPONENT ISOLATION principles.
