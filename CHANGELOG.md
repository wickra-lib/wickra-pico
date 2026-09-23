# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] - 2026-09-23

A maintenance release: the firmware and its signal engine are unchanged. It
publishes the refreshed dependency tree and toolchain pins.

### Changed

- **The family pins follow the owners' releases.** `wickra-embed-core` =0.1.1 ->
  =0.1.2 -- the exact pins this repository keeps on its siblings move to the
  versions those repositories release in the same train, and every tracked
  lockfile follows.
- **Third-party dependencies refreshed.** `Cargo.lock` takes 15 crates to their
  newest versions compatible with the Rust floor (the lock now resolves
  MSRV-aware, see below), run across the family in one pass so every repository
  resolves the same day's versions. The refresh itself changes no manifest.
- **The lockfile resolves for the Rust floor.** `.cargo/config.toml` sets
  `incompatible-rust-versions = "fallback"`, so `cargo update` takes the newest
  version the workspace's `rust-version` can build rather than the newest
  release -- the setting compile, copilot and shazam already carried, now
  family-wide. Without it, a routine refresh elsewhere in the family raised the
  icu crates to 2.3.0, which declares Rust 1.88, above a 1.86 floor. Re-resolved
  under it, the lock steps back to the newest versions the floor can build for
  `wasip2`, `wit-bindgen`.
- **The README's static badges are served by the organization** rather than
  hot-linked from shields.io, so they no longer break when shields is down.

### Tests

- **The golden generator's own artifacts are checked, not only the sequence it
  produces.** The CSV feed parses back to the feed's exact bits, the embedded
  `FEED` source is the committed one line for line, every artifact path lies
  inside the repository, `check` succeeds against the committed golden, and
  every signal line is an index and a token.

## [0.1.1] - 2026-09-18

### Changed

- **Family pins follow the owners' releases:** wickra-embed-core =0.1.0 -> =0.1.1. No code of this repository changes; the engine it links is the one those releases ship.
- **Every README follows wickra's shape.** A cross-repo scan compared the
  heading skeleton of each README against wickra's and this repository's
  differed throughout. The root README opens as wickra's does (banner, badges,
  the one-liner, the ecosystem line, no separate H1), gains the `## Status`
  section every sibling has, the License section carries wickra's wording and
  its `### Contribution` clause, and the shared sections run in wickra's order;
  `fuzz/README.md` and the `## Editing the docs` section of `docs/README.md`
  exist as they do in wickra.

### Changed

- **The repository spells shared things the way the family does.** A cross-repo
  scan lined the 24 wickra-lib repositories up and this one differed in:
  `fuzz/Cargo.lock` committed where 22 siblings treat it as the local artifact
  it is (it recorded crate versions from before the 0.1.0 bump and was never
  refreshed; it is untracked and ignored now, like the fuzz build directory),
  `panic-halt = "1"` where the family writes `"1.0"`, and the fuzz job on a
  rolling nightly rather than the family's pinned `nightly-2026-07-01`.

## [0.1.0] - 2026-09-14

### Changed

- **The kernel is the published `wickra-embed-core`.** The signal crate
  depended on the wickra-embed repository by git under the crate's
  pre-release name; it depends on `wickra-embed-core` 0.1.0 from crates.io
  now, pinned exactly as the released siblings pin each other, so a newer
  patch cannot leave two copies of the kernel in one graph. The firmware lock
  follows.
- **The repository has the family's shape.** SPDX-named licence copies under
  `LICENSES/` and beside each crate, a `docs/` index, the detailed issue and
  pull-request templates written for a firmware repository, Dependabot over
  the fuzz manifest, and no `rust-toolchain.toml`: CI pins its toolchains per
  job and installs the bare-metal target it needs; a local checkout uses its
  own.
- **CI and the release front.** Pull requests build against `main` only;
  the flake-resilience environment is set once; coverage uploads under the
  repository slug; osv-scanner, a fuzz smoke and the repository checks
  (`scripts/check_version_sync.py`, `scripts/check_license_copies.py`) run on
  every change; actionlint, the bench, CodSpeed, Scorecard and zizmor have
  their own workflows; CodeQL analyses Rust under a config. `release.yml`
  refuses anything but a `v*` tag, checks the tag against the declared
  version, builds the flashable UF2, gates on the tagged commit's CI, attests
  provenance and publishes the GitHub Release last.

