import init, {GameState, GameFailiure, JsSelectionSuccess, CardState, SelectionSuccessTags} from './pkg/nyt_connections.js';
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
        matches:0
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
        this.start_offset = 0;
    }

    async move_selection(){
        const source_indices = Array.from(elems.selection.all).map((card) => (card.index));
        const dest_indices = Array.from(this.cards_list).slice(this.start_offset, this.rowEnd).map((card) => (card.index));

        const cards_i = source_indices.map((card, index) => {
            return ({source:card, dest:dest_indices[index]});
        });

        console.log(cards_i);

        const cards = cards_i.map(({source, dest}) => {
            return ({source:this.at(source), dest:this.at(dest)});
        })

        console.log(cards);

        cards.forEach(({source, dest}) => {
            source.disabled = true;
            dest.disabled = true;
        });

        console.log(cards_i);
        
        for(const {source, dest} of cards_i.slice(0,3)){
            this.update();
            const s = this.at(source); 
            const d = this.at(dest); 

            this.swap_move(s,d);
            if(s.index >= this.rowEnd){
                s.disabled = false;
            }
        }
        this.update();
        const s = this.at(cards_i[3].source); 
        const d = this.at(cards_i[3].dest); 
        await this.swap_move(s,d);
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
        console.log("add_set");
        const last = this.cards_list[3];
        const slice = Array.from(this.cards_list).slice(0,3);
        for(const card of slice){
            console.log("removing ", card);
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
            {transform: "translate(0,0)"},
            {transform: `translate(${one_to_two[0]}px,${one_to_two[1]}px)`}
        ];
        const to_1 = [
            {transform: "translate(0,0)"},
            {transform: `translate(${two_to_one[0]}px,${two_to_one[1]}px)`}
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
        await check_selection();
    }catch (e){
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

async function check_selection(){
    let success = elems.game.check_selection();

    await update_match(success.color);

    if (success.result = SelectionSuccessTags.Won){
        win();
    }
}

async function update_match(color){
    await elems.board.move_selection(elems.matches);
    const new_set = new ConnectionSet();
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

    set moving(flag){
        if (flag) {
            this._internals.states.add("moving");
        } else {
            this._internals.states.delete("moving");
        }
    }

    get moving(){
        return this._internals.states.has("moving");
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
        this.selected = !this.selected;
        elems.game.select(this.index);
        update_buttons();
    }

    full_render(){
        const card = elems.game.get_owned(this.index);
        this.textContent = card.word;
        if(card.state == CardState.Selected){
            this.selected = true;
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
            this._internals.states.add(color);
        } else {
            this._internals.states.delete(color);
        }
    }
}
