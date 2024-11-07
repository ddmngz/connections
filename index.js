import init, {GameState, start_state} from './pkg/nyt_connections.js';



async function run() {
    const module = await WebAssembly.compileStreaming(fetch("./pkg/nyt_connections_bg.wasm"));

    console.log(module);
    await init(module);
    console.log("initialized");

    //const module = WebAssembly.compileStreaming(fetch('./pkg/nyt_connections_bg.wasm'));
    //await init();
    //await init(module);
}




function main(){


    const initializeCards = (card, card_key) => {
        card.addEventListener("click", () => {
            state.select(card,card_key)
            //CardClick(card, index);
        });
        //card.addEventListener("mouseover", () => {state.select(index)});
    }

    function shake_selection(){
        const cards = Array.from(document.getElementsByClassName("card"));
        const indices = Array.from(state.get_selection_indices());
        console.log("selection indices: ", indices);
        indices.forEach(async (i) => {
            cards[i].style.animation = "shake linear .25s";
	    await new Promise(r => setTimeout(r, 250));
            cards[i].style.animation = "";
        });
    }

    async function jump_selection() {
        const cards = Array.from(document.getElementsByClassName("card"));
        const indices = Array.from(state.get_selection_indices());
        console.log("selection indices: ", indices);
        for(const i of indices){
            cards[i].style.animation = "jump linear .25s";
	    await new Promise(r => setTimeout(r, 250));
            cards[i].style.animation = "";

        }
        //cards[0].style.animation = "jump linear .25s";
        /*
        for(const i in cards){
            cards[i].style.animation = "jump linear .25s";
	    await new Promise(r => setTimeout(r, 250));
        }
        */
    }

    function one_away(){
        const elem = document.getElementById("away");
        animate(elem);
    }

    function already_guessed(){
        const elem = document.getElementById("already");
        animate(elem);
    }

    async function animate(element){
        element.style.opacity = 1;
	await new Promise(r => setTimeout(r, 2000));
        element.style.opacity = 0;
    }

    const init_buttons = () => {
        const submit = document.getElementById("submit");
        submit.addEventListener("click", async () => {
            try{
                await jump_selection();
                state.check_selection();
                cards.forEach(renderCard);
                console.log("match")
            }catch (exceptionVar){
                switch (exceptionVar){
                    case 0: // MISMATCH
                        shake_selection();
                        console.log("mismatch");
                        break;
                    case 1: // NOT ENOUGH 
                        shake_selection();
                        console.log("not enough selected");
                        break;
                    case 2: //One Away
                        shake_selection();
                        one_away();
                        console.log("one away...");
                        break;
                    case 3: //lost
                        shake_selection();
                        console.log("you lose!");
                        break;
                    case 4:
                        already_guessed();
                        console.log("already tried");

                }
            }
        });

        const shuffle = document.getElementById("shuffle");
        shuffle.addEventListener("click", async () => {
	    const elems = Array.from(document.getElementsByClassName("card"));
	    elems.forEach((elem) => {elem.style["color"] = "rgba(0,0,0,0)"}); 
	    await new Promise(r => setTimeout(r, 150));
            state.shuffle();
            state.render_cards();
            //cards.forEach(renderCard);
        });

        const deselect = document.getElementById("deselect");

        deselect.addEventListener("click", () => {
            state.clear_selection();
        });

    }


    console.log("main");
    const game_board = document.getElementById("board");
    const cards = Array.from(game_board.children);
    const state = start_state();
    cards.forEach(initializeCards);
    state.render_cards();
    init_buttons();
}

await run();
//console.log("calling main");
main();



// onclick


// register render