### Added

- A criterion bench over `SignalEngine::on_tick` (`codspeed-criterion-compat`,
  so CodSpeed measures instruction counts on every push) and a libfuzzer
  target feeding arbitrary ticks through the engine, kept out of the
  workspace as cargo-fuzz builds it with sanitizer flags.

- CI/CD: the repository's workflow suite — `ci.yml` (host jobs: fmt, clippy,
  3-OS test, `no_std` thumbv6m build, host-golden parity check, MSRV 1.86,
  cargo-deny), `firmware.yml` (the RP2040 `thumbv6m-none-eabi` cross-compile,
  clippy and size report — the hardware gate), plus `codeql`, `scorecard`,
  `zizmor`, link-check and metadata-audit workflows, and a demo `release.yml`
  that builds the flashable `.uf2` with a SLSA build-provenance attestation and
  attaches it to a GitHub Release (no package registries — this is a demo).
- `docs/SIGNAL.md` and `docs/DETERMINISM.md`: the exact EMA-cross pipeline and
  the cross-target byte-parity guarantee (`f64` end to end, no platform math in
  the signal path, `libm::sinf` for a reproducible feed).
- `libm` is now used for the feed formula's `sin`, so the committed golden feed
  is bit-identical regardless of which OS runs `bless`.
- `docs/`: the demo reproducibility guides — `FLASHING.md` (BOOTSEL UF2 and
  probe-rs), `WIRING.md` (the on-board GPIO25 LED needs no wiring, plus an
  external-LED variant) and `VIDEO_SCRIPT.md` (the 30-second "LED blinks on the
  cross" clip).
- `golden/`: the cross-target reference corpus — `data/ema_cross.csv`, the
  byte-exact `expected/ema_cross.txt`, and `README.md` documenting the feed
  formula, the `bless` regeneration command, and why the on-device sequence
  matches the golden by construction (identical engine + feed) plus the host
  `check` and drift-guard verifications.
- `firmware/rp-pico`: the Raspberry Pi Pico (RP2040) firmware — the showcase.
  `#![no_std]`/`#![no_main]` on `thumbv6m-none-eabi` (rp-pico BSP + cortex-m-rt,
  blocking, `panic-halt`, no allocator). It streams the embedded `FEED` through
  the shared `SignalEngine` and toggles the on-board LED (GPIO25) on each cross —
  the same engine as the host reference, so the on-device sequence matches the
  golden. Workspace-excluded (own target/linker/`panic = "abort"`); a 10.5 KB
  release ELF, far under the 2 MB flash.
- `wickra-pico-host`: the host golden generator and byte-exact parity checker.
  `bless` regenerates the CSV feed, the embedded `FEED` const and the expected
  signal sequence from the single feed formula (`100 + 15·sin(i/8) + 0.05·i`,
  128 f32 ticks); `check` recomputes the sequence and asserts it matches the
  committed golden byte-for-byte — the host side of the cross-target guarantee.
  A drift-guard test pins the committed `embedded-data` `FEED` to the formula.
- `embedded-data`: the generated `FEED`/`FEED_LEN` const replay feed.
- `wickra-pico-signal`: the shared, `#![no_std]`, allocation-free EMA(9)/EMA(21)
  cross engine over `wickra-embed-core`. `SignalEngine::on_tick` streams prices and
  emits a `Signal` (golden/death cross) on each crossing, with an exact-`0.0` tie
  treated as a neutral hold. Host tests (warmup, ordered golden-then-death,
  reset, and proptest alternation/determinism) plus the `thumbv6m-none-eabi`
  build verify it runs identically on the host and bare metal.
- Repository scaffold: governance, supply-chain configuration (`deny.toml`,
  `lychee.toml`, `osv-scanner.toml`, `repo-metadata.toml`), the host workspace
  (`wickra-pico-signal`, `wickra-pico-host`, `embedded-data`) with the
  `firmware/*` crates excluded, and the `no_std`-kernel decision (path A:
  depend on `wickra-embed-core` — see `ARCHITECTURE.md`).

[Unreleased]: https://github.com/wickra-lib/wickra-pico/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/wickra-lib/wickra-pico/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/wickra-lib/wickra-pico/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/wickra-lib/wickra-pico/releases/tag/v0.1.0
