build:
	rustc -C target-feature=+crt-static hello.rs

run:	build
	./hello

clean:
	rm -f ./hello
