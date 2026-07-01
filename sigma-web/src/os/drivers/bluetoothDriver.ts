import { useState } from "react";

export interface BluetoothDeviceInfo { id: string; name?: string; connected: boolean; }
export interface BluetoothState { supported: boolean; scanning: boolean; device: BluetoothDeviceInfo|null; error: string|null; }

export const useBluetoothDriver = () => {
  const [state, setState] = useState<BluetoothState>({ supported: "bluetooth" in navigator, scanning: false, device: null, error: null });
  const requestDevice = async () => {
    if (!("bluetooth" in navigator)) return;
    setState(prev=>({...prev,scanning:true,error:null}));
    try {
      const device: any = await (navigator as any).bluetooth.requestDevice({ acceptAllDevices:true, optionalServices:[] });
      const server = await device.gatt?.connect().catch(()=>null);
      setState(prev=>({...prev,scanning:false,device:{id:device.id,name:device.name,connected:!!server}}));
      device.addEventListener("gattserverdisconnected", () => setState(prev=>prev.device?{...prev,device:{...prev.device,connected:false}}:prev));
    } catch(e:any) { setState(prev=>({...prev,scanning:false,error:e.name!=="NotFoundError"?e.message:null})); }
  };
  const disconnect = () => setState(prev=>({...prev,device:null}));
  return { ...state, requestDevice, disconnect };
};
