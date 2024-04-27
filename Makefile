require-version:
ifndef VERSION
$(error VERSION is not set)
endif

setup-dist:
	if [ -d "./dist" ]; then rm -rf ./dist; fi
	mkdir -p ./dist/macos

.PHONY: build-apple-silicon
build-apple-silicon: require-version
	./node_modules/.bin/tauri build --target aarch64-apple-darwin
	cp ./src-tauri/target/aarch64-apple-darwin/release/bundle/dmg/Cognitio_$(VERSION)_aarch64.dmg ./dist/macos/Cognitio_$(VERSION)_aarch64.dmg

.PHONY: build-apple-intel
build-apple-intel: require-version
	./node_modules/.bin/tauri build --target x86_64-apple-darwin
	cp ./src-tauri/target/x86_64-apple-darwin/release/bundle/dmg/Cognitio_$(VERSION)_x64.dmg ./dist/macos/Cognitio_$(VERSION)_x64.dmg

.PHONY: build-apple-universal
build-apple-universal: require-version
	./node_modules/.bin/tauri build --target universal-apple-darwin
	cp ./src-tauri/target/universal-apple-darwin/release/bundle/dmg/Cognitio_$(VERSION)_universal.dmg ./dist/macos/Cognitio_$(VERSION)_universal.dmg

.PHONY: build
build: require-version setup-dist build-apple-silicon build-apple-intel build-apple-universal
