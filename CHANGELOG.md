# Changelog

## [unreleased]

### Dependencies

- `tokio`: 1.47.0 → 1.47.1 ([#106](https://github.com/CramBL/fidelityfetch/pull/106))
- `tokio-util`: 0.7.15 → 0.7.16 ([#106](https://github.com/CramBL/fidelityfetch/pull/106))
- `clap`: 4.5.41 → 4.5.45 ([#110](https://github.com/CramBL/fidelityfetch/pull/110))
- `clap_complete`: 4.5.55 → 4.5.57 ([#110](https://github.com/CramBL/fidelityfetch/pull/110))
- `mdns-sd`: 0.13.11 → 0.14.1 ([#111](https://github.com/CramBL/fidelityfetch/pull/111))
- `percent-encoding`: 2.3.1 → 2.3.2 ([#111](https://github.com/CramBL/fidelityfetch/pull/111))
- `thiserror`: 2.0.12 → 2.0.16 ([#111](https://github.com/CramBL/fidelityfetch/pull/111))

## [1.0.5]

### Changed

- Migrate to back to the original `cargo-dist` project as it is now maintained again
- Publish to homebrew
- Set the example service file type to `exec`

### Dependencies

- `crate-ci/typos`: 1.33.1 → 1.34.0 ([#99](https://github.com/CramBL/fidelityfetch/pull/99))
- `clap`: 4.5.40 → 4.5.41 ([#101](https://github.com/CramBL/fidelityfetch/pull/101))
- `clap_complete`: 4.5.54 → 4.5.55 ([#101](https://github.com/CramBL/fidelityfetch/pull/101))
- `mdns-sd`: 0.13.9 → 0.13.11 ([#101](https://github.com/CramBL/fidelityfetch/pull/101))
- `strum`: 0.27.1 → 0.27.2 ([#102](https://github.com/CramBL/fidelityfetch/pull/102))
- `tokio`: 1.45.1 → 1.47.0 ([#103](https://github.com/CramBL/fidelityfetch/pull/103))

## [1.0.4]

### Changed

- Scrutinize dependencies and reduce binary size by ~15%

### Dependencies

- `tokio`: 1.45.0 → 1.45.1 ([#90](https://github.com/CramBL/fidelityfetch/pull/90))
- `crate-ci/typos`: 1.32.0 → 1.33.1 ([#92](https://github.com/CramBL/fidelityfetch/pull/92))
- `clap`: 4.5.37 → 4.5.40 ([#94](https://github.com/CramBL/fidelityfetch/pull/94))
- `clap_complete`: 4.5.50 → 4.5.54 ([#94](https://github.com/CramBL/fidelityfetch/pull/94))

## [1.0.3]

### Dependencies

- `crate-ci/typos`: 1.31.1 → 1.32.0 ([#83](https://github.com/CramBL/fidelityfetch/pull/83))
- `clap`: 4.5.36 → 4.5.37 ([#87](https://github.com/CramBL/fidelityfetch/pull/87))
- `clap_complete`: 4.5.47 → 4.5.50 ([#87](https://github.com/CramBL/fidelityfetch/pull/87))
- `axum`: 0.8.3 → 0.8.4 ([#87](https://github.com/CramBL/fidelityfetch/pull/87))
- `tokio`: 1.44.2 → 1.45.0 ([#87](https://github.com/CramBL/fidelityfetch/pull/87))
- `local-ip-address`: 0.6.3 → 0.6.5 ([#87](https://github.com/CramBL/fidelityfetch/pull/87))
- `mdns-sd`: 0.13.6 → 0.13.9 ([#87](https://github.com/CramBL/fidelityfetch/pull/87))
- `chrono`: 0.4.40 → 0.4.41 ([#87](https://github.com/CramBL/fidelityfetch/pull/87))
- `tokio-util`: 0.7.14 → 0.7.15 ([#87](https://github.com/CramBL/fidelityfetch/pull/87))
- `temp-dir`: 0.1.14 → 0.1.16 ([#87](https://github.com/CramBL/fidelityfetch/pull/87))

## [1.0.2]

### Changed

- Migrate from the discontinued original [cargo-dist](https://github.com/axodotdev/cargo-dist) project to the [astral fork](https://github.com/astral-sh/cargo-dist)

### Dependencies

- `tokio`: 1.44.0 → 1.44.2 ([#79](https://github.com/CramBL/fidelityfetch/pull/79))
- `tokio-util`: 0.7.13 → 0.7.14 ([#74](https://github.com/CramBL/fidelityfetch/pull/74))
- `clap_complete`: 4.5.46 → 4.5.47 ([#75](https://github.com/CramBL/fidelityfetch/pull/75))
- `axum`: 0.8.1 → 0.8.3 ([#77](https://github.com/CramBL/fidelityfetch/pull/77))
- `crate-ci/typos`: 1.30.0 → 1.31.1 ([#78](https://github.com/CramBL/fidelityfetch/pull/78))
- `clap`: 4.5.31 → 4.5.36 ([#80](https://github.com/CramBL/fidelityfetch/pull/80))
- `mdns-sd`: 0.13.3 → 0.13.6 ([#80](https://github.com/CramBL/fidelityfetch/pull/80))

## [1.0.1]

### Changed

- Remove support for the `i686-pc-windows-msvc` and `aarch64-pc-windows-msvc` targets. Install from source is still possible, but the lack of support for WiX v4 in `cargo-xwin` means distribution for these targets are temporarily paused

### Dependencies

- `tokio`: 1.43.0 → 1.44.0

## [1.0.0]

### Changed

- Set minimum support rust version (MSRV) and verify in CI
- Update `dist` to v0.28.0

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
