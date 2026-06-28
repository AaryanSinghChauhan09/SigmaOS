import { useState } from "react";

export interface ShareState { supported: boolean; canShare: boolean; }

export const useShareDriver = () => {
  const [state] = useState<ShareState>({ supported: "share" in navigator, canShare: "canShare" in navigator });
  const share = async (data: { title?: string; text?: string; url?: string }) => {
    if (!("share" in navigator)) return false;
    try { await navigator.share(data); return true; } catch { return false; }
  };
  const checkCanShare = (data: { title?: string; text?: string; url?: string }) => {
    if (!("canShare" in navigator)) return false;
    try { return (navigator as any).canShare(data); } catch { return false; }
  };
  return { ...state, share, checkCanShare };
};
