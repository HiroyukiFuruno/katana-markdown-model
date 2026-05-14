# Build the development-only manual harness
harness-check:
    {{CARGO}} check --manifest-path tools/manual-harness/Cargo.toml --locked

# Open the development-only manual harness for release-before inspection
harness-up path="":
    if [ -n "{{path}}" ]; then \
        {{CARGO}} run --manifest-path tools/manual-harness/Cargo.toml --locked -- "{{path}}"; \
    else \
        {{CARGO}} run --manifest-path tools/manual-harness/Cargo.toml --locked; \
    fi
