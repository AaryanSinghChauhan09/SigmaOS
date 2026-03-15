# Generated method: EnergyCore._update_battery_telemetry
import tkinter as tk
from tkinter import ttk, messagebox
import sys
import os
import time
import random
from userland.system_api.sigma_std import SigmaSys

class EnergyCore:
    def _update_battery_telemetry(self):
        try:
            battery = SigmaSys.sensors_battery()
            if battery:
                percent = battery.percent
                secsleft = battery.secsleft
                plugged = battery.power_plugged
                self.bat_percent_lbl.config(text=f'{int(percent)}%')
                if plugged:
                    self.bat_status_lbl.config(text='RECHARGING MATRIX AC ACTIVE', fg=PAL['accent'])
                    self.time_rem_lbl.config(text='CALCULATING TIME TO FULL...')
                    self.bat_percent_lbl.config(fg=PAL['accent'])
                else:
                    self.bat_status_lbl.config(text='DISCHARGING ON CORE BATTERY', fg=PAL['warning'])
                    if secsleft == psutil.POWER_TIME_UNLIMITED or secsleft < 0:
                        self.time_rem_lbl.config(text='ESTIMATING DEPLETION...')
                    else:
                        m, s = divmod(secsleft, 60)
                        h, m = divmod(m, 60)
                        self.time_rem_lbl.config(text=f'{int(h)}H {int(m)}M REMAINING')
                    if percent < 20:
                        self.bat_percent_lbl.config(fg=PAL['danger'])
                    elif percent < 50:
                        self.bat_percent_lbl.config(fg=PAL['warning'])
                    else:
                        self.bat_percent_lbl.config(fg=PAL['accent'])
            else:
                self.bat_percent_lbl.config(text='NO BAT')
                self.bat_status_lbl.config(text='DESKTOP MODE DETECTED')
                self.time_rem_lbl.config(text='AC POWER INFINITE')
        except:
            pass
        self.temp_card.val_lbl.config(text=f'{random.randint(30, 45)}°C')
        self.after(5000, self._update_battery_telemetry)