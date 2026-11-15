# Internal system destination correction — 0.6.1-beta.1

The 0.6.0 Installer still used /data/homebrew/titan-core/installed for its Internal choice. This was inconsistent with the owner’s intended /user/app/<TITLE_ID>/app.pkg workflow. This correction changes the actual internal copy destination, receipt discovery, metadata preparation and UI path, not merely a display label. External destinations remain <mount>/TITAN-CORE/Games/<TITLE_ID>/<TITLE_ID>.pkg. The external download source is retained for internal copies.

## Implementation

- Internal package commit is /user/app/TID/app.pkg; the selected drive key remains /data for SSD free-space reporting and request compatibility. The /user base identity and directory chain are independently guarded.
- A newly reserved system title folder gets matching sce_sys/param.json and icon0.png from the copied PKG before commit. The existing native cache/registration code then registers the title. System metadata staging uses bounded known paths and create-new files; normal failure removes only transaction-owned file identities.
- A pre-existing title directory can be reused only when its metadata matches the title, its native cache receipt proves this exact external source, no installed app.pkg/partial/receipt exists and no loose eboot is present. Existing metadata and unrelated files are retained. Other pre-existing titles/packages are refused.
- Installed-path native sources pass the existing per-title resident-service overlay policy gate before Ready status. If inventory is not yet available, a rescan is requested and the normal scan retries; no false-ready launch is reported. Original system-path source access receipts are retained.
- The 0.6.0 managed homebrew root remains readable through owned receipts; no automatic migration of existing legacy homebrew packages is added. System metadata is not removed by empty-receipt cleanup.
- French/Arabic and other core labels show the real internal path. Changing interface language refreshes dynamic destination options.

## Evidence

Final unified PS5 ELF built both stages without compiler warnings; pinned vendor hashes passed. Installer host tests: 19 passed; other existing host tests: 64 passed (83 total). Added tests verify exact /user/app/PPSA12345/app.pkg placement, byte equality and external source retention, metadata presence, metadata-stage failure cleanup, reuse of a proven external tile without overwriting its metadata/owner file, refusal of an untrusted title, legacy receipt discovery and preservation of existing app.pkg/partial data. These tests use Linux fixtures and a stub PKG metadata reader.

Bundled JavaScript syntax and localization placeholders passed. Synthetic browser check verified the French internal path, external path, Arabic-to-French destination labels, literal <TITLE_ID> display with no unintended HTML element, and no horizontal overflow or ReferenceError/TypeError/SyntaxError. Screenshot is a synthetic preview, not the user’s console.

No hardware install/launch or System storage accounting test was performed for 0.6.1 by the agent. The earlier user confirmation of classic installed-path operation applies to 0.5.7, not this new installer correction. Cache still consumes internal storage; actual System size/accounting and resident-service inventory timing need confirmation on PS5. No manual app.db size edits, new dependencies, drive formatting, overwrite of another installed package or full-PKG backup is introduced. Power-loss resume and all external-device race guarantees remain outside the implementation. No GitHub publication was performed for this version.
