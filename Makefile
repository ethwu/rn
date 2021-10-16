name := rn
bin := $(CARGO_HOME)/bin

.PHONY: install
install:
	cargo install --path .
	upx --best --lzma $(bin)/$(name)


