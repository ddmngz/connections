import init, {GameState, Failiure, JsSelectionSuccess, CardState, SelectionSuccessTags} from './pkg/nyt_connections.js';
import {Button} from './index.js';

let elems = null;


export function start_game(parent, puzzle){
    register_elems();
    init_board(parent);
    init_elems(puzzle);
}


function register_elems(){
    customElements.define("connections-card", Card )
    customElements.define("connections-set", ConnectionSet)
}

function init_board(parent){
    const template = document.getElementById(
        "connections-game",
    ).content;
    parent.replaceWith(template.cloneNode(true));
}

function init_elems(puzzle){
    const game = puzzle ? GameState.new(puzzle) : GameState.new();
    elems = {game:game};
    elems = {
        game:game,
        board:new Board(),
        shuffle: new Button("shuffle", shuffle),
        deselect: new Button("deselect", deselect),
        submit: new Button("submit", submit),
        remaining: new RemainingTries(),
        one_away: document.getElementById("away"),
        already_guessed: document.getElementById("already"),
        end_screen: end_screen(),
    };
    elems.selection = new Selection(elems.board);
    elems.shuffle.enable();
}

function render_card(card){
    card.textContent = elems.game.card_text(card.index);
}

class Board{
    constructor(){
        this.board = document.getElementById('board');
        this.cards_list = board.querySelectorAll('connections-card');
        this.forEach(render_card);
        this.start_offset = 0;
    }

    async move_selection(){
        const selection_indices = Array.from(elems.selection.all).map((card) => (card.index));
        const top_row_indices = Array.from(this.cards_list).slice(0, 4).map((card) => (card.index));

        const cards_i = selection_indices.map((card, index) => {
            return ({selection:card, top:top_row_indices[index]});
        });
        

        const cards = cards_i.map(({selection, top}) => {
            return ({selection:this.at(selection), top:this.at(top)});
        })


        cards.forEach(({selection, top}) => {
            selection.disabled = true;
            top.disabled = true;
        });

        
        for(const {selection, top} of cards_i.slice(0,3)){
            this.update();
            const s = this.at(selection); 
            const d = this.at(top); 

            this.swap_move(s,d);
            if(s.index >= this.rowEnd){
                s.disabled = false;
            }
        }
        this.update();
        const s = this.at(cards_i[3].selection); 
        const t = this.at(cards_i[3].top); 
        await this.swap_move(s,t);
        if(s.index >= this.rowEnd){
            s.disabled = false;
        }
        this.start_offset+=4;
        this.update();
    }

    get rowEnd(){
        this.start_offset + 4;
    }

    at(index){
        return this.cards_list[index - this.start_offset];
    }

    forEach(closure){
        this.cards_list.forEach(closure);
    }

    update(){
        this.cards_list = this.board.querySelectorAll('connections-card');
    }

    add_set(set){
        const last = this.cards_list[3];
        const slice = Array.from(this.cards_list).slice(0,3);
        for(const card of slice){
            card.remove();
        }
        last.replaceWith(set);
        elems.selection.update();
    }

    async swap_move(card_1, card_2){
        const c1pos = card_1.getBoundingClientRect();
        const c2pos = card_2.getBoundingClientRect();
        const two_to_one = [c2pos.x - c1pos.x, c2pos.y - c1pos.y];
        const one_to_two = [c1pos.x - c2pos.x, c1pos.y - c2pos.y];
        const to_2 = [
            {transform: "translate(0,0)", easing:"ease-out"},
            {transform: `translate(${one_to_two[0]}px,${one_to_two[1]}px)`, easing:"ease-out"}
        ];
        const to_1 = [
            {transform: "translate(0,0)", easing:"ease-out"},
            {transform: `translate(${two_to_one[0]}px,${two_to_one[1]}px)`, easing:"ease-out"}
        ];
        const promises = [card_1.animate(to_1,250).finished,card_2.animate(to_2,250).finished];
        await Promise.all(promises);
        return(this.swap(card_1,card_2));
    }

