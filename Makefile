.PHONY: build
build:
	./node_modules/.bin/tauri build --target aarch64-apple-darwin
	./node_modules/.bin/tauri build --target x86_64-apple-darwin