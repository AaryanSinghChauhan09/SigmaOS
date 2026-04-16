document.addEventListener("DOMContentLoaded", () => {
const overlay = document.getElementById('boot-overlay');
    const guiView = document.getElementById('gui-view');
    const cliView = document.getElementById('cli-view');

    document.getElementById('btn-gui').addEventListener('click', () => {
        overlay.classList.add('hidden');
        guiView.classList.remove('hidden');
        setTimeout(simulateBootProcess, 500);
        loadDirectory('/'); // Init GUI Explorer
    });

    document.getElementById('btn-cli').addEventListener('click', () => {
        overlay.classList.add('hidden');
        cliView.classList.remove('hidden');
        document.getElementById('cli-input').focus();
    });
});