import init, {ConnectionPuzzle} from './pkg/nyt_connections.js';

async function load_asm(){
    const url = "./pkg/nyt_connections_bg.wasm";
    const FetchOptions = {
        headers:{
            "Content-Type": "application/wasm",
        }
    };

    const wasm = await fetch(url, FetchOptions);
    const module = await WebAssembly.compileStreaming(wasm);

    await init(module);
}

const color = {
    blue: 'blue',
    purple: 'purple',
    yellow: 'yellow',
    green: 'green',
}

export class PuzzleArgs{
    constructor(theme, words){
        this.theme = theme;
        this.words = words;
    }
    
    get theme(){
        this.theme;
    }

    get words(){
        this.words;
    }
}

class InputSet{
    constructor(color){
        const wrapper_div = document.getElementById(color);
        this.theme = wrapper_div.getElementsByClassName("theme").item(0);
        const words = wrapper_div.getElementsByClassName("word-set").item(0);
        this.words = Array.from(words.children);
    }

    set_text(connection_set){
        this.theme.textContent = connection_set.theme();
        const words = connection_set.words();
        this.words.forEach((word, index) => {
            word.textContent = words[index];
        });
    }

    // how do I dO THIS UAGHHh
    set_callbacks(start){
        
        function update_elem(elem, index){
            if(elem.value != ""){
                Dom.inputs_filled[index] = true;
                check_full();
            }
        }

        this.theme.addEventListener("input", (elem) => {update_elem(elem, start)}, {passive:true});
        this.words[0].addEventListener("input", (elem) => {update_elem(elem, start+1)}, {passive:true});
        this.words[1].addEventListener("input", (elem) => {update_elem(elem, start+2)}, {passive:true});
        this.words[2].addEventListener("input", (elem) => {update_elem(elem, start+3)}, {passive:true});
        this.words[3].addEventListener("input", (elem) => {update_elem(elem, start+4)}, {passive:true});
    }

    full(){
        return this.theme.full && this.words.every((elem) => {elem.full})
    }

    args(){
        console.log(this.theme.value);
        const words = this.words.map((div) => {
            console.log(div);
            return (div.value);
        });
        const list = [this.theme.value].concat(words)
        console.log(list);
        return(list);
    }

}

class Inputs{
    constructor(){
        this.yellow = new InputSet(color.yellow)
        this.green = new InputSet(color.green)
        this.purple = new InputSet(color.purple)
        this.blue = new InputSet(color.blue)
    }

    set_text(connection_puzzle){
        this.yellow.set_text(connection_puzzle.purple());
        this.green.set_text(connection_puzzle.green());
        this.purple.set_text(connection_puzzle.purple());
        this.blue.set_text(connection_puzzle.blue());
    }

    set_callbacks(){
        this.yellow.set_callbacks(0);
        this.green.set_callbacks(5);
        this.purple.set_callbacks(10);
        this.blue.set_callbacks(15);
    }

    to_args(){
        return {yellow:this.yellow.args(), blue:this.blue.args(), purple:this.purple.args(), green:this.green.args()}

    }


}

function get_code(){
    console.log(Dom.url);
    return Dom.url.searchParams.get("game");
}

function is_full(){
    console.log(Dom.inputs_filled)
    for (var i = 0; i < Dom.inputs_filled.length; i++) { 
        if(Dom.inputs_filled[i] == false){
            return false;
        }
    }
    return true;

}

function check_full(){
    if (is_full()){
        enable_game();
    }else{
        disable_game();
    }
}


function start_editor(){
    const code = get_code();
    if(code != null){
        const puzzle = ConnectionPuzzle.decode(code);
        Dom.inputs.set_text(puzzle);
    }
    try{
        set_callbacks();
    }catch (e){
        console.log(e)
    }
}

class Button{
    constructor(id, callback) {
        this.div = document.getElementById(id);
        this.callback = callback;
    }
    enable(){
        this.div.classList.remove("hidden");
        this.div.addEventListener("click", this.callback);
    }

    disable(){
        this.div.classList.add("hidden");
        this.div.removeEventListener("click", this.callback);
    }
}

class CreateGame extends Button{
    constructor(){
        super("submit", enable_game)
    }

}

class CopyLink extends Button{
    constructor(){
        super("copy_link", copy_link)
    }
}

class TryGame extends Button{
    constructor(){
        super("try_game", try_game);
    }
}

function try_game(){
}




function enable_game(){
    const {yellow, blue, green, purple} = Dom.inputs.to_args();
    const puzzle = ConnectionPuzzle.from_js(yellow, blue, purple, green);
    Dom.puzzle = puzzle;
    Dom.try_game.enable();
    Dom.copy_link.enable();
}

function disable_game(){
    Dom.try_game.disable();
    Dom.copy_link.disable();
}

async function copy_link(){
    if (Dom.puzzle == null){
        return
    }
    const url = new URL("game.html",Dom.url);
    url.searchParams.set("game", Dom.puzzle.encode());
    // Copy the text inside the text field
    await navigator.clipboard.writeText(url.href);
    await copied_link(Dom);
}

async function copied_link(){
    Dom.copied_message.show();
    const keyframes = [{opacity:0, easing:"ease-out"},{opacity:1, easing:"ease-in"},{opacity:0 }]
    console.log(keyframes);
    await Dom.copied_message.animate(keyframes, 2000).finished;
    Dom.copied_message.close();
}

function set_callbacks(){
    Dom.inputs.set_callbacks();
    Dom.create_game.enable();
}

addEventListener("load", async (_) => {
    document.documentElement.removeAttribute("hidden");
    await main()
});

const Dom = {
    inputs: new Inputs(),
    create_game:new CreateGame(),
    try_game : new TryGame(),
    copy_link : new CopyLink(),
    copied_message : document.getElementById("copier"),
    game_div : document.getElementById("game"),
    url : new URL(document.URL),
    puzzle : null,
    inputs_filled: [
        false,false,false,false,false,
        false,false,false,false,false,
        false,false,false,false,false,
        false,false,false,false,false,
    ]
}
async function main(){
    await load_asm();

    start_editor();
}

