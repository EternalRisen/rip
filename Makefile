build:
	@cargo build

install:
	cp "target/debug/rip" "/usr/local/bin/rip"

uninstall:
	rm -f "/usr/local/bin/rip"

