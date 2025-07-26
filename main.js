import init, { Hangman } from './hangman_game.js';

let game;

async function run() {
    await init();
    game = Hangman.new("rustacean");
    document.getElementById("word").textContent = game.current_state();
}

window.guess = () => {
    const input = document.getElementById("letter");
    const result = game.guess(input.value);
    input.value = "";
    document.getElementById("word").textContent = game.current_state();

    if (game.is_won()) {
        document.getElementById("status").textContent = "You won!";
    } else if (game.is_over()) {
        document.getElementById("status").textContent = "Game over!";
    }
};

run();
