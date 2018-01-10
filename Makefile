.PHONY: all clean

all: launch.exe

launch.exe: launch/src/main.rs launch/Cargo.toml libres.a
	(cd launch && cargo build --release --target=x86_64-pc-windows-gnu)
	mv launch/target/x86_64-pc-windows-gnu/release/launch.exe .
	x86_64-w64-mingw32-strip launch.exe

icon.res: launch.rc icon.ico
	x86_64-w64-mingw32-windres launch.rc -O coff -o icon.res

libres.a: icon.res
	x86_64-w64-mingw32-ar q libres.a icon.res

clean:
	-rm *.a *.res
