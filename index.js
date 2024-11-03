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
    const renderCard = (card, index) => {
        state.render_card(card, index);
    }

    const initializeCards = (card, index) => {
        renderCard(card,index);
        card.addEventListener("click", () => {
            CardClick(card, index);
            renderCard(card, index);
        });
        //card.addEventListener("mouseover", () => {state.select(index)});
    }

    const init_buttons = () => {
        const submit = document.getElementById("button");
        submit.addEventListener("click", () => {
            try{
                state.check_selection();
                cards.forEach(renderCard);
                console.log("match")
            }catch (exceptionVar){
                switch (exceptionVar){
                    case 0: // MISMATCH
                        console.log("mismatch");
                        break;
                    case 1: // NOT ENOUGH
                        console.log("not enough selected");
                        break;
                    case 2: //One Away
                        console.log("one away...");
                        break;
                    case 3: //lost
                        console.log("you lose!");
                        break;
                    case 4:
                        console.log("already tried");

                }
            }
        });

        const shuffle = document.getElementById("shuffle");
        shuffle.addEventListener("click", () => {
            state.shuffle();
            cards.forEach(renderCard);
        });

        const deselect = document.getElementById("deselect");

        deselect.addEventListener("click", () => {
            state.clear_selection();
            cards.forEach(renderCard);
        });

    }
    function CardClick(card, index){
        if (!state.select(index)){
            console.log("selection is full!");
        // TODO: make the card animatecard.animate();
        }
    }

    console.log("main");
    const game_board = document.getElementById("board");
    const cards = Array.from(game_board.children);
    const state = start_state();
    cards.forEach(initializeCards);
    init_buttons();
}

await run();
//console.log("calling main");
main();



// onclick


// register render


