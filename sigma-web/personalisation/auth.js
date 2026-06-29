// auth.js — Authentication page logic
import { auth } from './db.js';
import { navigate } from './app.js';

export function renderAuthPage(container) {
  container.innerHTML = `
    <div class="auth-wrap">
      <div class="auth-card glass-panel">
        <div class="auth-logo">
          <span class="logo-sigma">Σ</span>
          <span class="logo-text">SigmaOS</span>
        </div>
        <p class="auth-tagline">Personalisation Hub — Sovereign Identity</p>

        <div class="auth-tabs">
          <button class="auth-tab active" data-tab="signin">Sign In</button>
          <button class="auth-tab" data-tab="signup">Sign Up</button>
        </div>

        <form class="auth-form" id="auth-form">
          <div class="form-group">
            <label class="form-label" for="auth-email">Email</label>
            <input class="form-input" id="auth-email" type="email"
              placeholder="user@sigmaos.dev" autocomplete="email" required />
          </div>
          <div class="form-group">
            <label class="form-label" for="auth-password">Password</label>
            <input class="form-input" id="auth-password" type="password"
              placeholder="••••••••" autocomplete="current-password" required />
          </div>
          <div class="form-group" id="displayname-group" style="display:none">
            <label class="form-label" for="auth-displayname">Display Name</label>
            <input class="form-input" id="auth-displayname" type="text"
              placeholder="SigmaOS User" />
          </div>
          <div class="auth-error" id="auth-error" aria-live="polite"></div>
          <button class="btn btn-primary btn-full" type="submit" id="auth-submit">
            Sign In
          </button>
          <button class="btn btn-ghost btn-full" type="button" id="auth-forgot">
            Forgot password?
          </button>
        </form>

        <div class="auth-footer">
          <span class="badge-sovereign">🔒 Sovereign — no external telemetry</span>
        </div>
      </div>
    </div>
  `;

  let mode = 'signin';

  container.querySelectorAll('.auth-tab').forEach(tab => {
    tab.addEventListener('click', () => {
      mode = tab.dataset.tab;
      container.querySelectorAll('.auth-tab').forEach(t => t.classList.remove('active'));
      tab.classList.add('active');
      document.getElementById('auth-submit').textContent =
        mode === 'signin' ? 'Sign In' : 'Create Account';
      document.getElementById('displayname-group').style.display =
        mode === 'signup' ? '' : 'none';
      document.getElementById('auth-error').textContent = '';
    });
  });

  document.getElementById('auth-forgot').addEventListener('click', () => {
    navigate('/reset-password');
  });

  document.getElementById('auth-form').addEventListener('submit', async (e) => {
    e.preventDefault();
    const email    = document.getElementById('auth-email').value.trim();
    const password = document.getElementById('auth-password').value;
    const errEl    = document.getElementById('auth-error');
    errEl.textContent = '';

    const btn = document.getElementById('auth-submit');
    btn.disabled = true;
    btn.textContent = mode === 'signin' ? 'Signing in…' : 'Creating account…';

    let result;
    if (mode === 'signin') {
      result = await auth.signIn({ email, password });
    } else {
      result = await auth.signUp({ email, password });
    }

    btn.disabled = false;
    btn.textContent = mode === 'signin' ? 'Sign In' : 'Create Account';

    if (result.error) {
      errEl.textContent = result.error.message;
    } else {
      navigate('/dashboard/appearance');
    }
  });
}

export function renderResetPage(container) {
  container.innerHTML = `
    <div class="auth-wrap">
      <div class="auth-card glass-panel">
        <div class="auth-logo">
          <span class="logo-sigma">Σ</span>
          <span class="logo-text">SigmaOS</span>
        </div>
        <p class="auth-tagline">Password Reset</p>
        <form id="reset-form">
          <div class="form-group">
            <label class="form-label" for="reset-email">Email</label>
            <input class="form-input" id="reset-email" type="email" required />
          </div>
          <div class="auth-error" id="reset-msg" aria-live="polite"></div>
          <button class="btn btn-primary btn-full" type="submit">Send Reset Link</button>
          <button class="btn btn-ghost btn-full" type="button"
            onclick="window.location.hash='/auth'">← Back to Sign In</button>
        </form>
      </div>
    </div>
  `;
  document.getElementById('reset-form').addEventListener('submit', async (e) => {
    e.preventDefault();
    const email  = document.getElementById('reset-email').value.trim();
    const result = await auth.resetPassword({ email });
    const msg    = document.getElementById('reset-msg');
    if (result.error) {
      msg.textContent = result.error.message;
      msg.style.color = 'var(--color-error)';
    } else {
      msg.textContent = 'Check your email for a reset link.';
      msg.style.color = 'var(--color-success)';
    }
  });
}
