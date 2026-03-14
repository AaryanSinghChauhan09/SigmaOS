"""
SigmaOS Sovereign Stopwatch & Timer v1.0
Stopwatch + Countdown + Pomodoro — 100% stdlib
"""
import tkinter as tk
from tkinter import ttk
import time, threading
from typing import Any

try:
    from sigma_core.ui.fluid_design import ICONS # type: ignore
except ImportError:
    ICONS = {}

PAL={"bg":"#0D0F18","panel":"#13162A","card":"#1A1E30","accent":"#EC4899",
     "success":"#00D26A","danger":"#FF4D4D","text":"#E8E8F0","dim":"#9090A0",
     "border":"#2A2D45","warn":"#F59E0B"}

class SigmaStopwatch(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("SigmaOS Stopwatch & Timer")
        self.geometry("520x580"); self.configure(bg=PAL["bg"]); self.resizable(False,False)
        self._sw_running=False; self._sw_elapsed=0.0; self._sw_start=0.0
        self._cd_running=False; self._cd_remaining=0.0; self._cd_total=0.0
        self._pomo_state="work"; self._pomo_running=False
        self._laps=[]; self._thread=None
        
        # UI Proxies
        self._sw_disp: Any = None
        self._sw_start_btn: Any = None
        self._lap_box: Any = None
        self._cd_disp: Any = None
        self._cd_btn: Any = None
        self._cd_status: Any = None
        self._pomo_disp: Any = None
        self._pomo_state_lbl: Any = None
        self._pomo_btn: Any = None
        self._pomo_lbl: Any = None
        self._hh: Any = None
        self._mm: Any = None
        self._ss: Any = None
        self._work_min: Any = None
        self._break_min: Any = None
        self._long_break: Any = None
        self._pomo_remaining: float = 0.0
        self._pomo_count: int = 0

        self._build()

    def _build(self):
        hdr=tk.Frame(self,bg=PAL["panel"],height=50); hdr.pack(fill="x"); hdr.pack_propagate(False)
        tk.Label(hdr,text=f"{ICONS.get('timer', '⏱')}  STOPWATCH & TIMER",fg=PAL["accent"],bg=PAL["panel"],
                 font=("Segoe UI Bold",13)).pack(side="left",padx=18,pady=10)

        nb=ttk.Notebook(self); nb.pack(fill="both",expand=True,padx=10,pady=10)
        self._build_stopwatch(nb)
        self._build_countdown(nb)
        self._build_pomodoro(nb)

    def _build_stopwatch(self,nb):
        tab=tk.Frame(nb,bg=PAL["bg"]); nb.add(tab,text=f"  {ICONS.get('timer', '⏱')} Stopwatch  ")
        self._sw_disp=tk.Label(tab,text="00:00:00.00",fg=PAL["accent"],bg=PAL["bg"],
                                font=("Cascadia Code",42))
        self._sw_disp.pack(pady=30)
        btn_fr=tk.Frame(tab,bg=PAL["bg"]); btn_fr.pack()
        self._sw_start_btn=tk.Button(btn_fr,text=f"{ICONS.get('perf', '▶')} START",bg=PAL["success"],fg="white",
                                      font=("Segoe UI Bold",11),relief="flat",padx=22,pady=10,
                                      command=self._sw_toggle)
        self._sw_start_btn.pack(side="left",padx=6)
        tk.Button(btn_fr,text="LAP",bg=PAL["card"],fg=PAL["text"],
                  font=("Segoe UI Bold",11),relief="flat",padx=22,pady=10,
                  command=self._sw_lap).pack(side="left",padx=6)
        tk.Button(btn_fr,text=f"{ICONS.get('minimalist', '↺')} RESET",bg=PAL["danger"],fg="white",
                  font=("Segoe UI Bold",11),relief="flat",padx=22,pady=10,
                  command=self._sw_reset).pack(side="left",padx=6)
        tk.Label(tab,text="LAP TIMES",fg=PAL["dim"],bg=PAL["bg"],
                 font=("Segoe UI",8,"bold")).pack(pady=(20,4))
        self._lap_box=tk.Text(tab,bg=PAL["card"],fg=PAL["text"],font=("Cascadia Code",9),
                                height=8,borderwidth=0,padx=10,pady=10)
        self._lap_box.pack(fill="x",padx=16)

    def _build_countdown(self,nb):
        tab=tk.Frame(nb,bg=PAL["bg"]); nb.add(tab,text=f"  {ICONS.get('snapshots', '⏳')} Countdown  ")
        self._cd_disp=tk.Label(tab,text="00:00:00",fg=PAL["warn"],bg=PAL["bg"],
                                font=("Cascadia Code",42))
        self._cd_disp.pack(pady=20)
        inp_fr=tk.Frame(tab,bg=PAL["bg"]); inp_fr.pack(pady=10)
        self._hh=self._spin(inp_fr,"Hours",0,23); tk.Label(inp_fr,text=":",fg=PAL["dim"],bg=PAL["bg"],font=("Cascadia Code",24)).pack(side="left")
        self._mm=self._spin(inp_fr,"Min",0,59); tk.Label(inp_fr,text=":",fg=PAL["dim"],bg=PAL["bg"],font=("Cascadia Code",24)).pack(side="left")
        self._ss=self._spin(inp_fr,"Sec",0,59)
        btn_fr=tk.Frame(tab,bg=PAL["bg"]); btn_fr.pack(pady=14)
        self._cd_btn=tk.Button(btn_fr,text=f"{ICONS.get('perf', '▶')} START",bg=PAL["warn"],fg="white",
                                font=("Segoe UI Bold",11),relief="flat",padx=22,pady=10,
                                command=self._cd_toggle)
        self._cd_btn.pack(side="left",padx=6)
        tk.Button(btn_fr,text=f"{ICONS.get('minimalist', '↺')} RESET",bg=PAL["danger"],fg="white",
                  font=("Segoe UI Bold",11),relief="flat",padx=22,pady=10,
                  command=self._cd_reset).pack(side="left",padx=6)
        self._cd_status=tk.Label(tab,text="Set timer and press START",fg=PAL["dim"],
                                  bg=PAL["bg"],font=("Segoe UI",10))
        self._cd_status.pack(pady=8)

    def _spin(self,parent,label,fr,to):
        v=tk.IntVar(value=0)
        tk.Label(parent,text=label,fg=PAL["dim"],bg=PAL["bg"],font=("Segoe UI",7)).pack(side="left",padx=(4,0))
        sb=tk.Spinbox(parent,from_=fr,to=to,textvariable=v,width=4,bg=PAL["card"],fg="white",
                      font=("Cascadia Code",18),buttonbackground=PAL["card"],relief="flat")
        sb.pack(side="left")
        return v

    def _build_pomodoro(self,nb):
        tab=tk.Frame(nb,bg=PAL["bg"]); nb.add(tab,text="  🍅 Pomodoro  ")
        self._pomo_disp=tk.Label(tab,text="25:00",fg=PAL["danger"],bg=PAL["bg"],
                                  font=("Cascadia Code",56))
        self._pomo_disp.pack(pady=24)
        self._pomo_state_lbl=tk.Label(tab,text="FOCUS SESSION",fg=PAL["dim"],
                                       bg=PAL["bg"],font=("Segoe UI",11))
        self._pomo_state_lbl.pack()
        btn_fr=tk.Frame(tab,bg=PAL["bg"]); btn_fr.pack(pady=16)
        self._pomo_btn=tk.Button(btn_fr,text=f"{ICONS.get('perf', '▶')} START",bg=PAL["danger"],fg="white",
                                  font=("Segoe UI Bold",11),relief="flat",padx=22,pady=10,
                                  command=self._pomo_toggle)
        self._pomo_btn.pack(side="left",padx=6)
        tk.Button(btn_fr,text=f"{ICONS.get('code', '⏭')} SKIP",bg=PAL["card"],fg=PAL["text"],
                  font=("Segoe UI Bold",11),relief="flat",padx=22,pady=10,
                  command=self._pomo_skip).pack(side="left",padx=6)
        cfg_fr=tk.Frame(tab,bg=PAL["card"],padx=16,pady=12); cfg_fr.pack(fill="x",padx=20,pady=16)
        tk.Label(cfg_fr,text=f"{ICONS.get('hal', '⚙️')} Settings",fg=PAL["accent"],bg=PAL["card"],
                 font=("Segoe UI Bold",10)).pack(anchor="w")
        self._work_min=self._mini_spin(cfg_fr,f"{ICONS.get('zero_trust', '🎯')} Work (min)",25)
        self._break_min=self._mini_spin(cfg_fr,"☕ Break (min)",5)
        self._long_break=self._mini_spin(cfg_fr,"🌿 Long break (min)",15)
        self._pomo_lbl=tk.Label(tab,text="Sessions: 0",fg=PAL["success"],
                                 bg=PAL["bg"],font=("Segoe UI",9))
        self._pomo_lbl.pack()

    def _mini_spin(self,parent,label,default):
        fr=tk.Frame(parent,bg=PAL["card"]); fr.pack(fill="x",pady=2)
        tk.Label(fr,text=label,fg=PAL["dim"],bg=PAL["card"],
                 font=("Segoe UI",9),width=16,anchor="w").pack(side="left")
        v=tk.IntVar(value=default)
        tk.Spinbox(fr,from_=1,to=120,textvariable=v,width=5,bg=PAL["bg"],fg="white",
                   font=("Cascadia Code",10),buttonbackground=PAL["bg"],relief="flat").pack(side="left")
        return v

    def _sw_toggle(self):
        if self._sw_running:
            self._sw_running=False
            self._sw_elapsed+=time.time()-self._sw_start
            self._sw_start_btn.config(text=f"{ICONS.get('perf', '▶')} START",bg=PAL["success"])
        else:
            self._sw_running=True
            self._sw_start=time.time()
            self._sw_start_btn.config(text="PAUSE",bg=PAL["warn"])
            self._sw_tick()

    def _sw_tick(self):
        if not self._sw_running: return
        elapsed=self._sw_elapsed+(time.time()-self._sw_start)
        h=int(elapsed//3600); m=int((elapsed%3600)//60); s=int(elapsed%60); cs=int((elapsed%1)*100)
        self._sw_disp.config(text=f"{h:02}:{m:02}:{s:02}.{cs:02}")
        self.after(10,self._sw_tick)

    def _sw_lap(self):
        elapsed=self._sw_elapsed+(time.time()-self._sw_start if self._sw_running else 0)
        h=int(elapsed//3600); m=int((elapsed%3600)//60); s=int(elapsed%60); cs=int((elapsed%1)*100)
        lap=f"Lap {len(self._laps)+1:>3}: {h:02}:{m:02}:{s:02}.{cs:02}\n"
        self._laps.append(lap)
        self._lap_box.insert("end",lap)
        self._lap_box.see("end")

    def _sw_reset(self):
        self._sw_running=False; self._sw_elapsed=0.0
        self._sw_disp.config(text="00:00:00.00")
        self._sw_start_btn.config(text=f"{ICONS.get('perf', '▶')} START",bg=PAL["success"])
        self._laps=[]; self._lap_box.delete("1.0","end")

    def _cd_toggle(self):
        if self._cd_running:
            self._cd_running=False; self._cd_btn.config(text="RESUME",bg=PAL["success"])
        else:
            if not self._cd_running and self._cd_remaining==0:
                self._cd_remaining=float(self._hh.get()*3600+self._mm.get()*60+self._ss.get())
                self._cd_total=self._cd_remaining
            self._cd_running=True; self._cd_btn.config(text="PAUSE",bg=PAL["warn"])
            self._cd_tick()

    def _cd_tick(self):
        if not self._cd_running: return
        if self._cd_remaining<=0:
            self._cd_running=False
            self._cd_disp.config(text="00:00:00",fg=PAL["danger"])
            self._cd_status.config(text="⏰ TIME'S UP!"); return
        rem = int(self._cd_remaining)
        h=rem//3600; m=(rem%3600)//60; s=rem%60
        self._cd_disp.config(text=f"{h:02}:{m:02}:{s:02}")
        pct=self._cd_remaining/self._cd_total if self._cd_total else 1.0
        self._cd_disp.config(fg=PAL["danger"] if pct<0.2 else (PAL["warn"] if pct<0.5 else PAL["success"]))
        self._cd_remaining-=1; self.after(1000,self._cd_tick)

    def _cd_reset(self):
        self._cd_running=False; self._cd_remaining=0.0
        self._cd_disp.config(text="00:00:00",fg=PAL["warn"])
        self._cd_btn.config(text=f"{ICONS.get('perf', '▶')} START",bg=PAL["warn"])
        self._cd_status.config(text="Set timer and press START")

    def _pomo_toggle(self):
        if self._pomo_running:
            self._pomo_running=False; self._pomo_btn.config(text="RESUME")
        else:
            self._pomo_running=True; self._pomo_btn.config(text="PAUSE")
            if self._pomo_remaining <= 0:
                self._pomo_remaining=float(self._work_min.get()*60)
            self._pomo_tick()

    def _pomo_skip(self):
        self._pomo_running=False
        self._pomo_next_phase()
        self._pomo_running=True; self._pomo_tick()

    def _pomo_next_phase(self):
        if self._pomo_state=="work":
            self._pomo_count+=1
            self._pomo_lbl.config(text=f"Sessions: {self._pomo_count}")
            if self._pomo_count%4==0:
                self._pomo_state="long"; self._pomo_remaining=float(self._long_break.get()*60)
                self._pomo_state_lbl.config(text="🌿 LONG BREAK",fg=PAL["success"])
            else:
                self._pomo_state="break"; self._pomo_remaining=float(self._break_min.get()*60)
                self._pomo_state_lbl.config(text="☕ SHORT BREAK",fg=PAL["success"])
        else:
            self._pomo_state="work"; self._pomo_remaining=float(self._work_min.get()*60)
            self._pomo_state_lbl.config(text="🎯 FOCUS SESSION",fg=PAL["danger"])

    def _pomo_tick(self):
        if not self._pomo_running: return
        if self._pomo_remaining<=0:
            self._pomo_next_phase(); self.bell()
        rem = int(self._pomo_remaining)
        m=rem//60; s=rem%60
        self._pomo_disp.config(text=f"{m:02}:{s:02}")
        self._pomo_remaining-=1; self.after(1000,self._pomo_tick)

def launch(kernel=None):
    SigmaStopwatch(kernel).mainloop()

if __name__=="__main__":
    launch()
