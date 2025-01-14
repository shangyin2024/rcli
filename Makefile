.PHONY: clean
clean:
	cargo fix --allow-dirty --allow-staged
	cargo clippy --all-targets
	cargo fmt --all
