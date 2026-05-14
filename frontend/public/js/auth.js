// requires.management — auth state
// Central store for the current user. All components read from Auth.user.

let _user = null;
let _listeners = [];

export const Auth = {
    /** The current session user, or null if not logged in. */
    get user() { return _user; },

    /** True if the user is logged in at any tier. */
    get isLoggedIn() { return _user !== null; },

    /** Fetch the current session user from the API and store it. */
    async init() {
        try {
            const resp = await fetch('/api/v1/me');
            if (resp.status === 401) {
                _user = null;
                return;
            }
            const json = await resp.json();
            if (json.success && json.data) {
                _user = json.data;
            } else {
                _user = null;
            }
        } catch (e) {
            console.warn('Auth init failed:', e);
            _user = null;
        }
        _notify();
    },

    /** Register a callback to run whenever auth state changes. */
    onChange(fn) {
        _listeners.push(fn);
        return () => { _listeners = _listeners.filter(l => l !== fn); };
    },

    /** Redirect to the login route. */
    login() {
        window.location.href = '/auth/login';
    },

    /** POST to logout, then reload. */
    async logout() {
        await fetch('/auth/logout', { method: 'POST' });
        _user = null;
        _notify();
        window.location.href = '/';
    },

    /** Tier checks. */
    canAccessRegistered() {
        return ['registered', 'clan', 'admin'].includes(_user?.tier);
    },
    canAccessClan() {
        return ['clan', 'admin'].includes(_user?.tier);
    },
    isAdmin() {
        return _user?.tier === 'admin';
    },
};

function _notify() {
    _listeners.forEach(fn => fn(_user));
}
