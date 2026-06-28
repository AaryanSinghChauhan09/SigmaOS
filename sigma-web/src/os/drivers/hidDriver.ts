import { useState, useEffect } from "react";

export interface HidDeviceInfo { vendorId: number; productId: number; productName: string; opened: boolean; }
export interface HidState { supported: boolean; devices: HidDeviceInfo[]; error: string|null; }

const toInfo = (d: any): HidDeviceInfo => ({ vendorId:d.vendorId, productId:d.productId, productName:d.productName, opened:d.opened });

export const useHidDriver = () => {
  const [state, setState] = useState<HidState>({ supported: "hid" in navigator, devices: [], error: null });
  const refresh = async () => { if(!("hid" in navigator)) return; try { setState(prev=>({...prev,devices:(await (navigator as any).hid.getDevices()).map(toInfo),error:null})); } catch(e:any) { setState(prev=>({...prev,error:e.message})); } };
  useEffect(() => { refresh(); const nav=navigator as any; if(!nav.hid) return; nav.hid.addEventListener("connect",refresh); nav.hid.addEventListener("disconnect",refresh); return ()=>{ nav.hid.removeEventListener("connect",refresh); nav.hid.removeEventListener("disconnect",refresh); }; }, []);
  const requestDevice = async () => { if(!("hid" in navigator)) return; try { const devs=await (navigator as any).hid.requestDevice({filters:[]}); setState(prev=>({...prev,devices:[...prev.devices,...devs.map(toInfo)],error:null})); } catch(e:any) { if(e.name!=="NotFoundError") setState(prev=>({...prev,error:e.message})); } };
  const openDevice = async (vendorId:number,productId:number) => { const nav=navigator as any; if(!nav.hid) return; try { const devs=await nav.hid.getDevices(); const d=devs.find((x:any)=>x.vendorId===vendorId&&x.productId===productId); if(d&&!d.opened) await d.open(); refresh(); } catch(e:any) { setState(prev=>({...prev,error:e.message})); } };
  return { ...state, requestDevice, openDevice, refresh };
};
