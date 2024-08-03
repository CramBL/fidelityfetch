import 'scripts/mod.just'

set shell := ["bash", "-uc"]

@_default:
    just --list --no-aliases
