import { useState, useEffect } from "react";

export interface GpuAdapterInfo { vendor: string; architecture: string; description: string; deviceType: string; }
export interface GpuState { supported: boolean; adapter: GpuAdapterInfo|null; features: string[]; limits: Record<string,number>; error: string|null; }

export const useGpuDriver = (): GpuState => {
  const [state, setState] = useState<GpuState>({ supported: "gpu" in navigator, adapter: null, features: [], limits: {}, error: null });
  useEffect(() => {
    if (!("gpu" in navigator)) return;
    (async () => {
      try {
        const adapter = await (navigator as any).gpu.requestAdapter();
        if (!adapter) { setState(prev=>({...prev,error:"No GPU adapter found"})); return; }
        const info = await adapter.requestAdapterInfo();
        const features: string[] = []; adapter.features.forEach((f:string)=>features.push(f));
        const limits: Record<string,number> = {};
        for (const key of ["maxTextureDimension1D","maxTextureDimension2D","maxTextureDimension3D","maxBindGroups","maxUniformBufferBindingSize","maxStorageBufferBindingSize","maxComputeWorkgroupSizeX","maxComputeWorkgroupSizeY"]) {
          try { limits[key]=adapter.limits[key]; } catch {}
        }
        setState({ supported:true, adapter:{ vendor:info.vendor||"Unknown", architecture:info.architecture||"Unknown", description:info.description||"Unknown", deviceType:info.deviceType||"Unknown" }, features, limits, error:null });
      } catch(e:any) { setState(prev=>({...prev,error:e.message})); }
    })();
  }, []);
  return state;
};
