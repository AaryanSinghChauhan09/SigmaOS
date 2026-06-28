import { useState, useEffect } from "react";

export interface UsbDeviceInfo { vendorId: number; productId: number; manufacturerName?: string; productName?: string; serialNumber?: string; }
export interface UsbState { supported: boolean; devices: UsbDeviceInfo[]; error: string|null; }

const toInfo = (d: any): UsbDeviceInfo => ({ vendorId:d.vendorId, productId:d.productId, manufacturerName:d.manufacturerName, productName:d.productName, serialNumber:d.serialNumber });

export const useUsbDriver = () => {
  const [state, setState] = useState<UsbState>({ supported: "usb" in navigator, devices: [], error: null });
  const refresh = async () => { if(!("usb" in navigator)) return; try { const devs = await (navigator as any).usb.getDevices(); setState(prev=>({...prev,devices:devs.map(toInfo),error:null})); } catch(e:any) { setState(prev=>({...prev,error:e.message})); } };
  useEffect(() => { refresh(); const nav=navigator as any; if(!nav.usb) return; nav.usb.addEventListener("connect",refresh); nav.usb.addEventListener("disconnect",refresh); return () => { nav.usb.removeEventListener("connect",refresh); nav.usb.removeEventListener("disconnect",refresh); }; }, []);
  const requestDevice = async () => { try { const d = await (navigator as any).usb.requestDevice({filters:[]}); setState(prev=>({...prev,devices:[...prev.devices,toInfo(d)],error:null})); } catch(e:any) { if(e.name!=="NotFoundError") setState(prev=>({...prev,error:e.message})); } };
  return { ...state, requestDevice, refresh };
};
