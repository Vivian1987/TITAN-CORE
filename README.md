> **Local test build 0.5.8-beta.1:** installed-package preparation now keeps the PKG at its original system path. This path has not yet been validated on PS5 hardware. The public download links below still refer to the previous published 0.5.4 release.

<p align="center"><img src="ps5/icon0.png" width="96" alt="TITAN CORE"></p>
<h1 align="center">TITAN CORE</h1>
<p align="center">A PS5 game workspace with a native FPKG engine, external-drive installer and live diagnostics.</p>

## ⬇️ تحميل TITAN CORE / Download TITAN CORE

### [⬇️ تحميل ملف ELF مباشرة — Download ELF directly](https://github.com/devchmod/TITAN-CORE/releases/download/v0.5.4-beta.1/TITAN-CORE-0.5.4-beta.1.elf)

**النسخة / Version: 0.5.4-beta.1** · **الملف / File: `TITAN-CORE-0.5.4-beta.1.elf`**

**بالعربية**

1. اضغط على رابط **تحميل ملف ELF مباشرة** أعلاه لتحميل ملف التشغيل.
2. إذا فتحت [صفحة الإصدار](https://github.com/devchmod/TITAN-CORE/releases/tag/v0.5.4-beta.1)، افتح **Assets** واضغط على **`TITAN-CORE-0.5.4-beta.1.elf`**.
3. الملف المطلوب للتشغيل هو **`.elf`**. ملفات **Source code (zip / tar.gz)** هي للمطورين وليست ملف التشغيل. لا تحتاج إلى فك ضغط ملف ELF.
4. أرسل ملف ELF إلى PS5 باستعمال أداة تحميل الـpayload/elfldr التي تستخدمها، ثم افتح `http://<PS5-IP>:8097` مع تعويض `<PS5-IP>` بعنوان جهازك.

**English**

1. Click **Download ELF directly** above to download the runnable payload.
2. If you open the [release page](https://github.com/devchmod/TITAN-CORE/releases/tag/v0.5.4-beta.1), expand **Assets** and select **`TITAN-CORE-0.5.4-beta.1.elf`**.
3. You need the **`.elf`** file to run TITAN CORE. **Source code (zip / tar.gz)** downloads are for developers. The ELF does not need extraction.
4. Send the ELF to your PS5 using your existing payload/elfldr loader, then open `http://<PS5-IP>:8097`, replacing `<PS5-IP>` with your console's address.

[جميع الإصدارات / All releases](https://github.com/devchmod/TITAN-CORE/releases) · [بصمات التحقق / SHA256SUMS](https://github.com/devchmod/TITAN-CORE/releases/download/v0.5.4-beta.1/SHA256SUMS)


## ⚙️ قبل التشغيل / Before loading TITAN CORE

**TITAN CORE يتضمن kstuff-lite وShadowMountPlus ويبدأ تشغيلهما تلقائياً. / TITAN CORE bundles kstuff-lite and ShadowMountPlus and starts them automatically.**

### بالعربية

- إذا كنت كتستعمل **kstuff أو ShadowMount كـpayloads مستقلّة**، عطّل تشغيلهما التلقائي أو احذف إدخالاتهما من لائحة **autoload / Payload Manager** الخاصة بهاد الإعداد. شغّل **TITAN CORE فقط** عوض تشغيل الحزم الثلاث كل واحدة بوحدها.
- المقصود هو إزالة **التشغيل المنفصل** لهذين المكوّنين؛ لا تحذف ملفات الجيلبريك أو elfldr أو ملفات الألعاب، ولا تحذف المكونات المضمّنة داخل TITAN CORE.
- إذا كانا مشغّلين من قبل، أغلق الألعاب وأعد تشغيل الجهاز لبدء جلسة جديدة، ثم فعّل الجيلبريك وelfldr بالطريقة المعتادة وحمّل ELF ديال TITAN CORE مرة واحدة.
- TITAN CORE لا يتضمن استغلال الجيلبريك نفسه؛ ما زلت تحتاج إلى الجيلبريك وأداة تحميل ELF. يمكن مراجعة حالة الخدمات في Dashboard وDiagnostics.

### English

- If you previously loaded **standalone kstuff or ShadowMount payloads**, disable their automatic loading or remove their entries from your **autoload / Payload Manager** list for this setup. Load **TITAN CORE only**, instead of loading all three separately.
- Remove the **separate launch entries**; you do not need to delete jailbreak, elfldr or game files. Keep TITAN CORE's bundled components intact.
- If standalone services are already running, close games and restart the console for a fresh session. Apply your normal jailbreak, make elfldr available, then load the TITAN CORE ELF once.
- TITAN CORE does not include a jailbreak exploit. You still need your usual jailbreak and ELF loader. Check service status in Dashboard and Diagnostics.

Developed and maintained by **[devchmod](https://github.com/devchmod)** following six months of independent research, TITAN CORE brings game browsing, USB installation and diagnostics into one PS5 workspace. It integrates the native mounting workflow from Kraken Mount, kstuff-lite 1.11 and ShadowMountPlus 1.7beta10, with TITAN CORE interface, installer and integration work. It targets jailbroken PS5 consoles with elfldr. The development console uses firmware 13.60; wider hardware compatibility has not been established. This project does not include a console exploit.

[Releases](https://github.com/devchmod/TITAN-CORE/releases) · [Source](https://github.com/devchmod/TITAN-CORE) · [Credits](CREDITS.md)

## Updates and installed-game management

The header displays update status beside the language selector. It checks up to 100 recent public releases from the official repository, compares semantic versions (including beta versions), ignores drafts/unpublished entries and requires an uploaded ELF with an official repository download URL. Automatic checks run after console identification and at most once per 30 minutes per page session, with a manual retry option. Network/rate-limit failures remain distinct from a successful check that finds no newer release. Downloads remain a user action; no payload is automatically loaded or replaced. Checking requires browser access to GitHub. Current public releases may lag local test builds.

Manage installed game opens TITAN CORE's own panel. It retains package/cache details, queues a library rescan, keeps the existing confirmed cache-rebuild action, and navigates to Diagnostics. Installed-game management no longer opens the ShadowMount web UI. Uninstallation guidance points to PS5 System Settings; no new game-deletion API was introduced. Image-only tools retain their separate existing behavior. Rebuild is disabled while a game is running. A successful ready-path mount/ownership check now clears a stale earlier error.

## Prepare supported installed PKGs in their original system location

With **Prepare installed PKGs in place automatically** enabled, TITAN CORE discovers eligible debug FPKGs under `/user/app/<TITLE_ID>/app.pkg` and builds its native launch cache without moving the package to homebrew. Read access is prepared for the game's read-only sandbox mount; original permissions are recorded first. The package remains a single physical file in the system location.

Receipt-proven internal packages moved by older TITAN CORE builds can be returned to their original installed path. The package title and recorded size must match. Pre-existing different `app.pkg` files are refused. Restoration journals and exclusively reserves the absent destination, verifies that reservation, then uses rename with a verified `/user/data` alias when required; there is no full-copy fallback. Do not install, update or uninstall the same title concurrently: the SDK does not expose an atomic no-replace rename primitive. A missing title directory can be recreated from matching cached registration metadata. Unsupported restoration leaves the old managed source intact. Manually relocated packages without a valid receipt are not moved automatically.

This test build replaces the unsupported hard-link restoration observed on the console in 0.5.6, and preserves the metadata-snapshot fix from 0.5.5. Cache still occupies additional storage. Previous successful console reports cover the older homebrew import workflow; original-path launch and PS5 System storage accounting must be checked on hardware for this build. This is still the native cache/mount engine, not a new classic-system installer or retail-package conversion.

## Features

- Dashboard and game library for supported debug FPKGs, folders and image sources, including exFAT.
- USB/extended-drive browser, internal FPKG copy with progress, byte-for-byte verification and cancellation.
- **Original-path preparation for supported Debug Settings/Overdrive installs:** keep `app.pkg` under `/user/app/<TITLE_ID>/`, with verified recovery of earlier internal imports.
- Reported system/payload CPU and memory, storage, live logs, kernel-log viewer and current alerts.
- Browser-local CPU history, read-only reported health checks, JSON/CSV exports and print/Save PDF reports.
- 31 console-language/regional choices, flag cues, Arabic RTL, themes and controller-friendly navigation.

![TITAN CORE diagnostics](docs/screenshots/monitor.png)
*Interface preview with synthetic data. Unsupported sensors remain unavailable on the actual console.*

## Use

1. Close games before loading a new payload.
2. Send the release ELF through your existing elfldr workflow.
3. Open `http://<PS5-IP>:8097` from the console or a browser on your Mac/PC/phone.
4. Browse Library or open Installer to choose a supported PS5 debug FPKG from a mounted USB/extended drive.

Installer copies one complete PKG internally and keeps the external original. The native engine then builds its cache, registers the game and mounts its content at launch. This is the homebrew/debug-FPKG workflow, not a general retail/encrypted PKG installer. Cache takes extra space and can be substantial for incompatible block layouts. Supported installed packages are prepared at their original system path. Legacy recovery refuses collisions and unavailable same-inode moves, leaving the managed source intact without creating another full package.

Do not disconnect a source drive during installation. Ordinary cancellation/errors clean the transaction's partial copy; power loss may leave a reserved `.part` directory that needs inspection before retrying. Resume and update/DLC merging are not implemented.

## Diagnostics and localization

Metrics are reported by the console, with payload-only fallbacks labeled explicitly. Temperature/GPU/I/O rate are not exposed by this backend and remain unknown. CPU history starts when the page opens; it is not historical console telemetry. Diagnostic checks inspect reported status only and are not hardware stress tests. Reports/history are handled in the browser.

English, French, Arabic, Simplified Chinese and German have full current catalogs. Additional languages currently translate 50 core labels and use English fallback for remaining technical text. See [coverage](Localizations/coverage.json).

## Technology

[![Rust](https://img.shields.io/badge/Rust-native_backend-DEA584?logo=rust)](https://github.com/devchmod/TITAN-CORE/search?l=rust) [![HTML](https://img.shields.io/badge/HTML-bundled_interface-E34F26?logo=html5&logoColor=white)](https://github.com/devchmod/TITAN-CORE/search?l=html) [![C](https://img.shields.io/badge/C-PS5_shims-555555?logo=c)](https://github.com/devchmod/TITAN-CORE/tree/main/ps5/shim) [![Shell](https://img.shields.io/badge/Shell-build_scripts-89E051?logo=gnubash&logoColor=white)](https://github.com/devchmod/TITAN-CORE/tree/main/ps5) [![Assembly](https://img.shields.io/badge/Assembly-startup-6E4C13)](https://github.com/devchmod/TITAN-CORE/tree/main/ps5) [![Docker](https://img.shields.io/badge/Docker-build_environment-2496ED?logo=docker&logoColor=white)](https://github.com/devchmod/TITAN-CORE/blob/main/ps5/Dockerfile) [![Python](https://img.shields.io/badge/Python-tooling-3572A5?logo=python&logoColor=white)](https://github.com/devchmod/TITAN-CORE/tree/main/review)

| Layer | Implementation |
|---|---|
| Console backend | Rust: package discovery, cache, registration, mounting, installer and HTTP service |
| Console integration | C and Assembly: PS5 shims, startup and linking support |
| Browser interface | HTML, CSS and JavaScript: library, settings, localization and diagnostics |
| Build and tooling | Shell, Dockerfile and Python: payload builds, UI embedding and review tools |

GitHub's language percentages measure classified source bytes; they do not measure engineering effort or ownership. See the repository's Languages panel for its current calculation.

## Build

The source uses Rust 1.99.0 and PS5 payload SDK 0.43 in the Docker build. Keep compiler parallelism at two jobs.

```sh
TITAN_UNIFIED_STACK=1 bash ps5/build.sh
```

The build verifies bundled runtime/service hashes and builds both ELF stages. The packages are `titan-core` and `crates/titan-core-pkg`; the Docker toolchain image is `titan-core-ps5` and the output is `target/ps5/titan-core.elf`. Upstream runtime protocol filenames remain compatible. See [architecture](DESIGN.md), [validation](docs/VALIDATION.md) and [credits](CREDITS.md).

## License and credits

GPL-3.0-or-later. TITAN CORE is a derivative/aggregate project, with original authorship and upstream notices retained. The display name `chmod v0.1` applies to a custom AMPR 0.4.0 launch-notice build; it does not claim a new upstream runtime or authorship of that engine. Bundled component source snapshots, modified runtime source and notices are included under `vendor/`.

See [CREDITS.md](CREDITS.md), [LICENSE](LICENSE) and [THIRD_PARTY_NOTICES](THIRD_PARTY_NOTICES).
