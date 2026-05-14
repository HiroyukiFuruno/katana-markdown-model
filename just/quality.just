# Apply rustfmt
fmt:
    {{CARGO}} fmt --all

# Check rustfmt output
fmt-check:
    {{CARGO}} fmt --all -- --check

# Run Clippy with zero warnings
lint:
    {{CARGO}} clippy -j {{JOBS}} --workspace --all-targets --all-features --locked -- -D warnings

# Run repository AST lint through the shared KAL crate
ast-lint:
    {{CARGO}} test -j {{JOBS}} --test repository_ast_lint --locked -- --nocapture

# Run unit and integration tests
test:
    {{CARGO}} test --workspace --locked

# Validate active OpenSpec changes
openspec-check:
    @for change_dir in openspec/changes/*; do \
        [ -d "$change_dir" ] || continue; \
        change="$(basename "$change_dir")"; \
        [ "$change" = "archive" ] && continue; \
        scripts/openspec validate "$change" --strict; \
    done

# Local quality gate
check: fmt-check lint ast-lint test openspec-check
    @echo "All checks passed"

# Release readiness gate without publishing
release-check: check
    {{CARGO}} package --locked --allow-dirty
    {{CARGO}} publish --dry-run --locked --allow-dirty
