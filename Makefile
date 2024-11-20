all: debug 

debug:
	wasm-pack build ./nyt_connections --debug --target=web --out-dir="../site/pkg"
release:
	wasm-pack build ./nyt_connections --release --target=web --out-dir="../site/pkg"


clean: 
	rm -r ./site/*

.PHONY: all debug release clean 

