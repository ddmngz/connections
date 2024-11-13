all: move 

setup: move

build:
	wasm-pack build ./nyt_connections --target=web --out-dir="./site/pkg"


move: build
	cp *.html *.js *.css ./site/

serve: move
	./server.py


.PHONY: build move serve all