    swap(source, dest){
        const two = dest.cloneNode(true);
        const one = source.cloneNode(true);
        two.index = source.index;
        one.index = dest.index;
        two.selected = dest.selected;
        one.selected = source.selected;

        dest.replaceWith(one);
        source.replaceWith(two);
        one.full_render();
        two.full_render();
        return(one);
    }

    async show(){
        await new Promise(r => setTimeout(r, 250));
        this.forEach((card) => {card.full_render(); card.shuffling = false});
        await new Promise(r => setTimeout(r, 250));
        this.forEach((card) => {card.disabled = false});
    }

    hide(){
        this.forEach((card) => {
            card.disabled = true;
            card.shuffling = true;
        });
    }

    update_text(){
        this.forEach((card) => {card.render_text()});
    }

    reset(){ 
        let new_cards = [new_card(0), new_card(1), new_card(2), new_card(3), 
            new_card(4), new_card(5), new_card(6), new_card(7), new_card(8),
            new_card(9), new_card(10), new_card(11), new_card(12), new_card(13),
            new_card(14), new_card(15)]
        this.board.replaceChildren(...new_cards);
        this.update();
        this.update_text();
        elems.selection.update();
        this.start_offset = 0;
    }

    disable(){
	    this.forEach((card) => {
		    card.disable;
	    });
    }
}

class Selection{
    static jump = [
        {transform:"translate(0,0)"},
        {transform:"translate(0,-10px)"},
        {transform:"translate(0,0)"},
    ];

    static shake = [
        {transform:"translate(0,0)"},
        {transform:"translate(-2px,0)"},
        {transform:"translate(2px,0)"},
        {transform:"translate(-2px,0)"},
        {transform:"translate(2px,0)"},
        {transform:"translate(0,0)"},
    ];

    constructor(){

        this.arr = document.querySelectorAll('connections-card:state(selected)');
    }

    update(){
        this.arr = document.querySelectorAll('connections-card:state(selected)');
    }

    get(index){
        return this.arr[index]
    }

    get all(){
        return this.arr;
    }


    async jump(){
        for(let i = 0; i< 3; i++){
            this.arr[i].animate(Selection.jump, 450 );
            await new Promise(r => setTimeout(r, 100));
        }
        await this.arr[3].animate(Selection.jump, 450 ).finished;

    }

    async shake(){
        this.arr.forEach((card) => {card.animate(Selection.shake,250)});
        await new Promise(r => setTimeout(r, 100));
    }

    get len(){
        return this.arr.length
    }

    deselect(){
        this.update();
        this.arr.forEach((card) => {card.selected = false});
        this.update();
    }
}

async function shuffle(){
    elems.board.hide();
    elems.game.shuffle();
    elems.board.show();

}
function deselect(){
    elems.deselect.disable();
    elems.game.clear_selection();
    elems.selection.deselect();
}

async function submit(){
    await elems.selection.jump();
    try{
        await check_selection();
    }catch (e){
        if(e == Failiure.AlreadyTried){
            pop_up(elems.already_guessed);
            return;
        }
        elems.remaining.lose_one();
        switch(e){
            case Failiure.Mismatch:
                await elems.selection.shake();
                break;
            case Failiure.OneAway:
                pop_up(elems.one_away);
                break;
            case Failiure.Lost:
                lost();
                break;
        }
    }
}

async function pop_up(modal){

    const animation = [
        {opacity:0, easing:"ease-out"},
        {offset:.25, display:"block", opacity:1, easing:"ease-in"},
        {offset:.75, display:"block", opacity:1, easing:"ease-in"},
        {opacity:0, easing:"ease-in"},
    ];
    modal.show();
    await modal.animate(animation, 2500).finished;
    modal.close();
}

async function check_selection(){
    let success = elems.game.check_selection();

    await update_match(success.color);

    if (success.result == SelectionSuccessTags.Won){
        win();
    }
}

