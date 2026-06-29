// app.js — Main Application Orchestration & SPA Router
import { auth } from './db.js';
import { initTheme } from './theme.js';
import { renderAuthPage, renderResetPage } from './auth.js';
import { renderSidebar } from './components/sidebar.js';
import { renderLivePreview } from './components/live-preview.js';
import { renderAppearancePage } from './components/appearance.js';
import { renderBehaviorPage } from './components/behavior.js';
import { renderProfilePage } from './components/profile.js';

// ── Routing Config ────────────────────────────────────────────────────────
const ROUTES = {
  '/auth':              { render: renderAuthPage,     public: true },
  '/reset-password':    { render: renderResetPage,    public: true },
  '/dashboard/appearance': { render: renderAppearancePage, public: false },
  '/dashboard/behavior':   { render: renderBehaviorPage,   public: false },
  '/dashboard/profile':    { render: renderProfilePage,    public: false },
};

export function navigate(route) {
  window.location.hash = route;
}

// ── Toast Notification System ─────────────────────────────────────────────
export function showToast(message, type = 'success') {
  const container = document.getElementById('toast-container');
  const toast = document.createElement('div');
  toast.className = `toast toast-${type}`;
  toast.innerHTML = `
    <span>${type === 'success' ? '✓' : type === 'error' ? '❌' : 'ℹ️'}</span>
    <span>${message}</span>
  `;
  container.appendChild(toast);

  setTimeout(() => {
    toast.style.animation = 'slideIn 0.3s ease-out reverse';
    setTimeout(() => toast.remove(), 300);
  }, 3000);
}

// ── Shell Rendering ───────────────────────────────────────────────────────
async function renderShell(route, userId) {
  const root = document.getElementById('app-root');
  
  // Enforce correct outer container structure
  root.innerHTML = `
    <div id="sidebar-container"></div>
    <main class="main-content">
      <div class="content-body" id="content-body"></div>
      <aside class="preview-sidebar" id="preview-container"></aside>
    </main>
  `;

  // Render Sidebar
  const sidebarContainer = document.getElementById('sidebar-container');
  renderSidebar(sidebarContainer, route);

  // Hook signout button
  const signoutBtn = document.getElementById('sidebar-signout');
  if (signoutBtn) {
    signoutBtn.addEventListener('click', async () => {
      await auth.signOut();
      navigate('/auth');
    });
  }

  // Set up live preview container
  const previewContainer = document.getElementById('preview-container');
  const contentBody = document.getElementById('content-body');

  // Render active route
  const target = ROUTES[route];
  if (target) {
    await target.render(contentBody, userId, previewContainer);
  }
}

// ── Main Router Entry ────────────────────────────────────────────────────
async function handleRouting() {
  let hash = window.location.hash.slice(1) || '/dashboard/appearance';
  
  // Normalize routes
  if (hash === '/' || hash === '/dashboard') {
    hash = '/dashboard/appearance';
  }

  const { data } = auth.getUser();
  const user = data?.user;

  const target = ROUTES[hash];

  if (!target) {
    navigate('/auth');
    return;
  }

  // Auth gate
  if (!target.public && !user) {
    navigate('/auth');
    return;
  }

  if (target.public && user) {
    navigate('/dashboard/appearance');
    return;
  }

  // Init theme properties for current user
  if (user) {
    await initTheme(user.id);
  }

  const root = document.getElementById('app-root');

  if (target.public) {
    // Render standalone page (no sidebar/preview shell)
    root.innerHTML = `<div id="auth-page-container"></div>`;
    target.render(document.getElementById('auth-page-container'));
  } else {
    // Render full settings shell layout
    await renderShell(hash, user.id);
  }
}

// ── Listeners ─────────────────────────────────────────────────────────────
window.addEventListener('hashchange', handleRouting);
window.addEventListener('DOMContentLoaded', handleRouting);
