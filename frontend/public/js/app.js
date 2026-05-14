// requires.management — app entry point
import { Auth } from './auth.js';
import { Router } from './router.js';
import { renderHeader } from './components/header.js';
import { renderHome } from './pages/home.js';

async function init() {
    // Boot sequence: fetch current user, then render
    await Auth.init();

    // Render persistent header
    const headerEl = document.getElementById('site-header');
    if (headerEl) renderHeader(headerEl);

    // Set up router
    const router = new Router({
        '/':        renderHome,
    });

    router.start();

    // Remove initial loader
    document.getElementById('initial-loader')?.remove();
}

init().catch(err => {
    console.error('App init failed:', err);
    const loader = document.getElementById('initial-loader');
    if (loader) {
        loader.innerHTML = '<span class="loader-text" style="color: var(--colour-danger)">initialisation failed — check console</span>';
    }
});
