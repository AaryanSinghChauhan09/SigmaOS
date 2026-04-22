const { spawn } = require('child_process');
const path = require('path');
const { ROOT_DIR } = require('../config');

module.exports = (req, res) => {
    let body = '';
    req.on('data', chunk => body += chunk.toString());
    req.on('end', () => {
        try {
            const { cmd, cwd } = JSON.parse(body);
            res.writeHead(200, {
                'Content-Type': 'text/plain; charset=utf-8',
                'Transfer-Encoding': 'chunked'
            });

            const child = spawn(cmd, { shell: true, cwd: path.join(ROOT_DIR, cwd || '') });
            
            child.stdout.on('data', data => res.write(data.toString()));
            child.stderr.on('data', data => res.write(`[ERR] ${data.toString()}`));
            child.on('close', code => res.end(`\n[Process Exited: ${code}]`));
            child.on('error', err => res.end(`\n[Spawn Error: ${err.message}]`));
        } catch (e) {
            res.writeHead(400);
            res.end('Invalid Payload');
        }
    });
};
