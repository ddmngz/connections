import init, {GameState, GameFailiure, JsSelectionSuccess} from './pkg/nyt_connections.js';
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
		endscreen:document.getElementById("end-screen"),
		shuffle: new Button("shuffle", shuffle),
		deselect: new Button("deselect", deselect),
		submit: new Button("submit", submit),
		remaining: new RemainingTries(),
		one_away: document.getElementById("away"),
		already_guessed: document.getElementById("already"),
		end_screen: new EndScreen(),
                matched_rows:0
	};
	elems.selection = new Selection(elems.board);
}

function render_card(card){
	card.textContent = elems.game.card_text(card.index);
}


class Board{
	constructor(){
		this.board = document.getElementById('board');
		this.cards_list = board.querySelectorAll('connections-card');
		this.forEach(render_card);
	}



	forEach(closure){
		this.cards_list.forEach(closure);
	}

	update(){

		this.cards_handle = this.board.querySelectorAll('connections-card');
	}

	add_set(set){
            const last = this.cards_list[3];
            const slice = Array.from(this.cards_list).slice(0,3);
            console.log(slice);
            for(const card of slice){
                console.log("removing ", card);
                card.remove();
            }
            last.replaceWith(set);
	}

	show(){
		this.forEach((card) => {card.from_blank()});
	}

	hide(){
		this.forEach((card) => {card.to_blank()});
	}

	update_text(){
		this.forEach((card) => {card.render_text()});
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
		this.arr = document.querySelectorAll('connections-card[selected=""]');
	}

	update(){
		this.arr = document.querySelectorAll('connections-card[selected=""]');
	}

        async move(row){
            // get the top 4 cards
            // animate swapping grid area for all of them
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
}

function shuffle(){
	elems.board.hide();
	elems.game.shuffle();
	elems.board.show();

}
function deselect(){
	elems.game.clear_selection();
	elems.board.forEach((card) => {card.toggleAttribute("selected")});
}

async function submit(){
	await elems.selection.jump();
	try{
		check_selection();

	}catch (e){
		console.log("failiure");
		if(e == GameFailiure.AlreadyTried){
			pop_up(elems.already_guessed);
			return;
		}
		elems.remaining.lose_one();
		switch(e){
			case GameFailiure.Mismatch:
				await elems.selection.shake();
				break;
			case GameFailiure.OneAway:
				pop_up(elems.one_away);
				break;
			case GameFailiure.Lost:
				lost();
				break;
		}
	}
}

function pop_up(modal){
	const animation = {}
	modal.animate(animation);
}

function check_selection(){
	let success = elems.game.check_selection();
	console.log(success);
	update_match(success.color);
        elems.matches++;
	if (success.result = SelectionSuccessTags.won){
		win();
	}
}

async function update_match(color){
    await elems.selection.move(elems.matches);
    const new_set = new ConnectionSet();
    console.log(color);
    new_set.theme_color = color;
    elems.board.update();
    elems.board.add_set(new_set);
}




function win(){
}

function lost(){
}

function update_buttons(){
    elems.selection.update();
    const len = elems.selection.len;
    update_submit(len);
    update_deselect(len);

}

function update_submit(len){
    console.log(len);
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
        if (flag) {
            this.setAttribute('i', index)
        } else {
            this.removeAttribute('i')
        }
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
        this.toggleAttribute("selected");
        elems.game.select(this.index);
        update_buttons();
    }



    full_render(){
        const card = elems.game.get_owned(this.index);
        this.textContent = card.word;
        if(card.state == CardState.Selected){
            this.selected = "";
        }
    }

    to_blank(){
        this.textContent = "";
        this._internals.states.delete("selected");
        this.disabled = "";
    }

    from_blank(){
        this.disabled = null;
        this.full_render();
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
        const arr = Array.from(handle);
        arr.forEach((span) => span.classList.remove('hidden'));
    }
}


class EndScreen{}


export class ConnectionSet extends HTMLElement{
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

      get blue() {
    return this._internals.states.has("blue");
  }



  set blue(flag) {
    if (flag) {
      this._internals.states.add("blue");
    } else {
      this._internals.states.delete("blue");
    }
  }

    get yellow() {
        return this._internals.states.has("yellow");
    }

    get purple() {
        return this._internals.states.has("purple");
    }

    get green() {
        return this._internals.states.has("green");
    }


    get theme_color() {
        return this._internals.states.get("theme_color");
    }

    set theme_color(color) {
        if (color) {
            console.log("adding state ", color)
            this._internals.states.add(color);
        } else {
            this._internals.states.delete(color);
        }
        console.log(this._internals.states);
    }
}
