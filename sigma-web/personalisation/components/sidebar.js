// sidebar.js — Fixed left navigation sidebar
export function renderSidebar(container, currentRoute) {
  const navItems = [
    { icon: '🎨', label: 'Appearance', route: '/dashboard/appearance' },
    { icon: '⚙️', label: 'Behavior',   route: '/dashboard/behavior'   },
    { icon: '👤', label: 'Profile',    route: '/dashboard/profile'    },
  ];

  container.innerHTML = `
    <nav class="sidebar" id="sidebar" aria-label="Dashboard navigation">
      <div class="sidebar-brand" title="SigmaOS Personalisation Hub">
        <span class="sidebar-logo">Σ</span>
        <span class="sidebar-name">SigmaOS</span>
      </div>
      <ul class="sidebar-nav" role="list">
        ${navItems.map(item => `
          <li class="sidebar-item ${currentRoute === item.route ? 'active' : ''}">
            <a class="sidebar-link" href="#${item.route}"
               aria-current="${currentRoute === item.route ? 'page' : 'false'}">
              <span class="sidebar-icon" aria-hidden="true">${item.icon}</span>
              <span class="sidebar-label">${item.label}</span>
            </a>
          </li>`).join('')}
      </ul>
      <div class="sidebar-footer">
        <button class="sidebar-link sidebar-signout" id="sidebar-signout" title="Sign out">
          <span class="sidebar-icon">⏻</span>
          <span class="sidebar-label">Sign Out</span>
        </button>
        <div class="sidebar-version">v15.0 Zenith</div>
      </div>
    </nav>
  `;
}
