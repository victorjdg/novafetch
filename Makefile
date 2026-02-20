RUST_RELEASE_BIN_DIR=./target/release
CLI_EXEC=novafetch
RUSTFETCH_EXEC_RELEASE=$(RUST_RELEASE_BIN_DIR)/$(CLI_EXEC)

ifeq ($(PREFIX),)
	PREFIX := /usr/
endif

$(CLI_EXEC) $(RUSTFETCH_EXEC_RELEASE):
	cargo build --all --release

install:
	install -D -m755 $(RUST_RELEASE_BIN_DIR)/$(CLI_EXEC) $(PREFIX)/bin/

uninstall:
	- rm -f $(PREFIX)/bin/$(CLI_EXEC)
