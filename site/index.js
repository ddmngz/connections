import init, {setup} from './pkg/nyt_connections.js';

async function run() {
    const module = await WebAssembly.compileStreaming(fetch("./pkg/nyt_connections_bg.wasm"));
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
        console.log(url)
        const game_url = new URL("./game.html",url);
        console.log(game_url)
        game_url.search = game_code;
        self.window.location.assign(game_url);
    }else{
        run();
    }
}


main();
//run();

