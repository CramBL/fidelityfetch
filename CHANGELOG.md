# Changelog

## Unreleased

## [0.4.1]

### Changed

- Reduce response size by trimming white space

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
