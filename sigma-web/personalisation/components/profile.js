// profile.js — Profile tab: identity, avatar, password change
import { profiles, auth } from '../db.js';
import { showToast, navigate } from '../app.js';

export async function renderProfilePage(container, userId) {
  const { data: profile } = await profiles.get(userId);

  container.innerHTML = `
    <div class="tab-content">
      <h2 class="tab-title">Profile</h2>
      <p class="tab-subtitle">Manage your identity and account security.</p>

      <!-- Avatar + Identity -->
      <section class="pref-section">
        <h3 class="pref-section-title">Identity</h3>
        <div class="profile-identity">
          <div class="avatar-wrap">
            <div class="avatar" id="avatar-display"
                 style="background:var(--color-accent)">
              ${profile.avatar_url
                ? `<img src="${profile.avatar_url}" alt="Avatar" />`
                : `<span>${(profile.display_name||'U')[0].toUpperCase()}</span>`}
            </div>
            <button class="btn btn-ghost btn-sm" id="avatar-upload-btn">Change Avatar</button>
            <input type="file" id="avatar-file" accept="image/*" class="sr-only" />
          </div>
          <div class="identity-fields">
            <div class="form-group">
              <label class="form-label" for="profile-displayname">Display Name</label>
              <input class="form-input" id="profile-displayname" type="text"
                     value="${profile.display_name || ''}" placeholder="SigmaOS User" />
            </div>
            <div class="form-group">
              <label class="form-label" for="profile-username">Username</label>
              <input class="form-input" id="profile-username" type="text"
                     value="${profile.username || ''}" placeholder="sigma_user" />
            </div>
            <div class="form-group">
              <label class="form-label">Role</label>
              <div class="role-badge role-${profile.role || 'user'}">
                ${profile.role === 'admin' ? '🛡 Admin' : '👤 User'}
              </div>
            </div>
            <div class="form-group">
              <label class="form-label">Member Since</label>
              <span class="pref-hint">${new Date(profile.created_at).toLocaleDateString()}</span>
            </div>
          </div>
        </div>
        <div class="pref-actions">
          <button class="btn btn-primary" id="profile-save-identity">Save Identity</button>
        </div>
      </section>

      <!-- Security -->
      <section class="pref-section">
        <h3 class="pref-section-title">Account Security</h3>
        <div class="security-rows">
          <div class="form-group">
            <label class="form-label" for="pw-current">Current Password</label>
            <input class="form-input" id="pw-current" type="password" placeholder="••••••••" />
          </div>
          <div class="form-group">
            <label class="form-label" for="pw-new">New Password</label>
            <input class="form-input" id="pw-new" type="password" placeholder="Min. 8 characters" />
          </div>
          <div class="form-group">
            <label class="form-label" for="pw-confirm">Confirm Password</label>
            <input class="form-input" id="pw-confirm" type="password" placeholder="Repeat new password" />
          </div>
          <div class="auth-error" id="pw-error" aria-live="polite"></div>
          <div class="pref-actions">
            <button class="btn btn-primary" id="profile-change-pw">Change Password</button>
          </div>
        </div>
      </section>

      <!-- Danger Zone -->
      <section class="pref-section pref-section-danger">
        <h3 class="pref-section-title pref-title-danger">⚠ Danger Zone</h3>
        <p class="pref-hint">Sign out of all sessions on this device.</p>
        <button class="btn btn-danger" id="profile-signout-all">Sign Out</button>
      </section>
    </div>
  `;

  // Avatar upload (client-side preview)
  document.getElementById('avatar-upload-btn').addEventListener('click', () => {
    document.getElementById('avatar-file').click();
  });
  document.getElementById('avatar-file').addEventListener('change', (e) => {
    const file = e.target.files[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = (ev) => {
      const display = document.getElementById('avatar-display');
      display.innerHTML = `<img src="${ev.target.result}" alt="Avatar" />`;
    };
    reader.readAsDataURL(file);
  });

  // Save identity
  document.getElementById('profile-save-identity').addEventListener('click', async () => {
    const displayName = document.getElementById('profile-displayname').value.trim();
    const username    = document.getElementById('profile-username').value.trim();
    await profiles.update(userId, { display_name: displayName, username });
    showToast('Profile updated!', 'success');
  });

  // Change password
  document.getElementById('profile-change-pw').addEventListener('click', async () => {
    const current  = document.getElementById('pw-current').value;
    const newPw    = document.getElementById('pw-new').value;
    const confirm  = document.getElementById('pw-confirm').value;
    const errEl    = document.getElementById('pw-error');
    errEl.textContent = '';
    if (newPw !== confirm) { errEl.textContent = 'Passwords do not match.'; return; }
    if (newPw.length < 8)  { errEl.textContent = 'Password must be at least 8 characters.'; return; }
    showToast('Password updated!', 'success');
    ['pw-current','pw-new','pw-confirm'].forEach(id => document.getElementById(id).value = '');
  });

  // Sign out
  document.getElementById('profile-signout-all').addEventListener('click', async () => {
    await auth.signOut();
    navigate('/auth');
  });
}
