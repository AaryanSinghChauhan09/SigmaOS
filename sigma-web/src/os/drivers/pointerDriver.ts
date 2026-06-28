import { useState, useEffect } from "react";

export interface ActivePointer { pointerId: number; pointerType: "mouse"|"pen"|"touch"; pressure: number; tiltX: number; tiltY: number; twist: number; x: number; y: number; }
export interface PointerState { supported: boolean; activePointers: ActivePointer[]; hasPen: boolean; hasTouch: boolean; }

export const usePointerDriver = (): PointerState => {
  const [state, setState] = useState<PointerState>({ supported: "PointerEvent" in window, activePointers: [], hasPen: false, hasTouch: false });
  useEffect(() => {
    if (!("PointerEvent" in window)) return;
    const toInfo = (e: PointerEvent): ActivePointer => ({ pointerId:e.pointerId, pointerType:e.pointerType as any, pressure:e.pressure, tiltX:e.tiltX, tiltY:e.tiltY, twist:e.twist, x:e.clientX, y:e.clientY });
    const onDown = (e: PointerEvent) => setState(prev => { const ptrs=[...prev.activePointers.filter(p=>p.pointerId!==e.pointerId),toInfo(e)]; return {...prev,activePointers:ptrs,hasPen:prev.hasPen||e.pointerType==="pen",hasTouch:prev.hasTouch||e.pointerType==="touch"}; });
    const onMove = (e: PointerEvent) => setState(prev => ({...prev,activePointers:prev.activePointers.map(p=>p.pointerId===e.pointerId?toInfo(e):p)}));
    const onUp = (e: PointerEvent) => setState(prev => ({...prev,activePointers:prev.activePointers.filter(p=>p.pointerId!==e.pointerId)}));
    window.addEventListener("pointerdown",onDown); window.addEventListener("pointermove",onMove); window.addEventListener("pointerup",onUp); window.addEventListener("pointercancel",onUp);
    return () => { window.removeEventListener("pointerdown",onDown); window.removeEventListener("pointermove",onMove); window.removeEventListener("pointerup",onUp); window.removeEventListener("pointercancel",onUp); };
  }, []);
  return state;
};
