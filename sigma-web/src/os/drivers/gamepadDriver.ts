import { useState, useEffect, useRef } from "react";

export interface GamepadInfo { id: string; index: number; connected: boolean; buttons: Array<{pressed:boolean;value:number}>; axes: number[]; timestamp: number; }
export interface GamepadState { supported: boolean; gamepads: GamepadInfo[]; }

export const useGamepadDriver = (): GamepadState => {
  const [state, setState] = useState<GamepadState>({ supported: "getGamepads" in navigator, gamepads: [] });
  const rafRef = useRef<number|null>(null);
  const poll = () => {
    const raw = Array.from(navigator.getGamepads()).filter(Boolean) as Gamepad[];
    setState(prev => ({ ...prev, gamepads: raw.map(gp => ({ id:gp.id, index:gp.index, connected:gp.connected, buttons:gp.buttons.map(b=>({pressed:b.pressed,value:b.value})), axes:Array.from(gp.axes), timestamp:gp.timestamp })) }));
    rafRef.current = requestAnimationFrame(poll);
  };
  useEffect(() => {
    if (!("getGamepads" in navigator)) return;
    const onConnect = () => poll();
    const onDisconnect = (e: GamepadEvent) => setState(prev=>({...prev,gamepads:prev.gamepads.filter(g=>g.index!==e.gamepad.index)}));
    window.addEventListener("gamepadconnected", onConnect); window.addEventListener("gamepaddisconnected", onDisconnect);
    rafRef.current = requestAnimationFrame(poll);
    return () => { window.removeEventListener("gamepadconnected", onConnect); window.removeEventListener("gamepaddisconnected", onDisconnect); if(rafRef.current) cancelAnimationFrame(rafRef.current); };
  }, []);
  return state;
};
