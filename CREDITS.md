# Development, thanks and licenses

## TITAN CORE — devchmod

TITAN CORE is maintained by **[devchmod](https://github.com/devchmod)**, following six months of independent research. Project work includes the TITAN CORE interface and branding, multilingual navigation, USB FPKG installer, installed-package preparation and legacy import recovery, diagnostics and reporting, and integration of the resident services.

**Installed-PKG launch handling is a TITAN CORE addition:** the earlier workflow brought supported Debug Settings/Overdrive installs into a playable homebrew library without retaining another complete PKG, with successful console use reported by the maintainer on firmware 13.60. Current versions keep eligible packages at their original system path and recover receipt-proven earlier imports, while retaining the native cache and launch engine. The maintainer confirmed original-path launch and full System storage sizes on the development console with 0.5.7-beta.1; see docs/USER_HARDWARE_CONFIRMATION.md. Native cache uses additional space.

Development also addresses package discovery and duplicate library filters. Build checks and documented tests are included in this repository; see [validation](docs/VALIDATION.md) for their scope.

## Thanks to the upstream developers

Thank you to **quer3q**, **EchoStretch**, **drakmor**, the **PS5 payload SDK contributors**, and the wider homebrew community for sharing their code, tools and research. Their contributions support the components listed below. TITAN CORE development and integration are described above; each upstream project retains credit for its own code.

## Included components

| Component | Upstream | Role |
|---|---|---|
| Kraken Mount | [quer3q/kraken-mount](https://github.com/quer3q/kraken-mount) | Native FPKG/folder mounting, cache, registration and launch code used and adapted in TITAN CORE |
| ampr-emu-ng 0.4.0 | [quer3q/ampr-emu-ng](https://github.com/quer3q/ampr-emu-ng/tree/v0.4.0) | Packed-game runtime; custom launch-notice display label `chmod v0.1` |
| kstuff-lite 1.11 | [EchoStretch/kstuff-lite](https://github.com/EchoStretch/kstuff-lite/releases/tag/v1.11) | Pinned resident payload, embedded unchanged |
| ShadowMountPlus 1.7beta10 | [drakmor/ShadowMountPlus](https://github.com/drakmor/ShadowMountPlus/releases/tag/1.7beta10) | Pinned image service, embedded unchanged |
| PS5 Dump Forge | [quer3q/ps5-dump-forge](https://github.com/quer3q/ps5-dump-forge) | Toolchain/workflow references and attributed sparse-format reader reference |
| ps5-payload-sdk | [ps5-payload-dev/sdk](https://github.com/ps5-payload-dev/sdk) | Payload build toolchain and PS5 libraries |

## Source and notices

- Root license: GPL-3.0-or-later; see [LICENSE](LICENSE).
- Existing dependency notices: [THIRD_PARTY_NOTICES](THIRD_PARTY_NOTICES).
- Pinned service versions, original source snapshots, component licenses and hashes: [vendor/unified](vendor/unified).
- Original AMPR 0.4.0 binary/source and custom display-only runtime source: [vendor/ampr-emu-ng](vendor/ampr-emu-ng), including [branding explanation](vendor/ampr-emu-ng/TITAN_BRANDING.md).
- Sparse-format port attribution and exact source reference: [vendor/format-reference](vendor/format-reference).
- TITAN CORE artwork provenance: [docs/branding/PROVENANCE.md](docs/branding/PROVENANCE.md).

The custom AMPR source changes only the launch-notice text. Compatible filenames, exported runtime identity and underlying engine version remain upstream 0.4.0. Upstream credits in source archives are preserved. Additional third-party code retains its own license terms; consult the included notices/source for each component.
