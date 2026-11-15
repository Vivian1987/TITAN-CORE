# TITAN runtime launch notice

Based on quer3q/ampr-emu-ng tag v0.4.0 (GPL-3.0-or-later). Previous TITAN notice: chmod v0.1 with mode/running suffix. Current owner-requested notice: PLAY NO LIMIT, exactly. Only load_notice and its adjacent description in crates/ampr-sys/src/rt/exports.rs differ from the previous custom source. Mode selection, hooks, ABI, paths and decoding are unchanged. The existing component UI label chmod v0.1 remains. Upstream authorship is preserved.

Original upstream binary/source and previous custom source are retained. play-no-limit-source.zip contains the complete current custom source, including the upstream ps5/target/x86_64-ps5-freebsd.json build specification missing from the older custom ZIP.

The exact modified production function was extracted into the notice test harness and tested across four modes plus an empty mode. The full upstream host workspace test suite passed. make -j2 verify passed: 137 exports, 141 definitions, 103 imports, FSELF 0x02000001. No console gameplay test was performed by the agent.
