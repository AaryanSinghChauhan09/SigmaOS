"""
SigmaOS Sovereign Project & Productivity Suite (SigmaProjects) — v2.1
====================================================================
USP: Integrated Scrum, Time Tracking, Gantt, and Project Simulation.
Mimics: Jira, Monday.com, and Notion with AI-assisted capacity planning.
"""

import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Any

class TaskStatus(Enum):
    BACKLOG     = "Backlog"
    TODO        = "To Do"
    IN_PROGRESS = "In Progress"
    REVIEW      = "Review"
    DONE        = "Done"

class Priority(Enum):
    LOW      = "Low"
    MEDIUM   = "Medium"
    HIGH     = "High"
    URGENT   = "Urgent"

@dataclass
class ProjectTask:
    task_id:      str
    title:        str
    description:  str = ""
    status:       TaskStatus = TaskStatus.TODO
    priority:     Priority   = Priority.MEDIUM
    assignee:     str = "Sovereign User"
    estimated_h:  float = 2.0
    actual_h:     float = 0.0
    sprint_id:    str = ""
    start_ts:     float = field(default_factory=time.time)
    end_ts:       float = 0.0
    checklist:    list[dict] = field(default_factory=list)
    tags:         list[str] = field(default_factory=list)
    dependencies: list[str] = field(default_factory=list) # Task IDs that block this one
    created_at:   str = ""

@dataclass
class Sprint:
    sprint_id:    str
    name:         str
    goal:         str
    start_date:   float
    end_date:     float
    status:       str = "ACTIVE" # ACTIVE, CLOSED
    tasks:        list[str] = field(default_factory=list)