async function update_match(color){
    await elems.board.move_selection();
    const new_set = new ConnectionSet();
    /*
    elems.board.update();
    elems.board.add_set(new_set);
    new_set.update_color(color);

    elems.board.update();
    */
}




function win(){
    show_end_screen(elems.end_screen,true);
}

function lost(){
    show_end_screen(elems.end_screen,false);
}

function update_buttons(){
    elems.selection.update();
    const len = elems.selection.len;
    update_submit(len);
    update_deselect(len);

}

function update_submit(len){
    if(len == 4){
        elems.submit.enable();
    }
    else if(len == 3){
        elems.submit.disable();
    }
}

function update_deselect(len){
    if(len == 1){
        elems.deselect.enable();
    }
    else if(len == 0){
        elems.deselect.disable();
    }
}

function new_card(i){
    const elem = document.createElement("connections-card");
    elem.setAttribute("i", i);
    return(elem);
}


class Card extends HTMLElement{
    static observedAttributes = ["disabled"];
    constructor(){
        super();
        this._internals = this.attachInternals()
    }

    connectedCallback() {
        if(!this.disabled){
            this.addEventListener("click", this.on_click);
        }
    }

    get selected() {
        this._internals.states;
        return this._internals.states.has("selected");
    }

    set selected(flag){
        if (flag) {
            this._internals.states.add("selected");
        } else {
            this._internals.states.delete("selected"); }
    }

    get index() {
        return Number(this.getAttribute('i'));
    }

    set index(index){
        if (index) {
            this.setAttribute('i', index)
        } else {
            this.removeAttribute('i')
        }
    }

    get row(){
        return(Math.floor( this.index / 4) + 1);
    }

    get column(){
        return(Math.floor( this.index % 4) + 1);
    }

    get disabled(){
        return this._internals.states.has("disabled");
    }

    set disabled(flag){
        if (flag) {
            this._internals.states.add("disabled");
        } else {
            this._internals.states.delete("disabled");
        }
    }

    attributeChangedCallback(name, _oldValue, newValue) {
        assert(name == "disabled")
        if(newValue){
            this.addEventListener("click", on_click);
        }else{
            this.removeEventListener("click", on_click);
        }
    }

    on_click(){
        if(this.selected || elems.selection.len < 4){
            this.toggle_select();
        }
    }

    toggle_select(){
        this.selected = !this.selected;
        elems.game.select(this.index);
        update_buttons();
    }

    full_render(){
        const card = elems.game.get_owned(this.index);
        this.textContent = card.word;
        if(card.state == CardState.Selected){
            this.selected = true;
        }else{
            this.selected = false;
        }
    }

    render_text(){
        this.textContent = elems.game.card_text(this.index);
    }

    

    get shuffling(){
        this._internals.states.has("shuffling");
    }

    set shuffling(flag){
        if (flag) {
            this._internals.states.add("shuffling");
        } else {
            this._internals.states.delete("shuffling");
        }
    }
}



class RemainingTries{
    constructor(){
        this.handle = document.getElementsByClassName('dot');
        this.hidden = document.getElementsByClassName('dot hidden');
    }

    lose_one(){
        const last_ind = this.handle.length - 1 - this.hidden.length ;
        this.handle.item(last_ind).classList.add('hidden');
    }

    reset(){
        const arr = Array.from(this.handle);
        arr.forEach((span) => span.classList.remove('hidden'));
    }
}




function show_end_screen(e, won){
    end_buttons(e).forEach((button) => {button.enable()});
    if(won){
        e.win.classList.add("enabled")
    }else{
        e.lose.classList.add("enabled")
    }
    e.modal.showModal();
}

