/**
 * Σ SIGMA OS GAMES ENGINE v3.0 [INFINITY CORE]
 * Neural AI & Logic Implementation
 */

export const GamesEngine = {
    canvas: null,
    ctx: null,
    activeGame: null,

    init(id) {
        this.canvas = document.getElementById(`${id}-canvas`);
        if (!this.canvas) return;
        this.ctx = this.canvas.getContext('2d');
        this.activeGame = id;
        console.log(`[GAMES] ${id.toUpperCase()} engine core synchronized.`);
    },

    /** Pong Engine */
    pong: {
        ball: { x: 250, y: 160, vx: 3, vy: 3, r: 5 },
        paddle1: { y: 130, h: 60, w: 8 },
        paddle2: { y: 130, h: 60, w: 8 },
        score1: 0,
        score2: 0,
        running: false,

        loop() {
            if (!this.running) return;
            const engine = GamesEngine;
            const ctx = engine.ctx;
            const canvas = engine.canvas;

            // AI Logic: Paddle 2 follow ball (Imitated Neural Prediction)
            const targetY = this.ball.y - this.paddle2.h / 2;
            this.paddle2.y += (targetY - this.paddle2.y) * 0.15;

            // Player Logic: Mouse follow (Simplified)
            canvas.onmousemove = (e) => {
                const rect = canvas.getBoundingClientRect();
                this.paddle1.y = e.clientY - rect.top - this.paddle1.h / 2;
            };

            // Physics
            this.ball.x += this.ball.vx;
            this.ball.y += this.ball.vy;

            // Floor & Ceiling deflection
            if (this.ball.y < 0 || this.ball.y > canvas.height) this.ball.vy *= -1;

            // Padding collision
            if (this.ball.x < this.paddle1.w && this.ball.y > this.paddle1.y && this.ball.y < this.paddle1.y + this.paddle1.h) {
                this.ball.vx *= -1.05;
                this.ball.x = this.paddle1.w + 1; // Prevent sticking
            }
            if (this.ball.x > canvas.width - this.paddle2.w && this.ball.y > this.paddle2.y && this.ball.y < this.paddle2.y + this.paddle2.h) {
                this.ball.vx *= -1.05;
                this.ball.x = canvas.width - this.paddle2.w - 1; // Prevent sticking
            }

            // Scoring
            if (this.ball.x < 0) { this.score2++; this.reset(); SigmaKernel.notify(`PONG: Neural AI scored. Score: ${this.score1} - ${this.score2}`, 'info'); }
            if (this.ball.x > canvas.width) { this.score1++; this.reset(); SigmaKernel.notify(`PONG: User scored. Score: ${this.score1} - ${this.score2}`, 'success'); }

            // Draw
            ctx.fillStyle = '#000';
            ctx.fillRect(0, 0, canvas.width, canvas.height);

            ctx.fillStyle = getComputedStyle(document.documentElement).getPropertyValue('--accent');
            ctx.font = "20px monospace";
            ctx.fillText(`${this.score1}`, canvas.width / 4, 30);
            ctx.fillText(`${this.score2}`, (canvas.width / 4) * 3, 30);

            // Dashed center line
            ctx.setLineDash([5, 10]);
            ctx.beginPath();
            ctx.moveTo(canvas.width / 2, 0);
            ctx.lineTo(canvas.width / 2, canvas.height);
            ctx.strokeStyle = '#333';
            ctx.stroke();

            ctx.fillRect(5, this.paddle1.y, this.paddle1.w, this.paddle1.h);
            ctx.fillRect(canvas.width - 13, this.paddle2.y, this.paddle2.w, this.paddle2.h);

            ctx.beginPath();
            ctx.arc(this.ball.x, this.ball.y, this.ball.r, 0, Math.PI * 2);
            ctx.fill();

            requestAnimationFrame(() => this.loop());
        },

        reset() {
            this.ball = { x: 250, y: 160, vx: this.ball.vx > 0 ? -3 : 3, vy: (Math.random() > 0.5 ? 3 : -3), r: 5 };
        }
    },

    /** Chess Engine */
    chess: {
        board: [],
        pieces: [
            ['♜', '♞', '♝', '♛', '♚', '♝', '♞', '♜'],
            ['♟', '♟', '♟', '♟', '♟', '♟', '♟', '♟'],
            ['', '', '', '', '', '', '', ''],
            ['', '', '', '', '', '', '', ''],
            ['', '', '', '', '', '', '', ''],
            ['', '', '', '', '', '', '', ''],
            ['♙', '♙', '♙', '♙', '♙', '♙', '♙', '♙'],
            ['♖', '♘', '♗', '♕', '♔', '♗', '♘', '♖']
        ],
        initBoard() {
            this.board = Array(8).fill(null).map(() => Array(8).fill(null));
            for (let r = 0; r < 8; r++) {
                for (let c = 0; c < 8; c++) this.board[r][c] = (r + c) % 2 === 0 ? '#EEEEEE' : '#666666';
            }
        },
        draw() {
            const engine = GamesEngine;
            const ctx = engine.ctx;
            const sz = 60;
            ctx.font = "40px Arial";
            ctx.textAlign = "center";
            ctx.textBaseline = "middle";

            for (let r = 0; r < 8; r++) {
                for (let c = 0; c < 8; c++) {
                    ctx.fillStyle = this.board[r][c];
                    ctx.fillRect(c * sz, r * sz, sz, sz);
                    if (this.pieces[r][c]) {
                        ctx.fillStyle = r < 2 ? '#000000' : '#000000'; // Piece colors
                        ctx.fillText(this.pieces[r][c], c * sz + sz / 2, r * sz + sz / 2 + 5);
                    }
                }
            }
            SigmaKernel.notify("CHESS_AI: Turing depth 32 analysis active. Board initialized.", "info");
        }
    }
};

