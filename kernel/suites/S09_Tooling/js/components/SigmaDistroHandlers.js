"use strict";

export const SigmaDistroHandlers = {
    setDistroMirror: (type, system) => {
        const root = document.documentElement;
        if (type === 'UBUNTU') {
            root.style.setProperty('--accent-primary', '#E95420');
            system.spawnToast('Distro Mirror: Ubuntu Parity ACTIVE.');
        } else if (type === 'ARCH') {
            root.style.setProperty('--accent-primary', '#1793D1');
            system.spawnToast('Distro Mirror: Arch Parity ACTIVE.');
        } else {
            root.style.setProperty('--accent-primary', '#00d2ff');
            system.spawnToast('Distro Mirror: Sovereign Mode [SIGMA].');
        }
    },

    applyPersona: (role, system) => {
        const shards = system.store.shards;
        const config = {
            'AI_RESEARCHER': ['aishard', 'mlshard', 'dsshard'],
            'DATA_SCIENTIST': ['dsshard', 'dsashard', 'planmaster'],
            'CYBER_EXPERT': ['cybershard', 'amnesicshard', 'oopsshard'],
            'FULL_STACK': ['webshard', 'vfsmanager', 'automationshard']
        };
        const targets = config[role] || [];
        shards.forEach(s => {
            if (targets.includes(s.id)) s.enabled = true;
            else s.enabled = false;
        });
        system.renderMenu();
        system.renderShardManager();
        system.spawnToast(`Persona ACTIVE: ${role}. Specialized shards ENABLED.`);
    }
};
