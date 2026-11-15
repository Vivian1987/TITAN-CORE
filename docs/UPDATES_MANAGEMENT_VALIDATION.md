# Updates and installed-game management — 0.5.8-beta.1

## Implementation

- Header update status is grouped with the language selector, including RTL. A bounded, credential-free browser request reads the official GitHub release listing. Semantic versions, published status and uploaded official-repository ELF assets determine availability. Drafts, missing payloads, foreign URLs and malformed versions are rejected. Checks time out, deduplicate concurrent requests and ignore obsolete results. Automatic checks are limited to one per 30 minutes per page session after identifying the console version; retry is user initiated. Network errors never masquerade as an up-to-date result. No automatic payload download/install.
- An installed game's Manage button expands a local TITAN panel instead of opening another service. Rescan, native cache rebuild confirmation and Diagnostics remain inside the application. Existing package path and native cache details are preserved. Deletion remains under PS5 System Settings; image-only tools retain their existing route. Pending preparation text describes in-place work. Successful ready-path checks clear stale error text; actual failures remain visible.
- Five primary language catalogs cover the new labels; the existing additional-language English fallback policy remains. No production dependency added. Original-path package/recovery logic is unchanged.

## Validation

- Fourteen Node policy tests passed for numerical semantic version ordering, beta/stable ordering, malformed input, draft/asset filtering, download origin/tag validation, unknown versions, no newer releases, failures, rate limits, timeouts, deduplication and stale response suppression.
- All 64 existing Rust host tests passed. The final unified PS5 build completed both stages with no reported warnings and at most two compiler jobs. Locale generation/placeholder checks and extracted UI JavaScript syntax passed.
- A local synthetic preview was exercised through the browser: update available, no newer release and network failure; English/French/Arabic labels; RTL language/update adjacency with no desktop overflow; installed-management expansion without a new tab; rescan acknowledgement, rebuild confirmation and Diagnostics navigation. Fixture requests used TITAN's own rescan/rebuild routes. No script ReferenceError/TypeError/SyntaxError was observed. Screenshots are synthetic, including the fictional 0.5.9 update shown for testing.
- The selector accepted the real public v0.5.4-beta.1 ELF and correctly found it older than this local 0.5.8 build. No ELF was downloaded by the check. No GitHub publication was performed.

No PS5 UI interaction, new hardware launch/performance test or remote console mutation was performed for this build. The previous 0.5.7 original-path recovery has owner-reported hardware confirmation, not a universal compatibility guarantee. Browser internet/TLS access to GitHub remains required for update checks. Readiness and System storage behavior must remain verified on the console; compilation or synthetic previews do not establish hardware results.

## API references

- https://docs.github.com/en/rest/releases/releases
- https://docs.github.com/en/rest/using-the-rest-api/using-cors-and-jsonp-to-make-cross-origin-requests
