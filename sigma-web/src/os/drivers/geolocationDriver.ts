import { useState, useRef } from "react";

export interface GeolocationState { supported: boolean; permission: "idle"|"granted"|"denied"|"requesting"; latitude: number|null; longitude: number|null; accuracy: number|null; altitude: number|null; speed: number|null; error: string|null; }

export const useGeolocationDriver = () => {
  const [state, setState] = useState<GeolocationState>({ supported: "geolocation" in navigator, permission: "idle", latitude: null, longitude: null, accuracy: null, altitude: null, speed: null, error: null });
  const watchIdRef = useRef<number | null>(null);
  const requestLocation = () => {
    if (!navigator.geolocation) { setState(prev => ({ ...prev, error: "Geolocation not supported" })); return; }
    setState(prev => ({ ...prev, permission: "requesting", error: null }));
    if (watchIdRef.current !== null) navigator.geolocation.clearWatch(watchIdRef.current);
    watchIdRef.current = navigator.geolocation.watchPosition(
      (pos) => setState(prev => ({ ...prev, permission: "granted", latitude: pos.coords.latitude, longitude: pos.coords.longitude, accuracy: pos.coords.accuracy, altitude: pos.coords.altitude, speed: pos.coords.speed, error: null })),
      (err) => setState(prev => ({ ...prev, permission: "denied", error: err.message })),
      { enableHighAccuracy: true }
    );
  };
  const stopLocation = () => { if (watchIdRef.current !== null) { navigator.geolocation.clearWatch(watchIdRef.current); watchIdRef.current = null; } setState(prev => ({ ...prev, permission: "idle" })); };
  return { ...state, requestLocation, stopLocation };
};
