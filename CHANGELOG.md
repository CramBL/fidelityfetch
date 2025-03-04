# Changelog

## [unreleased]

## [1.0.0]

### Changed

- Set minimum support rust version (MSRV) and verify in CI

### Dependencies

- `clap`: 4.5.30 → 4.5.31 ([#66](https://github.com/CramBL/fidelityfetch/pull/66))
- `clap_complete`: 4.5.45 → 4.5.46 ([#66](https://github.com/CramBL/fidelityfetch/pull/66))
- `mdns-sd`: 0.13.2 → 0.13.3 ([#66](https://github.com/CramBL/fidelityfetch/pull/66))
- `chrono`: 0.4.39 → 0.4.40 ([#66](https://github.com/CramBL/fidelityfetch/pull/66))
- `thiserror`: 2.0.11 → 2.0.12 ([#66](https://github.com/CramBL/fidelityfetch/pull/66))
- `crate-ci/typos`: 1.29.7 → 1.30.0 ([#67](https://github.com/CramBL/fidelityfetch/pull/67))

## [0.7.0]

### Added

- HTTP/2 support

### Changed

- Recognize `cxx` and `hxx` as C++ files.
- Add svgs for every recognized file extension and ensure alignment of icons.
- Align file size and timestamps

### Fixed

- Directories with one entry would say `1 items` instead of `1 item`.

### Dependencies

- `strum`: 0.27.0 → 0.27.1 ([#60](https://github.com/CramBL/fidelityfetch/pull/60))
- `cargo update`

### Documentation

- Add yocto example and update readme

## [0.5.0]

### Added

- Add environment variable for each option
- Add example systemd service unit file

## [0.4.3]

### Dependencies

- `clap`: 4.5.26 → 4.5.28 ([#55](https://github.com/CramBL/fidelityfetch/pull/55))
- `clap_complete`: 4.5.42 → 4.5.44 ([#55](https://github.com/CramBL/fidelityfetch/pull/55))
- `mdns-sd`: 0.13.1 → 0.13.2 ([#55](https://github.com/CramBL/fidelityfetch/pull/55))
- `strum`: 0.26.3 → 0.27.0 ([#55](https://github.com/CramBL/fidelityfetch/pull/55))
- `crate-ci/typos`: 1.29.4 → 1.29.7 ([#56](https://github.com/CramBL/fidelityfetch/pull/56))
- `cargo update`

## [0.4.2]

### Dependencies

- `mdns-sd`: 0.13.0 → 0.13.1
- `crambl/dependabot-changelog-writer`: 0.4.0 → 0.5.0
- `crate-ci/typos`: 1.28.3 → 1.29.4 ([#47](https://github.com/CramBL/fidelityfetch/pull/47))
- `axum`: 0.7.9 → 0.8.1 ([#48](https://github.com/CramBL/fidelityfetch/pull/48))
- `clap`: 4.5.23 → 4.5.26 ([#49](https://github.com/CramBL/fidelityfetch/pull/49))
- `clap_complete`: 4.5.38 → 4.5.42 ([#49](https://github.com/CramBL/fidelityfetch/pull/49))
- `tokio`: 1.42.0 → 1.43.0 ([#49](https://github.com/CramBL/fidelityfetch/pull/49))
- `thiserror`: 2.0.7 → 2.0.11 ([#49](https://github.com/CramBL/fidelityfetch/pull/49))

## [0.4.1]

### Changed

- Reduce response size by trimming white space
- Make the file list more compact

## [0.4.0]

### Added

- Allow logging to journald explicitly
- Allow setting logging destination to either `journald` (unix only), `stderr`, or `stdout`.

### Fix

- Main process now exits with a non-zero exit when failing to bind to a port.

### Misc.

- Update dependencies

## [0.3.3]

### Changed

- Update dependencies

### Internals

- Refactors
- More tests

## [0.3.2]

### Added

- npm publish job

## [0.3.1]

### Changed

- Update dependencies to get some critical fixes to some async dependencies.

## [0.3.0]

### Added

- Recognize and add icons to many more filetypes
- Better error messages

## [0.2.0]

### Added

- Completions for bash, zsh, fish, powershell, elvish.
- Favicon.ico
- Various tweaks

## [0.1.1]

### Fix

- Paths with spaces and non-English letters not being recognized
