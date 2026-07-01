import { useState } from "react";

export interface NotificationState { supported: boolean; permission: NotificationPermission; }

export const useNotificationDriver = () => {
  const [state, setState] = useState<NotificationState>({ supported: "Notification" in window, permission: "Notification" in window ? Notification.permission : "denied" });
  const requestPermission = async () => { if (!("Notification" in window)) return; const result = await Notification.requestPermission(); setState(prev=>({...prev,permission:result})); };
  const sendNotification = (title: string, options?: NotificationOptions) => { if (!("Notification" in window) || Notification.permission !== "granted") return; new Notification(title, { icon: "/favicon.ico", ...options }); };
  return { ...state, requestPermission, sendNotification };
};
