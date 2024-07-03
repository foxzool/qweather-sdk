lint:
  @echo "Lint"
  cargo clippy --workspace --all-targets --all-features -- -Dwarnings

publish:
  @echo "publish to crates-io"
  cargo publish --registry crates-io