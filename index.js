import init, {setup} from './wasm/pkg/nyt_connections.js';

async function run() {
    const module = await WebAssembly.compileStreaming(fetch("./wasm/pkg/nyt_connections_bg.wasm"));
    await init(module);
    setup();
}

addEventListener("load", (_) => {
    document.documentElement.removeAttribute("hidden");
});


function main(){
    let url = new URL(document.URL);
    if(url.searchParams.get("game") != null){
        let game_code = url.search;
        const game_url = new URL("/game.html",document.URL);
        game_url.search = game_code;
        self.window.location.assign(game_url);
    }else{
        run();
    }
}


main();
//run();

