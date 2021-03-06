VERSION=0.0.18

all:
	@echo "Select target"

tag:
	git tag -a v${VERSION} -m v${VERSION}
	git push origin --tags

ver:
	sed -i 's/^version = ".*/version = "${VERSION}"/g' Cargo.toml
	sed -i 's/^version = ".*/version = "${VERSION}"/g' ./bma-benchmark-proc/Cargo.toml
	sed -i 's/^bma-benchmark-proc =.*/bma-benchmark-proc = { path = ".\/bma-benchmark-proc", version = "=${VERSION}" }/g' Cargo.toml

doc:
	grep -v "^//!" src/lib.rs > src/lib.rs.tmp
	sed 's|^|//! |g' README.md > src/lib.rs
	cat src/lib.rs.tmp >> src/lib.rs
	rm -f src/lib.rs.tmp
	cargo doc

release: pub tag

pub: doc publish-cargo-crate

publish-cargo-crate:
	cd bma-benchmark-proc && cargo publish
	sleep 20
	cargo publish
