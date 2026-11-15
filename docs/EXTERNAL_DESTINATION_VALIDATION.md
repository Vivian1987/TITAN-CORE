# External destination installer and TITAN Control — 0.6.0-beta.1

## Behavior

Installer defaults to internal storage, preserving the existing copy workflow. The destination list includes only currently enumerated internal/USB/extended mount points. A selected external mount gets TITAN-CORE/Games/<TITLE_ID>/<TITLE_ID>.pkg and an owned install.json receipt. These receipt-proven per-title folders are added to native scan roots on every scan, including when custom scan paths are configured. Reconnecting a recognized mounted drive makes its owned folders discoverable again. There is no automatic fallback to internal storage if the selected external drive is unavailable.

On the same selected external filesystem, preparation renames the original PKG into its managed folder without a second complete PKG or hard link. On another drive, installation copies and verifies all bytes and preserves the download original. A pre-existing different library source/title or destination directory is refused. Source admission remains limited to supported external debug PKGs. Existing /user/app packages and installed-path restoration are unchanged.

Destination roots are checked against connected drive enumeration; symlinked folder chains are rejected before creation. File identity, size/mtime and destination directory identity are checked before commit. Transfers check pause/cancel and destination identity during copy/verification. New owned transactions are cleaned on ordinary failure only while their destination still matches. Same-drive moves preserve package bytes and record original permission mode; owner-only sources gain read bits for the game runtime. Read/traverse permissions are set only where required, avoiding unconditional POSIX chmod on exFAT. Atomic rename failure leaves the original source in place; no full-copy fallback is attempted for a same-drive move.

External installs still need internal cache space: a 256 MiB minimum reserve is checked, but actual cache size depends on the game/layout and may be much larger. Cache preparation can still fail if space is insufficient. The drive must remain connected during installation and play. PS5 System storage accounting for external games is not changed or guaranteed to show the full PKG. TITAN reports source and cache sizes separately through the existing library. No drive formatting or existing game migration occurs.

Settings has a TITAN Control hero, section navigation, grouped storage/log/compatibility cards, separate appearance/controller panels and the same setting IDs/values. The stale homebrew-import settings description is corrected to describe in-place system PKGs. New strings have English/French/Arabic/Simplified Chinese/German translations; other console locales retain the documented technical English fallback.

## Validation

- Final unified PS5 ELF build passed both stages without compiler warnings; pinned vendor hashes passed.
- Installer host suite: 13 tests passed, including same-drive inode preservation, cross-drive byte comparison/source retention, cancellation, internal cache reserve, symlink/traversal rejection, destination collision, missing destination, running game and existing internal title refusal, original permission receipt and owned-folder discovery/cleanup. Existing internal copy tests remain passing.
- Other host behavior suite: 64 tests passed. Total: 77 passing tests.
- Bundled JavaScript syntax passed; localization placeholder validation passed (409 entries, 31 choices, existing fallback policy).
- Browser CUA used an isolated synthetic localhost fixture: destination choice retained across browsing; same-drive move and other-drive copy confirmations; submitted JSON source/destination; French Settings save preserved all fields and submitted scan_interval_seconds=20; Arabic RTL and 1280/800 widths had no horizontal overflow or duplicate IDs. No captured ReferenceError/TypeError/SyntaxError. Viewport override was reset. Screenshots are explicitly synthetic previews.
- This version has NOT been tested for actual USB/exFAT/extended-drive gameplay on PS5. Host tests use Linux synthetic files and a stub package reader, not real FPKGs. Compilation and browser fixtures are not gameplay proof.

## Limits

No power-loss transaction resume. A reserved folder left by interruption requires inspection before retrying. The SDK has no atomic no-replace rename primitive; do not concurrently modify the same source/target title outside TITAN during installation. Destination guards do not claim to handle every forced-unmount/device-replacement race. Source copies can take substantial time because full verification is intentional. The minimum internal reserve is not an estimate of the final cache size. Read-only or unsupported external filesystems fail with a reported error.

No PS5 payload was launched or installed by the agent, and no GitHub release or source publication was performed for 0.6.0-beta.1.
