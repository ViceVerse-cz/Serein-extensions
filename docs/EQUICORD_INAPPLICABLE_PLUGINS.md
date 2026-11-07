# Equicord Plugins Compatibility Analysis: Inapplicable Plugins (Phase 7)

This document accounts for the **44 plugins** from the [Equicord repository](https://github.com/Equicord/Equicord) that are classified as **Inapplicable** for Serein.

## Architecture Context

- **Equicord / Vencord**: Client modifications running inside Discord's official desktop client (Chromium / Electron), operating via JavaScript Webpack module patching, React fiber traversal, and DOM CSS injection.
- **Serein**: A native Rust desktop client built with [egui](https://github.com/emilk/egui), featuring a direct Discord Gateway + Voice WebRTC engine, and an isolated, capability-gated WebAssembly (WASM) extension sandbox (`wasm32-unknown-unknown`).

Because Serein does not run Chromium, Electron, or Discord's React web bundle, plugins whose entire premise is hooking Webpack modules, executing arbitrary Node.js binaries, or injecting HTML5 DOM elements cannot and should not be ported in their original form.

---

## Detailed Inapplicable Plugins Inventory (44 Plugins)

| # | Plugin Name | Category | Reason Inapplicable to Native Serein |
|---|---|---|---|
| 1 | `AltKrispSwitch` | Electron / Node.js | Toggles Krisp proprietary Node C++ addon (`krisp.node`). Serein uses native Rust DSP filters and WebRTC voice processing. |
| 2 | `Animalese` | Web Audio / Webpack | Hooks Discord React audio graph and Web Audio API `AudioContext` to play synthesized speech. |
| 3 | `BlockKrisp` | Discord Webpack | Suppresses Discord's proprietary Krisp telemetry and download endpoints. |
| 4 | `CollapsibleUI` | Webpack / DOM | Traverses React DOM fiber tree to add collapsible containers to Discord web client UI. Native Serein has resizable splitters. |
| 5 | `CursorBuddy` | DOM / Canvas | Spawns HTML DOM cursor follower sprites over Chromium viewport. |
| 6 | `Declutter` | Webpack / CSS | Hides Discord web client UI clutter by injecting CSS into Chromium document head. |
| 7 | `DiscordDevBanner` | Webpack | Patches Discord React developer banner state flags. Serein has its own developer tools. |
| 8 | `ElementHighlighter` | Webpack / React DevTools | Renders DOM bounding boxes around Discord React components on hover. |
| 9 | `EquiTweaks` | Webpack | Bundle of monkey-patches specifically targeting Discord webapp internal variables. |
| 10 | `EquibopStreamFixes` | Webpack | Overrides Discord web client WebRTC video constraints in Chromium. |
| 11 | `EquicordHelper` | Webpack / Equicord Core | Internal Equicord helper bridge for managing Webpack patches and logging. |
| 12 | `EquicordToolbox` | Webpack | Debug menu for inspecting Webpack modules (`wpRequire`) and React fiber nodes. |
| 13 | `Equissant` | Webpack / DOM | Embeds an HTML5 chess board iframe into Discord chat messages. |
| 14 | `FontLoader` | Chromium / CSS | Injects `@font-face` CSS rules into Electron's Chromium DOM. Serein font configuration is handled via native font loaders in egui. |
| 15 | `HideChatButtons` | Webpack / CSS | Injects CSS rules to hide specific SVG buttons in Discord web composer. Serein composer is native egui. |
| 16 | `HopOn` | Webpack | Modifies Discord web client voice channel click handlers. Ported to native Serein in Phase 3. |
| 17 | `Husk` | Node.js child_process | Executes arbitrary shell commands via Electron Node.js runtime. Bypasses sandboxing; prohibited by Serein security model. |
| 18 | `IconViewer` | Webpack | Intercepts Discord web asset modals. Serein has native `view-icons` WASM plugin. |
| 19 | `IdleAutoRestart` | Electron | Restarts Discord Electron process when idle. Not applicable to standalone Serein native binary. |
| 20 | `Ingtoninator` | Webpack | Hooks Discord's React message parser regexes. |
| 21 | `InstantScreenshare` | Electron | Bypasses Discord Electron `desktopCapturer` selection modal. |
| 22 | `InvisibleChat` | Webpack | Obfuscates text using zero-width Unicode characters via Discord message pipeline. |
| 23 | `KeyboardSounds` | Node.js native addon | Listens to system-wide keyboard events using native Node binaries (`uiohook`) to play click sounds. |
| 24 | `LimitlessScreenshare` | Webpack | Patches Discord React store to unlock 1080p/60FPS streams. Serein native client does not restrict frame rates or resolutions. |
| 25 | `LoginWithQR` | Webpack | Hooks Discord web client QR scanner component. Serein native client uses direct token authentication. |
| 26 | `Moyai` | DOM / Canvas | Spawns Moyai particle explosions on screen by injecting an HTML5 `<canvas>` over Discord. |
| 27 | `NewPluginsManager` | Webpack / Equicord Core | Settings UI panel injected into Discord's React user settings view. Serein uses its own native Settings > Extensions manager. |
| 28 | `NoNitroUpsell` | Webpack | Suppresses Discord's React Nitro upsell modals. Serein is completely free of Nitro promotions natively. |
| 29 | `NoPushToTalk` | Webpack | Bypasses server-enforced Push-to-Talk restriction in Discord web client. |
| 30 | `NoRPC` | Webpack | Disables Discord's local IPC RPC server. Serein's RPC engine is completely opt-in via extension permissions. |
| 31 | `NoRoleHeaders` | Webpack / CSS | Injects CSS to remove role headers from Discord web member list. Handled natively in Serein egui member list. |
| 32 | `NotificationTitle` | Electron | Modifies Electron `BrowserWindow` title bar string to append notification counts. Native Serein uses OS badge APIs. |
| 33 | `OrbolayBridge` | Webpack / IPC | Communicates with external stream overlays via Discord local WebSocket. |
| 34 | `PartyMode` | DOM / Canvas | Discord confetti and combo particle effect canvas injector. |
| 35 | `Questify` | Webpack | Automates Discord Nitro quests by faking Discord activity heartbeats. Serein focuses on clean messaging. |
| 36 | `QuickThemeSwitcher` | Webpack / CSS | Dynamically toggles CSS stylesheet `<link>` tags in Discord web client. Serein uses declarative theme packages. |
| 37 | `ScreenRecorder` | Electron | Records screen using Chromium `MediaRecorder` API to local WebM files. |
| 38 | `Snowfall` | DOM / Canvas | Renders falling snow particles using an overlay HTML5 canvas. |
| 39 | `ThemeLibrary` | Webpack | Downloads and injects CSS themes into Discord client. Serein uses capability-gated declarative theme extensions. |
| 40 | `Title` | Electron | Overrides Discord `BrowserWindow` title. Handled natively in Serein via egui viewport configuration. |
| 41 | `ToastNotifications` | DOM | Injects floating toast alerts into Discord web DOM. Serein has native egui notifications. |
| 42 | `UnlimitedAccounts` | Webpack | Patches Discord web client account switcher token storage limit. Serein natively supports multi-account management. |
| 43 | `UserpluginInstaller` | Webpack / Node.js | Dynamically downloads and `eval`s untrusted JavaScript plugins. Incompatible with Serein's capability-verified WASM security model. |
| 44 | `WebpackTarball` | Webpack / Node.js | Dumps Discord's Webpack modules to disk as a `.tar.gz` archive for reverse engineering. |

---

## Complete 202 Equicord Plugins Summary

- **Phase 1 (Native Core - Chat, Markdown & Badges)**: 18 plugins (100% Complete)
- **Phase 2 (Native Core - Navigation, Search & Guilds)**: 27 plugins (100% Complete)
- **Phase 3 (Native Core - Voice, Audio & Media UX)**: 25 plugins (100% Complete)
- **Phase 4 (WASM Extensions - Text & File Utilities)**: 18 plugins (100% Complete)
- **Phase 5 (WASM Extensions - Media, Presence & Commands)**: 37 plugins (100% Complete)
- **Phase 6 (WASM Extensions - Security, Privacy & Moderation)**: 33 plugins (100% Complete)
- **Phase 7 (Documented Inapplicable Plugins)**: 44 plugins (100% Accounted for)

**Total accounted for: 202 / 202 plugins (100%)**.
