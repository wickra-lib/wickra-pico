# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Repository scaffold: governance, supply-chain configuration (`deny.toml`,
  `lychee.toml`, `osv-scanner.toml`, `repo-metadata.toml`), the host workspace
  (`wickra-pico-signal`, `wickra-pico-host`, `embedded-data`) with the
  `firmware/*` crates excluded, and the `no_std`-kernel decision (Weg A:
  `embed-core` git dependency — see `ARCHITECTURE.md`).

[Unreleased]: https://github.com/wickra-lib/wickra-pico/commits/main
