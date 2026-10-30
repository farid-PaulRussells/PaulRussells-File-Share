// Runs before the page is drawn so the chosen theme shows without a flash.
// Dark is the default; a light-mode system preference is used only until the
// user picks a theme with the toggle.
(function () {
  var theme = null;
  try {
    theme = localStorage.getItem('pr-theme');
  } catch (e) {}
  if (theme !== 'light' && theme !== 'dark') {
    theme = window.matchMedia && window.matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark';
  }
  document.documentElement.setAttribute('data-theme', theme);
})();
