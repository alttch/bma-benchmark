VERSION=0.0.1

all:
	@echo "Select target"

tag:
	git tag -a v${VERSION} -m v${VERSION}
	git push origin --tags

ver:
	sed -i 's/^version = ".*/version = "${VERSION}"/g' Cargo.toml

release: pub tag

pub: publish-cargo-crate

publish-cargo-crate:
	cargo publish
