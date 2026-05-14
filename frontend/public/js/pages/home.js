// requires.management — home page
import { Auth } from '../auth.js';

const SERVICES = [
    {
        id:        'warframe',
        name:      'Warframe',
        glyph:     '⬡',
        subdomain: 'warframe.requires.management',
        desc:      'Market pricing, relic planners, drop tables, world state timers, and in-game companion features.',
        tag:       'warframe companion',
        available: true,
    },
    {
        id:        'nodesync',
        name:      'NodeSync',
        glyph:     '◈',
        subdomain: 'nodesync.requires.management',
        desc:      'Infrastructure management dashboard for the Decrypt ecosystem. Monitor, deploy, and manage nodes.',
        tag:       'infrastructure',
        available: false,
    },
    {
        id:        'persona',
        name:      'Persona',
        glyph:     '◉',
        subdomain: 'persona.requires.management',
        desc:      'Personal dashboard — finance tracking, pantry management, meal planning, and health data.',
        tag:       'personal',
        available: false,
    },
];

export function renderHome(el) {
    const user = Auth.user;

    el.innerHTML = `
        <section class="page-hero">
            <div class="hero__grid"></div>
            <div class="hero__scanlines"></div>
            <div class="hero__glyph" aria-hidden="true">RM</div>

            <div class="hero__content">
                <span class="eyebrow hero__eyebrow">platform // decrypt ecosystem</span>

                <h1 class="hero__headline">
                    requires<span class="text-accent">.</span>management
                </h1>

                <p class="hero__body">
                    A suite of purpose-built tools for Warframe, infrastructure,
                    and personal automation. One login. Every tool.
                </p>

                <div class="hero__cta">
                    ${user
                        ? `<span class="font-mono text-small text-secondary">
                               &gt; authenticated // <span class="text-accent">${user.display_name}</span>
                           </span>`
                        : `<button class="btn-discord" onclick="window.location.href='/auth/login'">
                               ${discordIcon()} sign in with discord
                           </button>
                           <a href="#services" class="btn-ghost">explore tools</a>`
                    }
                </div>
            </div>
        </section>

        <section class="rm-section" id="services">
            <div class="section-inner">
                <span class="eyebrow">01 // services</span>
                <h2>Available Tools</h2>
                <p class="rm-section__intro">
                    Each tool lives at its own subdomain. Sign in once with Discord and
                    your session carries across the platform.
                </p>

                <div class="service-grid">
                    ${SERVICES.map(renderServiceCard).join('')}
                </div>
            </div>
        </section>

        <section class="rm-section">
            <div class="section-inner">
                <span class="eyebrow">02 // access</span>
                <h2>Tiers</h2>
                <p class="rm-section__intro">
                    Most tools are public. Sign in to unlock personal features.
                    Clan members get access to clan-specific tools and private guides.
                </p>

                <div style="display:grid;grid-template-columns:repeat(auto-fill,minmax(240px,1fr));gap:var(--space-4)">
                    ${renderTierCard('Public', '◌', 'No login required. Market lookups, drop tables, timers, and calculators.')}
                    ${renderTierCard('Registered', '◎', 'Discord login. Personal watchlists, saved builds, and trade history.')}
                    ${renderTierCard('Clan', '⬡', 'Clan server member. Private guides, clan tools, and shared resources.')}
                </div>
            </div>
        </section>

        <footer class="site-footer">
            <div class="site-footer__inner">
                <span class="site-footer__copy">
                    requires<span class="text-accent">.</span>management
                    // decrypt ecosystem
                </span>
                <span class="site-footer__copy">
                    ${user
                        ? `session active // <span class="text-accent">${user.display_name}</span>`
                        : 'not authenticated'}
                </span>
            </div>
        </footer>
    `;

    // Smooth scroll for anchor link
    el.querySelector('a[href="#services"]')?.addEventListener('click', e => {
        e.preventDefault();
        document.getElementById('services')?.scrollIntoView({ behavior: 'smooth' });
    });
}

function renderServiceCard(service) {
    const offline = !service.available;
    const href    = service.available ? `https://${service.subdomain}` : '#';

    return `
        <a class="service-card ${offline ? 'service-card--offline' : ''}"
           href="${href}"
           ${service.available ? 'target="_blank"' : 'tabindex="-1"'}>
            <div style="display:flex;align-items:flex-start;gap:var(--space-3)">
                <span style="font-size:1.8rem;color:var(--color-accent);
                             line-height:1;text-shadow:0 0 12px color-mix(in srgb,var(--color-accent) 40%,transparent)">
                    ${service.glyph}
                </span>
                <div style="display:flex;flex-direction:column;gap:var(--space-2);flex:1">
                    <div style="display:flex;align-items:baseline;gap:var(--space-3);flex-wrap:wrap">
                        <span class="service-card__name">${service.name}</span>
                        <span class="service-card__badge ${service.available ? 'service-card__badge--available' : 'service-card__badge--offline'}">
                            ${service.available ? 'online' : 'offline'}
                        </span>
                    </div>
                </div>
            </div>
            <p class="service-card__desc">${service.desc}</p>
            <div class="service-card__footer">
                <span class="service-card__tag">${service.subdomain}</span>
            </div>
        </a>
    `;
}

function renderTierCard(name, glyph, desc) {
    return `
        <div class="pillar-card">
            <div class="pillar-card__icon">${glyph}</div>
            <div class="pillar-card__title">${name}</div>
            <div class="pillar-card__body">${desc}</div>
        </div>
    `;
}

function discordIcon() {
    return `<svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="flex-shrink:0">
        <path d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0 12.64 12.64 0 0 0-.617-1.25.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057c.002.022.015.043.03.056a19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028 14.09 14.09 0 0 0 1.226-1.994.076.076 0 0 0-.041-.106 13.107 13.107 0 0 1-1.872-.892.077.077 0 0 1-.008-.128 10.2 10.2 0 0 0 .372-.292.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127 12.299 12.299 0 0 1-1.873.892.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028 19.839 19.839 0 0 0 6.002-3.03.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03z"/>
    </svg>`;
}
