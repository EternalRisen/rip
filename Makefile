build:
	@echo building...
	@cargo build --release

dev:
	@echo building...
	@cargo build

install:
	@echo installing...
	@cp "target/release/rip" "/usr/local/bin/rip"

uninstall:
	@echo removing...
	@rm -f "/usr/local/bin/rip"

