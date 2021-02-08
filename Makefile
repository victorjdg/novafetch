RUST_RELEASE_BIN_DIR=./target/release
CLI_EXEC=rustfetch
RUSTFETCH_EXEC_RELEASE=$(RUST_RELEASE_BIN_DIR)/$(CLI_EXEC)

ifeq ($(PREFIX),)
	PREFIX := /usr/
endif

$(CLI_EXEC) $(RUSTFETCH_EXEC_RELEASE):
	cargo build --all --release

install:
	install $(RUST_RELEASE_BIN_DIR)/$(CLI_EXEC) $(DESTDIR)$(PREFIX)/bin/

unistall:
	- rm -f $(DESTDIR)$(PREFIX)/bin/$(CLI_EXEC)
