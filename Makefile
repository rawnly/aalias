prog :=aalias
debug ?=

ifdef debug
	release :=
	target :=debug
else
	release :=--release
	target :=release
endif

build:
	cargo build $(release)

install:
	mv ./target/$(target)/$(prog) /usr/local/bin/$(prog)

tar:
	cargo build --release;
	tar -czf add-alias.tar.gz ./target/release/add-alias
	shasum -a 256 add-alias.tar.gz

tag:
	git tag -a v$(version) -m "version $(version)"
	git push --tags

help:
	@echo "usage: make $(prog) [debug=1]"
