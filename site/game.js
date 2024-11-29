import init from './pkg/nyt_connections.js';

async function run() {
    const url = "./pkg/nyt_connections_bg.wasm";
    const FetchOptions = {
        headers:{
            "Content-Type": "application/wasm",
        }
    };

    const wasm = fetch(url, FetchOptions);
    const module = await WebAssembly.compileStreaming(wasm);
    await init(module);
}


addEventListener("load", (_) => {
    document.documentElement.removeAttribute("hidden");
});

await run();
