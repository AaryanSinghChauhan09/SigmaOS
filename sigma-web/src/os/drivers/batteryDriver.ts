import { useState, useEffect } from "react";

export interface BatteryState {
  supported: boolean;
  level: number;
  charging: boolean;
  chargingTime: number;
  dischargingTime: number;
}

const defaults: BatteryState = { supported: false, level: 1, charging: true, chargingTime: 0, dischargingTime: Infinity };

export const useBatteryDriver = (): BatteryState => {
  const [state, setState] = useState<BatteryState>(defaults);
  useEffect(() => {
    const nav = navigator as any;
    if (!nav.getBattery) return;
    let battery: any = null;
    const update = (b: any) => setState({ supported: true, level: b.level, charging: b.charging, chargingTime: b.chargingTime, dischargingTime: b.dischargingTime });
    nav.getBattery().then((b: any) => {
      battery = b; update(b);
      b.addEventListener("levelchange", () => update(b));
      b.addEventListener("chargingchange", () => update(b));
      b.addEventListener("chargingtimechange", () => update(b));
      b.addEventListener("dischargingtimechange", () => update(b));
    }).catch(() => setState({ ...defaults, supported: false }));
    return () => { if (battery) ["levelchange","chargingchange","chargingtimechange","dischargingtimechange"].forEach(e => battery.removeEventListener(e, () => {})); };
  }, []);
  return state;
};
