ARCH:=		x86_64
VENDOR:=	unknown
TARGET_OS:=	freebsd

TARGET_TRIPLE=	${ARCH}-${VENDOR}-${TARGET_OS}


all: cargo

bootstrap: main.rs cmderror.rs rscrate.rs
	rustc main.rs -o bootstrap

cargo: bootstrap
	./bootstrap ${TARGET_OS} ${TARGET_TRIPLE}

clean:
	rm -rf bootstrap target
