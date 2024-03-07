set shell := ["sh", "-c"]


init:
    cargo install cargo-watch

dev-server:
	cargo watch -w src -w templates -w tailwind.config.js -w input.css -x run 

dev-tailwind:
	tailwindcss -i input.css -o assets/output.css --watch=always

build-server:
	cargo build --release

build-tailwind:
	./tailwindcss -i input.css -o assets/output.css --minify

dev:
    just dev-tailwind &
    just dev-server &