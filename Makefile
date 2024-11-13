all: move 

setup: move

build:
	wasm-pack build ./nyt_connections --target=web --out-dir="./site/pkg"


move: build
	cp -r *.html *.js *.css ./nyt_connections/pkg ./site/

serve: move
	./server.py


.PHONY: build move serve all

