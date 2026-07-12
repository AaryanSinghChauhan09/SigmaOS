/*
 * =========================================================================
 * Σ SIGMAOS: ZENITH DESKTOP INTERACTIVE CONTROLLER (v15.0)
 * Glassmorphic, Accessible, Keyboard-navigable User Experience
 * =========================================================================
 */

// Global State
let activeFocusWindow = null;
let screenReaderActive = false;
let highContrastActive = false;
let reducedMotionActive = false;

// Simulated Folder Tree
const fileSystem = {
    'home': [
        { name: 'Documents', type: 'dir' },
        { name: 'Downloads', type: 'dir' },
        { name: 'system_api', type: 'dir' },
        { name: 'readme.txt', type: 'file', content: 'Welcome to SigmaOS Zenith Desktop Environment!' }
    ],
    'Documents': [
        { name: 'project_notes.md', type: 'file', content: 'SigmaOS is built on a high-performance, no_std rust core.' },
        { name: 'budget.xlsx', type: 'file', content: 'Sovereign credits: 15,000,000' }
    ],
    'Downloads': [
        { name: 'zenith_v15_beta.iso', type: 'file', content: 'Raw binary partition payload.' }
    ],
    'system_api': [
        { name: 'web_interface.rs', type: 'file', content: 'Web server for AI models multi-model interaction.' },
        { name: 'zenith_accessibility.rs', type: 'file', content: 'Provides WCAG AAA high-contrast and text magnifier.' }
    ]
};

let currentDir = 'home';
let dirHistory = [];

// Screen Reader Sim Engine
function announce(message) {
    const logBox = document.getElementById('screen-reader-log');
    const textBox = document.getElementById('screen-reader-text');
    if (!logBox || !textBox) return;

    if (screenReaderActive) {
        textBox.innerText = message;
        logBox.classList.add('visible');

        // Hide after 4 seconds unless updated
        if (window.screenReaderTimeout) {
            clearTimeout(window.screenReaderTimeout);
        }
        window.screenReaderTimeout = setTimeout(() => {
            logBox.classList.remove('visible');
        }, 4000);
    }
}

// Window Management
export function toggleWindow(id) {
    const win = document.getElementById(id);
    if (!win) return;

    if (win.style.display === 'none') {
        win.style.display = 'flex';
        // Force reflow for transitions
        void win.offsetWidth;
        win.style.opacity = '1';
        win.style.pointerEvents = 'auto';
        win.style.transform = 'scale(1)';
        focusWindow(win);
        announce(`Opened window: ${win.getAttribute('aria-label')}`);
    } else {
        win.style.opacity = '0';
        win.style.pointerEvents = 'none';
        win.style.transform = 'scale(0.95)';
        // Wait for transitions
        setTimeout(() => {
            if (win.style.opacity === '0') {
                win.style.display = 'none';
            }
        }, 300);
        announce(`Closed window: ${win.getAttribute('aria-label')}`);
        if (activeFocusWindow === win) {
            activeFocusWindow = null;
        }
    }
}

export function focusWindow(win) {
    if (typeof win === 'string') {
        win = document.getElementById(win);
    }
    if (!win) return;

    // Remove active-focus from previous
    document.querySelectorAll('.window').forEach(w => w.classList.remove('active-focus'));

    win.classList.add('active-focus');
    activeFocusWindow = win;
    win.focus();
}

// Theme Setting
export function setTheme(themeName) {
    const body = document.body;
    body.className = ''; // reset
    body.classList.add(`theme-${themeName}`);
    if (highContrastActive) {
        body.classList.add('high-contrast');
    }
    announce(`Theme changed to: ${themeName}`);
}

// Expose globals for inline HTML event handlers
window.toggleWindow = toggleWindow;
window.setTheme = setTheme;

