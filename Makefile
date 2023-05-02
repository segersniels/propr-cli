.DEFAULT_GOAL := build

clean:
	cargo clean

build: clean
	cargo build --release

dev:
	cargo watch -- cargo build

lint:
	cargo clippy
	cargo fmt --check

lint-fix:
	cargo clippy --fix --allow-dirty
	cargo fmt

version:
	@cargo pkgid | sed 's/.*[@#]\(.*\)/\1/'

changelog:
	./scripts/generate-tags.sh
	rm CHANGELOG.md
	gitmoji-changelog . -o CHANGELOG.md