function end_screen(){

    const end_screen_elems = {
        modal : document.getElementById("endscreen"),
        win : document.getElementById("win"),
        lose : document.getElementById("lose"),
        again : new Button("again", play_again),
        view_board : new Button("see-board", view_board),
        share : new Button("share", share),
        copied : document.getElementById("copied"),
        edit : new Button("edit-me", edit),
        new : new Button("new-puzzle", new_puzzle),
        back : new Button("back", back_to_endscreen),
    }


	function back_to_endscreen(){
		end_screen_elems.back.div.classList.add("hidden");
        	end_screen_elems.modal.showModal();
		end_screen_elems.back.disable();
	}

	function play_again(){
		reset_elems();

		end_buttons(end_screen_elems).forEach((button) => button.disable());
		end_screen_elems.win.classList.remove("enabled");
		end_screen_elems.lose.classList.remove("enabled");
		end_screen_elems.modal.close();
	}

	function reset_elems(){
		elems.game.start_over();
		elems.remaining.reset();
		elems.selection.update();
		elems.board.reset();
		elems.deselect.disable();
		elems.shuffle.enable();
		elems.submit.disable();
	}

	function view_board(){
		end_screen_elems.back.enable();
		end_screen_elems.back.div.classList.remove("hidden");
        	end_screen_elems.modal.close();
		elems.board.disable();
		elems.shuffle.disable();
		elems.deselect.disable();
		elems.submit.disable();
		// show the back button
		// disable all the buttons except the back button
	}

	async function share(){
		const code = elems.game.puzzle_code();
		const url = new URL(document.URL);
		url.searchParams.delete("edit");
		url.searchParams.set("game", code);
		await navigator.clipboard.writeText(url.href);
		await copied_link();

		/*
			const url = new URL("game.html",Dom.url);
		url.searchParams.set("game", Dom.puzzle.encode());
		// Copy the text inside the text field
		await navigator.clipboard.writeText(url.href);
		await copied_link(Dom);
		*/
			// copy link to clipboard
		// animte "copied link"
	}

	async function copied_link(){

		end_screen_elems.share.div.textContent = 'Copied!';

		await new Promise(r => setTimeout(r, 750));

		end_screen_elems.share.div.textContent = "Copy To Clipboard";

	}

	function edit(){
		const code = elems.game.puzzle_code();
		const url = new URL(document.URL);
		url.searchParams.set("edit", code);
		url.searchParams.delete("game");
		window.location.replace(url.href);
	}

	function new_puzzle(){
		const url = new URL(document.URL);
		url.searchParams.delete("edit");
		url.searchParams.delete("game");
		window.location.replace(url.href);
	}

	return(end_screen_elems);
}

function end_buttons(e){
	return([e.again, e.view_board, e.share, e.edit, e.new]);
}




export class ConnectionSet extends HTMLElement{
	static observedAttributes = ["connections-color"];
	constructor(){
		super();
		this._internals = this.attachInternals();
	}

	connectedCallback(){
		const template = document.getElementById(
			"connections-set",
		);
		let templateContent = template.content;

		const shadowRoot = this.attachShadow({ mode: "open" });
		shadowRoot.appendChild(templateContent.cloneNode(true));
	}

	attributeChangedCallback(name, _oldValue, newValue) {
		if (name != "connections-color"){
			return;
		}
		const words_n_theme = elems.game.matched_text(newValue);
		this.theme = words_n_theme[0];
		this.words =  words_n_theme[1];
	}

	update_color(color) {
		if (color) {
			this.setAttribute("connections-color", color);
		} else {
			this.removeAttribute("connections-color");
		}
	}

	get words(){
	}

	set words(content){

		let slots = this.shadowRoot.querySelectorAll("slot");
		for (const slot of slots) {
			if(slot.name == "words"){
				slot.textContent= content;
				return
			}
		}
		throw new Error("couldn't find slot")
	}

	get theme(){
	}

	set theme(content){
		let slots = this.shadowRoot.querySelectorAll("slot");
		for (const slot of slots) {
			if(slot.name == "theme"){
				slot.textContent= content;
				return;
			}
		}
		throw new Error("couldn't find slot")
	}

}
