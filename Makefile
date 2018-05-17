TARGET ?= --target armv7-unknown-linux-musleabihf
all:: format check lint build clean
build::
	cargo build $(TARGET) --release
check::
	cargo check $(TARGET) --verbose
clean::
	cargo clean
debug::
	cargo build $(TARGET)
format::
	cargo fmt
run::
	$(error TODO scp to Pi and execute via ssh)
