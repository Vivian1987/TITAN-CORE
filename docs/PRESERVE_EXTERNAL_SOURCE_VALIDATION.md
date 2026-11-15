# External PKG archive preservation — 0.6.3-beta.1

The installer always copies a selected supported external debug PKG into the selected installation destination, including when it is the same external drive. It no longer renames the archive source into the game folder. External games use <drive>/TITAN-CORE/Games/<TITLE_ID>/<TITLE_ID>.pkg; Internal still uses /user/app/<TITLE_ID>/app.pkg. Existing installed /user/app sources remain prepared in place. This change does not restore download files already moved by earlier versions.

The copy is compared byte-for-byte and its title reparsed before the staging file is renamed into the final installed package. The original source is never renamed, deleted or chmodded by this installation transaction. Cancellation, running-game, mount identity, symlink, collision, source mutation and owned-file cleanup guards remain. The destination must have the full package size plus 256 MiB reserve available, even on the same drive; internal cache space is still needed. Keeping an installer archive and a separate installed copy consumes space for both. Earlier historical validation documents describe their respective older versions.

## Evidence

- 20 production installer/I/O tests passed in Docker with isolated /user and /data tmpfs and synthetic external fixtures. Same-drive success preserves source inode/bytes, produces a distinct installed inode with identical bytes, and registers the managed game root. Source mode 0600 remains unchanged while installed copy is 0644. Insufficient same-drive space rejects before creating a transaction folder and keeps the original. Cross-drive, internal path/metadata, collision, cancellation, running-game and path guards also pass.
- 64 existing host tests passed. No failures across 84 tests.
- Both optimized unified ELF stages built successfully with the existing Rust 1.99/SDK 0.43 Docker toolchain, at most two compiler jobs. Pinned bundled payload/runtime/source hashes passed; no compiler warnings/errors in the build log.
- Bundled JavaScript syntax passed node --check. Executing the actual confirmation function in a mocked UI confirmed same-drive, cross-drive and Internal choices all describe copying/keeping the original and pass the exact source and destination. This is a function check, not a rendered browser or hardware test.
- Localization regeneration validated 413 entries and placeholders, retaining the existing 31-choice/full-vs-partial catalog policy. New archive-preservation note translated in English, French, Arabic, Chinese and German.
- Only src/installer.rs and src/ui.html changed among production src files from 0.6.2. The PLAY NO LIMIT runtime, native scan/cache/launch engine and resident service binaries remain byte-identical.
- Delivery verification checks ELF magic, exact current UI and runtime bytes embedded, source ZIP integrity and SHA-256 sums.

No actual PS5 installation/gameplay test was performed for this change. No real console package files were moved or deleted. Power-loss resume and generic retail/encrypted/update/DLC support are not added. The current build is delivered locally; no new GitHub release was published.
