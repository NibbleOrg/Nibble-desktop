# Nibble local patch

Source: crates.io global-hotkey 0.8.0, already selected in Cargo.lock before this patch.
Upstream: https://github.com/tauri-apps/global-hotkey
Registry archive SHA-256: 8c386b0a4a70cb2d39fffd74480f985b6f0bfbcb934b6a6b6b7e630e448f242e
License: Apache-2.0 OR MIT; original license files and notices retained.

Only upstream source change: in `src/platform_impl/macos/mod.rs`, pass `1`
(`kEventHotKeyExclusive`, as declared by Apple's CarbonEvents.h) instead of `0`
to RegisterEventHotKey. Everything else is an unmodified copy of the locked
registry source (src, examples, manifests, README, changelog, licenses).
Registry metadata and the dependency's own lockfile are not copied.

Nibble's maintainer requested exclusive registration in the issue 33 implementation
conversation on 2026-11-17. The upstream API does not expose the registration flag.
The local Cargo patch keeps Tauri's existing event handler and transaction path;
there is no second OS registration manager. This applies to native-probe builds too.
Media keys use the upstream shared event-tap path, so Nibble rejects those keys
in its settings validator instead of claiming exclusive registration.

Verification: `cargo build --manifest-path src-tauri/Cargo.toml --locked --example
shortcut_exclusivity`, then run `src-tauri/target/debug/examples/shortcut_exclusivity`
in a macOS GUI session. The two-process check fails on the unpatched dependency and
passes on this copy, exercising the real application Service with native registration.

Update policy: compare a proposed upstream release against this copy, verify whether
it exposes equivalent exclusive registration, rerun the native check before removing
this patch, and review license/supply-chain changes. Do not edit cached registry files.
Rollback: remove [patch.crates-io], restore the original registry source/checksum in
Cargo.lock, and remove this vendored directory. That restores shared registration and
no longer satisfies the exclusive requirement; existing JSON format is unchanged.
Full dependency vulnerability audit: not verified.