// --- Initialize Interactive Events ---
document.addEventListener('DOMContentLoaded', () => {
    const mouseGlow = document.getElementById('mouse-glow');
    const clockTime = document.getElementById('clock-time');
    const clockDate = document.getElementById('clock-date');
    const terminalInput = document.getElementById('terminal-input');
    const terminalOutput = document.getElementById('terminal-output');
    const fmGrid = document.getElementById('fm-grid');
    const fmBackBtn = document.getElementById('fm-back-btn');
    const launcherSearch = document.getElementById('launcher-search');
    const appLauncher = document.getElementById('app-launcher');

    // Telemetry Simulation
    setInterval(() => {
        const cpu = Math.floor(Math.random() * 5) + 1;
        const mem = Math.floor(Math.random() * 3) + 12;
        const cpuEl = document.getElementById('cpu-load');
        const memEl = document.getElementById('mem-load');
        if (cpuEl) cpuEl.innerText = `${cpu.toString().padStart(2, '0')}%`;
        if (memEl) memEl.innerText = `${mem.toString().padStart(2, '0')}%`;
    }, 3000);

    // Live Clock
    function updateClock() {
        const now = new Date();
        if (clockTime) {
            clockTime.innerText = now.toTimeString().split(' ')[0];
        }
        if (clockDate) {
            const options = { weekday: 'short', month: 'short', day: 'numeric' };
            clockDate.innerText = now.toLocaleDateString('en-US', options).toUpperCase();
        }
    }
    updateClock();
    setInterval(updateClock, 1000);

    // Mouse Glow effect
    window.addEventListener('mousemove', (e) => {
        if (mouseGlow && !reducedMotionActive) {
            mouseGlow.style.opacity = '1';
            mouseGlow.style.left = `${e.clientX}px`;
            mouseGlow.style.top = `${e.clientY}px`;
        }
    });

    // Window Dragging & Selection Focus
    document.querySelectorAll('.window').forEach(win => {
        win.addEventListener('mousedown', () => focusWindow(win));

        const header = win.querySelector('.window-header');
        if (header) {
            let isDragging = false;
            let startX, startY, startLeft, startTop;

            header.addEventListener('mousedown', (e) => {
                if (e.target.classList.contains('control-dot')) return;
                isDragging = true;
                startX = e.clientX;
                startY = e.clientY;
                startLeft = parseInt(win.style.left) || 0;
                startTop = parseInt(win.style.top) || 0;
                focusWindow(win);
            });

            window.addEventListener('mousemove', (e) => {
                if (!isDragging) return;
                const dx = e.clientX - startX;
                const dy = e.clientY - startY;
                win.style.left = `${startLeft + dx}px`;
                win.style.top = `${startTop + dy}px`;
            });

            window.addEventListener('mouseup', () => {
                isDragging = false;
            });
        }
    });

    // Screen Reader Focus Tracking
    const interactiveElements = document.querySelectorAll('button, input, [tabindex="0"], .dock-icon, .settings-tab');
    interactiveElements.forEach(el => {
        el.addEventListener('focus', () => {
            const label = el.getAttribute('aria-label') || el.getAttribute('title') || el.innerText || el.placeholder || el.value;
            if (label) {
                announce(`Focus on: ${label.trim()}`);
            }
        });
    });

    // Dock Navigation Handles
    const dockLauncher = document.getElementById('dock-launcher');
    const dockTerminal = document.getElementById('dock-terminal');
    const dockFileExp = document.getElementById('dock-file-manager');
    const dockSettings = document.getElementById('dock-settings');

    if (dockLauncher) {
        dockLauncher.addEventListener('click', () => {
            appLauncher.classList.toggle('active');
            const isActive = appLauncher.classList.contains('active');
            dockLauncher.setAttribute('aria-expanded', isActive ? 'true' : 'false');
            if (isActive) {
                launcherSearch.focus();
                announce('Opened application launcher');
            } else {
                announce('Closed application launcher');
            }
        });
    }

    if (dockTerminal) dockTerminal.addEventListener('click', () => toggleWindow('win-terminal'));
    if (dockFileExp) dockFileExp.addEventListener('click', () => toggleWindow('win-file-manager'));
    if (dockSettings) dockSettings.addEventListener('click', () => toggleWindow('win-settings'));

    // App Launcher Search Filter
    if (launcherSearch) {
        launcherSearch.addEventListener('input', (e) => {
            const val = e.target.value.toLowerCase();
            document.querySelectorAll('.launcher-app').forEach(app => {
                const text = app.innerText.toLowerCase();
                if (text.includes(val)) {
                    app.style.display = 'flex';
                } else {
                    app.style.display = 'none';
                }
            });
        });
    }

    // App Launcher Launch Handlers
    document.querySelectorAll('.launcher-app').forEach(app => {
        const handleLaunch = () => {
            const target = app.getAttribute('data-target');
            if (target === 'terminal') toggleWindow('win-terminal');
            if (target === 'file-manager') toggleWindow('win-file-manager');
            if (target === 'settings') toggleWindow('win-settings');
            appLauncher.classList.remove('active');
            if (dockLauncher) {
                dockLauncher.setAttribute('aria-expanded', 'false');
            }
        };
        app.addEventListener('click', handleLaunch);
        app.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') handleLaunch();
        });
    });

    // Control Center Settings Panel Tabs
    const tabThemes = document.getElementById('tab-themes');
    const tabAccessibility = document.getElementById('tab-accessibility');
    const panelThemes = document.getElementById('panel-themes');
    const panelAccessibility = document.getElementById('panel-accessibility');

    if (tabThemes && tabAccessibility) {
        const switchTab = (activeTab, inactiveTab, activePanel, inactivePanel) => {
            activeTab.classList.add('active');
            activeTab.setAttribute('aria-selected', 'true');
            activeTab.setAttribute('tabindex', '0');

            inactiveTab.classList.remove('active');
            inactiveTab.setAttribute('aria-selected', 'false');
            inactiveTab.setAttribute('tabindex', '-1');

            activePanel.style.display = 'flex';
            inactivePanel.style.display = 'none';

            announce(`Switched to tab: ${activeTab.innerText}`);
        };

        tabThemes.addEventListener('click', () => switchTab(tabThemes, tabAccessibility, panelThemes, panelAccessibility));
        tabAccessibility.addEventListener('click', () => switchTab(tabAccessibility, tabThemes, panelAccessibility, panelThemes));
    }

    // Accessibility Configuration Controls
    const toggleHighContrast = document.getElementById('toggle-high-contrast');
    const toggleReducedMotion = document.getElementById('toggle-reduced-motion');
    const toggleScreenReader = document.getElementById('toggle-screen-reader');

    if (toggleHighContrast) {
        toggleHighContrast.addEventListener('change', (e) => {
            highContrastActive = e.target.checked;
            document.body.classList.toggle('high-contrast', highContrastActive);
            announce(`High Contrast Mode turned ${highContrastActive ? 'ON' : 'OFF'}`);
        });
    }

    if (toggleReducedMotion) {
        toggleReducedMotion.addEventListener('change', (e) => {
            reducedMotionActive = e.target.checked;
            document.body.classList.toggle('reduced-motion', reducedMotionActive);
            if (reducedMotionActive && mouseGlow) {
                mouseGlow.style.opacity = '0';
            }
            announce(`Reduced Motion Mode turned ${reducedMotionActive ? 'ON' : 'OFF'}`);
        });
    }

    if (toggleScreenReader) {
        toggleScreenReader.addEventListener('change', (e) => {
            screenReaderActive = e.target.checked;
            const logBox = document.getElementById('screen-reader-log');
            if (logBox) {
                logBox.classList.toggle('visible', screenReaderActive);
            }
            if (screenReaderActive) {
                announce('Screen Reader Simulator Activated. Hover or focus any UI element.');
            } else {
                announce('Screen Reader Simulator Deactivated.');
            }
        });
    }

    // Terminal Input processing
    if (terminalInput && terminalOutput) {
        terminalInput.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                const command = terminalInput.value.trim();
                terminalInput.value = '';
                if (!command) return;

                // Echo command
                const echoLine = document.createElement('div');
                echoLine.className = 'term-line';
                echoLine.innerHTML = `<span class="term-prompt">sigma@root:~$</span> ${command}`;
                terminalOutput.appendChild(echoLine);

                // Command responses
                const respLine = document.createElement('div');
                respLine.className = 'term-line';

                const lowerCmd = command.toLowerCase();
                if (lowerCmd === 'help') {
                    respLine.innerHTML = 'Available commands:<br>- help: list commands<br>- about: display kernel info<br>- theme [default/solar/crimson/gold]: change theme<br>- clear: clear screen';
                } else if (lowerCmd === 'about') {
                    respLine.innerHTML = 'SigmaOS Zenith OS v15.0.0 Sovereign microkernel. Written in Rust and Nim with Capability-Gated Security and Autonomous Agents.';
                } else if (lowerCmd === 'clear') {
                    terminalOutput.innerHTML = '';
                    return;
                } else if (lowerCmd.startsWith('theme ')) {
                    const arg = lowerCmd.replace('theme ', '').trim();
                    if (['default', 'solar', 'crimson', 'gold'].includes(arg)) {
                        setTheme(arg);
                        respLine.className = 'term-line success';
                        respLine.innerText = `Theme switched to ${arg}`;
                    } else {
                        respLine.className = 'term-line error';
                        respLine.innerText = `Unknown theme argument: '${arg}'. Try default, solar, crimson, or gold.`;
                    }
                } else {
                    respLine.className = 'term-line error';
                    respLine.innerText = `Command not found: '${command}'`;
                }

                terminalOutput.appendChild(respLine);
                terminalOutput.scrollTop = terminalOutput.scrollHeight;
                announce(`Terminal output: ${respLine.innerText || 'command complete'}`);
            }
        });
    }

    // File Explorer Navigation Logic
    function renderFileExplorer() {
        if (!fmGrid) return;
        fmGrid.innerHTML = '';

        const items = fileSystem[currentDir] || [];
        items.forEach(item => {
            const gridItem = document.createElement('div');
            gridItem.className = 'fm-grid-item';
            gridItem.setAttribute('tabindex', '0');
            gridItem.setAttribute('role', 'gridcell');
            gridItem.setAttribute('aria-label', `${item.name} (${item.type === 'dir' ? 'Folder' : 'File'})`);

            gridItem.innerHTML = `
                <div class="fm-icon">${item.type === 'dir' ? '📁' : '📄'}</div>
                <div class="fm-label">${item.name}</div>
            `;

            const handleActivation = () => {
                if (item.type === 'dir') {
                    dirHistory.push(currentDir);
                    currentDir = item.name;
                    if (fmBackBtn) fmBackBtn.removeAttribute('disabled');
                    renderFileExplorer();
                    announce(`Opened folder: ${item.name}`);
                } else {
                    announce(`File content for ${item.name}: ${item.content}`);
                    alert(`[File Content] ${item.name}:\n\n${item.content}`);
                }
            };

            gridItem.addEventListener('click', handleActivation);
            gridItem.addEventListener('keydown', (evt) => {
                if (evt.key === 'Enter') handleActivation();
            });

            fmGrid.appendChild(gridItem);
        });

        const currentDirSpan = document.getElementById('fm-current-dir');
        if (currentDirSpan) {
            currentDirSpan.innerText = currentDir;
        }
    }

    if (fmBackBtn) {
        fmBackBtn.addEventListener('click', () => {
            if (dirHistory.length > 0) {
                currentDir = dirHistory.pop();
                if (dirHistory.length === 0) {
                    fmBackBtn.setAttribute('disabled', 'true');
                }
                renderFileExplorer();
                announce(`Navigated back to ${currentDir}`);
            }
        });
    }

    // Initial render
    renderFileExplorer();

    // Universal Keyboard Navigation and Shortcuts (delightful touch)
    window.addEventListener('keydown', (e) => {
        // Ctrl+Alt+T -> Terminal
        if (e.ctrlKey && e.altKey && e.key.toLowerCase() === 't') {
            e.preventDefault();
            toggleWindow('win-terminal');
        }
        // Ctrl+Alt+S -> Settings
        if (e.ctrlKey && e.altKey && e.key.toLowerCase() === 's') {
            e.preventDefault();
            toggleWindow('win-settings');
        }
        // Ctrl+Alt+A -> Toggle Simulated Screen Reader
        if (e.ctrlKey && e.altKey && e.key.toLowerCase() === 'a') {
            e.preventDefault();
            if (toggleScreenReader) {
                toggleScreenReader.checked = !toggleScreenReader.checked;
                // Dispatch change
                toggleScreenReader.dispatchEvent(new Event('change'));
            }
        }
    });

    // Print shortcut instructions to console
    console.log("%cΣ SIGMAOS ZENITH DESKTOP SIMULATOR MOUNTED", "color: #ffaa00; font-weight: bold; font-size: 14px;");
    console.log("Keyboard Shortcuts available:");
    console.log("- Ctrl+Alt+T : Toggle OmniShell Terminal");
    console.log("- Ctrl+Alt+S : Toggle Control Center Settings");
    console.log("- Ctrl+Alt+A : Toggle Screen Reader Simulator");
});
