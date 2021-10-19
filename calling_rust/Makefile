test: src/main.c
	cargo build
	clang src/main.c -o test ../target/debug/libcalling_rust.dylib

clean:
	cargo clean
	rm test
