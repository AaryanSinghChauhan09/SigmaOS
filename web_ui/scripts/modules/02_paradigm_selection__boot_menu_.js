document.addEventListener("DOMContentLoaded", () => {
    document.getElementById('btn-gui').addEventListener('click', () => {
        bootOverlay.classList.add('hidden');
        guiView.classList.remove('hidden');
        setTimeout(window.simulateBootProcess, 500);
        window.loadDirectory('/'); // Init GUI Explorer
    });

    document.getElementById('btn-cli').addEventListener('click', () => {
        bootOverlay.classList.add('hidden');
        cliView.classList.remove('hidden');
        document.getElementById('cli-input').focus();
    });
});