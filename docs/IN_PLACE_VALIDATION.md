# Original-path installed PKGs — 0.5.6-beta.1 trial

## Behavior

Installed debug PKGs are parsed and prepared at `/user/app/<TITLE_ID>/app.pkg` (or their supported existing installed external path), rather than renamed into homebrew. A per-source permission receipt records original file/directory modes before adding the read/traverse bits required by the existing read-only per-title sandbox mount. The cache references the original path; package bytes and inode remain unchanged. Metadata backups and the 0.5.5 size-limit fix remain enabled. `auto_import_installed` retains its existing key/value, but now means in-place preparation. The primary five language catalogs use the new label. Native installed entries show cache size separately and request the native size provider.

Older internal imports are returned only with a valid TITAN CORE receipt, exact original path, title directory, expected byte length and PKG parser title match. Recovery creates a no-clobber hard link to the same inode, removes the old managed name, and records restoration. The validated `/user/data` alias is attempted when `/data` crosses a nullfs mount. A different existing `app.pkg`, changed source, missing title directory, parser error or unavailable same-inode move prevents restoration without falling back to copying. Older/unrecognized/manual sources are preserved. Running-game checks defer work. No generic /user/app scanning root, direct database size write, format conversion or classic system installer was introduced.

## Verified locally

- 60 host tests pass, including preserving the original source inode and bytes, read permissions and permission receipts, rejecting invalid paths/symlinks, single-inode legacy recovery, destination collisions, interrupted link recovery, parser rejection, running-game deferral and cross-filesystem no-copy failure.
- Six installer regression tests pass with a writable synthetic /data volume in Docker. The first installer run used a container without writable /data and failed the success fixture; rerunning with the correct isolated writable volume passed, with no installer code change.
- Localization generation and placeholder checks pass for 31 locale choices; extracted UI JavaScript passes node --check. The source change is small and no new browser/visual test is claimed.
- The existing Rust 1.99 / PS5 SDK Docker toolchain builds both unified ELF stages successfully with at most two compiler jobs and no reported warnings.

## Still required on PS5

Confirm an installed package remains at its original path after preparation, verify native launch (including sandbox mount access), open System > Storage to check full package accounting, and check that receipt-proven recovered packages no longer retain the old managed filename. UI size may include different units or additional data. Exact System accounting, nullfs alias recovery and successful original-path gameplay are not established by compilation or host tests. No console mutation, release publication, hardware benchmark or database edit was performed. Metadata/cache still consumes additional storage. Existing unrecognized sources and older pending metadata snapshots remain intact.
