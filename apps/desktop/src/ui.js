// Shared interface helpers: theme toggle and the remembered keep-awake choice.

const THEME_KEY = 'pr-theme';
const KEEP_AWAKE_KEY = 'pr-keep-awake';

function load(key) {
  try {
    return localStorage.getItem(key);
  } catch {
    return null;
  }
}

function save(key, value) {
  try {
    localStorage.setItem(key, value);
  } catch {
    // Storage unavailable: the choice just won't be remembered.
  }
}

function applyTheme(theme, button) {
  document.documentElement.setAttribute('data-theme', theme);
  if (button) {
    const label = theme === 'dark' ? 'Switch to light theme' : 'Switch to dark theme';
    button.setAttribute('aria-label', label);
    button.title = label;
  }
}

/** Wires the header button that flips between the dark and light themes. */
export function initThemeToggle() {
  const button = document.getElementById('themeToggle');
  const current = document.documentElement.getAttribute('data-theme') === 'light' ? 'light' : 'dark';
  applyTheme(current, button);
  button?.addEventListener('click', () => {
    const next = document.documentElement.getAttribute('data-theme') === 'light' ? 'dark' : 'light';
    applyTheme(next, button);
    save(THEME_KEY, next);
  });

  // Keep every open window in the same theme.
  window.addEventListener('storage', (event) => {
    if (event.key === THEME_KEY && (event.newValue === 'light' || event.newValue === 'dark')) {
      applyTheme(event.newValue, button);
    }
  });
}

/** Restores the keep-awake switch to the user's last choice (on by default). */
export function initKeepAwakeToggle(input) {
  if (!input) return;
  input.checked = load(KEEP_AWAKE_KEY) !== 'off';
  input.addEventListener('change', () => save(KEEP_AWAKE_KEY, input.checked ? 'on' : 'off'));
}
