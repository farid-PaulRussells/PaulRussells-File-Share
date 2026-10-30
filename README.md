# PaulRussells File Share

Developed by **Faridul Islam**. Version 3.1.0.

Share files and folders straight from your computer by link or QR code, peer to peer.
Nothing is uploaded to a server and there is no size limit. Only the sender needs the app;
recipients just open the link in any browser.

Based on [Thawdrop](https://github.com/papateo/thawdrop) by papateo, used under the MIT Licence (see LICENSE).

## What's new in 1.1.0

- Redesigned interface with the Paul Russells logo in the header
- Dark and light themes, switched with the button at the top right (remembered between launches)
- "Keep PC awake while sharing" switch: blocks sleep mode only while a share is running, then
  hands control back to Windows. The screen can still turn off and lock. It does not move the
  mouse or press keys.
- Animation that turns the PC into a server when a share goes live
- Bigger logo on the app icon
- "Developed by Faridul Islam" at the bottom of every window

## Building with GitHub (no setup needed)

1. Upload the contents of this folder to your GitHub repository, replacing the old files.
2. Make sure `.github/workflows/build.yml` is in the repository.
3. Open the **Actions** tab. A build starts automatically; otherwise click
   **Build PaulRussells File Share → Run workflow**.
4. After 15–20 minutes, open the finished run and download **PaulRussells-File-Share-Windows**
   from **Artifacts**. The installer is in the `nsis` folder.

## Building on a Windows PC

Install Node.js LTS, Visual Studio Build Tools ("Desktop development with C++") and Rust
(rustup.rs), restart, unzip this folder to `C:\PaulRussells-File-Share`, open Command Prompt
there and run:

```
npm install
npm run desktop:build
```

The installer appears in `apps\desktop\src-tauri\target\release\bundle\nsis\`.

## Notes

- The app is not code-signed, so Windows shows "Windows protected your PC" (click
  **More info → Run anyway**) and Macs say the app is "damaged" (run
  `xattr -cr "/Applications/PaulRussells File Share.app"` in Terminal).
- Automatic updates are off. To ship updates later, host your own update feed and remove the
  `return;` line in `apps/desktop/src/update-gate.js`.
- Share links open on thawdrop.com, the original developer's receiver page. The code for it is
  in `apps/web` if you want a branded copy on your own domain; then set `ICENET_WEB_URL` when building.
- Interface files: `apps/desktop/index.html`, `share.html` and `src/theme.css`.
  Keep-awake code: `apps/desktop/src-tauri/src/keep_awake.rs`.
- Icon source: `apps/desktop/src-tauri/icons/icon-source-2048.png`.
