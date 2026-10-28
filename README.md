# PaulRussells File Share

Developed by **Faridul Islam**. Version 2.0.0.

Share files and folders straight from your computer by link or QR code, peer to peer.
Nothing is uploaded to a server and there is no size limit. Only the sender needs the app;
recipients just open the link in any browser.

Based on [Thawdrop](https://github.com/papateo/thawdrop) by papateo, used under the MIT Licence (see LICENSE).

---

## Building on Windows

**1. Install the tools (one time only)**

- **Node.js LTS**: https://nodejs.org (accept the defaults)
- **Visual Studio Build Tools**: https://visualstudio.microsoft.com/visual-cpp-build-tools/
  In the installer, tick **"Desktop development with C++"**.
- **Rust**: https://rustup.rs. Run `rustup-init.exe` and accept the defaults.

Restart the computer after installing.

**2. Build**

Unzip this folder, open **PowerShell** inside it, and run:

```
npm install
npm run desktop:build
```

The first build takes 5 to 15 minutes. Your installers will be in:

```
apps\desktop\src-tauri\target\release\bundle\nsis\   ← PaulRussells File Share_1.0.0_x64-setup.exe
apps\desktop\src-tauri\target\release\bundle\msi\    ← .msi installer
```

**3. Optional: add the right-click "Share with PaulRussells File Share" menu**

After installing the app:

```
powershell -ExecutionPolicy Bypass -File .\scripts\windows\install.ps1 -ExePath "C:\Program Files\PaulRussells File Share\PaulRussells File Share.exe"
```

(Check the actual install path if you chose a different folder.)

---

## Building on Mac

```
xcode-select --install                                           # Apple build tools
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   # Rust
# Install Node.js LTS from https://nodejs.org
npm install
npm run desktop:build
```

The .dmg appears in `apps/desktop/src-tauri/target/release/bundle/dmg/`.
Because it isn't signed with an Apple Developer certificate, the first time you open it, right-click the app → **Open**.
Right-click menu: `./scripts/macos/install.sh "/Applications/PaulRussells File Share.app"`

---

## Notes

- **Windows SmartScreen** will warn "unknown publisher" until the installer is code-signed with a certificate in the company name.
- **Automatic updates are off.** To ship updates later, host your own update feed and remove the `return;` line in `apps/desktop/src/update-gate.js`.
- **Share links** still open on thawdrop.com, the original developer's receiver page. The code for it is in `apps/web` if you want to host a branded version on your own domain; then set `ICENET_WEB_URL` when building.
- The icon source is `apps/desktop/src-tauri/icons/icon-source-2048.png`.
