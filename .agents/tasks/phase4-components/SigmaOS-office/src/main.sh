#!/usr/bin/env bash
# SigmaOS Office — LibreOffice integration and office suite manager scaffold
# Upstream: LibreOffice (https://www.libreoffice.org/), OnlyOffice
set -euo pipefail

SUITE="${SIGMA_OFFICE_SUITE:-libreoffice}"

usage() {
    cat >&2 <<EOF
Usage: $(basename "$0") <subcommand> [options]

SigmaOS Office suite integration manager

Subcommands:
  install   [--suite libre|only]  Install the office suite
  open      <file>                Open a file in the appropriate app
  convert   <in> <out>            Convert between office formats (headless)
  writer                          Launch word processor
  calc                            Launch spreadsheet
  impress                         Launch presentation editor
  draw                            Launch drawing/vector editor
  status                          Show installed suite and version
  formats                         List supported file formats
  version                         Print manager version

EOF
}

cmd_install() {
    local suite="${1:---suite}"
    local name="${2:-libre}"
    case "$name" in
        libre|libreoffice)
            echo "[office] Installing LibreOffice..."
            echo "  sigmaos-pkg install libreoffice-fresh libreoffice-extension-languagetool"
            echo "  (scaffold: printed, not executed)"
            ;;
        only|onlyoffice)
            echo "[office] Installing OnlyOffice Desktop Editors..."
            echo "  sigmaos-pkg install onlyoffice-desktopeditors"
            echo "  (scaffold: printed, not executed)"
            ;;
        *)
            echo "[office] Unknown suite: $name  (valid: libre only)" >&2; exit 1 ;;
    esac
}

cmd_open() {
    local file="${1:?'file path required'}"
    if [ ! -f "$file" ]; then
        echo "[office] File not found: $file" >&2; exit 1
    fi
    local ext="${file##*.}"
    case "${ext,,}" in
        doc|docx|odt|rtf|txt)   app="writer" ;;
        xls|xlsx|ods|csv)       app="calc" ;;
        ppt|pptx|odp)           app="impress" ;;
        svg|odg)                 app="draw" ;;
        pdf)                     app="draw" ;;
        *)                       app="$SUITE" ;;
    esac
    echo "[office] Opening $file with $SUITE $app"
    command -v "$SUITE" &>/dev/null && exec "$SUITE" "--$app" "$file" || \
        echo "[office] $SUITE not installed — run: $(basename "$0") install"
}

cmd_convert() {
    local infile="${1:?'input file required'}"
    local outfile="${2:?'output file required'}"
    local outdir
    outdir=$(dirname "$outfile")
    local outfmt="${outfile##*.}"
    echo "[office] Converting: $infile → $outfile (format: $outfmt)"
    if command -v libreoffice &>/dev/null; then
        libreoffice --headless --convert-to "$outfmt" --outdir "$outdir" "$infile"
    else
        echo "[office] libreoffice not found — cannot convert" >&2; exit 1
    fi
}

launch_app() {
    local app="$1"; shift
    echo "[office] Launching $SUITE $app..."
    command -v "$SUITE" &>/dev/null && exec "$SUITE" "--$app" "$@" || \
        echo "[office] $SUITE not installed — run: $(basename "$0") install"
}

cmd_status() {
    echo "Office suite : $SUITE"
    if command -v libreoffice &>/dev/null; then
        echo "LibreOffice  : $(libreoffice --version 2>/dev/null | head -1)"
    else
        echo "LibreOffice  : not installed"
    fi
    if command -v onlyoffice-desktopeditors &>/dev/null; then
        echo "OnlyOffice   : installed"
    fi
}

cmd_formats() {
    cat <<'EOF'
Supported file formats (LibreOffice):
  Writer : .odt .docx .doc .rtf .txt .html .pdf (export)
  Calc   : .ods .xlsx .xls .csv
  Impress: .odp .pptx .ppt
  Draw   : .odg .svg .emf .wmf
  Math   : .odf (formulas)
Conversion: any of the above via: $(basename "$0") convert <in> <out>
EOF
}

case "${1:-}" in
    install)  cmd_install "${2:-}" "${3:-libre}" ;;
    open)     cmd_open "${2:-}" ;;
    convert)  cmd_convert "${2:-}" "${3:-}" ;;
    writer)   shift; launch_app writer "$@" ;;
    calc)     shift; launch_app calc "$@" ;;
    impress)  shift; launch_app impress "$@" ;;
    draw)     shift; launch_app draw "$@" ;;
    status)   cmd_status ;;
    formats)  cmd_formats ;;
    version)  echo "SigmaOS Office integration v0.5" ;;
    -h|--help|"") usage ;;
    *) echo "Unknown subcommand: $1" >&2; usage; exit 1 ;;
esac
