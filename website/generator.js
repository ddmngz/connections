import init, {setup} from '../pkg/nyt_connections.js';

async function run() {
    const module = await WebAssembly.compileStreaming(fetch("./pkg/nyt_connections_bg.wasm"));

    await init(module);
    setup();

    //const module = WebAssembly.compileStreaming(fetch('./pkg/nyt_connections_bg.wasm'));
    //await init();
    //await init(module);
}
run();

