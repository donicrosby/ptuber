windows-build-path = ./target/x86_64-pc-windows-gnu
windows-bin = ptuber.exe
windows-package-path = "$(windows-build-path)/package"

clean:
	cargo clean

windows-build:
	SFML_STATIC=1 SFML_STDCPP_STATIC=1 CXX="/usr/bin/x86_64-w64-mingw32-g++-posix" SFML_LIBS_DIR=$(shell pwd)/SFML-libs/mingw/lib/ SFML_INCLUDE_DIR=$(shell pwd)/SFML-libs/mingw/include/ cargo build --target x86_64-pc-windows-gnu $(if $(release), --release)

windows-package: windows-build
	mkdir -p $(windows-package-path)
	@if [ ! -z "$(release)" ] ; then \
		cp $(windows-build-path)/release/$(windows-bin) $(windows-package-path) ; \
	else \
		cp $(windows-build-path)/debug/$(windows-bin) $(windows-package-path) ; \
	fi
	./scripts/package.sh $(windows-package-path)

windows: windows-package
	cd ${windows-package-path}; zip package-`date +'%Y-%m-%d'`.zip ${windows-bin} *.dll

linux:
	cargo build $(if $(release), --release)