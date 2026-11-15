# PLAY NO LIMIT and game-folder wording — 0.6.2-beta.1

The runtime launch notice body is now exactly PLAY NO LIMIT for pkg/zstd/lz4/plain modes. It replaces the previous chmod v0.1 (mode) running message. The PS5 notification title itself is not changed. The existing chmod v0.1 component label remains in Settings and upstream engine identity stays ampr-emu-ng 0.4.0 with preserved licenses/credits.

Only load_notice and its adjacent description in the custom runtime exports.rs changed compared with the prior custom source archive. Native TITAN engine/installer files were byte-compared unchanged against 0.6.1. The original upstream binary/source and previous custom source remain included. The current complete modified runtime source is vendor/ampr-emu-ng/play-no-limit-source.zip. It includes the upstream ps5/target JSON omitted from the older custom source ZIP.

Settings now labels the field Game folders / Dossiers des jeux / مجلدات الألعاب. Help explains automatic discovery when empty, /user/app/<TITLE_ID>/app.pkg for installed internal PKGs and <drive>/TITAN-CORE/Games for external installs. The field does not move or install games. The automatic-discovery placeholder is translated. Existing user scan-path values and scan behavior were not changed.

## Validation

- Exact modified notice function extracted from production source: expected text checked across four modes plus empty mode.
- Upstream runtime workspace host test suite passed; 166 total tests including the notice harness, no failures.
- Runtime make -j2 verify passed with 137 exports, 141 definitions, 103 imports and FSELF 0x02000001. Correct source text is present in the built static library; the old notice prefix is absent. The signed/self-format SPRX was checked by the upstream verifier.
- Final unified TITAN ELF built both stages without warnings and all pinned runtime/service/source hashes passed.
- Bundled UI JavaScript syntax and localization placeholders passed (412 entries, documented existing language fallback policy).
- Synthetic local browser checks: French and Arabic labels/help/placeholders, literal path placeholders, no horizontal overflow or unexpected HTML elements, no captured ReferenceError/TypeError/SyntaxError. The screenshot contains synthetic data only.

No new PS5 gameplay or actual on-console notification test was performed by the agent. Close the running game before loading this ELF; the notice appears on the next game start after the bundled runtime/fakelib refresh. A user-supplied unrecognized custom runtime is intentionally preserved by the existing updater, so this notice only applies when the bundled runtime is being used. The previous internal destination and external installer changes still require their documented hardware acceptance checks. No GitHub source/release publication was performed for this version.
