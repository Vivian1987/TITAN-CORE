# External PKG installation-only discovery — 0.6.4-beta.1

## Behavior

Raw PKGs on connected USB/extended drives are installation archives. Native automatic discovery excludes them before package metadata parsing, title deduplication, cache building and registration, including when a custom scan root points at the archive folder. They remain available in Installer browse/start. Once installed, the native scanner accepts only the exact package in a receipt-validated TITAN installer root or a recognized system-installed package path. TITAN external installations use <drive>/TITAN-CORE/Games/<TITLE_ID>/<TITLE_ID>.pkg. A similarly named folder without a valid installation receipt is insufficient.

Excluding the archive before title selection prevents it from suppressing the matching /user/app installed candidate or creating a second playable archive card. The existing installed-candidate and library merge behavior then continues unchanged. Internal PKG discovery, internal installation paths, verified copying/original preservation, extracted external folders, compressed folders and image handling are unchanged. This is a native discovery rule, not drive formatting or file deletion.

## Validation

- Seven targeted tests execute production scan.rs with synthetic storage/cache/validated-root providers: raw archive not parsed/playable; archive cannot shadow internal installed source even when listed first; validated external installation selected without archive duplicates; directory name alone/extra files cannot bypass authorization; external system-installed app.pkg accepted; external Plain/Zstd/Lz4 folders preserved; invalid/custom-homebrew-root downloads excluded before parser errors.
- Discovery suite: 18 passed (seven targeted plus 11 existing production library tests).
- Production installer/I/O suite: 20 passed, including same-drive archive inode/content/permissions retained, distinct installed copy, receipt discovery, internal destination, collisions, cancel and insufficient-space guards.
- Existing host suite: 64 passed, including installed selection, images, ownership, permission receipts and relocation. Total executed: 102 tests, zero failures. Synthetic fixtures live only inside Docker /tmp and tmpfs /user, /data, /mnt; no console game files touched.
- Full two-stage optimized unified ELF build passed using existing Rust1.99/PS5SDK0.43 with at most two compiler jobs. Pinned runtime/service/source hashes passed; no compiler warning/error in build log.
- Byte comparison against 0.6.3 found only src/scan.rs changed among production src files. Installer, web UI, cache/register/launch engine and PLAY NO LIMIT runtime remain unchanged. Package version is 0.6.4-beta.1.
- Delivery checks verify ELF magic/version, exact bundled UI and runtime, ZIP CRC and SHA-256 checksums.

## Limits

No PS5 installation/gameplay or rendered browser test performed for this change. Close games before replacing the payload, then let a fresh scan complete. The rule does not forcibly unmount active games, delete existing caches/packages, or remove stale game tiles left by earlier versions. An old tile is not proof that an archive has been newly mounted. Existing installed-source preparation settings/readiness requirements remain. Real external/system installation and launch require hardware acceptance. No new GitHub release was published.
