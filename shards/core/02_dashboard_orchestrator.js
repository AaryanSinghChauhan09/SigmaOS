/**
 * SigmaOS Dashboard Orchestrator
 * Module 02: High-level business logic for the Zenith workspace experience.
 */

const DashboardOrchestrator = {
    state: {
        activeTab: 'mission-view',
        systemLoad: 0,
        coverage: 0
    },

    init() {
        console.log("Σ Dashboard Orchestrator: Workspace Orchestration Online.");
        this.setupTabListeners();
        this.startMetricsSync();
    },

    setupTabListeners() {
        document.querySelectorAll('.tab-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                const tabId = btn.dataset.tab;
                this.switchTab(tabId);
            });
        });
    },

    switchTab(tabId) {
        this.state.activeTab = tabId;
        document.querySelectorAll('.tab-btn, .tab-content').forEach(el => el.classList.remove('active'));
        
        const btn = document.querySelector(`.tab-btn[data-tab="${tabId}"]`);
        const content = document.getElementById(tabId);
        
        if (btn) btn.classList.add('active');
        if (content) content.classList.add('active');
        
        UIUtils.appendLog('audit-log', `Switched to workspace: [${tabId}]`, 'normal');
    },

    startMetricsSync() {
        if (window.EventBus) {
            EventBus.subscribe('analytics_update', (data) => {
                if (data.metric === 'cpu_usage') this.state.systemLoad = data.value;
            });
        }
    }
};

window.DashboardOrchestrator = DashboardOrchestrator;
