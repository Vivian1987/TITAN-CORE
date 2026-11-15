# Restore original installed paths — 0.5.7-beta.1

## Observed on the console

A read-only status query confirmed version 0.5.6-beta.1, healthy bundled services and native package sources still under the managed homebrew folder. Repeated restoration logs showed `Operation not supported (os error 45)` for the hard-link attempt. One title also reported a missing original installed directory. No console files were modified during diagnosis. A game was running, and restoration must wait until games are closed.

## Changes

- Replace creation of new hard links with an exclusively created, journaled destination reservation and rename. Validate the reservation inode and contents immediately before replacing this owned placeholder. Retry a proven interrupted reservation; refuse unrelated existing files.
- Retry cross-mount/unsupported rename through the verified /user/data alias, retaining the source if no supported same-filesystem rename succeeds. No full package is copied.
- Validate the package title and receipt size. For a missing original title directory, stage only matching cached param/icon metadata and register it before returning its package. Existing title directories are preserved.
- Keep support for recovering an interrupted hard link from the previous build, without creating any new hard link.
- Suppress repeated identical per-title restoration errors so useful logs remain visible.

The SDK exposes ordinary rename/renameat, without an atomic no-replace operation. A destination reservation plus checks protects pre-existing files, but cannot guarantee exclusion against an independent concurrent installer replacing the same pathname in the final check/rename window. Close games and do not install, update or uninstall the same titles during recovery. Unexpected targets stop recovery instead of being cleaned up. Unknown/unrecognized sources remain untouched.

## Validation

All 64 host regression tests passed, including reservation retry, conflicting destinations, missing/mismatched metadata, single-inode recovery, source-title rejection, cross-filesystem no-copy failure and running-game deferral. Existing installer code was not changed; its prior six tests passed in 0.5.6. No new dependency. Unified build evidence accompanies the delivery.

This replacement has not been loaded or tested on the PS5 by the agent. The earlier hard-link method is confirmed unsuitable on the observed console; successful rename restoration, original-path launch and full System storage display must be verified after loading this trial. The local private status snapshot is diagnostic evidence and is excluded from source/release archives.
