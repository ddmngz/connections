all: debug 

debug:
	wasm-pack build ./connections --debug --target=web --out-dir="../site/pkg"
release:
	wasm-pack build ./connections --release --target=web --out-dir="../site/pkg"


clean: 
	rm -r ./site/*

.PHONY: all debug release clean 

