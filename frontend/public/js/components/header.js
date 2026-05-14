// requires.management — header component
import { Auth } from '../auth.js';

export function renderHeader(el) {
    function build() {
        const user = Auth.user;

        el.innerHTML = `
            <div class="header-inner">
                <a href="/" data-route class="header-wordmark">
                    requires<span>.</span>management
                </a>

                <nav class="header-nav" id="header-nav">
                    <!-- Service nav links injected by each service page -->
                </nav>

                <div class="header-actions">
                    ${user ? renderUserMenu(user) : renderLoginButton()}
                </div>
            </div>
        `;

        // Wire logout
        el.querySelector('#btn-logout')?.addEventListener('click', () => Auth.logout());
    }

    build();
    Auth.onChange(() => build());
}

function renderLoginButton() {
    return `
        <button class="btn btn-discord" onclick="window.location.href='/auth/login'">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                <path d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0 12.64 12.64 0 0 0-.617-1.25.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057c.002.022.015.043.03.056a19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028 14.09 14.09 0 0 0 1.226-1.994.076.076 0 0 0-.041-.106 13.107 13.107 0 0 1-1.872-.892.077.077 0 0 1-.008-.128 10.2 10.2 0 0 0 .372-.292.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127 12.299 12.299 0 0 1-1.873.892.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028 19.839 19.839 0 0 0 6.002-3.03.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03z"/>
            </svg>
            Sign in with Discord
        </button>
    `;
}

function renderUserMenu(user) {
    const tierBadge = {
        registered: '<span class="badge badge-registered">registered</span>',
        clan:        '<span class="badge badge-clan">clan</span>',
        admin:       '<span class="badge badge-admin">admin</span>',
    }[user.tier] ?? '';

    const avatarContent = user.avatar_url
        ? `<img src="${user.avatar_url}" alt="${user.display_name}" />`
        : `<span style="font-size:var(--text-xs);color:var(--colour-text-1);display:flex;align-items:center;justify-content:center;height:100%">${user.display_name[0].toUpperCase()}</span>`;

    return `
        <div style="display:flex;align-items:center;gap:var(--space-3)">
            ${tierBadge}
            <span style="font-size:var(--text-sm);color:var(--colour-text-1)">${user.display_name}</span>
            <div class="user-avatar">${avatarContent}</div>
            <button class="btn btn-ghost" id="btn-logout" style="padding:var(--space-1) var(--space-3)">
                sign out
            </button>
        </div>
    `;
}
