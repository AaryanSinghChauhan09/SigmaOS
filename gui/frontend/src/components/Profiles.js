/**
 * gui/frontend/src/components/Profiles.js
 * Profile switcher — visual cards for developer/secure/lightweight
 */
import { setProfile } from '../api.js';

const PROFILES = [
    { name: 'developer',   icon: '⌨',  desc: 'Low blur, dev tools, max performance',  color: '#00f0ff' },
    { name: 'secure',      icon: '🔒', desc: 'Privacy shield, max blur, crypto active', color: '#ff0055' },
    { name: 'lightweight', icon: '⚡',  desc: 'Near-zero effects, minimal memory',       color: '#ffcc00' },
    { name: 'default',     icon: '◉',  desc: 'Balanced defaults, MATRIX theme',         color: '#8888ff' },
];

export function Profiles(mountPoint) {
    let active = null;

    async function switchProfile(name) {
        const r = await setProfile(name);
        if (r.ok) {
            active = name;
            render();
            // Trigger config bridge if available
            window.sigmaConfig?.applyToGUI?.();
            window.sigmaNotify?.(`PROFILE: ${name.toUpperCase()}`, 'OPTIMAL');
        } else {
            window.sigmaNotify?.(`Profile error: ${r.data}`, 'WARN');
        }
    }

    function render() {
        const container = document.getElementById(mountPoint);
        if (!container) return;
        container.innerHTML = `
        <div class="panel-section">
            <div class="profile-grid">
            ${PROFILES.map(p => `
            <div class="profile-card ${p.name === active ? 'profile-card-active' : ''}"
                 style="--profile-color:${p.color}"
                 data-profile="${p.name}">
                <span class="profile-icon">${p.icon}</span>
                <span class="profile-name">${p.name.toUpperCase()}</span>
                <span class="profile-desc">${p.desc}</span>
                ${p.name === active ? '<span class="profile-active-badge">ACTIVE</span>' : ''}
            </div>`).join('')}
            </div>
        </div>`;

        container.querySelectorAll('[data-profile]').forEach(card => {
            card.addEventListener('click', () => switchProfile(card.dataset.profile));
        });
    }

    render();
    return { switchProfile };
}
