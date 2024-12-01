import init, {start_editor} from './pkg/nyt_connections.js';

async function load_asm(){
    const module = await WebAssembly.compileStreaming(fetch("./pkg/nyt_connections_bg.wasm"));
    await init(module);
}



addEventListener("load", (_) => {
    document.documentElement.removeAttribute("hidden");
});



await load_asm();
start_editor();

//run();

