.PHONY: docker clean

docker:
	# rustc 1.70.0 (90c541806 2023-05-31)
	cp -f crate.proxy ~/.cargo/config
	rustup target add x86_64-unknown-linux-musl
	cargo build --target x86_64-unknown-linux-musl --release

	ls target/
	docker build -f Dockerfile -t rust-api-demo:latest .
	docker run --rm --name rust-api-demo -p 8080:8080 -d rust-api-demo:latest

clean:
	docker stop rust-api-demo
	docker rm -f rust-api-demo
	docker rmi rust-api-demo:latest
	rm -rf target
