# TITAN CORE package naming

0.5.9-beta.1 renames the root Cargo package to titan-core and the reader crate/directory to titan-core-pkg. Rust imports, Cargo.lock, release profile, build scripts, self-copy environment variable, container image and CI output path use the new names. Notification bundle identity is titan-core.

Validation: unified PS5 ELF built successfully in both stages; 64 host Rust tests and 14 Node update-policy tests passed. No new console gameplay test was performed for this naming change. The previous 0.5.7 hardware confirmation remains separately attributed.

Upstream authorship, vendor archives and licenses are retained. Kraken is also the compression format name; its decoder terminology is not project branding. Existing AMPR protocol paths (kraken.image, /app0/kraken and index names) and legacy migration reads remain compatible to avoid breaking games or existing installs. New project/package/build names use TITAN CORE.