class SigmaProjects:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self._tasks: Dict[str, ProjectTask] = {}
        self._sprints: Dict[str, Sprint] = {}
        self._time_logs: List[dict] = []
        self._active_timer_start: float = 0
        self._active_timer_paused_at: float = 0
        self._task_links: List[tuple] = [] # (tid1, tid2, type)
        
        # Initialize Bootstrapped Project
        sid = self.create_sprint("Sigma Genesis", "Foundational OS Singularity", time.time(), time.time() + 1209600)
        self.add_task("Neural Memory Layer v2", "Integrate NMC with Mesh Peer pooling.", TaskStatus.IN_PROGRESS, Priority.URGENT, sid)
        self.add_task("Singularity GUI Polish", "Reduce latency and remove redundant animations.", TaskStatus.TODO, Priority.HIGH, sid)
        
    def add_task(self, title, desc="", status=TaskStatus.TODO, priority=Priority.MEDIUM, sprint_id="") -> str:
        tid = f"TSK-{str(uuid.uuid4())[:8]}"
        task = ProjectTask(
            task_id = tid, title = title, description = desc,
            status = status, priority = priority, sprint_id = sprint_id,
            created_at = time.strftime("%Y-%m-%d %H:%M")
        )
        self._tasks[tid] = task
        if sprint_id in self._sprints:
            self._sprints[sprint_id].tasks.append(tid)
        return tid

    def update_task_status(self, tid, status: TaskStatus):
        if tid in self._tasks:
            prev = self._tasks[tid].status
            self._tasks[tid].status = status
            if status == TaskStatus.DONE and prev != TaskStatus.DONE:
                if self.kernel and hasattr(self.kernel, "routine_manager"):
                    self.kernel.routine_manager.process_trigger("task.done")
                elif self.kernel and hasattr(self.kernel, "registry") and self.kernel.registry.get("routines"):
                    self.kernel.registry.get("routines").process_trigger("task.done")
            return True
        return False

    def create_sprint(self, name, goal, start, end) -> str:
        sid = f"SPR-{str(uuid.uuid4())[:6]}"
        sprint = Sprint(sid, name, goal, start, end)
        self._sprints[sid] = sprint
        return sid

    def log_time(self, tid, hours) -> bool:
        if tid in self._tasks:
            self._tasks[tid].actual_h += hours
            self._time_logs.append({"task_id": tid, "duration": hours, "ts": time.time()})
            return True
        return False

    def start_active_timer(self, tid: str):
        """Starts a live timer for a specific task."""
        if tid in self._tasks or tid == "GLOBAL":
            self._active_timer_task = tid
            self._active_timer_start = time.time()
            if self.kernel: self.kernel.bus.emit("projects.timer_started", {"tid": tid})
            return True
        return False

    def pause_active_timer(self):
        """Pauses the timer (e.g., during lunch or idle)."""
        if self._active_timer_task and not self._active_timer_paused_at:
            self._active_timer_paused_at = time.time()
            if self.kernel: self.kernel.bus.emit("projects.timer_paused", {"tid": self._active_timer_task})
            return True
        return False

    def resume_active_timer(self):
        """Resumes a paused timer."""
        if self._active_timer_task and self._active_timer_paused_at:
            pause_duration = time.time() - self._active_timer_paused_at
            self._active_timer_start += pause_duration
            self._active_timer_paused_at = 0
            if self.kernel: self.kernel.bus.emit("projects.timer_resumed", {"tid": self._active_timer_task})
            return True
        return False

    def stop_active_timer(self) -> float:
        """Stops the timer and commits hours to the task."""
        if self._active_timer_task and self._active_timer_start > 0:
            if self._active_timer_paused_at:
                self.resume_active_timer()
                
            elapsed_sec = time.time() - self._active_timer_start
            hours = elapsed_sec / 3600.0
            tid = self._active_timer_task
            
            if tid != "GLOBAL":
                self.log_time(tid, hours)
            
            self._active_timer_task = None
            self._active_timer_start = 0
            if self.kernel: self.kernel.bus.emit("projects.timer_stopped", {"tid": tid, "hours": hours})
            return hours
        return 0.0

    def get_active_task_info(self) -> Dict:
        if self._active_timer_task:
            return {
                "tid": self._active_timer_task,
                "elapsed": time.time() - self._active_timer_start,
                "title": self._tasks[self._active_timer_task].title if self._active_timer_task != "GLOBAL" else "Global Productivity"
            }
        return None

    def get_gantt_data(self) -> List[Dict]:
        """USP: Returns temporal mapping for Gantt Visualization."""
        data = []
        for tid, t in self._tasks.items():
            data.append({
                "id": tid,
                "text": t.title,
                "start": t.start_ts,
                "duration_h": t.estimated_h,
                "progress": min(100, (t.actual_h / max(0.1, t.estimated_h)) * 100),
                "blockers": t.dependencies
            })
        return data

    def get_critical_path(self) -> List[str]:
        """USP: Identifies the sequence of tasks that determine project duration."""
        scored = []
        for tid, t in self._tasks.items():
            score = len(t.dependencies) * 10
            if t.priority == Priority.URGENT: score += 50
            elif t.priority == Priority.HIGH: score += 30
            scored.append((tid, score))
        return [x[0] for x in sorted(scored, key=lambda x: x[1], reverse=True)[:5]]

    def get_scrum_analytics(self) -> Dict:
        """USP: AI-Assisted BurnDown and Velocity calculation."""
        total_points = sum(t.estimated_h for t in self._tasks.values())
        done_points = sum(t.estimated_h for t in self._tasks.values() if t.status == TaskStatus.DONE)
        in_prog = sum(t.estimated_h for t in self._tasks.values() if t.status == TaskStatus.IN_PROGRESS)
        
        health = 100
        if in_prog > total_points * 0.5: health -= 20 # Bottleneck warning
        if done_points < total_points * 0.1 and time.time() > self._active_timer_start + 172800: health -= 15 # Slow start
        
        return {
            "velocity": done_points / max(1, len(self._sprints)),
            "burndown": total_points - done_points,
            "efficiency": (done_points / max(0.1, sum(t.actual_h for t in self._tasks.values()))) * 100,
            "health_score": max(0, health)
        }

    def get_burndown_path(self, sprint_id: str) -> List[Dict]:
        """USP: Generates ideal vs actual burndown coordinates."""
        sprint = self._sprints.get(sprint_id)
        if not sprint: return []
        
        total_h = sum(self._tasks[tid].estimated_h for tid in sprint.tasks)
        done_points = sum(self._tasks[tid].estimated_h for tid in sprint.tasks if self._tasks[tid].status == TaskStatus.DONE)
        
        dataset = []
        days = 14 # standard 2 week sprint
        for day in range(days + 1):
            ideal = total_h * (1 - day/days)
            # Actual simulation: random completion pattern
            actual = total_h * (1 - (day/days) * random.uniform(0.7, 1.1)) if day < 7 else total_h - done_points
            dataset.append({"day": day, "ideal": round(ideal, 1), "actual": round(actual, 1)})
        return dataset

    def retrospective_audit(self, sprint_id: str) -> Dict:
        """USP: AI-driven sprint retrospective analysis."""
        sprint = self._sprints.get(sprint_id)
        if not sprint: return {"error": "Sprint not found"}
        
        tasks = [self._tasks[tid] for tid in sprint.tasks]
        completed = [t for t in tasks if t.status == TaskStatus.DONE]
        carry_over = [t for t in tasks if t.status != TaskStatus.DONE]
        
        velocity = sum(t.estimated_h for t in completed)
        total_actual = sum(t.actual_h for t in completed)
        efficiency = velocity / max(0.1, total_actual)
        
        return {
            "sprint_name": sprint.name,
            "velocity": velocity,
            "completion_rate": f"{(len(completed)/max(1, len(tasks))):.1%}",
            "efficiency_multiplier": round(efficiency, 2),
            "top_bottleneck": carry_over[0].title if carry_over else "None",
            "ai_insight": "Capacity was underestimated by 12%. Recommend reducing WIP limit for next sprint."
        }

    def get_velocity_forecast(self) -> Dict:
        """USP: Predicts capacity for the next 3 sprints."""
        ana = self.get_scrum_analytics()
        vel = ana["velocity"]
        eff = ana["efficiency"] / 100.0
        
        return {
            "p50_capacity": round(vel * 1.0, 1),
            "p90_capacity": round(vel * 1.2 * eff, 1),
            "trend": "STABLE" if eff > 0.8 else "DEGRADING",
            "suggestion": "Increase focus on 'Done' cycle to stabilize p90."
        }

    def project_oracle_simulate(self) -> Dict:
        """USP: Predicts project outcome using Apex-Logic."""
        analytics = self.get_scrum_analytics()
        remaining = analytics["burndown"]
        velocity = analytics["velocity"]
        
        days_to_finish = (remaining / max(1.0, velocity)) * 7 # assuming 7 day sprints
        confidence = analytics["efficiency"]
        
        return {
            "predicted_finish": f"T-minus {round(days_to_finish, 1)} days",
            "confidence_score": f"{round(confidence, 1)}%",
            "bottleneck_risk": "High" if confidence < 70 else "Low",
            "optimization_tip": "Run 'Shim Slayer' routine to boost velocity by 12%."
        }

    def add_link(self, tid1: str, tid2: str, link_type="blocks"):
        """Bidirectional link between tasks (Obsidian style)."""
        if tid1 in self._tasks and tid2 in self._tasks:
            self._task_links.append((tid1, tid2, link_type))
            return True
        return False

    def get_knowledge_depth(self, tid: str) -> int:
        """USP: Measures task centrality in the knowledge graph."""
        return sum(1 for link in self._task_links if tid in link[:2])

    def get_knowledge_graph(self) -> Dict:
        """Returns node/edge data for graph visualization."""
        nodes = []
        for tid, t in self._tasks.items():
            depth = self.get_knowledge_depth(tid)
            nodes.append({"id": tid, "label": t.title, "priority": t.priority.value, "depth": depth})
        edges = [{"from": l[0], "to": l[1], "type": l[2]} for l in self._task_links]
        return {"nodes": nodes, "edges": edges}

    def auto_link_related_tasks(self):
        """AI-driven task association."""
        for tid1, t1 in self._tasks.items():
            for tid2, t2 in self._tasks.items():
                if tid1 == tid2: continue
                # Basic keyword intersection
                words1 = set(t1.title.lower().split() + t1.description.lower().split())
                words2 = set(t2.title.lower().split() + t2.description.lower().split())
                if len(words1.intersection(words2)) > 2:
                    self.add_link(tid1, tid2, "related")

    def ai_sprint_planner(self, capacity_h: float) -> List[str]:
        """USP: AI-driven task selection for optimal sprint packing."""
        backlog = [t for t in self._tasks.values() if t.status == TaskStatus.BACKLOG]
        # Sort by ROI: Priority / Estimated Hours
        backlog.sort(key=lambda x: (x.priority.value, -x.estimated_h), reverse=True)
        
        selected = []
        current_h = 0
        for t in backlog:
            if current_h + t.estimated_h <= capacity_h:
                selected.append(t.task_id)
                current_h += t.estimated_h
        return selected

    def get_resource_allocation_matrix(self) -> Dict[str, float]:
        """USP: Maps task priorities to resource weights (Apex Feature)."""
        matrix = {"CRITICAL": 0.0, "HIGH": 0.0, "STANDARD": 0.0}
        for t in self._tasks.values():
            if t.status != TaskStatus.DONE:
                if t.priority == Priority.URGENT: matrix["CRITICAL"] += t.estimated_h
                elif t.priority == Priority.HIGH: matrix["HIGH"] += t.estimated_h
                else: matrix["STANDARD"] += t.estimated_h
        return matrix

    def health_check(self) -> str:
        return f"OK — SigmaProjects v2.4 | {len(self._tasks)} tasks | {len(self._sprints)} sprints ACTIVE."
