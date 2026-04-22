/**
 * Zenith Dashboard - Task Manager Subsystem
 */

function refreshTaskList() {
    const tasks = [
        { id: 1, name: "Sovereign_Kernel", state: "RUNNING" },
        { id: 2, name: "Zenith_Dashboard", state: "READY" },
        { id: 3, name: "Storage_Shard", state: "BLOCKED" }
    ];

    const taskList = document.getElementById('task-list');
    if (taskList) {
        taskList.innerHTML = tasks.map(t => `
            <div class="task-item">
                <span class="task-id">ID: ${t.id}</span>
                <span class="task-name">${t.name}</span>
                <span class="task-state state-${t.state.toLowerCase()}">${t.state}</span>
            </div>
        `).join('');
    }
}

// Update every 500ms
setInterval(refreshTaskList, 500);
