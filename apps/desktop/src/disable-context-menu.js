// Right-clicking a plain WebView shows WebKit's own context menu (Reload,
// Inspect Element, etc.), which gives away that this window is a browser
// under the hood. PaulRussells File Share has no use for it, so suppress it everywhere.
document.addEventListener('contextmenu', (event) => event.preventDefault());
