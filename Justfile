set shell := ["bash", "-uc"]

REPO_ROOT := justfile_directory()
RTK := env_var_or_default("RTK", `command -v rtk 2> /dev/null || true`)
RTK_CMD := if RTK == "" { "" } else { RTK + " " }
CARGO := RTK_CMD + "cargo"
JOBS := env_var_or_default("JOBS", "2")

export RUSTFLAGS := env_var_or_default("RUSTFLAGS", "-D warnings")

[private]
default: help

# Show available recipes
help:
    @just --list --unsorted

import 'just/quality.just'
import 'just/harness.just'
import 'just/maintenance.just'
