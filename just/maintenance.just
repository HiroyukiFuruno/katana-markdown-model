# ============================================================
# Maintenance
# ============================================================

# 古いビルド成果物を掃除する（7日より古いものだけ）
sweep:
    @{{CARGO}} sweep --time 7 || true

# ビルド成果物を削除する
clean: sweep
    {{CARGO}} clean

# Cargo.toml の互換範囲内で依存クレートを更新する
update-safe:
    {{CARGO}} update

# 依存クレートを最新版へ更新し、Cargo.lock も更新する
update:
    {{CARGO}} upgrade -i
    {{CARGO}} update

# 古い依存クレートを確認する（cargo-outdated が必要）
outdated:
    @cp Cargo.toml Cargo.toml.bak
    @sed -e '/^\[patch\.crates-io\]/,$d' Cargo.toml.bak > Cargo.toml
    @{{CARGO}} outdated --workspace || (mv Cargo.toml.bak Cargo.toml && exit 1)
    @mv Cargo.toml.bak Cargo.toml
