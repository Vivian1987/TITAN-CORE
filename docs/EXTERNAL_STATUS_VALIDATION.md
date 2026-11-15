# External installed status and size — 0.6.5-beta.1

## Change

GET /api/state now has external_installed for a native PKG whose exact path belongs to a validated installer receipt on a connected external destination. installer::external_installs derives these paths from the same receipt roots used by discovery, once per API response. Directory naming alone, an archive original, an extra package filename, a missing/invalid receipt or an internal system package cannot receive this flag. Existing installed_pkg/native_installed flags retain system-path semantics; external native packages keep their existing confirmed-delete/rebuild controls and do not invoke system-installed management/image tools.

Library cards now show PKG · External installed. The details show Installed on external drive, actual drive/path, installed package size from the scanned file length, and internal cache size separately. English/French/Arabic/Chinese/German text is included with existing additional-locale fallback behavior. UI does not label archived installer originals as installed. The 0.6.4 external archive exclusion remains unchanged.

## PS5 System storage investigation

The inspected TITAN registration code stages metadata at /user/app/<TID> and calls sceAppInstUtilAppInstallTitleDir(tid, /user/app/, NULL). Its function signature has title/path/reserved arguments and no explicit external-volume accounting argument. The pinned ShadowMountPlus source snapshot also registers at APP_BASE. External TITAN package data stays in <drive>/TITAN-CORE/Games while the cache/registration metadata stays internal. These code observations explain why the current integration does not establish full external System accounting; they are not proof that every possible future integration is impossible.

Sony documents official USB extended storage as PS5 game storage requiring transfer back to console/M.2 for normal PS5 play: https://www.playstation.com/en-us/support/hardware/ps5-extended-storage/ and https://www.playstation.com/en-us/support/hardware/ps5-manage-storage-space/ (checked 2026-11-15). That official workflow differs from this homebrew mount pipeline. No verified integration was found in the inspected code to make the homebrew external package count as a full native USB installation in System storage. This build does not alter app.db sizes, registration paths or mount behavior. Full external System accounting remains unresolved and is not promised.

## Evidence

- Installer/I/O suite:22 passed, including exact owned-path classification, missing receipt, archive/extra filename rejection and internal-vs-external distinction. Final optimized snapshot implementation reran these22 successfully.
- Discovery suite:18 passed; existing host suite:64 passed. 104 tests across the complete suites, zero failures. Fixtures are isolated Docker synthetic files, no real console files changed.
- Localization regeneration:418 entries/placeholders validated; extracted bundled JavaScript passed node --check.
- Synthetic IAB browser: Arabic/French external detail labels, 200.00GB package and128MB internal cache; English/French/Arabic library badges; internal installed controls/size retained; RTL viewport1280 without horizontal document overflow; no captured browser error logs. Screenshot is synthetic UI, not console gameplay.
- Both final optimized unified ELF stages/pinned hashes passed using existing Rust1.99/PS5SDK0.43 with at most two compiler jobs. No compiler warnings/errors. Archive/ELF checks verify exact UI/runtime bytes, package version, source ZIP CRC and SHA256SUMS.
- Production source changes are limited to installer classification, web status serialization and UI display. Scanner/registration/cache/launch/runtime are byte-identical to0.6.4. No new production dependencies.

No new PS5 installation/gameplay/System accounting test performed by the agent. User-provided0.6.4 images confirm external managed-path use and internal System size on their console, but do not validate0.6.5. Close games before replacing ELF and allow a fresh status/scan. No new GitHub source/release publication performed.
