# ============================================================
# Maintenance
# ============================================================

# Sweep old build artifacts older than 7 days
sweep:
    @{{CARGO}} sweep --time 7 || true

# Remove build artifacts
clean: sweep
    {{CARGO}} clean

# Update dependency crates within Cargo.toml-compatible ranges
update-safe:
    {{CARGO}} update

# Upgrade dependency crates to the latest versions and update Cargo.lock
update:
    {{CARGO}} upgrade -i
    {{CARGO}} update

# List outdated dependency crates; requires cargo-outdated
outdated:
    @cp Cargo.toml Cargo.toml.bak
    @sed -e '/^\[patch\.crates-io\]/,$d' Cargo.toml.bak > Cargo.toml
    @{{CARGO}} outdated --workspace || (mv Cargo.toml.bak Cargo.toml && exit 1)
    @mv Cargo.toml.bak Cargo.toml
