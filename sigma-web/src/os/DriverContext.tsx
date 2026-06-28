import { createContext, useContext, ReactNode } from "react";
import { useBatteryDriver } from "./drivers/batteryDriver";
import { useNetworkDriver } from "./drivers/networkDriver";
import { useGeolocationDriver } from "./drivers/geolocationDriver";
import { useMotionDriver } from "./drivers/motionDriver";
import { useAmbientLightDriver } from "./drivers/ambientLightDriver";
import { useFilesystemDriver } from "./drivers/filesystemDriver";
import { useOpfsDriver } from "./drivers/opfsDriver";
import { useAudioDriver } from "./drivers/audioDriver";
import { useCameraDriver } from "./drivers/cameraDriver";
import { useScreenCaptureDriver } from "./drivers/screenCaptureDriver";
import { useGamepadDriver } from "./drivers/gamepadDriver";
import { useSpeechDriver } from "./drivers/speechDriver";
import { useNotificationDriver } from "./drivers/notificationDriver";
import { useClipboardDriver } from "./drivers/clipboardDriver";
import { useWakeLockDriver } from "./drivers/wakeLockDriver";
import { useMultiScreenDriver } from "./drivers/multiScreenDriver";
import { useUsbDriver } from "./drivers/usbDriver";
import { useBluetoothDriver } from "./drivers/bluetoothDriver";
import { useSerialDriver } from "./drivers/serialDriver";
import { useHidDriver } from "./drivers/hidDriver";
import { useMidiDriver } from "./drivers/midiDriver";
import { useGpuDriver } from "./drivers/gpuDriver";
import { useShareDriver } from "./drivers/shareDriver";
import { usePointerDriver } from "./drivers/pointerDriver";

type DriversContextType = {
  battery: ReturnType<typeof useBatteryDriver>;
  network: ReturnType<typeof useNetworkDriver>;
  geolocation: ReturnType<typeof useGeolocationDriver>;
  motion: ReturnType<typeof useMotionDriver>;
  ambientLight: ReturnType<typeof useAmbientLightDriver>;
  filesystem: ReturnType<typeof useFilesystemDriver>;
  opfs: ReturnType<typeof useOpfsDriver>;
  audio: ReturnType<typeof useAudioDriver>;
  camera: ReturnType<typeof useCameraDriver>;
  screenCapture: ReturnType<typeof useScreenCaptureDriver>;
  gamepad: ReturnType<typeof useGamepadDriver>;
  speech: ReturnType<typeof useSpeechDriver>;
  notification: ReturnType<typeof useNotificationDriver>;
  clipboard: ReturnType<typeof useClipboardDriver>;
  wakeLock: ReturnType<typeof useWakeLockDriver>;
  multiScreen: ReturnType<typeof useMultiScreenDriver>;
  usb: ReturnType<typeof useUsbDriver>;
  bluetooth: ReturnType<typeof useBluetoothDriver>;
  serial: ReturnType<typeof useSerialDriver>;
  hid: ReturnType<typeof useHidDriver>;
  midi: ReturnType<typeof useMidiDriver>;
  gpu: ReturnType<typeof useGpuDriver>;
  share: ReturnType<typeof useShareDriver>;
  pointer: ReturnType<typeof usePointerDriver>;
};

const DriversContext = createContext<DriversContextType | undefined>(undefined);

export const DriversProvider = ({ children }: { children: ReactNode }) => {
  const battery = useBatteryDriver();
  const network = useNetworkDriver();
  const geolocation = useGeolocationDriver();
  const motion = useMotionDriver();
  const ambientLight = useAmbientLightDriver();
  const filesystem = useFilesystemDriver();
  const opfs = useOpfsDriver();
  const audio = useAudioDriver();
  const camera = useCameraDriver();
  const screenCapture = useScreenCaptureDriver();
  const gamepad = useGamepadDriver();
  const speech = useSpeechDriver();
  const notification = useNotificationDriver();
  const clipboard = useClipboardDriver();
  const wakeLock = useWakeLockDriver();
  const multiScreen = useMultiScreenDriver();
  const usb = useUsbDriver();
  const bluetooth = useBluetoothDriver();
  const serial = useSerialDriver();
  const hid = useHidDriver();
  const midi = useMidiDriver();
  const gpu = useGpuDriver();
  const share = useShareDriver();
  const pointer = usePointerDriver();

  return (
    <DriversContext.Provider value={{
      battery, network, geolocation, motion, ambientLight,
      filesystem, opfs, audio, camera, screenCapture,
      gamepad, speech, notification, clipboard, wakeLock,
      multiScreen, usb, bluetooth, serial, hid, midi, gpu, share, pointer,
    }}>
      {children}
    </DriversContext.Provider>
  );
};

export const useDrivers = (): DriversContextType => {
  const ctx = useContext(DriversContext);
  if (!ctx) throw new Error("useDrivers must be used within DriversProvider");
  return ctx;
};
