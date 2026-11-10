# Tauri 2 平台能力与无公证发布边界

> Research ticket: [#3 研究 Tauri 2 能力与无公证发布边界](https://github.com/NibbleOrg/Nibble-desktop/issues/3)
>
> Checked: 2026-11-10
>
> Status: research evidence only. Product, interaction, architecture, and release decisions remain owned by their wayfinder tickets.

## Short version

- Tauri 2 can host the two Nibble surfaces as separate webview windows with separate window labels and least-privilege capability sets. One Rust core process owns OS access, global state, SQLite connections, and IPC routing; the WebView processes run the React UI.
- Global shortcuts, system tray menus/events, SQLite migrations, single-instance behavior, and signed updater artifacts are supported by official Tauri plugins.
- The official clipboard plugin supports read/write operations, but it does not expose clipboard-change events or clipboard-history enumeration. Nibble must add change detection itself. On macOS, `NSPasteboard.changeCount` increments when pasteboard ownership changes and can be compared over time to detect changes.
- Clipboard access is security- and privacy-sensitive. Tauri enables no clipboard permissions by default. On macOS 15.4 and later, Apple documents the general pasteboard's default programmatic access behavior as `ask`; the user may deny access. A clipboard-history product must design onboarding, denied, and revoked-permission states.
- Tauri's updater signature is independent of Apple code signing and cannot be disabled. The public key ships in the app; the private key signs update artifacts and must be protected. Losing the private key prevents future updates for installed users.
- Without an Apple Developer account, Nibble cannot produce a Developer ID-signed and notarized macOS build. Tauri can apply an ad-hoc signature, but Tauri explicitly cautions that this does not prevent macOS from requiring the user to whitelist the app in Privacy & Security. Apple describes this override as a manual security exception and warns about its malware risk.

## Platform findings

### Two surfaces and process boundaries

Tauri's process model has one **core process** with full OS access and one or more **WebView processes** that render HTML/CSS/JavaScript. The core creates windows, tray menus, and notifications, routes all IPC, and is the documented place for global state such as settings and database connections. Tauri recommends keeping business logic and secrets out of the frontend to reduce the attack surface.

The stable desktop multi-window primitive is `WebviewWindow`. Windows have labels such as `workbench` and `quick-panel`; Tauri capabilities can target individual window labels. Creating a webview window from JavaScript is not part of the default capability set, so the permission must be explicit. Tauri recommends giving each window only the features it needs.

Relevant constraints:

- Nibble can expose two independently designed React shells without creating two applications.
- The two WebViews should not each own independent domain stores. The documented Tauri architecture places shared app state and persistence in the core process.
- Hidden/summoned windows, focus changes, close requests, and show/hide can be implemented through window APIs, but the final lifecycle policy remains a product/UX decision.

### Single-instance behavior

The official single-instance plugin prevents multiple app processes from running. It must be registered before other plugins to work reliably. A second launch is closed and invokes a callback that receives the app handle, command-line arguments, and working directory. The documented example focuses the existing main window; if no action is taken, a second launch has no visible effect.

This supports a product behavior where reopening Nibble focuses or summons the existing instance, but that behavior still belongs to the interaction ticket.

### Global shortcut

The official global-shortcut plugin supports macOS, Windows, and Linux. It exposes registration, unregistration, and registration checks, with press/release states available in Rust. No shortcut capability is enabled by default; each command must be explicitly granted to the responsible window.

Constraints:

- The UI must handle registration failure and shortcut conflicts rather than assuming the requested chord is always available.
- Keeping the global-shortcut permission away from the Workbench WebView is possible if only the core process needs it.
- Shortcut choice, rebinding UX, and summon/dismiss semantics are not supplied by the plugin.

### Clipboard history and clipboard monitoring

The official clipboard-manager plugin supports text, image, HTML write, text/image read, clear, and secret-text write on desktop. Clipboard capabilities are disabled by default because clipboard content is inherently sensitive.

Two gaps matter for Nibble:

1. **No change monitor.** The JavaScript API is limited to read/write/clear operations. The Rust desktop implementation uses `arboard` and exposes the same operations. It does not register a change callback or stream pasteboard changes.
2. **No clipboard-history model.** The plugin reads the current text/image; it does not enumerate, deduplicate, persist, or classify a history of items.

On macOS, AppKit documents `NSPasteboard.changeCount`: it increments whenever pasteboard ownership changes. A native monitor can periodically compare the stored count and read content only after a change. This reduces unnecessary content reads compared with reading the pasteboard every tick, but polling frequency, power use, and the macOS privacy prompt must still be tested on supported OS versions.

Apple's current documentation for `NSPasteboard.AccessBehavior` (available on macOS 15.4+) states that the general pasteboard's default programmatic access behavior is `ask`. The first attempted access can show a permission alert. The user can then choose `ask`, `alwaysAllow`, or `alwaysDeny` in System Settings. User-originated, paste-related access is exempt from the alert. Therefore a history product needs explicit states for "not yet requested", "allowed", "denied", and "revoked".

The plugin's `writeSecret` operation asks clipboard managers to exclude sensitive content on macOS and Linux. Nibble should not assume this hint is sufficient for its own persistence layer: it still needs a policy for honoring the hint, avoiding feedback loops when Nibble writes history back to the pasteboard, and suppressing sensitive captures.

### Tray and resident lifecycle

Tauri provides a system tray icon, menus, and tray mouse events on desktop when the `tray-icon` feature is enabled. The official example handles a tray click by showing, unminimizing, and focusing a window by label. A tray menu item can explicitly exit the process.

The tray API creates access to actions; it does not decide what closing the Workbench means. Tauri exposes window close-request, show/hide, focus, and destroy operations, so "hide Workbench and remain resident in the tray" is implementable. The app must explicitly implement that lifecycle and a clear Quit action; otherwise standard close behavior may terminate or leave an inconsistent state.

### SQLite

The official SQL plugin supports SQLite through `sqlx`, including migrations defined in Rust. Migrations run inside a transaction; if one fails, the transaction rolls back. Each migration needs a unique version and should be idempotent and tested.

The default SQL permission set permits loading/closing and read operations. Executing arbitrary SQL, including writes, requires the explicit `sql:allow-execute` permission. Tauri's process-model guidance favors keeping the database and sensitive business logic in the core rather than granting the frontend broad SQL access.

For Nibble, the facts support a single local database with versioned migrations and a core-owned repository boundary. The concrete schema, retention, encryption, and query API remain decisions for the data and architecture tickets.

### Updater signing

Tauri's updater checks a signature and its signature verification cannot be disabled. Setup requires:

- A Tauri signer public key embedded in `tauri.conf.json` as content, not a file path.
- The matching private key, kept out of the frontend and repository, used to sign each update artifact.
- Updater artifacts enabled at bundle time.
- One or more HTTPS endpoints; production TLS is enforced by default.

The updater supports static JSON, including a GitHub Releases `latest.json` URL, or a dynamic update server. The static manifest requires a valid version, per-platform URL and signature, and Tauri validates the entire manifest before checking the version.

Losing the private signing key makes it impossible to issue trusted updates to users who already installed the app. This is a release-security constraint, not an Apple signing requirement.

### macOS packaging, signing, and notarization

Tauri produces a macOS `.app` bundle and supports DMG and other bundle formats. The default minimum system version is 10.13 and can be raised in configuration.

Tauri's documentation distinguishes three states:

1. **Developer ID signed and notarized:** requires an Apple Developer account, a Developer ID certificate, and an Apple notarization flow. This is the path for a low-friction direct download.
2. **Ad-hoc signed:** configured with signing identity `-`; does not use an Apple-authenticated identity. Tauri recommends ad-hoc signing for Apple Silicon apps distributed from GitHub to avoid macOS treating downloaded builds as damaged.
3. **Unauthenticated/no Apple signature:** the most fragile path; Gatekeeper may refuse launch or report that the app cannot be verified.

Ad-hoc signing is not equivalent to Apple signing or notarization. Tauri explicitly states that ad-hoc signing does not prevent macOS from requiring the user to open Privacy & Security and allow the app. Apple's Gatekeeper guidance says an unregistered app produces a warning because macOS has not reviewed it and cannot verify whether it was modified; overriding that protection requires a manual action and is associated with a significant malware risk. After the user allows the app, macOS records an exception and later launches can proceed normally.

The practical consequence is that the accepted no-Developer-ID distribution path can be made functional, but it cannot promise a frictionless first launch. Public release instructions must name the Gatekeeper prompt and the manual exception flow.

### GitHub Releases and CI

Tauri documents `tauri-action` for GitHub Actions, including matrix builds for macOS x64 and ARM64 and creation of a GitHub Release. The action can generate the static updater JSON used by GitHub Releases. The workflow needs write permission for release contents.

For an ad-hoc macOS build, Tauri's GitHub pipeline guidance explicitly recommends configuring the ad-hoc signing identity to reduce the chance that Apple Silicon downloads are treated as damaged. This is compatible with the project's accepted no-Developer-ID direction, subject to the manual Gatekeeper exception above.

## Security constraints for Nibble

- Treat every WebView as capable of receiving untrusted content. Sanitize display content and validate IPC payloads.
- Grant clipboard, window creation, global shortcut, SQL write, and updater capabilities only to the window or core component that needs them.
- Do not expose arbitrary SQL execution to React. Prefer typed core commands or a repository boundary.
- Do not store the updater private key in the repository, frontend bundle, or public GitHub artifact.
- Avoid clipboard feedback loops when Nibble writes an item back to the system clipboard.
- Handle macOS pasteboard denial and later revocation as normal product states, not exceptional crashes.
- Treat ad-hoc-signed distribution as a documented trust trade-off; checksums and updater signatures improve integrity but do not replace Apple notarization.

## Downstream decisions affected

- **Product contract:** define what “clipboard history” means for text, rich content, images, multiple pasteboard items, sensitive content, and denied permission.
- **Interaction lifecycle:** define permission onboarding, tray residency, close/quit behavior, shortcut failure, and existing-instance behavior.
- **Data/privacy policy:** define retention, deduplication, sensitive-item exclusion, export, deletion, and whether Nibble-originated writes are re-captured.
- **Application architecture:** decide the core-owned repository/IPC boundary and how a native macOS pasteboard monitor is isolated and tested.
- **macOS release/quality:** decide ad-hoc signing, Gatekeeper copy, minimum macOS, updater key custody, updater manifest, rollback, and diagnostic consent.
- **Windows migration:** repeat shortcut, clipboard access, updater, tray, installer, and WebView differences on Windows before making a compatibility promise.

## Sources

Official Tauri documentation:

- [Process Model](https://v2.tauri.app/concept/process-model/)
- [Capabilities for Different Windows and Platforms](https://v2.tauri.app/learn/security/capabilities-for-windows-and-platforms/)
- [WebviewWindow API](https://v2.tauri.app/reference/javascript/api/namespacewebviewwindow/)
- [Single Instance](https://v2.tauri.app/plugin/single-instance/)
- [Global Shortcut](https://v2.tauri.app/plugin/global-shortcut/)
- [Clipboard](https://v2.tauri.app/plugin/clipboard/)
- [System Tray](https://v2.tauri.app/learn/system-tray/)
- [SQL](https://v2.tauri.app/plugin/sql/)
- [Updater](https://v2.tauri.app/plugin/updater/)
- [macOS Application Bundle](https://v2.tauri.app/distribute/macos-application-bundle/)
- [macOS Code Signing](https://v2.tauri.app/distribute/sign/macos/)
- [GitHub release pipeline](https://v2.tauri.app/distribute/pipelines/github/)
- [Window customization](https://v2.tauri.app/learn/window-customization/)

Official Tauri source, pinned to `plugins-workspace` `v2` commit `92d6b3dd708056640b7fef16537e21542b4d2fdb`:

- [Clipboard JavaScript API](https://github.com/tauri-apps/plugins-workspace/blob/92d6b3dd708056640b7fef16537e21542b4d2fdb/plugins/clipboard-manager/guest-js/index.ts)
- [Clipboard desktop implementation](https://github.com/tauri-apps/plugins-workspace/blob/92d6b3dd708056640b7fef16537e21542b4d2fdb/plugins/clipboard-manager/src/desktop.rs)

Official Apple documentation:

- [NSPasteboard](https://developer.apple.com/documentation/appkit/nspasteboard)
- [NSPasteboard.changeCount](https://developer.apple.com/documentation/appkit/nspasteboard/changecount)
- [NSPasteboard.AccessBehavior](https://developer.apple.com/documentation/appkit/nspasteboard/accessbehavior-swift.enum)
- [Notarizing macOS software before distribution](https://developer.apple.com/documentation/security/notarizing-macos-software-before-distribution)
- [Open a Mac app from an unknown developer](https://support.apple.com/guide/mac-help/open-a-mac-app-from-an-unknown-developer-mh40616/mac)
