#!/bin/bash
# sigmaos shell plugin: weather
# Modeled on Omancy 4.0.5 omancy.weather shell plugin
# Shows weather information via Open-Meteo or cached data.

plugin_cmd="${1:-}"
payload="${2:-}"

WEATHER_FILE="${XDG_STATE_HOME:-$HOME/.local/state}/sigmaos/weather.json"

case "${plugin_cmd}" in
  summon)
    mkdir -p "$(dirname "${WEATHER_FILE}")"
    if [ -f "${WEATHER_FILE}" ]; then
      cat "${WEATHER_FILE}" 2>/dev/null || true
    else
      echo "No weather data configured. Use 'sigmaos-weather-location --set <city> <lat,lon>' to set location."
    fi
    echo "weather panel opened"
    ;;
  hide)
    echo "weather panel closed"
    ;;
  *)
    echo "Usage: sigmaos.weather <summon|hide>" >&2
    exit 1
    ;;
esac
