// File: zenith_desktop/modules/state-manager.js

class StateManager {
    constructor() {
        this.state = {
            user: {
                name: 'Sovereign_User',
                theme: 'cyan',
                automationEnabled: true
            },
            system: {
                cpuUsage: 0,
                memUsage: 0,
                batteryLevel: 87,
                isLocked: false
            },
            ui: {
                startMenuOpen: false,
                commandPaletteOpen: false,
                activeWindow: null
            }
        };

        this.observers = [];
    }

    subscribe(callback) {
        this.observers.push(callback);
        return () => {
            this.observers = this.observers.filter(cb => cb !== callback);
        };
    }

    setState(path, value) {
        const keys = path.split('.');
        let current = this.state;

        for (let i = 0; i < keys.length - 1; i++) {
            current = current[keys[i]];
        }

        current[keys[keys.length - 1]] = value;
        this.notifyObservers();
    }

    getState(path) {
        if (!path) return this.state;
        return path.split('.').reduce((obj, key) => obj[key], this.state);
    }

    notifyObservers() {
        this.observers.forEach(cb => cb(this.state));
    }

    loadState(saved) {
        this.state = { ...this.state, ...saved };
        this.notifyObservers();
    }

    saveState() {
        if (typeof localStorage !== 'undefined') {
            localStorage.setItem('sigmaos-state', JSON.stringify(this.state));
        }
    }
}

export const stateManager = new StateManager();
