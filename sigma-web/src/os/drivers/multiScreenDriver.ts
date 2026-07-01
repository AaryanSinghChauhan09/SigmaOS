import { useState } from "react";

export interface ScreenInfo { id: string; label: string; width: number; height: number; left: number; top: number; isPrimary: boolean; isInternal: boolean; devicePixelRatio: number; }
export interface MultiScreenState { supported: boolean; permission: "idle"|"granted"|"denied"|"requesting"; screens: ScreenInfo[]; }

export const useMultiScreenDriver = () => {
  const [state, setState] = useState<MultiScreenState>({ supported: "getScreenDetails" in window, permission: "idle", screens: [{ id:"primary", label:"Primary Display", width:screen.width, height:screen.height, left:0, top:0, isPrimary:true, isInternal:true, devicePixelRatio:window.devicePixelRatio }] });
  const requestScreenDetails = async () => {
    setState(prev=>({...prev,permission:"requesting"}));
    try {
      const details = await (window as any).getScreenDetails();
      setState(prev=>({...prev,permission:"granted",screens:details.screens.map((s:any,i:number)=>({id:s.label||`screen-${i}`,label:s.label||`Display ${i+1}`,width:s.width,height:s.height,left:s.left,top:s.top,isPrimary:s.isPrimary,isInternal:s.isInternal,devicePixelRatio:s.devicePixelRatio}))}));
    } catch { setState(prev=>({...prev,permission:"denied"})); }
  };
  return { ...state, requestScreenDetails };
};
