release:
	cargo build --release 
	ln -sf target/release/png ./png

clean:
	cargo clean
	rm -f png