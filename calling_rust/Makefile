test: src/main.c src/lib.rs
	cargo build
	clang src/main.c -o test ../target/debug/libcalling_rust.a

clean:
	cargo clean
	rm test
