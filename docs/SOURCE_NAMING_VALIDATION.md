# Source naming cleanup — 0.5.10-beta.1

Renamed the private reader module kraken.rs to layout.rs, the layout type to BlockLayout, private layout fields to layout, index records to CompressedBlock/compression and internal codec constants to descriptive names. Comments and decoder diagnostics use neutral technical terms. Original port references remain in CREDITS.md and licenses/vendor archives are preserved.

This is a naming/diagnostic change: compression tags, flags, limits, index serialization and runtime compatibility filenames are unchanged. Legacy migration reads remain intact. No game data or installed source path is modified by this refactor.

Validation results are recorded after running the two-stage unified build and existing tests. A successful build is not a console playtest.

Existing host tests: 64 passed; reader tests: 3 passed. Non-comment Rust code is equivalent to the prior source after normalizing only renamed identifiers and diagnostic strings (9 affected runtime/reader files). Protocol numeric tags and filenames are retained.

Final unified PS5 build passed both stages without compiler warnings. No new console gameplay test was performed for this naming-only version.
