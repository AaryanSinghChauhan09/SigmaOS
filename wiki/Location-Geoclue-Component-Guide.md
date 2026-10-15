# Geolocation & Privacy Coordinate Component Guide

## Component Overview
Provides geoclue-compatible location discovery, NMEA GPS receiver parsing, nightlight solar twilight calculation, and privacy-shielded coordinate fuzzing.

## Linux Inspiration
GeoClue2 D-Bus geolocation service, and redshift/wlsunset solar calculations.

## BSD Inspiration
OpenBSD nmea serial GPS driver and sensor framework.

## Current SigmaOS Status
Solar elevation and sunrise/sunset azimuth calculation implemented. GPS serial NMEA stream parser functional.

## Critical Missing Features
Anonymized Wi-Fi BSSID triangulation with hashed MACs, and granular per-application location permission toggles.

## Implementation Priority
LOW (Phase 3)

## Key Files to Create/Improve
src/location/gps.rs, src/location/solar.rs, src/desktop/sigma_nightlight.nim

## Testing Strategy
Astronomical solar calculation accuracy tests, GPS NMEA sentence checksum and parse stress tests.

## Dependencies
TIMEKEEPING_AGENT, DESKTOP_ENVIRONMENT_AGENT, SECURITY_AGENT

## Success Criteria
100% accurate solar nightlight color temp adjustment, total user location privacy protection against third-party apps.

## Open Source Competitors Analysis
Protects user privacy by fuzzing coordinates by default, unlike standard GeoClue which transmits unencrypted location data.

## Future Enhancements
Offline celestial navigation calculation allowing complete GPS tracking without any internet access.
