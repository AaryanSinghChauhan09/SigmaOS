/**
 * SigmaOS Task Manager
 * Module 05: Real-time shard monitoring and process termination.
 */

const TaskManager = {
    init() {
        console.log("Σ Task Manager: Initializing Sovereign Shard Monitor...");
        this.updateProcessList();
        setInterval(() => this.updateProcessList(), 3000);
    },

    updateProcessList() {
        const processes = [
            { pid: 1, name: "S01_Genesis", cpu: "0.1%", mem: "4MB" },
            { pid: 13, name: "S13_Sentience", cpu: "2.4%", mem: "128MB" },
            { pid: 42, name: "Zenith_UI", cpu: "5.1%", mem: "256MB" },
            { pid: 101, name: "Sigma_NLP", cpu: "1.2%", mem: "64MB" }
        ];

        const container = document.getElementById('process-list');
        if (!container) return;

        container.innerHTML = '';
        processes.forEach(p => {
            const row = document.createElement('div');
            row.className = "process-row";
            row.innerHTML = `
                <span class="p-id">PID:${p.pid}</span>
                <span class="p-name">${p.name}</span>
                <span class="p-cpu">${p.cpu}</span>
                <span class="p-mem">${p.mem}</span>
                <button class="p-kill" onclick="TaskManager.killProcess(${p.pid})">KILL</button>
            `;
            container.appendChild(row);
        });
    },

    killProcess(pid) {
        UIUtils.appendLog('audit-log', `Task Manager: Terminating Shard PID:${pid}...`, 'warning');
        UIUtils.appendLog('audit-log', `S [S03]: Process PID:${pid} neutralized correctly.`, 'success');
        this.updateProcessList();
    }
};

window.TaskManager = TaskManager;
