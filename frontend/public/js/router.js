// requires.management — client-side router
// Minimal SPA router. Maps pathnames to render functions.
// Each render function receives the main element and any route params.

export class Router {
    constructor(routes) {
        this._routes = routes;
        this._main = document.getElementById('site-main');
    }

    start() {
        // Initial render
        this._render(window.location.pathname);

        // Handle browser back/forward
        window.addEventListener('popstate', () => {
            this._render(window.location.pathname);
        });

        // Intercept internal link clicks
        document.addEventListener('click', e => {
            const anchor = e.target.closest('a[data-route]');
            if (!anchor) return;
            e.preventDefault();
            this.navigate(anchor.getAttribute('href'));
        });
    }

    navigate(path) {
        window.history.pushState({}, '', path);
        this._render(path);
    }

    _render(path) {
        const handler = this._routes[path] ?? this._routes['/'];
        if (!handler) return;

        if (this._main) {
            handler(this._main);
        }
    }
}