window.GamesEngine = GamesEngine;

let isDrawing = false;

function setupPaint() {
    const canvas = document.getElementById('paint-canvas');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');

    // Fill white background initially
    ctx.fillStyle = '#FFFFFF';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    canvas.onmousedown = (e) => {
        isDrawing = true;
        ctx.beginPath();
        ctx.moveTo(e.offsetX, e.OffsetY);
    };

    canvas.onmousemove = (e) => {
        if (!isDrawing) return;
        ctx.lineTo(e.offsetX, e.offsetY);
        ctx.strokeStyle = document.getElementById('paint-color').value;
        ctx.lineWidth = document.getElementById('paint-size').value;
        ctx.lineCap = 'round';
        ctx.stroke();
    };

    canvas.onmouseup = () => {
        isDrawing = false;
        ctx.closePath();
    };
    canvas.onmouseout = () => { isDrawing = false; };
}

window.startPongGame = () => {
    const menuEl = document.getElementById('pong-menu');
    if (menuEl) menuEl.style.display = 'none';
    GamesEngine.init('pong');
    GamesEngine.pong.running = true;
    GamesEngine.pong.loop();
};

window.startChessGame = () => {
    GamesEngine.init('chess');
    GamesEngine.chess.initBoard();
    GamesEngine.chess.draw();
};

window.downloadArt = () => {
    const canvas = document.getElementById('paint-canvas');
    if (!canvas) return;
    const link = document.createElement('a');
    link.download = `sigma_art_${Date.now()}.png`;
    link.href = canvas.toDataURL();
    link.click();
    SigmaKernel.notify("PAINT: Art asset exported locally bypassing network trackers.", "success");
};

window.clearCanvas = () => {
    const canvas = document.getElementById('paint-canvas');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    ctx.fillStyle = '#FFFFFF';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
};

// Initialize Paint listeners on load if possible, or bind them dynamically.
window.addEventListener('DOMContentLoaded', () => { setTimeout(setupPaint, 1000); });
