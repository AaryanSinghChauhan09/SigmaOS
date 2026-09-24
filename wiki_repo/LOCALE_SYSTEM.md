# SigmaOS Locale System & Regional Settings Specification

## 1. Overview

The SigmaOS Locale System manages global and per-process regional preferences, including system language, character encoding, number formatting, currency formatting, date/time representations, and collation sorting orders.

## 2. Environment Variables & Categories

SigmaOS adopts standard POSIX locale environment variables, extended with modern structured JSON/TOML locale configurations.

| Environment Variable | Description | Example Value |
|---|---|---|
| `LANG` | Global fallback locale | `en_US.UTF-8` |
| `LC_ALL` | Master override for all categories | `ja_JP.UTF-8` |
| `LC_MESSAGES` | UI text and message translations | `de_DE.UTF-8` |
| `LC_TIME` | Date and time formatting | `en_DK.UTF-8` (ISO-8601) |
| `LC_NUMERIC` | Decimal separator and group thousands | `fr_FR.UTF-8` (`1 234,56`) |
| `LC_MONETARY` | Currency symbols and placement | `en_US.UTF-8` (`$1,234.56`) |
| `LC_COLLATE` | String sorting and collation rules | `es_ES.UTF-8` |
| `LC_PAPER` | Default paper size (A4 vs Letter) | `en_GB.UTF-8` (A4) |

## 3. Locale Architecture & Data Definitions

Locale definitions reside in `/usr/share/locale/` or `/etc/locale.conf`.

```toml
# /etc/locale.conf - Master System Configuration
LANG="en_US.UTF-8"
LC_TIME="en_DK.UTF-8"
LC_NUMERIC="en_US.UTF-8"
LC_MONETARY="en_US.UTF-8"
```

### 3.1 Date and Time Formatting API
Supports locale-aware timestamp formatting:
- **`en_US`**: `12/31/2026, 11:59:59 PM`
- **`de_DE`**: `31.12.2026, 23:59:59`
- **`ja_JP`**: `2026年12月31日 23時59分59秒`
- **`ISO-8601` (`en_DK`)**: `2026-12-31 23:59:59`

### 3.2 Number and Currency Formatting API
- **Decimal Mark**: Standardizes handling of `.` vs `,`.
- **Digit Grouping**: Handles standard 3-digit thousands grouping (`1,000,000`) as well as Indian numbering system grouping (`10,000,00`).

## 4. Rust API Usage

```rust
use sigmaos_i18n::locale::{Locale, NumberFormat, DateFormat};

fn main() {
    let current_locale = Locale::current(); // Reads LANG / LC_*

    let formatted_number = NumberFormat::new(&current_locale)
        .format_float(1234567.89);
    println!("Formatted Number: {}", formatted_number);

    let formatted_currency = NumberFormat::new(&current_locale)
        .format_currency(49.99, "USD");
    println!("Formatted Currency: {}", formatted_currency);
}
```
