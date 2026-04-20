document.addEventListener("DOMContentLoaded", () => {
const cliOutput = document.getElementById('cli-output');
    const cliInput = document.getElementById('cli-input');
    let cliCurrentDir = '/';

    const renderCli = (html) => {
        const div = document.createElement('div');
        div.innerHTML = html;
        cliOutput.appendChild(div);
        cliOutput.scrollTop = cliOutput.scrollHeight;
    };

    cliInput.addEventListener('keydown', async (e) => {
        if (e.key === 'Enter') {
            const cmdText = cliInput.value.trim();
            cliInput.value = '';
            renderCli(`<div><span style="color:var(--acc-cyan)">root@sigma-zenith:${cliCurrentDir}#</span> ${cmdText}</div>`);
            
            if (!cmdText) return;
            const args = cmdText.split(' ');
            const cmd = args[0].toLowerCase();

            switch (cmd) {
                case 'help':
                    renderCli(`<div style="color:#aaa">SigmaOS Shell Commands:<br>ls [dir] - List files<br>cat &lt;file&gt; - Read file contents<br>cd &lt;dir&gt; - Move directory<br>clear - Clear shell<br>gui - Switch to Zenith GUI mode</div><br>`);
                    break;
                case 'clear':
                    cliOutput.innerHTML = '';
                    break;
                case 'gui':
                    cliView.classList.add('hidden');
                    guiView.classList.remove('hidden');
                    setTimeout(window.simulateBootProcess, 500);
                    window.loadDirectory('/');
                    break;
                case 'cd': {
                    let dir = args[1] || '/';
                    if (dir === '..') {
                        if (cliCurrentDir !== '/') {
                            const p = cliCurrentDir.split('/').filter(Boolean);
                            p.pop();
                            cliCurrentDir = '/' + p.join('/');
                        }
                    } else {
                        if (!dir.startsWith('/')) {
                            dir = cliCurrentDir === '/' ? `/${dir}` : `${cliCurrentDir}/${dir}`;
                        }
                        cliCurrentDir = dir;
                    }
                    try {
                        const res = await fetch(`/api/fs?path=${encodeURIComponent(cliCurrentDir)}`);
                        if (!res.ok) {
                            renderCli(`<div style="color:#ff5f56">bash: cd: ${dir}: No such file or directory</div><br>`);
                            cliCurrentDir = '/';
                        }
                    } catch(e) {
                         renderCli(`<div style="color:#ff5f56">Network error communicating with file orchestrator.</div><br>`);
                    }
                    document.querySelector('.cli-prompt').textContent = `root@sigma-zenith:${cliCurrentDir}#`;
                    break;
                }
                case 'clear':
                    cliOutput.innerHTML = '';
                    break;
                default:
                    // PILLAR 2: Run Real Native Commands
                    try {
                        const resp = await fetch('/api/run', {
                            method: 'POST',
                            headers: { 'Content-Type': 'application/json' },
                            body: JSON.stringify({ cmd: cmdText, cwd: cliCurrentDir })
                        });
                        
                        const reader = resp.body.getReader();
                        const decoder = new TextDecoder("utf-8");
                        
                        let currentLine = document.createElement('div');
                        currentLine.style.whiteSpace = 'pre-wrap';
                        cliOutput.appendChild(currentLine);
                        
                        while(true) {
                            const { done, value } = await reader.read();
                            if (done) break;
                            currentLine.innerHTML += decoder.decode(value).replace(/</g, '&lt;').replace(/>/g, '&gt;');
                            cliOutput.scrollTop = cliOutput.scrollHeight;
                        }
                        renderCli('<br>');
                    } catch(e) {
                        renderCli(`<div style="color:#ff5f56">Engine Execution Error: ${e.message}</div><br>`);
                    }
            }
        }
    });

    // Make sure click on CLI empty space focuses input
    cliView.addEventListener('click', () => cliInput.focus());
});