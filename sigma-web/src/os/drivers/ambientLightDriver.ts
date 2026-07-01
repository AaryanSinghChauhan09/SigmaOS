import { useState, useEffect } from "react";

export interface AmbientLightState { supported: boolean; illuminance: number|null; theme: "light"|"dark"; }

export const useAmbientLightDriver = (): AmbientLightState => {
  const [state, setState] = useState<AmbientLightState>({ supported: false, illuminance: null, theme: "dark" });
  useEffect(() => {
    let sensor: any = null;
    try {
      const ALS = (window as any).AmbientLightSensor;
      if (ALS) {
        sensor = new ALS();
        sensor.addEventListener("reading", () => { const lux: number = sensor.illuminance; setState({ supported: true, illuminance: lux, theme: lux < 50 ? "dark" : "light" }); });
        sensor.addEventListener("error", () => setState(prev => ({ ...prev, supported: false })));
        sensor.start();
      }
    } catch {}
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    const onChange = (e: MediaQueryListEvent) => setState(prev => ({ ...prev, theme: e.matches ? "dark" : "light" }));
    setState(prev => ({ ...prev, theme: mq.matches ? "dark" : "light" }));
    mq.addEventListener("change", onChange);
    return () => { if (sensor) sensor.stop(); mq.removeEventListener("change", onChange); };
  }, []);
  return state;
};
