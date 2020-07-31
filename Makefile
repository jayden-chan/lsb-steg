build: build-skylake

build-general:
	cargo build --release

build-skylake:
	RUSTFLAGS="-Ctarget-cpu-skylake" cargo build --release

.PHONY: build build-general build-skylake
