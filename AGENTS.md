# TITAN CORE contributor instructions

This is the Rust PS5 payload and bundled web interface derived from Kraken Mount. Read DESIGN.md and CREDITS.md before changes. Preserve GPL/upstream authorship and compatible engine identifiers.

- Work on the requested feature without changing unrelated game/cache/launch logic.
- Use existing Rust1.99/PS5SDK0.43 toolchain; compiler parallelism at most2jobs.
- Build with TITAN_UNIFIED_STACK=1 ./ps5/build.sh; inspect actual warnings/errors and both stages.
- Do not equate build or synthetic browser checks with console gameplay. Record actual validation and limits.
- Never force unmounts or change a running game's mounts. Native work must pause while a game runs.
- Preserve files written by games to writable caches during rebuild. Source Delete is separately confirmed.
- Keep installed PKGs in their original system path, as requested by the owner. Restore only proven legacy imports without overwriting another package; never retain a second full-PKG copy.
- No new production dependency without demonstrated need and matching license notices.
- Web UI is local/bundled, multilingual, RTL-aware, keyboard/controller accessible and PS5-browser compatible.
- Edit review/monitor-client.js then run review/embed-monitor.py; edit update-policy.js/updates-client.js/installed-management.js then run review/embed-features.py; run review/embed-localization.py and node --check on the extracted script.
- All game measurements/status claims must come from available console data. Unsupported sensors stay unknown.
- Never commit console snapshots, personal logs, credentials, build targets or caches.

- Installer destinations must be selected from connected mounts. Preserve the internal default. All installer destinations use verified copy and preserve the download original, including the same external drive. Reserve space for a full copy plus working headroom. Never move installed /user/app games through this installer.

- External raw PKGs are installer archives, never automatic native scan/mount sources. Only receipt-proven external TITAN packages or recognized system installed paths may enter native PKG processing. Preserve external folders/images and internal discovery.
