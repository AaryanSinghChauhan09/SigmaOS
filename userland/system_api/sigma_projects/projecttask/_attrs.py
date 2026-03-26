task_id: str
title: str
description: str = ''
status: TaskStatus = TaskStatus.TODO
priority: Priority = Priority.MEDIUM
assignee: str = 'Sovereign User'
estimated_h: float = 2.0
actual_h: float = 0.0
sprint_id: str = ''
start_ts: float = field(default_factory=time.time)
end_ts: float = 0.0
checklist: list[dict] = field(default_factory=list)
tags: list[str] = field(default_factory=list)
dependencies: list[str] = field(default_factory=list)
created_at: str = ''
