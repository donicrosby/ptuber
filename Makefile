clean:
	cargo clean

windows:
	SFML_STATIC=1 SFML_LIBS_DIR=/SFML/lib SFML_INCLUDE_DIR=/SFML/include  CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS="-Clink-self-contained=no" cross build --target x86_64-pc-windows-gnu -vvv