windows-build-path = ./target/x86_64-pc-windows-gnu
windows-bin = ptuber.exe
windows-package-path = "$(windows-build-path)/package"

clean:
	cargo clean

windows:
	SFML_STATIC=1 SFML_STDCPP_STATIC=1 CXX="/usr/bin/x86_64-w64-mingw32-g++-posix" SFML_LIBS_DIR=/SFML/lib/ SFML_INCLUDE_DIR=/SFML/include/ cross build --target x86_64-pc-windows-gnu $(if $(release), --release)

linux:
	cargo build $(if $(release), --release)