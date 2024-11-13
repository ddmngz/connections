import init, {setup, setup_with_code} from './pkg/nyt_connections.js';

async function load_asm(){
    const module = await WebAssembly.compileStreaming(fetch("./pkg/nyt_connections_bg.wasm"));
    await init(module);
}

async function run_with_code(code) {
    load_asm();
    setup_with_code(code);
}

async function run() {
	load_asm();
    setup();
}

addEventListener("load", (_) => {
    document.documentElement.removeAttribute("hidden");
});


async function main(){
    let url = new URL(document.URL);
    if(url.searchParams.get("game") != null){
        let game_code = url.search;
        console.log(url)
        const game_url = new URL("./game.html",url);
        console.log(game_url)
        game_url.search = game_code;
        self.window.location.assign(game_url);
    }else{
	await load_asm();
	if (url.searchParams.get("puzzle") != null){
		let game_code = url.searchParams.get("puzzle");
        	console.log(game_code);
		run_with_code(game_code);
	}else{
	    run();
	}
    }
}


main();
//run();

