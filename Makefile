all: setup 

setup: build

build:
	wasm-pack build ./nyt_connections --target=web --out-dir="../site/pkg"

serve: build
	./server.py

clean: 
	rm -r ./site/*

.PHONY: build move serve all setup clean

