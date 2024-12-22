import init, {ConnectionPuzzle} from './pkg/nyt_connections.js';
import {start_game} from './game.js';

async function load_asm(){
    const url = "./pkg/nyt_connections_bg.wasm";
    const FetchOptions = {
        headers:{
            "Content-Type": "application/wasm",
        }
    };

    const wasm = await fetch(url, FetchOptions);
    const module = await WebAssembly.compileStreaming(wasm);
    await init({module_or_path:module});
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
        this.theme.value = connection_set.theme();

        const words = connection_set.words_list();
        //const words = connection_set.words_list();
        this.words.forEach((word, index) => {
            word.value = words[index];
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
        const words = this.words.map((div) => {
            return (div.value);
        });
        const list = [this.theme.value].concat(words)
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
        this.yellow.set_text(connection_puzzle.yellow_owned());
        this.green.set_text(connection_puzzle.green_owned());
        this.purple.set_text(connection_puzzle.purple_owned());
        this.blue.set_text(connection_puzzle.blue_owned());
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

function game_code(){
    return Dom.url.searchParams.get("game");
}

function edit_code(){
    return Dom.url.searchParams.get("edit");
}

function is_full(){
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
    const game = game_code();
    const edit = edit_code();


    if (game != null){
        try{
            const puzzle = ConnectionPuzzle.decode(game);
            start_game(Dom.game_div, puzzle);
            return;
        }catch{
        }
    }
    

    if(edit != null){
        try{
            const puzzle = ConnectionPuzzle.decode(edit);
            Dom.inputs.set_text(puzzle);
        }catch{
        }
    } 

    try{
        set_callbacks();
    }catch (e){
        console.log(e)
    }
}

export class Button{
    constructor(id, callback) {
        this.div = document.getElementById(id);
        this.callback = callback;
    }
    enable(){
        this.div.disabled = false;
        this.div.addEventListener("click", this.callback);
    }

    disable(){

        this.div.disabled = true;
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
    const puzzle = get_puzzle();
    start_game(Dom.game_div, puzzle);
}


function get_puzzle(){
    const {yellow, blue, green, purple} = Dom.inputs.to_args();
    const puzzle = ConnectionPuzzle.from_js(yellow, blue, purple, green);
    return puzzle;

}

function enable_game(){
    const puzzle = get_puzzle();
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



