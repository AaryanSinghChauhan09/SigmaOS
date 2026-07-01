// db.js — Mock Supabase layer
// Swap this file for the real @supabase/supabase-js client by:
//   1. npm install @supabase/supabase-js
//   2. const { createClient } = await import('@supabase/supabase-js')
//   3. export const db = createClient(SUPABASE_URL, SUPABASE_ANON_KEY)
// All data shapes exactly match the described schema.

const STORE_KEY = 'sigma_personalisation_db';

function load() {
  try { return JSON.parse(localStorage.getItem(STORE_KEY) || '{}'); }
  catch { return {}; }
}
function save(data) { localStorage.setItem(STORE_KEY, JSON.stringify(data)); }

// ── Auth mock ───────────────────────────────────────────────────────────────
export const auth = {
  signUp: async ({ email, password }) => {
    const db = load();
    if (db.users?.[email]) return { error: { message: 'Email already registered.' } };
    const id = crypto.randomUUID();
    db.users = db.users || {};
    db.users[email] = { id, email, password, created_at: new Date().toISOString() };
    db.currentUser = { id, email };
    save(db);
    return { data: { user: db.currentUser }, error: null };
  },
  signIn: async ({ email, password }) => {
    const db = load();
    const u = db.users?.[email];
    if (!u || u.password !== password) return { error: { message: 'Invalid credentials.' } };
    db.currentUser = { id: u.id, email };
    save(db);
    return { data: { user: db.currentUser }, error: null };
  },
  signOut: async () => {
    const db = load(); db.currentUser = null; save(db);
    return { error: null };
  },
  getUser: () => {
    const db = load();
    return db.currentUser ? { data: { user: db.currentUser }, error: null }
                          : { data: { user: null }, error: null };
  },
  resetPassword: async ({ email }) => {
    const db = load();
    if (!db.users?.[email]) return { error: { message: 'No account found.' } };
    return { data: {}, error: null };
  },
};

// ── Profiles table ──────────────────────────────────────────────────────────
export const profiles = {
  get: async (userId) => {
    const db = load();
    const row = db.profiles?.[userId] || {
      id: userId, username: 'user', display_name: 'SigmaOS User',
      avatar_url: '', role: 'user', created_at: new Date().toISOString(),
    };
    return { data: row, error: null };
  },
  update: async (userId, patch) => {
    const db = load();
    db.profiles = db.profiles || {};
    db.profiles[userId] = { ...(db.profiles[userId] || {}), ...patch, id: userId };
    save(db);
    return { data: db.profiles[userId], error: null };
  },
};

// ── user_preferences table ──────────────────────────────────────────────────
export const userPreferences = {
  DEFAULT: {
    theme_mode: 'dark', accent_color: '#06b6d4',
    ui_density: 'comfortable', wallpaper_id: 'phosphor-grid', updated_at: '',
  },
  get: async (userId) => {
    const db = load();
    const row = { ...userPreferences.DEFAULT, ...(db.user_preferences?.[userId] || {}) };
    return { data: row, error: null };
  },
  update: async (userId, patch) => {
    const db = load();
    db.user_preferences = db.user_preferences || {};
    const existing = db.user_preferences[userId] || userPreferences.DEFAULT;
    db.user_preferences[userId] = { ...existing, ...patch, updated_at: new Date().toISOString() };
    save(db);
    return { data: db.user_preferences[userId], error: null };
  },
};

// ── functional_prefs table ──────────────────────────────────────────────────
export const functionalPrefs = {
  DEFAULT: {
    startup_apps: [], notification_rules: {},
    keyboard_shortcuts: {}, workspace_layout: 'default', updated_at: '',
  },
  get: async (userId) => {
    const db = load();
    const row = { ...functionalPrefs.DEFAULT, ...(db.functional_prefs?.[userId] || {}) };
    return { data: row, error: null };
  },
  update: async (userId, patch) => {
    const db = load();
    db.functional_prefs = db.functional_prefs || {};
    const existing = db.functional_prefs[userId] || functionalPrefs.DEFAULT;
    db.functional_prefs[userId] = { ...existing, ...patch, updated_at: new Date().toISOString() };
    save(db);
    return { data: db.functional_prefs[userId], error: null };
  },
};
